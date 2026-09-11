use crate::src::doomdef::NULL;
use crate::src::fixed_cstr::FixedCStr;
use crate::src::game_state::game_state;
use crate::src::i_system::I_Error;
use crate::src::i_system::FILE;
use crate::src::i_system::SEEK_SET;
use crate::src::i_system::{fclose, fopen, fread, fseek, ftell, fwrite};
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use libc::memset;
use libc::malloc;
extern "C" {
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> i32;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut i32;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> i32;
}
pub type __mode_t = u32;
pub const SEEK_END: i32 = 2;
pub const EISDIR: i32 = 21;
pub const DIR_SEPARATOR: i32 = '/' as i32;
pub const DIR_SEPARATOR_S: FixedCStr<2> = FixedCStr(*b"/\0");
pub unsafe fn M_MakeDirectory(mut path: *mut ::core::ffi::c_char) {
    mkdir(path, 0o755 as __mode_t);
}
pub unsafe fn M_FileExists(mut filename: *mut ::core::ffi::c_char) -> bool {
    let mut fstream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstream = fopen(filename, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if !fstream.is_null() {
        fclose(fstream);
        return true;
    } else {
        return *__errno_location() == EISDIR;
    };
}
pub unsafe fn M_FileLength(mut handle: *mut FILE) -> i64 {
    let mut savedpos: i64 = 0;
    let mut length: i64 = 0;
    savedpos = ftell(handle);
    fseek(handle, 0 as i64, SEEK_END);
    length = ftell(handle);
    fseek(handle, savedpos, SEEK_SET);
    return length;
}
pub unsafe fn M_WriteFile(
    mut name: *mut ::core::ffi::c_char,
    mut source: *mut ::core::ffi::c_void,
    mut length: i32,
) -> bool {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut count: i32 = 0;
    handle = fopen(name, b"wb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if handle.is_null() {
        return false;
    }
    count = fwrite(source, 1 as size_t, length as size_t, handle) as i32;
    fclose(handle);
    if count < length {
        return false;
    }
    return true;
}
pub unsafe fn M_ReadFile(mut name: *mut ::core::ffi::c_char, mut buffer: *mut *mut byte) -> i32 {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut count: i32 = 0;
    let mut length: i32 = 0;
    let mut buf: *mut byte = ::core::ptr::null_mut::<byte>();
    handle = fopen(name, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if handle.is_null() {
        I_Error(&format!(
            "Couldn't read file {}",
            ::std::ffi::CStr::from_ptr(name).to_str().unwrap()
        ));
    }
    length = M_FileLength(handle) as i32;
    buf = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        length,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    count = fread(
        buf as *mut ::core::ffi::c_void,
        1 as size_t,
        length as size_t,
        handle,
    ) as i32;
    fclose(handle);
    if count < length {
        I_Error(&format!(
            "Couldn't read file {}",
            ::std::ffi::CStr::from_ptr(name).to_str().unwrap()
        ));
    }
    *buffer = buf;
    return length;
}
pub unsafe fn M_TempFile(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut tempdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    tempdir = b"/tmp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    return M_StringJoin(tempdir, DIR_SEPARATOR_S.as_ptr(), s, NULL);
}
fn m_strtoint_digit_prefix(s: &str, radix: u32) -> Option<i32> {
    let end = s
        .find(|c: char| !c.is_digit(radix))
        .unwrap_or(s.len());
    if end == 0 {
        None
    } else {
        i32::from_str_radix(&s[..end], radix).ok()
    }
}
pub unsafe fn M_StrToInt(mut str: *const ::core::ffi::c_char, mut result: *mut i32) -> bool {
    let s = ::std::ffi::CStr::from_ptr(str).to_string_lossy();
    let trimmed = s.trim_start();
    let (sign, unsigned) = match trimmed.strip_prefix('-') {
        Some(rest) => (-1, rest),
        None => (1, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    let parsed = unsigned
        .strip_prefix("0x")
        .or_else(|| unsigned.strip_prefix("0X"))
        .and_then(|rest| m_strtoint_digit_prefix(rest, 16))
        .or_else(|| {
            unsigned
                .strip_prefix('0')
                .and_then(|rest| m_strtoint_digit_prefix(rest, 8))
        })
        .or_else(|| m_strtoint_digit_prefix(unsigned, 10));
    match parsed {
        Some(v) => {
            *result = sign * v;
            true
        }
        None => false,
    }
}
pub unsafe fn M_ExtractFileBase(
    mut path: *mut ::core::ffi::c_char,
    mut dest: *mut ::core::ffi::c_char,
) {
    let mut src: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut length: i32 = 0;
    src = path
        .offset(::std::ffi::CStr::from_ptr(path as *const ::core::ffi::c_char).to_bytes().len() as isize)
        .offset(-(1 as i32 as isize));
    while src != path && *src.offset(-(1 as i32 as isize)) as i32 != DIR_SEPARATOR {
        src = src.offset(-1);
    }
    filename = src;
    length = 0 as i32;
    memset(dest as *mut ::core::ffi::c_void, 0 as i32, 8 as size_t);
    while *src as i32 != '\0' as i32 && *src as i32 != '.' as i32 {
        if length >= 8 as i32 {
            println!(
                "Warning: Truncated '{}' lump name to '{:.8}'.",
                ::std::ffi::CStr::from_ptr(filename).to_string_lossy(),
                ::std::ffi::CStr::from_ptr(dest).to_string_lossy(),
            );
            break;
        } else {
            let fresh3 = length;
            length = length + 1;
            let fresh1 = src;
            src = src.offset(1);
            *dest.offset(fresh3 as isize) =
                (*fresh1 as u8).to_ascii_uppercase() as ::core::ffi::c_char;
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
pub unsafe fn M_StringReplace(
    mut haystack: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
    mut replacement: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dst: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut needle_len: size_t = ::std::ffi::CStr::from_ptr(needle as *const ::core::ffi::c_char).to_bytes().len();
    let mut result_len: size_t = 0;
    let mut dst_len: size_t = 0;
    result_len = ::std::ffi::CStr::from_ptr(haystack as *const ::core::ffi::c_char).to_bytes().len().wrapping_add(1 as size_t);
    p = haystack;
    loop {
        p = strstr(p, needle);
        if p.is_null() {
            break;
        }
        p = p.offset(needle_len as isize);
        result_len = result_len.wrapping_add(::std::ffi::CStr::from_ptr(replacement as *const ::core::ffi::c_char).to_bytes().len().wrapping_sub(needle_len));
    }
    result = malloc(result_len) as *mut ::core::ffi::c_char;
    if result.is_null() {
        I_Error("M_StringReplace: Failed to allocate new string");
    }
    dst = result;
    dst_len = result_len;
    p = haystack;
    while *p as i32 != '\0' as i32 {
        if ::std::ffi::CStr::from_ptr(p).to_bytes().get(..needle_len)
            == Some(::std::ffi::CStr::from_ptr(needle).to_bytes())
        {
            M_StringCopy(dst, replacement, dst_len);
            p = p.offset(needle_len as isize);
            dst = dst.offset(::std::ffi::CStr::from_ptr(replacement as *const ::core::ffi::c_char).to_bytes().len() as isize);
            dst_len = dst_len.wrapping_sub(::std::ffi::CStr::from_ptr(replacement as *const ::core::ffi::c_char).to_bytes().len());
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
pub unsafe fn M_StringCopy(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut dest_size: size_t,
) -> bool {
    if dest_size < 1 as size_t {
        return false;
    }
    let src_bytes = ::std::ffi::CStr::from_ptr(src).to_bytes();
    let copy_len = src_bytes.len().min(dest_size - 1 as size_t);
    for i in 0..dest_size {
        *dest.add(i) = if i < copy_len {
            src_bytes[i] as ::core::ffi::c_char
        } else {
            0
        };
    }
    src_bytes.len() == copy_len
}
pub unsafe fn M_StringConcat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut dest_size: size_t,
) -> bool {
    let mut offset: size_t = 0;
    offset = ::std::ffi::CStr::from_ptr(dest as *const ::core::ffi::c_char).to_bytes().len();
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
pub unsafe extern "C" fn M_StringJoin(
    mut s: *const ::core::ffi::c_char,
    mut args: ...
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result_len: size_t = 0;
    result_len = ::std::ffi::CStr::from_ptr(s as *const ::core::ffi::c_char).to_bytes().len().wrapping_add(1 as size_t);
    args_0 = args.clone();
    loop {
        v = args_0.arg::<*const ::core::ffi::c_char>();
        if v.is_null() {
            break;
        }
        result_len = result_len.wrapping_add(::std::ffi::CStr::from_ptr(v as *const ::core::ffi::c_char).to_bytes().len());
    }
    result = malloc(result_len) as *mut ::core::ffi::c_char;
    if result.is_null() {
        I_Error("M_StringJoin: Failed to allocate new string.");
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
pub unsafe fn M_vsnprintf(
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
        *buf.offset(buf_len.wrapping_sub(1 as size_t) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
        result = buf_len.wrapping_sub(1 as size_t) as i32;
    }
    return result;
}
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
