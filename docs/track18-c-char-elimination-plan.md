# Track 18 — Eliminate `c_char`, adopt native Rust strings

## Context

`rust-doomgeneric` is a c2rust transpile: every C `char`, `char[N]`, `char *` became
`::core::ffi::c_char`, `[c_char; N]`, `*mut/*const c_char`, manipulated via `libc`'s
`strlen`/`strcmp`/`snprintf`/etc. and C's `printf` family. Track 1 (8 phases, complete)
already converted most *function signatures* from raw `c_char` pointers to `&str`, but
deliberately deferred the underlying `c_char` type itself as "many genuine C-string
interop sites... needs a more careful per-site pass, not a phase-1/2-style sweep".

This track is that per-site pass. Scope as of 2026-09-11:

- **2262** `c_char` references across 50 files
- **488** fixed-size `[c_char; N]` struct fields (candidates for sized newtypes)
- **1369** raw `*mut`/`*const c_char` pointer occurrences
- **~100** libc string function calls (`strlen`/`strcmp`/`strncmp`/`strcasecmp`/
  `strncasecmp`/`strncpy`/`strdup`/`snprintf`/`sscanf`/`atoi`/`toupper`/`tolower`)
- **~101** `printf`/`fprintf`/`vfprintf`/`puts` calls (the "formatting" piece)

**One genuine hard boundary stays `c_char` forever**: `doomgeneric_xlib.rs`'s real X11
FFI calls (`XOpenDisplay` etc., ~15 occurrences) actually cross into the system's C
library — unlike the `atexit_func_t`/etc. hubs from Track 16, this `extern "C"` is a
real boundary, not a c2rust artifact. Everywhere else, `c_char` is purely this crate's
own internal representation and a legitimate conversion target.

## Decisions (user-confirmed 2026-09-11)

1. **Fixed-size C-string fields become sized newtypes preserving truncate/pad
   semantics**, not plain `String`. Several fixed widths are load-bearing for file
   format compatibility (WAD lump names are exactly 8 bytes on disk, savegame/demo
   fields have fixed slot widths) — a newtype must truncate/pad at construction the
   same way the current `[c_char; N]` does, so savegame/demo/WAD files stay
   byte-compatible. Example shape, informed by the pre-existing `wad_name8_to_string`
   helper's already-correct truncation/non-null-terminated-at-max-length semantics:

   ```rust
   pub struct LumpName([u8; 8]); // or String-backed with an enforced cap, TBD per-field
   ```

   Exact backing representation (fixed byte array vs. capped `String`) should be
   decided per-field based on how it's actually used (raw memcpy from file bytes vs.
   general text manipulation) — don't force one shape everywhere.

2. **`printf`/`fprintf`-family replacement is in scope for this track**, not deferred
   separately — converting the string representation naturally touches most `printf`
   call sites anyway (they're passed the same buffers/pointers this track is
   converting), so doing both together avoids two passes over the same lines.

## Phasing strategy (same discipline as every prior track: one branch/PR per phase,
full verification bar, re-survey before each phase since earlier phases shift what's
left)

Rough order, softest/most self-contained first (subject to revision as work proceeds,
matching every prior track's experience):

1. **Single-character libc functions** (`toupper`/`tolower`) — operate on one `i32`
   char code at a time, completely independent of whether the surrounding buffer is
   `c_char` or `u8`/`String`. Trivial, safe, ~11 call sites across 6 files. First
   phase, no design risk.
2. **Simple comparison/length libc functions** (`strlen`/`strcmp`/`strncmp`/
   `strcasecmp`/`strncasecmp`) at sites that already hold or can cheaply get a `&str`/
   `CStr` — replace with `.len()`/`==`/`.eq_ignore_ascii_case()`.
3. **`atoi`/`sscanf`** — replace with `str::parse`/manual parsing once operating on
   real Rust strings.
4. **Fixed-size array struct fields**, file by file (same unit-of-work discipline as
   Track 16's `GameState` threading) — starting with the smallest, least-coupled
   structs, saving WAD lump names / savegame fields (file-format-critical, touch raw
   byte I/O) for a dedicated phase once the newtype pattern is proven on lower-stakes
   fields.
5. **Raw `*mut`/`*const c_char` pointers** not already covered by (4) — many will
   convert as a natural side effect of the struct-field work; whatever remains gets
   its own sweep.
6. **`printf`/`fprintf`/`vfprintf`/`puts`** → `print!`/`eprintln!`/`format!` — likely
   interleaved with (2)-(5) rather than a separate pass, per decision 2 above, since
   most `printf` call sites are formatting the very buffers this track converts.

## Verification bar (unchanged from every prior track)

1. `cargo build` (lib and `--bin doomgeneric_xlib`): 0 new errors.
2. Full sorted-warning-text diff against the established baseline: empty (or an
   explicitly reviewed and justified change).
3. `cargo build --release --bin doomgeneric_xlib`, ≥3 clean Xvfb runs against
   `/home/michael/Downloads/doom1.wad`, screenshot-verified for any rendering-adjacent
   phase.
4. For any phase touching savegame/demo/WAD-lump fields specifically: a save/load or
   demo record/playback round-trip test, not just a boot smoke test — these are the
   fields where a subtle truncate/pad mismatch would silently corrupt a file format
   rather than crash.

## Status

**Phase 1 done** (`c-char-phase1-toupper-tolower`, PR #275): `toupper`/`tolower`
eliminated everywhere, including collapsing several c2rust glibc-macro-expansion
blocks it turned out most call sites were hiding inside (see commit message for the
`if 0 != 0` dead-branch pattern — check for this same shape before assuming any other
libc call is a simple one-liner). Next: phase 2, `strlen`/`strcmp`/`strncmp`/
`strcasecmp`/`strncasecmp` at sites already holding or cheaply able to get a `&str`.
