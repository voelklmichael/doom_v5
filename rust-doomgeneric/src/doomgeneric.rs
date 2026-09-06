use crate::src::m_argv::{myargv, M_FindResponseFile};
use crate::src::d_main::D_DoomMain;


extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn DG_Init();
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type pixel_t = uint32_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const DOOMGENERIC_RESX: i32 = 640 as i32;
pub const DOOMGENERIC_RESY: i32 = 400 as i32;
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
