use crate::src::d_items::weaponinfo;
use crate::src::p_mobj::{state_t, StateAction};
use crate::src::d_player::{player_t, PST_DEAD};
use crate::src::p_mobj::{mobj_t, pspdef_t};
use crate::src::p_mobj::P_SpawnPlayerMissile;
use crate::src::p_enemy::P_NoiseAlert;
use crate::src::p_map::P_LineAttack;
use crate::src::p_map::linetarget;
use crate::src::p_map::P_AimLineAttack;
use crate::src::p_inter::P_DamageMobj;
use crate::src::p_mobj::P_SetMobjState;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::info::states;
use crate::src::r_main::R_PointToAngle2;
use crate::src::m_random::P_Random;
use crate::src::p_tick::leveltime;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::p_mobj::MF_JUSTATTACKED;
use crate::src::sounds::{sfx_bfg, sfx_dshtgn, sfx_pistol, sfx_punch, sfx_sawful, sfx_sawhit, sfx_sawidl, sfx_sawup, sfx_shotgn};
use crate::src::d_ticcmd::BT_ATTACK;
use crate::src::d_player::pw_strength;
use crate::src::d_player::{NUMPSPRITES, ps_flash, ps_weapon};
use crate::src::p_mobj::{MT_BFG, MT_EXTRABFG, MT_PLASMA, MT_ROCKET, mobjtype_t};
use crate::src::p_mobj::statenum_t;
use crate::src::d_mode::{commercial, shareware};
use crate::src::d_player::{wp_bfg, wp_chaingun, wp_chainsaw, wp_fist, wp_missile, wp_nochange, wp_pistol, wp_plasma, wp_shotgun, wp_supershotgun};
use crate::src::d_player::{NUMAMMO, am_cell, am_clip, am_misl, am_noammo, am_shell, ammotype_t};
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::info::{S_CHAIN1, S_NULL, S_PLAY, S_PLAY_ATK1, S_PLAY_ATK2, S_SAW};


pub const NUMMOBJTYPES: mobjtype_t = 137;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const DEH_DEFAULT_BFG_CELLS_PER_SHOT: i32 = 40 as i32;
pub const deh_bfg_cells_per_shot: i32 = DEH_DEFAULT_BFG_CELLS_PER_SHOT;
pub const FRACBITS: i32 = 16 as i32;
pub const FRACUNIT: i32 = (1 as i32) << FRACBITS;
pub const FINEANGLES: i32 = 8192 as i32;
pub const FINEMASK: i32 = FINEANGLES - 1 as i32;
pub const ANG90: i32 = 0x40000000 as i32;
pub const ANG180: u32 = 0x80000000 as u32;
pub const MELEERANGE: i32 = 64 as i32 * FRACUNIT;
pub const MISSILERANGE: i32 = 32 as i32
    * 64 as i32 * FRACUNIT;
