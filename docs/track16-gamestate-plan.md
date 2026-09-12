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

## Final status (2026-09-11)

The bridge-collapse sub-track (BC1–BC94, run as a distinct effort after this plan's
original phase list finished — see git log for `Bridge-collapse BC*` commits) threaded
real `&mut GameState` parameters through essentially the entire codebase, including
every function-pointer-hub family this codebase has (`loop_interface_t`,
`menu_s.routine`/`menuitem_t.routine`, `f_wipe`'s wipe table, `StateAction`/
`ThinkerFn`, and — initially thought permanently blocked, until BC94 — `atexit_func_t`)
and all three whole-codebase-fanout utility hubs (`S_StartSound`, `W_CacheLumpNum`/
`W_CacheLumpName`, the `V_DrawPatch` family). `I_Error` was replaced with a bare
`panic!` instead of being threaded (see `known-deviations.md`).

**The `atexit_func_t` hub initially looked like a genuine C-ABI wall** (BC87/BC93 both
documented it as permanent): the 6 registered callbacks (`G_CheckDemoStatus`,
`D_QuitNetGame`, `D_Endoom`, `M_SaveDefaults`, `StatDump`, `S_Shutdown`) are stored as
`unsafe extern "C" fn() -> ()` in `atexit_listentry_t`. But BC94 revisited this and
found the `extern "C"` was never a real FFI boundary here — same reasoning already
established for `doomgeneric_Tick` back in BC-1 (this crate has zero `.c`/`.h` files).
Widened `atexit_func_t` to `Option<unsafe extern "C" fn(&mut GameState) -> ()>` and all
6 registrants along with it (3 were empty no-op stubs, trivial; `G_CheckDemoStatus` was
the real work, 40 internal `game_state()` calls). Along the way, found and fixed a real
landmine: `d_main.rs`/`d_net.rs` resolved `G_CheckDemoStatus`/`M_SaveDefaults`/
`StatDump` via stale c2rust `extern "C" { fn ... }` forward-declarations instead of real
`use` imports — exactly the "extern C fn-pointer trap" pattern this project's gotchas
memory already warns about. Left alone, widening the real definitions would have made
these declarations silently mismatched (UB, not a caught compile error). Replaced with
proper imports.

**Lesson**: don't trust an earlier phase's "permanently blocked" verdict without
re-checking the actual constraint. BC87/BC93 were right that these were `extern "C"`
zero-arg callbacks, but wrong that this crate has any real reason to keep them
C-ABI-shaped — the same "zero `.c`/`.h` files" fact that unblocked `doomgeneric_Tick`
in BC-1 unblocks any `extern "C"` boundary in this codebase, hub or not. Before
accepting a "permanent exception" claim (in this doc or in memory), ask why the
constraint is real, not just that it's labeled `extern "C"`.

A final full-codebase audit (BC93, revised by BC94) individually verified every
remaining `game_state()` call site and confirmed each falls into one of a small number
of genuinely permanent categories:

- **Confirmed dead code**, left in place untouched (this track's consistent precedent
  throughout, e.g. BC22's Heretic/Hexen/Strife binds): `m_controls.rs`'s Heretic/Hexen/
  Strife bind functions, `memio.rs`'s entire MEMFILE abstraction, `d_loop.rs`'s
  `D_Disconnected`/`D_ReceiveTic`, `i_scale.rs`'s `GenerateStretchTable` cluster,
  `p_pspr.rs`'s `P_CalcSwing`, `m_misc.rs`'s `M_ReadFile`, `m_argv.rs`'s
  `M_GetExecutableName`, `am_map.rs`'s `AM_updateLightLev`, `g_game.rs`'s
  `G_InitPlayer`.
- **Other C-ABI-bound hub payloads NOT revisited** (each would need its own check like
  `atexit_func_t` got, but the ripple is bigger and nobody's done that check yet):
  `w_file_stdc.rs`'s `W_StdC_OpenFile`/`CloseFile`/`Read` (`wad_file_class_t`'s
  fields), `st_lib.rs`'s `ST_loadCallback` / `wi_stuff.rs`'s `WI_loadCallback`
  (`load_callback_t`), `d_main.rs`'s `D_GrabMouseCallback` (`grabmouse_callback_t`,
  whose setter is a no-op stub, so this one is also dead-in-practice regardless).
