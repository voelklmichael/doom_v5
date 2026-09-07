use crate::src::game_state::game_state;
use crate::src::i_system::I_Error;
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FixedMul;
use crate::src::m_fixed::FRACBITS;
use crate::src::m_fixed::INT_MAX;
use crate::src::m_fixed::INT_MIN;
use crate::src::p_spec::ML_MAPPED;
use crate::src::r_data::column_t;
use crate::src::r_data::R_GetColumn;
use crate::src::r_defs::drawseg_t;
use crate::src::r_defs::lighttable_t;
use crate::src::r_main::R_PointToDist;
use crate::src::r_main::R_ScaleFromGlobalAngle;
use crate::src::r_main::LIGHTLEVELS;
use crate::src::r_main::LIGHTSCALESHIFT;
use crate::src::r_main::LIGHTSEGSHIFT;
use crate::src::r_main::MAXLIGHTSCALE;
use crate::src::r_plane::R_CheckPlane;
use crate::src::r_things::R_DrawMaskedColumn;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::tables::angle_t;
use crate::src::tables::finesine;
use crate::src::tables::finetangent;
use crate::src::tables::ANG180;
use crate::src::tables::ANG90;
use crate::src::tables::ANGLETOFINESHIFT;
use libc::memcpy;

pub struct RSegsState {
    pub segtextured: bool,
    pub markfloor: bool,
    pub markceiling: bool,
    pub maskedtexture: bool,
    pub toptexture: i32,
    pub bottomtexture: i32,
    pub midtexture: i32,
    pub rw_normalangle: angle_t,
    pub rw_angle1: i32,
    pub rw_x: i32,
    pub rw_stopx: i32,
    pub rw_centerangle: angle_t,
    pub rw_offset: fixed_t,
    pub rw_distance: fixed_t,
    pub rw_scale: fixed_t,
    pub rw_scalestep: fixed_t,
    pub rw_midtexturemid: fixed_t,
    pub rw_toptexturemid: fixed_t,
    pub rw_bottomtexturemid: fixed_t,
    pub worldtop: i32,
    pub worldbottom: i32,
    pub worldhigh: i32,
    pub worldlow: i32,
    pub pixhigh: fixed_t,
    pub pixlow: fixed_t,
    pub pixhighstep: fixed_t,
    pub pixlowstep: fixed_t,
    pub topfrac: fixed_t,
    pub topstep: fixed_t,
    pub bottomfrac: fixed_t,
    pub bottomstep: fixed_t,
    pub walllights: *mut *mut lighttable_t,
    pub maskedtexturecol: *mut i16,
}

impl RSegsState {
    pub const fn new() -> Self {
        RSegsState {
            segtextured: false,
            markfloor: false,
            markceiling: false,
            maskedtexture: false,
            toptexture: 0,
            bottomtexture: 0,
            midtexture: 0,
            rw_normalangle: 0,
            rw_angle1: 0,
            rw_x: 0,
            rw_stopx: 0,
            rw_centerangle: 0,
            rw_offset: 0,
            rw_distance: 0,
            rw_scale: 0,
            rw_scalestep: 0,
            rw_midtexturemid: 0,
            rw_toptexturemid: 0,
            rw_bottomtexturemid: 0,
            worldtop: 0,
            worldbottom: 0,
            worldhigh: 0,
            worldlow: 0,
            pixhigh: 0,
            pixlow: 0,
            pixhighstep: 0,
            pixlowstep: 0,
            topfrac: 0,
            topstep: 0,
            bottomfrac: 0,
            bottomstep: 0,
            walllights: 
        ::core::ptr::null::<*mut lighttable_t>() as *mut *mut lighttable_t,
            maskedtexturecol: ::core::ptr::null::<i16>() as *mut i16,
        }
    }
}


