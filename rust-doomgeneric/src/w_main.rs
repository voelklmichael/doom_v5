use crate::src::m_argv::{myargv, M_CheckParmWithArgs};
use crate::src::w_file::wad_file_t;
extern "C" {
    fn D_TryFindWADByName(
        filename: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn W_AddFile(filename: *mut ::core::ffi::c_char) -> *mut wad_file_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type boolean = ::core::ffi::c_uint;
pub type byte = uint8_t;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn W_ParseCommandLine() -> boolean {
    let mut modifiedgame: boolean = false_0 as boolean;
    let mut p: ::core::ffi::c_int = 0;
    p = M_CheckParmWithArgs("-file", 1 as ::core::ffi::c_int);
    if p != 0 {
        modifiedgame = true_0 as boolean;
        loop {
            p += 1;
            if !(p != myargv.len() as ::core::ffi::c_int
                && myargv[p as usize].as_bytes().first() != Some(&b'-'))
            {
                break;
            }
            let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            filename = D_TryFindWADByName(
                myargv[p as usize].as_ptr() as *mut ::core::ffi::c_char,
            );
            printf(
                b" adding %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
            W_AddFile(filename);
        }
    }
    return modifiedgame;
}
