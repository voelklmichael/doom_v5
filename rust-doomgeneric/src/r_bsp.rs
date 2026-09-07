use crate::src::game_state::game_state;
use crate::src::i_system::I_Error;
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::src::m_fixed::fixed_t;
use crate::src::p_mobj::{line_t, subsector_t};
use crate::src::p_setup::SectorId;
use crate::src::p_setup::SideId;
use crate::src::p_setup::SubsectorId;
use crate::src::r_defs::{drawseg_s, drawseg_t, node_t, seg_t, visplane_t};
use crate::src::r_main::R_PointOnSide;
use crate::src::r_main::R_PointToAngle;
use crate::src::r_plane::R_FindPlane;
use crate::src::r_segs::R_StoreWallRange;
use crate::src::r_things::R_AddSprites;
use crate::src::tables::angle_t;
use crate::src::tables::ANG180;
use crate::src::tables::ANG90;
use crate::src::tables::ANGLETOFINESHIFT;

pub struct RBspState {
    pub curline: *mut seg_t,
    pub sidedef: SideId,
    pub linedef: *mut line_t,
    pub frontsector: Option<SectorId>,
    pub backsector: Option<SectorId>,
    pub drawsegs: [drawseg_t; 256],
    pub ds_p: *mut drawseg_t,
    pub newend: *mut cliprange_t,
    pub solidsegs: [cliprange_t; 32],
}

