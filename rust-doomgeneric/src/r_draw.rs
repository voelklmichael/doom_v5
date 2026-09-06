use crate::src::hu_lib::patch_t;
use crate::src::i_system::I_Error;
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName};
use crate::src::r_main::centery;
use crate::src::v_video::V_UseBuffer;
use crate::src::v_video::V_RestoreBuffer;
use crate::src::r_data::colormaps;
use crate::src::v_video::V_MarkRect;
use crate::src::i_video::I_VideoBuffer;
use crate::src::doomstat::gamemode;
use crate::src::v_video::V_DrawPatch;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use crate::src::d_mode::commercial;
use crate::src::m_fixed::fixed_t;
use crate::src::r_defs::lighttable_t;
use libc::memcpy;

pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type byte = uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const FRACBITS: i32 = 16 as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const SBARHEIGHT: i32 = 32 as i32;
#[no_mangle]
pub static mut viewimage: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
pub static mut viewwidth: i32 = 0;
pub static mut scaledviewwidth: i32 = 0;
pub static mut viewheight: i32 = 0;
pub static mut viewwindowx: i32 = 0;
pub static mut viewwindowy: i32 = 0;
#[no_mangle]
pub static mut ylookup: [*mut byte; 832] = [::core::ptr::null::<byte>()
    as *mut byte; 832];
#[no_mangle]
pub static mut columnofs: [i32; 1120] = [0; 1120];
#[no_mangle]
pub static mut translations: [[byte; 256]; 3] = [[0; 256]; 3];
static mut background_buffer: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
pub static mut dc_colormap: *mut lighttable_t = ::core::ptr::null::<lighttable_t>()
    as *mut lighttable_t;
