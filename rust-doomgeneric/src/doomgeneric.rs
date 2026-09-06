use crate::src::m_argv::{myargv, M_FindResponseFile};
use crate::src::d_main::D_DoomMain;
use crate::src::doomdef::pixel_t;
use crate::src::stdint_types::size_t;
use libc::malloc;


extern "C" {
    fn DG_Init();
}
pub const DOOMGENERIC_RESX: i32 = 640;
pub const DOOMGENERIC_RESY: i32 = 400;
#[no_mangle]
pub static mut DG_ScreenBuffer: *mut pixel_t = ::core::ptr::null::<pixel_t>()
    as *mut pixel_t;
pub unsafe fn doomgeneric_Create(args: Vec<String>) {
    myargv = args
        .into_iter()
        .map(|arg| ::std::ffi::CString::new(arg).expect("argument contains a nul byte"))
        .collect();
    M_FindResponseFile();
    DG_ScreenBuffer = malloc(
        (DOOMGENERIC_RESX * DOOMGENERIC_RESY * 4 as i32) as size_t,
    ) as *mut pixel_t;
    DG_Init();
    D_DoomMain();
}
