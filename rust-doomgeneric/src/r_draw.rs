use crate::src::d_mode::commercial;
use crate::src::doomdef::NULL;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::game_state::game_state;
use crate::src::hu_lib::patch_t;
use crate::src::i_system::I_Error;
use crate::src::i_video::I_VideoBuffer;
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FRACBITS;
use crate::src::r_defs::lighttable_t;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::v_video::V_DrawPatch;
use crate::src::v_video::V_MarkRect;
use crate::src::v_video::V_RestoreBuffer;
use crate::src::v_video::V_UseBuffer;
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName};
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use libc::memcpy;

pub struct RDrawState {
    pub viewimage: *mut byte,
    pub viewwidth: i32,
    pub scaledviewwidth: i32,
    pub viewheight: i32,
    pub viewwindowx: i32,
    pub viewwindowy: i32,
    pub ylookup: [*mut byte; 832],
    pub columnofs: [i32; 1120],
    pub background_buffer: *mut byte,
    pub dc_colormap: *mut lighttable_t,
    pub dc_x: i32,
    pub dc_yl: i32,
    pub dc_yh: i32,
    pub dc_iscale: fixed_t,
    pub dc_texturemid: fixed_t,
    pub dc_source: *mut byte,
    pub dccount: i32,
    pub fuzzpos: i32,
    pub dc_translation: *mut byte,
    pub translationtables: *mut byte,
    pub ds_y: i32,
    pub ds_x1: i32,
    pub ds_x2: i32,
    pub ds_colormap: *mut lighttable_t,
    pub ds_xfrac: fixed_t,
    pub ds_yfrac: fixed_t,
    pub ds_xstep: fixed_t,
    pub ds_ystep: fixed_t,
    pub ds_source: *mut byte,
    pub dscount: i32,
}

impl RDrawState {
    pub const fn new() -> Self {
        RDrawState {
            viewimage: ::core::ptr::null::<byte>() as *mut byte,
            viewwidth: 0,
            scaledviewwidth: 0,
            viewheight: 0,
            viewwindowx: 0,
            viewwindowy: 0,
            ylookup: [::core::ptr::null::<byte>() as *mut byte; 832],
            columnofs: [0; 1120],
            background_buffer: ::core::ptr::null::<byte>() as *mut byte,
            dc_colormap: 
        ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
            dc_x: 0,
            dc_yl: 0,
            dc_yh: 0,
            dc_iscale: 0,
            dc_texturemid: 0,
            dc_source: ::core::ptr::null::<byte>() as *mut byte,
            dccount: 0,
            fuzzpos: 0,
            dc_translation: ::core::ptr::null::<byte>() as *mut byte,
            translationtables: ::core::ptr::null::<byte>() as *mut byte,
            ds_y: 0,
            ds_x1: 0,
            ds_x2: 0,
            ds_colormap: 
        ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
            ds_xfrac: 0,
            ds_yfrac: 0,
            ds_xstep: 0,
            ds_ystep: 0,
            ds_source: ::core::ptr::null::<byte>() as *mut byte,
            dscount: 0,
        }
    }
}


