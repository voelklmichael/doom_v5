# Running the Transpiled Rust Port Against a WAD

This documents what was needed to get the crate built in
`docs/rust-build.md` to actually run against a real IWAD via `cargo run`,
instead of crashing.

## Run

```bash
cd rust-doomgeneric
cargo run --release -- -iwad /path/to/doom1.wad
```

`--release` matters here (see "Integer overflow" below). For example:

```bash
cargo run --release -- -iwad /home/michael/Downloads/Doom1.WAD
```

Like the C version (`docs/build-and-run-c.md`), this needs an X11 display
(`$DISPLAY` set) and a legally obtained IWAD, which is not included in this
repository.

## What was broken, and why

c2rust translates each C expression to Rust as literally as possible. That
is correct C semantics, but Doom's renderer relies on a few classic C idioms
that are technically undefined behavior and have no direct Rust equivalent
in *safe*, bounds-checked form. Three issues showed up, all in the
renderer/WAD-loading code path, all with the same root cause: C code
deliberately reading or writing outside an array's declared bounds,
relying on how neighboring fields or heap allocations happen to sit in
memory. Rust's arrays are bounds-checked (both in debug and release), so
each one turned into a runtime panic.

### 1. `patch_t::columnofs` is a fake fixed-size array

`v_patch.h` declares:

```c
typedef struct {
    ...
    int columnofs[8];  // only [width] used
} patch_t;
```

The comment says it all: `patch_t` is read directly out of WAD lump bytes,
and `columnofs` actually has `width` elements trailing the struct in
memory — 8 is just a placeholder so the struct compiles. C code indexes
past 8 constantly; c2rust translated the field as a real `[c_int; 8]` Rust
array, so any index `>= 8` panicked
(`index out of bounds: the len is 8 but the index is 8`, first hit in
`r_data.rs`).

Fixed every read site (`r_data.rs`, `r_things.rs`, `v_video.rs`,
`f_finale.rs`) to do the same pointer-offset arithmetic C does, instead of
indexing the fixed-size array, e.g.:

```rust
// before (panics once idx >= 8)
(*patch).columnofs[idx as usize]

// after
*(&raw const (*patch).columnofs as *const ::core::ffi::c_int).offset(idx as isize)
```

### 2. Fixed-point overflow in game logic (debug builds only)

`p_mobj.rs` panicked with `attempt to multiply with overflow`. Doom's fixed-
point math (`FRACBITS`/`FixedMul`-style code) intentionally overflows
`i32` in places and relies on wrapping — normal, expected behavior for this
codebase, not a bug. Rust's debug profile inserts overflow checks that
panic; the release profile does not (it wraps silently, matching C's
behavior on this platform). Building with `cargo build --release` /
`cargo run --release` was sufficient here — no source change needed for
this one.

### 3. `visplane_t::top`/`bottom` sentinel padding

`r_defs.h` declares:

```c
typedef struct {
    ...
    byte pad1;
    byte top[SCREENWIDTH];
    byte pad2;
    byte pad3;
    byte bottom[SCREENWIDTH];
    byte pad4;
} visplane_t;
```

The comment above it — `// leave pads for [minx-1]/[maxx+1]` — explains the
trick: `R_DrawPlanes` writes a sentinel at `top[maxx+1]` and reads
`top[minx-1]`/`bottom[minx-1]`, one byte outside the declared 320-element
arrays, deliberately landing in the adjacent `pad1`/`pad2` byte fields
(this only works because the struct is laid out with `#[repr(C)]`, which
c2rust does emit). Same fix as `columnofs`: the specific out-of-range
accesses in `r_plane.rs` (`R_DrawPlanes`) now go through raw pointer
arithmetic instead of array indexing; in-bounds accesses elsewhere in the
same function were left as normal, safely bounds-checked indexing.

## Verification

Ran the release binary against the shareware `doom1.wad` on an X11 desktop
session and confirmed:

- The `DOOM Shareware` window opens.
- The title screen (DOOM logo, marine artwork, `id Software` banner)
  renders correctly — confirmed visually via a screenshot of the running
  window.
- The process runs continuously (tested well past the demo/title sequence)
  without panicking.

No gameplay/input verification (moving, shooting, menu navigation) was
done beyond this — the scope here was "boots and renders", not full parity
with the C version.
