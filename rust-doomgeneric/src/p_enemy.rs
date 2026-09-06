use crate::src::p_mobj::{thinker_t, mobjinfo_t, sector_t, line_s, ST_HORIZONTAL, vertex_t, line_t};
use crate::src::d_player::{player_t};
use crate::src::p_mobj::{mobj_s, mobj_t, pspdef_t};
use crate::src::i_system::I_Error;
use crate::src::p_mobj::P_SpawnMissile;
use crate::src::p_map::floatok;
use crate::src::p_map::tmfloorz;
use crate::src::p_map::spechit;
use crate::src::p_map::numspechit;
use crate::src::p_map::P_RadiusAttack;
use crate::src::p_map::P_TeleportMove;
use crate::src::p_map::P_TryMove;
use crate::src::p_map::P_LineAttack;
use crate::src::p_maputl::P_LineOpening;
use crate::src::p_maputl::P_BlockThingsIterator;
use crate::src::p_maputl::openrange;
use crate::src::p_mobj::P_SpawnPuff;
use crate::src::p_mobj::P_SubstNullMobj;
use crate::src::p_sight::P_CheckSight;
use crate::src::p_switch::P_UseSpecialLine;
use crate::src::d_main::fastparm;
use crate::src::g_game::G_ExitLevel;
use crate::src::p_doors::EV_DoDoor;
use crate::src::p_floor::EV_DoFloor;
use crate::src::p_map::P_CheckPosition;
use crate::src::p_map::P_AimLineAttack;
use crate::src::p_maputl::P_AproxDistance;
use crate::src::p_maputl::P_UnsetThingPosition;
use crate::src::d_loop::gametic;
use crate::src::p_inter::P_DamageMobj;
use crate::src::p_maputl::P_SetThingPosition;
use crate::src::p_setup::bmaporgx;
use crate::src::p_setup::bmaporgy;
use crate::src::p_tick::thinkercap;
use crate::src::g_game::gameskill;
use crate::src::info::mobjinfo;
use crate::src::p_mobj::P_SetMobjState;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::r_main::validcount;
use crate::src::g_game::gameepisode;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::g_game::gamemap;
use crate::src::p_setup::sides;
use crate::src::r_main::R_PointToAngle2;
use crate::src::g_game::playeringame;
use crate::src::m_random::P_Random;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::p_mobj::{MF_AMBUSH, MF_CORPSE, MF_FLOAT, MF_INFLOAT, MF_JUSTATTACKED, MF_JUSTHIT, MF_SHADOW, MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID};
use crate::src::sounds::{sfx_barexp, sfx_bgdth1, sfx_bgsit1, sfx_boscub, sfx_bosdth, sfx_bospit, sfx_bospn, sfx_bossit, sfx_bspwlk, sfx_claw, sfx_dbcls, sfx_dbload, sfx_dbopn, sfx_flame, sfx_flamst, sfx_hoof, sfx_manatk, sfx_metal, sfx_pdiehi, sfx_pistol, sfx_pldeth, sfx_podth1, sfx_posit1, sfx_shotgn, sfx_skepch, sfx_skeswg, sfx_slop, sfx_telept, sfx_vilatk};
use crate::src::p_mobj::{MT_ARACHPLAZ, MT_BABY, MT_BOSSTARGET, MT_BRUISER, MT_BRUISERSHOT, MT_CYBORG, MT_FATSHOT, MT_FATSO, MT_FIRE, MT_HEAD, MT_HEADSHOT, MT_KNIGHT, MT_PAIN, MT_PLAYER, MT_ROCKET, MT_SERGEANT, MT_SHADOWS, MT_SKULL, MT_SMOKE, MT_SPAWNFIRE, MT_SPAWNSHOT, MT_SPIDER, MT_TRACER, MT_TROOP, MT_TROOPSHOT, MT_UNDEAD, MT_VILE, mobjtype_t};
use crate::src::p_mobj::{ThinkerFn, statenum_t};
use crate::src::d_mode::commercial;
use crate::src::d_mode::exe_ultimate;
use crate::src::d_mode::{sk_easy, sk_nightmare};
use crate::src::p_doors::{vld_blazeOpen, vld_open};
use crate::src::p_floor::{lowerFloorToLowest, raiseToTexture};
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::doomdef::boolean;

use crate::src::p_pspr::A_ReFire;
use crate::src::info::{S_BRAINEXPLODE1, S_NULL, S_VILE_HEAL1};
use crate::src::doomdef::NULL;
use crate::src::doomdef::true_0;
use crate::src::doomdef::false_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::m_fixed::FRACUNIT;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG180;
use crate::src::tables::ANG90;
use crate::src::tables::ANG270;
use crate::src::p_spec::ML_TWOSIDED;
use crate::src::p_mobj::FLOATSPEED;
use crate::src::p_maputl::MAPBLOCKSHIFT;
pub type dirtype_t = u32;
pub const NUMDIRS: dirtype_t = 9;
pub const DI_NODIR: dirtype_t = 8;
pub const DI_SOUTHEAST: dirtype_t = 7;
pub const DI_SOUTH: dirtype_t = 6;
pub const DI_SOUTHWEST: dirtype_t = 5;
pub const DI_WEST: dirtype_t = 4;
pub const DI_NORTHWEST: dirtype_t = 3;
pub const DI_NORTH: dirtype_t = 2;
pub const DI_NORTHEAST: dirtype_t = 1;
pub const DI_EAST: dirtype_t = 0;
pub const ML_SOUNDBLOCK: i32 = 64;
pub const MELEERANGE: i32 = 64 * FRACUNIT;
pub const MISSILERANGE: i32 = 32
    * 64 as i32 * FRACUNIT;
