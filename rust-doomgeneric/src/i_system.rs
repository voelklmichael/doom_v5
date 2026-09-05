use crate::src::m_argv::{myargv, M_CheckParmWithArgs, M_ParmExists};
extern "C" {
    pub type FILE;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn system(__command: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn putchar(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn M_StrToInt(
        str: *const ::core::ffi::c_char,
        result: *mut ::core::ffi::c_int,
    ) -> boolean;
    fn M_vsnprintf(
        buf: *mut ::core::ffi::c_char,
        buf_len: size_t,
        s: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn M_snprintf(
        buf: *mut ::core::ffi::c_char,
        buf_len: size_t,
        s: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type boolean = ::core::ffi::c_uint;
pub type byte = uint8_t;
pub type atexit_func_t = Option<unsafe extern "C" fn() -> ()>;
pub type atexit_listentry_t = atexit_listentry_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atexit_listentry_s {
    pub func: atexit_func_t,
    pub run_on_error: boolean,
    pub next: *mut atexit_listentry_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEFAULT_RAM: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MIN_RAM: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut exit_funcs: *mut atexit_listentry_t = ::core::ptr::null::<
    atexit_listentry_t,
>() as *mut atexit_listentry_t;
#[no_mangle]
pub unsafe extern "C" fn I_AtExit(mut func: atexit_func_t, mut run_on_error: boolean) {
    let mut entry: *mut atexit_listentry_t = ::core::ptr::null_mut::<
        atexit_listentry_t,
    >();
    entry = malloc(::core::mem::size_of::<atexit_listentry_t>() as size_t)
        as *mut atexit_listentry_t;
    (*entry).func = func;
    (*entry).run_on_error = run_on_error;
    (*entry).next = exit_funcs;
    exit_funcs = entry;
}
#[no_mangle]
pub unsafe extern "C" fn I_Tactile(
    mut on: ::core::ffi::c_int,
    mut off: ::core::ffi::c_int,
    mut total: ::core::ffi::c_int,
) {}
unsafe extern "C" fn AutoAllocMemory(
    mut size: *mut ::core::ffi::c_int,
    mut default_ram: ::core::ffi::c_int,
    mut min_ram: ::core::ffi::c_int,
) -> *mut byte {
    let mut zonemem: *mut byte = ::core::ptr::null_mut::<byte>();
    zonemem = ::core::ptr::null_mut::<byte>();
    while zonemem.is_null() {
        if default_ram < min_ram {
            I_Error(&format!("Unable to allocate {} MiB of RAM for zone", default_ram));
        }
        *size = default_ram * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
        zonemem = malloc(*size as size_t) as *mut byte;
        if zonemem.is_null() {
            default_ram -= 1 as ::core::ffi::c_int;
        }
    }
    return zonemem;
}
#[no_mangle]
pub unsafe extern "C" fn I_ZoneBase(mut size: *mut ::core::ffi::c_int) -> *mut byte {
    let mut zonemem: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut min_ram: ::core::ffi::c_int = 0;
    let mut default_ram: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    p = M_CheckParmWithArgs("-mb", 1 as ::core::ffi::c_int);
    if p > 0 as ::core::ffi::c_int {
        default_ram = atoi(
            myargv[(p + 1 as ::core::ffi::c_int) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        );
        min_ram = default_ram;
    } else {
        default_ram = DEFAULT_RAM;
        min_ram = MIN_RAM;
    }
    zonemem = AutoAllocMemory(size, default_ram, min_ram);
    printf(
        b"zone memory: %p, %x allocated for zone\n\0" as *const u8
            as *const ::core::ffi::c_char,
        zonemem,
        *size,
    );
    return zonemem;
}
#[no_mangle]
pub unsafe extern "C" fn I_PrintBanner(mut msg: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut spaces: ::core::ffi::c_int = (35 as size_t)
        .wrapping_sub(strlen(msg).wrapping_div(2 as size_t)) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < spaces {
        putchar(' ' as i32);
        i += 1;
    }
    puts(msg);
}
#[no_mangle]
pub unsafe extern "C" fn I_PrintDivider() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 75 as ::core::ffi::c_int {
        putchar('=' as i32);
        i += 1;
    }
    putchar('\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn I_PrintStartupBanner(
    mut gamedescription: *mut ::core::ffi::c_char,
) {
    I_PrintDivider();
    I_PrintBanner(gamedescription);
    I_PrintDivider();
    printf(
        b" Doom Generic is free software, covered by the GNU General Public\n License.  There is NO warranty; not even for MERCHANTABILITY or FITNESS\n FOR A PARTICULAR PURPOSE. You are welcome to change and distribute\n copies under certain conditions. See the source for more information.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    I_PrintDivider();
}
#[no_mangle]
pub unsafe extern "C" fn I_ConsoleStdout() -> boolean {
    return 0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn I_Quit() {
    let mut entry: *mut atexit_listentry_t = ::core::ptr::null_mut::<
        atexit_listentry_t,
    >();
    entry = exit_funcs;
    while !entry.is_null() {
        (*entry).func.expect("non-null function pointer")();
        entry = (*entry).next;
    }
}
pub const ZENITY_BINARY: [::core::ffi::c_char; 16] = unsafe {
    ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"/usr/bin/zenity\0")
};
unsafe extern "C" fn ZenityAvailable() -> ::core::ffi::c_int {
    return (system(
        b"/usr/bin/zenity --help >/dev/null 2>&1\0" as *const u8
            as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn EscapeShellString(
    mut string: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    result = malloc(strlen(string).wrapping_mul(2 as size_t).wrapping_add(3 as size_t))
        as *mut ::core::ffi::c_char;
    r = result;
    *r = '"' as i32 as ::core::ffi::c_char;
    r = r.offset(1);
    s = string;
    while *s as ::core::ffi::c_int != '\0' as i32 {
        if !strchr(
                b"$`\\!\0" as *const u8 as *const ::core::ffi::c_char,
                *s as ::core::ffi::c_int,
            )
            .is_null()
        {
            *r = '\\' as i32 as ::core::ffi::c_char;
            r = r.offset(1);
        }
        *r = *s;
        r = r.offset(1);
        s = s.offset(1);
    }
    *r = '"' as i32 as ::core::ffi::c_char;
    r = r.offset(1);
    *r = '\0' as i32 as ::core::ffi::c_char;
    return result;
}
unsafe extern "C" fn ZenityErrorBox(
    mut message: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut escaped_message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut errorboxpath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    static mut errorboxpath_size: size_t = 0;
    if ZenityAvailable() == 0 {
        return 0 as ::core::ffi::c_int;
    }
    escaped_message = EscapeShellString(message);
    errorboxpath_size = strlen(ZENITY_BINARY.as_ptr())
        .wrapping_add(strlen(escaped_message))
        .wrapping_add(19 as size_t);
    errorboxpath = malloc(errorboxpath_size) as *mut ::core::ffi::c_char;
    M_snprintf(
        errorboxpath,
        errorboxpath_size,
        b"%s --error --text=%s\0" as *const u8 as *const ::core::ffi::c_char,
        ZENITY_BINARY.as_ptr(),
        escaped_message,
    );
    result = system(errorboxpath);
    free(errorboxpath as *mut ::core::ffi::c_void);
    free(escaped_message as *mut ::core::ffi::c_void);
    return result;
}
static mut already_quitting: bool = false;
pub unsafe fn I_Error(message: &str) {
    let mut entry: *mut atexit_listentry_t = ::core::ptr::null_mut::<
        atexit_listentry_t,
    >();
    let mut exit_gui_popup: boolean = 0;
    if already_quitting {
        fprintf(
            stderr,
            b"Warning: recursive call to I_Error detected.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        already_quitting = true;
    }
    let message_cstring = ::std::ffi::CString::new(message)
        .unwrap_or_else(|_| ::std::ffi::CString::new("(error message contains NUL)").unwrap());
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        message_cstring.as_ptr(),
    );
    fprintf(stderr, b"\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    fflush(stderr);
    entry = exit_funcs;
    while !entry.is_null() {
        if (*entry).run_on_error != 0 {
            (*entry).func.expect("non-null function pointer")();
        }
        entry = (*entry).next;
    }
    exit_gui_popup = (!M_ParmExists("-nogui")) as ::core::ffi::c_int as boolean;
    if exit_gui_popup != 0 && I_ConsoleStdout() == 0 {
        ZenityErrorBox(message_cstring.as_ptr() as *mut ::core::ffi::c_char);
    }
    exit(-(1 as ::core::ffi::c_int));
}
pub const DOS_MEM_DUMP_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut mem_dump_dos622: [::core::ffi::c_uchar; 10] = [
    0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut mem_dump_win98: [::core::ffi::c_uchar; 10] = [
    0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut mem_dump_dosbox: [::core::ffi::c_uchar; 10] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut mem_dump_custom: [::core::ffi::c_uchar; 10] = [0; 10];
static mut dos_mem_dump: *const ::core::ffi::c_uchar = unsafe {
    &raw const mem_dump_dos622 as *const ::core::ffi::c_uchar
};
#[no_mangle]
pub unsafe extern "C" fn I_GetMemoryValue(
    mut offset: ::core::ffi::c_uint,
    mut value: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> boolean {
    static mut firsttime: bool = true;
    if firsttime {
        let mut p: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut val: ::core::ffi::c_int = 0;
        firsttime = false;
        i = 0 as ::core::ffi::c_int;
        p = M_CheckParmWithArgs("-setmem", 1 as ::core::ffi::c_int);
        if p > 0 as ::core::ffi::c_int {
            if strcasecmp(
                myargv[(p + 1 as ::core::ffi::c_int) as usize].as_ptr(),
                b"dos622\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                dos_mem_dump = &raw const mem_dump_dos622 as *const ::core::ffi::c_uchar;
            }
            if strcasecmp(
                myargv[(p + 1 as ::core::ffi::c_int) as usize].as_ptr(),
                b"dos71\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                dos_mem_dump = &raw const mem_dump_win98 as *const ::core::ffi::c_uchar;
            } else if strcasecmp(
                myargv[(p + 1 as ::core::ffi::c_int) as usize].as_ptr(),
                b"dosbox\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                dos_mem_dump = &raw const mem_dump_dosbox as *const ::core::ffi::c_uchar;
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < DOS_MEM_DUMP_SIZE {
                    p += 1;
                    if p >= myargv.len() as ::core::ffi::c_int
                        || myargv[p as usize].as_bytes().first() == Some(&b'-')
                    {
                        break;
                    }
                    M_StrToInt(
                        myargv[p as usize].as_ptr() as *mut ::core::ffi::c_char,
                        &raw mut val,
                    );
                    let fresh0 = i;
                    i = i + 1;
                    mem_dump_custom[fresh0 as usize] = val as ::core::ffi::c_uchar;
                    i += 1;
                }
                dos_mem_dump = &raw mut mem_dump_custom as *mut ::core::ffi::c_uchar;
            }
        }
    }
    match size {
        1 => {
            *(value as *mut ::core::ffi::c_uchar) = *dos_mem_dump
                .offset(offset as isize);
            return true_0 as boolean;
        }
        2 => {
            *(value as *mut ::core::ffi::c_ushort) = (*dos_mem_dump
                .offset(offset as isize) as ::core::ffi::c_int
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_ushort;
            return true_0 as boolean;
        }
        4 => {
            *(value as *mut ::core::ffi::c_uint) = (*dos_mem_dump.offset(offset as isize)
                as ::core::ffi::c_int
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(2 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(3 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_int) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
            return true_0 as boolean;
        }
        _ => {}
    }
    return false_0 as boolean;
}
