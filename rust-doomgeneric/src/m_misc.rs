use crate::src::i_system::FILE;
use crate::src::i_system::I_Error;
extern "C" {
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> i32;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> i32;
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
        __whence: i32,
    ) -> i32;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: i32,
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
    ) -> i32;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> i32;
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
    ) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn toupper(__c: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> i32;
    fn Z_Malloc(
        size: i32,
        tag: i32,
        ptr: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __mode_t = u32;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub const PU_STATIC: C2RustUnnamed = 1;
pub type C2RustUnnamed = u32;
pub const PU_NUM_TAGS: C2RustUnnamed = 9;
pub const PU_CACHE: C2RustUnnamed = 8;
pub const PU_PURGELEVEL: C2RustUnnamed = 7;
pub const PU_LEVSPEC: C2RustUnnamed = 6;
pub const PU_LEVEL: C2RustUnnamed = 5;
pub const PU_FREE: C2RustUnnamed = 4;
pub const PU_MUSIC: C2RustUnnamed = 3;
pub const PU_SOUND: C2RustUnnamed = 2;
pub const SEEK_SET: i32 = 0 as i32;
pub const SEEK_END: i32 = 2 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const EISDIR: i32 = 21 as i32;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const DIR_SEPARATOR: i32 = '/' as i32;
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
        return (*__errno_location() == EISDIR) as i32 as boolean
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
    mut length: i32,
) -> boolean {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut count: i32 = 0;
    handle = fopen(name, b"wb\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if handle.is_null() {
        return false_0 as boolean;
    }
    count = fwrite(source, 1 as size_t, length as size_t, handle) as i32;
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
) -> i32 {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut count: i32 = 0;
    let mut length: i32 = 0;
    let mut buf: *mut byte = ::core::ptr::null_mut::<byte>();
    handle = fopen(name, b"rb\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if handle.is_null() {
        I_Error(&format!("Couldn't read file {}", ::std::ffi::CStr::from_ptr(name).to_str().unwrap()));
    }
    length = M_FileLength(handle) as i32;
    buf = Z_Malloc(length, PU_STATIC as i32, NULL) as *mut byte;
    count = fread(buf as *mut ::core::ffi::c_void, 1 as size_t, length as size_t, handle)
        as i32;
    fclose(handle);
    if count < length {
        I_Error(&format!("Couldn't read file {}", ::std::ffi::CStr::from_ptr(name).to_str().unwrap()));
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
    mut result: *mut i32,
) -> boolean {
    return (sscanf(str, b" 0x%x\0" as *const u8 as *const ::core::ffi::c_char, result)
        == 1 as i32
        || sscanf(str, b" 0X%x\0" as *const u8 as *const ::core::ffi::c_char, result)
            == 1 as i32
        || sscanf(str, b" 0%o\0" as *const u8 as *const ::core::ffi::c_char, result)
            == 1 as i32
        || sscanf(str, b" %d\0" as *const u8 as *const ::core::ffi::c_char, result)
            == 1 as i32) as i32 as boolean;
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
    let mut length: i32 = 0;
    src = path.offset(strlen(path) as isize).offset(-(1 as i32 as isize));
    while src != path
        && *src.offset(-(1 as i32 as isize)) as i32
            != DIR_SEPARATOR
    {
        src = src.offset(-1);
    }
    filename = src;
    length = 0 as i32;
    memset(dest as *mut ::core::ffi::c_void, 0 as i32, 8 as size_t);
    while *src as i32 != '\0' as i32
        && *src as i32 != '.' as i32
    {
        if length >= 8 as i32 {
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
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i32>() as usize > 1 as usize {
                    if 0 != 0 {
                        let fresh0 = src;
                        src = src.offset(1);
                        let mut __c: i32 = *fresh0 as i32;
                        __res = (if __c < -(128 as i32)
                            || __c > 255 as i32
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as i32;
                    } else {
                        let fresh1 = src;
                        src = src.offset(1);
                        __res = toupper(*fresh1 as i32);
                    }
                } else {
                    let fresh2 = src;
                    src = src.offset(1);
                    __res = *(*__ctype_toupper_loc())
                        .offset(*fresh2 as i32 as isize)
                        as i32;
                }
                __res
            }) as ::core::ffi::c_char;
        }
    }
}
pub fn M_ForceUppercase(text: &str) -> String {
    text.to_uppercase()
}
pub fn M_StrCaseStr<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    let haystack_lower = haystack.to_lowercase();
    let needle_lower = needle.to_lowercase();
    haystack_lower.find(&needle_lower).map(|i| &haystack[i..])
}
pub fn M_StringDuplicate(orig: &str) -> String {
    orig.to_string()
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
        I_Error("M_StringReplace: Failed to allocate new string");
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    dst = result;
    dst_len = result_len;
    p = haystack;
    while *p as i32 != '\0' as i32 {
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
    return (*src.offset(len as isize) as i32 == '\0' as i32)
        as i32 as boolean;
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
pub fn M_StringStartsWith(s: &str, prefix: &str) -> bool {
    s.len() > prefix.len() && s.starts_with(prefix)
}
pub fn M_StringEndsWith(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
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
        I_Error("M_StringJoin: Failed to allocate new string.");
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
) -> i32 {
    let mut result: i32 = 0;
    if buf_len < 1 as size_t {
        return 0 as i32;
    }
    result = vsnprintf(buf, buf_len, s, args.as_va_list());
    if result < 0 as i32 || result as size_t >= buf_len {
        *buf.offset(buf_len.wrapping_sub(1 as size_t) as isize) = '\0' as i32
            as ::core::ffi::c_char;
        result = buf_len.wrapping_sub(1 as size_t) as i32;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn M_snprintf(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_len: size_t,
    mut s: *const ::core::ffi::c_char,
    mut args: ...
) -> i32 {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result: i32 = 0;
    args_0 = args.clone();
    result = M_vsnprintf(buf, buf_len, s, args_0.as_va_list());
    return result;
}