#[no_mangle]
pub static mut opposite: [dirtype_t; 9] = [
    DI_WEST,
    DI_SOUTHWEST,
    DI_SOUTH,
    DI_SOUTHEAST,
    DI_EAST,
    DI_NORTHEAST,
    DI_NORTH,
    DI_NORTHWEST,
    DI_NODIR,
];
#[no_mangle]
pub static mut diags: [dirtype_t; 4] = [
    DI_NORTHWEST,
    DI_NORTHEAST,
    DI_SOUTHWEST,
    DI_SOUTHEAST,
];
#[no_mangle]
pub static mut soundtarget: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
pub unsafe fn P_RecursiveSound(
    mut sec: *mut sector_t,
    mut soundblocks: i32,
) {
    let mut i: i32 = 0;
    let mut check: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut other: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    if (*sec).validcount == validcount
        && (*sec).soundtraversed <= soundblocks + 1 as i32
    {
        return;
    }
    (*sec).validcount = validcount;
    (*sec).soundtraversed = soundblocks + 1 as i32;
    (*sec).soundtarget = soundtarget;
    i = 0 as i32;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        if !((*check).flags as i32 & ML_TWOSIDED == 0) {
            P_LineOpening(check);
            if !(openrange <= 0 as i32) {
                if (*sides
                    .offset((*check).sidenum[0 as i32 as usize] as isize))
                    .sector == sec
                {
                    other = (*sides
                        .offset(
                            (*check).sidenum[1 as i32 as usize] as isize,
                        ))
                        .sector;
                } else {
                    other = (*sides
                        .offset(
                            (*check).sidenum[0 as i32 as usize] as isize,
                        ))
                        .sector;
                }
                if (*check).flags as i32 & ML_SOUNDBLOCK != 0 {
                    if soundblocks == 0 {
                        P_RecursiveSound(other, 1 as i32);
                    }
                } else {
                    P_RecursiveSound(other, soundblocks);
                }
            }
        }
        i += 1;
    }
}
pub unsafe fn P_NoiseAlert(
    mut target: *mut mobj_t,
    mut emmiter: *mut mobj_t,
) {
    soundtarget = target;
    validcount += 1;
    P_RecursiveSound((*(*emmiter).subsector).sector, 0 as i32);
}
pub unsafe fn P_CheckMeleeRange(mut actor: *mut mobj_t) -> bool {
    let mut pl: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut dist: fixed_t = 0;
    if (*actor).target.is_null() {
        return false;
    }
    pl = (*actor).target as *mut mobj_t;
    dist = P_AproxDistance((*pl).x - (*actor).x, (*pl).y - (*actor).y);
    if dist >= MELEERANGE - 20 as i32 * FRACUNIT + (*(*pl).info).radius {
        return false;
    }
    if !P_CheckSight(actor, (*actor).target as *mut mobj_t) {
        return false;
    }
    return true;
}
pub unsafe fn P_CheckMissileRange(mut actor: *mut mobj_t) -> bool {
    let mut dist: fixed_t = 0;
    if !P_CheckSight(actor, (*actor).target as *mut mobj_t) {
        return false;
    }
    if (*actor).flags & MF_JUSTHIT as i32 != 0 {
        (*actor).flags &= !(MF_JUSTHIT as i32);
        return true;
    }
    if (*actor).reactiontime != 0 {
        return false;
    }
    dist = (P_AproxDistance(
        (*actor).x - (*(*actor).target).x,
        (*actor).y - (*(*actor).target).y,
    ) as i32 - 64 as i32 * FRACUNIT) as fixed_t;
    if (*(*actor).info).meleestate == 0 {
        dist -= 128 as i32 * FRACUNIT;
    }
    dist >>= 16 as i32;
    if (*actor).type_0 as u32
        == MT_VILE as i32 as u32
    {
        if dist > 14 as i32 * 64 as i32 {
            return false;
        }
    }
    if (*actor).type_0 as u32
        == MT_UNDEAD as i32 as u32
    {
        if dist < 196 as i32 {
            return false;
        }
        dist >>= 1 as i32;
    }
    if (*actor).type_0 as u32
        == MT_CYBORG as i32 as u32
        || (*actor).type_0 as u32
            == MT_SPIDER as i32 as u32
        || (*actor).type_0 as u32
            == MT_SKULL as i32 as u32
    {
        dist >>= 1 as i32;
    }
    if dist > 200 as i32 {
        dist = 200 as i32 as fixed_t;
    }
    if (*actor).type_0 as u32
        == MT_CYBORG as i32 as u32
        && dist > 160 as i32
    {
        dist = 160 as i32 as fixed_t;
    }
    if P_Random() < dist {
        return false;
    }
    return true;
}
#[no_mangle]
pub static mut xspeed: [fixed_t; 8] = [
    FRACUNIT,
    47000 as i32,
    0 as i32,
    -(47000 as i32),
    -FRACUNIT,
    -(47000 as i32),
    0 as i32,
    47000 as i32,
];
#[no_mangle]
pub static mut yspeed: [fixed_t; 8] = [
    0 as i32,
    47000 as i32,
    FRACUNIT,
    47000 as i32,
    0 as i32,
    -(47000 as i32),
    -FRACUNIT,
    -(47000 as i32),
];
pub unsafe fn P_Move(mut actor: *mut mobj_t) -> bool {
    let mut tryx: fixed_t = 0;
    let mut tryy: fixed_t = 0;
    let mut ld: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut try_ok: bool;
    let mut good: bool;
    if (*actor).movedir == DI_NODIR as i32 {
        return false;
    }
    if (*actor).movedir as u32 >= 8 as u32 {
        I_Error("Weird actor->movedir!");
    }
    tryx = (*actor).x
        + (*(*actor).info).speed as fixed_t * xspeed[(*actor).movedir as usize];
    tryy = (*actor).y
        + (*(*actor).info).speed as fixed_t * yspeed[(*actor).movedir as usize];
    try_ok = P_TryMove(actor, tryx, tryy);
    if !try_ok {
        if (*actor).flags & MF_FLOAT as i32 != 0 && floatok {
            if (*actor).z < tmfloorz {
                (*actor).z += FLOATSPEED;
            } else {
                (*actor).z -= FLOATSPEED;
            }
            (*actor).flags |= MF_INFLOAT as i32;
            return true;
        }
        if numspechit == 0 {
            return false;
        }
        (*actor).movedir = DI_NODIR as i32;
        good = false;
        loop {
            let fresh0 = numspechit;
            numspechit = numspechit - 1;
            if !(fresh0 != 0) {
                break;
            }
            ld = spechit[numspechit as usize];
            if P_UseSpecialLine(actor, ld, 0 as i32) {
                good = true;
            }
        }
        return good;
    } else {
        (*actor).flags &= !(MF_INFLOAT as i32);
    }
    if (*actor).flags & MF_FLOAT as i32 == 0 {
        (*actor).z = (*actor).floorz;
    }
    return true;
}
pub unsafe fn P_TryWalk(mut actor: *mut mobj_t) -> bool {
    if !P_Move(actor) {
        return false;
    }
    (*actor).movecount = P_Random() & 15 as i32;
    return true;
}
pub unsafe fn P_NewChaseDir(mut actor: *mut mobj_t) {
    let mut deltax: fixed_t = 0;
    let mut deltay: fixed_t = 0;
    let mut d: [dirtype_t; 3] = [DI_EAST; 3];
    let mut tdir: i32 = 0;
    let mut olddir: dirtype_t = DI_EAST;
    let mut turnaround: dirtype_t = DI_EAST;
    if (*actor).target.is_null() {
        I_Error("P_NewChaseDir: called with no target");
    }
    olddir = (*actor).movedir as dirtype_t;
    turnaround = opposite[olddir as usize];
    deltax = (*(*actor).target).x - (*actor).x;
    deltay = (*(*actor).target).y - (*actor).y;
    if deltax > 10 as i32 * FRACUNIT {
        d[1 as i32 as usize] = DI_EAST;
    } else if deltax < -(10 as i32) * FRACUNIT {
        d[1 as i32 as usize] = DI_WEST;
    } else {
        d[1 as i32 as usize] = DI_NODIR;
    }
    if deltay < -(10 as i32) * FRACUNIT {
        d[2 as i32 as usize] = DI_SOUTH;
    } else if deltay > 10 as i32 * FRACUNIT {
        d[2 as i32 as usize] = DI_NORTH;
    } else {
        d[2 as i32 as usize] = DI_NODIR;
    }
    if d[1 as i32 as usize] as u32
        != DI_NODIR as i32 as u32
        && d[2 as i32 as usize] as u32
            != DI_NODIR as i32 as u32
    {
        (*actor).movedir = diags[((((deltay < 0 as i32)
            as i32) << 1 as i32)
            + (deltax > 0 as i32) as i32) as usize]
            as i32;
        if (*actor).movedir != turnaround as i32 && P_TryWalk(actor)
        {
            return;
        }
    }
    if P_Random() > 200 as i32
        || (deltay as i32).abs() > (deltax as i32).abs()
    {
        tdir = d[1 as i32 as usize] as i32;
        d[1 as i32 as usize] = d[2 as i32 as usize];
        d[2 as i32 as usize] = tdir as dirtype_t;
    }
    if d[1 as i32 as usize] as u32
        == turnaround as u32
    {
        d[1 as i32 as usize] = DI_NODIR;
    }
    if d[2 as i32 as usize] as u32
        == turnaround as u32
    {
        d[2 as i32 as usize] = DI_NODIR;
    }
    if d[1 as i32 as usize] as u32
        != DI_NODIR as i32 as u32
    {
        (*actor).movedir = d[1 as i32 as usize] as i32;
        if P_TryWalk(actor) {
            return;
        }
    }
    if d[2 as i32 as usize] as u32
        != DI_NODIR as i32 as u32
    {
        (*actor).movedir = d[2 as i32 as usize] as i32;
        if P_TryWalk(actor) {
            return;
        }
    }
    if olddir as u32
        != DI_NODIR as i32 as u32
    {
        (*actor).movedir = olddir as i32;
        if P_TryWalk(actor) {
            return;
        }
    }
    if P_Random() & 1 as i32 != 0 {
        tdir = DI_EAST as i32;
        while tdir <= DI_SOUTHEAST as i32 {
            if tdir != turnaround as i32 {
                (*actor).movedir = tdir;
                if P_TryWalk(actor) {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = DI_SOUTHEAST as i32;
        while tdir != DI_EAST as i32 - 1 as i32 {
            if tdir != turnaround as i32 {
                (*actor).movedir = tdir;
                if P_TryWalk(actor) {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    if turnaround as u32
        != DI_NODIR as i32 as u32
    {
        (*actor).movedir = turnaround as i32;
        if P_TryWalk(actor) {
            return;
        }
    }
    (*actor).movedir = DI_NODIR as i32;
}
pub unsafe fn P_LookForPlayers(
    mut actor: *mut mobj_t,
    mut allaround: bool,
) -> bool {
    let mut c: i32 = 0;
    let mut stop: i32 = 0;
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut an: angle_t = 0;
    let mut dist: fixed_t = 0;
    c = 0 as i32;
    stop = (*actor).lastlook - 1 as i32 & 3 as i32;
    let mut current_block_9: u64;
    loop {
        if !(playeringame[(*actor).lastlook as usize] == 0) {
            let fresh1 = c;
            c = c + 1;
            if fresh1 == 2 as i32 || (*actor).lastlook == stop {
                return false;
            }
            player = (&raw mut players as *mut player_t)
                .offset((*actor).lastlook as isize) as *mut player_t;
            if !((*player).health <= 0 as i32) {
                if P_CheckSight(actor, (*player).mo) {
                    if !allaround {
                        an = R_PointToAngle2(
                                (*actor).x,
                                (*actor).y,
                                (*(*player).mo).x,
                                (*(*player).mo).y,
                            )
                            .wrapping_sub((*actor).angle);
                        if an > ANG90 as angle_t && an < ANG270 {
                            dist = P_AproxDistance(
                                (*(*player).mo).x - (*actor).x,
                                (*(*player).mo).y - (*actor).y,
                            );
                            if dist > MELEERANGE {
                                current_block_9 = 4644295000439058019;
                            } else {
                                current_block_9 = 8236137900636309791;
                            }
                        } else {
                            current_block_9 = 8236137900636309791;
                        }
                    } else {
                        current_block_9 = 8236137900636309791;
                    }
                    match current_block_9 {
                        4644295000439058019 => {}
                        _ => {
                            (*actor).target = (*player).mo as *mut mobj_s;
                            return true;
                        }
                    }
                }
            }
        }
        (*actor).lastlook = (*actor).lastlook + 1 as i32
            & 3 as i32;
    };
}
pub unsafe fn A_KeenDie(mut mo: *mut mobj_t) {
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut mo2: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut junk: line_t = line_s {
        v1: ::core::ptr::null_mut::<vertex_t>(),
        v2: ::core::ptr::null_mut::<vertex_t>(),
        dx: 0,
        dy: 0,
        flags: 0,
        special: 0,
        tag: 0,
        sidenum: [0; 2],
        bbox: [0; 4],
        slopetype: ST_HORIZONTAL,
        frontsector: ::core::ptr::null_mut::<sector_t>(),
        backsector: ::core::ptr::null_mut::<sector_t>(),
        validcount: 0,
        specialdata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    A_Fall(mo);
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        if matches!((*th).function, ThinkerFn::Mobj(_))
        {
            mo2 = th as *mut mobj_t;
            if mo2 != mo
                && (*mo2).type_0 as u32
                    == (*mo).type_0 as u32
                && (*mo2).health > 0 as i32
            {
                return;
            }
        }
        th = (*th).next as *mut thinker_t;
    }
    junk.tag = 666 as i16;
    EV_DoDoor(&raw mut junk, vld_open);
}
pub unsafe fn A_Look(mut actor: *mut mobj_t) {
    let mut current_block: u64;
    let mut targ: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    (*actor).threshold = 0 as i32;
    targ = (*(*(*actor).subsector).sector).soundtarget;
    if !targ.is_null() && (*targ).flags & MF_SHOOTABLE as i32 != 0 {
        (*actor).target = targ as *mut mobj_s;
        if (*actor).flags & MF_AMBUSH as i32 != 0 {
            if P_CheckSight(actor, (*actor).target as *mut mobj_t) {
                current_block = 10571674169298881693;
            } else {
                current_block = 15619007995458559411;
            }
        } else {
            current_block = 10571674169298881693;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            if !P_LookForPlayers(actor, false) {
                return;
            }
        }
        _ => {}
    }
    if (*(*actor).info).seesound != 0 {
        let mut sound: i32 = 0;
        match (*(*actor).info).seesound {
            36 | 37 | 38 => {
                sound = sfx_posit1 as i32
                    + P_Random() % 3 as i32;
            }
            39 | 40 => {
                sound = sfx_bgsit1 as i32
                    + P_Random() % 2 as i32;
            }
            _ => {
                sound = (*(*actor).info).seesound;
            }
        }
        if (*actor).type_0 as u32
            == MT_SPIDER as i32 as u32
            || (*actor).type_0 as u32
                == MT_CYBORG as i32 as u32
        {
            S_StartSound(NULL, sound);
        } else {
            S_StartSound(actor as *mut ::core::ffi::c_void, sound);
        }
    }
    P_SetMobjState(actor, (*(*actor).info).seestate as statenum_t);
}
pub unsafe fn A_Chase(mut actor: *mut mobj_t) {
    let mut delta: i32 = 0;
    if (*actor).reactiontime != 0 {
        (*actor).reactiontime -= 1;
    }
    if (*actor).threshold != 0 {
        if (*actor).target.is_null()
            || (*(*actor).target).health <= 0 as i32
        {
            (*actor).threshold = 0 as i32;
        } else {
            (*actor).threshold -= 1;
        }
    }
    if (*actor).movedir < 8 as i32 {
        (*actor).angle
            &= ((7 as i32) << 29 as i32) as angle_t;
        delta = (*actor)
            .angle
            .wrapping_sub(((*actor).movedir << 29 as i32) as angle_t)
            as i32;
        if delta > 0 as i32 {
            (*actor).angle = (*actor)
                .angle
                .wrapping_sub((ANG90 / 2 as i32) as angle_t);
        } else if delta < 0 as i32 {
            (*actor).angle = (*actor)
                .angle
                .wrapping_add((ANG90 / 2 as i32) as angle_t);
        }
    }
    if (*actor).target.is_null()
        || (*(*actor).target).flags & MF_SHOOTABLE as i32 == 0
    {
        if P_LookForPlayers(actor, true) {
            return;
        }
        P_SetMobjState(actor, (*(*actor).info).spawnstate as statenum_t);
        return;
    }
    if (*actor).flags & MF_JUSTATTACKED as i32 != 0 {
        (*actor).flags &= !(MF_JUSTATTACKED as i32);
        if gameskill as i32 != sk_nightmare as i32
            && !fastparm
        {
            P_NewChaseDir(actor);
        }
        return;
    }
    if (*(*actor).info).meleestate != 0 && P_CheckMeleeRange(actor) {
        if (*(*actor).info).attacksound != 0 {
            S_StartSound(
                actor as *mut ::core::ffi::c_void,
                (*(*actor).info).attacksound,
            );
        }
        P_SetMobjState(actor, (*(*actor).info).meleestate as statenum_t);
        return;
    }
    if (*(*actor).info).missilestate != 0 {
        if !((gameskill as i32) < sk_nightmare as i32
            && !fastparm && (*actor).movecount != 0)
        {
            if P_CheckMissileRange(actor) {
                P_SetMobjState(actor, (*(*actor).info).missilestate as statenum_t);
                (*actor).flags |= MF_JUSTATTACKED as i32;
                return;
            }
        }
    }
    if netgame && (*actor).threshold == 0
        && !P_CheckSight(actor, (*actor).target as *mut mobj_t)
    {
        if P_LookForPlayers(actor, true) {
            return;
        }
    }
    (*actor).movecount -= 1;
    if (*actor).movecount < 0 as i32 || !P_Move(actor) {
        P_NewChaseDir(actor);
    }
    if (*(*actor).info).activesound != 0 && P_Random() < 3 as i32 {
        S_StartSound(actor as *mut ::core::ffi::c_void, (*(*actor).info).activesound);
    }
}
pub unsafe fn A_FaceTarget(mut actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    (*actor).flags &= !(MF_AMBUSH as i32);
    (*actor).angle = R_PointToAngle2(
        (*actor).x,
        (*actor).y,
        (*(*actor).target).x,
        (*(*actor).target).y,
    );
    if (*(*actor).target).flags & MF_SHADOW as i32 != 0 {
        (*actor).angle = (*actor)
            .angle
            .wrapping_add(
                (P_Random() - P_Random() << 21 as i32) as angle_t,
            );
    }
}
pub unsafe fn A_PosAttack(mut actor: *mut mobj_t) {
    let mut angle: i32 = 0;
    let mut damage: i32 = 0;
    let mut slope: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    angle = (*actor).angle as i32;
    slope = P_AimLineAttack(actor, angle as angle_t, MISSILERANGE) as i32;
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_pistol as i32);
    angle += P_Random() - P_Random() << 20 as i32;
    damage = (P_Random() % 5 as i32 + 1 as i32)
        * 3 as i32;
    P_LineAttack(actor, angle as angle_t, MISSILERANGE, slope as fixed_t, damage);
}
pub unsafe fn A_SPosAttack(mut actor: *mut mobj_t) {
    let mut i: i32 = 0;
    let mut angle: i32 = 0;
    let mut bangle: i32 = 0;
    let mut damage: i32 = 0;
    let mut slope: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_shotgn as i32);
    A_FaceTarget(actor);
    bangle = (*actor).angle as i32;
    slope = P_AimLineAttack(actor, bangle as angle_t, MISSILERANGE)
        as i32;
    i = 0 as i32;
    while i < 3 as i32 {
        angle = bangle + (P_Random() - P_Random() << 20 as i32);
        damage = (P_Random() % 5 as i32 + 1 as i32)
            * 3 as i32;
        P_LineAttack(actor, angle as angle_t, MISSILERANGE, slope as fixed_t, damage);
        i += 1;
    }
}
pub unsafe fn A_CPosAttack(mut actor: *mut mobj_t) {
    let mut angle: i32 = 0;
    let mut bangle: i32 = 0;
    let mut damage: i32 = 0;
    let mut slope: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_shotgn as i32);
    A_FaceTarget(actor);
    bangle = (*actor).angle as i32;
    slope = P_AimLineAttack(actor, bangle as angle_t, MISSILERANGE)
        as i32;
    angle = bangle + (P_Random() - P_Random() << 20 as i32);
    damage = (P_Random() % 5 as i32 + 1 as i32)
        * 3 as i32;
    P_LineAttack(actor, angle as angle_t, MISSILERANGE, slope as fixed_t, damage);
}
pub unsafe fn A_CPosRefire(mut actor: *mut mobj_t) {
    A_FaceTarget(actor);
    if P_Random() < 40 as i32 {
        return;
    }
    if (*actor).target.is_null() || (*(*actor).target).health <= 0 as i32
        || !P_CheckSight(actor, (*actor).target as *mut mobj_t)
    {
        P_SetMobjState(actor, (*(*actor).info).seestate as statenum_t);
    }
}
pub unsafe fn A_SpidRefire(mut actor: *mut mobj_t) {
    A_FaceTarget(actor);
    if P_Random() < 10 as i32 {
        return;
    }
    if (*actor).target.is_null() || (*(*actor).target).health <= 0 as i32
        || !P_CheckSight(actor, (*actor).target as *mut mobj_t)
    {
        P_SetMobjState(actor, (*(*actor).info).seestate as statenum_t);
    }
}
pub unsafe fn A_BspiAttack(mut actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    P_SpawnMissile(actor, (*actor).target as *mut mobj_t, MT_ARACHPLAZ);
}
pub unsafe fn A_TroopAttack(mut actor: *mut mobj_t) {
    let mut damage: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor) {
        S_StartSound(actor as *mut ::core::ffi::c_void, sfx_claw as i32);
        damage = (P_Random() % 8 as i32 + 1 as i32)
            * 3 as i32;
        P_DamageMobj((*actor).target as *mut mobj_t, actor, actor, damage);
        return;
    }
    P_SpawnMissile(actor, (*actor).target as *mut mobj_t, MT_TROOPSHOT);
}
pub unsafe fn A_SargAttack(mut actor: *mut mobj_t) {
    let mut damage: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor) {
        damage = (P_Random() % 10 as i32 + 1 as i32)
            * 4 as i32;
        P_DamageMobj((*actor).target as *mut mobj_t, actor, actor, damage);
    }
}
pub unsafe fn A_HeadAttack(mut actor: *mut mobj_t) {
    let mut damage: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor) {
        damage = (P_Random() % 6 as i32 + 1 as i32)
            * 10 as i32;
        P_DamageMobj((*actor).target as *mut mobj_t, actor, actor, damage);
        return;
    }
    P_SpawnMissile(actor, (*actor).target as *mut mobj_t, MT_HEADSHOT);
}
pub unsafe fn A_CyberAttack(mut actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    P_SpawnMissile(actor, (*actor).target as *mut mobj_t, MT_ROCKET);
}
pub unsafe fn A_BruisAttack(mut actor: *mut mobj_t) {
    let mut damage: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    if P_CheckMeleeRange(actor) {
        S_StartSound(actor as *mut ::core::ffi::c_void, sfx_claw as i32);
        damage = (P_Random() % 8 as i32 + 1 as i32)
            * 10 as i32;
        P_DamageMobj((*actor).target as *mut mobj_t, actor, actor, damage);
        return;
    }
    P_SpawnMissile(actor, (*actor).target as *mut mobj_t, MT_BRUISERSHOT);
}
pub unsafe fn A_SkelMissile(mut actor: *mut mobj_t) {
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    (*actor).z += 16 as i32 * FRACUNIT;
    mo = P_SpawnMissile(actor, (*actor).target as *mut mobj_t, MT_TRACER);
    (*actor).z -= 16 as i32 * FRACUNIT;
    (*mo).x += (*mo).momx;
    (*mo).y += (*mo).momy;
    (*mo).tracer = (*actor).target;
}
#[no_mangle]
pub static mut TRACEANGLE: i32 = 0xc000000;
pub unsafe fn A_Tracer(mut actor: *mut mobj_t) {
    let mut exact: angle_t = 0;
    let mut dist: fixed_t = 0;
    let mut slope: fixed_t = 0;
    let mut dest: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    if gametic & 3 as i32 != 0 {
        return;
    }
    P_SpawnPuff((*actor).x, (*actor).y, (*actor).z);
    th = P_SpawnMobj(
        (*actor).x - (*actor).momx,
        (*actor).y - (*actor).momy,
        (*actor).z,
        MT_SMOKE,
    );
    (*th).momz = FRACUNIT as fixed_t;
    (*th).tics -= P_Random() & 3 as i32;
    if (*th).tics < 1 as i32 {
        (*th).tics = 1 as i32;
    }
    dest = (*actor).tracer as *mut mobj_t;
    if dest.is_null() || (*dest).health <= 0 as i32 {
        return;
    }
    exact = R_PointToAngle2((*actor).x, (*actor).y, (*dest).x, (*dest).y);
    if exact != (*actor).angle {
        if exact.wrapping_sub((*actor).angle) > 0x80000000 as u32 {
            (*actor).angle = (*actor).angle.wrapping_sub(TRACEANGLE as angle_t);
            if exact.wrapping_sub((*actor).angle) < 0x80000000 as u32 {
                (*actor).angle = exact;
            }
        } else {
            (*actor).angle = (*actor).angle.wrapping_add(TRACEANGLE as angle_t);
            if exact.wrapping_sub((*actor).angle) > 0x80000000 as u32 {
                (*actor).angle = exact;
            }
        }
    }
    exact = (*actor).angle >> ANGLETOFINESHIFT;
    (*actor).momx = FixedMul(
        (*(*actor).info).speed as fixed_t,
        finecosine[exact as isize],
    );
    (*actor).momy = FixedMul(
        (*(*actor).info).speed as fixed_t,
        finesine[exact as usize],
    );
    dist = P_AproxDistance((*dest).x - (*actor).x, (*dest).y - (*actor).y);
    dist = (dist as i32 / (*(*actor).info).speed) as fixed_t;
    if dist < 1 as i32 {
        dist = 1 as i32 as fixed_t;
    }
    slope = ((*dest).z + 40 as fixed_t * FRACUNIT - (*actor).z) / dist;
    if slope < (*actor).momz {
        (*actor).momz -= FRACUNIT / 8 as i32;
    } else {
        (*actor).momz += FRACUNIT / 8 as i32;
    };
}
pub unsafe fn A_SkelWhoosh(mut actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_skeswg as i32);
}
pub unsafe fn A_SkelFist(mut actor: *mut mobj_t) {
    let mut damage: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor) {
        damage = (P_Random() % 10 as i32 + 1 as i32)
            * 6 as i32;
        S_StartSound(
            actor as *mut ::core::ffi::c_void,
            sfx_skepch as i32,
        );
        P_DamageMobj((*actor).target as *mut mobj_t, actor, actor, damage);
    }
}
#[no_mangle]
pub static mut corpsehit: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut vileobj: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut viletryx: fixed_t = 0;
#[no_mangle]
pub static mut viletryy: fixed_t = 0;
#[no_mangle]
pub unsafe extern "C" fn PIT_VileCheck(mut thing: *mut mobj_t) -> boolean {
    let mut maxdist: i32 = 0;
    let mut check: bool = false;
    if (*thing).flags & MF_CORPSE as i32 == 0 {
        return true_0 as boolean;
    }
    if (*thing).tics != -(1 as i32) {
        return true_0 as boolean;
    }
    if (*(*thing).info).raisestate == S_NULL as i32 {
        return true_0 as boolean;
    }
    maxdist = (*(*thing).info).radius
        + mobjinfo[MT_VILE as i32 as usize].radius;
    if ((*thing).x as i32 - viletryx as i32).abs() > maxdist
        || ((*thing).y as i32 - viletryy as i32).abs()
            > maxdist
    {
        return true_0 as boolean;
    }
    corpsehit = thing;
    (*corpsehit).momy = 0 as i32 as fixed_t;
    (*corpsehit).momx = (*corpsehit).momy;
    (*corpsehit).height <<= 2 as i32;
    check = P_CheckPosition(corpsehit, (*corpsehit).x, (*corpsehit).y);
    (*corpsehit).height >>= 2 as i32;
    if !check {
        return true_0 as boolean;
    }
    return false_0 as boolean;
}
pub unsafe fn A_VileChase(mut actor: *mut mobj_t) {
    let mut xl: i32 = 0;
    let mut xh: i32 = 0;
    let mut yl: i32 = 0;
    let mut yh: i32 = 0;
    let mut bx: i32 = 0;
    let mut by: i32 = 0;
    let mut info: *mut mobjinfo_t = ::core::ptr::null_mut::<mobjinfo_t>();
    let mut temp: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    if (*actor).movedir != DI_NODIR as i32 {
        viletryx = (*actor).x
            + (*(*actor).info).speed as fixed_t * xspeed[(*actor).movedir as usize];
        viletryy = (*actor).y
            + (*(*actor).info).speed as fixed_t * yspeed[(*actor).movedir as usize];
        xl = viletryx as i32 - bmaporgx as i32
            - 32 as i32 * FRACUNIT * 2 as i32
            >> MAPBLOCKSHIFT;
        xh = viletryx as i32 - bmaporgx as i32
            + 32 as i32 * FRACUNIT * 2 as i32
            >> MAPBLOCKSHIFT;
        yl = viletryy as i32 - bmaporgy as i32
            - 32 as i32 * FRACUNIT * 2 as i32
            >> MAPBLOCKSHIFT;
        yh = viletryy as i32 - bmaporgy as i32
            + 32 as i32 * FRACUNIT * 2 as i32
            >> MAPBLOCKSHIFT;
        vileobj = actor;
        bx = xl;
        while bx <= xh {
            by = yl;
            while by <= yh {
                if !P_BlockThingsIterator(
                    bx,
                    by,
                    Some(PIT_VileCheck as unsafe extern "C" fn(*mut mobj_t) -> boolean),
                )
                {
                    temp = (*actor).target as *mut mobj_t;
                    (*actor).target = corpsehit as *mut mobj_s;
                    A_FaceTarget(actor);
                    (*actor).target = temp as *mut mobj_s;
                    P_SetMobjState(actor, S_VILE_HEAL1);
                    S_StartSound(
                        corpsehit as *mut ::core::ffi::c_void,
                        sfx_slop as i32,
                    );
                    info = (*corpsehit).info;
                    P_SetMobjState(corpsehit, (*info).raisestate as statenum_t);
                    (*corpsehit).height <<= 2 as i32;
                    (*corpsehit).flags = (*info).flags;
                    (*corpsehit).health = (*info).spawnhealth;
                    (*corpsehit).target = ::core::ptr::null_mut::<mobj_s>();
                    return;
                }
                by += 1;
            }
            bx += 1;
        }
    }
    A_Chase(actor);
}
pub unsafe fn A_VileStart(mut actor: *mut mobj_t) {
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_vilatk as i32);
}
pub unsafe fn A_StartFire(mut actor: *mut mobj_t) {
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_flamst as i32);
    A_Fire(actor);
}
pub unsafe fn A_FireCrackle(mut actor: *mut mobj_t) {
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_flame as i32);
    A_Fire(actor);
}
pub unsafe fn A_Fire(mut actor: *mut mobj_t) {
    let mut dest: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut target: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: u32 = 0;
    dest = (*actor).tracer as *mut mobj_t;
    if dest.is_null() {
        return;
    }
    target = P_SubstNullMobj((*actor).target as *mut mobj_t);
    if !P_CheckSight(target, dest) {
        return;
    }
    an = ((*dest).angle >> ANGLETOFINESHIFT) as u32;
    P_UnsetThingPosition(actor);
    (*actor).x = (*dest).x
        + FixedMul(24 as fixed_t * FRACUNIT, finecosine[an as isize]);
    (*actor).y = (*dest).y + FixedMul(24 as fixed_t * FRACUNIT, finesine[an as usize]);
    (*actor).z = (*dest).z;
    P_SetThingPosition(actor);
}
pub unsafe fn A_VileTarget(mut actor: *mut mobj_t) {
    let mut fog: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    fog = P_SpawnMobj(
        (*(*actor).target).x,
        (*(*actor).target).x,
        (*(*actor).target).z,
        MT_FIRE,
    );
    (*actor).tracer = fog as *mut mobj_s;
    (*fog).target = actor as *mut mobj_s;
    (*fog).tracer = (*actor).target;
    A_Fire(fog);
}
pub unsafe fn A_VileAttack(mut actor: *mut mobj_t) {
    let mut fire: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if !P_CheckSight(actor, (*actor).target as *mut mobj_t) {
        return;
    }
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_barexp as i32);
    P_DamageMobj((*actor).target as *mut mobj_t, actor, actor, 20 as i32);
    (*(*actor).target).momz = (1000 as i32 * FRACUNIT
        / (*(*(*actor).target).info).mass) as fixed_t;
    an = ((*actor).angle >> ANGLETOFINESHIFT) as i32;
    fire = (*actor).tracer as *mut mobj_t;
    if fire.is_null() {
        return;
    }
    (*fire).x = (*(*actor).target).x
        - FixedMul(24 as fixed_t * FRACUNIT, finecosine[an as isize]);
    (*fire).y = (*(*actor).target).y
        - FixedMul(24 as fixed_t * FRACUNIT, finesine[an as usize]);
    P_RadiusAttack(fire, actor, 70 as i32);
}
pub const FATSPREAD: i32 = ANG90 / 8 as i32;
pub unsafe fn A_FatRaise(mut actor: *mut mobj_t) {
    A_FaceTarget(actor);
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_manatk as i32);
}
pub unsafe fn A_FatAttack1(mut actor: *mut mobj_t) {
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut target: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: i32 = 0;
    A_FaceTarget(actor);
    (*actor).angle = (*actor).angle.wrapping_add(FATSPREAD as angle_t);
    target = P_SubstNullMobj((*actor).target as *mut mobj_t);
    P_SpawnMissile(actor, target, MT_FATSHOT);
    mo = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo).angle.wrapping_add(FATSPREAD as angle_t);
    an = ((*mo).angle >> ANGLETOFINESHIFT) as i32;
    (*mo).momx = FixedMul(
        (*(*mo).info).speed as fixed_t,
        finecosine[an as isize],
    );
    (*mo).momy = FixedMul((*(*mo).info).speed as fixed_t, finesine[an as usize]);
}
pub unsafe fn A_FatAttack2(mut actor: *mut mobj_t) {
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut target: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: i32 = 0;
    A_FaceTarget(actor);
    (*actor).angle = (*actor).angle.wrapping_sub(FATSPREAD as angle_t);
    target = P_SubstNullMobj((*actor).target as *mut mobj_t);
    P_SpawnMissile(actor, target, MT_FATSHOT);
    mo = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo)
        .angle
        .wrapping_sub((FATSPREAD * 2 as i32) as angle_t);
    an = ((*mo).angle >> ANGLETOFINESHIFT) as i32;
    (*mo).momx = FixedMul(
        (*(*mo).info).speed as fixed_t,
        finecosine[an as isize],
    );
    (*mo).momy = FixedMul((*(*mo).info).speed as fixed_t, finesine[an as usize]);
}
pub unsafe fn A_FatAttack3(mut actor: *mut mobj_t) {
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut target: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: i32 = 0;
    A_FaceTarget(actor);
    target = P_SubstNullMobj((*actor).target as *mut mobj_t);
    mo = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo)
        .angle
        .wrapping_sub((FATSPREAD / 2 as i32) as angle_t);
    an = ((*mo).angle >> ANGLETOFINESHIFT) as i32;
    (*mo).momx = FixedMul(
        (*(*mo).info).speed as fixed_t,
        finecosine[an as isize],
    );
    (*mo).momy = FixedMul((*(*mo).info).speed as fixed_t, finesine[an as usize]);
    mo = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo)
        .angle
        .wrapping_add((FATSPREAD / 2 as i32) as angle_t);
    an = ((*mo).angle >> ANGLETOFINESHIFT) as i32;
    (*mo).momx = FixedMul(
        (*(*mo).info).speed as fixed_t,
        finecosine[an as isize],
    );
    (*mo).momy = FixedMul((*(*mo).info).speed as fixed_t, finesine[an as usize]);
}
pub const SKULLSPEED: i32 = 20 * FRACUNIT;
pub unsafe fn A_SkullAttack(mut actor: *mut mobj_t) {
    let mut dest: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: angle_t = 0;
    let mut dist: i32 = 0;
    if (*actor).target.is_null() {
        return;
    }
    dest = (*actor).target as *mut mobj_t;
    (*actor).flags |= MF_SKULLFLY as i32;
    S_StartSound(actor as *mut ::core::ffi::c_void, (*(*actor).info).attacksound);
    A_FaceTarget(actor);
    an = (*actor).angle >> ANGLETOFINESHIFT;
    (*actor).momx = FixedMul(SKULLSPEED, finecosine[an as isize]);
    (*actor).momy = FixedMul(SKULLSPEED, finesine[an as usize]);
    dist = P_AproxDistance((*dest).x - (*actor).x, (*dest).y - (*actor).y)
        as i32;
    dist = dist / SKULLSPEED;
    if dist < 1 as i32 {
        dist = 1 as i32;
    }
    (*actor).momz = (((*dest).z as i32
        + ((*dest).height as i32 >> 1 as i32)
        - (*actor).z as i32) / dist) as fixed_t;
}
pub unsafe fn A_PainShootSkull(mut actor: *mut mobj_t, mut angle: angle_t) {
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut newmobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: angle_t = 0;
    let mut prestep: i32 = 0;
    let mut count: i32 = 0;
    let mut currentthinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    count = 0 as i32;
    currentthinker = thinkercap.next as *mut thinker_t;
    while currentthinker != &raw mut thinkercap {
        if matches!((*currentthinker).function, ThinkerFn::Mobj(_))
            && (*(currentthinker as *mut mobj_t)).type_0 as u32
                == MT_SKULL as i32 as u32
        {
            count += 1;
        }
        currentthinker = (*currentthinker).next as *mut thinker_t;
    }
    if count > 20 as i32 {
        return;
    }
    an = angle >> ANGLETOFINESHIFT;
    prestep = 4 as i32 * FRACUNIT
        + 3 as i32
            * ((*(*actor).info).radius
                + mobjinfo[MT_SKULL as i32 as usize].radius)
            / 2 as i32;
    x = (*actor).x + FixedMul(prestep as fixed_t, finecosine[an as isize]);
    y = (*actor).y + FixedMul(prestep as fixed_t, finesine[an as usize]);
    z = ((*actor).z as i32 + 8 as i32 * FRACUNIT)
        as fixed_t;
    newmobj = P_SpawnMobj(x, y, z, MT_SKULL);
    if !P_TryMove(newmobj, (*newmobj).x, (*newmobj).y) {
        P_DamageMobj(newmobj, actor, actor, 10000 as i32);
        return;
    }
    (*newmobj).target = (*actor).target;
    A_SkullAttack(newmobj);
}
pub unsafe fn A_PainAttack(mut actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    A_PainShootSkull(actor, (*actor).angle);
}
pub unsafe fn A_PainDie(mut actor: *mut mobj_t) {
    A_Fall(actor);
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG90 as angle_t));
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG180));
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG270));
}
pub unsafe fn A_Scream(mut actor: *mut mobj_t) {
    let mut sound: i32 = 0;
    match (*(*actor).info).deathsound {
        0 => return,
        59 | 60 | 61 => {
            sound = sfx_podth1 as i32
                + P_Random() % 3 as i32;
        }
        62 | 63 => {
            sound = sfx_bgdth1 as i32
                + P_Random() % 2 as i32;
        }
        _ => {
            sound = (*(*actor).info).deathsound;
        }
    }
    if (*actor).type_0 as u32
        == MT_SPIDER as i32 as u32
        || (*actor).type_0 as u32
            == MT_CYBORG as i32 as u32
    {
        S_StartSound(NULL, sound);
    } else {
        S_StartSound(actor as *mut ::core::ffi::c_void, sound);
    };
}
pub unsafe fn A_XScream(mut actor: *mut mobj_t) {
    S_StartSound(actor as *mut ::core::ffi::c_void, sfx_slop as i32);
}
pub unsafe fn A_Pain(mut actor: *mut mobj_t) {
    if (*(*actor).info).painsound != 0 {
        S_StartSound(actor as *mut ::core::ffi::c_void, (*(*actor).info).painsound);
    }
}
pub unsafe fn A_Fall(mut actor: *mut mobj_t) {
    (*actor).flags &= !(MF_SOLID as i32);
}
pub unsafe fn A_Explode(mut thingy: *mut mobj_t) {
    P_RadiusAttack(thingy, (*thingy).target as *mut mobj_t, 128 as i32);
}
unsafe fn CheckBossEnd(mut motype: mobjtype_t) -> bool {
    if (gameversion as u32)
        < exe_ultimate as i32 as u32
    {
        if gamemap != 8 as i32 {
            return false;
        }
        if motype as u32
            == MT_BRUISER as i32 as u32
            && gameepisode != 1 as i32
        {
            return false;
        }
        return true;
    } else {
        match gameepisode {
            1 => {
                return gamemap == 8 as i32
                    && motype as u32
                        == MT_BRUISER as i32 as u32;
            }
            2 => {
                return gamemap == 8 as i32
                    && motype as u32
                        == MT_CYBORG as i32 as u32;
            }
            3 => {
                return gamemap == 8 as i32
                    && motype as u32
                        == MT_SPIDER as i32 as u32;
            }
            4 => {
                return gamemap == 6 as i32
                    && motype as u32
                        == MT_CYBORG as i32 as u32
                    || gamemap == 8 as i32
                        && motype as u32
                            == MT_SPIDER as i32 as u32;
            }
            _ => {
                return gamemap == 8 as i32;
            }
        }
    };
}
pub unsafe fn A_BossDeath(mut mo: *mut mobj_t) {
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut mo2: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut junk: line_t = line_s {
        v1: ::core::ptr::null_mut::<vertex_t>(),
        v2: ::core::ptr::null_mut::<vertex_t>(),
        dx: 0,
        dy: 0,
        flags: 0,
        special: 0,
        tag: 0,
        sidenum: [0; 2],
        bbox: [0; 4],
        slopetype: ST_HORIZONTAL,
        frontsector: ::core::ptr::null_mut::<sector_t>(),
        backsector: ::core::ptr::null_mut::<sector_t>(),
        validcount: 0,
        specialdata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut i: i32 = 0;
    if gamemode as u32
        == commercial as i32 as u32
    {
        if gamemap != 7 as i32 {
            return;
        }
        if (*mo).type_0 as u32
            != MT_FATSO as i32 as u32
            && (*mo).type_0 as u32
                != MT_BABY as i32 as u32
        {
            return;
        }
    } else if !CheckBossEnd((*mo).type_0) {
        return
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0
            && players[i as usize].health > 0 as i32
        {
            break;
        }
        i += 1;
    }
    if i == MAXPLAYERS {
        return;
    }
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        if matches!((*th).function, ThinkerFn::Mobj(_))
        {
            mo2 = th as *mut mobj_t;
            if mo2 != mo
                && (*mo2).type_0 as u32
                    == (*mo).type_0 as u32
                && (*mo2).health > 0 as i32
            {
                return;
            }
        }
        th = (*th).next as *mut thinker_t;
    }
    if gamemode as u32
        == commercial as i32 as u32
    {
        if gamemap == 7 as i32 {
            if (*mo).type_0 as u32
                == MT_FATSO as i32 as u32
            {
                junk.tag = 666 as i16;
                EV_DoFloor(&raw mut junk, lowerFloorToLowest);
                return;
            }
            if (*mo).type_0 as u32
                == MT_BABY as i32 as u32
            {
                junk.tag = 667 as i16;
                EV_DoFloor(&raw mut junk, raiseToTexture);
                return;
            }
        }
    } else {
        match gameepisode {
            1 => {
                junk.tag = 666 as i16;
                EV_DoFloor(&raw mut junk, lowerFloorToLowest);
                return;
            }
            4 => {
                match gamemap {
                    6 => {
                        junk.tag = 666 as i16;
                        EV_DoDoor(&raw mut junk, vld_blazeOpen);
                        return;
                    }
                    8 => {
                        junk.tag = 666 as i16;
                        EV_DoFloor(&raw mut junk, lowerFloorToLowest);
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    G_ExitLevel();
}
pub unsafe fn A_Hoof(mut mo: *mut mobj_t) {
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_hoof as i32);
    A_Chase(mo);
}
pub unsafe fn A_Metal(mut mo: *mut mobj_t) {
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_metal as i32);
    A_Chase(mo);
}
pub unsafe fn A_BabyMetal(mut mo: *mut mobj_t) {
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_bspwlk as i32);
    A_Chase(mo);
}
pub unsafe fn A_OpenShotgun2(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_dbopn as i32,
    );
}
pub unsafe fn A_LoadShotgun2(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_dbload as i32,
    );
}
pub unsafe fn A_CloseShotgun2(
    mut player: *mut player_t,
    mut psp: *mut pspdef_t,
) {
    S_StartSound(
        (*player).mo as *mut ::core::ffi::c_void,
        sfx_dbcls as i32,
    );
    A_ReFire(player, psp);
}
#[no_mangle]
pub static mut braintargets: [*mut mobj_t; 32] = [::core::ptr::null::<mobj_t>()
    as *mut mobj_t; 32];
