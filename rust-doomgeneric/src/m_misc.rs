extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn toupper(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn I_Error(error: *mut ::core::ffi::c_char, ...);
    fn Z_Malloc(
        size: ::core::ffi::c_int,
        tag: ::core::ffi::c_int,
        ptr: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
pub type uint8_t = __uint8_t;
pub type boolean = ::core::ffi::c_uint;
pub type byte = uint8_t;
pub const PU_STATIC: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const PU_NUM_TAGS: C2RustUnnamed = 9;
pub const PU_CACHE: C2RustUnnamed = 8;
pub const PU_PURGELEVEL: C2RustUnnamed = 7;
pub const PU_LEVSPEC: C2RustUnnamed = 6;
pub const PU_LEVEL: C2RustUnnamed = 5;
pub const PU_FREE: C2RustUnnamed = 4;
pub const PU_MUSIC: C2RustUnnamed = 3;
pub const PU_SOUND: C2RustUnnamed = 2;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const DIR_SEPARATOR_S: [::core::ffi::c_char; 2] = unsafe {
    ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0")
};
#[no_mangle]
pub unsafe extern "C" fn M_MakeDirectory(mut path: *mut ::core::ffi::c_char) {
    mkdir(path, 0o755 as __mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn M_FileExists(
    mut filename: *mut ::core::ffi::c_char,
) -> boolean {
    let mut fstream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstream = fopen(filename, b"r\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if !fstream.is_null() {
        fclose(fstream);
        return true_0 as boolean;
    } else {
        return (*__errno_location() == EISDIR) as ::core::ffi::c_int as boolean
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_FileLength(mut handle: *mut FILE) -> ::core::ffi::c_long {
    let mut savedpos: ::core::ffi::c_long = 0;
    let mut length: ::core::ffi::c_long = 0;
    savedpos = ftell(handle);
    fseek(handle, 0 as ::core::ffi::c_long, SEEK_END);
    length = ftell(handle);
    fseek(handle, savedpos, SEEK_SET);
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn M_WriteFile(
    mut name: *mut ::core::ffi::c_char,
    mut source: *mut ::core::ffi::c_void,
    mut length: ::core::ffi::c_int,
) -> boolean {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut count: ::core::ffi::c_int = 0;
    handle = fopen(name, b"wb\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if handle.is_null() {
        return false_0 as boolean;
    }
    count = fwrite(source, 1 as size_t, length as size_t, handle) as ::core::ffi::c_int;
    fclose(handle);
    if count < length {
        return false_0 as boolean;
    }
    return true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadFile(
    mut name: *mut ::core::ffi::c_char,
    mut buffer: *mut *mut byte,
) -> ::core::ffi::c_int {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut count: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut buf: *mut byte = ::core::ptr::null_mut::<byte>();
    handle = fopen(name, b"rb\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if handle.is_null() {
        I_Error(
            b"Couldn't read file %s\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            name,
        );
    }
    length = M_FileLength(handle) as ::core::ffi::c_int;
    buf = Z_Malloc(length, PU_STATIC as ::core::ffi::c_int, NULL) as *mut byte;
    count = fread(buf as *mut ::core::ffi::c_void, 1 as size_t, length as size_t, handle)
        as ::core::ffi::c_int;
    fclose(handle);
    if count < length {
        I_Error(
            b"Couldn't read file %s\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            name,
        );
    }
    *buffer = buf;
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn M_TempFile(
    mut s: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut tempdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    tempdir = b"/tmp\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    return M_StringJoin(tempdir, DIR_SEPARATOR_S.as_ptr(), s, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn M_StrToInt(
    mut str: *const ::core::ffi::c_char,
    mut result: *mut ::core::ffi::c_int,
) -> boolean {
    return (sscanf(str, b" 0x%x\0" as *const u8 as *const ::core::ffi::c_char, result)
        == 1 as ::core::ffi::c_int
        || sscanf(str, b" 0X%x\0" as *const u8 as *const ::core::ffi::c_char, result)
            == 1 as ::core::ffi::c_int
        || sscanf(str, b" 0%o\0" as *const u8 as *const ::core::ffi::c_char, result)
            == 1 as ::core::ffi::c_int
        || sscanf(str, b" %d\0" as *const u8 as *const ::core::ffi::c_char, result)
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn M_ExtractFileBase(
    mut path: *mut ::core::ffi::c_char,
    mut dest: *mut ::core::ffi::c_char,
) {
    let mut src: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut length: ::core::ffi::c_int = 0;
    src = path.offset(strlen(path) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    while src != path
        && *src.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            != DIR_SEPARATOR
    {
        src = src.offset(-1);
    }
    filename = src;
    length = 0 as ::core::ffi::c_int;
    memset(dest as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 8 as size_t);
    while *src as ::core::ffi::c_int != '\0' as i32
        && *src as ::core::ffi::c_int != '.' as i32
    {
        if length >= 8 as ::core::ffi::c_int {
            printf(
                b"Warning: Truncated '%s' lump name to '%.8s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
                dest,
            );
            break;
        } else {
            let fresh3 = length;
            length = length + 1;
            *dest.offset(fresh3 as isize) = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_int>() as usize > 1 as usize {
                    if 0 != 0 {
                        let fresh0 = src;
                        src = src.offset(1);
                        let mut __c: ::core::ffi::c_int = *fresh0 as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        let fresh1 = src;
                        src = src.offset(1);
                        __res = toupper(*fresh1 as ::core::ffi::c_int);
                    }
                } else {
                    let fresh2 = src;
                    src = src.offset(1);
                    __res = *(*__ctype_toupper_loc())
                        .offset(*fresh2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as ::core::ffi::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_ForceUppercase(mut text: *mut ::core::ffi::c_char) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = text;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        *p = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = *p as ::core::ffi::c_int;
                    __res = (if __c < -(128 as ::core::ffi::c_int)
                        || __c > 255 as ::core::ffi::c_int
                    {
                        __c as __int32_t
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    }) as ::core::ffi::c_int;
                } else {
                    __res = toupper(*p as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            }
            __res
        }) as ::core::ffi::c_char;
        p = p.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_StrCaseStr(
    mut haystack: *mut ::core::ffi::c_char,
    mut needle: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut haystack_len: ::core::ffi::c_uint = 0;
    let mut needle_len: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    haystack_len = strlen(haystack) as ::core::ffi::c_uint;
    needle_len = strlen(needle) as ::core::ffi::c_uint;
    if haystack_len < needle_len {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    len = haystack_len.wrapping_sub(needle_len);
    i = 0 as ::core::ffi::c_uint;
    while i <= len {
        if strncasecmp(haystack.offset(i as isize), needle, needle_len as size_t) == 0 {
            return haystack.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn M_StringDuplicate(
    mut orig: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    result = strdup(orig);
    if result.is_null() {
        I_Error(
            b"Failed to duplicate string (length %i)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            strlen(orig),
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn M_StringReplace(
    mut haystack: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
    mut replacement: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dst: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut needle_len: size_t = strlen(needle);
    let mut result_len: size_t = 0;
    let mut dst_len: size_t = 0;
    result_len = strlen(haystack).wrapping_add(1 as size_t);
    p = haystack;
    loop {
        p = strstr(p, needle);
        if p.is_null() {
            break;
        }
        p = p.offset(needle_len as isize);
        result_len = result_len
            .wrapping_add(strlen(replacement).wrapping_sub(needle_len));
    }
    result = malloc(result_len) as *mut ::core::ffi::c_char;
    if result.is_null() {
        I_Error(
            b"M_StringReplace: Failed to allocate new string\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    dst = result;
    dst_len = result_len;
    p = haystack;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        if strncmp(p, needle, needle_len) == 0 {
            M_StringCopy(dst, replacement, dst_len);
            p = p.offset(needle_len as isize);
            dst = dst.offset(strlen(replacement) as isize);
            dst_len = dst_len.wrapping_sub(strlen(replacement));
        } else {
            *dst = *p;
            dst = dst.offset(1);
            dst_len = dst_len.wrapping_sub(1);
            p = p.offset(1);
        }
    }
    *dst = '\0' as i32 as ::core::ffi::c_char;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn M_StringCopy(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut dest_size: size_t,
) -> boolean {
    let mut len: size_t = 0;
    if dest_size >= 1 as size_t {
        *dest.offset(dest_size.wrapping_sub(1 as size_t) as isize) = '\0' as i32
            as ::core::ffi::c_char;
        strncpy(dest, src, dest_size.wrapping_sub(1 as size_t));
    } else {
        return false_0 as boolean
    }
    len = strlen(dest);
    return (*src.offset(len as isize) as ::core::ffi::c_int == '\0' as i32)
        as ::core::ffi::c_int as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn M_StringConcat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut dest_size: size_t,
) -> boolean {
    let mut offset: size_t = 0;
    offset = strlen(dest);
    if offset > dest_size {
        offset = dest_size;
    }
    return M_StringCopy(
        dest.offset(offset as isize),
        src,
        dest_size.wrapping_sub(offset),
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_StringStartsWith(
    mut s: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> boolean {
    return (strlen(s) > strlen(prefix)
        && strncmp(s, prefix, strlen(prefix)) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn M_StringEndsWith(
    mut s: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> boolean {
    return (strlen(s) >= strlen(suffix)
        && strcmp(
            s.offset(strlen(s) as isize).offset(-(strlen(suffix) as isize)),
            suffix,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn M_StringJoin(
    mut s: *const ::core::ffi::c_char,
    mut args: ...
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut v: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result_len: size_t = 0;
    result_len = strlen(s).wrapping_add(1 as size_t);
    args_0 = args.clone();
    loop {
        v = args_0.arg::<*const ::core::ffi::c_char>();
        if v.is_null() {
            break;
        }
        result_len = result_len.wrapping_add(strlen(v));
    }
    result = malloc(result_len) as *mut ::core::ffi::c_char;
    if result.is_null() {
        I_Error(
            b"M_StringJoin: Failed to allocate new string.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    M_StringCopy(result, s, result_len);
    args_0 = args.clone();
    loop {
        v = args_0.arg::<*const ::core::ffi::c_char>();
        if v.is_null() {
            break;
        }
        M_StringConcat(result, v, result_len);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn M_vsnprintf(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_len: size_t,
    mut s: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    if buf_len < 1 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    result = vsnprintf(buf, buf_len, s, args.as_va_list());
    if result < 0 as ::core::ffi::c_int || result as size_t >= buf_len {
        *buf.offset(buf_len.wrapping_sub(1 as size_t) as isize) = '\0' as i32
            as ::core::ffi::c_char;
        result = buf_len.wrapping_sub(1 as size_t) as ::core::ffi::c_int;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn M_snprintf(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_len: size_t,
    mut s: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result: ::core::ffi::c_int = 0;
    args_0 = args.clone();
    result = M_vsnprintf(buf, buf_len, s, args_0.as_va_list());
    return result;
}
