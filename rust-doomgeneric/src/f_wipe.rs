
use crate::src::i_video::I_ReadScreen;
use crate::src::m_random::M_Random;
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn Z_Malloc(
        size: i32,
        tag: i32,
        ptr: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn Z_Free(ptr: *mut ::core::ffi::c_void);
    static mut I_VideoBuffer: *mut byte;
    fn V_DrawBlock(
        x: i32,
        y_0: i32,
        width: i32,
        height: i32,
        src: *mut byte,
    );
    fn V_MarkRect(
        x: i32,
        y_0: i32,
        width: i32,
        height: i32,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type C2RustUnnamed = u32;
pub const PU_NUM_TAGS: C2RustUnnamed = 9;
pub const PU_CACHE: C2RustUnnamed = 8;
pub const PU_PURGELEVEL: C2RustUnnamed = 7;
pub const PU_LEVSPEC: C2RustUnnamed = 6;
pub const PU_LEVEL: C2RustUnnamed = 5;
pub const PU_FREE: C2RustUnnamed = 4;
pub const PU_MUSIC: C2RustUnnamed = 3;
pub const PU_SOUND: C2RustUnnamed = 2;
pub const PU_STATIC: C2RustUnnamed = 1;
pub type uint8_t = __uint8_t;
pub type byte = uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
static mut go: bool = false;
static mut wipe_scr_start: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut wipe_scr_end: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut wipe_scr: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub unsafe extern "C" fn wipe_shittyColMajorXform(
    mut array: *mut i16,
    mut width: i32,
    mut height: i32,
) {
    let mut x: i32 = 0;
    let mut y_0: i32 = 0;
    let mut dest: *mut i16 = ::core::ptr::null_mut::<
        i16,
    >();
    dest = Z_Malloc(
        width * height * 2 as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i16;
    y_0 = 0 as i32;
    while y_0 < height {
        x = 0 as i32;
        while x < width {
            *dest.offset((x * height + y_0) as isize) = *array
                .offset((y_0 * width + x) as isize);
            x += 1;
        }
        y_0 += 1;
    }
    memcpy(
        array as *mut ::core::ffi::c_void,
        dest as *const ::core::ffi::c_void,
        (width * height * 2 as i32) as size_t,
    );
    Z_Free(dest as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wipe_initColorXForm(
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    memcpy(
        wipe_scr as *mut ::core::ffi::c_void,
        wipe_scr_start as *const ::core::ffi::c_void,
        (width * height) as size_t,
    );
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn wipe_doColorXForm(
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    let mut changed: bool;
    let mut w: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut e: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut newval: i32 = 0;
    changed = false;
    w = wipe_scr;
    e = wipe_scr_end;
    while w != wipe_scr.offset((width * height) as isize) {
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
#[no_mangle]
pub unsafe extern "C" fn wipe_exitColorXForm(
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    return 0 as i32;
}
static mut y: *mut i32 = ::core::ptr::null::<i32>()
    as *mut i32;
#[no_mangle]
pub unsafe extern "C" fn wipe_initMelt(
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    memcpy(
        wipe_scr as *mut ::core::ffi::c_void,
        wipe_scr_start as *const ::core::ffi::c_void,
        (width * height) as size_t,
    );
    wipe_shittyColMajorXform(
        wipe_scr_start as *mut i16,
        width / 2 as i32,
        height,
    );
    wipe_shittyColMajorXform(
        wipe_scr_end as *mut i16,
        width / 2 as i32,
        height,
    );
    y = Z_Malloc(
        (width as usize)
            .wrapping_mul(::core::mem::size_of::<i32>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i32;
    *y.offset(0 as i32 as isize) = -(M_Random()
        % 16 as i32);
    i = 1 as i32;
    while i < width {
        r = M_Random() % 3 as i32 - 1 as i32;
        *y.offset(i as isize) = *y.offset((i - 1 as i32) as isize) + r;
        if *y.offset(i as isize) > 0 as i32 {
            *y.offset(i as isize) = 0 as i32;
        } else if *y.offset(i as isize) == -(16 as i32) {
            *y.offset(i as isize) = -(15 as i32);
        }
        i += 1;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn wipe_doMelt(
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut dy: i32 = 0;
    let mut idx: i32 = 0;
    let mut s: *mut i16 = ::core::ptr::null_mut::<
        i16,
    >();
    let mut d: *mut i16 = ::core::ptr::null_mut::<
        i16,
    >();
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
            if *y.offset(i as isize) < 0 as i32 {
                let ref mut fresh1 = *y.offset(i as isize);
                *fresh1 += 1;
                done = false;
            } else if *y.offset(i as isize) < height {
                dy = if *y.offset(i as isize) < 16 as i32 {
                    *y.offset(i as isize) + 1 as i32
                } else {
                    8 as i32
                };
                if *y.offset(i as isize) + dy >= height {
                    dy = height - *y.offset(i as isize);
                }
                s = (wipe_scr_end as *mut i16)
                    .offset((i * height + *y.offset(i as isize)) as isize)
                    as *mut i16;
                d = (wipe_scr as *mut i16)
                    .offset((*y.offset(i as isize) * width + i) as isize)
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
                *y.offset(i as isize) += dy;
                s = (wipe_scr_start as *mut i16)
                    .offset((i * height) as isize) as *mut i16;
                d = (wipe_scr as *mut i16)
                    .offset((*y.offset(i as isize) * width + i) as isize)
                    as *mut i16;
                idx = 0 as i32;
                j = height - *y.offset(i as isize);
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
#[no_mangle]
pub unsafe extern "C" fn wipe_exitMelt(
    mut width: i32,
    mut height: i32,
    mut ticks: i32,
) -> i32 {
    Z_Free(y as *mut ::core::ffi::c_void);
    Z_Free(wipe_scr_start as *mut ::core::ffi::c_void);
    Z_Free(wipe_scr_end as *mut ::core::ffi::c_void);
    return 0 as i32;
}
pub unsafe fn wipe_StartScreen(
    mut x: i32,
    mut y_0: i32,
    mut width: i32,
    mut height: i32,
) -> i32 {
    wipe_scr_start = Z_Malloc(
        SCREENWIDTH * SCREENHEIGHT,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    I_ReadScreen(wipe_scr_start);
    return 0 as i32;
}
pub unsafe fn wipe_EndScreen(
    mut x: i32,
    mut y_0: i32,
    mut width: i32,
    mut height: i32,
) -> i32 {
    wipe_scr_end = Z_Malloc(
        SCREENWIDTH * SCREENHEIGHT,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    I_ReadScreen(wipe_scr_end);
    V_DrawBlock(x, y_0, width, height, wipe_scr_start);
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
    static mut wipes: [Option<
        unsafe extern "C" fn(
            i32,
            i32,
            i32,
        ) -> i32,
    >; 6] = unsafe {
        [
            Some(
                wipe_initColorXForm
                    as unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
            ),
            Some(
                wipe_doColorXForm
                    as unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
            ),
            Some(
                wipe_exitColorXForm
                    as unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
            ),
            Some(
                wipe_initMelt
                    as unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
            ),
            Some(
                wipe_doMelt
                    as unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
            ),
            Some(
                wipe_exitMelt
                    as unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
            ),
        ]
    };
    if !go {
        go = true;
        wipe_scr = I_VideoBuffer;
        Some(
                (*(&raw mut wipes
                    as *mut Option<
                        unsafe extern "C" fn(
                            i32,
                            i32,
                            i32,
                        ) -> i32,
                    >)
                    .offset((wipeno * 3 as i32) as isize))
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(width, height, ticks);
    }
    V_MarkRect(0 as i32, 0 as i32, width, height);
    rc = Some(
            (*(&raw mut wipes
                as *mut Option<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                >)
                .offset(
                    (wipeno * 3 as i32 + 1 as i32) as isize,
                ))
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(width, height, ticks);
    if rc != 0 {
        go = false;
        Some(
                (*(&raw mut wipes
                    as *mut Option<
                        unsafe extern "C" fn(
                            i32,
                            i32,
                            i32,
                        ) -> i32,
                    >)
                    .offset(
                        (wipeno * 3 as i32 + 2 as i32)
                            as isize,
                    ))
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(width, height, ticks);
    }
    return (!go) as i32;
}
