use crate::src::d_iwad::D_TryFindWADByName;
use crate::src::game_state::GameState;
use crate::src::m_argv::M_CheckParmWithArgs;
use crate::src::w_wad::W_AddFile;
use libc::printf;

pub unsafe fn W_ParseCommandLine(state: &mut GameState) -> bool {
    let mut modifiedgame: bool = false;
    let mut p: i32 = 0;
    p = M_CheckParmWithArgs(state, "-file", 1 as i32);
    if p != 0 {
        modifiedgame = true;
        loop {
            p += 1;
            if !(p != state.m_argv.myargv.len() as i32
                && state.m_argv.myargv[p as usize].as_bytes().first() != Some(&b'-'))
            {
                break;
            }
            let mut filename: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            filename = D_TryFindWADByName(
                &mut state.d_iwad,
                state.m_argv.myargv[p as usize].as_ptr() as *mut ::core::ffi::c_char,
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
