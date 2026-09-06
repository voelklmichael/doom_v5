use crate::src::r_defs::{drawseg_s, drawseg_t, node_t, seg_t, side_t, visplane_t};
use crate::src::p_mobj::{sector_t, line_t, subsector_t};
use crate::src::i_system::I_Error;
use crate::src::r_main::clipangle;
use crate::src::r_main::viewangletox;
use crate::src::r_segs::rw_angle1;
use crate::src::r_main::sscount;
use crate::src::r_main::R_PointOnSide;
use crate::src::r_things::R_AddSprites;
use crate::src::r_segs::R_StoreWallRange;
use crate::src::p_setup::segs;
use crate::src::p_setup::numsubsectors;
use crate::src::r_plane::floorplane;
use crate::src::r_plane::ceilingplane;
use crate::src::p_setup::subsectors;
use crate::src::p_setup::nodes;
use crate::src::r_main::viewx;
use crate::src::r_main::viewy;
use crate::src::r_main::viewangle;
use crate::src::r_main::viewz;
use crate::src::r_draw::viewwidth;
use crate::src::r_sky::skyflatnum;
use crate::src::r_plane::R_FindPlane;
use crate::src::r_main::R_PointToAngle;
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG180;
use crate::src::tables::ANG90;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct cliprange_t {
    pub first: i32,
    pub last: i32,
}
pub const NF_SUBSECTOR: i32 = 0x8000 as i32;
pub static mut curline: *mut seg_t = ::core::ptr::null::<seg_t>() as *mut seg_t;
pub static mut sidedef: *mut side_t = ::core::ptr::null::<side_t>() as *mut side_t;
pub static mut linedef: *mut line_t = ::core::ptr::null::<line_t>() as *mut line_t;
pub static mut frontsector: *mut sector_t = ::core::ptr::null::<sector_t>()
    as *mut sector_t;
pub static mut backsector: *mut sector_t = ::core::ptr::null::<sector_t>()
    as *mut sector_t;
pub static mut drawsegs: [drawseg_t; 256] = [drawseg_s {
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
    sprbottomclip: ::core::ptr::null::<i16>()
        as *mut i16,
    maskedtexturecol: ::core::ptr::null::<i16>()
        as *mut i16,
}; 256];
pub static mut ds_p: *mut drawseg_t = ::core::ptr::null::<drawseg_t>() as *mut drawseg_t;
pub unsafe fn R_ClearDrawSegs() {
    ds_p = &raw mut drawsegs as *mut drawseg_t;
}
#[no_mangle]
pub static mut newend: *mut cliprange_t = ::core::ptr::null::<cliprange_t>()
    as *mut cliprange_t;
