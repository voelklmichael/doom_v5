use crate::src::i_system::FILE;
use crate::src::m_argv::M_CheckParm;
use crate::src::z_zone::Z_Free;
extern "C" {
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
    fn puts(__s: *const ::core::ffi::c_char) -> i32;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: i32,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn Z_Malloc(
        size: i32,
        tag: i32,
        ptr: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type byte = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_mode_t {
    pub width: i32,
    pub height: i32,
    pub InitMode: Option<unsafe extern "C" fn(*mut byte) -> ()>,
    pub DrawScreen: Option<
        unsafe extern "C" fn(
            i32,
            i32,
            i32,
            i32,
        ) -> bool,
    >,
    pub poor_quality: bool,
}
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const INT_MAX: i32 = __INT_MAX__;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const SCREENWIDTH_4_3: i32 = 256 as i32;
pub const SCREENHEIGHT_4_3: i32 = 240 as i32;
static mut src_buffer: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut dest_buffer: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut dest_pitch: i32 = 0;
static mut stretch_tables: [*mut byte; 2] = [
    ::core::ptr::null::<byte>() as *mut byte,
    ::core::ptr::null::<byte>() as *mut byte,
];
static mut half_stretch_table: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub unsafe extern "C" fn I_InitScale(
    mut _src_buffer: *mut byte,
    mut _dest_buffer: *mut byte,
    mut _dest_pitch: i32,
) {
    src_buffer = _src_buffer;
    dest_buffer = _dest_buffer;
    dest_pitch = _dest_pitch;
}
unsafe extern "C" fn I_Scale1x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    let mut w: i32 = x2 - x1;
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer.offset((y1 * dest_pitch) as isize).offset(x1 as isize);
    y = y1;
    while y < y2 {
        memcpy(
            screenp as *mut ::core::ffi::c_void,
            bufp as *const ::core::ffi::c_void,
            w as size_t,
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_scale_1x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH,
        height: SCREENHEIGHT,
        InitMode: None,
        DrawScreen: Some(
            I_Scale1x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
unsafe extern "C" fn I_Scale2x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut multi_pitch: i32 = 0;
    multi_pitch = dest_pitch * 2 as i32;
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer
        .offset(((y1 * dest_pitch + x1) * 2 as i32) as isize);
    screenp2 = screenp.offset(dest_pitch as isize);
    y = y1;
    while y < y2 {
        let mut sp: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp2: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut bp: *mut byte = ::core::ptr::null_mut::<byte>();
        sp = screenp;
        sp2 = screenp2;
        bp = bufp;
        x = x1;
        while x < x2 {
            let fresh0 = sp;
            sp = sp.offset(1);
            *fresh0 = *bp;
            let fresh1 = sp;
            sp = sp.offset(1);
            *fresh1 = *bp;
            let fresh2 = sp2;
            sp2 = sp2.offset(1);
            *fresh2 = *bp;
            let fresh3 = sp2;
            sp2 = sp2.offset(1);
            *fresh3 = *bp;
            bp = bp.offset(1);
            x += 1;
        }
        screenp = screenp.offset(multi_pitch as isize);
        screenp2 = screenp2.offset(multi_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_scale_2x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 2 as i32,
        height: SCREENHEIGHT * 2 as i32,
        InitMode: None,
        DrawScreen: Some(
            I_Scale2x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
unsafe extern "C" fn I_Scale3x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp3: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut multi_pitch: i32 = 0;
    multi_pitch = dest_pitch * 3 as i32;
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer
        .offset(((y1 * dest_pitch + x1) * 3 as i32) as isize);
    screenp2 = screenp.offset(dest_pitch as isize);
    screenp3 = screenp.offset((dest_pitch * 2 as i32) as isize);
    y = y1;
    while y < y2 {
        let mut sp: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp2: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp3: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut bp: *mut byte = ::core::ptr::null_mut::<byte>();
        sp = screenp;
        sp2 = screenp2;
        sp3 = screenp3;
        bp = bufp;
        x = x1;
        while x < x2 {
            let fresh4 = sp;
            sp = sp.offset(1);
            *fresh4 = *bp;
            let fresh5 = sp;
            sp = sp.offset(1);
            *fresh5 = *bp;
            let fresh6 = sp;
            sp = sp.offset(1);
            *fresh6 = *bp;
            let fresh7 = sp2;
            sp2 = sp2.offset(1);
            *fresh7 = *bp;
            let fresh8 = sp2;
            sp2 = sp2.offset(1);
            *fresh8 = *bp;
            let fresh9 = sp2;
            sp2 = sp2.offset(1);
            *fresh9 = *bp;
            let fresh10 = sp3;
            sp3 = sp3.offset(1);
            *fresh10 = *bp;
            let fresh11 = sp3;
            sp3 = sp3.offset(1);
            *fresh11 = *bp;
            let fresh12 = sp3;
            sp3 = sp3.offset(1);
            *fresh12 = *bp;
            bp = bp.offset(1);
            x += 1;
        }
        screenp = screenp.offset(multi_pitch as isize);
        screenp2 = screenp2.offset(multi_pitch as isize);
        screenp3 = screenp3.offset(multi_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_scale_3x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 3 as i32,
        height: SCREENHEIGHT * 3 as i32,
        InitMode: None,
        DrawScreen: Some(
            I_Scale3x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
unsafe extern "C" fn I_Scale4x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp3: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp4: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut multi_pitch: i32 = 0;
    multi_pitch = dest_pitch * 4 as i32;
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer
        .offset(((y1 * dest_pitch + x1) * 4 as i32) as isize);
    screenp2 = screenp.offset(dest_pitch as isize);
    screenp3 = screenp.offset((dest_pitch * 2 as i32) as isize);
    screenp4 = screenp.offset((dest_pitch * 3 as i32) as isize);
    y = y1;
    while y < y2 {
        let mut sp: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp2: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp3: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp4: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut bp: *mut byte = ::core::ptr::null_mut::<byte>();
        sp = screenp;
        sp2 = screenp2;
        sp3 = screenp3;
        sp4 = screenp4;
        bp = bufp;
        x = x1;
        while x < x2 {
            let fresh13 = sp;
            sp = sp.offset(1);
            *fresh13 = *bp;
            let fresh14 = sp;
            sp = sp.offset(1);
            *fresh14 = *bp;
            let fresh15 = sp;
            sp = sp.offset(1);
            *fresh15 = *bp;
            let fresh16 = sp;
            sp = sp.offset(1);
            *fresh16 = *bp;
            let fresh17 = sp2;
            sp2 = sp2.offset(1);
            *fresh17 = *bp;
            let fresh18 = sp2;
            sp2 = sp2.offset(1);
            *fresh18 = *bp;
            let fresh19 = sp2;
            sp2 = sp2.offset(1);
            *fresh19 = *bp;
            let fresh20 = sp2;
            sp2 = sp2.offset(1);
            *fresh20 = *bp;
            let fresh21 = sp3;
            sp3 = sp3.offset(1);
            *fresh21 = *bp;
            let fresh22 = sp3;
            sp3 = sp3.offset(1);
            *fresh22 = *bp;
            let fresh23 = sp3;
            sp3 = sp3.offset(1);
            *fresh23 = *bp;
            let fresh24 = sp3;
            sp3 = sp3.offset(1);
            *fresh24 = *bp;
            let fresh25 = sp4;
            sp4 = sp4.offset(1);
            *fresh25 = *bp;
            let fresh26 = sp4;
            sp4 = sp4.offset(1);
            *fresh26 = *bp;
            let fresh27 = sp4;
            sp4 = sp4.offset(1);
            *fresh27 = *bp;
            let fresh28 = sp4;
            sp4 = sp4.offset(1);
            *fresh28 = *bp;
            bp = bp.offset(1);
            x += 1;
        }
        screenp = screenp.offset(multi_pitch as isize);
        screenp2 = screenp2.offset(multi_pitch as isize);
        screenp3 = screenp3.offset(multi_pitch as isize);
        screenp4 = screenp4.offset(multi_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_scale_4x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 4 as i32,
        height: SCREENHEIGHT * 4 as i32,
        InitMode: None,
        DrawScreen: Some(
            I_Scale4x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
unsafe extern "C" fn I_Scale5x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp3: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp4: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp5: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut multi_pitch: i32 = 0;
    multi_pitch = dest_pitch * 5 as i32;
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer
        .offset(((y1 * dest_pitch + x1) * 5 as i32) as isize);
    screenp2 = screenp.offset(dest_pitch as isize);
    screenp3 = screenp.offset((dest_pitch * 2 as i32) as isize);
    screenp4 = screenp.offset((dest_pitch * 3 as i32) as isize);
    screenp5 = screenp.offset((dest_pitch * 4 as i32) as isize);
    y = y1;
    while y < y2 {
        let mut sp: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp2: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp3: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp4: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut sp5: *mut byte = ::core::ptr::null_mut::<byte>();
        let mut bp: *mut byte = ::core::ptr::null_mut::<byte>();
        sp = screenp;
        sp2 = screenp2;
        sp3 = screenp3;
        sp4 = screenp4;
        sp5 = screenp5;
        bp = bufp;
        x = x1;
        while x < x2 {
            let fresh29 = sp;
            sp = sp.offset(1);
            *fresh29 = *bp;
            let fresh30 = sp;
            sp = sp.offset(1);
            *fresh30 = *bp;
            let fresh31 = sp;
            sp = sp.offset(1);
            *fresh31 = *bp;
            let fresh32 = sp;
            sp = sp.offset(1);
            *fresh32 = *bp;
            let fresh33 = sp;
            sp = sp.offset(1);
            *fresh33 = *bp;
            let fresh34 = sp2;
            sp2 = sp2.offset(1);
            *fresh34 = *bp;
            let fresh35 = sp2;
            sp2 = sp2.offset(1);
            *fresh35 = *bp;
            let fresh36 = sp2;
            sp2 = sp2.offset(1);
            *fresh36 = *bp;
            let fresh37 = sp2;
            sp2 = sp2.offset(1);
            *fresh37 = *bp;
            let fresh38 = sp2;
            sp2 = sp2.offset(1);
            *fresh38 = *bp;
            let fresh39 = sp3;
            sp3 = sp3.offset(1);
            *fresh39 = *bp;
            let fresh40 = sp3;
            sp3 = sp3.offset(1);
            *fresh40 = *bp;
            let fresh41 = sp3;
            sp3 = sp3.offset(1);
            *fresh41 = *bp;
            let fresh42 = sp3;
            sp3 = sp3.offset(1);
            *fresh42 = *bp;
            let fresh43 = sp3;
            sp3 = sp3.offset(1);
            *fresh43 = *bp;
            let fresh44 = sp4;
            sp4 = sp4.offset(1);
            *fresh44 = *bp;
            let fresh45 = sp4;
            sp4 = sp4.offset(1);
            *fresh45 = *bp;
            let fresh46 = sp4;
            sp4 = sp4.offset(1);
            *fresh46 = *bp;
            let fresh47 = sp4;
            sp4 = sp4.offset(1);
            *fresh47 = *bp;
            let fresh48 = sp4;
            sp4 = sp4.offset(1);
            *fresh48 = *bp;
            let fresh49 = sp5;
            sp5 = sp5.offset(1);
            *fresh49 = *bp;
            let fresh50 = sp5;
            sp5 = sp5.offset(1);
            *fresh50 = *bp;
            let fresh51 = sp5;
            sp5 = sp5.offset(1);
            *fresh51 = *bp;
            let fresh52 = sp5;
            sp5 = sp5.offset(1);
            *fresh52 = *bp;
            let fresh53 = sp5;
            sp5 = sp5.offset(1);
            *fresh53 = *bp;
            bp = bp.offset(1);
            x += 1;
        }
        screenp = screenp.offset(multi_pitch as isize);
        screenp2 = screenp2.offset(multi_pitch as isize);
        screenp3 = screenp3.offset(multi_pitch as isize);
        screenp4 = screenp4.offset(multi_pitch as isize);
        screenp5 = screenp5.offset(multi_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_scale_5x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 5 as i32,
        height: SCREENHEIGHT * 5 as i32,
        InitMode: None,
        DrawScreen: Some(
            I_Scale5x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
unsafe extern "C" fn FindNearestColor(
    mut palette: *mut byte,
    mut r: i32,
    mut g: i32,
    mut b: i32,
) -> i32 {
    let mut col: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut best: i32 = 0;
    let mut best_diff: i32 = 0;
    let mut diff: i32 = 0;
    let mut i: i32 = 0;
    best = 0 as i32;
    best_diff = INT_MAX;
    i = 0 as i32;
    while i < 256 as i32 {
        col = palette.offset((i * 3 as i32) as isize);
        diff = (r - *col.offset(0 as i32 as isize) as i32)
            * (r - *col.offset(0 as i32 as isize) as i32)
            + (g - *col.offset(1 as i32 as isize) as i32)
                * (g
                    - *col.offset(1 as i32 as isize)
                        as i32)
            + (b - *col.offset(2 as i32 as isize) as i32)
                * (b
                    - *col.offset(2 as i32 as isize)
                        as i32);
        if diff == 0 as i32 {
            return i
        } else if diff < best_diff {
            best = i;
            best_diff = diff;
        }
        i += 1;
    }
    return best;
}
unsafe extern "C" fn GenerateStretchTable(
    mut palette: *mut byte,
    mut pct: i32,
) -> *mut byte {
    let mut result: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut col1: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut col2: *mut byte = ::core::ptr::null_mut::<byte>();
    result = Z_Malloc(
        256 as i32 * 256 as i32,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    x = 0 as i32;
    while x < 256 as i32 {
        y = 0 as i32;
        while y < 256 as i32 {
            col1 = palette.offset((x * 3 as i32) as isize);
            col2 = palette.offset((y * 3 as i32) as isize);
            r = (*col1.offset(0 as i32 as isize) as i32
                * pct
                + *col2.offset(0 as i32 as isize) as i32
                    * (100 as i32 - pct)) / 100 as i32;
            g = (*col1.offset(1 as i32 as isize) as i32
                * pct
                + *col2.offset(1 as i32 as isize) as i32
                    * (100 as i32 - pct)) / 100 as i32;
            b = (*col1.offset(2 as i32 as isize) as i32
                * pct
                + *col2.offset(2 as i32 as isize) as i32
                    * (100 as i32 - pct)) / 100 as i32;
            *result.offset((x * 256 as i32 + y) as isize) = FindNearestColor(
                palette,
                r,
                g,
                b,
            ) as byte;
            y += 1;
        }
        x += 1;
    }
    return result;
}
unsafe extern "C" fn I_InitStretchTables(mut palette: *mut byte) {
    if !stretch_tables[0 as i32 as usize].is_null() {
        return;
    }
    printf(
        b"I_InitStretchTables: Generating lookup tables..\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fflush(stdout);
    stretch_tables[0 as i32 as usize] = GenerateStretchTable(
        palette,
        20 as i32,
    );
    printf(b"..\0" as *const u8 as *const ::core::ffi::c_char);
    fflush(stdout);
    stretch_tables[1 as i32 as usize] = GenerateStretchTable(
        palette,
        40 as i32,
    );
    puts(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn I_InitSquashTable(mut palette: *mut byte) {
    if !half_stretch_table.is_null() {
        return;
    }
    printf(
        b"I_InitSquashTable: Generating lookup table..\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fflush(stdout);
    half_stretch_table = GenerateStretchTable(palette, 50 as i32);
    puts(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn I_ResetScaleTables(mut palette: *mut byte) {
    if !stretch_tables[0 as i32 as usize].is_null() {
        Z_Free(
            stretch_tables[0 as i32 as usize] as *mut ::core::ffi::c_void,
        );
        Z_Free(
            stretch_tables[1 as i32 as usize] as *mut ::core::ffi::c_void,
        );
        printf(
            b"I_ResetScaleTables: Regenerating lookup tables..\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        stretch_tables[0 as i32 as usize] = GenerateStretchTable(
            palette,
            20 as i32,
        );
        stretch_tables[1 as i32 as usize] = GenerateStretchTable(
            palette,
            40 as i32,
        );
    }
    if !half_stretch_table.is_null() {
        Z_Free(half_stretch_table as *mut ::core::ffi::c_void);
        printf(
            b"I_ResetScaleTables: Regenerating lookup table..\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        half_stretch_table = GenerateStretchTable(palette, 50 as i32);
    }
}
#[inline]
unsafe extern "C" fn WriteBlendedLine1x(
    mut dest: *mut byte,
    mut src1: *mut byte,
    mut src2: *mut byte,
    mut stretch_table: *mut byte,
) {
    let mut x: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        *dest = *stretch_table
            .offset(
                (*src1 as i32 * 256 as i32
                    + *src2 as i32) as isize,
            );
        dest = dest.offset(1);
        src1 = src1.offset(1);
        src2 = src2.offset(1);
        x += 1;
    }
}
unsafe extern "C" fn I_Stretch1x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer.offset((y1 * dest_pitch) as isize).offset(x1 as isize);
    y = 0 as i32;
    while y < SCREENHEIGHT {
        memcpy(
            screenp as *mut ::core::ffi::c_void,
            bufp as *const ::core::ffi::c_void,
            SCREENWIDTH as size_t,
        );
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine1x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteBlendedLine1x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteBlendedLine1x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteBlendedLine1x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        memcpy(
            screenp as *mut ::core::ffi::c_void,
            bufp as *const ::core::ffi::c_void,
            SCREENWIDTH as size_t,
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 5 as i32;
    }
    return true;
}
#[no_mangle]
pub static mut mode_stretch_1x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH,
        height: SCREENHEIGHT_4_3,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Stretch1x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: true,
    }
};
#[inline]
unsafe extern "C" fn WriteLine2x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        *dest.offset(0 as i32 as isize) = *src;
        *dest.offset(1 as i32 as isize) = *src;
        dest = dest.offset(2 as i32 as isize);
        src = src.offset(1);
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn WriteBlendedLine2x(
    mut dest: *mut byte,
    mut src1: *mut byte,
    mut src2: *mut byte,
    mut stretch_table: *mut byte,
) {
    let mut x: i32 = 0;
    let mut val: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        val = *stretch_table
            .offset(
                (*src1 as i32 * 256 as i32
                    + *src2 as i32) as isize,
            ) as i32;
        *dest.offset(0 as i32 as isize) = val as byte;
        *dest.offset(1 as i32 as isize) = val as byte;
        dest = dest.offset(2 as i32 as isize);
        src1 = src1.offset(1);
        src2 = src2.offset(1);
        x += 1;
    }
}
unsafe extern "C" fn I_Stretch2x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer.offset((y1 * dest_pitch) as isize).offset(x1 as isize);
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine2x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine2x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine2x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine2x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine2x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 5 as i32;
    }
    return true;
}
#[no_mangle]
pub static mut mode_stretch_2x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 2 as i32,
        height: SCREENHEIGHT_4_3 * 2 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Stretch2x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteLine3x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        *dest.offset(0 as i32 as isize) = *src;
        *dest.offset(1 as i32 as isize) = *src;
        *dest.offset(2 as i32 as isize) = *src;
        dest = dest.offset(3 as i32 as isize);
        src = src.offset(1);
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn WriteBlendedLine3x(
    mut dest: *mut byte,
    mut src1: *mut byte,
    mut src2: *mut byte,
    mut stretch_table: *mut byte,
) {
    let mut x: i32 = 0;
    let mut val: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        val = *stretch_table
            .offset(
                (*src1 as i32 * 256 as i32
                    + *src2 as i32) as isize,
            ) as i32;
        *dest.offset(0 as i32 as isize) = val as byte;
        *dest.offset(1 as i32 as isize) = val as byte;
        *dest.offset(2 as i32 as isize) = val as byte;
        dest = dest.offset(3 as i32 as isize);
        src1 = src1.offset(1);
        src2 = src2.offset(1);
        x += 1;
    }
}
unsafe extern "C" fn I_Stretch3x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer.offset((y1 * dest_pitch) as isize).offset(x1 as isize);
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine3x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine3x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine3x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine3x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine3x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 5 as i32;
    }
    return true;
}
#[no_mangle]
pub static mut mode_stretch_3x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 3 as i32,
        height: SCREENHEIGHT_4_3 * 3 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Stretch3x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteLine4x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        *dest.offset(0 as i32 as isize) = *src;
        *dest.offset(1 as i32 as isize) = *src;
        *dest.offset(2 as i32 as isize) = *src;
        *dest.offset(3 as i32 as isize) = *src;
        dest = dest.offset(4 as i32 as isize);
        src = src.offset(1);
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn WriteBlendedLine4x(
    mut dest: *mut byte,
    mut src1: *mut byte,
    mut src2: *mut byte,
    mut stretch_table: *mut byte,
) {
    let mut x: i32 = 0;
    let mut val: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        val = *stretch_table
            .offset(
                (*src1 as i32 * 256 as i32
                    + *src2 as i32) as isize,
            ) as i32;
        *dest.offset(0 as i32 as isize) = val as byte;
        *dest.offset(1 as i32 as isize) = val as byte;
        *dest.offset(2 as i32 as isize) = val as byte;
        *dest.offset(3 as i32 as isize) = val as byte;
        dest = dest.offset(4 as i32 as isize);
        src1 = src1.offset(1);
        src2 = src2.offset(1);
        x += 1;
    }
}
unsafe extern "C" fn I_Stretch4x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer.offset((y1 * dest_pitch) as isize).offset(x1 as isize);
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine4x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine4x(
            screenp,
            bufp.offset(SCREENWIDTH as isize),
            bufp,
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine4x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[1 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteBlendedLine4x(
            screenp,
            bufp,
            bufp.offset(SCREENWIDTH as isize),
            stretch_tables[0 as i32 as usize],
        );
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine4x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 5 as i32;
    }
    return true;
}
#[no_mangle]
pub static mut mode_stretch_4x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 4 as i32,
        height: SCREENHEIGHT_4_3 * 4 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Stretch4x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteLine5x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        *dest.offset(0 as i32 as isize) = *src;
        *dest.offset(1 as i32 as isize) = *src;
        *dest.offset(2 as i32 as isize) = *src;
        *dest.offset(3 as i32 as isize) = *src;
        *dest.offset(4 as i32 as isize) = *src;
        dest = dest.offset(5 as i32 as isize);
        src = src.offset(1);
        x += 1;
    }
}
unsafe extern "C" fn I_Stretch5x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer.offset((y1 * SCREENWIDTH) as isize).offset(x1 as isize);
    screenp = dest_buffer.offset((y1 * dest_pitch) as isize).offset(x1 as isize);
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteLine5x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine5x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine5x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine5x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine5x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        WriteLine5x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1 as i32;
    }
    if M_CheckParm("-scanline") > 0 as i32 {
        screenp = dest_buffer.offset((2 as i32 * dest_pitch) as isize);
        y = 0 as i32;
        while y < 1198 as i32 {
            memset(
                screenp as *mut ::core::ffi::c_void,
                0 as i32,
                1600 as size_t,
            );
            screenp = screenp.offset((dest_pitch * 3 as i32) as isize);
            y += 3 as i32;
        }
    }
    return true;
}
#[no_mangle]
pub static mut mode_stretch_5x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH * 5 as i32,
        height: SCREENHEIGHT_4_3 * 5 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Stretch5x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteSquashedLine1x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    x = 0 as i32;
    while x < SCREENWIDTH {
        let fresh54 = dest;
        dest = dest.offset(1);
        *fresh54 = *stretch_tables[0 as i32 as usize]
            .offset(
                (*src.offset(1 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(0 as i32 as isize)
                        as i32) as isize,
            );
        let fresh55 = dest;
        dest = dest.offset(1);
        *fresh55 = *stretch_tables[1 as i32 as usize]
            .offset(
                (*src.offset(2 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(1 as i32 as isize)
                        as i32) as isize,
            );
        let fresh56 = dest;
        dest = dest.offset(1);
        *fresh56 = *stretch_tables[1 as i32 as usize]
            .offset(
                (*src.offset(2 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(3 as i32 as isize)
                        as i32) as isize,
            );
        let fresh57 = dest;
        dest = dest.offset(1);
        *fresh57 = *stretch_tables[0 as i32 as usize]
            .offset(
                (*src.offset(3 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(4 as i32 as isize)
                        as i32) as isize,
            );
        x += 5 as i32;
        src = src.offset(5 as i32 as isize);
    }
}
unsafe extern "C" fn I_Squash1x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer;
    screenp = dest_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteSquashedLine1x(screenp, bufp);
        screenp = screenp.offset(dest_pitch as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_squash_1x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH_4_3,
        height: SCREENHEIGHT,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Squash1x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: true,
    }
};
#[inline]
unsafe extern "C" fn WriteSquashedLine2x(mut dest: *mut byte, mut src: *mut byte) {
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut c: i32 = 0;
    dest2 = dest.offset(dest_pitch as isize);
    x = 0 as i32;
    while x < SCREENWIDTH {
        c = *src.offset(0 as i32 as isize) as i32;
        let fresh58 = dest2;
        dest2 = dest2.offset(1);
        *fresh58 = c as byte;
        let fresh59 = dest;
        dest = dest.offset(1);
        *fresh59 = *fresh58;
        c = *stretch_tables[1 as i32 as usize]
            .offset(
                (*src.offset(1 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(0 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh60 = dest2;
        dest2 = dest2.offset(1);
        *fresh60 = c as byte;
        let fresh61 = dest;
        dest = dest.offset(1);
        *fresh61 = *fresh60;
        c = *src.offset(1 as i32 as isize) as i32;
        let fresh62 = dest2;
        dest2 = dest2.offset(1);
        *fresh62 = c as byte;
        let fresh63 = dest;
        dest = dest.offset(1);
        *fresh63 = *fresh62;
        c = *stretch_tables[0 as i32 as usize]
            .offset(
                (*src.offset(1 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(2 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh64 = dest2;
        dest2 = dest2.offset(1);
        *fresh64 = c as byte;
        let fresh65 = dest;
        dest = dest.offset(1);
        *fresh65 = *fresh64;
        c = *stretch_tables[0 as i32 as usize]
            .offset(
                (*src.offset(3 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(2 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh66 = dest2;
        dest2 = dest2.offset(1);
        *fresh66 = c as byte;
        let fresh67 = dest;
        dest = dest.offset(1);
        *fresh67 = *fresh66;
        c = *src.offset(3 as i32 as isize) as i32;
        let fresh68 = dest2;
        dest2 = dest2.offset(1);
        *fresh68 = c as byte;
        let fresh69 = dest;
        dest = dest.offset(1);
        *fresh69 = *fresh68;
        c = *stretch_tables[1 as i32 as usize]
            .offset(
                (*src.offset(3 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(4 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh70 = dest2;
        dest2 = dest2.offset(1);
        *fresh70 = c as byte;
        let fresh71 = dest;
        dest = dest.offset(1);
        *fresh71 = *fresh70;
        c = *src.offset(4 as i32 as isize) as i32;
        let fresh72 = dest2;
        dest2 = dest2.offset(1);
        *fresh72 = c as byte;
        let fresh73 = dest;
        dest = dest.offset(1);
        *fresh73 = *fresh72;
        x += 5 as i32;
        src = src.offset(5 as i32 as isize);
    }
}
unsafe extern "C" fn I_Squash2x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer;
    screenp = dest_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteSquashedLine2x(screenp, bufp);
        screenp = screenp.offset((dest_pitch * 2 as i32) as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_squash_2x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH_4_3 * 2 as i32,
        height: SCREENHEIGHT * 2 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Squash2x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteSquashedLine3x(mut dest: *mut byte, mut src: *mut byte) {
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest3: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut c: i32 = 0;
    dest2 = dest.offset(dest_pitch as isize);
    dest3 = dest.offset((dest_pitch * 2 as i32) as isize);
    x = 0 as i32;
    while x < SCREENWIDTH {
        c = *src.offset(0 as i32 as isize) as i32;
        let fresh74 = dest3;
        dest3 = dest3.offset(1);
        *fresh74 = c as byte;
        let fresh75 = dest2;
        dest2 = dest2.offset(1);
        *fresh75 = *fresh74;
        let fresh76 = dest;
        dest = dest.offset(1);
        *fresh76 = *fresh75;
        let fresh77 = dest3;
        dest3 = dest3.offset(1);
        *fresh77 = c as byte;
        let fresh78 = dest2;
        dest2 = dest2.offset(1);
        *fresh78 = *fresh77;
        let fresh79 = dest;
        dest = dest.offset(1);
        *fresh79 = *fresh78;
        c = *half_stretch_table
            .offset(
                (*src.offset(0 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(1 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh80 = dest3;
        dest3 = dest3.offset(1);
        *fresh80 = c as byte;
        let fresh81 = dest2;
        dest2 = dest2.offset(1);
        *fresh81 = *fresh80;
        let fresh82 = dest;
        dest = dest.offset(1);
        *fresh82 = *fresh81;
        c = *src.offset(1 as i32 as isize) as i32;
        let fresh83 = dest3;
        dest3 = dest3.offset(1);
        *fresh83 = c as byte;
        let fresh84 = dest2;
        dest2 = dest2.offset(1);
        *fresh84 = *fresh83;
        let fresh85 = dest;
        dest = dest.offset(1);
        *fresh85 = *fresh84;
        let fresh86 = dest3;
        dest3 = dest3.offset(1);
        *fresh86 = c as byte;
        let fresh87 = dest2;
        dest2 = dest2.offset(1);
        *fresh87 = *fresh86;
        let fresh88 = dest;
        dest = dest.offset(1);
        *fresh88 = *fresh87;
        x += 2 as i32;
        src = src.offset(2 as i32 as isize);
    }
}
unsafe extern "C" fn I_Squash3x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer;
    screenp = dest_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteSquashedLine3x(screenp, bufp);
        screenp = screenp.offset((dest_pitch * 3 as i32) as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_squash_3x: screen_mode_t = unsafe {
    screen_mode_t {
        width: 800 as i32,
        height: 600 as i32,
        InitMode: Some(I_InitSquashTable as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Squash3x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteSquashedLine4x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    let mut c: i32 = 0;
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest3: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest4: *mut byte = ::core::ptr::null_mut::<byte>();
    dest2 = dest.offset(dest_pitch as isize);
    dest3 = dest.offset((dest_pitch * 2 as i32) as isize);
    dest4 = dest.offset((dest_pitch * 3 as i32) as isize);
    x = 0 as i32;
    while x < SCREENWIDTH {
        c = *src.offset(0 as i32 as isize) as i32;
        let fresh89 = dest4;
        dest4 = dest4.offset(1);
        *fresh89 = c as byte;
        let fresh90 = dest3;
        dest3 = dest3.offset(1);
        *fresh90 = *fresh89;
        let fresh91 = dest2;
        dest2 = dest2.offset(1);
        *fresh91 = *fresh90;
        let fresh92 = dest;
        dest = dest.offset(1);
        *fresh92 = *fresh91;
        let fresh93 = dest4;
        dest4 = dest4.offset(1);
        *fresh93 = c as byte;
        let fresh94 = dest3;
        dest3 = dest3.offset(1);
        *fresh94 = *fresh93;
        let fresh95 = dest2;
        dest2 = dest2.offset(1);
        *fresh95 = *fresh94;
        let fresh96 = dest;
        dest = dest.offset(1);
        *fresh96 = *fresh95;
        let fresh97 = dest4;
        dest4 = dest4.offset(1);
        *fresh97 = c as byte;
        let fresh98 = dest3;
        dest3 = dest3.offset(1);
        *fresh98 = *fresh97;
        let fresh99 = dest2;
        dest2 = dest2.offset(1);
        *fresh99 = *fresh98;
        let fresh100 = dest;
        dest = dest.offset(1);
        *fresh100 = *fresh99;
        c = *stretch_tables[0 as i32 as usize]
            .offset(
                (*src.offset(0 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(1 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh101 = dest4;
        dest4 = dest4.offset(1);
        *fresh101 = c as byte;
        let fresh102 = dest3;
        dest3 = dest3.offset(1);
        *fresh102 = *fresh101;
        let fresh103 = dest2;
        dest2 = dest2.offset(1);
        *fresh103 = *fresh102;
        let fresh104 = dest;
        dest = dest.offset(1);
        *fresh104 = *fresh103;
        c = *src.offset(1 as i32 as isize) as i32;
        let fresh105 = dest4;
        dest4 = dest4.offset(1);
        *fresh105 = c as byte;
        let fresh106 = dest3;
        dest3 = dest3.offset(1);
        *fresh106 = *fresh105;
        let fresh107 = dest2;
        dest2 = dest2.offset(1);
        *fresh107 = *fresh106;
        let fresh108 = dest;
        dest = dest.offset(1);
        *fresh108 = *fresh107;
        let fresh109 = dest4;
        dest4 = dest4.offset(1);
        *fresh109 = c as byte;
        let fresh110 = dest3;
        dest3 = dest3.offset(1);
        *fresh110 = *fresh109;
        let fresh111 = dest2;
        dest2 = dest2.offset(1);
        *fresh111 = *fresh110;
        let fresh112 = dest;
        dest = dest.offset(1);
        *fresh112 = *fresh111;
        c = *stretch_tables[1 as i32 as usize]
            .offset(
                (*src.offset(1 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(2 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh113 = dest4;
        dest4 = dest4.offset(1);
        *fresh113 = c as byte;
        let fresh114 = dest3;
        dest3 = dest3.offset(1);
        *fresh114 = *fresh113;
        let fresh115 = dest2;
        dest2 = dest2.offset(1);
        *fresh115 = *fresh114;
        let fresh116 = dest;
        dest = dest.offset(1);
        *fresh116 = *fresh115;
        c = *src.offset(2 as i32 as isize) as i32;
        let fresh117 = dest4;
        dest4 = dest4.offset(1);
        *fresh117 = c as byte;
        let fresh118 = dest3;
        dest3 = dest3.offset(1);
        *fresh118 = *fresh117;
        let fresh119 = dest2;
        dest2 = dest2.offset(1);
        *fresh119 = *fresh118;
        let fresh120 = dest;
        dest = dest.offset(1);
        *fresh120 = *fresh119;
        let fresh121 = dest4;
        dest4 = dest4.offset(1);
        *fresh121 = c as byte;
        let fresh122 = dest3;
        dest3 = dest3.offset(1);
        *fresh122 = *fresh121;
        let fresh123 = dest2;
        dest2 = dest2.offset(1);
        *fresh123 = *fresh122;
        let fresh124 = dest;
        dest = dest.offset(1);
        *fresh124 = *fresh123;
        c = *stretch_tables[1 as i32 as usize]
            .offset(
                (*src.offset(3 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(2 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh125 = dest4;
        dest4 = dest4.offset(1);
        *fresh125 = c as byte;
        let fresh126 = dest3;
        dest3 = dest3.offset(1);
        *fresh126 = *fresh125;
        let fresh127 = dest2;
        dest2 = dest2.offset(1);
        *fresh127 = *fresh126;
        let fresh128 = dest;
        dest = dest.offset(1);
        *fresh128 = *fresh127;
        c = *src.offset(3 as i32 as isize) as i32;
        let fresh129 = dest4;
        dest4 = dest4.offset(1);
        *fresh129 = c as byte;
        let fresh130 = dest3;
        dest3 = dest3.offset(1);
        *fresh130 = *fresh129;
        let fresh131 = dest2;
        dest2 = dest2.offset(1);
        *fresh131 = *fresh130;
        let fresh132 = dest;
        dest = dest.offset(1);
        *fresh132 = *fresh131;
        let fresh133 = dest4;
        dest4 = dest4.offset(1);
        *fresh133 = c as byte;
        let fresh134 = dest3;
        dest3 = dest3.offset(1);
        *fresh134 = *fresh133;
        let fresh135 = dest2;
        dest2 = dest2.offset(1);
        *fresh135 = *fresh134;
        let fresh136 = dest;
        dest = dest.offset(1);
        *fresh136 = *fresh135;
        c = *stretch_tables[0 as i32 as usize]
            .offset(
                (*src.offset(4 as i32 as isize) as i32
                    * 256 as i32
                    + *src.offset(3 as i32 as isize)
                        as i32) as isize,
            ) as i32;
        let fresh137 = dest4;
        dest4 = dest4.offset(1);
        *fresh137 = c as byte;
        let fresh138 = dest3;
        dest3 = dest3.offset(1);
        *fresh138 = *fresh137;
        let fresh139 = dest2;
        dest2 = dest2.offset(1);
        *fresh139 = *fresh138;
        let fresh140 = dest;
        dest = dest.offset(1);
        *fresh140 = *fresh139;
        c = *src.offset(4 as i32 as isize) as i32;
        let fresh141 = dest4;
        dest4 = dest4.offset(1);
        *fresh141 = c as byte;
        let fresh142 = dest3;
        dest3 = dest3.offset(1);
        *fresh142 = *fresh141;
        let fresh143 = dest2;
        dest2 = dest2.offset(1);
        *fresh143 = *fresh142;
        let fresh144 = dest;
        dest = dest.offset(1);
        *fresh144 = *fresh143;
        let fresh145 = dest4;
        dest4 = dest4.offset(1);
        *fresh145 = c as byte;
        let fresh146 = dest3;
        dest3 = dest3.offset(1);
        *fresh146 = *fresh145;
        let fresh147 = dest2;
        dest2 = dest2.offset(1);
        *fresh147 = *fresh146;
        let fresh148 = dest;
        dest = dest.offset(1);
        *fresh148 = *fresh147;
        let fresh149 = dest4;
        dest4 = dest4.offset(1);
        *fresh149 = c as byte;
        let fresh150 = dest3;
        dest3 = dest3.offset(1);
        *fresh150 = *fresh149;
        let fresh151 = dest2;
        dest2 = dest2.offset(1);
        *fresh151 = *fresh150;
        let fresh152 = dest;
        dest = dest.offset(1);
        *fresh152 = *fresh151;
        x += 5 as i32;
        src = src.offset(5 as i32 as isize);
    }
}
unsafe extern "C" fn I_Squash4x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer;
    screenp = dest_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteSquashedLine4x(screenp, bufp);
        screenp = screenp.offset((dest_pitch * 4 as i32) as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_squash_4x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH_4_3 * 4 as i32,
        height: SCREENHEIGHT * 4 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Squash4x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
#[inline]
unsafe extern "C" fn WriteSquashedLine5x(mut dest: *mut byte, mut src: *mut byte) {
    let mut x: i32 = 0;
    let mut c: i32 = 0;
    let mut dest2: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest3: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest4: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest5: *mut byte = ::core::ptr::null_mut::<byte>();
    dest2 = dest.offset(dest_pitch as isize);
    dest3 = dest.offset((dest_pitch * 2 as i32) as isize);
    dest4 = dest.offset((dest_pitch * 3 as i32) as isize);
    dest5 = dest.offset((dest_pitch * 4 as i32) as isize);
    x = 0 as i32;
    while x < SCREENWIDTH {
        let fresh153 = src;
        src = src.offset(1);
        c = *fresh153 as i32;
        let fresh154 = dest5;
        dest5 = dest5.offset(1);
        *fresh154 = c as byte;
        let fresh155 = dest4;
        dest4 = dest4.offset(1);
        *fresh155 = *fresh154;
        let fresh156 = dest3;
        dest3 = dest3.offset(1);
        *fresh156 = *fresh155;
        let fresh157 = dest2;
        dest2 = dest2.offset(1);
        *fresh157 = *fresh156;
        let fresh158 = dest;
        dest = dest.offset(1);
        *fresh158 = *fresh157;
        let fresh159 = dest5;
        dest5 = dest5.offset(1);
        *fresh159 = c as byte;
        let fresh160 = dest4;
        dest4 = dest4.offset(1);
        *fresh160 = *fresh159;
        let fresh161 = dest3;
        dest3 = dest3.offset(1);
        *fresh161 = *fresh160;
        let fresh162 = dest2;
        dest2 = dest2.offset(1);
        *fresh162 = *fresh161;
        let fresh163 = dest;
        dest = dest.offset(1);
        *fresh163 = *fresh162;
        let fresh164 = dest5;
        dest5 = dest5.offset(1);
        *fresh164 = c as byte;
        let fresh165 = dest4;
        dest4 = dest4.offset(1);
        *fresh165 = *fresh164;
        let fresh166 = dest3;
        dest3 = dest3.offset(1);
        *fresh166 = *fresh165;
        let fresh167 = dest2;
        dest2 = dest2.offset(1);
        *fresh167 = *fresh166;
        let fresh168 = dest;
        dest = dest.offset(1);
        *fresh168 = *fresh167;
        let fresh169 = dest5;
        dest5 = dest5.offset(1);
        *fresh169 = c as byte;
        let fresh170 = dest4;
        dest4 = dest4.offset(1);
        *fresh170 = *fresh169;
        let fresh171 = dest3;
        dest3 = dest3.offset(1);
        *fresh171 = *fresh170;
        let fresh172 = dest2;
        dest2 = dest2.offset(1);
        *fresh172 = *fresh171;
        let fresh173 = dest;
        dest = dest.offset(1);
        *fresh173 = *fresh172;
        x += 1;
    }
}
unsafe extern "C" fn I_Squash5x(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> bool {
    let mut bufp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut screenp: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut y: i32 = 0;
    if x1 != 0 as i32 || y1 != 0 as i32
        || x2 != SCREENWIDTH || y2 != SCREENHEIGHT
    {
        return false;
    }
    bufp = src_buffer;
    screenp = dest_buffer;
    y = 0 as i32;
    while y < SCREENHEIGHT {
        WriteSquashedLine5x(screenp, bufp);
        screenp = screenp.offset((dest_pitch * 5 as i32) as isize);
        bufp = bufp.offset(SCREENWIDTH as isize);
        y += 1;
    }
    return true;
}
#[no_mangle]
pub static mut mode_squash_5x: screen_mode_t = unsafe {
    screen_mode_t {
        width: SCREENWIDTH_4_3 * 5 as i32,
        height: SCREENHEIGHT * 5 as i32,
        InitMode: Some(I_InitStretchTables as unsafe extern "C" fn(*mut byte) -> ()),
        DrawScreen: Some(
            I_Squash5x
                as unsafe extern "C" fn(
                    i32,
                    i32,
                    i32,
                    i32,
                ) -> bool,
        ),
        poor_quality: false,
    }
};
pub const __INT_MAX__: i32 = 2147483647 as i32;
