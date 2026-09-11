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

As of phase 9 (2026-09-11): `c_char` references down from 2262 to 1793 (21%
reduction). Every libc string/char function is now gone except 2 `strncasecmp` and 2
`strdup` sites, both deliberately deferred (see phase 2/3 notes below). The
`printf`/`fprintf`/`vfprintf`/`puts`/`putchar` family is now fully converted: every
live call site across the codebase uses `print!`/`println!`/`eprint!`/`eprintln!`;
the only `printf`/`fprintf` calls left (19 sites: 11 in `z_zone.rs`'s
`Z_DumpHeap`/`Z_FileDumpHeap`, 5 in `i_scale.rs`'s stretch-table cluster, 2 in
`memio.rs`'s MEMFILE module, plus `i_system.rs`'s `fprintf` extern declaration which
`z_zone.rs` still legitimately uses) are confirmed dead code (zero callers each,
verified by full-codebase grep) and left untouched per this track's "leave dead code
alone" precedent.
`FixedCStr<N>` (phase 4) is now the established, working pattern for every
WAD-lumpname-family field found across `w_wad.rs`/`r_data.rs`/`sounds.rs`/
`p_switch.rs`/`p_spec.rs`/`d_main.rs`/`hu_stuff.rs`/`m_menu.rs`/`p_setup.rs`. Remaining
known work: the local `[c_char; N]` scratch/formatting buffers this track
deliberately skipped throughout (lower value than struct fields — revisit only if
they start blocking something), `memcpy`/`memset`/`memmove` (74 sites, not yet
assessed — likely mostly legitimate raw-buffer operations rather than string-shaped,
needs per-site triage before deciding scope), and the general `*mut`/`*const c_char`
pointer sweep the plan's phase 5 describes (much of it may already be resolved as a
side effect of phases 4/6/7/9's struct-field and printf conversions — re-survey
before scoping that phase rather than trusting the original 1369-count estimate).