pub const SHRT_MAX: i32 = __SHRT_MAX__;
pub const ML_DONTPEGTOP: i32 = 8;
pub const ML_DONTPEGBOTTOM: i32 = 16;
pub const SIL_BOTTOM: i32 = 1;
pub const SIL_TOP: i32 = 2;
pub const SIL_BOTH: i32 = 3;
pub const MAXDRAWSEGS: i32 = 256;
pub unsafe fn R_RenderMaskedSegRange(mut ds: *mut drawseg_t, mut x1: i32, mut x2: i32) {
    let mut index: u32 = 0;
    let mut col: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut lightnum: i32 = 0;
    let mut texnum: i32 = 0;
    unsafe { game_state() }.r_bsp.curline = (*ds).curline;
    unsafe { game_state() }.r_bsp.frontsector = (*unsafe { game_state() }.r_bsp.curline).frontsector;
    unsafe { game_state() }.r_bsp.backsector = (*unsafe { game_state() }.r_bsp.curline).backsector;
    texnum = *unsafe { game_state() }.r_data.texturetranslation.offset((*unsafe { game_state() }.p_setup.side_mut((*unsafe { game_state() }.r_bsp.curline).sidedef)).midtexture as isize);
    lightnum = ((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32 >> LIGHTSEGSHIFT) + unsafe { game_state() }.r_main.extralight;
    if (*(*unsafe { game_state() }.r_bsp.curline).v1).y == (*(*unsafe { game_state() }.r_bsp.curline).v2).y {
        lightnum -= 1;
    } else if (*(*unsafe { game_state() }.r_bsp.curline).v1).x == (*(*unsafe { game_state() }.r_bsp.curline).v2).x {
        lightnum += 1;
    }
    if lightnum < 0 as i32 {
        unsafe { game_state() }.r_segs.walllights = &raw mut *(&raw mut unsafe { game_state() }.r_main.scalelight as *mut [*mut lighttable_t; 48])
            .offset(0 as i32 as isize) as *mut *mut lighttable_t;
    } else if lightnum >= LIGHTLEVELS {
        unsafe { game_state() }.r_segs.walllights = &raw mut *(&raw mut unsafe { game_state() }.r_main.scalelight as *mut [*mut lighttable_t; 48])
            .offset((LIGHTLEVELS - 1 as i32) as isize)
            as *mut *mut lighttable_t;
    } else {
        unsafe { game_state() }.r_segs.walllights = &raw mut *(&raw mut unsafe { game_state() }.r_main.scalelight as *mut [*mut lighttable_t; 48])
            .offset(lightnum as isize) as *mut *mut lighttable_t;
    }
    unsafe { game_state() }.r_segs.maskedtexturecol = (*ds).maskedtexturecol;
    unsafe { game_state() }.r_segs.rw_scalestep = (*ds).scalestep;
    unsafe { game_state() }.r_things.spryscale =
        (*ds).scale1 + (x1 as fixed_t - (*ds).x1 as fixed_t) * unsafe { game_state() }.r_segs.rw_scalestep;
    unsafe { game_state() }.r_things.mfloorclip = (*ds).sprbottomclip;
    unsafe { game_state() }.r_things.mceilingclip = (*ds).sprtopclip;
    if (*(*unsafe { game_state() }.r_bsp.curline).linedef).flags as i32 & ML_DONTPEGBOTTOM != 0 {
        unsafe { game_state() }.r_draw.dc_texturemid = if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight > (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight {
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight
        } else {
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight
        };
        unsafe { game_state() }.r_draw.dc_texturemid = unsafe { game_state() }.r_draw.dc_texturemid + *unsafe { game_state() }.r_data.textureheight.offset(texnum as isize) - unsafe { game_state() }.r_main.viewz;
    } else {
        unsafe { game_state() }.r_draw.dc_texturemid = if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight < (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight {
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight
        } else {
            (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight
        };
        unsafe { game_state() }.r_draw.dc_texturemid = unsafe { game_state() }.r_draw.dc_texturemid - unsafe { game_state() }.r_main.viewz;
    }
    unsafe { game_state() }.r_draw.dc_texturemid += (*unsafe { game_state() }.p_setup.side_mut((*unsafe { game_state() }.r_bsp.curline).sidedef)).rowoffset;
    if !unsafe { game_state() }.r_main.fixedcolormap.is_null() {
        unsafe { game_state() }.r_draw.dc_colormap = unsafe { game_state() }.r_main.fixedcolormap;
    }
    unsafe { game_state() }.r_draw.dc_x = x1;
    while unsafe { game_state() }.r_draw.dc_x <= x2 {
        if *unsafe { game_state() }.r_segs.maskedtexturecol.offset(unsafe { game_state() }.r_draw.dc_x as isize) as i32 != SHRT_MAX {
            if unsafe { game_state() }.r_main.fixedcolormap.is_null() {
                index = (unsafe { game_state() }.r_things.spryscale >> LIGHTSCALESHIFT) as u32;
                if index >= MAXLIGHTSCALE as u32 {
                    index = (MAXLIGHTSCALE - 1 as i32) as u32;
                }
                unsafe { game_state() }.r_draw.dc_colormap = *unsafe { game_state() }.r_segs.walllights.offset(index as isize);
            }
            unsafe { game_state() }.r_things.sprtopscreen =
                unsafe { game_state() }.r_main.centeryfrac - FixedMul(unsafe { game_state() }.r_draw.dc_texturemid, unsafe { game_state() }.r_things.spryscale);
            unsafe { game_state() }.r_draw.dc_iscale = (0xffffffff as u32)
                .wrapping_div(unsafe { game_state() }.r_things.spryscale as u32)
                as fixed_t;
            col = R_GetColumn(texnum, *unsafe { game_state() }.r_segs.maskedtexturecol.offset(unsafe { game_state() }.r_draw.dc_x as isize) as i32)
                .offset(-(3 as i32 as isize)) as *mut column_t;
            R_DrawMaskedColumn(col);
            *unsafe { game_state() }.r_segs.maskedtexturecol.offset(unsafe { game_state() }.r_draw.dc_x as isize) = SHRT_MAX as i16;
        }
        unsafe { game_state() }.r_things.spryscale += unsafe { game_state() }.r_segs.rw_scalestep;
        unsafe { game_state() }.r_draw.dc_x += 1;
    }
}
pub const HEIGHTBITS: i32 = 12;
pub const HEIGHTUNIT: i32 = (1 as i32) << HEIGHTBITS;
pub unsafe fn R_RenderSegLoop() {
    let mut angle: angle_t = 0;
    let mut index: u32 = 0;
    let mut yl: i32 = 0;
    let mut yh: i32 = 0;
    let mut mid: i32 = 0;
    let mut texturecolumn: fixed_t = 0;
    let mut top: i32 = 0;
    let mut bottom: i32 = 0;
    while unsafe { game_state() }.r_segs.rw_x < unsafe { game_state() }.r_segs.rw_stopx {
        yl = unsafe { game_state() }.r_segs.topfrac as i32 + HEIGHTUNIT - 1 as i32 >> HEIGHTBITS;
        if yl < unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 + 1 as i32 {
            yl = unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 + 1 as i32;
        }
        if unsafe { game_state() }.r_segs.markceiling {
            top = unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 + 1 as i32;
            bottom = yl - 1 as i32;
            if bottom >= unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 {
                bottom = unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 - 1 as i32;
            }
            if top <= bottom {
                (*unsafe { game_state() }.r_plane.ceilingplane).top[unsafe { game_state() }.r_segs.rw_x as usize] = top as byte;
                (*unsafe { game_state() }.r_plane.ceilingplane).bottom[unsafe { game_state() }.r_segs.rw_x as usize] = bottom as byte;
            }
        }
        yh = (unsafe { game_state() }.r_segs.bottomfrac >> HEIGHTBITS) as i32;
        if yh >= unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 {
            yh = unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 - 1 as i32;
        }
        if unsafe { game_state() }.r_segs.markfloor {
            top = yh + 1 as i32;
            bottom = unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 - 1 as i32;
            if top <= unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 {
                top = unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 + 1 as i32;
            }
            if top <= bottom {
                (*unsafe { game_state() }.r_plane.floorplane).top[unsafe { game_state() }.r_segs.rw_x as usize] = top as byte;
                (*unsafe { game_state() }.r_plane.floorplane).bottom[unsafe { game_state() }.r_segs.rw_x as usize] = bottom as byte;
            }
        }
        if unsafe { game_state() }.r_segs.segtextured {
            angle = unsafe { game_state() }.r_segs.rw_centerangle.wrapping_add(unsafe { game_state() }.r_main.xtoviewangle[unsafe { game_state() }.r_segs.rw_x as usize]) >> ANGLETOFINESHIFT;
            texturecolumn = unsafe { game_state() }.r_segs.rw_offset - FixedMul(finetangent[angle as usize], unsafe { game_state() }.r_segs.rw_distance);
            texturecolumn >>= FRACBITS;
            index = (unsafe { game_state() }.r_segs.rw_scale >> LIGHTSCALESHIFT) as u32;
            if index >= MAXLIGHTSCALE as u32 {
                index = (MAXLIGHTSCALE - 1 as i32) as u32;
            }
            unsafe { game_state() }.r_draw.dc_colormap = *unsafe { game_state() }.r_segs.walllights.offset(index as isize);
            unsafe { game_state() }.r_draw.dc_x = unsafe { game_state() }.r_segs.rw_x;
            unsafe { game_state() }.r_draw.dc_iscale = (0xffffffff as u32).wrapping_div(unsafe { game_state() }.r_segs.rw_scale as u32) as fixed_t;
        } else {
            texturecolumn = 0 as i32 as fixed_t;
        }
        if unsafe { game_state() }.r_segs.midtexture != 0 {
            unsafe { game_state() }.r_draw.dc_yl = yl;
            unsafe { game_state() }.r_draw.dc_yh = yh;
            unsafe { game_state() }.r_draw.dc_texturemid = unsafe { game_state() }.r_segs.rw_midtexturemid;
            unsafe { game_state() }.r_draw.dc_source = R_GetColumn(unsafe { game_state() }.r_segs.midtexture, texturecolumn as i32);
            unsafe { game_state() }.r_main.colfunc.expect("non-null function pointer")();
            unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] = unsafe { game_state() }.r_draw.viewheight as i16;
            unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] = -(1 as i32) as i16;
        } else {
            if unsafe { game_state() }.r_segs.toptexture != 0 {
                mid = (unsafe { game_state() }.r_segs.pixhigh >> HEIGHTBITS) as i32;
                unsafe { game_state() }.r_segs.pixhigh += unsafe { game_state() }.r_segs.pixhighstep;
                if mid >= unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 {
                    mid = unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 - 1 as i32;
                }
                if mid >= yl {
                    unsafe { game_state() }.r_draw.dc_yl = yl;
                    unsafe { game_state() }.r_draw.dc_yh = mid;
                    unsafe { game_state() }.r_draw.dc_texturemid = unsafe { game_state() }.r_segs.rw_toptexturemid;
                    unsafe { game_state() }.r_draw.dc_source = R_GetColumn(unsafe { game_state() }.r_segs.toptexture, texturecolumn as i32);
                    unsafe { game_state() }.r_main.colfunc.expect("non-null function pointer")();
                    unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] = mid as i16;
                } else {
                    unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] = (yl - 1 as i32) as i16;
                }
            } else if unsafe { game_state() }.r_segs.markceiling {
                unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] = (yl - 1 as i32) as i16;
            }
            if unsafe { game_state() }.r_segs.bottomtexture != 0 {
                mid = unsafe { game_state() }.r_segs.pixlow as i32 + HEIGHTUNIT - 1 as i32 >> HEIGHTBITS;
                unsafe { game_state() }.r_segs.pixlow += unsafe { game_state() }.r_segs.pixlowstep;
                if mid <= unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 {
                    mid = unsafe { game_state() }.r_plane.ceilingclip[unsafe { game_state() }.r_segs.rw_x as usize] as i32 + 1 as i32;
                }
                if mid <= yh {
                    unsafe { game_state() }.r_draw.dc_yl = mid;
                    unsafe { game_state() }.r_draw.dc_yh = yh;
                    unsafe { game_state() }.r_draw.dc_texturemid = unsafe { game_state() }.r_segs.rw_bottomtexturemid;
                    unsafe { game_state() }.r_draw.dc_source = R_GetColumn(unsafe { game_state() }.r_segs.bottomtexture, texturecolumn as i32);
                    unsafe { game_state() }.r_main.colfunc.expect("non-null function pointer")();
                    unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] = mid as i16;
                } else {
                    unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] = (yh + 1 as i32) as i16;
                }
            } else if unsafe { game_state() }.r_segs.markfloor {
                unsafe { game_state() }.r_plane.floorclip[unsafe { game_state() }.r_segs.rw_x as usize] = (yh + 1 as i32) as i16;
            }
            if unsafe { game_state() }.r_segs.maskedtexture {
                *unsafe { game_state() }.r_segs.maskedtexturecol.offset(unsafe { game_state() }.r_segs.rw_x as isize) = texturecolumn as i16;
            }
        }
        unsafe { game_state() }.r_segs.rw_scale += unsafe { game_state() }.r_segs.rw_scalestep;
        unsafe { game_state() }.r_segs.topfrac += unsafe { game_state() }.r_segs.topstep;
        unsafe { game_state() }.r_segs.bottomfrac += unsafe { game_state() }.r_segs.bottomstep;
        unsafe { game_state() }.r_segs.rw_x += 1;
    }
}
pub unsafe fn R_StoreWallRange(mut start: i32, mut stop: i32) {
    let mut hyp: fixed_t = 0;
    let mut sineval: fixed_t = 0;
    let mut distangle: angle_t = 0;
    let mut offsetangle: angle_t = 0;
    let mut vtop: fixed_t = 0;
    let mut lightnum: i32 = 0;
    if unsafe { game_state() }.r_bsp.ds_p == (&raw mut unsafe { game_state() }.r_bsp.drawsegs as *mut drawseg_t).offset(MAXDRAWSEGS as isize) as *mut drawseg_t
    {
        return;
    }
    if start >= unsafe { game_state() }.r_draw.viewwidth || start > stop {
        I_Error(&format!("Bad R_RenderWallRange: {} to {}", start, stop));
    }
    unsafe { game_state() }.r_bsp.sidedef = (*unsafe { game_state() }.r_bsp.curline).sidedef;
    unsafe { game_state() }.r_bsp.linedef = (*unsafe { game_state() }.r_bsp.curline).linedef;
    (*unsafe { game_state() }.r_bsp.linedef).flags = ((*unsafe { game_state() }.r_bsp.linedef).flags as i32 | ML_MAPPED) as i16;
    unsafe { game_state() }.r_segs.rw_normalangle = (*unsafe { game_state() }.r_bsp.curline).angle.wrapping_add(ANG90 as angle_t);
    offsetangle = (unsafe { game_state() }.r_segs.rw_normalangle.wrapping_sub(unsafe { game_state() }.r_segs.rw_angle1 as angle_t) as i32).abs() as angle_t;
    if offsetangle > ANG90 as angle_t {
        offsetangle = ANG90 as angle_t;
    }
    distangle = (ANG90 as angle_t).wrapping_sub(offsetangle);
    hyp = R_PointToDist((*(*unsafe { game_state() }.r_bsp.curline).v1).x, (*(*unsafe { game_state() }.r_bsp.curline).v1).y);
    sineval = finesine[(distangle >> ANGLETOFINESHIFT) as usize];
    unsafe { game_state() }.r_segs.rw_distance = FixedMul(hyp, sineval);
    unsafe { game_state() }.r_segs.rw_x = start;
    (*unsafe { game_state() }.r_bsp.ds_p).x1 = unsafe { game_state() }.r_segs.rw_x;
    (*unsafe { game_state() }.r_bsp.ds_p).x2 = stop;
    (*unsafe { game_state() }.r_bsp.ds_p).curline = unsafe { game_state() }.r_bsp.curline;
    unsafe { game_state() }.r_segs.rw_stopx = stop + 1 as i32;
    unsafe { game_state() }.r_segs.rw_scale = R_ScaleFromGlobalAngle(unsafe { game_state() }.r_main.viewangle.wrapping_add(unsafe { game_state() }.r_main.xtoviewangle[start as usize]));
    (*unsafe { game_state() }.r_bsp.ds_p).scale1 = unsafe { game_state() }.r_segs.rw_scale;
    if stop > start {
        (*unsafe { game_state() }.r_bsp.ds_p).scale2 =
            R_ScaleFromGlobalAngle(unsafe { game_state() }.r_main.viewangle.wrapping_add(unsafe { game_state() }.r_main.xtoviewangle[stop as usize]));
        unsafe { game_state() }.r_segs.rw_scalestep = (((*unsafe { game_state() }.r_bsp.ds_p).scale2 as i32 - unsafe { game_state() }.r_segs.rw_scale as i32) / (stop - start)) as fixed_t;
        (*unsafe { game_state() }.r_bsp.ds_p).scalestep = unsafe { game_state() }.r_segs.rw_scalestep;
    } else {
        (*unsafe { game_state() }.r_bsp.ds_p).scale2 = (*unsafe { game_state() }.r_bsp.ds_p).scale1;
    }
    unsafe { game_state() }.r_segs.worldtop = ((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight - unsafe { game_state() }.r_main.viewz) as i32;
    unsafe { game_state() }.r_segs.worldbottom = ((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight - unsafe { game_state() }.r_main.viewz) as i32;
    unsafe { game_state() }.r_segs.maskedtexture = false;
    unsafe { game_state() }.r_segs.bottomtexture = unsafe { game_state() }.r_segs.maskedtexture as i32;
    unsafe { game_state() }.r_segs.toptexture = unsafe { game_state() }.r_segs.bottomtexture;
    unsafe { game_state() }.r_segs.midtexture = unsafe { game_state() }.r_segs.toptexture;
    (*unsafe { game_state() }.r_bsp.ds_p).maskedtexturecol = ::core::ptr::null_mut::<i16>();
    if unsafe { game_state() }.r_bsp.backsector.is_none() {
        unsafe { game_state() }.r_segs.midtexture = *unsafe { game_state() }.r_data.texturetranslation.offset((*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).midtexture as isize);
        unsafe { game_state() }.r_segs.markceiling = true;
        unsafe { game_state() }.r_segs.markfloor = unsafe { game_state() }.r_segs.markceiling;
        if (*unsafe { game_state() }.r_bsp.linedef).flags as i32 & ML_DONTPEGBOTTOM != 0 {
            vtop =
                (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight + *unsafe { game_state() }.r_data.textureheight.offset((*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).midtexture as isize);
            unsafe { game_state() }.r_segs.rw_midtexturemid = vtop - unsafe { game_state() }.r_main.viewz;
        } else {
            unsafe { game_state() }.r_segs.rw_midtexturemid = unsafe { game_state() }.r_segs.worldtop as fixed_t;
        }
        unsafe { game_state() }.r_segs.rw_midtexturemid += (*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).rowoffset;
        (*unsafe { game_state() }.r_bsp.ds_p).silhouette = SIL_BOTH;
        (*unsafe { game_state() }.r_bsp.ds_p).sprtopclip =
            &raw mut unsafe { game_state() }.r_things.screenheightarray as *mut i16;
        (*unsafe { game_state() }.r_bsp.ds_p).sprbottomclip = &raw mut unsafe { game_state() }.r_things.negonearray as *mut i16;
        (*unsafe { game_state() }.r_bsp.ds_p).bsilheight = INT_MAX as fixed_t;
        (*unsafe { game_state() }.r_bsp.ds_p).tsilheight = INT_MIN as fixed_t;
    } else {
        (*unsafe { game_state() }.r_bsp.ds_p).sprbottomclip = ::core::ptr::null_mut::<i16>();
        (*unsafe { game_state() }.r_bsp.ds_p).sprtopclip = (*unsafe { game_state() }.r_bsp.ds_p).sprbottomclip;
        (*unsafe { game_state() }.r_bsp.ds_p).silhouette = 0 as i32;
        if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight > (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight {
            (*unsafe { game_state() }.r_bsp.ds_p).silhouette = SIL_BOTTOM;
            (*unsafe { game_state() }.r_bsp.ds_p).bsilheight = (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight;
        } else if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight > unsafe { game_state() }.r_main.viewz {
            (*unsafe { game_state() }.r_bsp.ds_p).silhouette = SIL_BOTTOM;
            (*unsafe { game_state() }.r_bsp.ds_p).bsilheight = INT_MAX as fixed_t;
        }
        if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight < (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight {
            (*unsafe { game_state() }.r_bsp.ds_p).silhouette |= SIL_TOP;
            (*unsafe { game_state() }.r_bsp.ds_p).tsilheight = (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight;
        } else if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight < unsafe { game_state() }.r_main.viewz {
            (*unsafe { game_state() }.r_bsp.ds_p).silhouette |= SIL_TOP;
            (*unsafe { game_state() }.r_bsp.ds_p).tsilheight = INT_MIN as fixed_t;
        }
        if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight <= (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight {
            (*unsafe { game_state() }.r_bsp.ds_p).sprbottomclip =
                &raw mut unsafe { game_state() }.r_things.negonearray as *mut i16;
            (*unsafe { game_state() }.r_bsp.ds_p).bsilheight = INT_MAX as fixed_t;
            (*unsafe { game_state() }.r_bsp.ds_p).silhouette |= SIL_BOTTOM;
        }
        if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight >= (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight {
            (*unsafe { game_state() }.r_bsp.ds_p).sprtopclip =
                &raw mut unsafe { game_state() }.r_things.screenheightarray as *mut i16;
            (*unsafe { game_state() }.r_bsp.ds_p).tsilheight = INT_MIN as fixed_t;
            (*unsafe { game_state() }.r_bsp.ds_p).silhouette |= SIL_TOP;
        }
        unsafe { game_state() }.r_segs.worldhigh = ((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight - unsafe { game_state() }.r_main.viewz) as i32;
        unsafe { game_state() }.r_segs.worldlow = ((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight - unsafe { game_state() }.r_main.viewz) as i32;
        if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingpic as i32 == unsafe { game_state() }.r_sky.skyflatnum
            && (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingpic as i32 == unsafe { game_state() }.r_sky.skyflatnum
        {
            unsafe { game_state() }.r_segs.worldtop = unsafe { game_state() }.r_segs.worldhigh;
        }
        if unsafe { game_state() }.r_segs.worldlow != unsafe { game_state() }.r_segs.worldbottom
            || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorpic as i32 != (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorpic as i32
            || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).lightlevel as i32 != (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32
        {
            unsafe { game_state() }.r_segs.markfloor = true;
        } else {
            unsafe { game_state() }.r_segs.markfloor = false;
        }
        if unsafe { game_state() }.r_segs.worldhigh != unsafe { game_state() }.r_segs.worldtop
            || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingpic as i32 != (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingpic as i32
            || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).lightlevel as i32 != (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32
        {
            unsafe { game_state() }.r_segs.markceiling = true;
        } else {
            unsafe { game_state() }.r_segs.markceiling = false;
        }
        if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight <= (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight
            || (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).floorheight >= (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight
        {
            unsafe { game_state() }.r_segs.markfloor = true;
            unsafe { game_state() }.r_segs.markceiling = unsafe { game_state() }.r_segs.markfloor;
        }
        if unsafe { game_state() }.r_segs.worldhigh < unsafe { game_state() }.r_segs.worldtop {
            unsafe { game_state() }.r_segs.toptexture = *unsafe { game_state() }.r_data.texturetranslation.offset((*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).toptexture as isize);
            if (*unsafe { game_state() }.r_bsp.linedef).flags as i32 & ML_DONTPEGTOP != 0 {
                unsafe { game_state() }.r_segs.rw_toptexturemid = unsafe { game_state() }.r_segs.worldtop as fixed_t;
            } else {
                vtop = (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.backsector.unwrap())).ceilingheight
                    + *unsafe { game_state() }.r_data.textureheight.offset((*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).toptexture as isize);
                unsafe { game_state() }.r_segs.rw_toptexturemid = vtop - unsafe { game_state() }.r_main.viewz;
            }
        }
        if unsafe { game_state() }.r_segs.worldlow > unsafe { game_state() }.r_segs.worldbottom {
            unsafe { game_state() }.r_segs.bottomtexture = *unsafe { game_state() }.r_data.texturetranslation.offset((*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).bottomtexture as isize);
            if (*unsafe { game_state() }.r_bsp.linedef).flags as i32 & ML_DONTPEGBOTTOM != 0 {
                unsafe { game_state() }.r_segs.rw_bottomtexturemid = unsafe { game_state() }.r_segs.worldtop as fixed_t;
            } else {
                unsafe { game_state() }.r_segs.rw_bottomtexturemid = unsafe { game_state() }.r_segs.worldlow as fixed_t;
            }
        }
        unsafe { game_state() }.r_segs.rw_toptexturemid += (*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).rowoffset;
        unsafe { game_state() }.r_segs.rw_bottomtexturemid += (*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).rowoffset;
        if (*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).midtexture != 0 {
            unsafe { game_state() }.r_segs.maskedtexture = true;
            unsafe { game_state() }.r_segs.maskedtexturecol = unsafe { game_state() }.r_plane.lastopening.offset(-(unsafe { game_state() }.r_segs.rw_x as isize));
            (*unsafe { game_state() }.r_bsp.ds_p).maskedtexturecol = unsafe { game_state() }.r_segs.maskedtexturecol;
            unsafe { game_state() }.r_plane.lastopening = unsafe { game_state() }.r_plane.lastopening.offset((unsafe { game_state() }.r_segs.rw_stopx - unsafe { game_state() }.r_segs.rw_x) as isize);
        }
    }
    unsafe { game_state() }.r_segs.segtextured = (unsafe { game_state() }.r_segs.midtexture | unsafe { game_state() }.r_segs.toptexture | unsafe { game_state() }.r_segs.bottomtexture) != 0 || unsafe { game_state() }.r_segs.maskedtexture;
    if unsafe { game_state() }.r_segs.segtextured {
        offsetangle = unsafe { game_state() }.r_segs.rw_normalangle.wrapping_sub(unsafe { game_state() }.r_segs.rw_angle1 as angle_t);
        if offsetangle > ANG180 {
            offsetangle = offsetangle.wrapping_neg();
        }
        if offsetangle > ANG90 as angle_t {
            offsetangle = ANG90 as angle_t;
        }
        sineval = finesine[(offsetangle >> ANGLETOFINESHIFT) as usize];
        unsafe { game_state() }.r_segs.rw_offset = FixedMul(hyp, sineval);
        if unsafe { game_state() }.r_segs.rw_normalangle.wrapping_sub(unsafe { game_state() }.r_segs.rw_angle1 as angle_t) < ANG180 {
            unsafe { game_state() }.r_segs.rw_offset = -unsafe { game_state() }.r_segs.rw_offset;
        }
        unsafe { game_state() }.r_segs.rw_offset += (*unsafe { game_state() }.p_setup.side_mut(unsafe { game_state() }.r_bsp.sidedef)).textureoffset + (*unsafe { game_state() }.r_bsp.curline).offset;
        unsafe { game_state() }.r_segs.rw_centerangle = (ANG90 as angle_t)
            .wrapping_add(unsafe { game_state() }.r_main.viewangle)
            .wrapping_sub(unsafe { game_state() }.r_segs.rw_normalangle);
        if unsafe { game_state() }.r_main.fixedcolormap.is_null() {
            lightnum = ((*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).lightlevel as i32 >> LIGHTSEGSHIFT) + unsafe { game_state() }.r_main.extralight;
            if (*(*unsafe { game_state() }.r_bsp.curline).v1).y == (*(*unsafe { game_state() }.r_bsp.curline).v2).y {
                lightnum -= 1;
            } else if (*(*unsafe { game_state() }.r_bsp.curline).v1).x == (*(*unsafe { game_state() }.r_bsp.curline).v2).x {
                lightnum += 1;
            }
            if lightnum < 0 as i32 {
                unsafe { game_state() }.r_segs.walllights = &raw mut *(&raw mut unsafe { game_state() }.r_main.scalelight as *mut [*mut lighttable_t; 48])
                    .offset(0 as i32 as isize)
                    as *mut *mut lighttable_t;
            } else if lightnum >= LIGHTLEVELS {
                unsafe { game_state() }.r_segs.walllights = &raw mut *(&raw mut unsafe { game_state() }.r_main.scalelight as *mut [*mut lighttable_t; 48])
                    .offset((LIGHTLEVELS - 1 as i32) as isize)
                    as *mut *mut lighttable_t;
            } else {
                unsafe { game_state() }.r_segs.walllights = &raw mut *(&raw mut unsafe { game_state() }.r_main.scalelight as *mut [*mut lighttable_t; 48])
                    .offset(lightnum as isize)
                    as *mut *mut lighttable_t;
            }
        }
    }
    if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).floorheight >= unsafe { game_state() }.r_main.viewz {
        unsafe { game_state() }.r_segs.markfloor = false;
    }
    if (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingheight <= unsafe { game_state() }.r_main.viewz
        && (*unsafe { game_state() }.p_setup.sector_mut(unsafe { game_state() }.r_bsp.frontsector.unwrap())).ceilingpic as i32 != unsafe { game_state() }.r_sky.skyflatnum
    {
        unsafe { game_state() }.r_segs.markceiling = false;
    }
    unsafe { game_state() }.r_segs.worldtop >>= 4 as i32;
    unsafe { game_state() }.r_segs.worldbottom >>= 4 as i32;
    unsafe { game_state() }.r_segs.topstep = -FixedMul(unsafe { game_state() }.r_segs.rw_scalestep, unsafe { game_state() }.r_segs.worldtop as fixed_t);
    unsafe { game_state() }.r_segs.topfrac = (unsafe { game_state() }.r_main.centeryfrac >> 4 as i32) - FixedMul(unsafe { game_state() }.r_segs.worldtop as fixed_t, unsafe { game_state() }.r_segs.rw_scale);
    unsafe { game_state() }.r_segs.bottomstep = -FixedMul(unsafe { game_state() }.r_segs.rw_scalestep, unsafe { game_state() }.r_segs.worldbottom as fixed_t);
    unsafe { game_state() }.r_segs.bottomfrac = (unsafe { game_state() }.r_main.centeryfrac >> 4 as i32) - FixedMul(unsafe { game_state() }.r_segs.worldbottom as fixed_t, unsafe { game_state() }.r_segs.rw_scale);
    if !unsafe { game_state() }.r_bsp.backsector.is_none() {
        unsafe { game_state() }.r_segs.worldhigh >>= 4 as i32;
        unsafe { game_state() }.r_segs.worldlow >>= 4 as i32;
        if unsafe { game_state() }.r_segs.worldhigh < unsafe { game_state() }.r_segs.worldtop {
            unsafe { game_state() }.r_segs.pixhigh = (unsafe { game_state() }.r_main.centeryfrac >> 4 as i32) - FixedMul(unsafe { game_state() }.r_segs.worldhigh as fixed_t, unsafe { game_state() }.r_segs.rw_scale);
            unsafe { game_state() }.r_segs.pixhighstep = -FixedMul(unsafe { game_state() }.r_segs.rw_scalestep, unsafe { game_state() }.r_segs.worldhigh as fixed_t);
        }
        if unsafe { game_state() }.r_segs.worldlow > unsafe { game_state() }.r_segs.worldbottom {
            unsafe { game_state() }.r_segs.pixlow = (unsafe { game_state() }.r_main.centeryfrac >> 4 as i32) - FixedMul(unsafe { game_state() }.r_segs.worldlow as fixed_t, unsafe { game_state() }.r_segs.rw_scale);
            unsafe { game_state() }.r_segs.pixlowstep = -FixedMul(unsafe { game_state() }.r_segs.rw_scalestep, unsafe { game_state() }.r_segs.worldlow as fixed_t);
        }
    }
    if unsafe { game_state() }.r_segs.markceiling {
        unsafe { game_state() }.r_plane.ceilingplane = R_CheckPlane(unsafe { game_state() }.r_plane.ceilingplane, unsafe { game_state() }.r_segs.rw_x, unsafe { game_state() }.r_segs.rw_stopx - 1 as i32);
    }
    if unsafe { game_state() }.r_segs.markfloor {
        unsafe { game_state() }.r_plane.floorplane = R_CheckPlane(unsafe { game_state() }.r_plane.floorplane, unsafe { game_state() }.r_segs.rw_x, unsafe { game_state() }.r_segs.rw_stopx - 1 as i32);
    }
    R_RenderSegLoop();
    if ((*unsafe { game_state() }.r_bsp.ds_p).silhouette & SIL_TOP != 0 || unsafe { game_state() }.r_segs.maskedtexture) && (*unsafe { game_state() }.r_bsp.ds_p).sprtopclip.is_null() {
        memcpy(
            unsafe { game_state() }.r_plane.lastopening as *mut ::core::ffi::c_void,
            (&raw mut unsafe { game_state() }.r_plane.ceilingclip as *mut i16).offset(start as isize) as *const ::core::ffi::c_void,
            (2 as i32 * (unsafe { game_state() }.r_segs.rw_stopx - start)) as size_t,
        );
        (*unsafe { game_state() }.r_bsp.ds_p).sprtopclip = unsafe { game_state() }.r_plane.lastopening.offset(-(start as isize));
        unsafe { game_state() }.r_plane.lastopening = unsafe { game_state() }.r_plane.lastopening.offset((unsafe { game_state() }.r_segs.rw_stopx - start) as isize);
    }
    if ((*unsafe { game_state() }.r_bsp.ds_p).silhouette & SIL_BOTTOM != 0 || unsafe { game_state() }.r_segs.maskedtexture) && (*unsafe { game_state() }.r_bsp.ds_p).sprbottomclip.is_null() {
        memcpy(
            unsafe { game_state() }.r_plane.lastopening as *mut ::core::ffi::c_void,
            (&raw mut unsafe { game_state() }.r_plane.floorclip as *mut i16).offset(start as isize) as *const ::core::ffi::c_void,
            (2 as i32 * (unsafe { game_state() }.r_segs.rw_stopx - start)) as size_t,
        );
        (*unsafe { game_state() }.r_bsp.ds_p).sprbottomclip = unsafe { game_state() }.r_plane.lastopening.offset(-(start as isize));
        unsafe { game_state() }.r_plane.lastopening = unsafe { game_state() }.r_plane.lastopening.offset((unsafe { game_state() }.r_segs.rw_stopx - start) as isize);
    }
    if unsafe { game_state() }.r_segs.maskedtexture && (*unsafe { game_state() }.r_bsp.ds_p).silhouette & SIL_TOP == 0 {
        (*unsafe { game_state() }.r_bsp.ds_p).silhouette |= SIL_TOP;
        (*unsafe { game_state() }.r_bsp.ds_p).tsilheight = INT_MIN as fixed_t;
    }
    if unsafe { game_state() }.r_segs.maskedtexture && (*unsafe { game_state() }.r_bsp.ds_p).silhouette & SIL_BOTTOM == 0 {
        (*unsafe { game_state() }.r_bsp.ds_p).silhouette |= SIL_BOTTOM;
        (*unsafe { game_state() }.r_bsp.ds_p).bsilheight = INT_MAX as fixed_t;
    }
    unsafe { game_state() }.r_bsp.ds_p = unsafe { game_state() }.r_bsp.ds_p.offset(1);
}
pub const __SHRT_MAX__: i32 = 32767;
