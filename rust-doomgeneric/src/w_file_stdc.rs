use crate::src::i_system::FILE;
use crate::src::w_file::{wad_file_class_t, wad_file_t};
use crate::src::m_misc::M_FileLength;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;

extern "C" {
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> u64;
    fn fseek(
        __stream: *mut FILE,
        __off: i64,
        __whence: i32,
    ) -> i32;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type byte = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stdc_wad_file_t {
    pub wad: wad_file_t,
    pub fstream: *mut FILE,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const SEEK_SET: i32 = 0 as i32;
unsafe extern "C" fn W_StdC_OpenFile(
    mut path: *mut ::core::ffi::c_char,
) -> *mut wad_file_t {
    let mut result: *mut stdc_wad_file_t = ::core::ptr::null_mut::<stdc_wad_file_t>();
    let mut fstream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstream = fopen(path, b"rb\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if fstream.is_null() {
        return ::core::ptr::null_mut::<wad_file_t>();
    }
    result = Z_Malloc(
        ::core::mem::size_of::<stdc_wad_file_t>() as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut stdc_wad_file_t;
    (*result).wad.file_class = &raw mut stdc_wad_file;
    (*result).wad.mapped = ::core::ptr::null_mut::<byte>();
    (*result).wad.length = M_FileLength(fstream) as u32;
    (*result).fstream = fstream;
    return &raw mut (*result).wad;
}
unsafe extern "C" fn W_StdC_CloseFile(mut wad: *mut wad_file_t) {
    let mut stdc_wad: *mut stdc_wad_file_t = ::core::ptr::null_mut::<stdc_wad_file_t>();
    stdc_wad = wad as *mut stdc_wad_file_t;
    fclose((*stdc_wad).fstream);
    Z_Free(stdc_wad as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn W_StdC_Read(
    mut wad: *mut wad_file_t,
    mut offset: u32,
    mut buffer: *mut ::core::ffi::c_void,
    mut buffer_len: size_t,
) -> size_t {
    let mut stdc_wad: *mut stdc_wad_file_t = ::core::ptr::null_mut::<stdc_wad_file_t>();
    let mut result: size_t = 0;
    stdc_wad = wad as *mut stdc_wad_file_t;
    fseek((*stdc_wad).fstream, offset as i64, SEEK_SET);
    result = fread(buffer, 1 as size_t, buffer_len, (*stdc_wad).fstream) as size_t;
    return result;
}
pub static mut stdc_wad_file: wad_file_class_t = unsafe {
    wad_file_class_t {
        OpenFile: Some(
            W_StdC_OpenFile
                as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> *mut wad_file_t,
        ),
        CloseFile: Some(W_StdC_CloseFile as unsafe extern "C" fn(*mut wad_file_t) -> ()),
        Read: Some(
            W_StdC_Read
                as unsafe extern "C" fn(
                    *mut wad_file_t,
                    u32,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
        ),
    }
};