- **Per-file `run_static_initializers`-style constructor blocks**: `am_map.rs`'s
  `cheat_amap`, `st_stuff.rs`'s `cheat_clev`/`cheat_mypos`/etc. — a long-standing
  const-fn-construction exception (see the doom_v5_known_gotchas memory / BC10's
  landmine note), not related to threading at all.
- **One deliberately deferred large-fanout cluster**: `w_wad.rs`'s
  `W_CheckNumForName`/`W_GetNumForName` (19 and 15 external call sites across 5-6
  files respectively) and the leaf functions tightly coupled to them
  (`W_AddFile`/`ExtendLumpInfo`/`W_LumpLength`/`W_ReleaseLumpNum`/etc.) — same
  S_StartSound-scale class the other three hubs were, deliberately left for a future
  dedicated phase if ever revisited.
- **Two deliberate low-value shims** (poor effort/benefit — widening would ripple a
  whole dispatch table or narrow-substate call chain for one field read):
  `i_scale.rs`'s `I_Stretch5x` `-scanline` check (the `screen_mode_t.DrawScreen` hub),
  `v_video.rs`'s `V_LoadTintTable`/`V_LoadXlaTable`.
- **The one legitimate root**: `doomgeneric_xlib.rs`'s single `game_state()` call that
  constructs the initial state before the tick loop begins — this is what `game_state()`
  exists to bootstrap, not a leftover.

**Conclusion (superseded 2026-09-12, see below): `GAME_STATE`/`OnceLock`/`game_state()`
still cannot be deleted today** — this turned out to be wrong; kept for history.

## Final status (2026-09-12, BC95/BC96)

The "open question" above was answered the very next session: the user asked to
reopen the track, and applying BC94's own "is this a real FFI boundary?" check to
every remaining category found none of them were actually permanent.

- **BC95**: deleted `i_scale.rs` outright (confirmed zero callers for its entire
  ~1900-line scale-driver apparatus, including the `screen_mode_t.DrawScreen` hub
  that the `-scanline` shim was deferred on — moot once the whole file was gone).
  Converted the remaining dead-code list from the audit above
  (`m_controls.rs`'s Heretic/Hexen/Strife binds, `d_loop.rs`'s
  `D_Disconnected`/`D_ReceiveTic`, `p_pspr.rs`'s `P_CalcSwing`, `am_map.rs`'s
  `AM_updateLightLev`, `g_game.rs`'s `G_InitPlayer`, `d_main.rs`'s
  `D_GrabMouseCallback`) and the two low-value shims (`v_video.rs`'s
  `V_LoadTintTable`/`V_LoadXlaTable`) to take state, same as any other function.
  Widened `load_callback_t` (st_stuff.rs/wi_stuff.rs) and `wad_file_class_t`
  (w_file.rs/w_file_stdc.rs) — neither was a real FFI boundary either.
- **BC96**: converted the one deliberately-deferred large-fanout cluster,
  `w_wad.rs`'s `W_CheckNumForName`/`W_GetNumForName` and its tightly-coupled
  leaves (`W_AddFile`/`ExtendLumpInfo`/`W_LumpLength`/`W_ReadLump`/
  `W_ReleaseLumpNum`/`W_ReleaseLumpName`/`W_GenerateHashTable`/
  `W_CheckCorrectIWAD`) — same S_StartSound-scale shape as the other three hubs,
  ~55 external call sites across 13 files, almost all one-line fixes.
- The per-file `run_static_initializers`-style const-fn constructor blocks
  (`am_map.rs`'s `cheat_amap`, etc.) were never actually blockers — they don't
  call `game_state()` at all, they were just listed alongside the real
  categories in the audit above.
- **The "one legitimate root" (`doomgeneric_xlib.rs`'s bootstrap call) turned out
  not to exist by this point** — `main()` already called `init_game_state` and
  used its return value directly, never the bare `game_state()` accessor.

With every category converted, `game_state()` had zero remaining callers anywhere
(verified by a full-codebase grep, comments excluded). Deleted `game_state()` and
the `GAME_STATE: OnceLock` static entirely. `init_game_state` now builds the
`GameState` and obtains its `&'static mut` via `Box::leak(Box::new(...))` instead
of a lazily-initialized static — simpler, and needs no unsafe accessor at all.

**Lesson**: a "cannot be deleted" or "permanent exception" conclusion is a
snapshot of what nobody has re-checked yet, not a proof. The 2026-09-11 closure's
own text acknowledged this ("don't assume... without applying the same check
first") but still led with "cannot be deleted today" as the headline — the doc
should have led with the open question, not the stale-by-construction conclusion.
