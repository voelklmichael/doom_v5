use crate::src::d_mode::{doom, GameMission_t};
use crate::src::d_mode::{exe_final2, GameVersion_t};
use crate::src::d_mode::{indetermined, GameMode_t};
pub static mut gamemode: GameMode_t = indetermined;
pub static mut gamemission: GameMission_t = doom;
pub static mut gameversion: GameVersion_t = exe_final2;
pub static mut gamedescription: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
pub static mut modifiedgame: bool = false;
