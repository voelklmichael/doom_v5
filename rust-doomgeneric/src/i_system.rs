use crate::src::m_argv::{myargv, M_CheckParmWithArgs, M_ParmExists};
use crate::src::m_misc::M_StrToInt;
use crate::src::m_misc::M_snprintf;

extern "C" {
    pub type FILE;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> i32;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: i32) -> !;
    fn system(__command: *const ::core::ffi::c_char) -> i32;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> i32;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> i32;
    fn putchar(__c: i32) -> i32;
    fn puts(__s: *const ::core::ffi::c_char) -> i32;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: i32,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: i32,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> i32;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
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
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const DEFAULT_RAM: i32 = 6 as i32;
pub const MIN_RAM: i32 = 6 as i32;
static mut exit_funcs: *mut atexit_listentry_t = ::core::ptr::null::<
    atexit_listentry_t,
>() as *mut atexit_listentry_t;
pub unsafe fn I_AtExit(mut func: atexit_func_t, mut run_on_error: boolean) {
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
pub unsafe fn I_Tactile(
    mut on: i32,
    mut off: i32,
    mut total: i32,
) {}
unsafe extern "C" fn AutoAllocMemory(
    mut size: *mut i32,
    mut default_ram: i32,
    mut min_ram: i32,
) -> *mut byte {
    let mut zonemem: *mut byte = ::core::ptr::null_mut::<byte>();
    zonemem = ::core::ptr::null_mut::<byte>();
    while zonemem.is_null() {
        if default_ram < min_ram {
            I_Error(&format!("Unable to allocate {} MiB of RAM for zone", default_ram));
        }
        *size = default_ram * 1024 as i32 * 1024 as i32;
        zonemem = malloc(*size as size_t) as *mut byte;
        if zonemem.is_null() {
            default_ram -= 1 as i32;
        }
    }
    return zonemem;
}
pub unsafe fn I_ZoneBase(mut size: *mut i32) -> *mut byte {
    let mut zonemem: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut min_ram: i32 = 0;
    let mut default_ram: i32 = 0;
    let mut p: i32 = 0;
    p = M_CheckParmWithArgs("-mb", 1 as i32);
    if p > 0 as i32 {
        default_ram = atoi(
            myargv[(p + 1 as i32) as usize].as_ptr()
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
pub unsafe fn I_PrintBanner(mut msg: *mut ::core::ffi::c_char) {
    let mut i: i32 = 0;
    let mut spaces: i32 = (35 as size_t)
        .wrapping_sub(strlen(msg).wrapping_div(2 as size_t)) as i32;
    i = 0 as i32;
    while i < spaces {
        putchar(' ' as i32);
        i += 1;
    }
    puts(msg);
}
pub unsafe fn I_PrintDivider() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 75 as i32 {
        putchar('=' as i32);
        i += 1;
    }
    putchar('\n' as i32);
}
pub unsafe fn I_PrintStartupBanner(
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
pub unsafe fn I_ConsoleStdout() -> boolean {
    return 0 as boolean;
}
pub unsafe fn I_Quit() {
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
unsafe extern "C" fn ZenityAvailable() -> i32 {
    return (system(
        b"/usr/bin/zenity --help >/dev/null 2>&1\0" as *const u8
            as *const ::core::ffi::c_char,
    ) == 0 as i32) as i32;
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
    while *s as i32 != '\0' as i32 {
        if !strchr(
                b"$`\\!\0" as *const u8 as *const ::core::ffi::c_char,
                *s as i32,
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
) -> i32 {
    let mut result: i32 = 0;
    let mut escaped_message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut errorboxpath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    static mut errorboxpath_size: size_t = 0;
    if ZenityAvailable() == 0 {
        return 0 as i32;
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
    exit_gui_popup = (!M_ParmExists("-nogui")) as i32 as boolean;
    if exit_gui_popup != 0 && I_ConsoleStdout() == 0 {
        ZenityErrorBox(message_cstring.as_ptr() as *mut ::core::ffi::c_char);
    }
    exit(-(1 as i32));
}
pub const DOS_MEM_DUMP_SIZE: i32 = 10 as i32;
static mut mem_dump_dos622: [u8; 10] = [
    0x57 as i32 as u8,
    0x92 as i32 as u8,
    0x19 as i32 as u8,
    0 as i32 as u8,
    0xf4 as i32 as u8,
    0x6 as i32 as u8,
    0x70 as i32 as u8,
    0 as i32 as u8,
    0x16 as i32 as u8,
    0 as i32 as u8,
];
static mut mem_dump_win98: [u8; 10] = [
    0x9e as i32 as u8,
    0xf as i32 as u8,
    0xc9 as i32 as u8,
    0 as i32 as u8,
    0x65 as i32 as u8,
    0x4 as i32 as u8,
    0x70 as i32 as u8,
    0 as i32 as u8,
    0x16 as i32 as u8,
    0 as i32 as u8,
];
static mut mem_dump_dosbox: [u8; 10] = [
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xf1 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x7 as i32 as u8,
    0 as i32 as u8,
];
static mut mem_dump_custom: [u8; 10] = [0; 10];
static mut dos_mem_dump: *const u8 = unsafe {
    &raw const mem_dump_dos622 as *const u8
};
pub unsafe fn I_GetMemoryValue(
    mut offset: u32,
    mut value: *mut ::core::ffi::c_void,
    mut size: i32,
) -> boolean {
    static mut firsttime: bool = true;
    if firsttime {
        let mut p: i32 = 0;
        let mut i: i32 = 0;
        let mut val: i32 = 0;
        firsttime = false;
        i = 0 as i32;
        p = M_CheckParmWithArgs("-setmem", 1 as i32);
        if p > 0 as i32 {
            if strcasecmp(
                myargv[(p + 1 as i32) as usize].as_ptr(),
                b"dos622\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                dos_mem_dump = &raw const mem_dump_dos622 as *const u8;
            }
            if strcasecmp(
                myargv[(p + 1 as i32) as usize].as_ptr(),
                b"dos71\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                dos_mem_dump = &raw const mem_dump_win98 as *const u8;
            } else if strcasecmp(
                myargv[(p + 1 as i32) as usize].as_ptr(),
                b"dosbox\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                dos_mem_dump = &raw const mem_dump_dosbox as *const u8;
            } else {
                i = 0 as i32;
                while i < DOS_MEM_DUMP_SIZE {
                    p += 1;
                    if p >= myargv.len() as i32
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
                    mem_dump_custom[fresh0 as usize] = val as u8;
                    i += 1;
                }
                dos_mem_dump = &raw mut mem_dump_custom as *mut u8;
            }
        }
    }
    match size {
        1 => {
            *(value as *mut u8) = *dos_mem_dump
                .offset(offset as isize);
            return true_0 as boolean;
        }
        2 => {
            *(value as *mut u16) = (*dos_mem_dump
                .offset(offset as isize) as i32
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(1 as u32) as isize)
                    as i32) << 8 as i32)
                as u16;
            return true_0 as boolean;
        }
        4 => {
            *(value as *mut u32) = (*dos_mem_dump.offset(offset as isize)
                as i32
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(1 as u32) as isize)
                    as i32) << 8 as i32
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(2 as u32) as isize)
                    as i32) << 16 as i32
                | (*dos_mem_dump
                    .offset(offset.wrapping_add(3 as u32) as isize)
                    as i32) << 24 as i32)
                as u32;
            return true_0 as boolean;
        }
        _ => {}
    }
    return false_0 as boolean;
}
