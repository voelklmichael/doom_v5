#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-link-lib=X11");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
