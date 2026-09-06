use crate::src::m_argv::{myargv, M_CheckParmWithArgs};
use crate::src::d_iwad::D_TryFindWADByName;
use crate::src::w_wad::W_AddFile;
use libc::printf;

pub unsafe fn W_ParseCommandLine() -> bool {
    let mut modifiedgame: bool = false;
    let mut p: i32 = 0;
    p = M_CheckParmWithArgs("-file", 1 as i32);
    if p != 0 {
        modifiedgame = true;
        loop {
            p += 1;
            if !(p != myargv.len() as i32
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
