# Track 16 — Eliminate `static mut` / `unsafe` via a threaded `GameState`

## Context

`rust-doomgeneric` is a c2rust transpile of doomgeneric. Every previous idiomatic-Rust
track (strings, `boolean`→`bool`, type/const dedup, `extern "C"` linkage cleanup,
action-function enums, ...) has been a mechanical, name-by-name or file-tiered sweep
that never needed to touch the fundamental architecture: C's implicit global mutable
state, transpiled 1:1 into ~974 `static mut` declarations across 65 files, accessed
from 881 `unsafe fn`.

This track is different in kind, not just size: removing `static mut` means every
function that reads or writes one must instead receive that state explicitly, which
means every *caller* of that function must also receive it (to pass it down), all the
way up to the program's actual entry point. This is a call-graph-shaped problem, not a
file-tiering problem, and it cannot be scripted the way prior tracks were.

Two exploratory investigations grounded this plan:
- The real per-tic root is `doomgeneric_xlib.rs`'s bin-crate `main()` loop
  (`doomgeneric_xlib.rs:1148-1157`), which repeatedly calls `doomgeneric_Tick()`
  (`d_main.rs:505-512`) across the `staticlib`/`[[bin]]` crate boundary via
  `extern "C"`. `doomgeneric_Tick` fans out to `TryRunTics`, `S_UpdateSounds`,
  `D_Display` — the true depth-1 call graph.
- Four "atomic hub" callback families exist where a function's signature is fixed by
  being stored as a value in a shared table/enum, so they can't be converted one name
  at a time — each needs its own single, atomic, all-at-once edit, same discipline as
  Track 9/10's dispatch-table work: `StateAction`/`ThinkerFn` (p_mobj.rs enums, driving
  the `A_*`/`T_*` functions), the `PIT_*`/`PTR_*` iterator-callback family
  (`traverser_t` and friends in p_maputl.rs/p_map.rs), and `m_menu.rs`'s
  `menuitem_t.routine`/`menu_t.routine` tables.

User-confirmed design decisions (via AskUserQuestion):
1. **Single unified `GameState` struct**, not independent per-module state types —
   the call graph is heavily cross-coupled (e.g. `A_Chase` alone touches globals from
   `doomstat.rs`, its own file, and calls into `s_sound.rs`/`m_random.rs`), so most
   non-trivial functions would need many separate `&mut` params under a per-module
   design. One `&mut GameState` threaded down avoids that.
2. **Transitional bridge is acceptable**: a temporary `static mut GAME_STATE` plus an
   `unsafe fn game_state() -> &'static mut GameState` accessor, used only at call
   sites not yet converted, shrinking phase by phase, fully gone only in the final
   phase. This is required to keep every phase independently buildable/reviewable,
   matching every prior track's verification bar.
3. **The `DG_Init`/`DG_DrawFrame`/`DG_SleepMs`/`DG_GetTicksMs`/`DG_GetKey` trait is
   out of scope for this track** — it requires restructuring the `staticlib`+`[[bin]]`
   crate split, which is orthogonal to state-threading. Revisit as its own future
   track once `GameState` is a stable concrete type.

## Architecture

**New file `src/game_state.rs`**, canonical home for the aggregate struct:

```rust
pub struct GameState {
    pub d_event: d_event::DEventState,
    // one field per module converted so far, added incrementally
}

impl GameState {
    pub fn new() -> Self { ... } // mirrors today's static initializers
}

static mut GAME_STATE: GameState = ...; // transitional only, deleted in the final phase
pub unsafe fn game_state() -> &'static mut GameState {
    &mut GAME_STATE
}
```

Each converted module keeps its own `XxxState` struct **defined in that module's own
file** (matching the existing "canonical home = matching C header" convention from
every prior track), holding what used to be that file's `static mut` items. A
module's functions become plain (non-`unsafe`) functions/methods taking:
- `&mut GameState` by default (the common case — most functions reach into more than
  one module's state once you look closely, per the exploration above), or
- a narrower `&mut XxxState` **only** when a function is proven to touch exactly one
  module's state and nothing else (e.g. `d_event.rs`'s two functions) — a tighter,
  more idiomatic signature where it's actually true.

