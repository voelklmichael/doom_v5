use crate::src::p_maputl::{intercept_t};
use crate::src::p_mobj::{sector_t, ST_VERTICAL, ST_HORIZONTAL, line_t, subsector_t};
use crate::src::d_player::{player_t};
use crate::src::p_mobj::{mobj_t};
use crate::src::i_system::I_Error;
use crate::src::m_argv::{myargv, M_CheckParmWithArgs};
use crate::src::p_mobj::P_SpawnBlood;
use crate::src::p_maputl::P_PointOnLineSide;
use crate::src::p_maputl::P_BoxOnLineSide;
use crate::src::p_maputl::opentop;
use crate::src::p_maputl::openbottom;
use crate::src::p_maputl::lowfloor;
use crate::src::p_maputl::P_BlockLinesIterator;
use crate::src::p_maputl::trace;
use crate::src::p_maputl::P_PathTraverse;
use crate::src::p_inter::P_TouchSpecialThing;
use crate::src::p_spec::P_ShootSpecialLine;
use crate::src::p_spec::P_CrossSpecialLine;
use crate::src::p_sight::topslope;
use crate::src::p_sight::bottomslope;
use crate::src::p_maputl::P_LineOpening;
use crate::src::p_maputl::P_BlockThingsIterator;
use crate::src::p_maputl::openrange;
use crate::src::p_mobj::P_SpawnPuff;
use crate::src::p_mobj::P_SubstNullMobj;
use crate::src::p_sight::P_CheckSight;
use crate::src::p_switch::P_UseSpecialLine;
use crate::src::m_misc::M_StrToInt;
use crate::src::p_maputl::P_AproxDistance;
use crate::src::p_maputl::P_UnsetThingPosition;
use crate::src::p_inter::P_DamageMobj;
use crate::src::p_maputl::P_SetThingPosition;
use crate::src::p_setup::bmaporgx;
use crate::src::p_setup::bmaporgy;
use crate::src::r_main::R_PointInSubsector;
use crate::src::p_mobj::P_SetMobjState;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::p_setup::lines;
use crate::src::r_main::validcount;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::r_sky::skyflatnum;
use crate::src::g_game::gamemap;
use crate::src::m_fixed::FixedDiv;
use crate::src::r_main::R_PointToAngle2;
use crate::src::m_random::P_Random;
use crate::src::p_tick::leveltime;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::s_sound::S_StartSound;
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::src::p_mobj::{MF_DROPOFF, MF_DROPPED, MF_FLOAT, MF_MISSILE, MF_NOBLOOD, MF_NOCLIP, MF_PICKUP, MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID, MF_SPECIAL, MF_TELEPORT};
use crate::src::sounds::sfx_noway;
use crate::src::i_system::{fprintf, stderr};
use crate::src::p_mobj::{MT_BLOOD, MT_BRUISER, MT_CYBORG, MT_KNIGHT, MT_PLAYER, MT_SPIDER};
use crate::src::p_mobj::statenum_t;
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::doomdef::boolean;
use crate::src::info::{S_GIBS};
use crate::src::doomdef::true_0;
use crate::src::doomdef::false_0;
use crate::src::m_fixed::FRACUNIT;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG180;
use crate::src::p_spec::ML_TWOSIDED;
use crate::src::p_maputl::PT_ADDLINES;
use crate::src::p_maputl::PT_ADDTHINGS;
use crate::src::p_maputl::MAPBLOCKSHIFT;
use crate::src::m_fixed::FRACBITS;

pub const DEH_DEFAULT_SPECIES_INFIGHTING: i32 = 0;
pub const deh_species_infighting: i32 = DEH_DEFAULT_SPECIES_INFIGHTING;
pub const ML_BLOCKING: i32 = 1;
pub const ML_BLOCKMONSTERS: i32 = 2;
pub const USERANGE: i32 = 64 * FRACUNIT;
pub const MAXSPECIALCROSS_ORIGINAL: i32 = 8;
pub const DEFAULT_SPECHIT_MAGIC: i32 = 0x1c09c98;
#[no_mangle]
pub static mut tmbbox: [fixed_t; 4] = [0; 4];
#[no_mangle]
pub static mut tmthing: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut tmflags: i32 = 0;
#[no_mangle]
pub static mut tmx: fixed_t = 0;
#[no_mangle]
pub static mut tmy: fixed_t = 0;
pub static mut floatok: bool = false;
pub static mut tmfloorz: fixed_t = 0;
#[no_mangle]
pub static mut tmceilingz: fixed_t = 0;
#[no_mangle]
pub static mut tmdropoffz: fixed_t = 0;
pub static mut ceilingline: *mut line_t = ::core::ptr::null::<line_t>() as *mut line_t;
pub static mut spechit: [*mut line_t; 20] = [::core::ptr::null::<line_t>()
    as *mut line_t; 20];