**Phase 1 done** (`c-char-phase1-toupper-tolower`, PR #275): `toupper`/`tolower`
eliminated everywhere, including collapsing several c2rust glibc-macro-expansion
blocks it turned out most call sites were hiding inside (see commit message for the
`if 0 != 0` dead-branch pattern — check for this same shape before assuming any other
libc call is a simple one-liner).

**Phase 2 done** (`c-char-phase2-strlen-strcmp`, PR #276): `strlen`/`strcmp`/
`strncmp`/`strcasecmp`/`strncasecmp` eliminated everywhere except 6 sites that compare
a WAD lump name's fixed 8-byte (possibly non-null-terminated) field — deliberately
deferred to the lump-name newtype phase rather than risk the truncation edge case.
Confirmed via full-codebase audit that every comparison-function usage in this
codebase only checks equality (`== 0`/`!= 0`), never true C-style ordering, which
simplified every conversion.

**Phase 3 done** (`c-char-phase3-atoi-sscanf-strncpy`, PR #277): `atoi` (all 7 sites
were reaching into an already-`Vec<CString>` `myargv`, replaced with a hand-written
`M_ArgvAtoi` matching `atoi`'s actual lenient-prefix semantics), `sscanf` (both
`M_StrToInt`/`ParseIntParameter` were independently reimplementing the same hex/
octal/decimal auto-detect logic — consolidated), `strncpy` (`M_StringCopy` rewritten
natively, exact zero-pad-the-rest semantics preserved). `strdup` (2 sites) and
`w_wad.rs`'s lumpname `strncpy` deliberately left — see their commit messages for why.

**Phase 4 done** (`c-char-phase4-wadname-newtype`, PR #278): added `FixedCStr<const N:
usize>` (`src/fixed_cstr.rs`) — the newtype design decision 1 called for. Converted
`lumpinfo_t`/`filelump_t`/`wadinfo_t.identification` (`w_wad.rs`) and `texture_t`/
`maptexture_t` (`r_data.rs`), resolving the strncasecmp/strncpy sites deferred since
phases 2-3. `#[repr(transparent)]` meant most surrounding raw-pointer C code needed
zero changes.

**Phase 5 done** (`c-char-phase5-const-tables`, PR #279): converted the largest
const-data-table files — `sounds.rs`'s `sfxinfo_t.name` (109 entries, confirmed never
read/compared at runtime), `p_switch.rs`'s `switchlist_t.name1/name2` (82 sites),
`p_spec.rs`'s `animdef_t.endname/startname` (46 sites, the animated flat/texture
table), `d_main.rs`'s registered-version demo-name table, `hu_stuff.rs`'s chat-macro/
player-color constants. 265 literal-construction sites total.

**Phase 6 done** (`c-char-phase6-remaining-literals`, PR #280): mopped up remaining
individual constants (`PACKAGE_STRING`/`D_DEVSTR`/`PROGRAM_PREFIX`/`DIR_SEPARATOR_S`)
and `m_menu.rs`'s `menuitem_t.name` (41 sites across 9 menu tables — same
WAD-lumpname family as phase 4, just in a file that phase's sweep didn't reach). Hit
and fixed a real landmine removing 9 now-unnecessary `unsafe {}` wrappers: a
regex-based first attempt matched the wrong closing brace for one field (indentation
coincidence), corrupting a struct literal — caught by `cargo build`, fixed with an
actual brace-depth-counting scan instead of pattern-matching on indentation. Left
`am_map.rs`/`st_stuff.rs`'s 34 `cheatseq_t` sites alone (that struct's fields were
deliberately kept as plain arrays back in phase 2/3).

**Phase 7 done** (`c-char-phase7-level-data-names`, PR #281): `p_setup.rs`'s
`mapsidedef_t`/`mapsector_t` on-disk level-data texture/flat name fields — the same
WAD-lumpname family found in a file outside phase 4's original sweep. Confirmed via
the runtime `side_t` struct that these names are purely transient (resolved to `i16`
indices during level load, never kept as strings).

**Phase 8 done** (`c-char-phase8-fprintf-puts-putchar`, PR #282): first half of the
printf-family phase. `fprintf`: all 13 live call sites wrote to stderr, converted to
`eprintln!`/`eprint!` (one exception, `am_map.rs`'s debug counter, uses `\r` not `\n`
so became `eprint!` to preserve the in-place-overwrite behavior); `z_zone.rs`'s
`Z_FileDumpHeap` left untouched (dead code, zero callers). `puts`/`putchar`:
`i_system.rs`'s `I_PrintBanner`/`I_PrintDivider`/`I_PrintStartupBanner` converted to
`print!`/`println!`, including their embedded `printf` calls (`%p`/`%x` zone-memory
line, literal license-text block). `i_scale.rs`'s 2 `puts("")` calls left alone (dead
code). Removed a fully dead `vfprintf` extern declaration.

**Phase 9 done** (`c-char-phase9-printf-sweep`, PR pending): second half — the
remaining ~90 plain `printf` call sites across `r_main.rs`, `r_data.rs`, `w_main.rs`,
`w_wad.rs`, `d_net.rs`, `d_iwad.rs`, `d_loop.rs`, `i_video.rs`, `g_game.rs`,
`wi_stuff.rs`, `m_misc.rs`, `m_config.rs`, and `d_main.rs` (28 sites, the largest
single chunk — startup banner/init-sequence messages, turbo-scale/demo-name/
mission-pack diagnostics). All converted to `print!`/`println!`, using
`CStr::from_ptr(...).to_string_lossy()` wherever the original argument was a raw
`*const c_char` pointer rather than a literal. `PrintDehackedBanners` (`d_main.rs`)
simplified along the way: its `printf("%s", deh_s)` was round-tripping an already-
owned `&str` through a `CString`/`CStr` conversion for no reason (the `if deh_s_str
!= copyright_banners[i]` guard compares the string to itself, always false — a
preexisting artifact, not something this phase changed) — replaced with a direct
`print!("{}", deh_s_str)`. Verified: 0 new build warnings (full sorted-warning-text
diff against the pre-phase baseline is empty, 104 warnings both sides), release build
clean, 3x Xvfb boot with no panics, screenshot-confirmed rendering intact, startup
banner output spot-checked byte-for-byte against the original printf formatting
(including the `R_Init: Init DOOM refresh daemon - .........` line, which splits a
`print!` without a trailing newline across two source locations — the dots are
printed by `R_Init` itself, and the newline comes from the following `P_Init` message,
exactly as the original C code structured it).

Printf-family conversion is now complete except for the 19 dead-code call sites
documented in Status above.

**Phase 10 done** (`c-char-phase10-sprnames-musicinfo`, PR pending): the re-survey
turned up three more WAD-lumpname/const-table-shaped clusters, all converted:
- `info.rs`'s `InfoState.sprnames: [*mut c_char; 139]` (a NULL-terminated pointer
  array, the classic C idiom) → `[&'static str; 138]` (dropped the NULL sentinel —
  Rust arrays carry their own length). Required touching the consumer chain:
  `p_setup.rs::P_Init` (builds the pointer, calls `R_InitSprites`), `r_things.rs`'s
  `R_InitSprites`/`R_InitSpriteDefs` (`*mut *mut c_char` param → `&[&'static str]`,
  the NULL-terminated `while !(*check).is_null()` walk → `namelist.len()`), and
  `RThingsState.spritename` (`*mut c_char` → `&'static str`, default `""`) plus its
  6 `CStr::from_ptr(...).to_str().unwrap()` read sites, now direct field reads.
- `sounds.rs`'s `sfxinfo_struct.tagname: *mut c_char` (109 entries) — confirmed via
  full-codebase grep this field is never read anywhere, only ever written `NULL`;
  retyped to `Option<&'static str>` (kept the field rather than deleting it, since
  unlike the track's confirmed-dead *functions* this is a live struct still used
  every frame — only this one field is inert) with all 109 sites → `None`.
- `sounds.rs`'s `musicinfo_t.name: *mut c_char` (68 entries, max length 6) — same
  WAD-lumpname family as `sfxinfo_struct.name` (phase 5), just missed by that sweep
  since it's a different struct in the same file. Converted to `FixedCStr<7>`; the
  one live read site (`s_sound.rs`'s `S_ChangeMusic`, feeding it to `M_snprintf`'s
  `%s`) updated to `.as_ptr() as *const c_char`, matching the `D_DEVSTR` precedent
  from phase 9. The `mus_None` dummy entry's `name: null` became `FixedCStr([0u8;
  7])` (all-zero/empty) — strictly safer than the null pointer it replaced.

`c_char` references: 1793 → 1260 (a further 30% cut, 44% cumulative from the
original 2262). Verified with the full bar: 0 new warnings, release build clean, 3x
Xvfb boot with no panics, screenshot-confirmed sprite rendering intact (the
`R_InitSprites` path is exercised by every level load).

**Phase 11 done** (`c-char-phase11-return-string-fns`, PR pending): swept the
codebase for functions with a `-> *mut/*const c_char` return signature (a cleaner,
more mechanically-verifiable unit than a raw grep-count) and converted every live
one:
- `d_mode.rs`'s `D_GameMissionString` — a pure match-over-literals function, ->
  `&'static str` directly, its one caller (`w_wad.rs`) simplified to drop the
  `CStr::from_ptr(...).to_str().unwrap()` wrapping.
- `d_iwad.rs`'s `D_SuggestGameName`/`D_SaveGameIWADName` — both were doing
  `CString::new(iwad.description_or_name).unwrap().into_raw()`, i.e. **leaking
  heap memory on every call** to manufacture a `*mut c_char` out of an `iwad_t`
  field that was already `&'static str` (from an earlier track). Converting both
  to return `&'static str` directly eliminates the leak as a side effect, not just
  the `c_char`. `D_SuggestGameName`'s caller (`w_wad.rs`) simplified to match.
  `D_SaveGameIWADName`'s caller (`d_main.rs`) needed no change (already just
  forwards the value into `M_GetSaveGameDir`).
- `m_config.rs`'s `M_GetSaveGameDir`'s `iwadname` parameter — confirmed via
  reading the function body that it's never actually used (dead parameter, a
  vestige of chocolate-doom's per-game save-dir logic this port doesn't need) —
  retyped `*mut c_char` -> `&'static str` to match its new caller, zero functional
  risk since nothing inside the function reads it.
- `d_iwad.rs`'s `D_SuggestIWADName` (same `into_raw()`-leaking shape, confirmed
  zero callers via full-codebase grep) and `m_config.rs`'s `GetDefaultConfigDir`/
  `M_GetStrVariable`/`M_TempFile`/`p_saveg.rs`'s `P_TempSaveGameFile`/
  `P_SaveGameFile` left alone — each either confirmed dead (`M_GetStrVariable`,
  reached only through `M_SetVariable`, which is itself zero-caller: the whole
  runtime `.cfg`-file-parsing path that would invoke `SetVariable`/`strdup` does
  not exist in this codebase, so **the 2 "deliberately deferred" `strdup` sites
  noted since phase 2/3 are in fact confirmed dead code**, not merely low-priority)
  or depends on the `M_StringJoin`/`configdir` cluster, a genuinely bigger
  structural piece (real file-I/O path-building, used pervasively) deferred to a
  dedicated future phase rather than rushed here.

`c_char` references: 1260 → 1236. Small numerically, but each site converted here
was a correctness/resource-leak fix as much as a style change. Verified: 0 new
warnings (one pre-existing `unused variable: iwadname` warning disappeared, a
strict improvement, not investigated further), release build clean, 3x Xvfb boot
with no panics, savegame-dir startup output spot-checked byte-for-byte (`Using .
for configuration and saves` / `Using ./.savegame/ for savegames`).

**Phase 12 done** (`c-char-phase12-configdir-savegame-paths`, PR pending): tackled
the `M_StringJoin`/`configdir`/`savegamedir` file-I/O path-building cluster flagged
above as needing its own dedicated phase. This was the riskiest phase in the track
so far — it touches real save-file I/O, not just display strings — so it got the
full verification bar including an actual interactive save/load round-trip test
(see below), not just a boot smoke test.
- `m_misc.rs`'s `M_StringJoin` (a variadic C-ABI function, `extern "C" fn(s: *const
  c_char, args: ...)`) had only 5 call sites total — removed entirely, each site
  replaced with a direct `format!`. `DIR_SEPARATOR_S` (duplicated per-file as
  `FixedCStr<2>` in `m_misc.rs`/`m_config.rs` alongside an already-native `&str`
  copy in `d_iwad.rs`) converted to plain `&str = "/"` in both remaining files,
  matching the existing precedent. `M_MakeDirectory` retyped to take `&str`,
  building the `CString` for the `mkdir` libc call internally.
- `m_config.rs`'s `MConfigState.configdir`/`default_main_config`/
  `default_extra_config` and `default_collection_t.filename` (the `doom_defaults`/
  `extra_defaults` display-only path fields) converted to `String`/`&'static str`.
  `M_SetConfigDir`'s `dir: *mut c_char` parameter (a "null means use the default"
  C idiom) became `dir: Option<&str>`, the natural Rust equivalent.
  `GetDefaultConfigDir`/`M_GetSaveGameDir` now return `String` directly instead of
  a `malloc`'d/`strdup`'d buffer. `M_SaveDefaultsAlternate` (confirmed zero
  callers, same dead-code status noted in phase 11) needed its body's raw-pointer
  swap-and-restore dance updated to `.clone()` to keep compiling, since `String`
  isn't `Copy` — a one-line fix to keep dead code compiling, not a behavior
  change.
- `d_main.rs`'s `DMainState.savegamedir` converted to `String`; its two
  `M_SetConfigDir`/`M_SetConfigFilenames` call sites updated to pass
  `None`/plain `&str` literals instead of raw pointers.
- `p_saveg.rs`'s `PSavegState` simplified: `temp_savegame_filename` (`*mut
  c_char`) became `Option<String>`, preserving the original's compute-once-cache
  behavior. `savegame_file_filename`/`savegame_file_filename_size` (a
  `malloc`'d-once, `M_snprintf`'d-fresh-every-call buffer — a pure
  implementation-detail optimization with no observable behavior difference from
  just building a fresh `String`) removed entirely; `P_SaveGameFile`/
  `P_TempSaveGameFile` now return `String` built via `format!`.
- Every downstream consumer of these paths updated to match, following this
  codebase's established `CString::new(path).unwrap()` bound to a local variable
  before passing `.as_ptr()` to the real libc boundary call (`fopen`/`remove`/
  `rename`/`mkdir`) — the same pattern already used elsewhere in this codebase
  (`d_main.rs`, `m_menu.rs`, `w_wad.rs`, `hu_stuff.rs`) before this phase, not a
  new one invented for it. Three call sites (`g_game.rs`'s `G_DoSaveGame`,
  `m_menu.rs`'s `M_ReadSaveStrings`/`M_LoadSelect`) that previously round-tripped
  through a local `[c_char; 256]` scratch buffer via `M_StringCopy` before
  `fopen`/`G_LoadGame` were simplified to build the `CString` directly from the
  now-owned `String`, removing the unnecessary intermediate buffer.
- **Verification**: 0 new build warnings (same one pre-existing warning
  improvement as phase 11), release build clean, 3x Xvfb boot with no panics. Plus
  a real interactive save/load round-trip via `xdotool` (see below): saved a game
  ("GAME SAVED." confirmed on screen, `doomsav0.dsg` written to `.savegame/`),
  restarted the process fresh, opened the Load Game menu (confirmed the saved
  slot's description showing), selected it, and confirmed gameplay resumed with
  no panic. This exercised every `fopen`/`remove`/`rename` boundary call and their
  `CString` lifetimes for real, not just the display-only startup paths the boot
  smoke test alone would have covered.
- **Environment note for future interactive tests in this repo**: this Xvfb
  instance's XKB keymap has a reproducible **+3 offset bug** on F-keys only —
  sending keysym `F`*N* via `xdotool key` triggers the DOOM action normally bound
  to `F`*(N+3)* (confirmed via screenshots: requested `F1`→got Sound Volume menu,
  which is `key_menu_volume`/F4's action; requested `F3`→got the Save Game menu,
  which is `key_menu_qsave`/F6's fallback behavior when no slot is picked yet).
  Return, Escape, arrow keys, and letter keys are all unaffected — only the F-row
  is shifted. Root cause not investigated (likely an Xvfb/XTEST keycode-table
  quirk unrelated to this codebase). Workaround used successfully: navigate the
  actual menu screens with Escape/arrows/Return instead of relying on the F-key
  shortcuts, and remember to call `xdotool windowfocus <winid>` before sending any
  keys — this app never calls `XSetInputFocus` itself, so without an explicit
  `windowfocus` call (there is no window manager in this environment to grant
  focus automatically) no keyboard input reaches it at all.

`c_char` references: 1236 → 1141.

**Phase 13 done** (`c-char-phase13-player-message`, PR pending): converted
`player_t.message` (the pending-HUD-message slot: pickup notices, cheat feedback,
"game saved.", chat messages, etc.) from `*mut c_char` to `Option<String>` — a
89-usage hub spanning 10 files (`d_player.rs`, `g_game.rs`, `d_net.rs`,
`hu_stuff.rs`, `p_doors.rs`, `st_stuff.rs`, `p_inter.rs`, `m_menu.rs`, `am_map.rs`,
`p_saveg.rs`, `p_mobj.rs`).
- `player_s` dropped its `#[derive(Copy, ...)]` (kept `Clone`) since `String` isn't
  `Copy`. Verified safe first: full-codebase grep for any by-value `player_t`
  usage (function parameters, struct-to-struct assignment) found none — every use
  is through a pointer or reference.
- Found and fixed a real memset-landmine before it could bite: `G_PlayerReborn`
  does `memset(p, 0, size_of::<player_t>())` then explicitly restores a few
  preserved fields (frags/killcount/itemcount/secretcount) — added
  `ptr::write(&raw mut (*p).message, None)` to that same restore step, since a raw
  memset doesn't properly initialize a non-`Copy` field even though the resulting
  all-zero-bytes bit pattern happens to be a valid `None` today. Matches this
  function's own established "memset then fix up specific fields" idiom rather
  than introducing a new pattern.
- Three now-pointless scratch buffers removed entirely, once `.message` could own
  its `String` directly instead of pointing at them: `hu_stuff.rs`'s
  `hu_responder_lastmessage` (chat messages — simplified to
  `Some(w_chat.l.l.clone())`, no longer even needs the old 81-byte truncation
  logic since `HU_MAXLINELENGTH` already caps typed chat input at 80 chars, so the
  truncation path was dead weight, not a behavior difference), `g_game.rs`'s
  `g_ticker_turbomessage` ("X is turbo!"), `st_stuff.rs`'s `st_responder_mypos_buf`
  ("ang=0x..;x,y=(..)" debug readout), `am_map.rs`'s `am_responder_buffer`
  ("Marked Spot N") — each was an `M_snprintf`-into-fixed-buffer-then-point-at-it
  dance, replaced by a direct `format!` → `Some(...)`.
- `d_net.rs`'s `PlayerQuitGame` simplified similarly: the C original copied a
  literal "Player 1 left the game" into a function-local `static mut` buffer then
  arithmetically incremented the `'1'` byte to reflect the actual player number —
  replaced with `format!("Player {} left the game", player_num + 1)`, removing the
  `static mut` entirely.
- The savegame-format serialization pair (`saveg_read_player_t`/
  `saveg_write_player_t` in `p_saveg.rs`) needed real thought since `.message` is
  part of the `.dsg` binary layout — see `docs/known-deviations.md`'s new entry
  for the full reasoning: the field was always meaningless padding in practice
  (its loaded value gets unconditionally discarded by `P_UnArchivePlayers` every
  time, confirming upstream never actually restored it), so the new code keeps the
  same 4-byte-slot file layout but writes a fixed placeholder and always loads as
  `None` — strictly safer than the original's latent wild-pointer-read risk, not a
  behavior loss for any code path that exists today.
- Verified with the full bar plus two targeted interactive checks beyond the
  standard save/load round-trip (redone here too, since this phase touches the
  save format): the `idkfa` cheat (screenshot-confirmed "VERY HAPPY AMMO ADDED"
  displays correctly through the new `Option<String>` HUD path) and a fresh
  save→restart→load round trip (`doomsav0.dsg` identical size to phase 12's,
  confirming the file layout truly didn't change; load completed with no panic
  and no garbled message).

`c_char` references: 1141 → 955.

**Phase 14 done** (`c-char-phase14-callback-hub-and-shiftxform`, PR pending): two
independent wins found during the fresh post-phase-13 survey.
- `i_input.rs`'s `shiftxform` — a 128-entry keyboard shift-translation lookup table
  (raw keycode → its shifted character, e.g. `'1'` → `'!'`). This was never a
  string; `c_char` was being used purely as a numeric byte type here (a Track-4-
  flavored finding inside a Track-18 sweep). All 128 entries are non-negative
  ASCII values, confirmed by inspection, so `[c_char; 128]` → `[u8; 128]` is a
  pure representation change with zero risk. Its one call site's `as u8` cast and
  a `size_of::<[c_char;128]>()/size_of::<c_char>()` array-length idiom (a c2rust
  way of writing `128`) both simplified away. `i_input.rs`: 131 → 0.
- The `load_callback_t` hub (`st_stuff.rs`/`wi_stuff.rs`) — a shared function-
  pointer TYPE (`Option<unsafe fn(*mut c_char, *mut *mut patch_t)>`) used by
  `ST_loadUnloadGraphics`/`WI_loadUnloadData` to abstract "load or unload this WAD
  lump into this patch-pointer slot" over 4 different callback implementations
  (`ST_loadCallback`/`ST_unloadCallback`/`WI_loadCallback`/`WI_unloadCallback`).
  Matches the project's known "shared fn-pointer type is a real blocker, must
  convert every implementor together" pattern (see [[doom_v5_known_gotchas]] in
  memory) — but unlike Track 2's boolean-callback hub, this one turned out
  tractable in one phase since it's fully contained to 2 files. Each callback
  body was already calling `wad_name8_to_string(...)` on its raw pointer
  immediately, then passing the result to `W_CacheLumpName`/`W_ReleaseLumpName`
  (both already `&str`-taking) — so retyping the callback parameter to `&str`
  let every implementation drop that conversion wrapper entirely, not just
  change its signature. The two call sites (`ST_loadUnloadGraphics`,
  `WI_loadUnloadData`) mechanically converted via script: ~90 `M_snprintf`/
  `snprintf`-into-a-`[c_char;9]`-buffer-then-pass-its-address call pairs became
  direct `format!(...)` calls (with a small printf-format-string-to-Rust-format-
  string translator handling the `%d`/`%2.2d`/`%.2d` specifiers actually used),
  and literal-only calls dropped their byte-string-cast boilerplate for a plain
  `&str` literal. One non-matching call site (`WI_loadUnloadData`'s background-
  pic selection, an if/else-if/else choosing between two literals and one
  `format!`) needed manual conversion since the buffer-write and callback-call
  were separated by branching logic the mechanical script's pattern didn't cover.
  Both now-empty `[c_char; 9]` scratch buffers removed. Verified beyond the
  standard bar: screenshot-confirmed the status bar (health/ammo/armor/face/arms
  — all loaded through `ST_loadCallback`) renders pixel-correct; the
  intermission-screen half (`WI_loadCallback`) wasn't feasible to reach via an
  automated Xvfb test (requires completing a level) but is the identical
  mechanism verified working on the status-bar half, same call pattern, same
  underlying `W_CacheLumpName`.

`c_char` references: 955 → 700.

**Phase 15 done** (`c-char-phase15-texture-flat-lookup`, PR pending): converted the
`R_FlatNumForName`/`R_CheckTextureNumForName`/`R_TextureNumForName` trio
(`r_data.rs`) from `*mut c_char` to `&str` — a small (3-function) but
widely-called mini-hub for resolving a texture/flat name to its internal index,
used across level setup, animated-texture init, switch init, and sky selection.
- `R_InitTextures`'s PNAMES-parsing loop turned out to have a fully redundant
  `[c_char; 9]` scratch buffer: it was `M_StringCopy`-ing 8 raw bytes out of the
  WAD's patch-name table into a local buffer, null-padding it, then immediately
  calling `wad_name8_to_string` on it — but `wad_name8_to_string` already reads
  exactly 8 raw bytes and handles the not-necessarily-null-terminated case
  itself, so the buffer added nothing. Simplified to call it directly on the WAD
  buffer offset.
- `R_FlatNumForName`'s own error-path had the same redundancy (re-decoding the
  same raw pointer into a second local buffer just to print it) — removed, using
  the already-decoded `&str` parameter directly.
- `R_CheckTextureNumForName` bridges to `W_LumpNameHash` (a lower-level hash
  function still on the `*const c_char` boundary, used elsewhere too and out of
  scope for this phase) via a `CString`, matching this codebase's established
  boundary-conversion pattern rather than converting that function too.
- All 8 external call sites (`g_game.rs` ×3 — including simplifying two
  "select a sky texture name by branching" locals from raw-pointer juggling to
  plain `&str` assignment, `p_setup.rs` ×5 reading `mapsector_t`/`mapsidedef_t`'s
  already-`FixedCStr` fields via `.as_str()`, `p_switch.rs` ×2 simplifying
  needlessly-manual pointer-offset array indexing to plain indexing,
  `p_spec.rs` ×5 reading `animdef_t`'s already-`FixedCStr` fields) updated to
  match — every one of them was already backed by a `FixedCStr` field, a
  literal, or a branch-selected literal, so no new `CString` bridging was
  needed at any call site.
- Verified beyond the standard bar with a screenshot: this trio resolves every
  floor/ceiling/wall texture and the sky texture for the whole level, so a
  pixel-correct render is a strong end-to-end signal for the entire change.

`c_char` references: 700 → 649.

**Phase 16 done** (`c-char-phase16-snd-musiccmd-and-pcx`, PR pending): two more
small, independent wins found while responding to the user opening `i_sound.rs` in
their editor.
- `i_sound.rs`'s `snd_musiccmd` (the external-MIDI-player-command config
  variable) — confirmed via full-codebase grep it's never read anywhere, only
  ever written (its always-empty default). Same category as phase 10's
  `sfxinfo_struct.tagname` and phase 11's dead-`.cfg`-parsing-path finding: it's
  bound via `M_BindVariable` into the config table, but the runtime path that
  would ever populate a `DEFAULT_STRING` variable from an actual config file is
  confirmed dead (see phase 11). Converted `*mut c_char` → `Option<&'static
  str>`, default `None`. In the same survey, confirmed `savedir`/`back_flat`/
  `nickname`/`video_driver`/`window_position` (the other unbound `DEFAULT_STRING`
  table entries) have no backing field at all to convert — they're declared in
  the config table but never `M_BindVariable`'d anywhere, so there's nothing
  `c_char`-shaped to touch for those.
- `v_video.rs`'s `pcx_t` (the PCX screenshot file-format header struct) —
  `manufacturer`/`version`/`encoding`/`bits_per_pixel`/`reserved`/
  `color_planes`/`filler` were never string data, `c_char` was used purely as a
  numeric byte type for this `#[repr(C, packed)]` binary-format struct (matching
  phase 14's `shiftxform` finding). Converted to `u8`, matching the struct's own
  other byte fields (`palette: [u8; 48]`, `data: u8`). While in the file, also
  converted `M_FileExists`/`M_WriteFile` (`m_misc.rs`) and `WritePCXfile`
  (`v_video.rs`) from `*mut c_char` to `&str`, using this codebase's established
  `CString::new(...).unwrap()`-bound-to-a-local boundary pattern for their real
  `fopen` calls. `V_ScreenShot`'s filename-building `format!` lost a
  now-unnecessary manually-embedded `\0` (was needed only for the old raw-pointer
  path). Fixed up all 3 external callers, including `d_iwad.rs`'s `file_exists`
  wrapper, which collapsed from a 3-line `CString` bridge to a direct call now
  that `M_FileExists` does its own bridging internally.
- **Verification note**: `M_FileExists`'s new `CString` bridge is exercised on
  every single boot (it backs `d_iwad.rs`'s IWAD-file search, run unconditionally
  at startup) — all 3 Xvfb boots finding and loading `doom1.wad` is a real,
  repeated end-to-end confirmation of that half. `WritePCXfile`/`M_WriteFile`'s
  screenshot-writing path itself could not be exercised via automated Xvfb
  testing: it requires `-devparm` plus the `key_menu_help` (F1) binding, and this
  sandbox's Xvfb F-key +3 offset bug (see phase 12) has no way to synthesize an
  "effective F1" press (would need to request `F(1-3)`, not a valid key).
  Confidence here rests on: the identical, already-proven `CString` pattern: pure
  type-label swaps for the PCX header (no logic change, 1-byte-either-way); and
  the RLE-packing/file-writing logic itself untouched.

`c_char` references: 649 → 626.

Next candidate: continue the raw `*mut`/`*const c_char` pointer sweep — remaining
concentrations are `st_stuff.rs` (the `cheatseq_t` byte-sequence family,
deliberately kept as plain `c_char` arrays since Track 18 phase 2/3 — not a string
in any meaningful sense, a sequence of raw keycodes matched byte-by-byte), `d_main.rs`
(~120, largely local `[c_char; 256]` scratch buffers for demo/turbo/mission-pack
argument parsing — lower value, this track has deliberately deferred local scratch
buffers throughout), `g_game.rs`, `p_inter.rs`, `wi_stuff.rs` (its own remaining
non-callback-hub sites), `m_misc.rs` (mostly `M_snprintf`/`M_vsnprintf` themselves —
genuine variadic C-ABI printf reimplementations still used by many buffer-building
call sites across the codebase, a structural piece rather than a simple field
conversion — replacing these would mean auditing and converting every one of their
callers, a much larger undertaking than a bounded field-conversion phase), `m_menu.rs`,
`hu_stuff.rs`. Each needs the same per-cluster triage phases 10-16 used (is it a
lumpname-shaped const table, an always-null/always-dead field or function, a local
scratch buffer, a cheat-sequence byte array, a numeric-lookup-table mislabeled as
c_char, a shared callback-type hub, a small widely-called name-resolution function
trio, or a structural printf/path-building engine piece) before deciding scope —
the original blanket 1369-count estimate has already proven an unreliable guide to
where the real work is.
