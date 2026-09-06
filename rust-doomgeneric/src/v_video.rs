use crate::src::r_data::column_t;
use crate::src::hu_lib::patch_t;
use crate::src::i_system::I_Error;
use crate::src::w_wad::W_CacheLumpName;
use crate::src::i_video::mouse_acceleration;
use crate::src::i_video::mouse_threshold;
use crate::src::m_misc::M_FileExists;
use crate::src::i_video::usemouse;
use crate::src::m_bbox::M_AddToBox;
use crate::src::m_misc::M_WriteFile;
use crate::src::i_video::I_VideoBuffer;
use crate::src::m_misc::M_snprintf;
use crate::src::i_video::I_GetPaletteIndex;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use libc::{memcpy, memset};

extern "C" {
    fn fabs(__x: f64) -> f64;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub type fixed_t = i32;
pub type vpatchclipfunc_t = Option<
    unsafe extern "C" fn(*mut patch_t, i32, i32) -> boolean,
>;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct pcx_t {
    pub manufacturer: ::core::ffi::c_char,
    pub version: ::core::ffi::c_char,
    pub encoding: ::core::ffi::c_char,
    pub bits_per_pixel: ::core::ffi::c_char,
    pub xmin: u16,
    pub ymin: u16,
    pub xmax: u16,
    pub ymax: u16,
    pub hres: u16,
    pub vres: u16,
    pub palette: [u8; 48],
    pub reserved: ::core::ffi::c_char,
    pub color_planes: ::core::ffi::c_char,
    pub bytes_per_line: u16,
    pub palette_type: u16,
    pub filler: [::core::ffi::c_char; 58],
    pub data: u8,
}
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
#[no_mangle]
pub static mut tinttable: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub static mut xlatab: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut dest_screen: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub static mut dirtybox: [i32; 4] = [0; 4];
static mut patchclip_callback: vpatchclipfunc_t = None;
pub unsafe fn V_MarkRect(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
) {
    if dest_screen == I_VideoBuffer {
        M_AddToBox(&raw mut dirtybox as *mut fixed_t, x as fixed_t, y as fixed_t);
        M_AddToBox(
            &raw mut dirtybox as *mut fixed_t,
            x as fixed_t + width as fixed_t - 1 as fixed_t,
            y as fixed_t + height as fixed_t - 1 as fixed_t,
        );
    }
}
pub unsafe fn V_CopyRect(
    mut srcx: i32,
    mut srcy: i32,
    mut source: *mut byte,
    mut width: i32,
    mut height: i32,
    mut destx: i32,
    mut desty: i32,
) {
    let mut src: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    if srcx < 0 as i32 || srcx + width > SCREENWIDTH
        || srcy < 0 as i32 || srcy + height > SCREENHEIGHT
        || destx < 0 as i32 || destx + width > SCREENWIDTH
        || desty < 0 as i32 || desty + height > SCREENHEIGHT
    {
        I_Error("Bad V_CopyRect");
    }
    V_MarkRect(destx, desty, width, height);
    src = source.offset((SCREENWIDTH * srcy) as isize).offset(srcx as isize);
    dest = dest_screen.offset((SCREENWIDTH * desty) as isize).offset(destx as isize);
    while height > 0 as i32 {
        memcpy(
            dest as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            width as size_t,
        );
        src = src.offset(SCREENWIDTH as isize);
        dest = dest.offset(SCREENWIDTH as isize);
        height -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_SetPatchClipCallback(mut func: vpatchclipfunc_t) {
    patchclip_callback = func;
}
pub unsafe fn V_DrawPatch(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    let mut count: i32 = 0;
    let mut col: i32 = 0;
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut w: i32 = 0;
    y -= (*patch).topoffset as i32;
    x -= (*patch).leftoffset as i32;
    if patchclip_callback.is_some() {
        if patchclip_callback.expect("non-null function pointer")(patch, x, y) == 0 {
            return;
        }
    }
    if x < 0 as i32
        || x + (*patch).width as i32 > SCREENWIDTH
        || y < 0 as i32
        || y + (*patch).height as i32 > SCREENHEIGHT
    {
        I_Error(&format!(
            "Bad V_DrawPatch x={} y={} patch.width={} patch.height={} topoffset={} leftoffset={}",
            x,
            y,
            (*patch).width as i32,
            (*patch).height as i32,
            (*patch).topoffset as i32,
            (*patch).leftoffset as i32,
        ));
    }
    V_MarkRect(
        x,
        y,
        (*patch).width as i32,
        (*patch).height as i32,
    );
    col = 0 as i32;
    desttop = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    w = (*patch).width as i32;
    while col < w {
        column = (patch as *mut byte)
            .offset(
                *(&raw const (*patch).columnofs as *const i32)
                    .offset(col as isize) as isize,
            ) as *mut column_t;
        while (*column).topdelta as i32 != 0xff as i32 {
            source = (column as *mut byte).offset(3 as i32 as isize);
            dest = desttop
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            count = (*column).length as i32;
            loop {
                let fresh0 = count;
                count = count - 1;
                if !(fresh0 != 0) {
                    break;
                }
                let fresh1 = source;
                source = source.offset(1);
                *dest = *fresh1;
                dest = dest.offset(SCREENWIDTH as isize);
            }
            column = (column as *mut byte)
                .offset((*column).length as i32 as isize)
                .offset(4 as i32 as isize) as *mut column_t;
        }
        x += 1;
        col += 1;
        desttop = desttop.offset(1);
    }
}
pub unsafe fn V_DrawPatchFlipped(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    let mut count: i32 = 0;
    let mut col: i32 = 0;
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut w: i32 = 0;
    y -= (*patch).topoffset as i32;
    x -= (*patch).leftoffset as i32;
    if patchclip_callback.is_some() {
        if patchclip_callback.expect("non-null function pointer")(patch, x, y) == 0 {
            return;
        }
    }
    if x < 0 as i32
        || x + (*patch).width as i32 > SCREENWIDTH
        || y < 0 as i32
        || y + (*patch).height as i32 > SCREENHEIGHT
    {
        I_Error("Bad V_DrawPatchFlipped");
    }
    V_MarkRect(
        x,
        y,
        (*patch).width as i32,
        (*patch).height as i32,
    );
    col = 0 as i32;
    desttop = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    w = (*patch).width as i32;
    while col < w {
        column = (patch as *mut byte)
            .offset(
                *(&raw const (*patch).columnofs as *const i32)
                    .offset((w - 1 as i32 - col) as isize) as isize,
            ) as *mut column_t;
        while (*column).topdelta as i32 != 0xff as i32 {
            source = (column as *mut byte).offset(3 as i32 as isize);
            dest = desttop
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            count = (*column).length as i32;
            loop {
                let fresh2 = count;
                count = count - 1;
                if !(fresh2 != 0) {
                    break;
                }
                let fresh3 = source;
                source = source.offset(1);
                *dest = *fresh3;
                dest = dest.offset(SCREENWIDTH as isize);
            }
            column = (column as *mut byte)
                .offset((*column).length as i32 as isize)
                .offset(4 as i32 as isize) as *mut column_t;
        }
        x += 1;
        col += 1;
        desttop = desttop.offset(1);
    }
}
pub unsafe fn V_DrawPatchDirect(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    V_DrawPatch(x, y, patch);
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawTLPatch(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    let mut count: i32 = 0;
    let mut col: i32 = 0;
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut w: i32 = 0;
    y -= (*patch).topoffset as i32;
    x -= (*patch).leftoffset as i32;
    if x < 0 as i32
        || x + (*patch).width as i32 > SCREENWIDTH
        || y < 0 as i32
        || y + (*patch).height as i32 > SCREENHEIGHT
    {
        I_Error("Bad V_DrawTLPatch");
    }
    col = 0 as i32;
    desttop = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    w = (*patch).width as i32;
    while col < w {
        column = (patch as *mut byte)
            .offset(
                *(&raw const (*patch).columnofs as *const i32)
                    .offset(col as isize) as isize,
            ) as *mut column_t;
        while (*column).topdelta as i32 != 0xff as i32 {
            source = (column as *mut byte).offset(3 as i32 as isize);
            dest = desttop
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            count = (*column).length as i32;
            loop {
                let fresh4 = count;
                count = count - 1;
                if !(fresh4 != 0) {
                    break;
                }
                let fresh5 = source;
                source = source.offset(1);
                *dest = *tinttable
                    .offset(
                        (((*dest as i32) << 8 as i32)
                            + *fresh5 as i32) as isize,
                    );
                dest = dest.offset(SCREENWIDTH as isize);
            }
            column = (column as *mut byte)
                .offset((*column).length as i32 as isize)
                .offset(4 as i32 as isize) as *mut column_t;
        }
        x += 1;
        col += 1;
        desttop = desttop.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawXlaPatch(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    let mut count: i32 = 0;
    let mut col: i32 = 0;
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut w: i32 = 0;
    y -= (*patch).topoffset as i32;
    x -= (*patch).leftoffset as i32;
    if patchclip_callback.is_some() {
        if patchclip_callback.expect("non-null function pointer")(patch, x, y) == 0 {
            return;
        }
    }
    col = 0 as i32;
    desttop = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    w = (*patch).width as i32;
    while col < w {
        column = (patch as *mut byte)
            .offset(
                *(&raw const (*patch).columnofs as *const i32)
                    .offset(col as isize) as isize,
            ) as *mut column_t;
        while (*column).topdelta as i32 != 0xff as i32 {
            source = (column as *mut byte).offset(3 as i32 as isize);
            dest = desttop
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            count = (*column).length as i32;
            loop {
                let fresh10 = count;
                count = count - 1;
                if !(fresh10 != 0) {
                    break;
                }
                *dest = *xlatab
                    .offset(
                        (*dest as i32
                            + ((*source as i32)
                                << 8 as i32)) as isize,
                    );
                source = source.offset(1);
                dest = dest.offset(SCREENWIDTH as isize);
            }
            column = (column as *mut byte)
                .offset((*column).length as i32 as isize)
                .offset(4 as i32 as isize) as *mut column_t;
        }
        x += 1;
        col += 1;
        desttop = desttop.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawAltTLPatch(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    let mut count: i32 = 0;
    let mut col: i32 = 0;
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut w: i32 = 0;
    y -= (*patch).topoffset as i32;
    x -= (*patch).leftoffset as i32;
    if x < 0 as i32
        || x + (*patch).width as i32 > SCREENWIDTH
        || y < 0 as i32
        || y + (*patch).height as i32 > SCREENHEIGHT
    {
        I_Error("Bad V_DrawAltTLPatch");
    }
    col = 0 as i32;
    desttop = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    w = (*patch).width as i32;
    while col < w {
        column = (patch as *mut byte)
            .offset(
                *(&raw const (*patch).columnofs as *const i32)
                    .offset(col as isize) as isize,
            ) as *mut column_t;
        while (*column).topdelta as i32 != 0xff as i32 {
            source = (column as *mut byte).offset(3 as i32 as isize);
            dest = desttop
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            count = (*column).length as i32;
            loop {
                let fresh6 = count;
                count = count - 1;
                if !(fresh6 != 0) {
                    break;
                }
                let fresh7 = source;
                source = source.offset(1);
                *dest = *tinttable
                    .offset(
                        (((*dest as i32) << 8 as i32)
                            + *fresh7 as i32) as isize,
                    );
                dest = dest.offset(SCREENWIDTH as isize);
            }
            column = (column as *mut byte)
                .offset((*column).length as i32 as isize)
                .offset(4 as i32 as isize) as *mut column_t;
        }
        x += 1;
        col += 1;
        desttop = desttop.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawShadowedPatch(
    mut x: i32,
    mut y: i32,
    mut patch: *mut patch_t,
) {
    let mut count: i32 = 0;
    let mut col: i32 = 0;
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut desttop2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut w: i32 = 0;
    y -= (*patch).topoffset as i32;
    x -= (*patch).leftoffset as i32;
    if x < 0 as i32
        || x + (*patch).width as i32 > SCREENWIDTH
        || y < 0 as i32
        || y + (*patch).height as i32 > SCREENHEIGHT
    {
        I_Error("Bad V_DrawShadowedPatch");
    }
    col = 0 as i32;
    desttop = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    desttop2 = dest_screen
        .offset(((y + 2 as i32) * SCREENWIDTH) as isize)
        .offset(x as isize)
        .offset(2 as i32 as isize);
    w = (*patch).width as i32;
    while col < w {
        column = (patch as *mut byte)
            .offset(
                *(&raw const (*patch).columnofs as *const i32)
                    .offset(col as isize) as isize,
            ) as *mut column_t;
        while (*column).topdelta as i32 != 0xff as i32 {
            source = (column as *mut byte).offset(3 as i32 as isize);
            dest = desttop
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            dest2 = desttop2
                .offset(
                    ((*column).topdelta as i32 * SCREENWIDTH) as isize,
                );
            count = (*column).length as i32;
            loop {
                let fresh8 = count;
                count = count - 1;
                if !(fresh8 != 0) {
                    break;
                }
                *dest2 = *tinttable
                    .offset(
                        ((*dest2 as i32) << 8 as i32)
                            as isize,
                    );
                dest2 = dest2.offset(SCREENWIDTH as isize);
                let fresh9 = source;
                source = source.offset(1);
                *dest = *fresh9;
                dest = dest.offset(SCREENWIDTH as isize);
            }
            column = (column as *mut byte)
                .offset((*column).length as i32 as isize)
                .offset(4 as i32 as isize) as *mut column_t;
        }
        x += 1;
        col += 1;
        desttop = desttop.offset(1);
        desttop2 = desttop2.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_LoadTintTable() {
    tinttable = W_CacheLumpName("TINTTAB",
        PU_STATIC as i32,
    ) as *mut byte;
}
#[no_mangle]
pub unsafe extern "C" fn V_LoadXlaTable() {
    xlatab = W_CacheLumpName("XLATAB",
        PU_STATIC as i32,
    ) as *mut byte;
}
pub unsafe fn V_DrawBlock(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut src: *mut byte,
) {
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    if x < 0 as i32 || x + width > SCREENWIDTH
        || y < 0 as i32 || y + height > SCREENHEIGHT
    {
        I_Error("Bad V_DrawBlock");
    }
    V_MarkRect(x, y, width, height);
    dest = dest_screen.offset((y * SCREENWIDTH) as isize).offset(x as isize);
    loop {
        let fresh11 = height;
        height = height - 1;
        if !(fresh11 != 0) {
            break;
        }
        memcpy(
            dest as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            width as size_t,
        );
        src = src.offset(width as isize);
        dest = dest.offset(SCREENWIDTH as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawFilledBox(
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut c: i32,
) {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut buf1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut x1: i32 = 0;
    let mut y1: i32 = 0;
    buf = I_VideoBuffer.offset((SCREENWIDTH * y) as isize).offset(x as isize)
        as *mut uint8_t;
    y1 = 0 as i32;
    while y1 < h {
        buf1 = buf;
        x1 = 0 as i32;
        while x1 < w {
            let fresh12 = buf1;
            buf1 = buf1.offset(1);
            *fresh12 = c as uint8_t;
            x1 += 1;
        }
        buf = buf.offset(SCREENWIDTH as isize);
        y1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawHorizLine(
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut c: i32,
) {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut x1: i32 = 0;
    buf = I_VideoBuffer.offset((SCREENWIDTH * y) as isize).offset(x as isize)
        as *mut uint8_t;
    x1 = 0 as i32;
    while x1 < w {
        let fresh13 = buf;
        buf = buf.offset(1);
        *fresh13 = c as uint8_t;
        x1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawVertLine(
    mut x: i32,
    mut y: i32,
    mut h: i32,
    mut c: i32,
) {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut y1: i32 = 0;
    buf = I_VideoBuffer.offset((SCREENWIDTH * y) as isize).offset(x as isize)
        as *mut uint8_t;
    y1 = 0 as i32;
    while y1 < h {
        *buf = c as uint8_t;
        buf = buf.offset(SCREENWIDTH as isize);
        y1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawBox(
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut c: i32,
) {
    V_DrawHorizLine(x, y, w, c);
    V_DrawHorizLine(x, y + h - 1 as i32, w, c);
    V_DrawVertLine(x, y, h, c);
    V_DrawVertLine(x + w - 1 as i32, y, h, c);
}
#[no_mangle]
pub unsafe extern "C" fn V_DrawRawScreen(mut raw: *mut byte) {
    memcpy(
        dest_screen as *mut ::core::ffi::c_void,
        raw as *const ::core::ffi::c_void,
        (SCREENWIDTH * SCREENHEIGHT) as size_t,
    );
}
pub unsafe fn V_Init() {}
pub unsafe fn V_UseBuffer(mut buffer: *mut byte) {
    dest_screen = buffer;
}
pub unsafe fn V_RestoreBuffer() {
    dest_screen = I_VideoBuffer;
}
#[no_mangle]
pub unsafe extern "C" fn WritePCXfile(
    mut filename: *mut ::core::ffi::c_char,
    mut data: *mut byte,
    mut width: i32,
    mut height: i32,
    mut palette: *mut byte,
) {
    let mut i: i32 = 0;
    let mut length: i32 = 0;
    let mut pcx: *mut pcx_t = ::core::ptr::null_mut::<pcx_t>();
    let mut pack: *mut byte = ::core::ptr::null_mut::<byte>();
    pcx = Z_Malloc(
        width * height * 2 as i32 + 1000 as i32,
        PU_STATIC as i32,
        NULL,
    ) as *mut pcx_t;
    (*pcx).manufacturer = 0xa as ::core::ffi::c_char;
    (*pcx).version = 5 as ::core::ffi::c_char;
    (*pcx).encoding = 1 as ::core::ffi::c_char;
    (*pcx).bits_per_pixel = 8 as ::core::ffi::c_char;
    (*pcx).xmin = 0 as u16;
    (*pcx).ymin = 0 as u16;
    (*pcx).xmax = (width - 1 as i32) as i16
        as u16;
    (*pcx).ymax = (height - 1 as i32) as i16
        as u16;
    (*pcx).hres = width as i16 as u16;
    (*pcx).vres = height as i16 as u16;
    memset(
        &raw mut (*pcx).palette as *mut u8 as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 48]>() as size_t,
    );
    (*pcx).color_planes = 1 as ::core::ffi::c_char;
    (*pcx).bytes_per_line = width as i16 as u16;
    (*pcx).palette_type = 2 as i32 as i16
        as u16;
    memset(
        &raw mut (*pcx).filler as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[::core::ffi::c_char; 58]>() as size_t,
    );
    pack = &raw mut (*pcx).data as *mut byte;
    i = 0 as i32;
    while i < width * height {
        if *data as i32 & 0xc0 as i32
            != 0xc0 as i32
        {
            let fresh14 = data;
            data = data.offset(1);
            let fresh15 = pack;
            pack = pack.offset(1);
            *fresh15 = *fresh14;
        } else {
            let fresh16 = pack;
            pack = pack.offset(1);
            *fresh16 = 0xc1 as byte;
            let fresh17 = data;
            data = data.offset(1);
            let fresh18 = pack;
            pack = pack.offset(1);
            *fresh18 = *fresh17;
        }
        i += 1;
    }
    let fresh19 = pack;
    pack = pack.offset(1);
    *fresh19 = 0xc as byte;
    i = 0 as i32;
    while i < 768 as i32 {
        let fresh20 = palette;
        palette = palette.offset(1);
        let fresh21 = pack;
        pack = pack.offset(1);
        *fresh21 = *fresh20;
        i += 1;
    }
    length = pack.offset_from(pcx as *mut byte) as i64
        as i32;
    M_WriteFile(filename, pcx as *mut ::core::ffi::c_void, length);
    Z_Free(pcx as *mut ::core::ffi::c_void);
}
pub unsafe fn V_ScreenShot(mut format: *mut ::core::ffi::c_char) {
    let mut i: i32 = 0;
    let mut lbmname: [::core::ffi::c_char; 16] = [0; 16];
    let mut ext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    ext = b"pcx\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    i = 0 as i32;
    while i <= 99 as i32 {
        M_snprintf(
            &raw mut lbmname as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
            format,
            i,
            ext,
        );
        if !M_FileExists(&raw mut lbmname as *mut ::core::ffi::c_char) {
            break;
        }
        i += 1;
    }
    if i == 100 as i32 {
        I_Error("V_ScreenShot: Couldn't create a PCX");
    }
    WritePCXfile(
        &raw mut lbmname as *mut ::core::ffi::c_char,
        I_VideoBuffer,
        SCREENWIDTH,
        SCREENHEIGHT,
        W_CacheLumpName("PLAYPAL",
            PU_CACHE as i32,
        ) as *mut byte,
    );
}
pub const MOUSE_SPEED_BOX_WIDTH: i32 = 120 as i32;
pub const MOUSE_SPEED_BOX_HEIGHT: i32 = 9 as i32;
pub unsafe fn V_DrawMouseSpeedBox(mut speed: i32) {
    let mut bgcolor: i32 = 0;
    let mut bordercolor: i32 = 0;
    let mut red: i32 = 0;
    let mut black: i32 = 0;
    let mut white: i32 = 0;
    let mut yellow: i32 = 0;
    let mut box_x: i32 = 0;
    let mut box_y: i32 = 0;
    let mut original_speed: i32 = 0;
    let mut redline_x: i32 = 0;
    let mut linelen: i32 = 0;
    bgcolor = I_GetPaletteIndex(
        0x77 as i32,
        0x77 as i32,
        0x77 as i32,
    );
    bordercolor = I_GetPaletteIndex(
        0x55 as i32,
        0x55 as i32,
        0x55 as i32,
    );
    red = I_GetPaletteIndex(
        0xff as i32,
        0 as i32,
        0 as i32,
    );
    black = I_GetPaletteIndex(
        0 as i32,
        0 as i32,
        0 as i32,
    );
    yellow = I_GetPaletteIndex(
        0xff as i32,
        0xff as i32,
        0 as i32,
    );
    white = I_GetPaletteIndex(
        0xff as i32,
        0xff as i32,
        0xff as i32,
    );
    if usemouse == 0
        || fabs(
            (mouse_acceleration - 1 as i32 as f32)
                as f64,
        ) < 0.01f64
    {
        return;
    }
    box_x = SCREENWIDTH - MOUSE_SPEED_BOX_WIDTH - 10 as i32;
    box_y = 15 as i32;
    V_DrawFilledBox(
        box_x,
        box_y,
        MOUSE_SPEED_BOX_WIDTH,
        MOUSE_SPEED_BOX_HEIGHT,
        bgcolor,
    );
    V_DrawBox(box_x, box_y, MOUSE_SPEED_BOX_WIDTH, MOUSE_SPEED_BOX_HEIGHT, bordercolor);
    redline_x = MOUSE_SPEED_BOX_WIDTH / 3 as i32;
    if speed < mouse_threshold {
        original_speed = speed;
    } else {
        original_speed = speed - mouse_threshold;
        original_speed = (original_speed as f32 / mouse_acceleration)
            as i32;
        original_speed += mouse_threshold;
    }
    linelen = original_speed * redline_x / mouse_threshold;
    if linelen > MOUSE_SPEED_BOX_WIDTH - 1 as i32 {
        linelen = MOUSE_SPEED_BOX_WIDTH - 1 as i32;
    }
    V_DrawHorizLine(
        box_x + 1 as i32,
        box_y + 4 as i32,
        MOUSE_SPEED_BOX_WIDTH - 2 as i32,
        black,
    );
    if linelen < redline_x {
        V_DrawHorizLine(
            box_x + 1 as i32,
            box_y + MOUSE_SPEED_BOX_HEIGHT / 2 as i32,
            linelen,
            white,
        );
    } else {
        V_DrawHorizLine(
            box_x + 1 as i32,
            box_y + MOUSE_SPEED_BOX_HEIGHT / 2 as i32,
            redline_x,
            white,
        );
        V_DrawHorizLine(
            box_x + redline_x,
            box_y + MOUSE_SPEED_BOX_HEIGHT / 2 as i32,
            linelen - redline_x,
            yellow,
        );
    }
    V_DrawVertLine(
        box_x + redline_x,
        box_y + 1 as i32,
        MOUSE_SPEED_BOX_HEIGHT - 2 as i32,
        red,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
