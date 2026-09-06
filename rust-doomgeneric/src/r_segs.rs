use crate::src::r_data::column_t;
use crate::src::r_defs::drawseg_t;
use crate::src::i_system::I_Error;
use crate::src::r_main::R_PointToDist;
use crate::src::r_main::R_ScaleFromGlobalAngle;
use crate::src::r_bsp::curline;
use crate::src::r_bsp::sidedef;
use crate::src::r_bsp::linedef;
use crate::src::r_bsp::frontsector;
use crate::src::r_bsp::backsector;
use crate::src::r_plane::lastopening;
use crate::src::r_plane::floorclip;
use crate::src::r_plane::ceilingclip;
use crate::src::r_plane::R_CheckPlane;
use crate::src::r_things::negonearray;
use crate::src::r_things::mfloorclip;
use crate::src::r_things::mceilingclip;
use crate::src::r_things::spryscale;
use crate::src::r_things::sprtopscreen;
use crate::src::r_things::R_DrawMaskedColumn;
use crate::src::r_data::R_GetColumn;
use crate::src::r_data::textureheight;
use crate::src::r_data::texturetranslation;
use crate::src::r_main::centeryfrac;
use crate::src::r_main::xtoviewangle;
use crate::src::r_main::scalelight;
use crate::src::r_plane::floorplane;
use crate::src::r_plane::ceilingplane;
use crate::src::r_things::screenheightarray;
use crate::src::r_bsp::drawsegs;
use crate::src::r_bsp::ds_p;
use crate::src::r_draw::dc_colormap;
use crate::src::r_draw::dc_x;
use crate::src::r_draw::dc_yl;
use crate::src::r_draw::dc_yh;
use crate::src::r_draw::dc_iscale;
use crate::src::r_draw::dc_texturemid;
use crate::src::r_draw::dc_source;
use crate::src::r_main::fixedcolormap;
use crate::src::r_main::viewangle;
use crate::src::r_main::extralight;
use crate::src::r_main::colfunc;
use crate::src::tables::finetangent;
use crate::src::r_main::viewz;
use crate::src::r_draw::viewwidth;
use crate::src::r_draw::viewheight;
use crate::src::r_sky::skyflatnum;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::r_defs::lighttable_t;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use libc::memcpy;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG180;
use crate::src::tables::ANG90;
use crate::src::m_fixed::INT_MAX;
use crate::src::m_fixed::INT_MIN;
use crate::src::p_spec::ML_MAPPED;
use crate::src::r_main::MAXLIGHTSCALE;
use crate::src::r_main::LIGHTSCALESHIFT;
use crate::src::r_main::LIGHTSEGSHIFT;
use crate::src::r_main::LIGHTLEVELS;
use crate::src::m_fixed::FRACBITS;