pub static mut numspechit: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn PIT_StompThing(mut thing: *mut mobj_t) -> boolean {
    let mut blockdist: fixed_t = 0;
    if (*thing).flags & MF_SHOOTABLE as i32 == 0 {
        return true_0 as boolean;
    }
    blockdist = (*thing).radius + (*tmthing).radius;
    if ((*thing).x as i32 - tmx as i32).abs() >= blockdist
        || ((*thing).y as i32 - tmy as i32).abs() >= blockdist
    {
        return true_0 as boolean;
    }
    if thing == tmthing {
        return true_0 as boolean;
    }
    if (*tmthing).player.is_null() && gamemap != 30 as i32 {
        return false_0 as boolean;
    }
    P_DamageMobj(thing, tmthing, tmthing, 10000 as i32);
    return true_0 as boolean;
}
pub unsafe fn P_TeleportMove(
    mut thing: *mut mobj_t,
    mut x: fixed_t,
    mut y: fixed_t,
) -> bool {
    let mut xl: i32 = 0;
    let mut xh: i32 = 0;
    let mut yl: i32 = 0;
    let mut yh: i32 = 0;
    let mut bx: i32 = 0;
    let mut by: i32 = 0;
    let mut newsubsec: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    tmthing = thing;
    tmflags = (*thing).flags;
    tmx = x;
    tmy = y;
    tmbbox[BOXTOP as i32 as usize] = y + (*tmthing).radius;
    tmbbox[BOXBOTTOM as i32 as usize] = y - (*tmthing).radius;
    tmbbox[BOXRIGHT as i32 as usize] = x + (*tmthing).radius;
    tmbbox[BOXLEFT as i32 as usize] = x - (*tmthing).radius;
    newsubsec = R_PointInSubsector(x, y);
    ceilingline = ::core::ptr::null_mut::<line_t>();
    tmdropoffz = (*(*newsubsec).sector).floorheight;
    tmfloorz = tmdropoffz;
    tmceilingz = (*(*newsubsec).sector).ceilingheight;
    validcount += 1;
    numspechit = 0 as i32;
    xl = tmbbox[BOXLEFT as i32 as usize] - bmaporgx as i32
        - 32 as i32 * FRACUNIT >> MAPBLOCKSHIFT;
    xh = tmbbox[BOXRIGHT as i32 as usize] - bmaporgx as i32
        + 32 as i32 * FRACUNIT >> MAPBLOCKSHIFT;
    yl = tmbbox[BOXBOTTOM as i32 as usize]
        - bmaporgy as i32 - 32 as i32 * FRACUNIT
        >> MAPBLOCKSHIFT;
    yh = tmbbox[BOXTOP as i32 as usize] - bmaporgy as i32
        + 32 as i32 * FRACUNIT >> MAPBLOCKSHIFT;
    bx = xl;
    while bx <= xh {
        by = yl;
        while by <= yh {
            if !P_BlockThingsIterator(
                bx,
                by,
                Some(PIT_StompThing as unsafe extern "C" fn(*mut mobj_t) -> boolean),
            )
            {
                return false;
            }
            by += 1;
        }
        bx += 1;
    }
    P_UnsetThingPosition(thing);
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    (*thing).x = x;
    (*thing).y = y;
    P_SetThingPosition(thing);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn PIT_CheckLine(mut ld: *mut line_t) -> boolean {
    if tmbbox[BOXRIGHT as i32 as usize]
        <= (*ld).bbox[BOXLEFT as i32 as usize]
        || tmbbox[BOXLEFT as i32 as usize]
            >= (*ld).bbox[BOXRIGHT as i32 as usize]
        || tmbbox[BOXTOP as i32 as usize]
            <= (*ld).bbox[BOXBOTTOM as i32 as usize]
        || tmbbox[BOXBOTTOM as i32 as usize]
            >= (*ld).bbox[BOXTOP as i32 as usize]
    {
        return true_0 as boolean;
    }
    if P_BoxOnLineSide(&raw mut tmbbox as *mut fixed_t, ld) != -(1 as i32)
    {
        return true_0 as boolean;
    }
    if (*ld).backsector.is_null() {
        return false_0 as boolean;
    }
    if (*tmthing).flags & MF_MISSILE as i32 == 0 {
        if (*ld).flags as i32 & ML_BLOCKING != 0 {
            return false_0 as boolean;
        }
        if (*tmthing).player.is_null()
            && (*ld).flags as i32 & ML_BLOCKMONSTERS != 0
        {
            return false_0 as boolean;
        }
    }
    P_LineOpening(ld);
    if opentop < tmceilingz {
        tmceilingz = opentop;
        ceilingline = ld;
    }
    if openbottom > tmfloorz {
        tmfloorz = openbottom;
    }
    if lowfloor < tmdropoffz {
        tmdropoffz = lowfloor;
    }
    if (*ld).special != 0 {
        spechit[numspechit as usize] = ld;
        numspechit += 1;
        if numspechit > MAXSPECIALCROSS_ORIGINAL {
            SpechitOverrun(ld);
        }
    }
    return true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn PIT_CheckThing(mut thing: *mut mobj_t) -> boolean {
    let mut blockdist: fixed_t = 0;
    let mut solid: bool = false;
    let mut damage: i32 = 0;
    if (*thing).flags
        & (MF_SOLID as i32 | MF_SPECIAL as i32
            | MF_SHOOTABLE as i32) == 0
    {
        return true_0 as boolean;
    }
    blockdist = (*thing).radius + (*tmthing).radius;
    if ((*thing).x as i32 - tmx as i32).abs() >= blockdist
        || ((*thing).y as i32 - tmy as i32).abs() >= blockdist
    {
        return true_0 as boolean;
    }
    if thing == tmthing {
        return true_0 as boolean;
    }
    if (*tmthing).flags & MF_SKULLFLY as i32 != 0 {
        damage = (P_Random() % 8 as i32 + 1 as i32)
            * (*(*tmthing).info).damage;
        P_DamageMobj(thing, tmthing, tmthing, damage);
        (*tmthing).flags &= !(MF_SKULLFLY as i32);
        (*tmthing).momz = 0 as i32 as fixed_t;
        (*tmthing).momy = (*tmthing).momz;
        (*tmthing).momx = (*tmthing).momy;
        P_SetMobjState(tmthing, (*(*tmthing).info).spawnstate as statenum_t);
        return false_0 as boolean;
    }
    if (*tmthing).flags & MF_MISSILE as i32 != 0 {
        if (*tmthing).z > (*thing).z + (*thing).height {
            return true_0 as boolean;
        }
        if (*tmthing).z + (*tmthing).height < (*thing).z {
            return true_0 as boolean;
        }
        if !(*tmthing).target.is_null()
            && ((*(*tmthing).target).type_0 as u32
                == (*thing).type_0 as u32
                || (*(*tmthing).target).type_0 as u32
                    == MT_KNIGHT as i32 as u32
                    && (*thing).type_0 as u32
                        == MT_BRUISER as i32 as u32
                || (*(*tmthing).target).type_0 as u32
                    == MT_BRUISER as i32 as u32
                    && (*thing).type_0 as u32
                        == MT_KNIGHT as i32 as u32)
        {
            if thing == (*tmthing).target {
                return true_0 as boolean;
            }
            if (*thing).type_0 as u32
                != MT_PLAYER as i32 as u32
                && deh_species_infighting == 0
            {
                return false_0 as boolean;
            }
        }
        if (*thing).flags & MF_SHOOTABLE as i32 == 0 {
            return ((*thing).flags & MF_SOLID as i32 == 0)
                as i32 as boolean;
        }
        damage = (P_Random() % 8 as i32 + 1 as i32)
            * (*(*tmthing).info).damage;
        P_DamageMobj(thing, tmthing, (*tmthing).target as *mut mobj_t, damage);
        return false_0 as boolean;
    }
    if (*thing).flags & MF_SPECIAL as i32 != 0 {
        solid = (*thing).flags & MF_SOLID as i32 != 0;
        if tmflags & MF_PICKUP as i32 != 0 {
            P_TouchSpecialThing(thing, tmthing);
        }
        return (!solid) as i32 as boolean;
    }
    return ((*thing).flags & MF_SOLID as i32 == 0) as i32
        as boolean;
}
pub unsafe fn P_CheckPosition(
    mut thing: *mut mobj_t,
    mut x: fixed_t,
    mut y: fixed_t,
) -> bool {
    let mut xl: i32 = 0;
    let mut xh: i32 = 0;
    let mut yl: i32 = 0;
    let mut yh: i32 = 0;
    let mut bx: i32 = 0;
    let mut by: i32 = 0;
    let mut newsubsec: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    tmthing = thing;
    tmflags = (*thing).flags;
    tmx = x;
    tmy = y;
    tmbbox[BOXTOP as i32 as usize] = y + (*tmthing).radius;
    tmbbox[BOXBOTTOM as i32 as usize] = y - (*tmthing).radius;
    tmbbox[BOXRIGHT as i32 as usize] = x + (*tmthing).radius;
    tmbbox[BOXLEFT as i32 as usize] = x - (*tmthing).radius;
    newsubsec = R_PointInSubsector(x, y);
    ceilingline = ::core::ptr::null_mut::<line_t>();
    tmdropoffz = (*(*newsubsec).sector).floorheight;
    tmfloorz = tmdropoffz;
    tmceilingz = (*(*newsubsec).sector).ceilingheight;
    validcount += 1;
    numspechit = 0 as i32;
    if tmflags & MF_NOCLIP as i32 != 0 {
        return true;
    }
    xl = tmbbox[BOXLEFT as i32 as usize] - bmaporgx as i32
        - 32 as i32 * FRACUNIT >> MAPBLOCKSHIFT;
    xh = tmbbox[BOXRIGHT as i32 as usize] - bmaporgx as i32
        + 32 as i32 * FRACUNIT >> MAPBLOCKSHIFT;
    yl = tmbbox[BOXBOTTOM as i32 as usize]
        - bmaporgy as i32 - 32 as i32 * FRACUNIT
        >> MAPBLOCKSHIFT;
    yh = tmbbox[BOXTOP as i32 as usize] - bmaporgy as i32
        + 32 as i32 * FRACUNIT >> MAPBLOCKSHIFT;
    bx = xl;
    while bx <= xh {
        by = yl;
        while by <= yh {
            if !P_BlockThingsIterator(
                bx,
                by,
                Some(PIT_CheckThing as unsafe extern "C" fn(*mut mobj_t) -> boolean),
            )
            {
                return false;
            }
            by += 1;
        }
        bx += 1;
    }
    xl = (tmbbox[BOXLEFT as i32 as usize] - bmaporgx >> MAPBLOCKSHIFT)
        as i32;
    xh = (tmbbox[BOXRIGHT as i32 as usize] - bmaporgx >> MAPBLOCKSHIFT)
        as i32;
    yl = (tmbbox[BOXBOTTOM as i32 as usize] - bmaporgy >> MAPBLOCKSHIFT)
        as i32;
    yh = (tmbbox[BOXTOP as i32 as usize] - bmaporgy >> MAPBLOCKSHIFT)
        as i32;
    bx = xl;
    while bx <= xh {
        by = yl;
        while by <= yh {
            if !P_BlockLinesIterator(
                bx,
                by,
                Some(PIT_CheckLine as unsafe extern "C" fn(*mut line_t) -> boolean),
            )
            {
                return false;
            }
            by += 1;
        }
        bx += 1;
    }
    return true;
}
pub unsafe fn P_TryMove(
    mut thing: *mut mobj_t,
    mut x: fixed_t,
    mut y: fixed_t,
) -> bool {
    let mut oldx: fixed_t = 0;
    let mut oldy: fixed_t = 0;
    let mut side: i32 = 0;
    let mut oldside: i32 = 0;
    let mut ld: *mut line_t = ::core::ptr::null_mut::<line_t>();
    floatok = false;
    if !P_CheckPosition(thing, x, y) {
        return false;
    }
    if (*thing).flags & MF_NOCLIP as i32 == 0 {
        if tmceilingz - tmfloorz < (*thing).height {
            return false;
        }
        floatok = true;
        if (*thing).flags & MF_TELEPORT as i32 == 0
            && tmceilingz - (*thing).z < (*thing).height
        {
            return false;
        }
        if (*thing).flags & MF_TELEPORT as i32 == 0
            && tmfloorz - (*thing).z > 24 as i32 * FRACUNIT
        {
            return false;
        }
        if (*thing).flags
            & (MF_DROPOFF as i32 | MF_FLOAT as i32) == 0
            && tmfloorz - tmdropoffz > 24 as i32 * FRACUNIT
        {
            return false;
        }
    }
    P_UnsetThingPosition(thing);
    oldx = (*thing).x;
    oldy = (*thing).y;
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    (*thing).x = x;
    (*thing).y = y;
    P_SetThingPosition(thing);
    if (*thing).flags
        & (MF_TELEPORT as i32 | MF_NOCLIP as i32) == 0
    {
        loop {
            let fresh0 = numspechit;
            numspechit = numspechit - 1;
            if !(fresh0 != 0) {
                break;
            }
            ld = spechit[numspechit as usize];
            side = P_PointOnLineSide((*thing).x, (*thing).y, ld);
            oldside = P_PointOnLineSide(oldx, oldy, ld);
            if side != oldside {
                if (*ld).special != 0 {
                    P_CrossSpecialLine(
                        ld.offset_from(lines) as i64
                            as i32,
                        oldside,
                        thing,
                    );
                }
            }
        }
    }
    return true;
}
pub unsafe fn P_ThingHeightClip(mut thing: *mut mobj_t) -> bool {
    let mut onfloor: bool;
    onfloor = (*thing).z == (*thing).floorz;
    P_CheckPosition(thing, (*thing).x, (*thing).y);
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    if onfloor {
        (*thing).z = (*thing).floorz;
    } else if (*thing).z + (*thing).height > (*thing).ceilingz {
        (*thing).z = (*thing).ceilingz - (*thing).height;
    }
    if (*thing).ceilingz - (*thing).floorz < (*thing).height {
        return false;
    }
    return true;
}
#[no_mangle]
pub static mut bestslidefrac: fixed_t = 0;
#[no_mangle]
pub static mut secondslidefrac: fixed_t = 0;
#[no_mangle]
pub static mut bestslideline: *mut line_t = ::core::ptr::null::<line_t>() as *mut line_t;
#[no_mangle]
pub static mut secondslideline: *mut line_t = ::core::ptr::null::<line_t>()
    as *mut line_t;
#[no_mangle]
pub static mut slidemo: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut tmxmove: fixed_t = 0;
#[no_mangle]
pub static mut tmymove: fixed_t = 0;
pub unsafe fn P_HitSlideLine(mut ld: *mut line_t) {
    let mut side: i32 = 0;
    let mut lineangle: angle_t = 0;
    let mut moveangle: angle_t = 0;
    let mut deltaangle: angle_t = 0;
    let mut movelen: fixed_t = 0;
    let mut newlen: fixed_t = 0;
    if (*ld).slopetype as u32
        == ST_HORIZONTAL as i32 as u32
    {
        tmymove = 0 as i32 as fixed_t;
        return;
    }
    if (*ld).slopetype as u32
        == ST_VERTICAL as i32 as u32
    {
        tmxmove = 0 as i32 as fixed_t;
        return;
    }
    side = P_PointOnLineSide((*slidemo).x, (*slidemo).y, ld);
    lineangle = R_PointToAngle2(0 as fixed_t, 0 as fixed_t, (*ld).dx, (*ld).dy);
    if side == 1 as i32 {
        lineangle = (lineangle as u32).wrapping_add(ANG180) as angle_t
            as angle_t;
    }
    moveangle = R_PointToAngle2(0 as fixed_t, 0 as fixed_t, tmxmove, tmymove);
    deltaangle = moveangle.wrapping_sub(lineangle);
    if deltaangle > ANG180 {
        deltaangle = (deltaangle as u32).wrapping_add(ANG180) as angle_t
            as angle_t;
    }
    lineangle >>= ANGLETOFINESHIFT;
    deltaangle >>= ANGLETOFINESHIFT;
    movelen = P_AproxDistance(tmxmove, tmymove);
    newlen = FixedMul(movelen, finecosine[deltaangle as isize]);
    tmxmove = FixedMul(newlen, finecosine[lineangle as isize]);
    tmymove = FixedMul(newlen, finesine[lineangle as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn PTR_SlideTraverse(mut in_0: *mut intercept_t) -> boolean {
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    if !(*in_0).isaline {
        I_Error("PTR_SlideTraverse: not a line?");
    }
    li = (*in_0).d.line;
    if (*li).flags as i32 & ML_TWOSIDED == 0 {
        if P_PointOnLineSide((*slidemo).x, (*slidemo).y, li) != 0 {
            return true_0 as boolean;
        }
    } else {
        P_LineOpening(li);
        if !(openrange < (*slidemo).height) {
            if !(opentop - (*slidemo).z < (*slidemo).height) {
                if !(openbottom - (*slidemo).z > 24 as i32 * FRACUNIT) {
                    return true_0 as boolean;
                }
            }
        }
    }
    if (*in_0).frac < bestslidefrac {
        secondslidefrac = bestslidefrac;
        secondslideline = bestslideline;
        bestslidefrac = (*in_0).frac;
        bestslideline = li;
    }
    return false_0 as boolean;
}
pub unsafe fn P_SlideMove(mut mo: *mut mobj_t) {
    let mut leadx: fixed_t = 0;
    let mut leady: fixed_t = 0;
    let mut trailx: fixed_t = 0;
    let mut traily: fixed_t = 0;
    let mut newx: fixed_t = 0;
    let mut newy: fixed_t = 0;
    let mut hitcount: i32 = 0;
    slidemo = mo;
    hitcount = 0 as i32;
    loop {
        hitcount += 1;
        if hitcount == 3 as i32 {
            break;
        }
        if (*mo).momx > 0 as i32 {
            leadx = (*mo).x + (*mo).radius;
            trailx = (*mo).x - (*mo).radius;
        } else {
            leadx = (*mo).x - (*mo).radius;
            trailx = (*mo).x + (*mo).radius;
        }
        if (*mo).momy > 0 as i32 {
            leady = (*mo).y + (*mo).radius;
            traily = (*mo).y - (*mo).radius;
        } else {
            leady = (*mo).y - (*mo).radius;
            traily = (*mo).y + (*mo).radius;
        }
        bestslidefrac = (FRACUNIT + 1 as i32) as fixed_t;
        P_PathTraverse(
            leadx,
            leady,
            leadx + (*mo).momx,
            leady + (*mo).momy,
            PT_ADDLINES,
            Some(PTR_SlideTraverse as unsafe extern "C" fn(*mut intercept_t) -> boolean),
        );
        P_PathTraverse(
            trailx,
            leady,
            trailx + (*mo).momx,
            leady + (*mo).momy,
            PT_ADDLINES,
            Some(PTR_SlideTraverse as unsafe extern "C" fn(*mut intercept_t) -> boolean),
        );
        P_PathTraverse(
            leadx,
            traily,
            leadx + (*mo).momx,
            traily + (*mo).momy,
            PT_ADDLINES,
            Some(PTR_SlideTraverse as unsafe extern "C" fn(*mut intercept_t) -> boolean),
        );
        if bestslidefrac == FRACUNIT + 1 as i32 {
            break;
        }
        bestslidefrac -= 0x800 as i32;
        if bestslidefrac > 0 as i32 {
            newx = FixedMul((*mo).momx, bestslidefrac);
            newy = FixedMul((*mo).momy, bestslidefrac);
            if !P_TryMove(mo, (*mo).x + newx, (*mo).y + newy) {
                break;
            }
        }
        bestslidefrac = (FRACUNIT
            - (bestslidefrac as i32 + 0x800 as i32))
            as fixed_t;
        if bestslidefrac > FRACUNIT {
            bestslidefrac = FRACUNIT as fixed_t;
        }
        if bestslidefrac <= 0 as i32 {
            return;
        }
        tmxmove = FixedMul((*mo).momx, bestslidefrac);
        tmymove = FixedMul((*mo).momy, bestslidefrac);
        P_HitSlideLine(bestslideline);
        (*mo).momx = tmxmove;
        (*mo).momy = tmymove;
        if !P_TryMove(mo, (*mo).x + tmxmove, (*mo).y + tmymove) {
            continue;
        }
        return;
    }
    if !P_TryMove(mo, (*mo).x, (*mo).y + (*mo).momy) {
        P_TryMove(mo, (*mo).x + (*mo).momx, (*mo).y);
    }
}
pub static mut linetarget: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut shootthing: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut shootz: fixed_t = 0;
#[no_mangle]
pub static mut la_damage: i32 = 0;
pub static mut attackrange: fixed_t = 0;
#[no_mangle]
pub static mut aimslope: fixed_t = 0;
#[no_mangle]
pub unsafe extern "C" fn PTR_AimTraverse(mut in_0: *mut intercept_t) -> boolean {
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut slope: fixed_t = 0;
    let mut thingtopslope: fixed_t = 0;
    let mut thingbottomslope: fixed_t = 0;
    let mut dist: fixed_t = 0;
    if (*in_0).isaline {
        li = (*in_0).d.line;
        if (*li).flags as i32 & ML_TWOSIDED == 0 {
            return false_0 as boolean;
        }
        P_LineOpening(li);
        if openbottom >= opentop {
            return false_0 as boolean;
        }
        dist = FixedMul(attackrange, (*in_0).frac);
        if (*li).backsector.is_null()
            || (*(*li).frontsector).floorheight != (*(*li).backsector).floorheight
        {
            slope = FixedDiv(openbottom - shootz, dist);
            if slope > bottomslope {
                bottomslope = slope;
            }
        }
        if (*li).backsector.is_null()
            || (*(*li).frontsector).ceilingheight != (*(*li).backsector).ceilingheight
        {
            slope = FixedDiv(opentop - shootz, dist);
            if slope < topslope {
                topslope = slope;
            }
        }
        if topslope <= bottomslope {
            return false_0 as boolean;
        }
        return true_0 as boolean;
    }
    th = (*in_0).d.thing;
    if th == shootthing {
        return true_0 as boolean;
    }
    if (*th).flags & MF_SHOOTABLE as i32 == 0 {
        return true_0 as boolean;
    }
    dist = FixedMul(attackrange, (*in_0).frac);
    thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);
    if thingtopslope < bottomslope {
        return true_0 as boolean;
    }
    thingbottomslope = FixedDiv((*th).z - shootz, dist);
    if thingbottomslope > topslope {
        return true_0 as boolean;
    }
    if thingtopslope > topslope {
        thingtopslope = topslope;
    }
    if thingbottomslope < bottomslope {
        thingbottomslope = bottomslope;
    }
    aimslope = ((thingtopslope as i32
        + thingbottomslope as i32) / 2 as i32) as fixed_t;
    linetarget = th;
    return false_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn PTR_ShootTraverse(mut in_0: *mut intercept_t) -> boolean {
    let mut current_block: u64;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut frac: fixed_t = 0;
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut slope: fixed_t = 0;
    let mut dist: fixed_t = 0;
    let mut thingtopslope: fixed_t = 0;
    let mut thingbottomslope: fixed_t = 0;
    if (*in_0).isaline {
        li = (*in_0).d.line;
        if (*li).special != 0 {
            P_ShootSpecialLine(shootthing, li);
        }
        if !((*li).flags as i32 & ML_TWOSIDED == 0) {
            P_LineOpening(li);
            dist = FixedMul(attackrange, (*in_0).frac);
            if (*li).backsector.is_null() {
                slope = FixedDiv(openbottom - shootz, dist);
                if slope > aimslope {
                    current_block = 15534775465039326179;
                } else {
                    slope = FixedDiv(opentop - shootz, dist);
                    if slope < aimslope {
                        current_block = 15534775465039326179;
                    } else {
                        current_block = 4808432441040389987;
                    }
                }
            } else {
                if (*(*li).frontsector).floorheight != (*(*li).backsector).floorheight {
                    slope = FixedDiv(openbottom - shootz, dist);
                    if slope > aimslope {
                        current_block = 15534775465039326179;
                    } else {
                        current_block = 12039483399334584727;
                    }
                } else {
                    current_block = 12039483399334584727;
                }
                match current_block {
                    15534775465039326179 => {}
                    _ => {
                        if (*(*li).frontsector).ceilingheight
                            != (*(*li).backsector).ceilingheight
                        {
                            slope = FixedDiv(opentop - shootz, dist);
                            if slope < aimslope {
                                current_block = 15534775465039326179;
                            } else {
                                current_block = 4808432441040389987;
                            }
                        } else {
                            current_block = 4808432441040389987;
                        }
                    }
                }
            }
            match current_block {
                15534775465039326179 => {}
                _ => return true_0 as boolean,
            }
        }
        frac = (*in_0).frac - FixedDiv(4 as fixed_t * FRACUNIT, attackrange);
        x = trace.x + FixedMul(trace.dx, frac);
        y = trace.y + FixedMul(trace.dy, frac);
        z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));
        if (*(*li).frontsector).ceilingpic as i32 == skyflatnum {
            if z > (*(*li).frontsector).ceilingheight {
                return false_0 as boolean;
            }
            if !(*li).backsector.is_null()
                && (*(*li).backsector).ceilingpic as i32 == skyflatnum
            {
                return false_0 as boolean;
            }
        }
        P_SpawnPuff(x, y, z);
        return false_0 as boolean;
    } else {
        th = (*in_0).d.thing;
        if th == shootthing {
            return true_0 as boolean;
        }
        if (*th).flags & MF_SHOOTABLE as i32 == 0 {
            return true_0 as boolean;
        }
        dist = FixedMul(attackrange, (*in_0).frac);
        thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);
        if thingtopslope < aimslope {
            return true_0 as boolean;
        }
        thingbottomslope = FixedDiv((*th).z - shootz, dist);
        if thingbottomslope > aimslope {
            return true_0 as boolean;
        }
        frac = (*in_0).frac - FixedDiv(10 as fixed_t * FRACUNIT, attackrange);
        x = trace.x + FixedMul(trace.dx, frac);
        y = trace.y + FixedMul(trace.dy, frac);
        z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));
        if (*(*in_0).d.thing).flags & MF_NOBLOOD as i32 != 0 {
            P_SpawnPuff(x, y, z);
        } else {
            P_SpawnBlood(x, y, z, la_damage);
        }
        if la_damage != 0 {
            P_DamageMobj(th, shootthing, shootthing, la_damage);
        }
        return false_0 as boolean;
    };
}
pub unsafe fn P_AimLineAttack(
    mut t1: *mut mobj_t,
    mut angle: angle_t,
    mut distance: fixed_t,
) -> fixed_t {
    let mut x2: fixed_t = 0;
    let mut y2: fixed_t = 0;
    t1 = P_SubstNullMobj(t1);
    angle >>= ANGLETOFINESHIFT;
    shootthing = t1;
    x2 = (*t1).x + (distance >> FRACBITS) * finecosine[angle as isize];
    y2 = (*t1).y + (distance >> FRACBITS) * finesine[angle as usize];
    shootz = ((*t1).z as i32
        + ((*t1).height as i32 >> 1 as i32)
        + 8 as i32 * FRACUNIT) as fixed_t;
    topslope = (100 as i32 * FRACUNIT / 160 as i32)
        as fixed_t;
    bottomslope = (-(100 as i32) * FRACUNIT / 160 as i32)
        as fixed_t;
    attackrange = distance;
    linetarget = ::core::ptr::null_mut::<mobj_t>();
    P_PathTraverse(
        (*t1).x,
        (*t1).y,
        x2,
        y2,
        PT_ADDLINES | PT_ADDTHINGS,
        Some(PTR_AimTraverse as unsafe extern "C" fn(*mut intercept_t) -> boolean),
    );
    if !linetarget.is_null() {
        return aimslope;
    }
    return 0 as fixed_t;
}
pub unsafe fn P_LineAttack(
    mut t1: *mut mobj_t,
    mut angle: angle_t,
    mut distance: fixed_t,
    mut slope: fixed_t,
    mut damage: i32,
) {
    let mut x2: fixed_t = 0;
    let mut y2: fixed_t = 0;
    angle >>= ANGLETOFINESHIFT;
    shootthing = t1;
    la_damage = damage;
    x2 = (*t1).x + (distance >> FRACBITS) * finecosine[angle as isize];
    y2 = (*t1).y + (distance >> FRACBITS) * finesine[angle as usize];
    shootz = ((*t1).z as i32
        + ((*t1).height as i32 >> 1 as i32)
        + 8 as i32 * FRACUNIT) as fixed_t;
    attackrange = distance;
    aimslope = slope;
    P_PathTraverse(
        (*t1).x,
        (*t1).y,
        x2,
        y2,
        PT_ADDLINES | PT_ADDTHINGS,
        Some(PTR_ShootTraverse as unsafe extern "C" fn(*mut intercept_t) -> boolean),
    );
}
#[no_mangle]
pub static mut usething: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub unsafe extern "C" fn PTR_UseTraverse(mut in_0: *mut intercept_t) -> boolean {
    let mut side: i32 = 0;
    if (*(*in_0).d.line).special == 0 {
        P_LineOpening((*in_0).d.line);
        if openrange <= 0 as i32 {
            S_StartSound(
                usething as *mut ::core::ffi::c_void,
                sfx_noway as i32,
            );
            return false_0 as boolean;
        }
        return true_0 as boolean;
    }
    side = 0 as i32;
    if P_PointOnLineSide((*usething).x, (*usething).y, (*in_0).d.line)
        == 1 as i32
    {
        side = 1 as i32;
    }
    P_UseSpecialLine(usething, (*in_0).d.line, side);
    return false_0 as boolean;
}
pub unsafe fn P_UseLines(mut player: *mut player_t) {
    let mut angle: i32 = 0;
    let mut x1: fixed_t = 0;
    let mut y1: fixed_t = 0;
    let mut x2: fixed_t = 0;
    let mut y2: fixed_t = 0;
    usething = (*player).mo;
    angle = ((*(*player).mo).angle >> ANGLETOFINESHIFT) as i32;
    x1 = (*(*player).mo).x;
    y1 = (*(*player).mo).y;
    x2 = x1 + (USERANGE >> FRACBITS) * finecosine[angle as isize];
    y2 = y1 + (USERANGE >> FRACBITS) * finesine[angle as usize];
    P_PathTraverse(
        x1,
        y1,
        x2,
        y2,
        PT_ADDLINES,
        Some(PTR_UseTraverse as unsafe extern "C" fn(*mut intercept_t) -> boolean),
    );
}
#[no_mangle]
pub static mut bombsource: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut bombspot: *mut mobj_t = ::core::ptr::null::<mobj_t>() as *mut mobj_t;
#[no_mangle]
pub static mut bombdamage: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn PIT_RadiusAttack(mut thing: *mut mobj_t) -> boolean {
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut dist: fixed_t = 0;
    if (*thing).flags & MF_SHOOTABLE as i32 == 0 {
        return true_0 as boolean;
    }
    if (*thing).type_0 as u32
        == MT_CYBORG as i32 as u32
        || (*thing).type_0 as u32
            == MT_SPIDER as i32 as u32
    {
        return true_0 as boolean;
    }
    dx = ((*thing).x as i32 - (*bombspot).x as i32).abs()
        as fixed_t;
    dy = ((*thing).y as i32 - (*bombspot).y as i32).abs()
        as fixed_t;
    dist = if dx > dy { dx } else { dy };
    dist = dist - (*thing).radius >> FRACBITS;
    if dist < 0 as i32 {
        dist = 0 as i32 as fixed_t;
    }
    if dist >= bombdamage {
        return true_0 as boolean;
    }
    if P_CheckSight(thing, bombspot) {
        P_DamageMobj(
            thing,
            bombspot,
            bombsource,
            bombdamage - dist as i32,
        );
    }
    return true_0 as boolean;
}
pub unsafe fn P_RadiusAttack(
    mut spot: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: i32,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xl: i32 = 0;
    let mut xh: i32 = 0;
    let mut yl: i32 = 0;
    let mut yh: i32 = 0;
    let mut dist: fixed_t = 0;
    dist = (damage + 32 as i32 * FRACUNIT << FRACBITS) as fixed_t;
    yh = ((*spot).y + dist - bmaporgy >> MAPBLOCKSHIFT) as i32;
    yl = ((*spot).y - dist - bmaporgy >> MAPBLOCKSHIFT) as i32;
    xh = ((*spot).x + dist - bmaporgx >> MAPBLOCKSHIFT) as i32;
    xl = ((*spot).x - dist - bmaporgx >> MAPBLOCKSHIFT) as i32;
    bombspot = spot;
    bombsource = source;
    bombdamage = damage;
    y = yl;
    while y <= yh {
        x = xl;
        while x <= xh {
            P_BlockThingsIterator(
                x,
                y,
                Some(PIT_RadiusAttack as unsafe extern "C" fn(*mut mobj_t) -> boolean),
            );
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub static mut crushchange: boolean = 0;
#[no_mangle]
pub static mut nofit: boolean = 0;
#[no_mangle]
pub unsafe extern "C" fn PIT_ChangeSector(mut thing: *mut mobj_t) -> boolean {
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    if P_ThingHeightClip(thing) {
        return true_0 as boolean;
    }
    if (*thing).health <= 0 as i32 {
        P_SetMobjState(thing, S_GIBS);
        (*thing).flags &= !(MF_SOLID as i32);
        (*thing).height = 0 as i32 as fixed_t;
        (*thing).radius = 0 as i32 as fixed_t;
        return true_0 as boolean;
    }
    if (*thing).flags & MF_DROPPED as i32 != 0 {
        P_RemoveMobj(thing);
        return true_0 as boolean;
    }
    if (*thing).flags & MF_SHOOTABLE as i32 == 0 {
        return true_0 as boolean;
    }
    nofit = true_0 as boolean;
    if crushchange != 0 && leveltime & 3 as i32 == 0 {
        P_DamageMobj(
            thing,
            ::core::ptr::null_mut::<mobj_t>(),
            ::core::ptr::null_mut::<mobj_t>(),
            10 as i32,
        );
        mo = P_SpawnMobj(
            (*thing).x,
            (*thing).y,
            (*thing).z + (*thing).height / 2 as fixed_t,
            MT_BLOOD,
        );
        (*mo).momx = (P_Random() - P_Random() << 12 as i32) as fixed_t;
        (*mo).momy = (P_Random() - P_Random() << 12 as i32) as fixed_t;
    }
    return true_0 as boolean;
}
pub unsafe fn P_ChangeSector(
    mut sector: *mut sector_t,
    mut crunch: bool,
) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    nofit = false_0 as boolean;
    crushchange = crunch as i32 as boolean;
    x = (*sector).blockbox[BOXLEFT as i32 as usize];
    while x <= (*sector).blockbox[BOXRIGHT as i32 as usize] {
        y = (*sector).blockbox[BOXBOTTOM as i32 as usize];
        while y <= (*sector).blockbox[BOXTOP as i32 as usize] {
            P_BlockThingsIterator(
                x,
                y,
                Some(PIT_ChangeSector as unsafe extern "C" fn(*mut mobj_t) -> boolean),
            );
            y += 1;
        }
        x += 1;
    }
    return nofit != 0;
}
unsafe extern "C" fn SpechitOverrun(mut ld: *mut line_t) {
    static mut baseaddr: u32 = 0 as u32;
    let mut addr: u32 = 0;
    if baseaddr == 0 as u32 {
        let mut p: i32 = 0;
        p = M_CheckParmWithArgs("-spechit", 1 as i32);
        if p > 0 as i32 {
            M_StrToInt(
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
                &raw mut baseaddr as *mut i32,
            );
        } else {
            baseaddr = DEFAULT_SPECHIT_MAGIC as u32;
        }
    }
    addr = (baseaddr as i64
        + ld.offset_from(lines) as i64 * 0x3e as i64)
        as u32;
    match numspechit {
        9 | 10 | 11 | 12 => {
            tmbbox[(numspechit - 9 as i32) as usize] = addr as fixed_t;
        }
        13 => {
            crushchange = addr as boolean;
        }
        14 => {
            nofit = addr as boolean;
        }
        _ => {
            fprintf(
                stderr,
                b"SpechitOverrun: Warning: unable to emulatean overrun where numspechit=%i\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                numspechit,
            );
        }
    };
}
