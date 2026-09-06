use crate::src::d_player::pw_invisibility;
use crate::src::d_player::NUMPSPRITES;
use crate::src::doomdef::boolean;
use crate::src::doomdef::false_0;
use crate::src::doomdef::true_0;
use crate::src::doomdef::NULL;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::doomstat::modifiedgame;
use crate::src::hu_lib::patch_t;
use crate::src::i_system::I_Error;
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FixedDiv;
use crate::src::m_fixed::FixedMul;
use crate::src::m_fixed::FRACBITS;
use crate::src::m_fixed::FRACUNIT;
use crate::src::m_fixed::INT_MAX;
use crate::src::p_mobj::sector_t;
use crate::src::p_mobj::{mobj_t, pspdef_t};
use crate::src::p_mobj::{MF_SHADOW, MF_TRANSLATION, MF_TRANSSHIFT};
use crate::src::r_bsp::drawsegs;
use crate::src::r_bsp::ds_p;
use crate::src::r_data::colormaps;
use crate::src::r_data::column_t;
use crate::src::r_data::firstspritelump;
use crate::src::r_data::lastspritelump;
use crate::src::r_data::spriteoffset;
use crate::src::r_data::spritetopoffset;
use crate::src::r_data::spritewidth;
use crate::src::r_defs::lighttable_t;
use crate::src::r_defs::{drawseg_t, spritedef_t, spriteframe_t};
use crate::src::r_draw::dc_colormap;
use crate::src::r_draw::dc_iscale;
use crate::src::r_draw::dc_source;
use crate::src::r_draw::dc_texturemid;
use crate::src::r_draw::dc_translation;
use crate::src::r_draw::dc_x;
use crate::src::r_draw::dc_yh;
use crate::src::r_draw::dc_yl;
use crate::src::r_draw::translationtables;
use crate::src::r_draw::viewheight;
use crate::src::r_draw::viewwidth;
use crate::src::r_main::basecolfunc;
use crate::src::r_main::centerxfrac;
use crate::src::r_main::centeryfrac;
use crate::src::r_main::colfunc;
use crate::src::r_main::detailshift;
use crate::src::r_main::extralight;
use crate::src::r_main::fixedcolormap;
use crate::src::r_main::fuzzcolfunc;
use crate::src::r_main::projection;
use crate::src::r_main::scalelight;
use crate::src::r_main::transcolfunc;
use crate::src::r_main::validcount;
use crate::src::r_main::viewangleoffset;
use crate::src::r_main::viewcos;
use crate::src::r_main::viewplayer;
use crate::src::r_main::viewsin;
use crate::src::r_main::viewx;
use crate::src::r_main::viewy;
use crate::src::r_main::viewz;
use crate::src::r_main::R_PointOnSegSide;
use crate::src::r_main::R_PointToAngle;
use crate::src::r_main::LIGHTLEVELS;
use crate::src::r_main::LIGHTSCALESHIFT;
use crate::src::r_main::LIGHTSEGSHIFT;
use crate::src::r_main::MAXLIGHTSCALE;
use crate::src::r_segs::R_RenderMaskedSegRange;
use crate::src::r_segs::SIL_BOTTOM;
use crate::src::r_segs::SIL_TOP;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::tables::angle_t;
use crate::src::tables::ANG45;
use crate::src::w_wad::lumpinfo;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::w_wad::{wad_name8_to_string, W_GetNumForName};
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use libc::strncasecmp;
use libc::{memcpy, memset};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vissprite_s {
    pub prev: *mut vissprite_s,
    pub next: *mut vissprite_s,
    pub x1: i32,
    pub x2: i32,
    pub gx: fixed_t,
    pub gy: fixed_t,
    pub gz: fixed_t,
    pub gzt: fixed_t,
    pub startfrac: fixed_t,
    pub scale: fixed_t,
    pub xiscale: fixed_t,
    pub texturemid: fixed_t,
    pub patch: i32,
    pub colormap: *mut lighttable_t,
    pub mobjflags: i32,
}
pub type vissprite_t = vissprite_s;
pub const FF_FULLBRIGHT: i32 = 0x8000;
pub const FF_FRAMEMASK: i32 = 0x7fff;
pub const MAXVISSPRITES: i32 = 128;
pub const MINZ: i32 = FRACUNIT * 4 as i32;
pub const BASEYCENTER: i32 = 100;
pub static mut pspritescale: fixed_t = 0;
pub static mut pspriteiscale: fixed_t = 0;
#[no_mangle]
pub static mut spritelights: *mut *mut lighttable_t =
    ::core::ptr::null::<*mut lighttable_t>() as *mut *mut lighttable_t;
