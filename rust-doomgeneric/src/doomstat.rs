use crate::src::d_mode::GameVersion;
use crate::src::d_mode::{doom, GameMission_t};
use crate::src::d_mode::{indetermined, GameMode_t};
pub struct DoomstatState {
    pub gamemode: GameMode_t,
    pub gamemission: GameMission_t,
    pub gameversion: GameVersion,
    pub gamedescription: *mut ::core::ffi::c_char,
    pub modifiedgame: bool,
}
impl DoomstatState {
    pub const fn new() -> Self {
        DoomstatState {
            gamemode: indetermined,
            gamemission: doom,
            gameversion: GameVersion::final2,
            gamedescription: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            modifiedgame: false,
        }
    }
}