pub const SBARHEIGHT: i32 = 32;
#[no_mangle]
pub static translations: [[byte; 256]; 3] = [[0; 256]; 3];
pub unsafe fn R_DrawColumn() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = unsafe { game_state() }.r_draw.dc_yh - unsafe { game_state() }.r_draw.dc_yl;
    if count < 0 as i32 {
        return;
    }
    if unsafe { game_state() }.r_draw.dc_x as u32 >= SCREENWIDTH as u32 || unsafe { game_state() }.r_draw.dc_yl < 0 as i32 || unsafe { game_state() }.r_draw.dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", unsafe { game_state() }.r_draw.dc_yl, unsafe { game_state() }.r_draw.dc_yh, unsafe { game_state() }.r_draw.dc_x));
    }
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[unsafe { game_state() }.r_draw.dc_x as usize] as isize);
    fracstep = unsafe { game_state() }.r_draw.dc_iscale;
    frac = unsafe { game_state() }.r_draw.dc_texturemid + (unsafe { game_state() }.r_draw.dc_yl as fixed_t - unsafe { game_state() }.r_main.centery as fixed_t) * fracstep;
    loop {
        *dest = *unsafe { game_state() }.r_draw.dc_colormap
            .offset(*unsafe { game_state() }.r_draw.dc_source.offset((frac as i32 >> FRACBITS & 127 as i32) as isize) as isize)
            as byte;
        dest = dest.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 != 0) {
            break;
        }
    }
}
pub unsafe fn R_DrawColumnLow() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    let mut x: i32 = 0;
    count = unsafe { game_state() }.r_draw.dc_yh - unsafe { game_state() }.r_draw.dc_yl;
    if count < 0 as i32 {
        return;
    }
    if unsafe { game_state() }.r_draw.dc_x as u32 >= SCREENWIDTH as u32 || unsafe { game_state() }.r_draw.dc_yl < 0 as i32 || unsafe { game_state() }.r_draw.dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", unsafe { game_state() }.r_draw.dc_yl, unsafe { game_state() }.r_draw.dc_yh, unsafe { game_state() }.r_draw.dc_x));
    }
    x = unsafe { game_state() }.r_draw.dc_x << 1 as i32;
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[x as usize] as isize);
    dest2 = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[(x + 1 as i32) as usize] as isize);
    fracstep = unsafe { game_state() }.r_draw.dc_iscale;
    frac = unsafe { game_state() }.r_draw.dc_texturemid + (unsafe { game_state() }.r_draw.dc_yl as fixed_t - unsafe { game_state() }.r_main.centery as fixed_t) * fracstep;
    loop {
        *dest = *unsafe { game_state() }.r_draw.dc_colormap
            .offset(*unsafe { game_state() }.r_draw.dc_source.offset((frac as i32 >> FRACBITS & 127 as i32) as isize) as isize)
            as byte;
        *dest2 = *dest;
        dest = dest.offset(SCREENWIDTH as isize);
        dest2 = dest2.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 != 0) {
            break;
        }
    }
}
pub const FUZZTABLE: i32 = 50;
pub const FUZZOFF: i32 = 320;
#[no_mangle]
pub static fuzzoffset: [i32; 50] = [
    FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
];
pub unsafe fn R_DrawFuzzColumn() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    if unsafe { game_state() }.r_draw.dc_yl == 0 {
        unsafe { game_state() }.r_draw.dc_yl = 1 as i32;
    }
    if unsafe { game_state() }.r_draw.dc_yh == unsafe { game_state() }.r_draw.viewheight - 1 as i32 {
        unsafe { game_state() }.r_draw.dc_yh = unsafe { game_state() }.r_draw.viewheight - 2 as i32;
    }
    count = unsafe { game_state() }.r_draw.dc_yh - unsafe { game_state() }.r_draw.dc_yl;
    if count < 0 as i32 {
        return;
    }
    if unsafe { game_state() }.r_draw.dc_x as u32 >= SCREENWIDTH as u32 || unsafe { game_state() }.r_draw.dc_yl < 0 as i32 || unsafe { game_state() }.r_draw.dc_yh >= SCREENHEIGHT {
        I_Error(&format!(
            "R_DrawFuzzColumn: {} to {} at {}",
            unsafe { game_state() }.r_draw.dc_yl, unsafe { game_state() }.r_draw.dc_yh, unsafe { game_state() }.r_draw.dc_x
        ));
    }
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[unsafe { game_state() }.r_draw.dc_x as usize] as isize);
    fracstep = unsafe { game_state() }.r_draw.dc_iscale;
    frac = unsafe { game_state() }.r_draw.dc_texturemid + (unsafe { game_state() }.r_draw.dc_yl as fixed_t - unsafe { game_state() }.r_main.centery as fixed_t) * fracstep;
    loop {
        *dest = *unsafe { game_state() }.r_data.colormaps.offset(
            (6 as i32 * 256 as i32 + *dest.offset(fuzzoffset[unsafe { game_state() }.r_draw.fuzzpos as usize] as isize) as i32)
                as isize,
        ) as byte;
        unsafe { game_state() }.r_draw.fuzzpos += 1;
        if unsafe { game_state() }.r_draw.fuzzpos == FUZZTABLE {
            unsafe { game_state() }.r_draw.fuzzpos = 0 as i32;
        }
        dest = dest.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh2 = count;
        count = count - 1;
        if !(fresh2 != 0) {
            break;
        }
    }
}
pub unsafe fn R_DrawFuzzColumnLow() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    let mut x: i32 = 0;
    if unsafe { game_state() }.r_draw.dc_yl == 0 {
        unsafe { game_state() }.r_draw.dc_yl = 1 as i32;
    }
    if unsafe { game_state() }.r_draw.dc_yh == unsafe { game_state() }.r_draw.viewheight - 1 as i32 {
        unsafe { game_state() }.r_draw.dc_yh = unsafe { game_state() }.r_draw.viewheight - 2 as i32;
    }
    count = unsafe { game_state() }.r_draw.dc_yh - unsafe { game_state() }.r_draw.dc_yl;
    if count < 0 as i32 {
        return;
    }
    x = unsafe { game_state() }.r_draw.dc_x << 1 as i32;
    if x as u32 >= SCREENWIDTH as u32 || unsafe { game_state() }.r_draw.dc_yl < 0 as i32 || unsafe { game_state() }.r_draw.dc_yh >= SCREENHEIGHT {
        I_Error(&format!(
            "R_DrawFuzzColumn: {} to {} at {}",
            unsafe { game_state() }.r_draw.dc_yl, unsafe { game_state() }.r_draw.dc_yh, unsafe { game_state() }.r_draw.dc_x
        ));
    }
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[x as usize] as isize);
    dest2 = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[(x + 1 as i32) as usize] as isize);
    fracstep = unsafe { game_state() }.r_draw.dc_iscale;
    frac = unsafe { game_state() }.r_draw.dc_texturemid + (unsafe { game_state() }.r_draw.dc_yl as fixed_t - unsafe { game_state() }.r_main.centery as fixed_t) * fracstep;
    loop {
        *dest = *unsafe { game_state() }.r_data.colormaps.offset(
            (6 as i32 * 256 as i32 + *dest.offset(fuzzoffset[unsafe { game_state() }.r_draw.fuzzpos as usize] as isize) as i32)
                as isize,
        ) as byte;
        *dest2 = *unsafe { game_state() }.r_data.colormaps.offset(
            (6 as i32 * 256 as i32 + *dest2.offset(fuzzoffset[unsafe { game_state() }.r_draw.fuzzpos as usize] as isize) as i32)
                as isize,
        ) as byte;
        unsafe { game_state() }.r_draw.fuzzpos += 1;
        if unsafe { game_state() }.r_draw.fuzzpos == FUZZTABLE {
            unsafe { game_state() }.r_draw.fuzzpos = 0 as i32;
        }
        dest = dest.offset(SCREENWIDTH as isize);
        dest2 = dest2.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh3 = count;
        count = count - 1;
        if !(fresh3 != 0) {
            break;
        }
    }
}
pub unsafe fn R_DrawTranslatedColumn() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = unsafe { game_state() }.r_draw.dc_yh - unsafe { game_state() }.r_draw.dc_yl;
    if count < 0 as i32 {
        return;
    }
    if unsafe { game_state() }.r_draw.dc_x as u32 >= SCREENWIDTH as u32 || unsafe { game_state() }.r_draw.dc_yl < 0 as i32 || unsafe { game_state() }.r_draw.dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", unsafe { game_state() }.r_draw.dc_yl, unsafe { game_state() }.r_draw.dc_yh, unsafe { game_state() }.r_draw.dc_x));
    }
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[unsafe { game_state() }.r_draw.dc_x as usize] as isize);
    fracstep = unsafe { game_state() }.r_draw.dc_iscale;
    frac = unsafe { game_state() }.r_draw.dc_texturemid + (unsafe { game_state() }.r_draw.dc_yl as fixed_t - unsafe { game_state() }.r_main.centery as fixed_t) * fracstep;
    loop {
        *dest = *unsafe { game_state() }.r_draw.dc_colormap.offset(
            *unsafe { game_state() }.r_draw.dc_translation.offset(*unsafe { game_state() }.r_draw.dc_source.offset((frac >> FRACBITS) as isize) as isize)
                as isize,
        ) as byte;
        dest = dest.offset(SCREENWIDTH as isize);
        frac += fracstep;
        let fresh4 = count;
        count = count - 1;
        if !(fresh4 != 0) {
            break;
        }
    }
}
pub unsafe fn R_DrawTranslatedColumnLow() {
    let mut count: i32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    let mut x: i32 = 0;
    count = unsafe { game_state() }.r_draw.dc_yh - unsafe { game_state() }.r_draw.dc_yl;
    if count < 0 as i32 {
        return;
    }
    x = unsafe { game_state() }.r_draw.dc_x << 1 as i32;
    if x as u32 >= SCREENWIDTH as u32 || unsafe { game_state() }.r_draw.dc_yl < 0 as i32 || unsafe { game_state() }.r_draw.dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", unsafe { game_state() }.r_draw.dc_yl, unsafe { game_state() }.r_draw.dc_yh, x));
    }
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[x as usize] as isize);
    dest2 = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.dc_yl as usize].offset(unsafe { game_state() }.r_draw.columnofs[(x + 1 as i32) as usize] as isize);
    fracstep = unsafe { game_state() }.r_draw.dc_iscale;
    frac = unsafe { game_state() }.r_draw.dc_texturemid + (unsafe { game_state() }.r_draw.dc_yl as fixed_t - unsafe { game_state() }.r_main.centery as fixed_t) * fracstep;
    loop {
        *dest = *unsafe { game_state() }.r_draw.dc_colormap.offset(
            *unsafe { game_state() }.r_draw.dc_translation.offset(*unsafe { game_state() }.r_draw.dc_source.offset((frac >> FRACBITS) as isize) as isize)
                as isize,
        ) as byte;
        *dest2 = *unsafe { game_state() }.r_draw.dc_colormap.offset(
            *unsafe { game_state() }.r_draw.dc_translation.offset(*unsafe { game_state() }.r_draw.dc_source.offset((frac >> FRACBITS) as isize) as isize)
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
    }
}
pub unsafe fn R_InitTranslationTables() {
    let mut i: i32 = 0;
    unsafe { game_state() }.r_draw.translationtables = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        256 as i32 * 3 as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut byte;
    i = 0 as i32;
    while i < 256 as i32 {
        if i >= 0x70 as i32 && i <= 0x7f as i32 {
            *unsafe { game_state() }.r_draw.translationtables.offset(i as isize) = (0x60 as i32 + (i & 0xf as i32)) as byte;
            *unsafe { game_state() }.r_draw.translationtables.offset((i + 256 as i32) as isize) =
                (0x40 as i32 + (i & 0xf as i32)) as byte;
            *unsafe { game_state() }.r_draw.translationtables.offset((i + 512 as i32) as isize) =
                (0x20 as i32 + (i & 0xf as i32)) as byte;
        } else {
            let ref mut fresh11 = *unsafe { game_state() }.r_draw.translationtables.offset((i + 512 as i32) as isize);
            *fresh11 = i as byte;
            let ref mut fresh12 = *unsafe { game_state() }.r_draw.translationtables.offset((i + 256 as i32) as isize);
            *fresh12 = *fresh11;
            *unsafe { game_state() }.r_draw.translationtables.offset(i as isize) = *fresh12;
        }
        i += 1;
    }
}
pub unsafe fn R_DrawSpan() {
    let mut position: u32 = 0;
    let mut step: u32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut count: i32 = 0;
    let mut spot: i32 = 0;
    let mut xtemp: u32 = 0;
    let mut ytemp: u32 = 0;
    if unsafe { game_state() }.r_draw.ds_x2 < unsafe { game_state() }.r_draw.ds_x1
        || unsafe { game_state() }.r_draw.ds_x1 < 0 as i32
        || unsafe { game_state() }.r_draw.ds_x2 >= SCREENWIDTH
        || unsafe { game_state() }.r_draw.ds_y as u32 > SCREENHEIGHT as u32
    {
        I_Error(&format!("R_DrawSpan: {} to {} at {}", unsafe { game_state() }.r_draw.ds_x1, unsafe { game_state() }.r_draw.ds_x2, unsafe { game_state() }.r_draw.ds_y));
    }
    position = (unsafe { game_state() }.r_draw.ds_xfrac << 10 as i32) as u32 & 0xffff0000 as u32
        | (unsafe { game_state() }.r_draw.ds_yfrac as i32 >> 6 as i32 & 0xffff as i32) as u32;
    step = (unsafe { game_state() }.r_draw.ds_xstep << 10 as i32) as u32 & 0xffff0000 as u32
        | (unsafe { game_state() }.r_draw.ds_ystep as i32 >> 6 as i32 & 0xffff as i32) as u32;
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.ds_y as usize].offset(unsafe { game_state() }.r_draw.columnofs[unsafe { game_state() }.r_draw.ds_x1 as usize] as isize);
    count = unsafe { game_state() }.r_draw.ds_x2 - unsafe { game_state() }.r_draw.ds_x1;
    loop {
        ytemp = position >> 4 as i32 & 0xfc0 as u32;
        xtemp = position >> 26 as i32;
        spot = (xtemp | ytemp) as i32;
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = *unsafe { game_state() }.r_draw.ds_colormap.offset(*unsafe { game_state() }.r_draw.ds_source.offset(spot as isize) as isize) as byte;
        position = position.wrapping_add(step);
        let fresh7 = count;
        count = count - 1;
        if !(fresh7 != 0) {
            break;
        }
    }
}
pub unsafe fn R_DrawSpanLow() {
    let mut position: u32 = 0;
    let mut step: u32 = 0;
    let mut xtemp: u32 = 0;
    let mut ytemp: u32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut count: i32 = 0;
    let mut spot: i32 = 0;
    if unsafe { game_state() }.r_draw.ds_x2 < unsafe { game_state() }.r_draw.ds_x1
        || unsafe { game_state() }.r_draw.ds_x1 < 0 as i32
        || unsafe { game_state() }.r_draw.ds_x2 >= SCREENWIDTH
        || unsafe { game_state() }.r_draw.ds_y as u32 > SCREENHEIGHT as u32
    {
        I_Error(&format!("R_DrawSpan: {} to {} at {}", unsafe { game_state() }.r_draw.ds_x1, unsafe { game_state() }.r_draw.ds_x2, unsafe { game_state() }.r_draw.ds_y));
    }
    position = (unsafe { game_state() }.r_draw.ds_xfrac << 10 as i32) as u32 & 0xffff0000 as u32
        | (unsafe { game_state() }.r_draw.ds_yfrac as i32 >> 6 as i32 & 0xffff as i32) as u32;
    step = (unsafe { game_state() }.r_draw.ds_xstep << 10 as i32) as u32 & 0xffff0000 as u32
        | (unsafe { game_state() }.r_draw.ds_ystep as i32 >> 6 as i32 & 0xffff as i32) as u32;
    count = unsafe { game_state() }.r_draw.ds_x2 - unsafe { game_state() }.r_draw.ds_x1;
    unsafe { game_state() }.r_draw.ds_x1 <<= 1 as i32;
    unsafe { game_state() }.r_draw.ds_x2 <<= 1 as i32;
    dest = unsafe { game_state() }.r_draw.ylookup[unsafe { game_state() }.r_draw.ds_y as usize].offset(unsafe { game_state() }.r_draw.columnofs[unsafe { game_state() }.r_draw.ds_x1 as usize] as isize);
    loop {
        ytemp = position >> 4 as i32 & 0xfc0 as u32;
        xtemp = position >> 26 as i32;
        spot = (xtemp | ytemp) as i32;
        let fresh8 = dest;
        dest = dest.offset(1);
        *fresh8 = *unsafe { game_state() }.r_draw.ds_colormap.offset(*unsafe { game_state() }.r_draw.ds_source.offset(spot as isize) as isize) as byte;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = *unsafe { game_state() }.r_draw.ds_colormap.offset(*unsafe { game_state() }.r_draw.ds_source.offset(spot as isize) as isize) as byte;
        position = position.wrapping_add(step);
        let fresh10 = count;
        count = count - 1;
        if !(fresh10 != 0) {
            break;
        }
    }
}
pub unsafe fn R_InitBuffer(mut width: i32, mut height: i32) {
    let mut i: i32 = 0;
    unsafe { game_state() }.r_draw.viewwindowx = SCREENWIDTH - width >> 1 as i32;
    i = 0 as i32;
    while i < width {
        unsafe { game_state() }.r_draw.columnofs[i as usize] = unsafe { game_state() }.r_draw.viewwindowx + i;
        i += 1;
    }
    if width == SCREENWIDTH {
        unsafe { game_state() }.r_draw.viewwindowy = 0 as i32;
    } else {
        unsafe { game_state() }.r_draw.viewwindowy = SCREENHEIGHT - SBARHEIGHT - height >> 1 as i32;
    }
    i = 0 as i32;
    while i < height {
        unsafe { game_state() }.r_draw.ylookup[i as usize] = I_VideoBuffer.offset(((i + unsafe { game_state() }.r_draw.viewwindowy) * SCREENWIDTH) as isize);
        i += 1;
    }
}
pub unsafe fn R_FillBackScreen() {
    let mut src: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut patch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    let mut name1: *mut ::core::ffi::c_char =
        b"FLOOR7_2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut name2: *mut ::core::ffi::c_char =
        b"GRNROCK\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if unsafe { game_state() }.r_draw.scaledviewwidth == SCREENWIDTH {
        if !unsafe { game_state() }.r_draw.background_buffer.is_null() {
            Z_Free(
                unsafe { &mut game_state().z_zone },
                unsafe { game_state() }.r_draw.background_buffer as *mut ::core::ffi::c_void,
            );
            unsafe { game_state() }.r_draw.background_buffer = ::core::ptr::null_mut::<byte>();
        }
        return;
    }
    if unsafe { game_state() }.r_draw.background_buffer.is_null() {
        unsafe { game_state() }.r_draw.background_buffer = Z_Malloc(
            unsafe { &mut game_state().z_zone },
            SCREENWIDTH * (SCREENHEIGHT - SBARHEIGHT),
            PU_STATIC as i32,
            NULL,
        ) as *mut byte;
    }
    if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as i32 as u32 {
        name = name2;
    } else {
        name = name1;
    }
    src = W_CacheLumpName(&wad_name8_to_string(name), PU_CACHE as i32) as *mut byte;
    dest = unsafe { game_state() }.r_draw.background_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT - SBARHEIGHT {
        x = 0 as i32;
        while x < SCREENWIDTH / 64 as i32 {
            memcpy(
                dest as *mut ::core::ffi::c_void,
                src.offset(((y & 63 as i32) << 6 as i32) as isize) as *const ::core::ffi::c_void,
                64 as size_t,
            );
            dest = dest.offset(64 as i32 as isize);
            x += 1;
        }
        if SCREENWIDTH & 63 as i32 != 0 {
            memcpy(
                dest as *mut ::core::ffi::c_void,
                src.offset(((y & 63 as i32) << 6 as i32) as isize) as *const ::core::ffi::c_void,
                (SCREENWIDTH & 63 as i32) as size_t,
            );
            dest = dest.offset((SCREENWIDTH & 63 as i32) as isize);
        }
        y += 1;
    }
    V_UseBuffer(unsafe { &mut game_state().v_video }, unsafe { game_state() }.r_draw.background_buffer);
    patch = W_CacheLumpName("brdr_t", PU_CACHE as i32) as *mut patch_t;
    x = 0 as i32;
    while x < unsafe { game_state() }.r_draw.scaledviewwidth {
        V_DrawPatch(
            unsafe { &mut game_state().v_video },
            unsafe { game_state() }.r_draw.viewwindowx + x,
            unsafe { game_state() }.r_draw.viewwindowy - 8 as i32,
            patch,
        );
        x += 8 as i32;
    }
    patch = W_CacheLumpName("brdr_b", PU_CACHE as i32) as *mut patch_t;
    x = 0 as i32;
    while x < unsafe { game_state() }.r_draw.scaledviewwidth {
        V_DrawPatch(
            unsafe { &mut game_state().v_video },
            unsafe { game_state() }.r_draw.viewwindowx + x,
            unsafe { game_state() }.r_draw.viewwindowy + unsafe { game_state() }.r_draw.viewheight,
            patch,
        );
        x += 8 as i32;
    }
    patch = W_CacheLumpName("brdr_l", PU_CACHE as i32) as *mut patch_t;
    y = 0 as i32;
    while y < unsafe { game_state() }.r_draw.viewheight {
        V_DrawPatch(
            unsafe { &mut game_state().v_video },
            unsafe { game_state() }.r_draw.viewwindowx - 8 as i32,
            unsafe { game_state() }.r_draw.viewwindowy + y,
            patch,
        );
        y += 8 as i32;
    }
    patch = W_CacheLumpName("brdr_r", PU_CACHE as i32) as *mut patch_t;
    y = 0 as i32;
    while y < unsafe { game_state() }.r_draw.viewheight {
        V_DrawPatch(
            unsafe { &mut game_state().v_video },
            unsafe { game_state() }.r_draw.viewwindowx + unsafe { game_state() }.r_draw.scaledviewwidth,
            unsafe { game_state() }.r_draw.viewwindowy + y,
            patch,
        );
        y += 8 as i32;
    }
    V_DrawPatch(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.r_draw.viewwindowx - 8 as i32,
        unsafe { game_state() }.r_draw.viewwindowy - 8 as i32,
        W_CacheLumpName("brdr_tl", PU_CACHE as i32) as *mut patch_t,
    );
    V_DrawPatch(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.r_draw.viewwindowx + unsafe { game_state() }.r_draw.scaledviewwidth,
        unsafe { game_state() }.r_draw.viewwindowy - 8 as i32,
        W_CacheLumpName("brdr_tr", PU_CACHE as i32) as *mut patch_t,
    );
    V_DrawPatch(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.r_draw.viewwindowx - 8 as i32,
        unsafe { game_state() }.r_draw.viewwindowy + unsafe { game_state() }.r_draw.viewheight,
        W_CacheLumpName("brdr_bl", PU_CACHE as i32) as *mut patch_t,
    );
    V_DrawPatch(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.r_draw.viewwindowx + unsafe { game_state() }.r_draw.scaledviewwidth,
        unsafe { game_state() }.r_draw.viewwindowy + unsafe { game_state() }.r_draw.viewheight,
        W_CacheLumpName("brdr_br", PU_CACHE as i32) as *mut patch_t,
    );
    V_RestoreBuffer(unsafe { &mut game_state().v_video });
}
pub unsafe fn R_VideoErase(mut ofs: u32, mut count: i32) {
    if !unsafe { game_state() }.r_draw.background_buffer.is_null() {
        memcpy(
            I_VideoBuffer.offset(ofs as isize) as *mut ::core::ffi::c_void,
            unsafe { game_state() }.r_draw.background_buffer.offset(ofs as isize) as *const ::core::ffi::c_void,
            count as size_t,
        );
    }
}
pub unsafe fn R_DrawViewBorder() {
    let mut top: i32 = 0;
    let mut side: i32 = 0;
    let mut ofs: i32 = 0;
    let mut i: i32 = 0;
    if unsafe { game_state() }.r_draw.scaledviewwidth == SCREENWIDTH {
        return;
    }
    top = (SCREENHEIGHT - SBARHEIGHT - unsafe { game_state() }.r_draw.viewheight) / 2 as i32;
    side = (SCREENWIDTH - unsafe { game_state() }.r_draw.scaledviewwidth) / 2 as i32;
    R_VideoErase(0 as u32, top * SCREENWIDTH + side);
    ofs = (unsafe { game_state() }.r_draw.viewheight + top) * SCREENWIDTH - side;
    R_VideoErase(ofs as u32, top * SCREENWIDTH + side);
    ofs = top * SCREENWIDTH + SCREENWIDTH - side;
    side <<= 1 as i32;
    i = 1 as i32;
    while i < unsafe { game_state() }.r_draw.viewheight {
        R_VideoErase(ofs as u32, side);
        ofs += SCREENWIDTH;
        i += 1;
    }
    V_MarkRect(
        unsafe { &mut game_state().v_video },
        0 as i32,
        0 as i32,
        SCREENWIDTH,
        SCREENHEIGHT - SBARHEIGHT,
    );
}