impl RBspState {
    pub const fn new() -> Self {
        RBspState {
            curline: ::core::ptr::null::<seg_t>() as *mut seg_t,
            sidedef: SideId(0),
            linedef: ::core::ptr::null::<line_t>() as *mut line_t,
            frontsector: None,
            backsector: None,
            drawsegs: [drawseg_s {
        curline: ::core::ptr::null::<seg_t>() as *mut seg_t,
        x1: 0,
        x2: 0,
        scale1: 0,
        scale2: 0,
        scalestep: 0,
        silhouette: 0,
        bsilheight: 0,
        tsilheight: 0,
        sprtopclip: ::core::ptr::null::<i16>() as *mut i16,
        sprbottomclip: ::core::ptr::null::<i16>() as *mut i16,
        maskedtexturecol: ::core::ptr::null::<i16>() as *mut i16,
    }; 256],
            ds_p: ::core::ptr::null::<drawseg_t>() as *mut drawseg_t,
            newend: ::core::ptr::null::<cliprange_t>() as *mut cliprange_t,
            solidsegs: [cliprange_t { first: 0, last: 0 }; 32],
        }
    }
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct cliprange_t {
    pub first: i32,
    pub last: i32,
}
pub const NF_SUBSECTOR: i32 = 0x8000;
pub unsafe fn R_ClearDrawSegs() {
    unsafe { game_state() }.r_bsp.ds_p = &raw mut unsafe { game_state() }.r_bsp.drawsegs as *mut drawseg_t;
}
pub unsafe fn R_ClipSolidWallSegment(mut first: i32, mut last: i32) {
    let mut current_block: u64;
    let mut next: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    let mut start: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    start = &raw mut unsafe { game_state() }.r_bsp.solidsegs as *mut cliprange_t;
    while (*start).last < first - 1 as i32 {
        start = start.offset(1);
    }
    if first < (*start).first {
        if last < (*start).first - 1 as i32 {
            R_StoreWallRange(unsafe { game_state() }, first, last);
            next = unsafe { game_state() }.r_bsp.newend;
            unsafe { game_state() }.r_bsp.newend = unsafe { game_state() }.r_bsp.newend.offset(1);
            while next != start {
                *next = *next.offset(-(1 as i32 as isize));
                next = next.offset(-1);
            }
            (*next).first = first;
            (*next).last = last;
            return;
        }
        R_StoreWallRange(unsafe { game_state() }, first, (*start).first - 1 as i32);
        (*start).first = first;
    }
    if last <= (*start).last {
        return;
    }
    next = start;
    loop {
        if !(last >= (*next.offset(1 as i32 as isize)).first - 1 as i32) {
            current_block = 224731115979188411;
            break;
        }
        R_StoreWallRange(
            unsafe { game_state() },
            (*next).last + 1 as i32,
            (*next.offset(1 as i32 as isize)).first - 1 as i32,
        );
        next = next.offset(1);
        if !(last <= (*next).last) {
            continue;
        }
        (*start).last = (*next).last;
        current_block = 18287538169731164953;
        break;
    }
    match current_block {
        224731115979188411 => {
            R_StoreWallRange(unsafe { game_state() }, (*next).last + 1 as i32, last);
            (*start).last = last;
        }
        _ => {}
    }
    if next == start {
        return;
    }
    loop {
        let fresh0 = next;
        next = next.offset(1);
        if !(fresh0 != unsafe { game_state() }.r_bsp.newend) {
            break;
        }
        start = start.offset(1);
        *start = *next;
    }
    unsafe { game_state() }.r_bsp.newend = start.offset(1 as i32 as isize);
}
pub unsafe fn R_ClipPassWallSegment(mut first: i32, mut last: i32) {
    let mut start: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    start = &raw mut unsafe { game_state() }.r_bsp.solidsegs as *mut cliprange_t;
    while (*start).last < first - 1 as i32 {
        start = start.offset(1);
    }
    if first < (*start).first {
        if last < (*start).first - 1 as i32 {
            R_StoreWallRange(unsafe { game_state() }, first, last);
            return;
        }
        R_StoreWallRange(unsafe { game_state() }, first, (*start).first - 1 as i32);
    }
    if last <= (*start).last {
        return;
    }
    while last >= (*start.offset(1 as i32 as isize)).first - 1 as i32 {
        R_StoreWallRange(
            unsafe { game_state() },
            (*start).last + 1 as i32,
            (*start.offset(1 as i32 as isize)).first - 1 as i32,
        );
        start = start.offset(1);
        if last <= (*start).last {
            return;
        }
    }
    R_StoreWallRange(unsafe { game_state() }, (*start).last + 1 as i32, last);
}
pub unsafe fn R_ClearClipSegs() {
    unsafe { game_state() }.r_bsp.solidsegs[0 as i32 as usize].first = -(0x7fffffff as i32);
    unsafe { game_state() }.r_bsp.solidsegs[0 as i32 as usize].last = -(1 as i32);
    unsafe { game_state() }.r_bsp.solidsegs[1 as i32 as usize].first = unsafe { game_state() }.r_draw.viewwidth;
    unsafe { game_state() }.r_bsp.solidsegs[1 as i32 as usize].last = 0x7fffffff as i32;
    unsafe { game_state() }.r_bsp.newend = (&raw mut unsafe { game_state() }.r_bsp.solidsegs as *mut cliprange_t).offset(2 as i32 as isize);
}
pub unsafe fn R_AddLine(mut line: *mut seg_t) {
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut angle1: angle_t = 0;
    let mut angle2: angle_t = 0;
    let mut span: angle_t = 0;
    let mut tspan: angle_t = 0;
    unsafe { game_state() }.r_bsp.curline = line;
    angle1 = R_PointToAngle((*(*line).v1).x, (*(*line).v1).y);
    angle2 = R_PointToAngle((*(*line).v2).x, (*(*line).v2).y);
    span = angle1.wrapping_sub(angle2);
    if span >= ANG180 {
        return;
    }
    unsafe { game_state() }.r_segs.rw_angle1 = angle1 as i32;
    angle1 = angle1.wrapping_sub(unsafe { game_state() }.r_main.viewangle);
    angle2 = angle2.wrapping_sub(unsafe { game_state() }.r_main.viewangle);
    tspan = angle1.wrapping_add(unsafe { game_state() }.r_main.clipangle);
    if tspan > (2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle));
        if tspan >= span {
            return;
        }
        angle1 = unsafe { game_state() }.r_main.clipangle;
    }
    tspan = unsafe { game_state() }.r_main.clipangle.wrapping_sub(angle2);
    if tspan > (2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle));
        if tspan >= span {
            return;
        }
        angle2 = unsafe { game_state() }.r_main.clipangle.wrapping_neg();
    }
    angle1 = angle1.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    angle2 = angle2.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    x1 = unsafe { game_state() }.r_main.viewangletox[angle1 as usize];
    x2 = unsafe { game_state() }.r_main.viewangletox[angle2 as usize];
    if x1 == x2 {
        return;
    }
    unsafe { game_state() }.r_bsp.backsector = (*line).backsector;
    if !unsafe { game_state() }.r_bsp.backsector.is_none() {
        if !((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight <= (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight
            || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight >= (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight)
        {
            if !((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight != (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight
                || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight != (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight)
            {
                if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingpic as i32 == (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingpic as i32
                    && (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorpic as i32 == (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorpic as i32
                    && (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).lightlevel as i32 == (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32
                    && (*unsafe { game_state() }.p_setup.side_mut((*unsafe { game_state() }.r_bsp.curline).sidedef)).midtexture as i32 == 0 as i32
                {
                    return;
                }
            }
            R_ClipPassWallSegment(x1, x2 - 1 as i32);
            return;
        }
    }
    R_ClipSolidWallSegment(x1, x2 - 1 as i32);
}
#[no_mangle]
pub static checkcoord: [[i32; 4]; 12] = [
    [3 as i32, 0 as i32, 2 as i32, 1 as i32],
    [3 as i32, 0 as i32, 2 as i32, 0 as i32],
    [3 as i32, 1 as i32, 2 as i32, 0 as i32],
    [0 as i32; 4],
    [2 as i32, 0 as i32, 2 as i32, 1 as i32],
    [0 as i32, 0 as i32, 0 as i32, 0 as i32],
    [3 as i32, 1 as i32, 3 as i32, 0 as i32],
    [0 as i32; 4],
    [2 as i32, 0 as i32, 3 as i32, 1 as i32],
    [2 as i32, 1 as i32, 3 as i32, 1 as i32],
    [2 as i32, 1 as i32, 3 as i32, 0 as i32],
    [0; 4],
];
pub unsafe fn R_CheckBBox(mut bspcoord: *mut fixed_t) -> bool {
    let mut boxx: i32 = 0;
    let mut boxy: i32 = 0;
    let mut boxpos: i32 = 0;
    let mut x1: fixed_t = 0;
    let mut y1: fixed_t = 0;
    let mut x2: fixed_t = 0;
    let mut y2: fixed_t = 0;
    let mut angle1: angle_t = 0;
    let mut angle2: angle_t = 0;
    let mut span: angle_t = 0;
    let mut tspan: angle_t = 0;
    let mut start: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    let mut sx1: i32 = 0;
    let mut sx2: i32 = 0;
    if unsafe { game_state() }.r_main.viewx <= *bspcoord.offset(BOXLEFT as i32 as isize) {
        boxx = 0 as i32;
    } else if unsafe { game_state() }.r_main.viewx < *bspcoord.offset(BOXRIGHT as i32 as isize) {
        boxx = 1 as i32;
    } else {
        boxx = 2 as i32;
    }
    if unsafe { game_state() }.r_main.viewy >= *bspcoord.offset(BOXTOP as i32 as isize) {
        boxy = 0 as i32;
    } else if unsafe { game_state() }.r_main.viewy > *bspcoord.offset(BOXBOTTOM as i32 as isize) {
        boxy = 1 as i32;
    } else {
        boxy = 2 as i32;
    }
    boxpos = (boxy << 2 as i32) + boxx;
    if boxpos == 5 as i32 {
        return true;
    }
    x1 = *bspcoord.offset(checkcoord[boxpos as usize][0 as i32 as usize] as isize);
    y1 = *bspcoord.offset(checkcoord[boxpos as usize][1 as i32 as usize] as isize);
    x2 = *bspcoord.offset(checkcoord[boxpos as usize][2 as i32 as usize] as isize);
    y2 = *bspcoord.offset(checkcoord[boxpos as usize][3 as i32 as usize] as isize);
    angle1 = R_PointToAngle(x1, y1).wrapping_sub(unsafe { game_state() }.r_main.viewangle);
    angle2 = R_PointToAngle(x2, y2).wrapping_sub(unsafe { game_state() }.r_main.viewangle);
    span = angle1.wrapping_sub(angle2);
    if span >= ANG180 {
        return true;
    }
    tspan = angle1.wrapping_add(unsafe { game_state() }.r_main.clipangle);
    if tspan > (2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle));
        if tspan >= span {
            return false;
        }
        angle1 = unsafe { game_state() }.r_main.clipangle;
    }
    tspan = unsafe { game_state() }.r_main.clipangle.wrapping_sub(angle2);
    if tspan > (2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(unsafe { game_state() }.r_main.clipangle));
        if tspan >= span {
            return false;
        }
        angle2 = unsafe { game_state() }.r_main.clipangle.wrapping_neg();
    }
    angle1 = angle1.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    angle2 = angle2.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    sx1 = unsafe { game_state() }.r_main.viewangletox[angle1 as usize];
    sx2 = unsafe { game_state() }.r_main.viewangletox[angle2 as usize];
    if sx1 == sx2 {
        return false;
    }
    sx2 -= 1;
    start = &raw mut unsafe { game_state() }.r_bsp.solidsegs as *mut cliprange_t;
    while (*start).last < sx2 {
        start = start.offset(1);
    }
    if sx1 >= (*start).first && sx2 <= (*start).last {
        return false;
    }
    return true;
}
pub unsafe fn R_Subsector(mut num: i32) {
    let mut count: i32 = 0;
    let mut line: *mut seg_t = ::core::ptr::null_mut::<seg_t>();
    let mut sub: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    if num >= unsafe { game_state() }.p_setup.numsubsectors {
        I_Error(&format!(
            "R_Subsector: ss {} with numss = {}",
            num, unsafe { game_state() }.p_setup.numsubsectors
        ));
    }
    unsafe { game_state() }.r_main.sscount += 1;
    sub = unsafe { game_state() }
        .p_setup
        .subsector_mut(SubsectorId(num as u32));
    unsafe { game_state() }.r_bsp.frontsector = Some((*sub).sector);
    count = (*sub).numlines as i32;
    line = unsafe { game_state() }.p_setup.segs.offset((*sub).firstline as isize) as *mut seg_t;
    if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight < unsafe { game_state() }.r_main.viewz {
        unsafe { game_state() }.r_plane.floorplane = R_FindPlane(
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight,
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorpic as i32,
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32,
        );
    } else {
        unsafe { game_state() }.r_plane.floorplane = ::core::ptr::null_mut::<visplane_t>();
    }
    if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight > unsafe { game_state() }.r_main.viewz
        || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingpic as i32 == unsafe { game_state() }.r_sky.skyflatnum
    {
        unsafe { game_state() }.r_plane.ceilingplane = R_FindPlane(
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight,
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingpic as i32,
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32,
        );
    } else {
        unsafe { game_state() }.r_plane.ceilingplane = ::core::ptr::null_mut::<visplane_t>();
    }
    R_AddSprites(unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap()));
    loop {
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 != 0) {
            break;
        }
        R_AddLine(line);
        line = line.offset(1);
    }
}
pub unsafe fn R_RenderBSPNode(mut bspnum: i32) {
    let mut bsp: *mut node_t = ::core::ptr::null_mut::<node_t>();
    let mut side: i32 = 0;
    if bspnum & NF_SUBSECTOR != 0 {
        if bspnum == -(1 as i32) {
            R_Subsector(0 as i32);
        } else {
            R_Subsector(bspnum & !NF_SUBSECTOR);
        }
        return;
    }
    bsp = unsafe { game_state() }.p_setup.nodes.offset(bspnum as isize) as *mut node_t;
    side = R_PointOnSide(unsafe { game_state() }.r_main.viewx, unsafe { game_state() }.r_main.viewy, bsp);
    R_RenderBSPNode((*bsp).children[side as usize] as i32);
    if R_CheckBBox(
        &raw mut *(&raw mut (*bsp).bbox as *mut [fixed_t; 4]).offset((side ^ 1 as i32) as isize)
            as *mut fixed_t,
    ) {
        R_RenderBSPNode((*bsp).children[(side ^ 1 as i32) as usize] as i32);
    }
}
