use crate::src::doomdef::SCREENWIDTH;
use crate::src::game_state::game_state;
use crate::src::game_state::GameState;
use crate::src::i_system::I_Error;
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FixedDiv;
use crate::src::m_fixed::FixedMul;
use crate::src::r_data::R_GetColumn;
use crate::src::r_defs::lighttable_t;
use crate::src::r_defs::{drawseg_t, visplane_t};
use crate::src::r_main::LIGHTLEVELS;
use crate::src::r_main::LIGHTSEGSHIFT;
use crate::src::r_main::LIGHTZSHIFT;
use crate::src::r_main::MAXLIGHTZ;
use crate::src::r_segs::MAXDRAWSEGS;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::tables::angle_t;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::tables::ANG90;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::w_wad::W_ReleaseLumpNum;
use crate::src::z_zone::PU_STATIC;
use libc::memset;

pub struct RPlaneState {
    pub floorfunc: planefunction_t,
    pub ceilingfunc: planefunction_t,
    pub visplanes: [visplane_t; 128],
    pub lastvisplane: *mut visplane_t,
    pub floorplane: *mut visplane_t,
    pub ceilingplane: *mut visplane_t,
    pub openings: [i16; 20480],
    pub lastopening: *mut i16,
    pub floorclip: [i16; 320],
    pub ceilingclip: [i16; 320],
    pub spanstart: [i32; 200],
    pub spanstop: [i32; 200],
    pub planezlight: *mut *mut lighttable_t,
    pub planeheight: fixed_t,
    pub yslope: [fixed_t; 200],
    pub distscale: [fixed_t; 320],
    pub basexscale: fixed_t,
    pub baseyscale: fixed_t,
    pub cachedheight: [fixed_t; 200],
    pub cacheddistance: [fixed_t; 200],
    pub cachedxstep: [fixed_t; 200],
    pub cachedystep: [fixed_t; 200],
}

impl RPlaneState {
    pub const fn new() -> Self {
        RPlaneState {
            floorfunc: None,
            ceilingfunc: None,
            visplanes: [visplane_t {
        height: 0,
        picnum: 0,
        lightlevel: 0,
        minx: 0,
        maxx: 0,
        pad1: 0,
        top: [0; 320],
        pad2: 0,
        pad3: 0,
        bottom: [0; 320],
        pad4: 0,
    }; 128],
            lastvisplane: ::core::ptr::null::<visplane_t>() as *mut visplane_t,
            floorplane: ::core::ptr::null::<visplane_t>() as *mut visplane_t,
            ceilingplane: ::core::ptr::null::<visplane_t>() as *mut visplane_t,
            openings: [0; 20480],
            lastopening: ::core::ptr::null::<i16>() as *mut i16,
            floorclip: [0; 320],
            ceilingclip: [0; 320],
            spanstart: [0; 200],
            spanstop: [0; 200],
            planezlight: 
        ::core::ptr::null::<*mut lighttable_t>() as *mut *mut lighttable_t,
            planeheight: 0,
            yslope: [0; 200],
            distscale: [0; 320],
            basexscale: 0,
            baseyscale: 0,
            cachedheight: [0; 200],
            cacheddistance: [0; 200],
            cachedxstep: [0; 200],
            cachedystep: [0; 200],
        }
    }
}