#[no_mangle]
pub static mut numbraintargets: i32 = 0;
#[no_mangle]
pub static mut braintargeton: i32 = 0;
pub unsafe fn A_BrainAwake(mut mo: *mut mobj_t) {
    let mut thinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut m: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    numbraintargets = 0 as i32;
    braintargeton = 0 as i32;
    thinker = thinkercap.next as *mut thinker_t;
    thinker = thinkercap.next as *mut thinker_t;
    while thinker != &raw mut thinkercap {
        if matches!((*thinker).function, ThinkerFn::Mobj(_))
        {
            m = thinker as *mut mobj_t;
            if (*m).type_0 as u32
                == MT_BOSSTARGET as i32 as u32
            {
                braintargets[numbraintargets as usize] = m;
                numbraintargets += 1;
            }
        }
        thinker = (*thinker).next as *mut thinker_t;
    }
    S_StartSound(NULL, sfx_bossit as i32);
}
pub unsafe fn A_BrainPain(mut mo: *mut mobj_t) {
    S_StartSound(NULL, sfx_bospn as i32);
}
pub unsafe fn A_BrainScream(mut mo: *mut mobj_t) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    x = (*mo).x as i32 - 196 as i32 * FRACUNIT;
    while x < (*mo).x as i32 + 320 as i32 * FRACUNIT {
        y = (*mo).y as i32 - 320 as i32 * FRACUNIT;
        z = 128 as i32 + P_Random() * 2 as i32 * FRACUNIT;
        th = P_SpawnMobj(x as fixed_t, y as fixed_t, z as fixed_t, MT_ROCKET);
        (*th).momz = (P_Random() * 512 as i32) as fixed_t;
        P_SetMobjState(th, S_BRAINEXPLODE1);
        (*th).tics -= P_Random() & 7 as i32;
        if (*th).tics < 1 as i32 {
            (*th).tics = 1 as i32;
        }
        x += FRACUNIT * 8 as i32;
    }
    S_StartSound(NULL, sfx_bosdth as i32);
}
pub unsafe fn A_BrainExplode(mut mo: *mut mobj_t) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    x = (*mo).x as i32
        + (P_Random() - P_Random()) * 2048 as i32;
    y = (*mo).y as i32;
    z = 128 as i32 + P_Random() * 2 as i32 * FRACUNIT;
    th = P_SpawnMobj(x as fixed_t, y as fixed_t, z as fixed_t, MT_ROCKET);
    (*th).momz = (P_Random() * 512 as i32) as fixed_t;
    P_SetMobjState(th, S_BRAINEXPLODE1);
    (*th).tics -= P_Random() & 7 as i32;
    if (*th).tics < 1 as i32 {
        (*th).tics = 1 as i32;
    }
}
pub unsafe fn A_BrainDie(mut mo: *mut mobj_t) {
    G_ExitLevel();
}
pub unsafe fn A_BrainSpit(mut mo: *mut mobj_t) {
    let mut targ: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut newmobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    static mut easy: i32 = 0;
    easy ^= 1 as i32;
    if gameskill as i32 <= sk_easy as i32 && easy == 0 {
        return;
    }
    targ = braintargets[braintargeton as usize];
    braintargeton = (braintargeton + 1 as i32) % numbraintargets;
    newmobj = P_SpawnMissile(mo, targ, MT_SPAWNSHOT);
    (*newmobj).target = targ as *mut mobj_s;
    (*newmobj).reactiontime = ((*targ).y as i32
        - (*mo).y as i32) / (*newmobj).momy as i32
        / (*(*newmobj).state).tics;
    S_StartSound(NULL, sfx_bospit as i32);
}
pub unsafe fn A_SpawnSound(mut mo: *mut mobj_t) {
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_boscub as i32);
    A_SpawnFly(mo);
}
pub unsafe fn A_SpawnFly(mut mo: *mut mobj_t) {
    let mut newmobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut fog: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut targ: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut r: i32 = 0;
    let mut type_0: mobjtype_t = MT_PLAYER;
    (*mo).reactiontime -= 1;
    if (*mo).reactiontime != 0 {
        return;
    }
    targ = P_SubstNullMobj((*mo).target as *mut mobj_t);
    fog = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, MT_SPAWNFIRE);
    S_StartSound(fog as *mut ::core::ffi::c_void, sfx_telept as i32);
    r = P_Random();
    if r < 50 as i32 {
        type_0 = MT_TROOP;
    } else if r < 90 as i32 {
        type_0 = MT_SERGEANT;
    } else if r < 120 as i32 {
        type_0 = MT_SHADOWS;
    } else if r < 130 as i32 {
        type_0 = MT_PAIN;
    } else if r < 160 as i32 {
        type_0 = MT_HEAD;
    } else if r < 162 as i32 {
        type_0 = MT_VILE;
    } else if r < 172 as i32 {
        type_0 = MT_UNDEAD;
    } else if r < 192 as i32 {
        type_0 = MT_BABY;
    } else if r < 222 as i32 {
        type_0 = MT_FATSO;
    } else if r < 246 as i32 {
        type_0 = MT_KNIGHT;
    } else {
        type_0 = MT_BRUISER;
    }
    newmobj = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, type_0);
    if P_LookForPlayers(newmobj, true) {
        P_SetMobjState(newmobj, (*(*newmobj).info).seestate as statenum_t);
    }
    P_TeleportMove(newmobj, (*newmobj).x, (*newmobj).y);
    P_RemoveMobj(mo);
}
pub unsafe fn A_PlayerScream(mut mo: *mut mobj_t) {
    let mut sound: i32 = sfx_pldeth as i32;
    if gamemode as u32
        == commercial as i32 as u32
        && (*mo).health < -(50 as i32)
    {
        sound = sfx_pdiehi as i32;
    }
    S_StartSound(mo as *mut ::core::ffi::c_void, sound);
}
