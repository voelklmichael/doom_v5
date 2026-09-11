# Known Deviations from Upstream doomgeneric

## Context

`rust-doomgeneric` is a c2rust transpile of doomgeneric, then converted phase by phase
toward idiomatic Rust (see `track16-gamestate-plan.md`, `track17-arena-indices-idea.md`).
Almost all of that work is behavior-preserving: same logic, different representation.
This document tracks the exceptions — places where a conversion phase deliberately
changed *observable runtime behavior*, not just internal structure, usually because
reproducing the original C behavior exactly would have meant fighting the type system
or threading global state through code that has no real need for it.

Each entry should say what changed, why, and what was explicitly given up, so nobody
mistakes a deviation for a bug or tries to "fix" it back to C parity without knowing
the tradeoff was intentional.

## `I_Error`: replaced with a bare Rust panic (2026-09-11)

**What changed**: `I_Error(message)` used to reimplement C's fatal-error path by hand —
print to `stderr`, run every `atexit`-registered handler flagged `run_on_error`, show a
Zenity GUI error dialog on Linux desktops (unless `-nogui`), guard against recursive
calls via an `already_quitting` flag, then `exit(-1)`. It now does exactly this:

```rust
pub unsafe fn I_Error(message: &str) -> ! {
    panic!("{}", message);
}
```

All ~110 call sites across the codebase are untouched — every one of them just calls
`I_Error(&format!(...))` or `I_Error("literal")` exactly as before.

**Why**: `I_Error` was the last of the codebase's three whole-codebase-fanout utility
hubs left over from the `GameState` bridge-collapse track (see
`track16-gamestate-plan.md`). Converting it to thread a real `&mut GameState` parameter
like its two
siblings (`S_StartSound`, `W_CacheLumpNum`/`W_CacheLumpName`) turned out to be much
more tractable than earlier tracked (82% of call sites already had `state` in scope
after those two phases), but the remaining handful required either widening more leaf
functions or accepting a real structural blocker: `G_CheckDemoStatus` is registered via
`I_AtExit` as a raw C-ABI `unsafe extern "C" fn() -> ()`, which cannot carry a `&mut
GameState` argument without breaking that callback contract. Rather than solve that,
the user asked whether `I_Error` could just become a panic instead — which sidesteps
the whole problem, since `panic!` doesn't need any state at all.

**What was given up**:
- The 3 `run_on_error`-flagged `atexit` cleanup handlers (`D_QuitNetGame` — network
  disconnect notice, `S_Shutdown` — audio device teardown, `StatDump` — crash-time
  stats dump) no longer run when `I_Error` fires. Normal quit (`I_Quit`, unrelated
  code path) still runs all registered handlers unconditionally — only the
  crash/error path lost this.
- The Zenity GUI error popup (and the `-nogui` flag that suppressed it) is gone. A
  crash now only prints Rust's standard panic message to stderr.
- The recursive-`I_Error`-call warning guard is gone (a panic inside a panic's unwind
  path aborts the process instead, which is arguably a reasonable outcome on its own).

**Removed as a result** (dead once `I_Error`'s body no longer needs them):
`ZenityErrorBox`/`EscapeShellString`/`ZenityAvailable`/`ZENITY_BINARY`,
`ISystemState::already_quitting`, `ISystemState::zenity_errorboxpath_size`, and a
handful of now-unreachable `return`/assignment statements immediately after
`I_Error(...)` calls in `g_game.rs`/`m_misc.rs`/`p_enemy.rs` (the compiler's own
`unreachable_code` lint caught these once `I_Error`'s return type became `!`).
