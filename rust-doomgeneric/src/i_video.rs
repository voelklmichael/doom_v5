use crate::src::i_system::I_Error;
use ::c2rust_bitfields;
use crate::src::m_argv::{myargv, M_CheckParmWithArgs};
use crate::src::i_input::I_GetEvent;
use crate::src::tables::gammatable;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use crate::src::doomdef::boolean;
use crate::src::doomdef::pixel_t;
use crate::src::stdint_types::{byte, uint8_t};
use crate::src::stdint_types::uint32_t;
use crate::src::stdint_types::size_t;
use libc::{memcpy, memset};
use libc::{atoi, strcmp};
use libc::printf;

extern "C" {
    static mut DG_ScreenBuffer: *mut pixel_t;
    fn DG_DrawFrame();
    fn DG_SetWindowTitle(title: *const ::core::ffi::c_char);
}
pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type grabmouse_callback_t = Option<unsafe fn() -> boolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FB_ScreenInfo {
    pub xres: uint32_t,
    pub yres: uint32_t,
    pub xres_virtual: uint32_t,
    pub yres_virtual: uint32_t,
    pub bits_per_pixel: uint32_t,
    pub red: FB_BitField,
    pub green: FB_BitField,
    pub blue: FB_BitField,
    pub transp: FB_BitField,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FB_BitField {
    pub offset: uint32_t,
    pub length: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct color {
    #[bitfield(name = "b", ty = "uint32_t", bits = "0..=7")]
    #[bitfield(name = "g", ty = "uint32_t", bits = "8..=15")]
    #[bitfield(name = "r", ty = "uint32_t", bits = "16..=23")]
    #[bitfield(name = "a", ty = "uint32_t", bits = "24..=31")]
    pub b_g_r_a: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct col_t {
    pub r: byte,
    pub g: byte,
    pub b: byte,
}
pub const INT_MAX: i32 = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const DOOMGENERIC_RESX: i32 = 640 as i32;
pub const DOOMGENERIC_RESY: i32 = 400 as i32;
static mut s_Fb: FB_ScreenInfo = FB_ScreenInfo {
    xres: 0,
    yres: 0,
    xres_virtual: 0,
    yres_virtual: 0,
    bits_per_pixel: 0,
    red: FB_BitField {
        offset: 0,
        length: 0,
    },
    green: FB_BitField {
        offset: 0,
        length: 0,
    },
    blue: FB_BitField {
        offset: 0,
        length: 0,
    },
    transp: FB_BitField {
        offset: 0,
        length: 0,
    },
};
#[no_mangle]
pub static mut fb_scaling: i32 = 1 as i32;
pub static mut usemouse: i32 = 0 as i32;
static mut colors: [color; 256] = [color { b_g_r_a: [0; 4] }; 256];
pub static mut I_VideoBuffer: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
pub static mut screensaver_mode: bool = false;
pub static mut screenvisible: bool = false;
pub static mut mouse_acceleration: f32 = 2.0f32;
pub static mut mouse_threshold: i32 = 10 as i32;
pub static mut usegamma: i32 = 0 as i32;
static mut rgb565_palette: [uint16_t; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn cmap_to_rgb565(
    mut out: *mut uint16_t,
    mut in_0: *mut uint8_t,
    mut in_pixels: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: color = color { b_g_r_a: [0; 4] };
    let mut r: uint16_t = 0;
    let mut g: uint16_t = 0;
    let mut b: uint16_t = 0;
    i = 0 as i32;
    while i < in_pixels {
        c = colors[*in_0 as usize];
        r = (((c.r() as i32 >> 3 as i32) as uint16_t
            as i32) << 11 as i32) as uint16_t;
        g = (((c.g() as i32 >> 2 as i32) as uint16_t
            as i32) << 5 as i32) as uint16_t;
        b = (((c.b() as i32 >> 3 as i32) as uint16_t
            as i32) << 0 as i32) as uint16_t;
        *out = (r as i32 | g as i32
            | b as i32) as uint16_t;
        in_0 = in_0.offset(1);
        j = 0 as i32;
        while j < fb_scaling {
            out = out.offset(1);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmap_to_fb(
    mut out: *mut uint8_t,
    mut in_0: *mut uint8_t,
    mut in_pixels: i32,
) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut c: color = color { b_g_r_a: [0; 4] };
    let mut pix: uint32_t = 0;
    i = 0 as i32;
    while i < in_pixels {
        c = colors[*in_0 as usize];
        if s_Fb.bits_per_pixel == 16 as uint32_t {
            let mut p: uint16_t = ((c.r() as i32
                & 0xf8 as i32) << 8 as i32
                | (c.g() as i32 & 0xfc as i32)
                    << 3 as i32
                | c.b() as i32 >> 3 as i32) as uint16_t;
            k = 0 as i32;
            while k < fb_scaling {
                *(out as *mut uint16_t) = p;
                out = out.offset(2 as i32 as isize);
                k += 1;
            }
        } else if s_Fb.bits_per_pixel == 32 as uint32_t {
            pix = ((c.r() as i32) << s_Fb.red.offset
                | (c.g() as i32) << s_Fb.green.offset
                | (c.b() as i32) << s_Fb.blue.offset) as uint32_t;
            k = 0 as i32;
            while k < fb_scaling {
                *(out as *mut uint32_t) = pix;
                out = out.offset(4 as i32 as isize);
                k += 1;
            }
        } else {
            I_Error(&format!("No idea how to convert {} bpp pixels", s_Fb.bits_per_pixel));
        }
        in_0 = in_0.offset(1);
        i += 1;
    }
}
pub unsafe fn I_InitGraphics() {
    let mut i: i32 = 0;
    let mut gfxmodeparm: i32 = 0;
    let mut mode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    memset(
        &raw mut s_Fb as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<FB_ScreenInfo>() as size_t,
    );
    s_Fb.xres = DOOMGENERIC_RESX as uint32_t;
    s_Fb.yres = DOOMGENERIC_RESY as uint32_t;
    s_Fb.xres_virtual = s_Fb.xres;
    s_Fb.yres_virtual = s_Fb.yres;
    gfxmodeparm = M_CheckParmWithArgs("-gfxmode", 1 as i32);
    if gfxmodeparm != 0 {
        mode = myargv[(gfxmodeparm + 1 as i32) as usize].as_ptr()
            as *mut ::core::ffi::c_char;
    } else {
        mode = b"rgba8888\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    if strcmp(mode, b"rgba8888\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as i32
    {
        s_Fb.bits_per_pixel = 32 as uint32_t;
        s_Fb.blue.length = 8 as uint32_t;
        s_Fb.green.length = 8 as uint32_t;
        s_Fb.red.length = 8 as uint32_t;
        s_Fb.transp.length = 8 as uint32_t;
        s_Fb.blue.offset = 0 as uint32_t;
        s_Fb.green.offset = 8 as uint32_t;
        s_Fb.red.offset = 16 as uint32_t;
        s_Fb.transp.offset = 24 as uint32_t;
    } else if strcmp(mode, b"rgb565\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as i32
    {
        s_Fb.bits_per_pixel = 16 as uint32_t;
        s_Fb.blue.length = 5 as uint32_t;
        s_Fb.green.length = 6 as uint32_t;
        s_Fb.red.length = 5 as uint32_t;
        s_Fb.transp.length = 0 as uint32_t;
        s_Fb.blue.offset = 11 as uint32_t;
        s_Fb.green.offset = 5 as uint32_t;
        s_Fb.red.offset = 0 as uint32_t;
        s_Fb.transp.offset = 16 as uint32_t;
    } else {
        I_Error(&format!(
            "Unknown gfxmode value: {}\n",
            ::std::ffi::CStr::from_ptr(mode).to_str().unwrap(),
        ));
    }
    printf(
        b"I_InitGraphics: framebuffer: x_res: %d, y_res: %d, x_virtual: %d, y_virtual: %d, bpp: %d\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        s_Fb.xres,
        s_Fb.yres,
        s_Fb.xres_virtual,
        s_Fb.yres_virtual,
        s_Fb.bits_per_pixel,
    );
    printf(
        b"I_InitGraphics: framebuffer: RGBA: %d%d%d%d, red_off: %d, green_off: %d, blue_off: %d, transp_off: %d\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        s_Fb.red.length,
        s_Fb.green.length,
        s_Fb.blue.length,
        s_Fb.transp.length,
        s_Fb.red.offset,
        s_Fb.green.offset,
        s_Fb.blue.offset,
        s_Fb.transp.offset,
    );
    printf(
        b"I_InitGraphics: DOOM screen size: w x h: %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        SCREENWIDTH,
        SCREENHEIGHT,
    );
    i = M_CheckParmWithArgs("-scaling", 1 as i32);
    if i > 0 as i32 {
        i = atoi(
            myargv[(i + 1 as i32) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        );
        fb_scaling = i;
        printf(
            b"I_InitGraphics: Scaling factor: %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            fb_scaling,
        );
    } else {
        fb_scaling = s_Fb.xres.wrapping_div(SCREENWIDTH as uint32_t)
            as i32;
        if s_Fb.yres.wrapping_div(SCREENHEIGHT as uint32_t) < fb_scaling as uint32_t {
            fb_scaling = s_Fb.yres.wrapping_div(SCREENHEIGHT as uint32_t)
                as i32;
        }
        printf(
            b"I_InitGraphics: Auto-scaling factor: %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            fb_scaling,
        );
    }
    I_VideoBuffer = Z_Malloc(
        SCREENWIDTH * SCREENHEIGHT,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    screenvisible = true;
    extern "C" {
        #[link_name = "I_InitInput"]
        fn I_InitInput_0();
    }
    I_InitInput_0();
}
#[no_mangle]
pub unsafe extern "C" fn I_ShutdownGraphics() {
    Z_Free(I_VideoBuffer as *mut ::core::ffi::c_void);
}
pub unsafe fn I_StartFrame() {}
pub unsafe fn I_StartTic() {
    I_GetEvent();
}
pub unsafe fn I_UpdateNoBlit() {}
pub unsafe fn I_FinishUpdate() {
    let mut y: i32 = 0;
    let mut x_offset: i32 = 0;
    let mut y_offset: i32 = 0;
    let mut x_offset_end: i32 = 0;
    let mut line_in: *mut u8 = ::core::ptr::null_mut::<
        u8,
    >();
    let mut line_out: *mut u8 = ::core::ptr::null_mut::<
        u8,
    >();
    y_offset = s_Fb
        .yres
        .wrapping_sub((SCREENHEIGHT * fb_scaling) as uint32_t)
        .wrapping_mul(s_Fb.bits_per_pixel)
        .wrapping_div(8 as uint32_t)
        .wrapping_div(2 as uint32_t) as i32;
    x_offset = s_Fb
        .xres
        .wrapping_sub((SCREENWIDTH * fb_scaling) as uint32_t)
        .wrapping_mul(s_Fb.bits_per_pixel)
        .wrapping_div(8 as uint32_t)
        .wrapping_div(2 as uint32_t) as i32;
    x_offset_end = s_Fb
        .xres
        .wrapping_sub((SCREENWIDTH * fb_scaling) as uint32_t)
        .wrapping_mul(s_Fb.bits_per_pixel)
        .wrapping_div(8 as uint32_t)
        .wrapping_sub(x_offset as uint32_t) as i32;
    line_in = I_VideoBuffer as *mut u8;
    line_out = DG_ScreenBuffer as *mut u8;
    y = SCREENHEIGHT;
    loop {
        let fresh3 = y;
        y = y - 1;
        if !(fresh3 != 0) {
            break;
        }
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < fb_scaling {
            line_out = line_out.offset(x_offset as isize);
            cmap_to_fb(
                line_out as *mut ::core::ffi::c_void as *mut uint8_t,
                line_in as *mut ::core::ffi::c_void as *mut uint8_t,
                SCREENWIDTH,
            );
            line_out = line_out
                .offset(
                    ((SCREENWIDTH * fb_scaling) as uint32_t)
                        .wrapping_mul(s_Fb.bits_per_pixel.wrapping_div(8 as uint32_t))
                        .wrapping_add(x_offset_end as uint32_t) as isize,
                );
            i += 1;
        }
        line_in = line_in.offset(SCREENWIDTH as isize);
    }
    DG_DrawFrame();
}
pub unsafe fn I_ReadScreen(mut scr: *mut byte) {
    memcpy(
        scr as *mut ::core::ffi::c_void,
        I_VideoBuffer as *const ::core::ffi::c_void,
        (SCREENWIDTH * SCREENHEIGHT) as size_t,
    );
}
pub unsafe fn I_SetPalette(mut palette: *mut byte) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 256 as i32 {
        colors[i as usize].set_a(0 as uint32_t as uint32_t);
        let mut rhs = {
            let fresh0 = palette;
            palette = palette.offset(1);
            gammatable[usegamma as usize][*fresh0 as usize] as uint32_t
        } as uint32_t;
        colors[i as usize].set_r(rhs);
        let mut rhs_0 = {
            let fresh1 = palette;
            palette = palette.offset(1);
            gammatable[usegamma as usize][*fresh1 as usize] as uint32_t
        } as uint32_t;
        colors[i as usize].set_g(rhs_0);
        let mut rhs_1 = {
            let fresh2 = palette;
            palette = palette.offset(1);
            gammatable[usegamma as usize][*fresh2 as usize] as uint32_t
        } as uint32_t;
        colors[i as usize].set_b(rhs_1);
        i += 1;
    }
}
pub unsafe fn I_GetPaletteIndex(
    mut r: i32,
    mut g: i32,
    mut b: i32,
) -> i32 {
    let mut best: i32 = 0;
    let mut best_diff: i32 = 0;
    let mut diff: i32 = 0;
    let mut i: i32 = 0;
    let mut color: col_t = col_t { r: 0, g: 0, b: 0 };
    printf(b"I_GetPaletteIndex\n\0" as *const u8 as *const ::core::ffi::c_char);
    best = 0 as i32;
    best_diff = INT_MAX;
    i = 0 as i32;
    while i < 256 as i32 {
        color.r = ((0xf800 as i32
            & rgb565_palette[i as usize] as i32)
            >> 11 as i32) as byte;
        color.g = ((0x7e0 as i32
            & rgb565_palette[i as usize] as i32)
            >> 5 as i32) as byte;
        color.b = (0x1f as i32
            & rgb565_palette[i as usize] as i32) as byte;
        diff = (r - color.r as i32) * (r - color.r as i32)
            + (g - color.g as i32) * (g - color.g as i32)
            + (b - color.b as i32) * (b - color.b as i32);
        if diff < best_diff {
            best = i;
            best_diff = diff;
        }
        if diff == 0 as i32 {
            break;
        }
        i += 1;
    }
    return best;
}
pub unsafe fn I_BeginRead() {}
pub unsafe fn I_EndRead() {}
pub unsafe fn I_SetWindowTitle(mut title: *mut ::core::ffi::c_char) {
    DG_SetWindowTitle(title);
}
pub unsafe fn I_GraphicsCheckCommandLine() {}
pub unsafe fn I_SetGrabMouseCallback(mut func: grabmouse_callback_t) {}
pub unsafe fn I_EnableLoadingDisk() {}
pub unsafe fn I_BindVideoVariables() {}
pub unsafe fn I_DisplayFPSDots(mut dots_on: bool) {}
pub unsafe fn I_CheckIsScreensaver() {}
pub const __INT_MAX__: i32 = 2147483647 as i32;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
