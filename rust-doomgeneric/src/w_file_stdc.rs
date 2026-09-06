use crate::src::i_system::FILE;
use crate::src::i_system::SEEK_SET;
use crate::src::i_system::{fclose, fopen, fread, fseek};
use crate::src::m_misc::M_FileLength;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::w_file::{wad_file_class_t, wad_file_t};
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stdc_wad_file_t {
    pub wad: wad_file_t,
    pub fstream: *mut FILE,
}
unsafe fn W_StdC_OpenFile(mut path: *mut ::core::ffi::c_char) -> *mut wad_file_t {
    let mut result: *mut stdc_wad_file_t = ::core::ptr::null_mut::<stdc_wad_file_t>();
    let mut fstream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstream = fopen(path, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
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
unsafe fn W_StdC_CloseFile(mut wad: *mut wad_file_t) {
    let mut stdc_wad: *mut stdc_wad_file_t = ::core::ptr::null_mut::<stdc_wad_file_t>();
    stdc_wad = wad as *mut stdc_wad_file_t;
    fclose((*stdc_wad).fstream);
    Z_Free(stdc_wad as *mut ::core::ffi::c_void);
}
pub unsafe fn W_StdC_Read(
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
        OpenFile: Some(W_StdC_OpenFile as unsafe fn(*mut ::core::ffi::c_char) -> *mut wad_file_t),
        CloseFile: Some(W_StdC_CloseFile as unsafe fn(*mut wad_file_t) -> ()),
        Read: Some(
            W_StdC_Read
                as unsafe fn(*mut wad_file_t, u32, *mut ::core::ffi::c_void, size_t) -> size_t,
        ),
    }
};
