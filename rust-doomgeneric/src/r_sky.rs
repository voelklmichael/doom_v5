use crate::src::game_state::game_state;
use crate::src::m_fixed::FRACUNIT;

pub struct RSkyState {
    pub skyflatnum: i32,
    pub skytexture: i32,
    pub skytexturemid: i32,
}

impl RSkyState {
    pub const fn new() -> Self {
        RSkyState {
            skyflatnum: 0,
            skytexture: 0,
            skytexturemid: 0,
        }
    }
}

pub unsafe fn R_InitSkyMap() {
    unsafe { game_state() }.r_sky.skytexturemid = 100 as i32 * FRACUNIT;
}
