use crate::src::d_player::player_t;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::game_state::game_state;
use crate::src::game_state::GameState;
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
pub unsafe fn P_RunThinkers(state: &mut GameState) {
    let mut currentthinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    currentthinker = state.p_tick.thinkercap.next as *mut thinker_t;
    while currentthinker != &raw mut state.p_tick.thinkercap {
        match (*currentthinker).function {
            ThinkerFn::Removed => {
                (*(*currentthinker).next).prev = (*currentthinker).prev;
                (*(*currentthinker).prev).next = (*currentthinker).next;
                Z_Free(
                    &mut state.z_zone,
                    currentthinker as *mut ::core::ffi::c_void,
                );
            }
            ThinkerFn::Paused | ThinkerFn::Unresolved => {}
            ThinkerFn::Mobj(f) => {
                let mobj_id = (*(currentthinker as *mut mobj_t)).id;
                f(state, mobj_id);
            }
            ThinkerFn::Ceiling(f) => f(state, currentthinker as *mut ceiling_t),
            ThinkerFn::Door(f) => f(state, currentthinker as *mut vldoor_t),
            ThinkerFn::Floor(f) => f(state, currentthinker as *mut floormove_t),
            ThinkerFn::Plat(f) => f(state, currentthinker as *mut plat_t),
            ThinkerFn::FireFlicker(f) => f(state, currentthinker as *mut fireflicker_t),
            ThinkerFn::LightFlash(f) => f(state, currentthinker as *mut lightflash_t),
            ThinkerFn::Strobe(f) => f(state, currentthinker as *mut strobe_t),
            ThinkerFn::Glow(f) => f(state, currentthinker as *mut glow_t),
        }
        currentthinker = (*currentthinker).next as *mut thinker_t;
    }
}
pub unsafe fn P_Ticker(state: &mut GameState) {
    let mut i: i32 = 0;
    if state.g_game.paused {
        return;
    }
    if !state.g_game.netgame
        && state.m_menu.menuactive
        && !state.g_game.demoplayback
        && state.g_game.players[state.g_game.consoleplayer as usize].viewz != 1 as i32
    {
        return;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        if state.g_game.playeringame[i as usize] != 0 {
            P_PlayerThink(
                &mut state.p_user,
                (&raw mut state.g_game.players as *mut player_t).offset(i as isize) as *mut player_t,
            );
        }
        i += 1;
    }
    P_RunThinkers(state);
    P_UpdateSpecials(state);
    P_RespawnSpecials(state);
    state.p_tick.leveltime += 1;
}
