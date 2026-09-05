use crate::src::sha1::{sha1_context_s, sha1_context_t, SHA1_Init, SHA1_Final, SHA1_UpdateInt32, SHA1_UpdateString};
use crate::src::w_wad::lumpinfo_t;
use crate::src::w_file::wad_file_t;
extern "C" {
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn M_StringCopy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dest_size: size_t,
    ) -> boolean;
    static mut lumpinfo: *mut lumpinfo_t;
    static mut numlumps: u32;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
static mut open_wadfiles: *mut *mut wad_file_t = ::core::ptr::null::<*mut wad_file_t>()
    as *mut *mut wad_file_t;
static mut num_open_wadfiles: i32 = 0 as i32;
unsafe extern "C" fn GetFileNumber(mut handle: *mut wad_file_t) -> i32 {
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    i = 0 as i32;
    while i < num_open_wadfiles {
        if *open_wadfiles.offset(i as isize) == handle {
            return i;
        }
        i += 1;
    }
    open_wadfiles = realloc(
        open_wadfiles as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut wad_file_t>() as size_t)
            .wrapping_mul((num_open_wadfiles + 1 as i32) as size_t),
    ) as *mut *mut wad_file_t;
    let ref mut fresh0 = *open_wadfiles.offset(num_open_wadfiles as isize);
    *fresh0 = handle;
    result = num_open_wadfiles;
    num_open_wadfiles += 1;
    return result;
}
unsafe extern "C" fn ChecksumAddLump(
    mut sha1_context: *mut sha1_context_t,
    mut lump: *mut lumpinfo_t,
) {
    let mut buf: [::core::ffi::c_char; 9] = [0; 9];
    M_StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        &raw mut (*lump).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
    );
    SHA1_UpdateString(sha1_context, &raw mut buf as *mut ::core::ffi::c_char);
    SHA1_UpdateInt32(
        sha1_context,
        GetFileNumber((*lump).wad_file) as u32,
    );
    SHA1_UpdateInt32(sha1_context, (*lump).position as u32);
    SHA1_UpdateInt32(sha1_context, (*lump).size as u32);
}
#[no_mangle]
pub unsafe extern "C" fn W_Checksum(mut digest: *mut byte) {
    let mut sha1_context: sha1_context_t = sha1_context_s {
        h0: 0,
        h1: 0,
        h2: 0,
        h3: 0,
        h4: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut i: u32 = 0;
    SHA1_Init(&raw mut sha1_context);
    num_open_wadfiles = 0 as i32;
    i = 0 as u32;
    while i < numlumps {
        ChecksumAddLump(
            &raw mut sha1_context,
            lumpinfo.offset(i as isize) as *mut lumpinfo_t,
        );
        i = i.wrapping_add(1);
    }
    SHA1_Final(digest, &raw mut sha1_context);
}