#[no_mangle]
pub unsafe extern "C" fn P_SetPsprite(
    mut player: *mut player_t,
    mut position: i32,
    mut stnum: statenum_t,
) {
    let mut psp: *mut pspdef_t = ::core::ptr::null_mut::<pspdef_t>();
    let mut state: *mut state_t = ::core::ptr::null_mut::<state_t>();
    psp = (&raw mut (*player).psprites as *mut pspdef_t).offset(position as isize)
        as *mut pspdef_t;
    loop {
        if stnum as u64 == 0 {
            (*psp).state = ::core::ptr::null_mut::<state_t>();
            break;
        } else {
            state = (&raw mut states as *mut state_t).offset(stnum as isize)
                as *mut state_t;
            (*psp).state = state;
            (*psp).tics = (*state).tics;
            if (*state).misc1 != 0 {
                (*psp).sx = ((*state).misc1 << FRACBITS) as fixed_t;
                (*psp).sy = ((*state).misc2 << FRACBITS) as fixed_t;
            }
            if let StateAction::Weapon(f) = (*state).action {
                f(player, psp);
                if (*psp).state.is_null() {
                    break;
                }
            }
            stnum = (*(*psp).state).nextstate;
            if !((*psp).tics == 0) {
                break;
            }
        }
    };
}
#[no_mangle]
pub static mut swingx: fixed_t = 0;
#[no_mangle]
pub static mut swingy: fixed_t = 0;
#[no_mangle]
pub unsafe extern "C" fn P_CalcSwing(mut player: *mut player_t) {
    let mut swing: fixed_t = 0;
    let mut angle: i32 = 0;
    swing = (*player).bob;
    angle = FINEANGLES / 70 as i32 * leveltime & FINEMASK;
    swingx = FixedMul(swing, finesine[angle as usize]);
    angle = FINEANGLES / 70 as i32 * leveltime
        + FINEANGLES / 2 as i32 & FINEMASK;
    swingy = -FixedMul(swingx, finesine[angle as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn P_BringUpWeapon(mut player: *mut player_t) {
    let mut newstate: statenum_t = S_NULL;
    if (*player).pendingweapon as u32
        == wp_nochange as i32 as u32
    {
        (*player).pendingweapon = (*player).readyweapon;
    }
    if (*player).pendingweapon as u32
        == wp_chainsaw as i32 as u32
    {
        S_StartSound(
            (*player).mo as *mut ::core::ffi::c_void,
            sfx_sawup as i32,
        );
    }
    newstate = weaponinfo[(*player).pendingweapon as usize].upstate as statenum_t;
    (*player).pendingweapon = wp_nochange;
    (*player).psprites[ps_weapon as i32 as usize].sy = (128
        as i32 * FRACUNIT) as fixed_t;
    P_SetPsprite(player, ps_weapon as i32, newstate);
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckAmmo(mut player: *mut player_t) -> bool {
    let mut ammo: ammotype_t = am_clip;
    let mut count: i32 = 0;
    ammo = weaponinfo[(*player).readyweapon as usize].ammo;
    if (*player).readyweapon as u32
        == wp_bfg as i32 as u32
    {
        count = deh_bfg_cells_per_shot;
    } else if (*player).readyweapon as u32
        == wp_supershotgun as i32 as u32
    {
        count = 2 as i32;
    } else {
        count = 1 as i32;
    }
    if ammo as u32
        == am_noammo as i32 as u32
        || (*player).ammo[ammo as usize] >= count
    {
        return true;
    }
    loop {
        if (*player).weaponowned[wp_plasma as i32 as usize]
            && (*player).ammo[am_cell as i32 as usize] != 0
            && gamemode as u32
                != shareware as i32 as u32
        {
            (*player).pendingweapon = wp_plasma;
        } else if (*player).weaponowned[wp_supershotgun as i32 as usize]
            && (*player).ammo[am_shell as i32 as usize]
                > 2 as i32
            && gamemode as u32
                == commercial as i32 as u32
        {
            (*player).pendingweapon = wp_supershotgun;
        } else if (*player).weaponowned[wp_chaingun as i32 as usize]
            && (*player).ammo[am_clip as i32 as usize] != 0
        {
            (*player).pendingweapon = wp_chaingun;
        } else if (*player).weaponowned[wp_shotgun as i32 as usize]
            && (*player).ammo[am_shell as i32 as usize] != 0
        {
            (*player).pendingweapon = wp_shotgun;
        } else if (*player).ammo[am_clip as i32 as usize] != 0 {
            (*player).pendingweapon = wp_pistol;
        } else if (*player).weaponowned[wp_chainsaw as i32 as usize]
        {
            (*player).pendingweapon = wp_chainsaw;
        } else if (*player).weaponowned[wp_missile as i32 as usize]
            && (*player).ammo[am_misl as i32 as usize] != 0
        {
            (*player).pendingweapon = wp_missile;
        } else if (*player).weaponowned[wp_bfg as i32 as usize]
            && (*player).ammo[am_cell as i32 as usize]
                > 40 as i32
            && gamemode as u32
                != shareware as i32 as u32
        {
            (*player).pendingweapon = wp_bfg;
        } else {
            (*player).pendingweapon = wp_fist;
        }
        if !((*player).pendingweapon as u32
            == wp_nochange as i32 as u32)
        {
            break;
        }
    }
    P_SetPsprite(
        player,
        ps_weapon as i32,
        weaponinfo[(*player).readyweapon as usize].downstate as statenum_t,
    );
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn P_FireWeapon(mut player: *mut player_t) {
    let mut newstate: statenum_t = S_NULL;
    if !P_CheckAmmo(player) {
        return;
    }
    P_SetMobjState((*player).mo, S_PLAY_ATK1);
    newstate = weaponinfo[(*player).readyweapon as usize].atkstate as statenum_t;
    P_SetPsprite(player, ps_weapon as i32, newstate);
    P_NoiseAlert((*player).mo, (*player).mo);
}
pub unsafe fn P_DropWeapon(mut player: *mut player_t) {
    P_SetPsprite(
        player,
        ps_weapon as i32,
        weaponinfo[(*player).readyweapon as usize].downstate as statenum_t,
    );
}
pub unsafe fn A_WeaponReady(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    let mut newstate: statenum_t = S_NULL;
    let mut angle: i32 = 0;
    if (*(*player).mo).state
        == (&raw mut states as *mut state_t)
            .offset(S_PLAY_ATK1 as i32 as isize) as *mut state_t
        || (*(*player).mo).state
            == (&raw mut states as *mut state_t)
                .offset(S_PLAY_ATK2 as i32 as isize) as *mut state_t
    {
        P_SetMobjState((*player).mo, S_PLAY);
    }
    if (*player).readyweapon as u32
        == wp_chainsaw as i32 as u32
        && (*psp).state
            == (&raw mut states as *mut state_t)
                .offset(S_SAW as i32 as isize) as *mut state_t
    {
        S_StartSound(
            (*player).mo as *mut ::core::ffi::c_void,
            sfx_sawidl as i32,
        );
    }
    if (*player).pendingweapon as u32
        != wp_nochange as i32 as u32
        || (*player).health == 0
    {
        newstate = weaponinfo[(*player).readyweapon as usize].downstate as statenum_t;
        P_SetPsprite(player, ps_weapon as i32, newstate);
        return;
    }
    if (*player).cmd.buttons as i32 & BT_ATTACK as i32 != 0
    {
        if (*player).attackdown == 0
            || (*player).readyweapon as u32
                != wp_missile as i32 as u32
                && (*player).readyweapon as u32
                    != wp_bfg as i32 as u32
        {
            (*player).attackdown = true_0;
            P_FireWeapon(player);
            return;
        }
    } else {
        (*player).attackdown = false_0;
    }
    angle = 128 as i32 * leveltime & FINEMASK;
    (*psp).sx = FRACUNIT + FixedMul((*player).bob, finecosine[angle as isize]);
    angle &= FINEANGLES / 2 as i32 - 1 as i32;
    (*psp).sy = 32 as fixed_t * FRACUNIT
        + FixedMul((*player).bob, finesine[angle as usize]);
}
pub unsafe fn A_ReFire(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    if (*player).cmd.buttons as i32 & BT_ATTACK as i32 != 0
        && (*player).pendingweapon as u32
            == wp_nochange as i32 as u32
        && (*player).health != 0
    {
        (*player).refire += 1;
        P_FireWeapon(player);
    } else {
        (*player).refire = 0 as i32;
        P_CheckAmmo(player);
    };
}
pub unsafe fn A_CheckReload(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    P_CheckAmmo(player);
}
pub unsafe fn A_Lower(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    (*psp).sy += FRACUNIT * 6 as i32;
    if (*psp).sy < 128 as i32 * FRACUNIT {
        return;
    }
    if (*player).playerstate as u32
        == PST_DEAD as i32 as u32
    {
        (*psp).sy = (128 as i32 * FRACUNIT) as fixed_t;
        return;
    }
    if (*player).health == 0 {
        P_SetPsprite(player, ps_weapon as i32, S_NULL);
        return;
    }
    (*player).readyweapon = (*player).pendingweapon;
    P_BringUpWeapon(player);
}
pub unsafe fn A_Raise(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    let mut newstate: statenum_t = S_NULL;
    (*psp).sy -= FRACUNIT * 6 as i32;
    if (*psp).sy > 32 as i32 * FRACUNIT {
        return;
    }
    (*psp).sy = (32 as i32 * FRACUNIT) as fixed_t;
    newstate = weaponinfo[(*player).readyweapon as usize].readystate as statenum_t;
    P_SetPsprite(player, ps_weapon as i32, newstate);
}
pub unsafe fn A_GunFlash(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    P_SetPsprite(
        player,
        ps_flash as i32,
        weaponinfo[(*player).readyweapon as usize].flashstate as statenum_t,
    );
}
pub unsafe fn A_Punch(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    let mut angle: angle_t = 0;
    let mut damage: i32 = 0;
    let mut slope: i32 = 0;
    damage = (P_Random() % 10 as i32 + 1 as i32)
        << 1 as i32;
    if (*player).powers[pw_strength as i32 as usize] != 0 {
        damage *= 10 as i32;
    }
    angle = (*(*player).mo).angle;
    angle = angle
        .wrapping_add((P_Random() - P_Random() << 18 as i32) as angle_t);
    slope = P_AimLineAttack((*player).mo, angle, MELEERANGE) as i32;
    P_LineAttack((*player).mo, angle, MELEERANGE, slope as fixed_t, damage);
    if !linetarget.is_null() {
        S_StartSound(
            (*player).mo as *mut ::core::ffi::c_void,
            sfx_punch as i32,
        );
        (*(*player).mo).angle = R_PointToAngle2(
            (*(*player).mo).x,
            (*(*player).mo).y,
            (*linetarget).x,
            (*linetarget).y,
        );
    }
}
pub unsafe fn A_Saw(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    let mut angle: angle_t = 0;
    let mut damage: i32 = 0;
    let mut slope: i32 = 0;
    damage = 2 as i32
        * (P_Random() % 10 as i32 + 1 as i32);
    angle = (*(*player).mo).angle;
    angle = angle
        .wrapping_add((P_Random() - P_Random() << 18 as i32) as angle_t);
    slope = P_AimLineAttack((*player).mo, angle, MELEERANGE + 1 as fixed_t)
        as i32;
    P_LineAttack(
        (*player).mo,
        angle,
        MELEERANGE + 1 as fixed_t,
        slope as fixed_t,
        damage,
    );
    if linetarget.is_null() {
        S_StartSound(
            (*player).mo as *mut ::core::ffi::c_void,
            sfx_sawful as i32,
        );
        return;
    }
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_sawhit as i32,
    );
    angle = R_PointToAngle2(
        (*(*player).mo).x,
        (*(*player).mo).y,
        (*linetarget).x,
        (*linetarget).y,
    );
    if angle.wrapping_sub((*(*player).mo).angle) > ANG180 {
        if (angle.wrapping_sub((*(*player).mo).angle) as i32)
            < -ANG90 / 20 as i32
        {
            (*(*player).mo).angle = angle
                .wrapping_add((ANG90 / 21 as i32) as angle_t);
        } else {
            (*(*player).mo).angle = (*(*player).mo)
                .angle
                .wrapping_sub((ANG90 / 20 as i32) as angle_t);
        }
    } else if angle.wrapping_sub((*(*player).mo).angle)
        > (ANG90 / 20 as i32) as angle_t
    {
        (*(*player).mo).angle = angle
            .wrapping_sub((ANG90 / 21 as i32) as angle_t);
    } else {
        (*(*player).mo).angle = (*(*player).mo)
            .angle
            .wrapping_add((ANG90 / 20 as i32) as angle_t);
    }
    (*(*player).mo).flags |= MF_JUSTATTACKED as i32;
}
unsafe extern "C" fn DecreaseAmmo(
    mut player: *mut player_t,
    mut ammonum: i32,
    mut amount: i32,
) {
    if ammonum < NUMAMMO as i32 {
        (*player).ammo[ammonum as usize] -= amount;
    } else {
        (*player).maxammo[(ammonum - NUMAMMO as i32) as usize] -= amount;
    };
}
pub unsafe fn A_FireMissile(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        1 as i32,
    );
    P_SpawnPlayerMissile((*player).mo, MT_ROCKET);
}
pub unsafe fn A_FireBFG(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        deh_bfg_cells_per_shot,
    );
    P_SpawnPlayerMissile((*player).mo, MT_BFG);
}
pub unsafe fn A_FirePlasma(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        1 as i32,
    );
    P_SetPsprite(
        player,
        ps_flash as i32,
        (weaponinfo[(*player).readyweapon as usize].flashstate
            + (P_Random() & 1 as i32)) as statenum_t,
    );
    P_SpawnPlayerMissile((*player).mo, MT_PLASMA);
}
pub static mut bulletslope: fixed_t = 0;
#[no_mangle]
pub unsafe extern "C" fn P_BulletSlope(mut mo: *mut mobj_t) {
    let mut an: angle_t = 0;
    an = (*mo).angle;
    bulletslope = P_AimLineAttack(mo, an, 16 as fixed_t * 64 as fixed_t * FRACUNIT);
    if linetarget.is_null() {
        an = an
            .wrapping_add(
                ((1 as i32) << 26 as i32) as angle_t,
            );
        bulletslope = P_AimLineAttack(mo, an, 16 as fixed_t * 64 as fixed_t * FRACUNIT);
        if linetarget.is_null() {
            an = an
                .wrapping_sub(
                    ((2 as i32) << 26 as i32) as angle_t,
                );
            bulletslope = P_AimLineAttack(
                mo,
                an,
                16 as fixed_t * 64 as fixed_t * FRACUNIT,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_GunShot(mut mo: *mut mobj_t, mut accurate: bool) {
    let mut angle: angle_t = 0;
    let mut damage: i32 = 0;
    damage = 5 as i32
        * (P_Random() % 3 as i32 + 1 as i32);
    angle = (*mo).angle;
    if !accurate {
        angle = angle
            .wrapping_add(
                (P_Random() - P_Random() << 18 as i32) as angle_t,
            );
    }
    P_LineAttack(mo, angle, MISSILERANGE, bulletslope, damage);
}
pub unsafe fn A_FirePistol(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_pistol as i32,
    );
    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        1 as i32,
    );
    P_SetPsprite(
        player,
        ps_flash as i32,
        weaponinfo[(*player).readyweapon as usize].flashstate as statenum_t,
    );
    P_BulletSlope((*player).mo);
    P_GunShot((*player).mo, (*player).refire == 0);
}
pub unsafe fn A_FireShotgun(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    let mut i: i32 = 0;
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_shotgn as i32,
    );
    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        1 as i32,
    );
    P_SetPsprite(
        player,
        ps_flash as i32,
        weaponinfo[(*player).readyweapon as usize].flashstate as statenum_t,
    );
    P_BulletSlope((*player).mo);
    i = 0 as i32;
    while i < 7 as i32 {
        P_GunShot((*player).mo, false);
        i += 1;
    }
}
pub unsafe fn A_FireShotgun2(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    let mut i: i32 = 0;
    let mut angle: angle_t = 0;
    let mut damage: i32 = 0;
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_dshtgn as i32,
    );
    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        2 as i32,
    );
    P_SetPsprite(
        player,
        ps_flash as i32,
        weaponinfo[(*player).readyweapon as usize].flashstate as statenum_t,
    );
    P_BulletSlope((*player).mo);
    i = 0 as i32;
    while i < 20 as i32 {
        damage = 5 as i32
            * (P_Random() % 3 as i32 + 1 as i32);
        angle = (*(*player).mo).angle;
        angle = angle
            .wrapping_add(
                (P_Random() - P_Random() << 19 as i32) as angle_t,
            );
        P_LineAttack(
            (*player).mo,
            angle,
            MISSILERANGE,
            bulletslope
                + ((P_Random() as fixed_t - P_Random() as fixed_t)
                    << 5 as i32),
            damage,
        );
        i += 1;
    }
}
pub unsafe fn A_FireCGun(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_pistol as i32,
    );
    if (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] == 0 {
        return;
    }
    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo as i32,
        1 as i32,
    );
    P_SetPsprite(
        player,
        ps_flash as i32,
        (*psp)
            .state
            .offset(weaponinfo[(*player).readyweapon as usize].flashstate as isize)
            .offset_from(
                (&raw mut states as *mut state_t)
                    .offset(S_CHAIN1 as i32 as isize) as *mut state_t,
            ) as i64 as statenum_t,
    );
    P_BulletSlope((*player).mo);
    P_GunShot((*player).mo, (*player).refire == 0);
}
pub unsafe fn A_Light0(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    (*player).extralight = 0 as i32;
}
pub unsafe fn A_Light1(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    (*player).extralight = 1 as i32;
}
pub unsafe fn A_Light2(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    (*player).extralight = 2 as i32;
}
pub unsafe fn A_BFGSpray(mut mo: *mut mobj_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut damage: i32 = 0;
    let mut an: angle_t = 0;
    i = 0 as i32;
    while i < 40 as i32 {
        an = (*mo)
            .angle
            .wrapping_sub((ANG90 / 2 as i32) as angle_t)
            .wrapping_add((ANG90 / 40 as i32 * i) as angle_t);
        P_AimLineAttack(
            (*mo).target as *mut mobj_t,
            an,
            16 as fixed_t * 64 as fixed_t * FRACUNIT,
        );
        if !linetarget.is_null() {
            P_SpawnMobj(
                (*linetarget).x,
                (*linetarget).y,
                (*linetarget).z + ((*linetarget).height >> 2 as i32),
                MT_EXTRABFG,
            );
            damage = 0 as i32;
            j = 0 as i32;
            while j < 15 as i32 {
                damage
                    += (P_Random() & 7 as i32) + 1 as i32;
                j += 1;
            }
            P_DamageMobj(
                linetarget,
                (*mo).target as *mut mobj_t,
                (*mo).target as *mut mobj_t,
                damage,
            );
        }
        i += 1;
    }
}
pub unsafe fn A_BFGsound(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_bfg as i32,
    );
}
pub unsafe fn P_SetupPsprites(mut player: *mut player_t) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < NUMPSPRITES as i32 {
        (*player).psprites[i as usize].state = ::core::ptr::null_mut::<state_t>();
        i += 1;
    }
    (*player).pendingweapon = (*player).readyweapon;
    P_BringUpWeapon(player);
}
pub unsafe fn P_MovePsprites(mut player: *mut player_t) {
    let mut i: i32 = 0;
    let mut psp: *mut pspdef_t = ::core::ptr::null_mut::<pspdef_t>();
    let mut state: *mut state_t = ::core::ptr::null_mut::<state_t>();
    psp = (&raw mut (*player).psprites as *mut pspdef_t)
        .offset(0 as i32 as isize) as *mut pspdef_t;
    i = 0 as i32;
    while i < NUMPSPRITES as i32 {
        state = (*psp).state;
        if !state.is_null() {
            if (*psp).tics != -(1 as i32) {
                (*psp).tics -= 1;
                if (*psp).tics == 0 {
                    P_SetPsprite(player, i, (*(*psp).state).nextstate);
                }
            }
        }
        i += 1;
        psp = psp.offset(1);
    }
    (*player).psprites[ps_flash as i32 as usize].sx = (*player)
        .psprites[ps_weapon as i32 as usize]
        .sx;
    (*player).psprites[ps_flash as i32 as usize].sy = (*player)
        .psprites[ps_weapon as i32 as usize]
        .sy;
}