pub const SHRT_MAX: i32 = __SHRT_MAX__;
pub const ML_DONTPEGTOP: i32 = 8 as i32;
pub const ML_DONTPEGBOTTOM: i32 = 16 as i32;
pub const SIL_BOTTOM: i32 = 1 as i32;
pub const SIL_TOP: i32 = 2 as i32;
pub const SIL_BOTH: i32 = 3 as i32;
pub const MAXDRAWSEGS: i32 = 256 as i32;
#[no_mangle]
pub static mut segtextured: bool = false;
#[no_mangle]
pub static mut markfloor: bool = false;
#[no_mangle]
pub static mut markceiling: bool = false;
#[no_mangle]
pub static mut maskedtexture: bool = false;
#[no_mangle]
pub static mut toptexture: i32 = 0;
#[no_mangle]
pub static mut bottomtexture: i32 = 0;
#[no_mangle]
pub static mut midtexture: i32 = 0;
pub static mut rw_normalangle: angle_t = 0;
pub static mut rw_angle1: i32 = 0;
#[no_mangle]
pub static mut rw_x: i32 = 0;
#[no_mangle]
pub static mut rw_stopx: i32 = 0;
#[no_mangle]
pub static mut rw_centerangle: angle_t = 0;
#[no_mangle]
pub static mut rw_offset: fixed_t = 0;
pub static mut rw_distance: fixed_t = 0;
#[no_mangle]
pub static mut rw_scale: fixed_t = 0;
#[no_mangle]
pub static mut rw_scalestep: fixed_t = 0;
#[no_mangle]
pub static mut rw_midtexturemid: fixed_t = 0;
#[no_mangle]
pub static mut rw_toptexturemid: fixed_t = 0;
#[no_mangle]
pub static mut rw_bottomtexturemid: fixed_t = 0;
#[no_mangle]
pub static mut worldtop: i32 = 0;
#[no_mangle]
pub static mut worldbottom: i32 = 0;
#[no_mangle]
pub static mut worldhigh: i32 = 0;
#[no_mangle]
pub static mut worldlow: i32 = 0;
#[no_mangle]
pub static mut pixhigh: fixed_t = 0;
#[no_mangle]
pub static mut pixlow: fixed_t = 0;
#[no_mangle]
pub static mut pixhighstep: fixed_t = 0;
#[no_mangle]
pub static mut pixlowstep: fixed_t = 0;
#[no_mangle]
pub static mut topfrac: fixed_t = 0;
#[no_mangle]
pub static mut topstep: fixed_t = 0;
#[no_mangle]
pub static mut bottomfrac: fixed_t = 0;
#[no_mangle]
pub static mut bottomstep: fixed_t = 0;
pub static mut walllights: *mut *mut lighttable_t = ::core::ptr::null::<
    *mut lighttable_t,
>() as *mut *mut lighttable_t;
#[no_mangle]
pub static mut maskedtexturecol: *mut i16 = ::core::ptr::null::<
    i16,
