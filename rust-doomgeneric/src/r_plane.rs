use crate::src::doomdef::SCREENWIDTH;
use crate::src::i_system::I_Error;
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FixedDiv;
use crate::src::m_fixed::FixedMul;
use crate::src::r_bsp::drawsegs;
use crate::src::r_bsp::ds_p;
use crate::src::r_data::colormaps;
use crate::src::r_data::firstflat;
use crate::src::r_data::flattranslation;
use crate::src::r_data::R_GetColumn;
use crate::src::r_defs::lighttable_t;
use crate::src::r_defs::{drawseg_t, visplane_t};
use crate::src::r_draw::dc_colormap;
use crate::src::r_draw::dc_iscale;
use crate::src::r_draw::dc_source;
use crate::src::r_draw::dc_texturemid;
use crate::src::r_draw::dc_x;
use crate::src::r_draw::dc_yh;
use crate::src::r_draw::dc_yl;
use crate::src::r_draw::ds_colormap;
use crate::src::r_draw::ds_source;
use crate::src::r_draw::ds_x1;
use crate::src::r_draw::ds_x2;
use crate::src::r_draw::ds_xfrac;
use crate::src::r_draw::ds_xstep;
use crate::src::r_draw::ds_y;
use crate::src::r_draw::ds_yfrac;
use crate::src::r_draw::ds_ystep;
use crate::src::r_draw::viewheight;
use crate::src::r_draw::viewwidth;
use crate::src::r_main::centerxfrac;
use crate::src::r_main::colfunc;
use crate::src::r_main::detailshift;
use crate::src::r_main::extralight;
use crate::src::r_main::fixedcolormap;
use crate::src::r_main::spanfunc;
use crate::src::r_main::viewangle;
use crate::src::r_main::viewx;
use crate::src::r_main::viewy;
use crate::src::r_main::viewz;
use crate::src::r_main::xtoviewangle;
use crate::src::r_main::zlight;
use crate::src::r_main::LIGHTLEVELS;
use crate::src::r_main::LIGHTSEGSHIFT;
use crate::src::r_main::LIGHTZSHIFT;
use crate::src::r_main::MAXLIGHTZ;
use crate::src::r_segs::MAXDRAWSEGS;
use crate::src::r_sky::skyflatnum;
use crate::src::r_sky::skytexture;
use crate::src::r_sky::skytexturemid;
use crate::src::r_things::pspriteiscale;
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

pub type planefunction_t = Option<unsafe extern "C" fn(i32, i32) -> ()>;
pub const ANGLETOSKYSHIFT: i32 = 22;
#[no_mangle]
pub static mut floorfunc: planefunction_t = None;
#[no_mangle]
pub static mut ceilingfunc: planefunction_t = None;
pub const MAXVISPLANES: i32 = 128;
#[no_mangle]
pub static mut visplanes: [visplane_t; 128] = [visplane_t {
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
}; 128];
#[no_mangle]
pub static mut lastvisplane: *mut visplane_t = ::core::ptr::null::<visplane_t>() as *mut visplane_t;
pub static mut floorplane: *mut visplane_t = ::core::ptr::null::<visplane_t>() as *mut visplane_t;
pub static mut ceilingplane: *mut visplane_t = ::core::ptr::null::<visplane_t>() as *mut visplane_t;
#[no_mangle]
pub static mut openings: [i16; 20480] = [0; 20480];
pub static mut lastopening: *mut i16 = ::core::ptr::null::<i16>() as *mut i16;
pub static mut floorclip: [i16; 320] = [0; 320];
pub static mut ceilingclip: [i16; 320] = [0; 320];
#[no_mangle]
pub static mut spanstart: [i32; 200] = [0; 200];
#[no_mangle]
pub static mut spanstop: [i32; 200] = [0; 200];
#[no_mangle]
pub static mut planezlight: *mut *mut lighttable_t =
    ::core::ptr::null::<*mut lighttable_t>() as *mut *mut lighttable_t;
