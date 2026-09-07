use crate::src::d_player::player_t;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::game_state::game_state;
use crate::src::p_doors::vldoor_t;
use crate::src::p_lights::{fireflicker_t, glow_t, lightflash_t, strobe_t};
use crate::src::p_mobj::P_RespawnSpecials;
use crate::src::p_mobj::{mobj_t, thinker_s, thinker_t, ThinkerFn};
use crate::src::p_spec::P_UpdateSpecials;
use crate::src::p_spec::{ceiling_t, floormove_t, plat_t};
use crate::src::p_user::P_PlayerThink;
use crate::src::z_zone::Z_Free;
use ::libc;

pub struct PTickState {
    pub leveltime: i32,
    pub thinkercap: thinker_t,
}

impl PTickState {
    pub const fn new() -> Self {
        PTickState {
            leveltime: 0,
            thinkercap: thinker_s {
        prev: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
        next: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
        function: ThinkerFn::Paused,
    },
        }
    }
}


pub unsafe fn P_InitThinkers() {
    unsafe { game_state() }.p_tick.thinkercap.next = &raw mut unsafe { game_state() }.p_tick.thinkercap as *mut thinker_s;
    unsafe { game_state() }.p_tick.thinkercap.prev = unsafe { game_state() }.p_tick.thinkercap.next;
}
pub unsafe fn P_AddThinker(mut thinker: *mut thinker_t) {
    (*unsafe { game_state() }.p_tick.thinkercap.prev).next = thinker as *mut thinker_s;
    (*thinker).next = &raw mut unsafe { game_state() }.p_tick.thinkercap as *mut thinker_s;
    (*thinker).prev = unsafe { game_state() }.p_tick.thinkercap.prev;
    unsafe { game_state() }.p_tick.thinkercap.prev = thinker as *mut thinker_s;
}
pub unsafe fn P_RemoveThinker(mut thinker: *mut thinker_t) {
    (*thinker).function = ThinkerFn::Removed;
}
pub unsafe fn P_RunThinkers() {
    let mut currentthinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    currentthinker = unsafe { game_state() }.p_tick.thinkercap.next as *mut thinker_t;
    while currentthinker != &raw mut unsafe { game_state() }.p_tick.thinkercap {
        match (*currentthinker).function {
            ThinkerFn::Removed => {
                (*(*currentthinker).next).prev = (*currentthinker).prev;
                (*(*currentthinker).prev).next = (*currentthinker).next;
                Z_Free(
                    unsafe { &mut game_state().z_zone },
                    currentthinker as *mut ::core::ffi::c_void,
                );
            }
            ThinkerFn::Paused | ThinkerFn::Unresolved => {}
            ThinkerFn::Mobj(f) => f((*(currentthinker as *mut mobj_t)).id),
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
    if unsafe { game_state() }.g_game.paused {
        return;
    }
    if !unsafe { game_state() }.g_game.netgame
        && unsafe { game_state() }.m_menu.menuactive
        && !unsafe { game_state() }.g_game.demoplayback
        && unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .viewz
            != 1 as i32
    {
        return;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        if unsafe { game_state() }.g_game.playeringame[i as usize] != 0 {
            P_PlayerThink(
                unsafe { &mut game_state().p_user },
                (&raw mut unsafe { game_state() }.g_game.players as *mut player_t)
                    .offset(i as isize) as *mut player_t,
            );
        }
        i += 1;
    }
    P_RunThinkers();
    P_UpdateSpecials(unsafe { &mut game_state().p_switch });
    P_RespawnSpecials(unsafe { &mut game_state().p_mobj });
    unsafe { game_state() }.p_tick.leveltime += 1;
}
