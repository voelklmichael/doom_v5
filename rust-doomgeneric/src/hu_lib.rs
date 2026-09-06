
use crate::src::r_draw::R_VideoErase;
use crate::src::r_draw::viewwindowx;
use crate::src::r_draw::viewwindowy;
use crate::src::v_video::V_DrawPatchDirect;
use crate::src::r_draw::viewwidth;
use crate::src::r_draw::viewheight;
extern "C" {
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn toupper(__c: i32) -> i32;
    static mut automapactive: bool;
}
pub type __int32_t = i32;
pub type boolean = u32;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct patch_t {
    pub width: i16,
    pub height: i16,
    pub leftoffset: i16,
    pub topoffset: i16,
    pub columnofs: [i32; 8],
}
#[derive(Clone)]
pub struct hu_textline_t {
    pub x: i32,
    pub y: i32,
    pub f: *mut *mut patch_t,
    pub sc: i32,
    pub l: String,
    pub needsupdate: i32,
}
#[derive(Clone)]
pub struct hu_stext_t {
    pub l: [hu_textline_t; 4],
    pub h: i32,
    pub cl: i32,
    pub on: *mut bool,
    pub laston: bool,
}
#[derive(Clone)]
pub struct hu_itext_t {
    pub l: hu_textline_t,
    pub lm: i32,
    pub on: *mut bool,
    pub laston: bool,
}
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const KEY_ENTER: i32 = 13 as i32;
pub const KEY_BACKSPACE: i32 = 0x7f as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const HU_MAXLINELENGTH: i32 = 80 as i32;
#[no_mangle]
pub unsafe extern "C" fn HUlib_init() {}
#[no_mangle]
pub unsafe extern "C" fn HUlib_clearTextLine(mut t: *mut hu_textline_t) {
    (*t).l.clear();
    (*t).needsupdate = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_initTextLine(
    mut t: *mut hu_textline_t,
    mut x: i32,
    mut y: i32,
    mut f: *mut *mut patch_t,
    mut sc: i32,
) {
    (*t).x = x;
    (*t).y = y;
    (*t).f = f;
    (*t).sc = sc;
    HUlib_clearTextLine(t);
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_addCharToTextLine(
    mut t: *mut hu_textline_t,
    mut ch: ::core::ffi::c_char,
) -> boolean {
    if (*t).l.len() as i32 == HU_MAXLINELENGTH {
        return false_0 as boolean
    } else {
        (*t).l.push(ch as u8 as char);
        (*t).needsupdate = 4 as i32;
        return true_0 as boolean;
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_delCharFromTextLine(
    mut t: *mut hu_textline_t,
) -> boolean {
    if (*t).l.is_empty() {
        return false_0 as boolean
    } else {
        (*t).l.pop();
        (*t).needsupdate = 4 as i32;
        return true_0 as boolean;
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_drawTextLine(
    mut l: *mut hu_textline_t,
    mut drawcursor: boolean,
) {
    let mut i: i32 = 0;
    let mut w: i32 = 0;
    let mut x: i32 = 0;
    let mut c: u8 = 0;
    x = (*l).x;
    i = 0 as i32;
    while i < (*l).l.len() as i32 {
        c = toupper((*l).l.as_bytes()[i as usize] as i32) as u8;
        if c as i32 != ' ' as i32 && c as i32 >= (*l).sc
            && c as i32 <= '_' as i32
        {
            w = (**(*l).f.offset((c as i32 - (*l).sc) as isize)).width
                as i32;
            if x + w > SCREENWIDTH {
                break;
            }
            V_DrawPatchDirect(
                x,
                (*l).y,
                *(*l).f.offset((c as i32 - (*l).sc) as isize),
            );
            x += w;
        } else {
            x += 4 as i32;
            if x >= SCREENWIDTH {
                break;
            }
        }
        i += 1;
    }
    if drawcursor != 0
        && x
            + (**(*l).f.offset(('_' as i32 - (*l).sc) as isize)).width
                as i32 <= SCREENWIDTH
    {
        V_DrawPatchDirect(x, (*l).y, *(*l).f.offset(('_' as i32 - (*l).sc) as isize));
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_eraseTextLine(mut l: *mut hu_textline_t) {
    let mut lh: i32 = 0;
    let mut y: i32 = 0;
    let mut yoffset: i32 = 0;
    if !automapactive && viewwindowx != 0 && (*l).needsupdate != 0 {
        lh = (**(*l).f.offset(0 as i32 as isize)).height
            as i32 + 1 as i32;
        y = (*l).y;
        yoffset = y * SCREENWIDTH;
        while y < (*l).y + lh {
            if y < viewwindowy || y >= viewwindowy + viewheight {
                R_VideoErase(yoffset as u32, SCREENWIDTH);
            } else {
                R_VideoErase(yoffset as u32, viewwindowx);
                R_VideoErase(
                    (yoffset + viewwindowx + viewwidth) as u32,
                    viewwindowx,
                );
            }
            y += 1;
            yoffset += SCREENWIDTH;
        }
    }
    if (*l).needsupdate != 0 {
        (*l).needsupdate -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_initSText(
    mut s: *mut hu_stext_t,
    mut x: i32,
    mut y: i32,
    mut h: i32,
    mut font: *mut *mut patch_t,
    mut startchar: i32,
    mut on: *mut bool,
) {
    let mut i: i32 = 0;
    (*s).h = h;
    (*s).on = on;
    (*s).laston = true;
    (*s).cl = 0 as i32;
    i = 0 as i32;
    while i < h {
        HUlib_initTextLine(
            (&raw mut (*s).l as *mut hu_textline_t).offset(i as isize)
                as *mut hu_textline_t,
            x,
            y
                - i
                    * ((**font.offset(0 as i32 as isize)).height
                        as i32 + 1 as i32),
            font,
            startchar,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_addLineToSText(mut s: *mut hu_stext_t) {
    let mut i: i32 = 0;
    (*s).cl += 1;
    if (*s).cl == (*s).h {
        (*s).cl = 0 as i32;
    }
    HUlib_clearTextLine(
        (&raw mut (*s).l as *mut hu_textline_t).offset((*s).cl as isize)
            as *mut hu_textline_t,
    );
    i = 0 as i32;
    while i < (*s).h {
        (*s).l[i as usize].needsupdate = 4 as i32;
        i += 1;
    }
}
pub unsafe fn HUlib_addMessageToSText(
    mut s: *mut hu_stext_t,
    mut prefix: *mut ::core::ffi::c_char,
    msg: &str,
) {
    HUlib_addLineToSText(s);
    if !prefix.is_null() {
        while *prefix != 0 {
            let fresh1 = prefix;
            prefix = prefix.offset(1);
            HUlib_addCharToTextLine(
                (&raw mut (*s).l as *mut hu_textline_t).offset((*s).cl as isize)
                    as *mut hu_textline_t,
                *fresh1,
            );
        }
    }
    for b in msg.bytes() {
        HUlib_addCharToTextLine(
            (&raw mut (*s).l as *mut hu_textline_t).offset((*s).cl as isize)
                as *mut hu_textline_t,
            b as ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_drawSText(mut s: *mut hu_stext_t) {
    let mut i: i32 = 0;
    let mut idx: i32 = 0;
    let mut l: *mut hu_textline_t = ::core::ptr::null_mut::<hu_textline_t>();
    if !*(*s).on {
        return;
    }
    i = 0 as i32;
    while i < (*s).h {
        idx = (*s).cl - i;
        if idx < 0 as i32 {
            idx += (*s).h;
        }
        l = (&raw mut (*s).l as *mut hu_textline_t).offset(idx as isize)
            as *mut hu_textline_t;
        HUlib_drawTextLine(l, false_0 as boolean);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_eraseSText(mut s: *mut hu_stext_t) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*s).h {
        if (*s).laston && !*(*s).on {
            (*s).l[i as usize].needsupdate = 4 as i32;
        }
        HUlib_eraseTextLine(
            (&raw mut (*s).l as *mut hu_textline_t).offset(i as isize)
                as *mut hu_textline_t,
        );
        i += 1;
    }
    (*s).laston = *(*s).on;
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_initIText(
    mut it: *mut hu_itext_t,
    mut x: i32,
    mut y: i32,
    mut font: *mut *mut patch_t,
    mut startchar: i32,
    mut on: *mut bool,
) {
    (*it).lm = 0 as i32;
    (*it).on = on;
    (*it).laston = true;
    HUlib_initTextLine(&raw mut (*it).l, x, y, font, startchar);
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_delCharFromIText(mut it: *mut hu_itext_t) {
    if (*it).l.l.len() as i32 != (*it).lm {
        HUlib_delCharFromTextLine(&raw mut (*it).l);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_eraseLineFromIText(mut it: *mut hu_itext_t) {
    while (*it).lm != (*it).l.l.len() as i32 {
        HUlib_delCharFromTextLine(&raw mut (*it).l);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_resetIText(mut it: *mut hu_itext_t) {
    (*it).lm = 0 as i32;
    HUlib_clearTextLine(&raw mut (*it).l);
}
pub unsafe fn HUlib_addPrefixToIText(it: *mut hu_itext_t, s: &str) {
    for b in s.bytes() {
        HUlib_addCharToTextLine(&raw mut (*it).l, b as ::core::ffi::c_char);
    }
    (*it).lm = (*it).l.l.len() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_keyInIText(
    mut it: *mut hu_itext_t,
    mut ch: u8,
) -> boolean {
    ch = ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as usize > 1 as usize {
            if 0 != 0 {
                let mut __c: i32 = ch as i32;
                __res = (if __c < -(128 as i32)
                    || __c > 255 as i32
                {
                    __c as __int32_t
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                }) as i32;
            } else {
                __res = toupper(ch as i32);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(ch as i32 as isize)
                as i32;
        }
        __res
    }) as u8;
    if ch as i32 >= ' ' as i32 && ch as i32 <= '_' as i32 {
        HUlib_addCharToTextLine(&raw mut (*it).l, ch as ::core::ffi::c_char);
    } else if ch as i32 == KEY_BACKSPACE {
        HUlib_delCharFromIText(it);
    } else if ch as i32 != KEY_ENTER {
        return false_0 as boolean
    }
    return true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_drawIText(mut it: *mut hu_itext_t) {
    let mut l: *mut hu_textline_t = &raw mut (*it).l;
    if !*(*it).on {
        return;
    }
    HUlib_drawTextLine(l, true_0 as boolean);
}
#[no_mangle]
pub unsafe extern "C" fn HUlib_eraseIText(mut it: *mut hu_itext_t) {
    if (*it).laston && !*(*it).on {
        (*it).l.needsupdate = 4 as i32;
    }
    HUlib_eraseTextLine(&raw mut (*it).l);
    (*it).laston = *(*it).on;
}
