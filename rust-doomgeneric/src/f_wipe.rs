use crate::src::doomdef::NULL;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::game_state::game_state;
use crate::src::i_video::I_ReadScreen;
use crate::src::m_random::M_Random;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::v_video::V_DrawBlock;
use crate::src::v_video::V_MarkRect;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use libc::memcpy;

pub struct FWipeState {
    pub go: bool,
    pub wipe_scr_start: *mut byte,
    pub wipe_scr_end: *mut byte,
    pub wipe_scr: *mut byte,
    pub y: *mut i32,
}

impl FWipeState {
    pub const fn new() -> Self {
        FWipeState {
            go: false,
            wipe_scr_start: ::core::ptr::null::<byte>() as *mut byte,
            wipe_scr_end: ::core::ptr::null::<byte>() as *mut byte,
            wipe_scr: ::core::ptr::null::<byte>() as *mut byte,
            y: ::core::ptr::null::<i32>() as *mut i32,
        }
    }
}

pub unsafe fn wipe_shittyColMajorXform(mut array: *mut i16, mut width: i32, mut height: i32) {
    let mut x: i32 = 0;
    let mut y_0: i32 = 0;
    let mut dest: *mut i16 = ::core::ptr::null_mut::<i16>();
    dest = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        width * height * 2 as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i16;
    y_0 = 0 as i32;
    while y_0 < height {
        x = 0 as i32;
        while x < width {
            *dest.offset((x * height + y_0) as isize) = *array.offset((y_0 * width + x) as isize);
            x += 1;
        }
        y_0 += 1;
    }
    memcpy(
        array as *mut ::core::ffi::c_void,
        dest as *const ::core::ffi::c_void,
        (width * height * 2 as i32) as size_t,
    );
    Z_Free(
        unsafe { &mut game_state().z_zone },
        dest as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn wipe_initColorXForm(mut width: i32, mut height: i32, mut ticks: i32) -> i32 {
    memcpy(
        unsafe { game_state() }.f_wipe.wipe_scr as *mut ::core::ffi::c_void,
        unsafe { game_state() }.f_wipe.wipe_scr_start as *const ::core::ffi::c_void,
        (width * height) as size_t,
    );
    return 0 as i32;
}
pub unsafe fn wipe_doColorXForm(mut width: i32, mut height: i32, mut ticks: i32) -> i32 {
    let mut changed: bool;
    let mut w: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut e: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut newval: i32 = 0;
    changed = false;
    w = unsafe { game_state() }.f_wipe.wipe_scr;
    e = unsafe { game_state() }.f_wipe.wipe_scr_end;
    while w != unsafe { game_state() }.f_wipe.wipe_scr.offset((width * height) as isize) {
        if *w as i32 != *e as i32 {
            if *w as i32 > *e as i32 {
                newval = *w as i32 - ticks;
                if newval < *e as i32 {
                    *w = *e;
                } else {
                    *w = newval as byte;
                }
                changed = true;
            } else if (*w as i32) < *e as i32 {
                newval = *w as i32 + ticks;
                if newval > *e as i32 {
                    *w = *e;
                } else {
                    *w = newval as byte;
                }
                changed = true;
            }
        }
        w = w.offset(1);
        e = e.offset(1);
    }
    return (!changed) as i32;
}
pub unsafe fn wipe_exitColorXForm(mut width: i32, mut height: i32, mut ticks: i32) -> i32 {
    return 0 as i32;
}
pub unsafe fn wipe_initMelt(mut width: i32, mut height: i32, mut ticks: i32) -> i32 {
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    memcpy(
        unsafe { game_state() }.f_wipe.wipe_scr as *mut ::core::ffi::c_void,
        unsafe { game_state() }.f_wipe.wipe_scr_start as *const ::core::ffi::c_void,
        (width * height) as size_t,
    );
    wipe_shittyColMajorXform(unsafe { game_state() }.f_wipe.wipe_scr_start as *mut i16, width / 2 as i32, height);
    wipe_shittyColMajorXform(unsafe { game_state() }.f_wipe.wipe_scr_end as *mut i16, width / 2 as i32, height);
    unsafe { game_state() }.f_wipe.y = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        (width as usize).wrapping_mul(::core::mem::size_of::<i32>() as usize) as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i32;
    *unsafe { game_state() }.f_wipe.y.offset(0 as i32 as isize) = -(M_Random(unsafe { &mut game_state().m_random }) % 16 as i32);
    i = 1 as i32;
    while i < width {
        r = M_Random(unsafe { &mut game_state().m_random }) % 3 as i32 - 1 as i32;
        *unsafe { game_state() }.f_wipe.y.offset(i as isize) = *unsafe { game_state() }.f_wipe.y.offset((i - 1 as i32) as isize) + r;
        if *unsafe { game_state() }.f_wipe.y.offset(i as isize) > 0 as i32 {
            *unsafe { game_state() }.f_wipe.y.offset(i as isize) = 0 as i32;
        } else if *unsafe { game_state() }.f_wipe.y.offset(i as isize) == -(16 as i32) {
            *unsafe { game_state() }.f_wipe.y.offset(i as isize) = -(15 as i32);
        }
        i += 1;
    }
    return 0 as i32;
}
pub unsafe fn wipe_doMelt(mut width: i32, mut height: i32, mut ticks: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut dy: i32 = 0;
    let mut idx: i32 = 0;
    let mut s: *mut i16 = ::core::ptr::null_mut::<i16>();
    let mut d: *mut i16 = ::core::ptr::null_mut::<i16>();
    let mut done: bool = true;
    width /= 2 as i32;
    loop {
        let fresh0 = ticks;
        ticks = ticks - 1;
        if !(fresh0 != 0) {
            break;
        }
        i = 0 as i32;
        while i < width {
            if *unsafe { game_state() }.f_wipe.y.offset(i as isize) < 0 as i32 {
                let ref mut fresh1 = *unsafe { game_state() }.f_wipe.y.offset(i as isize);
                *fresh1 += 1;
                done = false;
            } else if *unsafe { game_state() }.f_wipe.y.offset(i as isize) < height {
                dy = if *unsafe { game_state() }.f_wipe.y.offset(i as isize) < 16 as i32 {
                    *unsafe { game_state() }.f_wipe.y.offset(i as isize) + 1 as i32
                } else {
                    8 as i32
                };
                if *unsafe { game_state() }.f_wipe.y.offset(i as isize) + dy >= height {
                    dy = height - *unsafe { game_state() }.f_wipe.y.offset(i as isize);
                }
                s = (unsafe { game_state() }.f_wipe.wipe_scr_end as *mut i16).offset((i * height + *unsafe { game_state() }.f_wipe.y.offset(i as isize)) as isize)
                    as *mut i16;
                d = (unsafe { game_state() }.f_wipe.wipe_scr as *mut i16).offset((*unsafe { game_state() }.f_wipe.y.offset(i as isize) * width + i) as isize)
                    as *mut i16;
                idx = 0 as i32;
                j = dy;
                while j != 0 {
                    let fresh2 = s;
                    s = s.offset(1);
                    *d.offset(idx as isize) = *fresh2;
                    idx += width;
                    j -= 1;
                }
                *unsafe { game_state() }.f_wipe.y.offset(i as isize) += dy;
                s = (unsafe { game_state() }.f_wipe.wipe_scr_start as *mut i16).offset((i * height) as isize) as *mut i16;
                d = (unsafe { game_state() }.f_wipe.wipe_scr as *mut i16).offset((*unsafe { game_state() }.f_wipe.y.offset(i as isize) * width + i) as isize)
                    as *mut i16;
                idx = 0 as i32;
                j = height - *unsafe { game_state() }.f_wipe.y.offset(i as isize);
                while j != 0 {
                    let fresh3 = s;
                    s = s.offset(1);
                    *d.offset(idx as isize) = *fresh3;
                    idx += width;
                    j -= 1;
                }
                done = false;
            }
            i += 1;
        }
    }
    return done as i32;
}
pub unsafe fn wipe_exitMelt(mut width: i32, mut height: i32, mut ticks: i32) -> i32 {
    Z_Free(
        unsafe { &mut game_state().z_zone },
        unsafe { game_state() }.f_wipe.y as *mut ::core::ffi::c_void,
    );
    Z_Free(
        unsafe { &mut game_state().z_zone },
        unsafe { game_state() }.f_wipe.wipe_scr_start as *mut ::core::ffi::c_void,
    );
    Z_Free(
        unsafe { &mut game_state().z_zone },
        unsafe { game_state() }.f_wipe.wipe_scr_end as *mut ::core::ffi::c_void,
    );
    return 0 as i32;
}
pub unsafe fn wipe_StartScreen(mut x: i32, mut y_0: i32, mut width: i32, mut height: i32) -> i32 {
    unsafe { game_state() }.f_wipe.wipe_scr_start = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        SCREENWIDTH * SCREENHEIGHT,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    I_ReadScreen(unsafe { game_state() }.f_wipe.wipe_scr_start);
    return 0 as i32;
}
pub unsafe fn wipe_EndScreen(mut x: i32, mut y_0: i32, mut width: i32, mut height: i32) -> i32 {
    unsafe { game_state() }.f_wipe.wipe_scr_end = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        SCREENWIDTH * SCREENHEIGHT,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    I_ReadScreen(unsafe { game_state() }.f_wipe.wipe_scr_end);
    V_DrawBlock(
        unsafe { &mut game_state().v_video },
        x,
        y_0,
        width,
        height,
        unsafe { game_state() }.f_wipe.wipe_scr_start,
    );
    return 0 as i32;
}
pub unsafe fn wipe_ScreenWipe(
    mut wipeno: i32,
    mut x: i32,
    mut y_0: i32,
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    let mut rc: i32 = 0;
    const wipes: [Option<unsafe fn(i32, i32, i32) -> i32>; 6] = {
        [
            Some(wipe_initColorXForm as unsafe fn(i32, i32, i32) -> i32),
            Some(wipe_doColorXForm as unsafe fn(i32, i32, i32) -> i32),
            Some(wipe_exitColorXForm as unsafe fn(i32, i32, i32) -> i32),
            Some(wipe_initMelt as unsafe fn(i32, i32, i32) -> i32),
            Some(wipe_doMelt as unsafe fn(i32, i32, i32) -> i32),
            Some(wipe_exitMelt as unsafe fn(i32, i32, i32) -> i32),
        ]
    };
    if !unsafe { game_state() }.f_wipe.go {
        unsafe { game_state() }.f_wipe.go = true;
        unsafe { game_state() }.f_wipe.wipe_scr = unsafe { game_state() }.i_video.I_VideoBuffer;
        wipes[(wipeno * 3 as i32) as usize].expect("non-null function pointer")(
            width, height, ticks,
        );
    }
    V_MarkRect(
        unsafe { &mut game_state().v_video },
        0 as i32,
        0 as i32,
        width,
        height,
    );
    rc = wipes[(wipeno * 3 as i32 + 1 as i32) as usize].expect("non-null function pointer")(
        width, height, ticks,
    );
    if rc != 0 {
        unsafe { game_state() }.f_wipe.go = false;
        wipes[(wipeno * 3 as i32 + 2 as i32) as usize].expect("non-null function pointer")(
            width, height, ticks,
        );
    }
    return (!unsafe { game_state() }.f_wipe.go) as i32;
}