pub type planefunction_t = Option<unsafe extern "C" fn(i32, i32) -> ()>;
pub const ANGLETOSKYSHIFT: i32 = 22;
pub const MAXVISPLANES: i32 = 128;
pub unsafe fn R_MapPlane(mut y: i32, mut x1: i32, mut x2: i32) {
    let mut angle: angle_t = 0;
    let mut distance: fixed_t = 0;
    let mut length: fixed_t = 0;
    let mut index: u32 = 0;
    if x2 < x1 || x1 < 0 as i32 || x2 >= unsafe { game_state() }.r_draw.viewwidth || y > unsafe { game_state() }.r_draw.viewheight {
        I_Error(&format!("R_MapPlane: {}, {} at {}", x1, x2, y));
    }
    if unsafe { game_state() }.r_plane.planeheight != unsafe { game_state() }.r_plane.cachedheight[y as usize] {
        unsafe { game_state() }.r_plane.cachedheight[y as usize] = unsafe { game_state() }.r_plane.planeheight;
        unsafe { game_state() }.r_plane.cacheddistance[y as usize] = FixedMul(unsafe { game_state() }.r_plane.planeheight, unsafe { game_state() }.r_plane.yslope[y as usize]);
        distance = unsafe { game_state() }.r_plane.cacheddistance[y as usize];
        unsafe { game_state() }.r_plane.cachedxstep[y as usize] = FixedMul(distance, unsafe { game_state() }.r_plane.basexscale);
        unsafe { game_state() }.r_draw.ds_xstep = unsafe { game_state() }.r_plane.cachedxstep[y as usize];
        unsafe { game_state() }.r_plane.cachedystep[y as usize] = FixedMul(distance, unsafe { game_state() }.r_plane.baseyscale);
        unsafe { game_state() }.r_draw.ds_ystep = unsafe { game_state() }.r_plane.cachedystep[y as usize];
    } else {
        distance = unsafe { game_state() }.r_plane.cacheddistance[y as usize];
        unsafe { game_state() }.r_draw.ds_xstep = unsafe { game_state() }.r_plane.cachedxstep[y as usize];
        unsafe { game_state() }.r_draw.ds_ystep = unsafe { game_state() }.r_plane.cachedystep[y as usize];
    }
    length = FixedMul(distance, unsafe { game_state() }.r_plane.distscale[x1 as usize]);
    angle = unsafe { game_state() }.r_main.viewangle.wrapping_add(unsafe { game_state() }.r_main.xtoviewangle[x1 as usize]) >> ANGLETOFINESHIFT;
    unsafe { game_state() }.r_draw.ds_xfrac = unsafe { game_state() }.r_main.viewx + FixedMul(finecosine[angle as isize], length);
    unsafe { game_state() }.r_draw.ds_yfrac = -unsafe { game_state() }.r_main.viewy - FixedMul(finesine[angle as usize], length);
    if !unsafe { game_state() }.r_main.fixedcolormap.is_null() {
        unsafe { game_state() }.r_draw.ds_colormap = unsafe { game_state() }.r_main.fixedcolormap;
    } else {
        index = (distance >> LIGHTZSHIFT) as u32;
        if index >= MAXLIGHTZ as u32 {
            index = (MAXLIGHTZ - 1 as i32) as u32;
        }
        unsafe { game_state() }.r_draw.ds_colormap = *unsafe { game_state() }.r_plane.planezlight.offset(index as isize);
    }
    unsafe { game_state() }.r_draw.ds_y = y;
    unsafe { game_state() }.r_draw.ds_x1 = x1;
    unsafe { game_state() }.r_draw.ds_x2 = x2;
    unsafe { game_state() }.r_main.spanfunc.expect("non-null function pointer")();
}
pub unsafe fn R_ClearPlanes() {
    let mut i: i32 = 0;
    let mut angle: angle_t = 0;
    i = 0 as i32;
    while i < unsafe { game_state() }.r_draw.viewwidth {
        unsafe { game_state() }.r_plane.floorclip[i as usize] = unsafe { game_state() }.r_draw.viewheight as i16;
        unsafe { game_state() }.r_plane.ceilingclip[i as usize] = -(1 as i32) as i16;
        i += 1;
    }
    unsafe { game_state() }.r_plane.lastvisplane = &raw mut unsafe { game_state() }.r_plane.visplanes as *mut visplane_t;
    unsafe { game_state() }.r_plane.lastopening = &raw mut unsafe { game_state() }.r_plane.openings as *mut i16;
    memset(
        &raw mut unsafe { game_state() }.r_plane.cachedheight as *mut fixed_t as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[fixed_t; 200]>() as size_t,
    );
    angle = unsafe { game_state() }.r_main.viewangle.wrapping_sub(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    unsafe { game_state() }.r_plane.basexscale = FixedDiv(finecosine[angle as isize], unsafe { game_state() }.r_main.centerxfrac);
    unsafe { game_state() }.r_plane.baseyscale = -FixedDiv(finesine[angle as usize], unsafe { game_state() }.r_main.centerxfrac);
}
pub unsafe fn R_FindPlane(
    state: &mut GameState,
    mut height: fixed_t,
    mut picnum: i32,
    mut lightlevel: i32,
) -> *mut visplane_t {
    let mut check: *mut visplane_t = ::core::ptr::null_mut::<visplane_t>();
    if picnum == state.r_sky.skyflatnum {
        height = 0 as i32 as fixed_t;
        lightlevel = 0 as i32;
    }
    check = &raw mut state.r_plane.visplanes as *mut visplane_t;
    while check < state.r_plane.lastvisplane {
        if height == (*check).height
            && picnum == (*check).picnum
            && lightlevel == (*check).lightlevel
        {
            break;
        }
        check = check.offset(1);
    }
    if check < state.r_plane.lastvisplane {
        return check;
    }
    if state.r_plane.lastvisplane.offset_from(&raw mut state.r_plane.visplanes as *mut visplane_t) as i64 == MAXVISPLANES as i64
    {
        I_Error("R_FindPlane: no more visplanes");
    }
    state.r_plane.lastvisplane = state.r_plane.lastvisplane.offset(1);
    (*check).height = height;
    (*check).picnum = picnum;
    (*check).lightlevel = lightlevel;
    (*check).minx = SCREENWIDTH;
    (*check).maxx = -(1 as i32);
    memset(
        &raw mut (*check).top as *mut byte as *mut ::core::ffi::c_void,
        0xff as i32,
        ::core::mem::size_of::<[byte; 320]>() as size_t,
    );
    return check;
}
pub unsafe fn R_CheckPlane(state: &mut GameState, 
    mut pl: *mut visplane_t,
    mut start: i32,
    mut stop: i32,
) -> *mut visplane_t {
    let mut intrl: i32 = 0;
    let mut intrh: i32 = 0;
    let mut unionl: i32 = 0;
    let mut unionh: i32 = 0;
    let mut x: i32 = 0;
    if start < (*pl).minx {
        intrl = (*pl).minx;
        unionl = start;
    } else {
        unionl = (*pl).minx;
        intrl = start;
    }
    if stop > (*pl).maxx {
        intrh = (*pl).maxx;
        unionh = stop;
    } else {
        unionh = (*pl).maxx;
        intrh = stop;
    }
    x = intrl;
    while x <= intrh {
        if (*pl).top[x as usize] as i32 != 0xff as i32 {
            break;
        }
        x += 1;
    }
    if x > intrh {
        (*pl).minx = unionl;
        (*pl).maxx = unionh;
        return pl;
    }
    (*state.r_plane.lastvisplane).height = (*pl).height;
    (*state.r_plane.lastvisplane).picnum = (*pl).picnum;
    (*state.r_plane.lastvisplane).lightlevel = (*pl).lightlevel;
    let fresh0 = state.r_plane.lastvisplane;
    state.r_plane.lastvisplane = state.r_plane.lastvisplane.offset(1);
    pl = fresh0;
    (*pl).minx = start;
    (*pl).maxx = stop;
    memset(
        &raw mut (*pl).top as *mut byte as *mut ::core::ffi::c_void,
        0xff as i32,
        ::core::mem::size_of::<[byte; 320]>() as size_t,
    );
    return pl;
}
pub unsafe fn R_MakeSpans(mut x: i32, mut t1: i32, mut b1: i32, mut t2: i32, mut b2: i32) {
    while t1 < t2 && t1 <= b1 {
        R_MapPlane(t1, unsafe { game_state() }.r_plane.spanstart[t1 as usize], x - 1 as i32);
        t1 += 1;
    }
    while b1 > b2 && b1 >= t1 {
        R_MapPlane(b1, unsafe { game_state() }.r_plane.spanstart[b1 as usize], x - 1 as i32);
        b1 -= 1;
    }
    while t2 < t1 && t2 <= b2 {
        unsafe { game_state() }.r_plane.spanstart[t2 as usize] = x;
        t2 += 1;
    }
    while b2 > b1 && b2 >= t2 {
        unsafe { game_state() }.r_plane.spanstart[b2 as usize] = x;
        b2 -= 1;
    }
}
pub unsafe fn R_DrawPlanes() {
    let mut pl: *mut visplane_t = ::core::ptr::null_mut::<visplane_t>();
    let mut light: i32 = 0;
    let mut x: i32 = 0;
    let mut stop: i32 = 0;
    let mut angle: i32 = 0;
    let mut lumpnum: i32 = 0;
    if unsafe { game_state() }.r_bsp.ds_p.offset_from(&raw mut unsafe { game_state() }.r_bsp.drawsegs as *mut drawseg_t) as i64 > MAXDRAWSEGS as i64 {
        I_Error(&format!(
            "R_DrawPlanes: drawsegs overflow ({})",
            unsafe { game_state() }.r_bsp.ds_p.offset_from(&raw mut unsafe { game_state() }.r_bsp.drawsegs as *mut drawseg_t) as i64,
        ));
    }
    if unsafe { game_state() }.r_plane.lastvisplane.offset_from(&raw mut unsafe { game_state() }.r_plane.visplanes as *mut visplane_t) as i64 > MAXVISPLANES as i64
    {
        I_Error(&format!(
            "R_DrawPlanes: visplane overflow ({})",
            unsafe { game_state() }.r_plane.lastvisplane.offset_from(&raw mut unsafe { game_state() }.r_plane.visplanes as *mut visplane_t) as i64,
        ));
    }
    if unsafe { game_state() }.r_plane.lastopening.offset_from(&raw mut unsafe { game_state() }.r_plane.openings as *mut i16) as i64
        > (SCREENWIDTH * 64 as i32) as i64
    {
        I_Error(&format!(
            "R_DrawPlanes: opening overflow ({})",
            unsafe { game_state() }.r_plane.lastopening.offset_from(&raw mut unsafe { game_state() }.r_plane.openings as *mut i16) as i64,
        ));
    }
    pl = &raw mut unsafe { game_state() }.r_plane.visplanes as *mut visplane_t;
    while pl < unsafe { game_state() }.r_plane.lastvisplane {
        if !((*pl).minx > (*pl).maxx) {
            if (*pl).picnum == unsafe { game_state() }.r_sky.skyflatnum {
                unsafe { game_state() }.r_draw.dc_iscale = unsafe { game_state() }.r_things.pspriteiscale >> unsafe { game_state() }.r_main.detailshift;
                unsafe { game_state() }.r_draw.dc_colormap = unsafe { game_state() }.r_data.colormaps;
                unsafe { game_state() }.r_draw.dc_texturemid = unsafe { game_state() }.r_sky.skytexturemid as fixed_t;
                x = (*pl).minx;
                while x <= (*pl).maxx {
                    unsafe { game_state() }.r_draw.dc_yl = (*pl).top[x as usize] as i32;
                    unsafe { game_state() }.r_draw.dc_yh = (*pl).bottom[x as usize] as i32;
                    if unsafe { game_state() }.r_draw.dc_yl <= unsafe { game_state() }.r_draw.dc_yh {
                        angle = (unsafe { game_state() }.r_main.viewangle.wrapping_add(unsafe { game_state() }.r_main.xtoviewangle[x as usize])
                            >> ANGLETOSKYSHIFT) as i32;
                        unsafe { game_state() }.r_draw.dc_x = x;
                        unsafe { game_state() }.r_draw.dc_source = R_GetColumn(unsafe { game_state() }.r_sky.skytexture, angle);
                        unsafe { game_state() }.r_main.colfunc.expect("non-null function pointer")();
                    }
                    x += 1;
                }
            } else {
                lumpnum = unsafe { game_state() }.r_data.firstflat + *unsafe { game_state() }.r_data.flattranslation.offset((*pl).picnum as isize);
                unsafe { game_state() }.r_draw.ds_source = W_CacheLumpNum(lumpnum, PU_STATIC as i32) as *mut byte;
                unsafe { game_state() }.r_plane.planeheight = ((*pl).height as i32 - unsafe { game_state() }.r_main.viewz as i32).abs() as fixed_t;
                light = ((*pl).lightlevel >> LIGHTSEGSHIFT) + unsafe { game_state() }.r_main.extralight;
                if light >= LIGHTLEVELS {
                    light = LIGHTLEVELS - 1 as i32;
                }
                if light < 0 as i32 {
                    light = 0 as i32;
                }
                unsafe { game_state() }.r_plane.planezlight = &raw mut *(&raw mut unsafe { game_state() }.r_main.zlight as *mut [*mut lighttable_t; 128])
                    .offset(light as isize) as *mut *mut lighttable_t;
                *(&raw mut (*pl).top as *mut byte).offset(((*pl).maxx + 1 as i32) as isize) =
                    0xff as byte;
                *(&raw mut (*pl).top as *mut byte).offset(((*pl).minx - 1 as i32) as isize) =
                    0xff as byte;
                stop = (*pl).maxx + 1 as i32;
                x = (*pl).minx;
                while x <= stop {
                    R_MakeSpans(
                        x,
                        *(&raw const (*pl).top as *const byte).offset((x - 1 as i32) as isize)
                            as i32,
                        *(&raw const (*pl).bottom as *const byte).offset((x - 1 as i32) as isize)
                            as i32,
                        *(&raw const (*pl).top as *const byte).offset(x as isize) as i32,
                        *(&raw const (*pl).bottom as *const byte).offset(x as isize) as i32,
                    );
                    x += 1;
                }
                W_ReleaseLumpNum(lumpnum);
            }
        }
        pl = pl.offset(1);
    }
}