pub static mut negonearray: [i16; 320] = [0; 320];
pub static mut screenheightarray: [i16; 320] = [0; 320];
pub static mut sprites: *mut spritedef_t = ::core::ptr::null::<spritedef_t>() as *mut spritedef_t;
pub static mut numsprites: i32 = 0;
#[no_mangle]
pub static mut sprtemp: [spriteframe_t; 29] = [spriteframe_t {
    rotate: 0,
    lump: [0; 8],
    flip: [0; 8],
}; 29];
#[no_mangle]
pub static mut maxframe: i32 = 0;
#[no_mangle]
pub static mut spritename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
pub unsafe fn R_InstallSpriteLump(
    mut lump: i32,
    mut frame: u32,
    mut rotation: u32,
    mut flipped: bool,
) {
    let mut r: i32 = 0;
    if frame >= 29 as u32 || rotation > 8 as u32 {
        I_Error(&format!(
            "R_InstallSpriteLump: Bad frame characters in lump {}",
            lump
        ));
    }
    if frame as i32 > maxframe {
        maxframe = frame as i32;
    }
    if rotation == 0 as u32 {
        if sprtemp[frame as usize].rotate == false_0 as boolean {
            I_Error(&format!(
                "R_InitSprites: Sprite {} frame {} has multip rot=0 lump",
                ::std::ffi::CStr::from_ptr(spritename).to_str().unwrap(),
                ('A' as i32 as u32).wrapping_add(frame) as u8 as char,
            ));
        }
        if sprtemp[frame as usize].rotate == true_0 as boolean {
            I_Error(&format!(
                "R_InitSprites: Sprite {} frame {} has rotations and a rot=0 lump",
                ::std::ffi::CStr::from_ptr(spritename).to_str().unwrap(),
                ('A' as i32 as u32).wrapping_add(frame) as u8 as char,
            ));
        }
        sprtemp[frame as usize].rotate = false_0 as boolean;
        r = 0 as i32;
        while r < 8 as i32 {
            sprtemp[frame as usize].lump[r as usize] = (lump - firstspritelump) as i16;
            sprtemp[frame as usize].flip[r as usize] = flipped as byte;
            r += 1;
        }
        return;
    }
    if sprtemp[frame as usize].rotate == false_0 as boolean {
        I_Error(&format!(
            "R_InitSprites: Sprite {} frame {} has rotations and a rot=0 lump",
            ::std::ffi::CStr::from_ptr(spritename).to_str().unwrap(),
            ('A' as i32 as u32).wrapping_add(frame) as u8 as char,
        ));
    }
    sprtemp[frame as usize].rotate = true_0 as boolean;
    rotation = rotation.wrapping_sub(1);
    if sprtemp[frame as usize].lump[rotation as usize] as i32 != -(1 as i32) {
        I_Error(&format!(
            "R_InitSprites: Sprite {} : {} : {} has two lumps mapped to it",
            ::std::ffi::CStr::from_ptr(spritename).to_str().unwrap(),
            ('A' as i32 as u32).wrapping_add(frame) as u8 as char,
            ('1' as i32 as u32).wrapping_add(rotation) as u8 as char,
        ));
    }
    sprtemp[frame as usize].lump[rotation as usize] = (lump - firstspritelump) as i16;
    sprtemp[frame as usize].flip[rotation as usize] = flipped as byte;
}
pub unsafe fn R_InitSpriteDefs(mut namelist: *mut *mut ::core::ffi::c_char) {
    let mut check: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    let mut frame: i32 = 0;
    let mut rotation: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut patched: i32 = 0;
    check = namelist;
    while !(*check).is_null() {
        check = check.offset(1);
    }
    numsprites = check.offset_from(namelist) as i64 as i32;
    if numsprites == 0 {
        return;
    }
    sprites = Z_Malloc(
        (numsprites as usize).wrapping_mul(::core::mem::size_of::<spritedef_t>() as usize) as i32,
        PU_STATIC as i32,
        NULL,
    ) as *mut spritedef_t;
    start = firstspritelump - 1 as i32;
    end = lastspritelump + 1 as i32;
    i = 0 as i32;
    while i < numsprites {
        spritename = *namelist.offset(i as isize);
        memset(
            &raw mut sprtemp as *mut spriteframe_t as *mut ::core::ffi::c_void,
            -(1 as i32),
            ::core::mem::size_of::<[spriteframe_t; 29]>() as size_t,
        );
        maxframe = -(1 as i32);
        l = start + 1 as i32;
        while l < end {
            if strncasecmp(
                &raw mut (*lumpinfo.offset(l as isize)).name as *mut ::core::ffi::c_char,
                spritename,
                4 as size_t,
            ) == 0
            {
                frame = (*lumpinfo.offset(l as isize)).name[4 as i32 as usize] as i32 - 'A' as i32;
                rotation =
                    (*lumpinfo.offset(l as isize)).name[5 as i32 as usize] as i32 - '0' as i32;
                if modifiedgame {
                    patched = W_GetNumForName(&wad_name8_to_string(
                        &raw const (*lumpinfo.offset(l as isize)).name
                            as *const ::core::ffi::c_char,
                    ));
                } else {
                    patched = l;
                }
                R_InstallSpriteLump(patched, frame as u32, rotation as u32, false);
                if (*lumpinfo.offset(l as isize)).name[6 as i32 as usize] != 0 {
                    frame =
                        (*lumpinfo.offset(l as isize)).name[6 as i32 as usize] as i32 - 'A' as i32;
                    rotation =
                        (*lumpinfo.offset(l as isize)).name[7 as i32 as usize] as i32 - '0' as i32;
                    R_InstallSpriteLump(l, frame as u32, rotation as u32, true);
                }
            }
            l += 1;
        }
        if maxframe == -(1 as i32) {
            (*sprites.offset(i as isize)).numframes = 0 as i32;
        } else {
            maxframe += 1;
            frame = 0 as i32;
            while frame < maxframe {
                match sprtemp[frame as usize].rotate as i32 {
                    -1 => {
                        I_Error(&format!(
                            "R_InitSprites: No patches found for {} frame {}",
                            ::std::ffi::CStr::from_ptr(spritename).to_str().unwrap(),
                            (frame + 'A' as i32) as u8 as char,
                        ));
                    }
                    1 => {
                        rotation = 0 as i32;
                        while rotation < 8 as i32 {
                            if sprtemp[frame as usize].lump[rotation as usize] as i32 == -(1 as i32)
                            {
                                I_Error(&format!(
                                    "R_InitSprites: Sprite {} frame {} is missing rotations",
                                    ::std::ffi::CStr::from_ptr(spritename).to_str().unwrap(),
                                    (frame + 'A' as i32) as u8 as char,
                                ));
                            }
                            rotation += 1;
                        }
                    }
                    0 | _ => {}
                }
                frame += 1;
            }
            (*sprites.offset(i as isize)).numframes = maxframe;
            let ref mut fresh1 = (*sprites.offset(i as isize)).spriteframes;
            *fresh1 = Z_Malloc(
                (maxframe as usize).wrapping_mul(::core::mem::size_of::<spriteframe_t>() as usize)
                    as i32,
                PU_STATIC as i32,
                NULL,
            ) as *mut spriteframe_t;
            memcpy(
                (*sprites.offset(i as isize)).spriteframes as *mut ::core::ffi::c_void,
                &raw mut sprtemp as *mut spriteframe_t as *const ::core::ffi::c_void,
                (maxframe as size_t)
                    .wrapping_mul(::core::mem::size_of::<spriteframe_t>() as size_t),
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub static mut vissprites: [vissprite_t; 128] = [vissprite_s {
    prev: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
    next: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
    mobjflags: 0,
}; 128];
#[no_mangle]
pub static mut vissprite_p: *mut vissprite_t =
    ::core::ptr::null::<vissprite_t>() as *mut vissprite_t;
#[no_mangle]
pub static mut newvissprite: i32 = 0;
pub unsafe fn R_InitSprites(mut namelist: *mut *mut ::core::ffi::c_char) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < SCREENWIDTH {
        negonearray[i as usize] = -(1 as i32) as i16;
        i += 1;
    }
    R_InitSpriteDefs(namelist);
}
pub unsafe fn R_ClearSprites() {
    vissprite_p = &raw mut vissprites as *mut vissprite_t;
}
#[no_mangle]
pub static mut overflowsprite: vissprite_t = vissprite_s {
    prev: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
    next: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
    mobjflags: 0,
};
pub unsafe fn R_NewVisSprite() -> *mut vissprite_t {
    if vissprite_p
        == (&raw mut vissprites as *mut vissprite_t).offset(MAXVISSPRITES as isize)
            as *mut vissprite_t
    {
        return &raw mut overflowsprite;
    }
    vissprite_p = vissprite_p.offset(1);
    return vissprite_p.offset(-(1 as i32 as isize));
}
pub static mut mfloorclip: *mut i16 = ::core::ptr::null::<i16>() as *mut i16;
pub static mut mceilingclip: *mut i16 = ::core::ptr::null::<i16>() as *mut i16;
pub static mut spryscale: fixed_t = 0;
pub static mut sprtopscreen: fixed_t = 0;
pub unsafe fn R_DrawMaskedColumn(mut column: *mut column_t) {
    let mut topscreen: i32 = 0;
    let mut bottomscreen: i32 = 0;
    let mut basetexturemid: fixed_t = 0;
    basetexturemid = dc_texturemid;
    while (*column).topdelta as i32 != 0xff as i32 {
        topscreen = sprtopscreen as i32 + spryscale as i32 * (*column).topdelta as i32;
        bottomscreen = topscreen + spryscale as i32 * (*column).length as i32;
        dc_yl = topscreen + FRACUNIT - 1 as i32 >> FRACBITS;
        dc_yh = bottomscreen - 1 as i32 >> FRACBITS;
        if dc_yh >= *mfloorclip.offset(dc_x as isize) as i32 {
            dc_yh = *mfloorclip.offset(dc_x as isize) as i32 - 1 as i32;
        }
        if dc_yl <= *mceilingclip.offset(dc_x as isize) as i32 {
            dc_yl = *mceilingclip.offset(dc_x as isize) as i32 + 1 as i32;
        }
        if dc_yl <= dc_yh {
            dc_source = (column as *mut byte).offset(3 as i32 as isize);
            dc_texturemid =
                (basetexturemid as i32 - (((*column).topdelta as i32) << FRACBITS)) as fixed_t;
            colfunc.expect("non-null function pointer")();
        }
        column = (column as *mut byte)
            .offset((*column).length as i32 as isize)
            .offset(4 as i32 as isize) as *mut column_t;
    }
    dc_texturemid = basetexturemid;
}
pub unsafe fn R_DrawVisSprite(mut vis: *mut vissprite_t, mut x1: i32, mut x2: i32) {
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut texturecolumn: i32 = 0;
    let mut frac: fixed_t = 0;
    let mut patch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    patch = W_CacheLumpNum((*vis).patch + firstspritelump, PU_CACHE as i32) as *mut patch_t;
    dc_colormap = (*vis).colormap;
    if dc_colormap.is_null() {
        colfunc = fuzzcolfunc;
    } else if (*vis).mobjflags & MF_TRANSLATION as i32 != 0 {
        colfunc = transcolfunc;
        dc_translation = translationtables.offset(-(256 as i32 as isize)).offset(
            (((*vis).mobjflags & MF_TRANSLATION as i32) >> MF_TRANSSHIFT as i32 - 8 as i32)
                as isize,
        );
    }
    dc_iscale = (((*vis).xiscale as i32).abs() >> detailshift) as fixed_t;
    dc_texturemid = (*vis).texturemid;
    frac = (*vis).startfrac;
    spryscale = (*vis).scale;
    sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);
    dc_x = (*vis).x1;
    while dc_x <= (*vis).x2 {
        texturecolumn = (frac >> FRACBITS) as i32;
        if texturecolumn < 0 as i32 || texturecolumn >= (*patch).width as i32 {
            I_Error("R_DrawSpriteRange: bad texturecolumn");
        }
        column = (patch as *mut byte).offset(
            *(&raw const (*patch).columnofs as *const i32).offset(texturecolumn as isize) as isize,
        ) as *mut column_t;
        R_DrawMaskedColumn(column);
        dc_x += 1;
        frac += (*vis).xiscale;
    }
    colfunc = basecolfunc;
}
pub unsafe fn R_ProjectSprite(mut thing: *mut mobj_t) {
    let mut tr_x: fixed_t = 0;
    let mut tr_y: fixed_t = 0;
    let mut gxt: fixed_t = 0;
    let mut gyt: fixed_t = 0;
    let mut tx: fixed_t = 0;
    let mut tz: fixed_t = 0;
    let mut xscale: fixed_t = 0;
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut sprdef: *mut spritedef_t = ::core::ptr::null_mut::<spritedef_t>();
    let mut sprframe: *mut spriteframe_t = ::core::ptr::null_mut::<spriteframe_t>();
    let mut lump: i32 = 0;
    let mut rot: u32 = 0;
    let mut flip: bool = false;
    let mut index: i32 = 0;
    let mut vis: *mut vissprite_t = ::core::ptr::null_mut::<vissprite_t>();
    let mut ang: angle_t = 0;
    let mut iscale: fixed_t = 0;
    tr_x = (*thing).x - viewx;
    tr_y = (*thing).y - viewy;
    gxt = FixedMul(tr_x, viewcos);
    gyt = -FixedMul(tr_y, viewsin);
    tz = gxt - gyt;
    if tz < MINZ {
        return;
    }
    xscale = FixedDiv(projection, tz);
    gxt = -FixedMul(tr_x, viewsin);
    gyt = FixedMul(tr_y, viewcos);
    tx = -(gyt + gxt);
    if (tx as i32).abs() > tz << 2 as i32 {
        return;
    }
    if (*thing).sprite as u32 >= numsprites as u32 {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite number {} ",
            (*thing).sprite as u32,
        ));
    }
    sprdef = sprites.offset((*thing).sprite as isize) as *mut spritedef_t;
    if (*thing).frame & FF_FRAMEMASK >= (*sprdef).numframes {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite frame {} : {} ",
            (*thing).sprite as u32,
            (*thing).frame,
        ));
    }
    sprframe = (*sprdef)
        .spriteframes
        .offset(((*thing).frame & FF_FRAMEMASK) as isize) as *mut spriteframe_t;
    if (*sprframe).rotate != 0 {
        ang = R_PointToAngle((*thing).x, (*thing).y);
        rot = (ang as u32)
            .wrapping_sub((*thing).angle as u32)
            .wrapping_add(((ANG45 / 2 as i32) as u32).wrapping_mul(9 as u32))
            >> 29 as i32;
        lump = (*sprframe).lump[rot as usize] as i32;
        flip = (*sprframe).flip[rot as usize] != 0;
    } else {
        lump = (*sprframe).lump[0 as i32 as usize] as i32;
        flip = (*sprframe).flip[0 as i32 as usize] != 0;
    }
    tx -= *spriteoffset.offset(lump as isize);
    x1 = (centerxfrac + FixedMul(tx, xscale) >> FRACBITS) as i32;
    if x1 > viewwidth {
        return;
    }
    tx += *spritewidth.offset(lump as isize);
    x2 = (centerxfrac as i32 + FixedMul(tx, xscale) as i32 >> FRACBITS) - 1 as i32;
    if x2 < 0 as i32 {
        return;
    }
    vis = R_NewVisSprite();
    (*vis).mobjflags = (*thing).flags;
    (*vis).scale = xscale << detailshift;
    (*vis).gx = (*thing).x;
    (*vis).gy = (*thing).y;
    (*vis).gz = (*thing).z;
    (*vis).gzt = (*thing).z + *spritetopoffset.offset(lump as isize);
    (*vis).texturemid = (*vis).gzt - viewz;
    (*vis).x1 = if x1 < 0 as i32 { 0 as i32 } else { x1 };
    (*vis).x2 = if x2 >= viewwidth {
        viewwidth - 1 as i32
    } else {
        x2
    };
    iscale = FixedDiv(FRACUNIT, xscale);
    if flip {
        (*vis).startfrac = (*spritewidth.offset(lump as isize) as i32 - 1 as i32) as fixed_t;
        (*vis).xiscale = -iscale;
    } else {
        (*vis).startfrac = 0 as i32 as fixed_t;
        (*vis).xiscale = iscale;
    }
    if (*vis).x1 > x1 {
        (*vis).startfrac += (*vis).xiscale as i32 * ((*vis).x1 - x1);
    }
    (*vis).patch = lump;
    if (*thing).flags & MF_SHADOW as i32 != 0 {
        (*vis).colormap = ::core::ptr::null_mut::<lighttable_t>();
    } else if !fixedcolormap.is_null() {
        (*vis).colormap = fixedcolormap;
    } else if (*thing).frame & FF_FULLBRIGHT != 0 {
        (*vis).colormap = colormaps;
    } else {
        index = (xscale >> LIGHTSCALESHIFT - detailshift) as i32;
        if index >= MAXLIGHTSCALE {
            index = MAXLIGHTSCALE - 1 as i32;
        }
        (*vis).colormap = *spritelights.offset(index as isize);
    };
}
pub unsafe fn R_AddSprites(mut sec: *mut sector_t) {
    let mut thing: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut lightnum: i32 = 0;
    if (*sec).validcount == validcount {
        return;
    }
    (*sec).validcount = validcount;
    lightnum = ((*sec).lightlevel as i32 >> LIGHTSEGSHIFT) + extralight;
    if lightnum < 0 as i32 {
        spritelights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset(0 as i32 as isize) as *mut *mut lighttable_t;
    } else if lightnum >= LIGHTLEVELS {
        spritelights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset((LIGHTLEVELS - 1 as i32) as isize)
            as *mut *mut lighttable_t;
    } else {
        spritelights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset(lightnum as isize) as *mut *mut lighttable_t;
    }
    thing = (*sec).thinglist;
    while !thing.is_null() {
        R_ProjectSprite(thing);
        thing = (*thing).snext as *mut mobj_t;
    }
}
pub unsafe fn R_DrawPSprite(mut psp: *mut pspdef_t) {
    let mut tx: fixed_t = 0;
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut sprdef: *mut spritedef_t = ::core::ptr::null_mut::<spritedef_t>();
    let mut sprframe: *mut spriteframe_t = ::core::ptr::null_mut::<spriteframe_t>();
    let mut lump: i32 = 0;
    let mut flip: bool = false;
    let mut vis: *mut vissprite_t = ::core::ptr::null_mut::<vissprite_t>();
    let mut avis: vissprite_t = vissprite_s {
        prev: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
        next: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
        x1: 0,
        x2: 0,
        gx: 0,
        gy: 0,
        gz: 0,
        gzt: 0,
        startfrac: 0,
        scale: 0,
        xiscale: 0,
        texturemid: 0,
        patch: 0,
        colormap: ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
        mobjflags: 0,
    };
    if (*(*psp).state).sprite as u32 >= numsprites as u32 {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite number {} ",
            (*(*psp).state).sprite as u32,
        ));
    }
    sprdef = sprites.offset((*(*psp).state).sprite as isize) as *mut spritedef_t;
    if (*(*psp).state).frame & FF_FRAMEMASK >= (*sprdef).numframes {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite frame {} : {} ",
            (*(*psp).state).sprite as u32,
            (*(*psp).state).frame,
        ));
    }
    sprframe = (*sprdef)
        .spriteframes
        .offset(((*(*psp).state).frame & FF_FRAMEMASK) as isize)
        as *mut spriteframe_t;
    lump = (*sprframe).lump[0 as i32 as usize] as i32;
    flip = (*sprframe).flip[0 as i32 as usize] != 0;
    tx = ((*psp).sx as i32 - 160 as i32 * FRACUNIT) as fixed_t;
    tx -= *spriteoffset.offset(lump as isize);
    x1 = (centerxfrac + FixedMul(tx, pspritescale) >> FRACBITS) as i32;
    if x1 > viewwidth {
        return;
    }
    tx += *spritewidth.offset(lump as isize);
    x2 = (centerxfrac as i32 + FixedMul(tx, pspritescale) as i32 >> FRACBITS) - 1 as i32;
    if x2 < 0 as i32 {
        return;
    }
    vis = &raw mut avis;
    (*vis).mobjflags = 0 as i32;
    (*vis).texturemid = (BASEYCENTER << FRACBITS) + FRACUNIT / 2 as fixed_t
        - ((*psp).sy - *spritetopoffset.offset(lump as isize));
    (*vis).x1 = if x1 < 0 as i32 { 0 as i32 } else { x1 };
    (*vis).x2 = if x2 >= viewwidth {
        viewwidth - 1 as i32
    } else {
        x2
    };
    (*vis).scale = pspritescale << detailshift;
    if flip {
        (*vis).xiscale = -pspriteiscale;
        (*vis).startfrac = (*spritewidth.offset(lump as isize) as i32 - 1 as i32) as fixed_t;
    } else {
        (*vis).xiscale = pspriteiscale;
        (*vis).startfrac = 0 as i32 as fixed_t;
    }
    if (*vis).x1 > x1 {
        (*vis).startfrac += (*vis).xiscale as i32 * ((*vis).x1 - x1);
    }
    (*vis).patch = lump;
    if (*viewplayer).powers[pw_invisibility as i32 as usize] > 4 as i32 * 32 as i32
        || (*viewplayer).powers[pw_invisibility as i32 as usize] & 8 as i32 != 0
    {
        (*vis).colormap = ::core::ptr::null_mut::<lighttable_t>();
    } else if !fixedcolormap.is_null() {
        (*vis).colormap = fixedcolormap;
    } else if (*(*psp).state).frame & FF_FULLBRIGHT != 0 {
        (*vis).colormap = colormaps;
    } else {
        (*vis).colormap = *spritelights.offset((MAXLIGHTSCALE - 1 as i32) as isize);
    }
    R_DrawVisSprite(vis, (*vis).x1, (*vis).x2);
}
pub unsafe fn R_DrawPlayerSprites() {
    let mut i: i32 = 0;
    let mut lightnum: i32 = 0;
    let mut psp: *mut pspdef_t = ::core::ptr::null_mut::<pspdef_t>();
    lightnum = ((*(*(*(*viewplayer).mo).subsector).sector).lightlevel as i32 >> LIGHTSEGSHIFT)
        + extralight;
    if lightnum < 0 as i32 {
        spritelights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset(0 as i32 as isize) as *mut *mut lighttable_t;
    } else if lightnum >= LIGHTLEVELS {
        spritelights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset((LIGHTLEVELS - 1 as i32) as isize)
            as *mut *mut lighttable_t;
    } else {
        spritelights = &raw mut *(&raw mut scalelight as *mut [*mut lighttable_t; 48])
            .offset(lightnum as isize) as *mut *mut lighttable_t;
    }
    mfloorclip = &raw mut screenheightarray as *mut i16;
    mceilingclip = &raw mut negonearray as *mut i16;
    i = 0 as i32;
    psp = &raw mut (*viewplayer).psprites as *mut pspdef_t;
    while i < NUMPSPRITES as i32 {
        if !(*psp).state.is_null() {
            R_DrawPSprite(psp);
        }
        i += 1;
        psp = psp.offset(1);
    }
}
#[no_mangle]
pub static mut vsprsortedhead: vissprite_t = vissprite_s {
    prev: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
    next: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
    mobjflags: 0,
};
pub unsafe fn R_SortVisSprites() {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut ds: *mut vissprite_t = ::core::ptr::null_mut::<vissprite_t>();
    let mut best: *mut vissprite_t = ::core::ptr::null_mut::<vissprite_t>();
    let mut unsorted: vissprite_t = vissprite_s {
        prev: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
        next: ::core::ptr::null::<vissprite_s>() as *mut vissprite_s,
        x1: 0,
        x2: 0,
        gx: 0,
        gy: 0,
        gz: 0,
        gzt: 0,
        startfrac: 0,
        scale: 0,
        xiscale: 0,
        texturemid: 0,
        patch: 0,
        colormap: ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
        mobjflags: 0,
    };
    let mut bestscale: fixed_t = 0;
    count = vissprite_p.offset_from(&raw mut vissprites as *mut vissprite_t) as i64 as i32;
    unsorted.prev = &raw mut unsorted as *mut vissprite_s;
    unsorted.next = unsorted.prev;
    if count == 0 {
        return;
    }
    ds = &raw mut vissprites as *mut vissprite_t;
    while ds < vissprite_p {
        (*ds).next = ds.offset(1 as i32 as isize) as *mut vissprite_s;
        (*ds).prev = ds.offset(-(1 as i32 as isize)) as *mut vissprite_s;
        ds = ds.offset(1);
    }
    vissprites[0 as i32 as usize].prev = &raw mut unsorted as *mut vissprite_s;
    unsorted.next = (&raw mut vissprites as *mut vissprite_t).offset(0 as i32 as isize)
        as *mut vissprite_t as *mut vissprite_s;
    let ref mut fresh0 = (*vissprite_p.offset(-(1 as i32 as isize))).next;
    *fresh0 = &raw mut unsorted as *mut vissprite_s;
    unsorted.prev = vissprite_p.offset(-(1 as i32 as isize)) as *mut vissprite_s;
    vsprsortedhead.prev = &raw mut vsprsortedhead as *mut vissprite_s;
    vsprsortedhead.next = vsprsortedhead.prev;
    i = 0 as i32;
    while i < count {
        bestscale = INT_MAX as fixed_t;
        best = unsorted.next as *mut vissprite_t;
        ds = unsorted.next as *mut vissprite_t;
        while ds != &raw mut unsorted {
            if (*ds).scale < bestscale {
                bestscale = (*ds).scale;
                best = ds;
            }
            ds = (*ds).next as *mut vissprite_t;
        }
        (*(*best).next).prev = (*best).prev;
        (*(*best).prev).next = (*best).next;
        (*best).next = &raw mut vsprsortedhead as *mut vissprite_s;
        (*best).prev = vsprsortedhead.prev;
        (*vsprsortedhead.prev).next = best as *mut vissprite_s;
        vsprsortedhead.prev = best as *mut vissprite_s;
        i += 1;
    }
}
static mut clipbot: [i16; 320] = [0; 320];
static mut cliptop: [i16; 320] = [0; 320];
pub unsafe fn R_DrawSprite(mut spr: *mut vissprite_t) {
    let mut ds: *mut drawseg_t = ::core::ptr::null_mut::<drawseg_t>();
    let mut x: i32 = 0;
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    let mut scale: fixed_t = 0;
    let mut lowscale: fixed_t = 0;
    let mut silhouette: i32 = 0;
    x = (*spr).x1;
    while x <= (*spr).x2 {
        cliptop[x as usize] = -(2 as i32) as i16;
        clipbot[x as usize] = cliptop[x as usize];
        x += 1;
    }
    ds = ds_p.offset(-(1 as i32 as isize));
    while ds >= &raw mut drawsegs as *mut drawseg_t {
        if !((*ds).x1 > (*spr).x2
            || (*ds).x2 < (*spr).x1
            || (*ds).silhouette == 0 && (*ds).maskedtexturecol.is_null())
        {
            r1 = if (*ds).x1 < (*spr).x1 {
                (*spr).x1
            } else {
                (*ds).x1
            };
            r2 = if (*ds).x2 > (*spr).x2 {
                (*spr).x2
            } else {
                (*ds).x2
            };
            if (*ds).scale1 > (*ds).scale2 {
                lowscale = (*ds).scale2;
                scale = (*ds).scale1;
            } else {
                lowscale = (*ds).scale1;
                scale = (*ds).scale2;
            }
            if scale < (*spr).scale
                || lowscale < (*spr).scale
                    && R_PointOnSegSide((*spr).gx, (*spr).gy, (*ds).curline) == 0
            {
                if !(*ds).maskedtexturecol.is_null() {
                    R_RenderMaskedSegRange(ds, r1, r2);
                }
            } else {
                silhouette = (*ds).silhouette;
                if (*spr).gz >= (*ds).bsilheight {
                    silhouette &= !SIL_BOTTOM;
                }
                if (*spr).gzt <= (*ds).tsilheight {
                    silhouette &= !SIL_TOP;
                }
                if silhouette == 1 as i32 {
                    x = r1;
                    while x <= r2 {
                        if clipbot[x as usize] as i32 == -(2 as i32) {
                            clipbot[x as usize] = *(*ds).sprbottomclip.offset(x as isize);
                        }
                        x += 1;
                    }
                } else if silhouette == 2 as i32 {
                    x = r1;
                    while x <= r2 {
                        if cliptop[x as usize] as i32 == -(2 as i32) {
                            cliptop[x as usize] = *(*ds).sprtopclip.offset(x as isize);
                        }
                        x += 1;
                    }
                } else if silhouette == 3 as i32 {
                    x = r1;
                    while x <= r2 {
                        if clipbot[x as usize] as i32 == -(2 as i32) {
                            clipbot[x as usize] = *(*ds).sprbottomclip.offset(x as isize);
                        }
                        if cliptop[x as usize] as i32 == -(2 as i32) {
                            cliptop[x as usize] = *(*ds).sprtopclip.offset(x as isize);
                        }
                        x += 1;
                    }
                }
            }
        }
        ds = ds.offset(-1);
    }
    x = (*spr).x1;
    while x <= (*spr).x2 {
        if clipbot[x as usize] as i32 == -(2 as i32) {
            clipbot[x as usize] = viewheight as i16;
        }
        if cliptop[x as usize] as i32 == -(2 as i32) {
            cliptop[x as usize] = -(1 as i32) as i16;
        }
        x += 1;
    }
    mfloorclip = &raw mut clipbot as *mut i16;
    mceilingclip = &raw mut cliptop as *mut i16;
    R_DrawVisSprite(spr, (*spr).x1, (*spr).x2);
}
pub unsafe fn R_DrawMasked() {
    let mut spr: *mut vissprite_t = ::core::ptr::null_mut::<vissprite_t>();
    let mut ds: *mut drawseg_t = ::core::ptr::null_mut::<drawseg_t>();
    R_SortVisSprites();
    if vissprite_p > &raw mut vissprites as *mut vissprite_t {
        spr = vsprsortedhead.next as *mut vissprite_t;
        while spr != &raw mut vsprsortedhead {
            R_DrawSprite(spr);
            spr = (*spr).next as *mut vissprite_t;
        }
    }
    ds = ds_p.offset(-(1 as i32 as isize));
    while ds >= &raw mut drawsegs as *mut drawseg_t {
        if !(*ds).maskedtexturecol.is_null() {
            R_RenderMaskedSegRange(ds, (*ds).x1, (*ds).x2);
        }
        ds = ds.offset(-1);
    }
    if viewangleoffset == 0 {
        R_DrawPlayerSprites();
    }
}