#[no_mangle]
pub static mut solidsegs: [cliprange_t; 32] = [cliprange_t { first: 0, last: 0 }; 32];
#[no_mangle]
pub unsafe extern "C" fn R_ClipSolidWallSegment(
    mut first: i32,
    mut last: i32,
) {
    let mut current_block: u64;
    let mut next: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    let mut start: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    start = &raw mut solidsegs as *mut cliprange_t;
    while (*start).last < first - 1 as i32 {
        start = start.offset(1);
    }
    if first < (*start).first {
        if last < (*start).first - 1 as i32 {
            R_StoreWallRange(first, last);
            next = newend;
            newend = newend.offset(1);
            while next != start {
                *next = *next.offset(-(1 as i32 as isize));
                next = next.offset(-1);
            }
            (*next).first = first;
            (*next).last = last;
            return;
        }
        R_StoreWallRange(first, (*start).first - 1 as i32);
        (*start).first = first;
    }
    if last <= (*start).last {
        return;
    }
    next = start;
    loop {
        if !(last
            >= (*next.offset(1 as i32 as isize)).first
                - 1 as i32)
        {
            current_block = 224731115979188411;
            break;
        }
        R_StoreWallRange(
            (*next).last + 1 as i32,
            (*next.offset(1 as i32 as isize)).first
                - 1 as i32,
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
            R_StoreWallRange((*next).last + 1 as i32, last);
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
        if !(fresh0 != newend) {
            break;
        }
        start = start.offset(1);
        *start = *next;
    }
    newend = start.offset(1 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn R_ClipPassWallSegment(
    mut first: i32,
    mut last: i32,
) {
    let mut start: *mut cliprange_t = ::core::ptr::null_mut::<cliprange_t>();
    start = &raw mut solidsegs as *mut cliprange_t;
    while (*start).last < first - 1 as i32 {
        start = start.offset(1);
    }
    if first < (*start).first {
        if last < (*start).first - 1 as i32 {
            R_StoreWallRange(first, last);
            return;
        }
        R_StoreWallRange(first, (*start).first - 1 as i32);
    }
    if last <= (*start).last {
        return;
    }
    while last
        >= (*start.offset(1 as i32 as isize)).first
            - 1 as i32
    {
        R_StoreWallRange(
            (*start).last + 1 as i32,
            (*start.offset(1 as i32 as isize)).first
                - 1 as i32,
        );
        start = start.offset(1);
        if last <= (*start).last {
            return;
        }
    }
    R_StoreWallRange((*start).last + 1 as i32, last);
}
pub unsafe fn R_ClearClipSegs() {
    solidsegs[0 as i32 as usize].first = -(0x7fffffff
        as i32);
    solidsegs[0 as i32 as usize].last = -(1 as i32);
    solidsegs[1 as i32 as usize].first = viewwidth;
    solidsegs[1 as i32 as usize].last = 0x7fffffff as i32;
    newend = (&raw mut solidsegs as *mut cliprange_t)
        .offset(2 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn R_AddLine(mut line: *mut seg_t) {
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut angle1: angle_t = 0;
    let mut angle2: angle_t = 0;
    let mut span: angle_t = 0;
    let mut tspan: angle_t = 0;
    curline = line;
    angle1 = R_PointToAngle((*(*line).v1).x, (*(*line).v1).y);
    angle2 = R_PointToAngle((*(*line).v2).x, (*(*line).v2).y);
    span = angle1.wrapping_sub(angle2);
    if span >= ANG180 {
        return;
    }
    rw_angle1 = angle1 as i32;
    angle1 = angle1.wrapping_sub(viewangle);
    angle2 = angle2.wrapping_sub(viewangle);
    tspan = angle1.wrapping_add(clipangle);
    if tspan > (2 as angle_t).wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(clipangle));
        if tspan >= span {
            return;
        }
        angle1 = clipangle;
    }
    tspan = clipangle.wrapping_sub(angle2);
    if tspan > (2 as angle_t).wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(clipangle));
        if tspan >= span {
            return;
        }
        angle2 = clipangle.wrapping_neg();
    }
    angle1 = angle1.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    angle2 = angle2.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    x1 = viewangletox[angle1 as usize];
    x2 = viewangletox[angle2 as usize];
    if x1 == x2 {
        return;
    }
    backsector = (*line).backsector;
    if !backsector.is_null() {
        if !((*backsector).ceilingheight <= (*frontsector).floorheight
            || (*backsector).floorheight >= (*frontsector).ceilingheight)
        {
            if !((*backsector).ceilingheight != (*frontsector).ceilingheight
                || (*backsector).floorheight != (*frontsector).floorheight)
            {
                if (*backsector).ceilingpic as i32
                    == (*frontsector).ceilingpic as i32
                    && (*backsector).floorpic as i32
                        == (*frontsector).floorpic as i32
                    && (*backsector).lightlevel as i32
                        == (*frontsector).lightlevel as i32
                    && (*(*curline).sidedef).midtexture as i32
                        == 0 as i32
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
pub static mut checkcoord: [[i32; 4]; 12] = [
    [
        3 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
    ],
    [
        3 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
    ],
    [
        3 as i32,
        1 as i32,
        2 as i32,
        0 as i32,
    ],
    [0 as i32; 4],
    [
        2 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
    ],
    [
        3 as i32,
        1 as i32,
        3 as i32,
        0 as i32,
    ],
    [0 as i32; 4],
    [
        2 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
    ],
    [
        2 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
    ],
    [
        2 as i32,
        1 as i32,
        3 as i32,
        0 as i32,
    ],
    [0; 4],
];
#[no_mangle]
pub unsafe extern "C" fn R_CheckBBox(mut bspcoord: *mut fixed_t) -> bool {
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
    if viewx <= *bspcoord.offset(BOXLEFT as i32 as isize) {
        boxx = 0 as i32;
    } else if viewx < *bspcoord.offset(BOXRIGHT as i32 as isize) {
        boxx = 1 as i32;
    } else {
        boxx = 2 as i32;
    }
    if viewy >= *bspcoord.offset(BOXTOP as i32 as isize) {
        boxy = 0 as i32;
    } else if viewy > *bspcoord.offset(BOXBOTTOM as i32 as isize) {
        boxy = 1 as i32;
    } else {
        boxy = 2 as i32;
    }
    boxpos = (boxy << 2 as i32) + boxx;
    if boxpos == 5 as i32 {
        return true;
    }
    x1 = *bspcoord
        .offset(checkcoord[boxpos as usize][0 as i32 as usize] as isize);
    y1 = *bspcoord
        .offset(checkcoord[boxpos as usize][1 as i32 as usize] as isize);
    x2 = *bspcoord
        .offset(checkcoord[boxpos as usize][2 as i32 as usize] as isize);
    y2 = *bspcoord
        .offset(checkcoord[boxpos as usize][3 as i32 as usize] as isize);
    angle1 = R_PointToAngle(x1, y1).wrapping_sub(viewangle);
    angle2 = R_PointToAngle(x2, y2).wrapping_sub(viewangle);
    span = angle1.wrapping_sub(angle2);
    if span >= ANG180 {
        return true;
    }
    tspan = angle1.wrapping_add(clipangle);
    if tspan > (2 as angle_t).wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(clipangle));
        if tspan >= span {
            return false;
        }
        angle1 = clipangle;
    }
    tspan = clipangle.wrapping_sub(angle2);
    if tspan > (2 as angle_t).wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub((2 as angle_t).wrapping_mul(clipangle));
        if tspan >= span {
            return false;
        }
        angle2 = clipangle.wrapping_neg();
    }
    angle1 = angle1.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    angle2 = angle2.wrapping_add(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    sx1 = viewangletox[angle1 as usize];
    sx2 = viewangletox[angle2 as usize];
    if sx1 == sx2 {
        return false;
    }
    sx2 -= 1;
    start = &raw mut solidsegs as *mut cliprange_t;
    while (*start).last < sx2 {
        start = start.offset(1);
    }
    if sx1 >= (*start).first && sx2 <= (*start).last {
        return false;
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn R_Subsector(mut num: i32) {
    let mut count: i32 = 0;
    let mut line: *mut seg_t = ::core::ptr::null_mut::<seg_t>();
    let mut sub: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    if num >= numsubsectors {
        I_Error(&format!("R_Subsector: ss {} with numss = {}", num, numsubsectors));
    }
    sscount += 1;
    sub = subsectors.offset(num as isize) as *mut subsector_t;
    frontsector = (*sub).sector;
    count = (*sub).numlines as i32;
    line = segs.offset((*sub).firstline as isize) as *mut seg_t;
    if (*frontsector).floorheight < viewz {
        floorplane = R_FindPlane(
            (*frontsector).floorheight,
            (*frontsector).floorpic as i32,
            (*frontsector).lightlevel as i32,
        );
    } else {
        floorplane = ::core::ptr::null_mut::<visplane_t>();
    }
    if (*frontsector).ceilingheight > viewz
        || (*frontsector).ceilingpic as i32 == skyflatnum
    {
        ceilingplane = R_FindPlane(
            (*frontsector).ceilingheight,
            (*frontsector).ceilingpic as i32,
            (*frontsector).lightlevel as i32,
        );
    } else {
        ceilingplane = ::core::ptr::null_mut::<visplane_t>();
    }
    R_AddSprites(frontsector);
    loop {
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 != 0) {
            break;
        }
        R_AddLine(line);
        line = line.offset(1);
    };
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
    bsp = nodes.offset(bspnum as isize) as *mut node_t;
    side = R_PointOnSide(viewx, viewy, bsp);
    R_RenderBSPNode((*bsp).children[side as usize] as i32);
    if R_CheckBBox(
        &raw mut *(&raw mut (*bsp).bbox as *mut [fixed_t; 4])
            .offset((side ^ 1 as i32) as isize) as *mut fixed_t,
    )
    {
        R_RenderBSPNode(
            (*bsp).children[(side ^ 1 as i32) as usize]
                as i32,
        );
    }
}