>() as *mut i16;
pub unsafe fn R_RenderMaskedSegRange(
    mut ds: *mut drawseg_t,
    mut x1: i32,
    mut x2: i32,
) {
    let mut index: u32 = 0;
    let mut col: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut lightnum: i32 = 0;
    let mut texnum: i32 = 0;
    curline = (*ds).curline;
    frontsector = (*curline).frontsector;
    backsector = (*curline).backsector;
    texnum = *texturetranslation.offset((*(*curline).sidedef).midtexture as isize);
    lightnum = ((*frontsector).lightlevel as i32 >> LIGHTSEGSHIFT)
        + extralight;
    if (*(*curline).v1).y == (*(*curline).v2).y {
        lightnum -= 1;
    } else if (*(*curline).v1).x == (*(*curline).v2).x {
        lightnum += 1;
    }
    if lightnum < 0 as i32 {
        walllights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset(0 as i32 as isize) as *mut *mut lighttable_t;
    } else if lightnum >= LIGHTLEVELS {
        walllights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset((LIGHTLEVELS - 1 as i32) as isize)
            as *mut *mut lighttable_t;
    } else {
        walllights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset(lightnum as isize) as *mut *mut lighttable_t;
    }
    maskedtexturecol = (*ds).maskedtexturecol;
    rw_scalestep = (*ds).scalestep;
    spryscale = (*ds).scale1 + (x1 as fixed_t - (*ds).x1 as fixed_t) * rw_scalestep;
    mfloorclip = (*ds).sprbottomclip;
    mceilingclip = (*ds).sprtopclip;
    if (*(*curline).linedef).flags as i32 & ML_DONTPEGBOTTOM != 0 {
        dc_texturemid = if (*frontsector).floorheight > (*backsector).floorheight {
            (*frontsector).floorheight
        } else {
            (*backsector).floorheight
        };
        dc_texturemid = dc_texturemid + *textureheight.offset(texnum as isize) - viewz;
    } else {
        dc_texturemid = if (*frontsector).ceilingheight < (*backsector).ceilingheight {
            (*frontsector).ceilingheight
        } else {
            (*backsector).ceilingheight
        };
        dc_texturemid = dc_texturemid - viewz;
    }
    dc_texturemid += (*(*curline).sidedef).rowoffset;
    if !fixedcolormap.is_null() {
        dc_colormap = fixedcolormap;
    }
    dc_x = x1;
    while dc_x <= x2 {
        if *maskedtexturecol.offset(dc_x as isize) as i32 != SHRT_MAX {
            if fixedcolormap.is_null() {
                index = (spryscale >> LIGHTSCALESHIFT) as u32;
                if index >= MAXLIGHTSCALE as u32 {
                    index = (MAXLIGHTSCALE - 1 as i32)
                        as u32;
                }
                dc_colormap = *walllights.offset(index as isize);
            }
            sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);
            dc_iscale = (0xffffffff as u32)
                .wrapping_div(spryscale as u32) as fixed_t;
            col = R_GetColumn(
                    texnum,
                    *maskedtexturecol.offset(dc_x as isize) as i32,
                )
                .offset(-(3 as i32 as isize)) as *mut column_t;
            R_DrawMaskedColumn(col);
            *maskedtexturecol.offset(dc_x as isize) = SHRT_MAX as i16;
        }
        spryscale += rw_scalestep;
        dc_x += 1;
    }
}
pub const HEIGHTBITS: i32 = 12 as i32;
pub const HEIGHTUNIT: i32 = (1 as i32) << HEIGHTBITS;
#[no_mangle]
pub unsafe extern "C" fn R_RenderSegLoop() {
    let mut angle: angle_t = 0;
    let mut index: u32 = 0;
    let mut yl: i32 = 0;
    let mut yh: i32 = 0;
    let mut mid: i32 = 0;
    let mut texturecolumn: fixed_t = 0;
    let mut top: i32 = 0;
    let mut bottom: i32 = 0;
    while rw_x < rw_stopx {
        yl = topfrac as i32 + HEIGHTUNIT - 1 as i32
            >> HEIGHTBITS;
        if yl
            < ceilingclip[rw_x as usize] as i32 + 1 as i32
        {
            yl = ceilingclip[rw_x as usize] as i32
                + 1 as i32;
        }
        if markceiling {
            top = ceilingclip[rw_x as usize] as i32
                + 1 as i32;
            bottom = yl - 1 as i32;
            if bottom >= floorclip[rw_x as usize] as i32 {
                bottom = floorclip[rw_x as usize] as i32
                    - 1 as i32;
            }
            if top <= bottom {
                (*ceilingplane).top[rw_x as usize] = top as byte;
                (*ceilingplane).bottom[rw_x as usize] = bottom as byte;
            }
        }
        yh = (bottomfrac >> HEIGHTBITS) as i32;
        if yh >= floorclip[rw_x as usize] as i32 {
            yh = floorclip[rw_x as usize] as i32
                - 1 as i32;
        }
        if markfloor {
            top = yh + 1 as i32;
            bottom = floorclip[rw_x as usize] as i32
                - 1 as i32;
            if top <= ceilingclip[rw_x as usize] as i32 {
                top = ceilingclip[rw_x as usize] as i32
                    + 1 as i32;
            }
            if top <= bottom {
                (*floorplane).top[rw_x as usize] = top as byte;
                (*floorplane).bottom[rw_x as usize] = bottom as byte;
            }
        }
        if segtextured {
            angle = rw_centerangle.wrapping_add(xtoviewangle[rw_x as usize])
                >> ANGLETOFINESHIFT;
            texturecolumn = rw_offset
                - FixedMul(finetangent[angle as usize], rw_distance);
            texturecolumn >>= FRACBITS;
            index = (rw_scale >> LIGHTSCALESHIFT) as u32;
            if index >= MAXLIGHTSCALE as u32 {
                index = (MAXLIGHTSCALE - 1 as i32) as u32;
            }
            dc_colormap = *walllights.offset(index as isize);
            dc_x = rw_x;
            dc_iscale = (0xffffffff as u32)
                .wrapping_div(rw_scale as u32) as fixed_t;
        } else {
            texturecolumn = 0 as i32 as fixed_t;
        }
        if midtexture != 0 {
            dc_yl = yl;
            dc_yh = yh;
            dc_texturemid = rw_midtexturemid;
            dc_source = R_GetColumn(midtexture, texturecolumn as i32);
            colfunc.expect("non-null function pointer")();
            ceilingclip[rw_x as usize] = viewheight as i16;
            floorclip[rw_x as usize] = -(1 as i32)
                as i16;
        } else {
            if toptexture != 0 {
                mid = (pixhigh >> HEIGHTBITS) as i32;
                pixhigh += pixhighstep;
                if mid >= floorclip[rw_x as usize] as i32 {
                    mid = floorclip[rw_x as usize] as i32
                        - 1 as i32;
                }
                if mid >= yl {
                    dc_yl = yl;
                    dc_yh = mid;
                    dc_texturemid = rw_toptexturemid;
                    dc_source = R_GetColumn(
                        toptexture,
                        texturecolumn as i32,
                    );
                    colfunc.expect("non-null function pointer")();
                    ceilingclip[rw_x as usize] = mid as i16;
                } else {
                    ceilingclip[rw_x as usize] = (yl - 1 as i32)
                        as i16;
                }
            } else if markceiling {
                ceilingclip[rw_x as usize] = (yl - 1 as i32)
                    as i16;
            }
            if bottomtexture != 0 {
                mid = pixlow as i32 + HEIGHTUNIT - 1 as i32
                    >> HEIGHTBITS;
                pixlow += pixlowstep;
                if mid <= ceilingclip[rw_x as usize] as i32 {
                    mid = ceilingclip[rw_x as usize] as i32
                        + 1 as i32;
                }
                if mid <= yh {
                    dc_yl = mid;
                    dc_yh = yh;
                    dc_texturemid = rw_bottomtexturemid;
                    dc_source = R_GetColumn(
                        bottomtexture,
                        texturecolumn as i32,
                    );
                    colfunc.expect("non-null function pointer")();
                    floorclip[rw_x as usize] = mid as i16;
                } else {
                    floorclip[rw_x as usize] = (yh + 1 as i32)
                        as i16;
                }
            } else if markfloor {
                floorclip[rw_x as usize] = (yh + 1 as i32)
                    as i16;
            }
            if maskedtexture {
                *maskedtexturecol.offset(rw_x as isize) = texturecolumn
                    as i16;
            }
        }
        rw_scale += rw_scalestep;
        topfrac += topstep;
        bottomfrac += bottomstep;
        rw_x += 1;
    }
}
pub unsafe fn R_StoreWallRange(
    mut start: i32,
    mut stop: i32,
) {
    let mut hyp: fixed_t = 0;
    let mut sineval: fixed_t = 0;
    let mut distangle: angle_t = 0;
    let mut offsetangle: angle_t = 0;
    let mut vtop: fixed_t = 0;
    let mut lightnum: i32 = 0;
    if ds_p
        == (&raw mut drawsegs as *mut drawseg_t).offset(MAXDRAWSEGS as isize)
            as *mut drawseg_t
    {
        return;
    }
    if start >= viewwidth || start > stop {
        I_Error(&format!("Bad R_RenderWallRange: {} to {}", start, stop));
    }
    sidedef = (*curline).sidedef;
    linedef = (*curline).linedef;
    (*linedef).flags = ((*linedef).flags as i32 | ML_MAPPED)
        as i16;
    rw_normalangle = (*curline).angle.wrapping_add(ANG90 as angle_t);
    offsetangle = (
        rw_normalangle.wrapping_sub(rw_angle1 as angle_t) as i32
    ).abs() as angle_t;
    if offsetangle > ANG90 as angle_t {
        offsetangle = ANG90 as angle_t;
    }
    distangle = (ANG90 as angle_t).wrapping_sub(offsetangle);
    hyp = R_PointToDist((*(*curline).v1).x, (*(*curline).v1).y);
    sineval = finesine[(distangle >> ANGLETOFINESHIFT) as usize];
    rw_distance = FixedMul(hyp, sineval);
    rw_x = start;
    (*ds_p).x1 = rw_x;
    (*ds_p).x2 = stop;
    (*ds_p).curline = curline;
    rw_stopx = stop + 1 as i32;
    rw_scale = R_ScaleFromGlobalAngle(
        viewangle.wrapping_add(xtoviewangle[start as usize]),
    );
    (*ds_p).scale1 = rw_scale;
    if stop > start {
        (*ds_p).scale2 = R_ScaleFromGlobalAngle(
            viewangle.wrapping_add(xtoviewangle[stop as usize]),
        );
        rw_scalestep = (((*ds_p).scale2 as i32
            - rw_scale as i32) / (stop - start)) as fixed_t;
        (*ds_p).scalestep = rw_scalestep;
    } else {
        (*ds_p).scale2 = (*ds_p).scale1;
    }
    worldtop = ((*frontsector).ceilingheight - viewz) as i32;
    worldbottom = ((*frontsector).floorheight - viewz) as i32;
    maskedtexture = false;
    bottomtexture = maskedtexture as i32;
    toptexture = bottomtexture;
    midtexture = toptexture;
    (*ds_p).maskedtexturecol = ::core::ptr::null_mut::<i16>();
    if backsector.is_null() {
        midtexture = *texturetranslation.offset((*sidedef).midtexture as isize);
        markceiling = true;
        markfloor = markceiling;
        if (*linedef).flags as i32 & ML_DONTPEGBOTTOM != 0 {
            vtop = (*frontsector).floorheight
                + *textureheight.offset((*sidedef).midtexture as isize);
            rw_midtexturemid = vtop - viewz;
        } else {
            rw_midtexturemid = worldtop as fixed_t;
        }
        rw_midtexturemid += (*sidedef).rowoffset;
        (*ds_p).silhouette = SIL_BOTH;
        (*ds_p).sprtopclip = &raw mut screenheightarray as *mut i16;
        (*ds_p).sprbottomclip = &raw mut negonearray as *mut i16;
        (*ds_p).bsilheight = INT_MAX as fixed_t;
        (*ds_p).tsilheight = INT_MIN as fixed_t;
    } else {
        (*ds_p).sprbottomclip = ::core::ptr::null_mut::<i16>();
        (*ds_p).sprtopclip = (*ds_p).sprbottomclip;
        (*ds_p).silhouette = 0 as i32;
        if (*frontsector).floorheight > (*backsector).floorheight {
            (*ds_p).silhouette = SIL_BOTTOM;
            (*ds_p).bsilheight = (*frontsector).floorheight;
        } else if (*backsector).floorheight > viewz {
            (*ds_p).silhouette = SIL_BOTTOM;
            (*ds_p).bsilheight = INT_MAX as fixed_t;
        }
        if (*frontsector).ceilingheight < (*backsector).ceilingheight {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = (*frontsector).ceilingheight;
        } else if (*backsector).ceilingheight < viewz {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = INT_MIN as fixed_t;
        }
        if (*backsector).ceilingheight <= (*frontsector).floorheight {
            (*ds_p).sprbottomclip = &raw mut negonearray as *mut i16;
            (*ds_p).bsilheight = INT_MAX as fixed_t;
            (*ds_p).silhouette |= SIL_BOTTOM;
        }
        if (*backsector).floorheight >= (*frontsector).ceilingheight {
            (*ds_p).sprtopclip = &raw mut screenheightarray as *mut i16;
            (*ds_p).tsilheight = INT_MIN as fixed_t;
            (*ds_p).silhouette |= SIL_TOP;
        }
        worldhigh = ((*backsector).ceilingheight - viewz) as i32;
        worldlow = ((*backsector).floorheight - viewz) as i32;
        if (*frontsector).ceilingpic as i32 == skyflatnum
            && (*backsector).ceilingpic as i32 == skyflatnum
        {
            worldtop = worldhigh;
        }
        if worldlow != worldbottom
            || (*backsector).floorpic as i32
                != (*frontsector).floorpic as i32
            || (*backsector).lightlevel as i32
                != (*frontsector).lightlevel as i32
        {
            markfloor = true;
        } else {
            markfloor = false;
        }
        if worldhigh != worldtop
            || (*backsector).ceilingpic as i32
                != (*frontsector).ceilingpic as i32
            || (*backsector).lightlevel as i32
                != (*frontsector).lightlevel as i32
        {
            markceiling = true;
        } else {
            markceiling = false;
        }
        if (*backsector).ceilingheight <= (*frontsector).floorheight
            || (*backsector).floorheight >= (*frontsector).ceilingheight
        {
            markfloor = true;
            markceiling = markfloor;
        }
        if worldhigh < worldtop {
            toptexture = *texturetranslation.offset((*sidedef).toptexture as isize);
            if (*linedef).flags as i32 & ML_DONTPEGTOP != 0 {
                rw_toptexturemid = worldtop as fixed_t;
            } else {
                vtop = (*backsector).ceilingheight
                    + *textureheight.offset((*sidedef).toptexture as isize);
                rw_toptexturemid = vtop - viewz;
            }
        }
        if worldlow > worldbottom {
            bottomtexture = *texturetranslation
                .offset((*sidedef).bottomtexture as isize);
            if (*linedef).flags as i32 & ML_DONTPEGBOTTOM != 0 {
                rw_bottomtexturemid = worldtop as fixed_t;
            } else {
                rw_bottomtexturemid = worldlow as fixed_t;
            }
        }
        rw_toptexturemid += (*sidedef).rowoffset;
        rw_bottomtexturemid += (*sidedef).rowoffset;
        if (*sidedef).midtexture != 0 {
            maskedtexture = true;
            maskedtexturecol = lastopening.offset(-(rw_x as isize));
            (*ds_p).maskedtexturecol = maskedtexturecol;
            lastopening = lastopening.offset((rw_stopx - rw_x) as isize);
        }
    }
    segtextured = (midtexture | toptexture | bottomtexture) != 0 || maskedtexture;
    if segtextured {
        offsetangle = rw_normalangle.wrapping_sub(rw_angle1 as angle_t);
        if offsetangle > ANG180 {
            offsetangle = offsetangle.wrapping_neg();
        }
        if offsetangle > ANG90 as angle_t {
            offsetangle = ANG90 as angle_t;
        }
        sineval = finesine[(offsetangle >> ANGLETOFINESHIFT) as usize];
        rw_offset = FixedMul(hyp, sineval);
        if rw_normalangle.wrapping_sub(rw_angle1 as angle_t) < ANG180 {
            rw_offset = -rw_offset;
        }
        rw_offset += (*sidedef).textureoffset + (*curline).offset;
        rw_centerangle = (ANG90 as angle_t)
            .wrapping_add(viewangle)
            .wrapping_sub(rw_normalangle);
        if fixedcolormap.is_null() {
            lightnum = ((*frontsector).lightlevel as i32 >> LIGHTSEGSHIFT)
                + extralight;
            if (*(*curline).v1).y == (*(*curline).v2).y {
                lightnum -= 1;
            } else if (*(*curline).v1).x == (*(*curline).v2).x {
                lightnum += 1;
            }
            if lightnum < 0 as i32 {
                walllights = &raw mut *(&raw mut scalelight
                    as *mut [*mut lighttable_t; 48])
                    .offset(0 as i32 as isize) as *mut *mut lighttable_t;
            } else if lightnum >= LIGHTLEVELS {
                walllights = &raw mut *(&raw mut scalelight
                    as *mut [*mut lighttable_t; 48])
                    .offset((LIGHTLEVELS - 1 as i32) as isize)
                    as *mut *mut lighttable_t;
            } else {
                walllights = &raw mut *(&raw mut scalelight
                    as *mut [*mut lighttable_t; 48])
                    .offset(lightnum as isize) as *mut *mut lighttable_t;
            }
        }
    }
    if (*frontsector).floorheight >= viewz {
        markfloor = false;
    }
    if (*frontsector).ceilingheight <= viewz
        && (*frontsector).ceilingpic as i32 != skyflatnum
    {
        markceiling = false;
    }
    worldtop >>= 4 as i32;
    worldbottom >>= 4 as i32;
    topstep = -FixedMul(rw_scalestep, worldtop as fixed_t);
    topfrac = (centeryfrac >> 4 as i32)
        - FixedMul(worldtop as fixed_t, rw_scale);
    bottomstep = -FixedMul(rw_scalestep, worldbottom as fixed_t);
    bottomfrac = (centeryfrac >> 4 as i32)
        - FixedMul(worldbottom as fixed_t, rw_scale);
    if !backsector.is_null() {
        worldhigh >>= 4 as i32;
        worldlow >>= 4 as i32;
        if worldhigh < worldtop {
            pixhigh = (centeryfrac >> 4 as i32)
                - FixedMul(worldhigh as fixed_t, rw_scale);
            pixhighstep = -FixedMul(rw_scalestep, worldhigh as fixed_t);
        }
        if worldlow > worldbottom {
            pixlow = (centeryfrac >> 4 as i32)
                - FixedMul(worldlow as fixed_t, rw_scale);
            pixlowstep = -FixedMul(rw_scalestep, worldlow as fixed_t);
        }
    }
    if markceiling {
        ceilingplane = R_CheckPlane(
            ceilingplane,
            rw_x,
            rw_stopx - 1 as i32,
        );
    }
    if markfloor {
        floorplane = R_CheckPlane(floorplane, rw_x, rw_stopx - 1 as i32);
    }
    R_RenderSegLoop();
    if ((*ds_p).silhouette & SIL_TOP != 0 || maskedtexture)
        && (*ds_p).sprtopclip.is_null()
    {
        memcpy(
            lastopening as *mut ::core::ffi::c_void,
            (&raw mut ceilingclip as *mut i16).offset(start as isize)
                as *const ::core::ffi::c_void,
            (2 as i32 * (rw_stopx - start)) as size_t,
        );
        (*ds_p).sprtopclip = lastopening.offset(-(start as isize));
        lastopening = lastopening.offset((rw_stopx - start) as isize);
    }
    if ((*ds_p).silhouette & SIL_BOTTOM != 0 || maskedtexture)
        && (*ds_p).sprbottomclip.is_null()
    {
        memcpy(
            lastopening as *mut ::core::ffi::c_void,
            (&raw mut floorclip as *mut i16).offset(start as isize)
                as *const ::core::ffi::c_void,
            (2 as i32 * (rw_stopx - start)) as size_t,
        );
        (*ds_p).sprbottomclip = lastopening.offset(-(start as isize));
        lastopening = lastopening.offset((rw_stopx - start) as isize);
    }
    if maskedtexture && (*ds_p).silhouette & SIL_TOP == 0 {
        (*ds_p).silhouette |= SIL_TOP;
        (*ds_p).tsilheight = INT_MIN as fixed_t;
    }
    if maskedtexture && (*ds_p).silhouette & SIL_BOTTOM == 0 {
        (*ds_p).silhouette |= SIL_BOTTOM;
        (*ds_p).bsilheight = INT_MAX as fixed_t;
    }
    ds_p = ds_p.offset(1);
}
pub const __SHRT_MAX__: i32 = 32767 as i32;
