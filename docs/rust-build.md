# Building the Transpiled Rust Crate

This documents what was needed to get `rust-doomgeneric/` (produced by
`docs/c2rust-transpile.md`) to actually compile with `cargo build`. It does
not yet run correctly against a WAD file — see the follow-up doc for that.

## Toolchain

`rust-doomgeneric/rust-toolchain.toml` pins `nightly-2023-04-15`, which the
translated code requires for the unstable `c_variadic`, `extern_types`, and
`raw_ref_op` features. If that toolchain isn't installed yet:

```bash
rustup toolchain install nightly-2023-04-15
```

`rustup` will pick it up automatically via the `rust-toolchain.toml`
override whenever `cargo`/`rustc` run inside `rust-doomgeneric/`.

## Build

```bash
cd rust-doomgeneric
cargo build
```

The Rust translation itself compiled without changes — c2rust's output was
already valid Rust (module-for-module, function-for-function). The build
only failed at the **link** step:

```
undefined reference to `XOpenDisplay'
undefined reference to `XCreateSimpleWindow'
undefined reference to `XSelectInput'
...
```

c2rust translates C `extern` function declarations (from `X11/Xlib.h` etc.)
into Rust `extern "C"` blocks, but it has no way to know which system
library actually provides those symbols — that information lived in the
Makefile's `LIBS += -lm -lc -lX11`, which c2rust does not read. `libc` and
`libm` are linked by Rust's default C runtime setup, but `libX11` is not, so
it needs to be linked explicitly.

### Fix: link libX11 in build.rs

c2rust emits a stub `build.rs` with a comment showing how to add link flags.
The one-line fix, in `rust-doomgeneric/build.rs`:

```rust
#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-link-lib=X11");
}
```

After that, `cargo build` succeeds, producing warnings only (112 warnings:
mostly unnecessary `unsafe` blocks and one `clashing_extern_declarations`
between two conflicting hand-written prototypes of `A_ReFire` in the
original C — harmless, and present in the C original too) and no errors:

```
$ cargo build
   ...
warning: `rust_doomgeneric` (lib) generated 112 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 14.78s
```

The resulting binary is `target/debug/doomgeneric_xlib` (named after the
translation unit containing `main()`, per `[[bin]]` in `Cargo.toml`).

## Result

- `cargo build` succeeds from a clean checkout with no source changes beyond
  the one `println!` added to `build.rs`.
- The binary has not yet been run against a WAD file; see the next doc for
  what running it needs.