#[no_mangle]
pub static mut planeheight: fixed_t = 0;
pub static mut yslope: [fixed_t; 200] = [0; 200];
pub static mut distscale: [fixed_t; 320] = [0; 320];
#[no_mangle]
pub static mut basexscale: fixed_t = 0;
#[no_mangle]
pub static mut baseyscale: fixed_t = 0;
#[no_mangle]
pub static mut cachedheight: [fixed_t; 200] = [0; 200];
#[no_mangle]
pub static mut cacheddistance: [fixed_t; 200] = [0; 200];
#[no_mangle]
pub static mut cachedxstep: [fixed_t; 200] = [0; 200];
#[no_mangle]
pub static mut cachedystep: [fixed_t; 200] = [0; 200];
pub unsafe fn R_MapPlane(mut y: i32, mut x1: i32, mut x2: i32) {
    let mut angle: angle_t = 0;
    let mut distance: fixed_t = 0;
    let mut length: fixed_t = 0;
    let mut index: u32 = 0;
    if x2 < x1 || x1 < 0 as i32 || x2 >= viewwidth || y > viewheight {
        I_Error(&format!("R_MapPlane: {}, {} at {}", x1, x2, y));
    }
    if planeheight != cachedheight[y as usize] {
        cachedheight[y as usize] = planeheight;
        cacheddistance[y as usize] = FixedMul(planeheight, yslope[y as usize]);
        distance = cacheddistance[y as usize];
        cachedxstep[y as usize] = FixedMul(distance, basexscale);
        ds_xstep = cachedxstep[y as usize];
        cachedystep[y as usize] = FixedMul(distance, baseyscale);
        ds_ystep = cachedystep[y as usize];
    } else {
        distance = cacheddistance[y as usize];
        ds_xstep = cachedxstep[y as usize];
        ds_ystep = cachedystep[y as usize];
    }
    length = FixedMul(distance, distscale[x1 as usize]);
    angle = viewangle.wrapping_add(xtoviewangle[x1 as usize]) >> ANGLETOFINESHIFT;
    ds_xfrac = viewx + FixedMul(finecosine[angle as isize], length);
    ds_yfrac = -viewy - FixedMul(finesine[angle as usize], length);
    if !fixedcolormap.is_null() {
        ds_colormap = fixedcolormap;
    } else {
        index = (distance >> LIGHTZSHIFT) as u32;
        if index >= MAXLIGHTZ as u32 {
            index = (MAXLIGHTZ - 1 as i32) as u32;
        }
        ds_colormap = *planezlight.offset(index as isize);
    }
    ds_y = y;
    ds_x1 = x1;
    ds_x2 = x2;
    spanfunc.expect("non-null function pointer")();
}
pub unsafe fn R_ClearPlanes() {
    let mut i: i32 = 0;
    let mut angle: angle_t = 0;
    i = 0 as i32;
    while i < viewwidth {
        floorclip[i as usize] = viewheight as i16;
        ceilingclip[i as usize] = -(1 as i32) as i16;
        i += 1;
    }
    lastvisplane = &raw mut visplanes as *mut visplane_t;
    lastopening = &raw mut openings as *mut i16;
    memset(
        &raw mut cachedheight as *mut fixed_t as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[fixed_t; 200]>() as size_t,
    );
    angle = viewangle.wrapping_sub(ANG90 as angle_t) >> ANGLETOFINESHIFT;
    basexscale = FixedDiv(finecosine[angle as isize], centerxfrac);
    baseyscale = -FixedDiv(finesine[angle as usize], centerxfrac);
}
pub unsafe fn R_FindPlane(
    mut height: fixed_t,
    mut picnum: i32,
    mut lightlevel: i32,
) -> *mut visplane_t {
    let mut check: *mut visplane_t = ::core::ptr::null_mut::<visplane_t>();
    if picnum == skyflatnum {
        height = 0 as i32 as fixed_t;
        lightlevel = 0 as i32;
    }
    check = &raw mut visplanes as *mut visplane_t;
    while check < lastvisplane {
        if height == (*check).height
            && picnum == (*check).picnum
            && lightlevel == (*check).lightlevel
        {
            break;
        }
        check = check.offset(1);
    }
    if check < lastvisplane {
        return check;
    }
    if lastvisplane.offset_from(&raw mut visplanes as *mut visplane_t) as i64 == MAXVISPLANES as i64
    {
        I_Error("R_FindPlane: no more visplanes");
    }
    lastvisplane = lastvisplane.offset(1);
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
pub unsafe fn R_CheckPlane(
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
    (*lastvisplane).height = (*pl).height;
    (*lastvisplane).picnum = (*pl).picnum;
    (*lastvisplane).lightlevel = (*pl).lightlevel;
    let fresh0 = lastvisplane;
    lastvisplane = lastvisplane.offset(1);
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
        R_MapPlane(t1, spanstart[t1 as usize], x - 1 as i32);
        t1 += 1;
    }
    while b1 > b2 && b1 >= t1 {
        R_MapPlane(b1, spanstart[b1 as usize], x - 1 as i32);
        b1 -= 1;
    }
    while t2 < t1 && t2 <= b2 {
        spanstart[t2 as usize] = x;
        t2 += 1;
    }
    while b2 > b1 && b2 >= t2 {
        spanstart[b2 as usize] = x;
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
    if ds_p.offset_from(&raw mut drawsegs as *mut drawseg_t) as i64 > MAXDRAWSEGS as i64 {
        I_Error(&format!(
            "R_DrawPlanes: drawsegs overflow ({})",
            ds_p.offset_from(&raw mut drawsegs as *mut drawseg_t) as i64,
        ));
    }
    if lastvisplane.offset_from(&raw mut visplanes as *mut visplane_t) as i64 > MAXVISPLANES as i64
    {
        I_Error(&format!(
            "R_DrawPlanes: visplane overflow ({})",
            lastvisplane.offset_from(&raw mut visplanes as *mut visplane_t) as i64,
        ));
    }
    if lastopening.offset_from(&raw mut openings as *mut i16) as i64
        > (SCREENWIDTH * 64 as i32) as i64
    {
        I_Error(&format!(
            "R_DrawPlanes: opening overflow ({})",
            lastopening.offset_from(&raw mut openings as *mut i16) as i64,
        ));
    }
    pl = &raw mut visplanes as *mut visplane_t;
    while pl < lastvisplane {
        if !((*pl).minx > (*pl).maxx) {
            if (*pl).picnum == skyflatnum {
                dc_iscale = pspriteiscale >> detailshift;
                dc_colormap = colormaps;
                dc_texturemid = skytexturemid as fixed_t;
                x = (*pl).minx;
                while x <= (*pl).maxx {
                    dc_yl = (*pl).top[x as usize] as i32;
                    dc_yh = (*pl).bottom[x as usize] as i32;
                    if dc_yl <= dc_yh {
                        angle = (viewangle.wrapping_add(xtoviewangle[x as usize])
                            >> ANGLETOSKYSHIFT) as i32;
                        dc_x = x;
                        dc_source = R_GetColumn(skytexture, angle);
                        colfunc.expect("non-null function pointer")();
                    }
                    x += 1;
                }
            } else {
                lumpnum = firstflat + *flattranslation.offset((*pl).picnum as isize);
                ds_source = W_CacheLumpNum(lumpnum, PU_STATIC as i32) as *mut byte;
                planeheight = ((*pl).height as i32 - viewz as i32).abs() as fixed_t;
                light = ((*pl).lightlevel >> LIGHTSEGSHIFT) + extralight;
                if light >= LIGHTLEVELS {
                    light = LIGHTLEVELS - 1 as i32;
                }
                if light < 0 as i32 {
                    light = 0 as i32;
                }
                planezlight = &raw mut *(&raw mut zlight as *mut [*mut lighttable_t; 128])
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