pub static mut dc_x: i32 = 0;
pub static mut dc_yl: i32 = 0;
pub static mut dc_yh: i32 = 0;
pub static mut dc_iscale: fixed_t = 0;
pub static mut dc_texturemid: fixed_t = 0;
pub static mut dc_source: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub static mut dccount: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn R_DrawColumn() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl;
    if count < 0 as i32 {
        return;
    }
    if dc_x as u32 >= SCREENWIDTH as u32
        || dc_yl < 0 as i32 || dc_yh >= SCREENHEIGHT
    {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }
    dest = ylookup[dc_yl as usize].offset(columnofs[dc_x as usize] as isize);
    fracstep = dc_iscale;
    frac = dc_texturemid + (dc_yl as fixed_t - centery as fixed_t) * fracstep;
    loop {
        *dest = *dc_colormap
            .offset(
                *dc_source
                    .offset(
                        (frac as i32 >> FRACBITS
                            & 127 as i32) as isize,
                    ) as isize,
            ) as byte;
        dest = dest.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawColumnLow() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    let mut x: i32 = 0;
    count = dc_yh - dc_yl;
    if count < 0 as i32 {
        return;
    }
    if dc_x as u32 >= SCREENWIDTH as u32
        || dc_yl < 0 as i32 || dc_yh >= SCREENHEIGHT
    {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }
    x = dc_x << 1 as i32;
    dest = ylookup[dc_yl as usize].offset(columnofs[x as usize] as isize);
    dest2 = ylookup[dc_yl as usize]
        .offset(columnofs[(x + 1 as i32) as usize] as isize);
    fracstep = dc_iscale;
    frac = dc_texturemid + (dc_yl as fixed_t - centery as fixed_t) * fracstep;
    loop {
        *dest = *dc_colormap
            .offset(
                *dc_source
                    .offset(
                        (frac as i32 >> FRACBITS
                            & 127 as i32) as isize,
                    ) as isize,
            ) as byte;
        *dest2 = *dest;
        dest = dest.offset(SCREENWIDTH as isize);
        dest2 = dest2.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 != 0) {
            break;
        }
    };
}
pub const FUZZTABLE: i32 = 50 as i32;
pub const FUZZOFF: i32 = 320 as i32;
#[no_mangle]
pub static mut fuzzoffset: [i32; 50] = [
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
    FUZZOFF,
    -FUZZOFF,
    FUZZOFF,
];
#[no_mangle]
pub static mut fuzzpos: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn R_DrawFuzzColumn() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    if dc_yl == 0 {
        dc_yl = 1 as i32;
    }
    if dc_yh == viewheight - 1 as i32 {
        dc_yh = viewheight - 2 as i32;
    }
    count = dc_yh - dc_yl;
    if count < 0 as i32 {
        return;
    }
    if dc_x as u32 >= SCREENWIDTH as u32
        || dc_yl < 0 as i32 || dc_yh >= SCREENHEIGHT
    {
        I_Error(&format!("R_DrawFuzzColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }
    dest = ylookup[dc_yl as usize].offset(columnofs[dc_x as usize] as isize);
    fracstep = dc_iscale;
    frac = dc_texturemid + (dc_yl as fixed_t - centery as fixed_t) * fracstep;
    loop {
        *dest = *colormaps
            .offset(
                (6 as i32 * 256 as i32
                    + *dest.offset(fuzzoffset[fuzzpos as usize] as isize)
                        as i32) as isize,
            ) as byte;
        fuzzpos += 1;
        if fuzzpos == FUZZTABLE {
            fuzzpos = 0 as i32;
        }
        dest = dest.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh2 = count;
        count = count - 1;
        if !(fresh2 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawFuzzColumnLow() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    let mut x: i32 = 0;
    if dc_yl == 0 {
        dc_yl = 1 as i32;
    }
    if dc_yh == viewheight - 1 as i32 {
        dc_yh = viewheight - 2 as i32;
    }
    count = dc_yh - dc_yl;
    if count < 0 as i32 {
        return;
    }
    x = dc_x << 1 as i32;
    if x as u32 >= SCREENWIDTH as u32
        || dc_yl < 0 as i32 || dc_yh >= SCREENHEIGHT
    {
        I_Error(&format!("R_DrawFuzzColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }
    dest = ylookup[dc_yl as usize].offset(columnofs[x as usize] as isize);
    dest2 = ylookup[dc_yl as usize]
        .offset(columnofs[(x + 1 as i32) as usize] as isize);
    fracstep = dc_iscale;
    frac = dc_texturemid + (dc_yl as fixed_t - centery as fixed_t) * fracstep;
    loop {
        *dest = *colormaps
            .offset(
                (6 as i32 * 256 as i32
                    + *dest.offset(fuzzoffset[fuzzpos as usize] as isize)
                        as i32) as isize,
            ) as byte;
        *dest2 = *colormaps
            .offset(
                (6 as i32 * 256 as i32
                    + *dest2.offset(fuzzoffset[fuzzpos as usize] as isize)
                        as i32) as isize,
            ) as byte;
        fuzzpos += 1;
        if fuzzpos == FUZZTABLE {
            fuzzpos = 0 as i32;
        }
        dest = dest.offset(SCREENWIDTH as isize);
        dest2 = dest2.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh3 = count;
        count = count - 1;
        if !(fresh3 != 0) {
            break;
        }
    };
}
pub static mut dc_translation: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
pub static mut translationtables: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslatedColumn() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl;
    if count < 0 as i32 {
        return;
    }
    if dc_x as u32 >= SCREENWIDTH as u32
        || dc_yl < 0 as i32 || dc_yh >= SCREENHEIGHT
    {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }
    dest = ylookup[dc_yl as usize].offset(columnofs[dc_x as usize] as isize);
    fracstep = dc_iscale;
    frac = dc_texturemid + (dc_yl as fixed_t - centery as fixed_t) * fracstep;
    loop {
        *dest = *dc_colormap
            .offset(
                *dc_translation
                    .offset(*dc_source.offset((frac >> FRACBITS) as isize) as isize)
                    as isize,
            ) as byte;
        dest = dest.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh4 = count;
        count = count - 1;
        if !(fresh4 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslatedColumnLow() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    let mut x: i32 = 0;
    count = dc_yh - dc_yl;
    if count < 0 as i32 {
        return;
    }
    x = dc_x << 1 as i32;
    if x as u32 >= SCREENWIDTH as u32
        || dc_yl < 0 as i32 || dc_yh >= SCREENHEIGHT
    {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, x));
    }
    dest = ylookup[dc_yl as usize].offset(columnofs[x as usize] as isize);
    dest2 = ylookup[dc_yl as usize]
        .offset(columnofs[(x + 1 as i32) as usize] as isize);
    fracstep = dc_iscale;
    frac = dc_texturemid + (dc_yl as fixed_t - centery as fixed_t) * fracstep;
    loop {
        *dest = *dc_colormap
            .offset(
                *dc_translation
                    .offset(*dc_source.offset((frac >> FRACBITS) as isize) as isize)
                    as isize,
            ) as byte;
        *dest2 = *dc_colormap
            .offset(
                *dc_translation
                    .offset(*dc_source.offset((frac >> FRACBITS) as isize) as isize)
                    as isize,
            ) as byte;
        dest = dest.offset(SCREENWIDTH as isize);
        dest2 = dest2.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh5 = count;
        count = count - 1;
        if !(fresh5 != 0) {
            break;
        }
    };
}
pub unsafe fn R_InitTranslationTables() {
    let mut i: i32 = 0;
    translationtables = Z_Malloc(
        256 as i32 * 3 as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut byte;
    i = 0 as i32;
    while i < 256 as i32 {
        if i >= 0x70 as i32 && i <= 0x7f as i32 {
            *translationtables.offset(i as isize) = (0x60 as i32
                + (i & 0xf as i32)) as byte;
            *translationtables.offset((i + 256 as i32) as isize) = (0x40
                as i32 + (i & 0xf as i32)) as byte;
            *translationtables.offset((i + 512 as i32) as isize) = (0x20
                as i32 + (i & 0xf as i32)) as byte;
        } else {
            let ref mut fresh11 = *translationtables
                .offset((i + 512 as i32) as isize);
            *fresh11 = i as byte;
            let ref mut fresh12 = *translationtables
                .offset((i + 256 as i32) as isize);
            *fresh12 = *fresh11;
            *translationtables.offset(i as isize) = *fresh12;
        }
        i += 1;
    }
}
pub static mut ds_y: i32 = 0;
pub static mut ds_x1: i32 = 0;
pub static mut ds_x2: i32 = 0;
pub static mut ds_colormap: *mut lighttable_t = ::core::ptr::null::<lighttable_t>()
    as *mut lighttable_t;
pub static mut ds_xfrac: fixed_t = 0;
pub static mut ds_yfrac: fixed_t = 0;
pub static mut ds_xstep: fixed_t = 0;
pub static mut ds_ystep: fixed_t = 0;
pub static mut ds_source: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub static mut dscount: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpan() {
    let mut position: u32 = 0;
    let mut step: u32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut count: i32 = 0;
    let mut spot: i32 = 0;
    let mut xtemp: u32 = 0;
    let mut ytemp: u32 = 0;
    if ds_x2 < ds_x1 || ds_x1 < 0 as i32 || ds_x2 >= SCREENWIDTH
        || ds_y as u32 > SCREENHEIGHT as u32
    {
        I_Error(&format!("R_DrawSpan: {} to {} at {}", ds_x1, ds_x2, ds_y));
    }
    position = (ds_xfrac << 10 as i32) as u32
        & 0xffff0000 as u32
        | (ds_yfrac as i32 >> 6 as i32
            & 0xffff as i32) as u32;
    step = (ds_xstep << 10 as i32) as u32
        & 0xffff0000 as u32
        | (ds_ystep as i32 >> 6 as i32
            & 0xffff as i32) as u32;
    dest = ylookup[ds_y as usize].offset(columnofs[ds_x1 as usize] as isize);
    count = ds_x2 - ds_x1;
    loop {
        ytemp = position >> 4 as i32 & 0xfc0 as u32;
        xtemp = position >> 26 as i32;
        spot = (xtemp | ytemp) as i32;
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = *ds_colormap.offset(*ds_source.offset(spot as isize) as isize) as byte;
        position = position.wrapping_add(step);
        let fresh7 = count;
        count = count - 1;
        if !(fresh7 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanLow() {
    let mut position: u32 = 0;
    let mut step: u32 = 0;
    let mut xtemp: u32 = 0;
    let mut ytemp: u32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut count: i32 = 0;
    let mut spot: i32 = 0;
    if ds_x2 < ds_x1 || ds_x1 < 0 as i32 || ds_x2 >= SCREENWIDTH
        || ds_y as u32 > SCREENHEIGHT as u32
    {
        I_Error(&format!("R_DrawSpan: {} to {} at {}", ds_x1, ds_x2, ds_y));
    }
    position = (ds_xfrac << 10 as i32) as u32
        & 0xffff0000 as u32
        | (ds_yfrac as i32 >> 6 as i32
            & 0xffff as i32) as u32;
    step = (ds_xstep << 10 as i32) as u32
        & 0xffff0000 as u32
        | (ds_ystep as i32 >> 6 as i32
            & 0xffff as i32) as u32;
    count = ds_x2 - ds_x1;
    ds_x1 <<= 1 as i32;
    ds_x2 <<= 1 as i32;
    dest = ylookup[ds_y as usize].offset(columnofs[ds_x1 as usize] as isize);
    loop {
        ytemp = position >> 4 as i32 & 0xfc0 as u32;
        xtemp = position >> 26 as i32;
        spot = (xtemp | ytemp) as i32;
        let fresh8 = dest;
        dest = dest.offset(1);
        *fresh8 = *ds_colormap.offset(*ds_source.offset(spot as isize) as isize) as byte;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = *ds_colormap.offset(*ds_source.offset(spot as isize) as isize) as byte;
        position = position.wrapping_add(step);
        let fresh10 = count;
        count = count - 1;
        if !(fresh10 != 0) {
            break;
        }
    };
}
pub unsafe fn R_InitBuffer(
    mut width: i32,
    mut height: i32,
) {
    let mut i: i32 = 0;
    viewwindowx = SCREENWIDTH - width >> 1 as i32;
    i = 0 as i32;
    while i < width {
        columnofs[i as usize] = viewwindowx + i;
        i += 1;
    }
    if width == SCREENWIDTH {
        viewwindowy = 0 as i32;
    } else {
        viewwindowy = SCREENHEIGHT - SBARHEIGHT - height >> 1 as i32;
    }
    i = 0 as i32;
    while i < height {
        ylookup[i as usize] = I_VideoBuffer
            .offset(((i + viewwindowy) * SCREENWIDTH) as isize);
        i += 1;
    }
}
pub unsafe fn R_FillBackScreen() {
    let mut src: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut patch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    let mut name1: *mut ::core::ffi::c_char = b"FLOOR7_2\0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut name2: *mut ::core::ffi::c_char = b"GRNROCK\0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if scaledviewwidth == SCREENWIDTH {
        if !background_buffer.is_null() {
            Z_Free(background_buffer as *mut ::core::ffi::c_void);
            background_buffer = ::core::ptr::null_mut::<byte>();
        }
        return;
    }
    if background_buffer.is_null() {
        background_buffer = Z_Malloc(
            SCREENWIDTH * (SCREENHEIGHT - SBARHEIGHT),
            PU_STATIC as i32,
            NULL,
        ) as *mut byte;
    }
    if gamemode as u32
        == commercial as i32 as u32
    {
        name = name2;
    } else {
        name = name1;
    }
    src = W_CacheLumpName(
        &wad_name8_to_string(name),
        PU_CACHE as i32,
    ) as *mut byte;
    dest = background_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT - SBARHEIGHT {
        x = 0 as i32;
        while x < SCREENWIDTH / 64 as i32 {
            memcpy(
                dest as *mut ::core::ffi::c_void,
                src
                    .offset(
                        ((y & 63 as i32) << 6 as i32)
                            as isize,
                    ) as *const ::core::ffi::c_void,
                64 as size_t,
            );
            dest = dest.offset(64 as i32 as isize);
            x += 1;
        }
        if SCREENWIDTH & 63 as i32 != 0 {
            memcpy(
                dest as *mut ::core::ffi::c_void,
                src
                    .offset(
                        ((y & 63 as i32) << 6 as i32)
                            as isize,
                    ) as *const ::core::ffi::c_void,
                (SCREENWIDTH & 63 as i32) as size_t,
            );
            dest = dest.offset((SCREENWIDTH & 63 as i32) as isize);
        }
        y += 1;
    }
    V_UseBuffer(background_buffer);
    patch = W_CacheLumpName("brdr_t",
        PU_CACHE as i32,
    ) as *mut patch_t;
    x = 0 as i32;
    while x < scaledviewwidth {
        V_DrawPatch(viewwindowx + x, viewwindowy - 8 as i32, patch);
        x += 8 as i32;
    }
    patch = W_CacheLumpName("brdr_b",
        PU_CACHE as i32,
    ) as *mut patch_t;
    x = 0 as i32;
    while x < scaledviewwidth {
        V_DrawPatch(viewwindowx + x, viewwindowy + viewheight, patch);
        x += 8 as i32;
    }
    patch = W_CacheLumpName("brdr_l",
        PU_CACHE as i32,
    ) as *mut patch_t;
    y = 0 as i32;
    while y < viewheight {
        V_DrawPatch(viewwindowx - 8 as i32, viewwindowy + y, patch);
        y += 8 as i32;
    }
    patch = W_CacheLumpName("brdr_r",
        PU_CACHE as i32,
    ) as *mut patch_t;
    y = 0 as i32;
    while y < viewheight {
        V_DrawPatch(viewwindowx + scaledviewwidth, viewwindowy + y, patch);
        y += 8 as i32;
    }
    V_DrawPatch(
        viewwindowx - 8 as i32,
        viewwindowy - 8 as i32,
        W_CacheLumpName("brdr_tl",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatch(
        viewwindowx + scaledviewwidth,
        viewwindowy - 8 as i32,
        W_CacheLumpName("brdr_tr",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatch(
        viewwindowx - 8 as i32,
        viewwindowy + viewheight,
        W_CacheLumpName("brdr_bl",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatch(
        viewwindowx + scaledviewwidth,
        viewwindowy + viewheight,
        W_CacheLumpName("brdr_br",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_RestoreBuffer();
}
pub unsafe fn R_VideoErase(
    mut ofs: u32,
    mut count: i32,
) {
    if !background_buffer.is_null() {
        memcpy(
            I_VideoBuffer.offset(ofs as isize) as *mut ::core::ffi::c_void,
            background_buffer.offset(ofs as isize) as *const ::core::ffi::c_void,
            count as size_t,
        );
    }
}
pub unsafe fn R_DrawViewBorder() {
    let mut top: i32 = 0;
    let mut side: i32 = 0;
    let mut ofs: i32 = 0;
    let mut i: i32 = 0;
    if scaledviewwidth == SCREENWIDTH {
        return;
    }
    top = (SCREENHEIGHT - SBARHEIGHT - viewheight) / 2 as i32;
    side = (SCREENWIDTH - scaledviewwidth) / 2 as i32;
    R_VideoErase(0 as u32, top * SCREENWIDTH + side);
    ofs = (viewheight + top) * SCREENWIDTH - side;
    R_VideoErase(ofs as u32, top * SCREENWIDTH + side);
    ofs = top * SCREENWIDTH + SCREENWIDTH - side;
    side <<= 1 as i32;
    i = 1 as i32;
    while i < viewheight {
        R_VideoErase(ofs as u32, side);
        ofs += SCREENWIDTH;
        i += 1;
    }
    V_MarkRect(
        0 as i32,
        0 as i32,
        SCREENWIDTH,
        SCREENHEIGHT - SBARHEIGHT,
    );
}
