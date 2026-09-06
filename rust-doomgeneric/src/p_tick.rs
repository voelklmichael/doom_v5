use ::libc;
use crate::src::p_mobj::{thinker_s, thinker_t, ThinkerFn, mobj_t};
use crate::src::p_spec::{ceiling_t, floormove_t, plat_t};
use crate::src::p_doors::vldoor_t;
use crate::src::p_lights::{fireflicker_t, lightflash_t, strobe_t, glow_t};
use crate::src::d_player::{player_t};
use crate::src::p_user::P_PlayerThink;
use crate::src::p_mobj::P_RespawnSpecials;
use crate::src::p_spec::P_UpdateSpecials;
use crate::src::g_game::paused;
use crate::src::m_menu::menuactive;
use crate::src::g_game::demoplayback;
use crate::src::g_game::playeringame;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::players;
use crate::src::z_zone::Z_Free;
use crate::src::doomdef::MAXPLAYERS;


pub static mut leveltime: i32 = 0;
pub static mut thinkercap: thinker_t = thinker_s {
    prev: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
    next: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
    function: ThinkerFn::Paused,
};
pub unsafe fn P_InitThinkers() {
    thinkercap.next = &raw mut thinkercap as *mut thinker_s;
    thinkercap.prev = thinkercap.next;
}
pub unsafe fn P_AddThinker(mut thinker: *mut thinker_t) {
    (*thinkercap.prev).next = thinker as *mut thinker_s;
    (*thinker).next = &raw mut thinkercap as *mut thinker_s;
    (*thinker).prev = thinkercap.prev;
    thinkercap.prev = thinker as *mut thinker_s;
}
pub unsafe fn P_RemoveThinker(mut thinker: *mut thinker_t) {
    (*thinker).function = ThinkerFn::Removed;
}
#[no_mangle]
pub unsafe extern "C" fn P_AllocateThinker(mut thinker: *mut thinker_t) {}
#[no_mangle]
pub unsafe extern "C" fn P_RunThinkers() {
    let mut currentthinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    currentthinker = thinkercap.next as *mut thinker_t;
    while currentthinker != &raw mut thinkercap {
        match (*currentthinker).function {
            ThinkerFn::Removed => {
                (*(*currentthinker).next).prev = (*currentthinker).prev;
                (*(*currentthinker).prev).next = (*currentthinker).next;
                Z_Free(currentthinker as *mut ::core::ffi::c_void);
            }
            ThinkerFn::Paused | ThinkerFn::Unresolved => {}
            ThinkerFn::Mobj(f) => f(currentthinker as *mut mobj_t),
            ThinkerFn::Ceiling(f) => f(currentthinker as *mut ceiling_t),
            ThinkerFn::Door(f) => f(currentthinker as *mut vldoor_t),
            ThinkerFn::Floor(f) => f(currentthinker as *mut floormove_t),
            ThinkerFn::Plat(f) => f(currentthinker as *mut plat_t),
            ThinkerFn::FireFlicker(f) => f(currentthinker as *mut fireflicker_t),
            ThinkerFn::LightFlash(f) => f(currentthinker as *mut lightflash_t),
            ThinkerFn::Strobe(f) => f(currentthinker as *mut strobe_t),
            ThinkerFn::Glow(f) => f(currentthinker as *mut glow_t),
        }
        currentthinker = (*currentthinker).next as *mut thinker_t;
    }
}
pub unsafe fn P_Ticker() {
    let mut i: i32 = 0;
    if paused {
        return;
    }
    if !netgame && menuactive && !demoplayback
        && players[consoleplayer as usize].viewz != 1 as i32
    {
        return;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            P_PlayerThink(
                (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t,
            );
        }
        i += 1;
    }
    P_RunThinkers();
    P_UpdateSpecials();
    P_RespawnSpecials();
    leveltime += 1;
}
