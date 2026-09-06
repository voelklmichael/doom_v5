use crate::src::p_mobj::{mapthing_t, subsector_s, sector_t, line_t, subsector_t};
use crate::src::p_mobj::{mobj_s, mobj_t};
use crate::src::p_setup::blockmaplump;
use crate::src::p_setup::blockmap;
use crate::src::p_setup::bmapwidth;
use crate::src::p_setup::bmapheight;
use crate::src::p_setup::blocklinks;
use crate::src::p_pspr::bulletslope;
use crate::src::p_setup::playerstarts;
use crate::src::p_setup::bmaporgx;
use crate::src::p_setup::bmaporgy;
use crate::src::r_main::R_PointInSubsector;
use crate::src::p_setup::lines;
use crate::src::r_main::validcount;
use crate::src::m_fixed::FixedDiv;
use crate::src::m_fixed::FixedMul;
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::src::p_mobj::{MF_NOBLOCKMAP, MF_NOSECTOR};
use crate::src::m_fixed::fixed_t;
use crate::src::doomdef::boolean;
use crate::src::doomdef::NULL;
use crate::src::doomdef::true_0;
use crate::src::doomdef::false_0;
use crate::src::m_fixed::FRACUNIT;
use crate::src::m_fixed::INT_MAX;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct divline_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intercept_t {
    pub frac: fixed_t,
    pub isaline: bool,
    pub d: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub thing: *mut mobj_t,
    pub line: *mut line_t,
}
pub type traverser_t = Option<unsafe extern "C" fn(*mut intercept_t) -> boolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intercepts_overrun_t {
    pub len: i32,
    pub addr: *mut ::core::ffi::c_void,
    pub int16_array: bool,
}
pub const FRACBITS: i32 = 16 as i32;
pub const MAPBLOCKUNITS: i32 = 128 as i32;
pub const MAPBLOCKSIZE: i32 = MAPBLOCKUNITS * FRACUNIT;
pub const MAPBLOCKSHIFT: i32 = FRACBITS + 7 as i32;
pub const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;
pub const MAXINTERCEPTS_ORIGINAL: i32 = 128 as i32;
pub const PT_ADDLINES: i32 = 1 as i32;
pub const PT_ADDTHINGS: i32 = 2 as i32;
pub const PT_EARLYOUT: i32 = 4 as i32;
pub unsafe fn P_AproxDistance(mut dx: fixed_t, mut dy: fixed_t) -> fixed_t {
    dx = (dx as i32).abs() as fixed_t;
    dy = (dy as i32).abs() as fixed_t;
    if dx < dy {
        return dx + dy - (dx >> 1 as i32);
    }
    return dx + dy - (dy >> 1 as i32);
}
pub unsafe fn P_PointOnLineSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut line: *mut line_t,
) -> i32 {
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut left: fixed_t = 0;
    let mut right: fixed_t = 0;
    if (*line).dx == 0 {
        if x <= (*(*line).v1).x {
            return ((*line).dy > 0 as i32) as i32;
        }
        return ((*line).dy < 0 as i32) as i32;
    }
    if (*line).dy == 0 {
        if y <= (*(*line).v1).y {
            return ((*line).dx < 0 as i32) as i32;
        }
        return ((*line).dx > 0 as i32) as i32;
    }
    dx = x - (*(*line).v1).x;
    dy = y - (*(*line).v1).y;
    left = FixedMul((*line).dy >> FRACBITS, dx);
    right = FixedMul(dy, (*line).dx >> FRACBITS);
    if right < left {
        return 0 as i32;
    }
    return 1 as i32;
}
pub unsafe fn P_BoxOnLineSide(
    mut tmbox: *mut fixed_t,
    mut ld: *mut line_t,
) -> i32 {
    let mut p1: i32 = 0 as i32;
    let mut p2: i32 = 0 as i32;
    match (*ld).slopetype as u32 {
        0 => {
            p1 = (*tmbox.offset(BOXTOP as i32 as isize) > (*(*ld).v1).y)
                as i32;
            p2 = (*tmbox.offset(BOXBOTTOM as i32 as isize)
                > (*(*ld).v1).y) as i32;
            if (*ld).dx < 0 as i32 {
                p1 ^= 1 as i32;
                p2 ^= 1 as i32;
            }
        }
        1 => {
            p1 = (*tmbox.offset(BOXRIGHT as i32 as isize) < (*(*ld).v1).x)
                as i32;
            p2 = (*tmbox.offset(BOXLEFT as i32 as isize) < (*(*ld).v1).x)
                as i32;
            if (*ld).dy < 0 as i32 {
                p1 ^= 1 as i32;
                p2 ^= 1 as i32;
            }
        }
        2 => {
            p1 = P_PointOnLineSide(
                *tmbox.offset(BOXLEFT as i32 as isize),
                *tmbox.offset(BOXTOP as i32 as isize),
                ld,
            );
            p2 = P_PointOnLineSide(
                *tmbox.offset(BOXRIGHT as i32 as isize),
                *tmbox.offset(BOXBOTTOM as i32 as isize),
                ld,
            );
        }
        3 => {
            p1 = P_PointOnLineSide(
                *tmbox.offset(BOXRIGHT as i32 as isize),
                *tmbox.offset(BOXTOP as i32 as isize),
                ld,
            );
            p2 = P_PointOnLineSide(
                *tmbox.offset(BOXLEFT as i32 as isize),
                *tmbox.offset(BOXBOTTOM as i32 as isize),
                ld,
            );
        }
        _ => {}
    }
    if p1 == p2 {
        return p1;
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn P_PointOnDivlineSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut line: *mut divline_t,
) -> i32 {
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut left: fixed_t = 0;
    let mut right: fixed_t = 0;
    if (*line).dx == 0 {
        if x <= (*line).x {
            return ((*line).dy > 0 as i32) as i32;
        }
        return ((*line).dy < 0 as i32) as i32;
    }
    if (*line).dy == 0 {
        if y <= (*line).y {
            return ((*line).dx < 0 as i32) as i32;
        }
        return ((*line).dx > 0 as i32) as i32;
    }
    dx = x - (*line).x;
    dy = y - (*line).y;
    if ((*line).dy ^ (*line).dx ^ dx ^ dy) as u32
        & 0x80000000 as u32 != 0
    {
        if ((*line).dy ^ dx) as u32 & 0x80000000 as u32
            != 0
        {
            return 1 as i32;
        }
        return 0 as i32;
    }
    left = FixedMul(
        (*line).dy >> 8 as i32,
        dx >> 8 as i32,
    );
    right = FixedMul(
        dy >> 8 as i32,
        (*line).dx >> 8 as i32,
    );
    if right < left {
        return 0 as i32;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn P_MakeDivline(mut li: *mut line_t, mut dl: *mut divline_t) {
    (*dl).x = (*(*li).v1).x;
    (*dl).y = (*(*li).v1).y;
    (*dl).dx = (*li).dx;
    (*dl).dy = (*li).dy;
}
#[no_mangle]
pub unsafe extern "C" fn P_InterceptVector(
    mut v2: *mut divline_t,
    mut v1: *mut divline_t,
) -> fixed_t {
    let mut frac: fixed_t = 0;
    let mut num: fixed_t = 0;
    let mut den: fixed_t = 0;
    den = FixedMul((*v1).dy >> 8 as i32, (*v2).dx)
        - FixedMul((*v1).dx >> 8 as i32, (*v2).dy);
    if den == 0 as i32 {
        return 0 as fixed_t;
    }
    num = FixedMul((*v1).x - (*v2).x >> 8 as i32, (*v1).dy)
        + FixedMul((*v2).y - (*v1).y >> 8 as i32, (*v1).dx);
    frac = FixedDiv(num, den);
    return frac;
}
pub static mut opentop: fixed_t = 0;
pub static mut openbottom: fixed_t = 0;
pub static mut openrange: fixed_t = 0;
pub static mut lowfloor: fixed_t = 0;
pub unsafe fn P_LineOpening(mut linedef: *mut line_t) {
    let mut front: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut back: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    if (*linedef).sidenum[1 as i32 as usize] as i32
        == -(1 as i32)
    {
        openrange = 0 as i32 as fixed_t;
        return;
    }
    front = (*linedef).frontsector;
    back = (*linedef).backsector;
    if (*front).ceilingheight < (*back).ceilingheight {
        opentop = (*front).ceilingheight;
    } else {
        opentop = (*back).ceilingheight;
    }
    if (*front).floorheight > (*back).floorheight {
        openbottom = (*front).floorheight;
        lowfloor = (*back).floorheight;
    } else {
        openbottom = (*back).floorheight;
        lowfloor = (*front).floorheight;
    }
    openrange = opentop - openbottom;
}
pub unsafe fn P_UnsetThingPosition(mut thing: *mut mobj_t) {
    let mut blockx: i32 = 0;
    let mut blocky: i32 = 0;
    if (*thing).flags & MF_NOSECTOR as i32 == 0 {
        if !(*thing).snext.is_null() {
            (*(*thing).snext).sprev = (*thing).sprev;
        }
        if !(*thing).sprev.is_null() {
            (*(*thing).sprev).snext = (*thing).snext;
        } else {
            (*(*(*thing).subsector).sector).thinglist = (*thing).snext as *mut mobj_t;
        }
    }
    if (*thing).flags & MF_NOBLOCKMAP as i32 == 0 {
        if !(*thing).bnext.is_null() {
            (*(*thing).bnext).bprev = (*thing).bprev;
        }
        if !(*thing).bprev.is_null() {
            (*(*thing).bprev).bnext = (*thing).bnext;
        } else {
            blockx = ((*thing).x - bmaporgx >> MAPBLOCKSHIFT) as i32;
            blocky = ((*thing).y - bmaporgy >> MAPBLOCKSHIFT) as i32;
            if blockx >= 0 as i32 && blockx < bmapwidth
                && blocky >= 0 as i32 && blocky < bmapheight
            {
                let ref mut fresh1 = *blocklinks
                    .offset((blocky * bmapwidth + blockx) as isize);
                *fresh1 = (*thing).bnext as *mut mobj_t;
            }
        }
    }
}
pub unsafe fn P_SetThingPosition(mut thing: *mut mobj_t) {
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut blockx: i32 = 0;
    let mut blocky: i32 = 0;
    let mut link: *mut *mut mobj_t = ::core::ptr::null_mut::<*mut mobj_t>();
    ss = R_PointInSubsector((*thing).x, (*thing).y);
    (*thing).subsector = ss as *mut subsector_s;
    if (*thing).flags & MF_NOSECTOR as i32 == 0 {
        sec = (*ss).sector;
        (*thing).sprev = ::core::ptr::null_mut::<mobj_s>();
        (*thing).snext = (*sec).thinglist as *mut mobj_s;
        if !(*sec).thinglist.is_null() {
            (*(*sec).thinglist).sprev = thing as *mut mobj_s;
        }
        (*sec).thinglist = thing;
    }
    if (*thing).flags & MF_NOBLOCKMAP as i32 == 0 {
        blockx = ((*thing).x - bmaporgx >> MAPBLOCKSHIFT) as i32;
        blocky = ((*thing).y - bmaporgy >> MAPBLOCKSHIFT) as i32;
        if blockx >= 0 as i32 && blockx < bmapwidth
            && blocky >= 0 as i32 && blocky < bmapheight
        {
            link = blocklinks.offset((blocky * bmapwidth + blockx) as isize)
                as *mut *mut mobj_t;
            (*thing).bprev = ::core::ptr::null_mut::<mobj_s>();
            (*thing).bnext = *link as *mut mobj_s;
            if !(*link).is_null() {
                (**link).bprev = thing as *mut mobj_s;
            }
            *link = thing;
        } else {
            (*thing).bprev = ::core::ptr::null_mut::<mobj_s>();
            (*thing).bnext = (*thing).bprev;
        }
    }
}
pub unsafe fn P_BlockLinesIterator(
    mut x: i32,
    mut y: i32,
    mut func: Option<unsafe extern "C" fn(*mut line_t) -> boolean>,
) -> bool {
    let mut offset: i32 = 0;
    let mut list: *mut i16 = ::core::ptr::null_mut::<
        i16,
    >();
    let mut ld: *mut line_t = ::core::ptr::null_mut::<line_t>();
    if x < 0 as i32 || y < 0 as i32 || x >= bmapwidth
        || y >= bmapheight
    {
        return true;
    }
    offset = y * bmapwidth + x;
    offset = *blockmap.offset(offset as isize) as i32;
    list = blockmaplump.offset(offset as isize);
    while *list as i32 != -(1 as i32) {
        ld = lines.offset(*list as isize) as *mut line_t;
        if !((*ld).validcount == validcount) {
            (*ld).validcount = validcount;
            if func.expect("non-null function pointer")(ld) == 0 {
                return false;
            }
        }
        list = list.offset(1);
    }
    return true;
}
pub unsafe fn P_BlockThingsIterator(
    mut x: i32,
    mut y: i32,
    mut func: Option<unsafe extern "C" fn(*mut mobj_t) -> boolean>,
) -> bool {
    let mut mobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    if x < 0 as i32 || y < 0 as i32 || x >= bmapwidth
        || y >= bmapheight
    {
        return true;
    }
    mobj = *blocklinks.offset((y * bmapwidth + x) as isize);
    while !mobj.is_null() {
        if func.expect("non-null function pointer")(mobj) == 0 {
            return false;
        }
        mobj = (*mobj).bnext as *mut mobj_t;
    }
    return true;
}
#[no_mangle]
pub static mut intercepts: [intercept_t; 189] = [intercept_t {
    frac: 0,
    isaline: false,
    d: C2RustUnnamed_1 {
        thing: ::core::ptr::null::<mobj_t>() as *mut mobj_t,
    },
}; 189];
#[no_mangle]
pub static mut intercept_p: *mut intercept_t = ::core::ptr::null::<intercept_t>()
    as *mut intercept_t;
pub static mut trace: divline_t = divline_t {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};
#[no_mangle]
pub static mut earlyout: bool = false;
#[no_mangle]
pub static mut ptflags: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn PIT_AddLineIntercepts(mut ld: *mut line_t) -> boolean {
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut frac: fixed_t = 0;
    let mut dl: divline_t = divline_t {
        x: 0,
        y: 0,
        dx: 0,
        dy: 0,
    };
    if trace.dx > FRACUNIT * 16 as i32
        || trace.dy > FRACUNIT * 16 as i32
        || trace.dx < -FRACUNIT * 16 as i32
        || trace.dy < -FRACUNIT * 16 as i32
    {
        s1 = P_PointOnDivlineSide((*(*ld).v1).x, (*(*ld).v1).y, &raw mut trace);
        s2 = P_PointOnDivlineSide((*(*ld).v2).x, (*(*ld).v2).y, &raw mut trace);
    } else {
        s1 = P_PointOnLineSide(trace.x, trace.y, ld);
        s2 = P_PointOnLineSide(trace.x + trace.dx, trace.y + trace.dy, ld);
    }
    if s1 == s2 {
        return true_0 as boolean;
    }
    P_MakeDivline(ld, &raw mut dl);
    frac = P_InterceptVector(&raw mut trace, &raw mut dl);
    if frac < 0 as i32 {
        return true_0 as boolean;
    }
    if earlyout && frac < FRACUNIT && (*ld).backsector.is_null() {
        return false_0 as boolean;
    }
    (*intercept_p).frac = frac;
    (*intercept_p).isaline = true;
    (*intercept_p).d.line = ld;
    InterceptsOverrun(
        intercept_p.offset_from(&raw mut intercepts as *mut intercept_t)
            as i64 as i32,
        intercept_p,
    );
    intercept_p = intercept_p.offset(1);
    return true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn PIT_AddThingIntercepts(mut thing: *mut mobj_t) -> boolean {
    let mut x1: fixed_t = 0;
    let mut y1: fixed_t = 0;
    let mut x2: fixed_t = 0;
    let mut y2: fixed_t = 0;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut tracepositive: bool = false;
    let mut dl: divline_t = divline_t {
        x: 0,
        y: 0,
        dx: 0,
        dy: 0,
    };
    let mut frac: fixed_t = 0;
    tracepositive = trace.dx ^ trace.dy > 0 as i32;
    if tracepositive {
        x1 = (*thing).x - (*thing).radius;
        y1 = (*thing).y + (*thing).radius;
        x2 = (*thing).x + (*thing).radius;
        y2 = (*thing).y - (*thing).radius;
    } else {
        x1 = (*thing).x - (*thing).radius;
        y1 = (*thing).y - (*thing).radius;
        x2 = (*thing).x + (*thing).radius;
        y2 = (*thing).y + (*thing).radius;
    }
    s1 = P_PointOnDivlineSide(x1, y1, &raw mut trace);
    s2 = P_PointOnDivlineSide(x2, y2, &raw mut trace);
    if s1 == s2 {
        return true_0 as boolean;
    }
    dl.x = x1;
    dl.y = y1;
    dl.dx = x2 - x1;
    dl.dy = y2 - y1;
    frac = P_InterceptVector(&raw mut trace, &raw mut dl);
    if frac < 0 as i32 {
        return true_0 as boolean;
    }
    (*intercept_p).frac = frac;
    (*intercept_p).isaline = false;
    (*intercept_p).d.thing = thing;
    InterceptsOverrun(
        intercept_p.offset_from(&raw mut intercepts as *mut intercept_t)
            as i64 as i32,
        intercept_p,
    );
    intercept_p = intercept_p.offset(1);
    return true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn P_TraverseIntercepts(
    mut func: traverser_t,
    mut maxfrac: fixed_t,
) -> bool {
    let mut count: i32 = 0;
    let mut dist: fixed_t = 0;
    let mut scan: *mut intercept_t = ::core::ptr::null_mut::<intercept_t>();
    let mut in_0: *mut intercept_t = ::core::ptr::null_mut::<intercept_t>();
    count = intercept_p.offset_from(&raw mut intercepts as *mut intercept_t)
        as i64 as i32;
    in_0 = ::core::ptr::null_mut::<intercept_t>();
    loop {
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 != 0) {
            break;
        }
        dist = INT_MAX as fixed_t;
        scan = &raw mut intercepts as *mut intercept_t;
        while scan < intercept_p {
            if (*scan).frac < dist {
                dist = (*scan).frac;
                in_0 = scan;
            }
            scan = scan.offset(1);
        }
        if dist > maxfrac {
            return true;
        }
        if func.expect("non-null function pointer")(in_0) == 0 {
            return false;
        }
        (*in_0).frac = INT_MAX as fixed_t;
    }
    return true;
}
static mut intercepts_overrun: [intercepts_overrun_t; 23] = unsafe {
    [
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const lowfloor as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const openbottom as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const opentop as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const openrange as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 120 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 8 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const bulletslope as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 40 as i32,
            addr: &raw const playerstarts as *mut [mapthing_t; 4]
                as *mut ::core::ffi::c_void,
            int16_array: true,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const bmapwidth as *mut i32
                as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const bmaporgx as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const bmaporgy as *mut fixed_t as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: NULL,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 4 as i32,
            addr: &raw const bmapheight as *mut i32
                as *mut ::core::ffi::c_void,
            int16_array: false,
        },
        intercepts_overrun_t {
            len: 0 as i32,
            addr: NULL,
            int16_array: false,
        },
    ]
};
unsafe extern "C" fn InterceptsMemoryOverrun(
    mut location: i32,
    mut value: i32,
) {
    let mut i: i32 = 0;
    let mut offset: i32 = 0;
    let mut index: i32 = 0;
    let mut addr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    i = 0 as i32;
    offset = 0 as i32;
    while intercepts_overrun[i as usize].len != 0 as i32 {
        if offset + intercepts_overrun[i as usize].len > location {
            addr = intercepts_overrun[i as usize].addr;
            if !addr.is_null() {
                if intercepts_overrun[i as usize].int16_array {
                    index = (location - offset) / 2 as i32;
                    *(addr as *mut i16).offset(index as isize) = (value
                        & 0xffff as i32) as i16;
                    *(addr as *mut i16)
                        .offset((index + 1 as i32) as isize) = (value
                        >> 16 as i32 & 0xffff as i32)
                        as i16;
                } else {
                    index = (location - offset) / 4 as i32;
                    *(addr as *mut i32).offset(index as isize) = value;
                }
            }
            break;
        } else {
            offset += intercepts_overrun[i as usize].len;
            i += 1;
        }
    }
}
unsafe extern "C" fn InterceptsOverrun(
    mut num_intercepts: i32,
    mut intercept: *mut intercept_t,
) {
    let mut location: i32 = 0;
    if num_intercepts <= MAXINTERCEPTS_ORIGINAL {
        return;
    }
    location = (num_intercepts - MAXINTERCEPTS_ORIGINAL - 1 as i32)
        * 12 as i32;
    InterceptsMemoryOverrun(location, (*intercept).frac as i32);
    InterceptsMemoryOverrun(
        location + 4 as i32,
        (*intercept).isaline as i32,
    );
    InterceptsMemoryOverrun(
        location + 8 as i32,
        (*intercept).d.thing as i32,
    );
}
pub unsafe fn P_PathTraverse(
    mut x1: fixed_t,
    mut y1: fixed_t,
    mut x2: fixed_t,
    mut y2: fixed_t,
    mut flags: i32,
    mut trav: Option<unsafe extern "C" fn(*mut intercept_t) -> boolean>,
) -> bool {
    let mut xt1: fixed_t = 0;
    let mut yt1: fixed_t = 0;
    let mut xt2: fixed_t = 0;
    let mut yt2: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut partial: fixed_t = 0;
    let mut xintercept: fixed_t = 0;
    let mut yintercept: fixed_t = 0;
    let mut mapx: i32 = 0;
    let mut mapy: i32 = 0;
    let mut mapxstep: i32 = 0;
    let mut mapystep: i32 = 0;
    let mut count: i32 = 0;
    earlyout = (flags & PT_EARLYOUT) != 0;
    validcount += 1;
    intercept_p = &raw mut intercepts as *mut intercept_t;
    if x1 as i32 - bmaporgx as i32
        & MAPBLOCKSIZE - 1 as i32 == 0 as i32
    {
        x1 += FRACUNIT;
    }
    if y1 as i32 - bmaporgy as i32
        & MAPBLOCKSIZE - 1 as i32 == 0 as i32
    {
        y1 += FRACUNIT;
    }
    trace.x = x1;
    trace.y = y1;
    trace.dx = x2 - x1;
    trace.dy = y2 - y1;
    x1 -= bmaporgx;
    y1 -= bmaporgy;
    xt1 = x1 >> MAPBLOCKSHIFT;
    yt1 = y1 >> MAPBLOCKSHIFT;
    x2 -= bmaporgx;
    y2 -= bmaporgy;
    xt2 = x2 >> MAPBLOCKSHIFT;
    yt2 = y2 >> MAPBLOCKSHIFT;
    if xt2 > xt1 {
        mapxstep = 1 as i32;
        partial = (FRACUNIT
            - (x1 as i32 >> MAPBTOFRAC
                & FRACUNIT - 1 as i32)) as fixed_t;
        ystep = FixedDiv(
            y2 - y1,
            (x2 as i32 - x1 as i32).abs() as fixed_t,
        );
    } else if xt2 < xt1 {
        mapxstep = -(1 as i32);
        partial = (x1 as i32 >> MAPBTOFRAC
            & FRACUNIT - 1 as i32) as fixed_t;
        ystep = FixedDiv(
            y2 - y1,
            (x2 as i32 - x1 as i32).abs() as fixed_t,
        );
    } else {
        mapxstep = 0 as i32;
        partial = FRACUNIT as fixed_t;
        ystep = (256 as i32 * FRACUNIT) as fixed_t;
    }
    yintercept = (y1 >> MAPBTOFRAC) + FixedMul(partial, ystep);
    if yt2 > yt1 {
        mapystep = 1 as i32;
        partial = (FRACUNIT
            - (y1 as i32 >> MAPBTOFRAC
                & FRACUNIT - 1 as i32)) as fixed_t;
        xstep = FixedDiv(
            x2 - x1,
            (y2 as i32 - y1 as i32).abs() as fixed_t,
        );
    } else if yt2 < yt1 {
        mapystep = -(1 as i32);
        partial = (y1 as i32 >> MAPBTOFRAC
            & FRACUNIT - 1 as i32) as fixed_t;
        xstep = FixedDiv(
            x2 - x1,
            (y2 as i32 - y1 as i32).abs() as fixed_t,
        );
    } else {
        mapystep = 0 as i32;
        partial = FRACUNIT as fixed_t;
        xstep = (256 as i32 * FRACUNIT) as fixed_t;
    }
    xintercept = (x1 >> MAPBTOFRAC) + FixedMul(partial, xstep);
    mapx = xt1 as i32;
    mapy = yt1 as i32;
    count = 0 as i32;
    while count < 64 as i32 {
        if flags & PT_ADDLINES != 0 {
            if !P_BlockLinesIterator(
                mapx,
                mapy,
                Some(
                    PIT_AddLineIntercepts as unsafe extern "C" fn(*mut line_t) -> boolean,
                ),
            )
            {
                return false;
            }
        }
        if flags & PT_ADDTHINGS != 0 {
            if !P_BlockThingsIterator(
                mapx,
                mapy,
                Some(
                    PIT_AddThingIntercepts
                        as unsafe extern "C" fn(*mut mobj_t) -> boolean,
                ),
            )
            {
                return false;
            }
        }
        if mapx == xt2 && mapy == yt2 {
            break;
        }
        if yintercept >> FRACBITS == mapy {
            yintercept += ystep;
            mapx += mapxstep;
        } else if xintercept >> FRACBITS == mapx {
            xintercept += xstep;
            mapy += mapystep;
        }
        count += 1;
    }
    return P_TraverseIntercepts(trav as traverser_t, FRACUNIT);
}