**Call-site convention during migration**: any call to a converted function from code
that hasn't been converted yet becomes `f(unsafe { game_state() }, args...)` — the
`unsafe` is confined to this one-line shim, not sprinkled through the function body
the way today's implicit-static access is. When the *caller* itself later gets
converted (receives `&mut GameState` from further up), the shim is deleted and the
already-in-hand reference is forwarded instead — mechanically:
`f(unsafe { game_state() }, ...)` → `f(state, ...)`.

The **final phase** of the whole track deletes `GAME_STATE`/`game_state()` entirely
once the frontier has reached the true root, replacing it with a real stack-local
`GameState` value living in `doomgeneric_Tick`'s caller. That last phase needs its own
design pass at the time (it crosses the `extern "C"` bin/lib boundary), and is called
out as a distinct milestone below rather than planned in detail now.

## Phasing strategy

Tiering is by **call-graph exposure**, not file-reference count (prior tracks' tiering
technique doesn't apply here): start at self-contained leaves with few external
callers, and defer the four atomic hub families until most of the individual
functions feeding them already have their own state cataloged into `GameState`.

**Phase 0 — infrastructure (no behavior change)**
Create `src/game_state.rs` with an initially near-empty `GameState`, the transitional
`static mut GAME_STATE`/`game_state()` accessor, and wire the module into `lib.rs`.
Pure scaffolding — build/warning-diff/Xvfb bar still applies and should show zero
functional difference.

**Phase 1 — pilot: `d_event.rs`**
Exactly the module the user proposed to start with, and the smallest real case that
exercises the whole pattern end-to-end:
- `events: [event_t; 64]`, `eventhead: i32`, `eventtail: i32` (`d_event.rs:33-41`) move
  into a new `DEventState` struct in `d_event.rs`, added as `GameState.d_event`.
- `D_PostEvent`/`D_PopEvent` (`d_event.rs:42-55`) become plain `fn(state: &mut
  DEventState, ...)` (narrow type is provably correct here — these two functions touch
  nothing else) — no longer `unsafe fn`.
- Exactly 3 call sites need updating, all outside this file, all not yet converted:
  `i_input.rs:197`, `i_input.rs:204` (`D_PostEvent(&raw mut event)` →
  `D_PostEvent(unsafe { &mut game_state().d_event }, &event)`), and `d_main.rs:245`
  (`D_PopEvent()` → `D_PopEvent(unsafe { &mut game_state().d_event })`).
- This phase validates: the `GameState`/sub-state split, the shim-at-uncoverted-caller
  pattern, and that the verification bar (build, warning-diff, Xvfb smoke test) still
  works the same way on this fundamentally different kind of change.

**Phase 2+ — outward from the pilot, smallest/most self-contained modules first**
Build a survey (same spirit as every prior track's survey script, adapted to this
track's shape): for each file with `static mut` items, count (a) how many *other*
files call its public functions, and (b) whether its own functions call into other
not-yet-converted modules' state. Order phases by rising exposure. Good early
candidates by raw `static mut` count from the current survey (`z_zone.rs`,
`w_file.rs`/`w_file_stdc.rs`, `st_lib.rs`, `i_timer.rs`, `i_cdmus.rs`,
`doomgeneric.rs`, `d_items.rs`, `m_argv.rs` — each with 1 static mut) — but the real
ordering criterion is external-call-site count and cross-module reads, not just how
many statics a file owns, so re-derive the actual tier order from the survey at the
start of phase 2 rather than trusting this list. `p_user.rs`/`p_plats.rs`/`p_ceilng.rs`
look small by static-mut count but are entangled with the `ThinkerFn` atomic family —
treat with the same caution as the hub families below, not as ordinary small modules.

**Deferred milestone — the four atomic hub families**
Each of these needs a single atomic, all-files-at-once edit (confirmed by
exploration), so schedule them as dedicated late-stage phases, after most of the
individual functions feeding them already have their own per-module state cataloged
into `GameState` (so the atomic edit is "add a parameter to an already-understood set
of functions," not "discover 40 functions' globals and add a parameter" in one PR):
1. `StateAction`/`ThinkerFn` (p_mobj.rs enum definitions + every `A_*`/`T_*` function
   + the two dispatch sites `p_mobj.rs:600`, `p_pspr.rs:71`, `p_tick.rs:44-60`).
2. The `PIT_*`/`PTR_*` iterator-callback family (per-iterator atomic: everything
   passed to `P_BlockLinesIterator` together, everything passed to
   `P_BlockThingsIterator` together, `P_TraverseIntercepts`'s `traverser_t` family
   together).
3. `m_menu.rs`'s `menuitem_t.routine` (all menu-item routines + the one dispatch site)
   and separately `menu_t.routine` (all per-menu draw routines + its dispatch site).

**Final milestone — collapse the transitional bridge**
Once the frontier reaches `doomgeneric_Tick`, design how a real (non-static)
`GameState` crosses the `extern "C"` `staticlib`/`[[bin]]` boundary (likely: change
`doomgeneric_Tick`'s signature to take a state pointer/reference, with the bin owning
the actual `GameState` value) and delete `GAME_STATE`/`game_state()`. This is the
natural point to reconsider the deferred DG_* trait, since `GameState` will by then be
a settled concrete type. Not designed in detail now — revisit when the frontier
actually gets there.

## Verification bar (unchanged from every prior track)

1. `cargo build` (lib) and `cargo build --bin doomgeneric_xlib`: 0 new errors.
2. Full sorted-warning-text diff against the established baseline log: empty (or, for
   phases that remove `unsafe fn`/add real borrows, an explicitly reviewed and
   justified change — converting `unsafe fn` to safe `fn` is expected to remove
   "unnecessary `unsafe`"-adjacent warnings if any exist; confirm via a clean-baseline
   comparison the same way Libc-extern-dedup landmine 2 was confirmed benign).
3. `cargo build --release --bin doomgeneric_xlib`, run under Xvfb against
   `/home/michael/Downloads/doom1.wad` with `-warp 1 1 -skill 3`, ≥3 consecutive clean
   runs, screenshot-verified. For phase 1 specifically: input events and menu/game
   responsiveness must be exercised (both feed through `D_PostEvent`/`D_PopEvent`).

## New risks specific to this track (watch for these from phase 1 onward)

- **Borrow-splitting at call sites that need two module-states at once**: with one
  unified `GameState`, a function needing simultaneous mutable access to two
  sub-states must destructure (`let GameState { d_event, g_game, .. } = state;`) to
  get disjoint field borrows, rather than calling two functions that each take
  `&mut GameState` in the same expression (impossible — only one `&mut GameState` can
  exist at a time). Original C code is sequential statement-by-statement, so this
  should almost always be resolvable by reordering into sequential narrow borrows;
  flag any case where it isn't.
- **Raw-pointer-returning functions** (e.g. `D_PopEvent() -> *mut event_t`) need their
  return type reconsidered once the backing array lives inside `&mut DEventState`
  rather than a `'static` global — a returned reference can't outlive the borrow of
  `state`. Check every caller's actual usage pattern (does it use the pointer
  immediately and discard it, or store it across other calls?) before deciding
  between keeping a raw pointer (still valid since the backing array is still
  logically `'static` for now, just reached differently) vs. changing to `Option<&mut
  event_t>` with a real lifetime.
- **Function-pointer storage sites not yet identified** may exist beyond the four
  known hub families — before converting any module, grep for its function names used
  as bare values (same check Track 5/9/13 already established) to catch a fifth hub
  early rather than discovering it mid-phase.
