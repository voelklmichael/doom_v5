use crate::src::sha1::{sha1_context_s, sha1_context_t, SHA1_Init, SHA1_Final, SHA1_UpdateInt32, SHA1_UpdateString};
use crate::src::w_wad::lumpinfo_t;
use crate::src::w_file::wad_file_t;
use crate::src::w_wad::numlumps;
use crate::src::w_wad::lumpinfo;
use crate::src::m_misc::M_StringCopy;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
extern "C" {
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub struct WChecksumState {
    open_wadfiles: *mut *mut wad_file_t,
    num_open_wadfiles: i32,
}

impl WChecksumState {
    pub const fn new() -> Self {
        WChecksumState {
            open_wadfiles: ::core::ptr::null::<*mut wad_file_t>() as *mut *mut wad_file_t,
            num_open_wadfiles: 0,
        }
    }
}

unsafe fn GetFileNumber(state: &mut WChecksumState, mut handle: *mut wad_file_t) -> i32 {
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    i = 0 as i32;
    while i < state.num_open_wadfiles {
        if *state.open_wadfiles.offset(i as isize) == handle {
            return i;
        }
        i += 1;
    }
    state.open_wadfiles = realloc(
        state.open_wadfiles as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut wad_file_t>() as size_t)
            .wrapping_mul((state.num_open_wadfiles + 1 as i32) as size_t),
    ) as *mut *mut wad_file_t;
    let ref mut fresh0 = *state.open_wadfiles.offset(state.num_open_wadfiles as isize);
    *fresh0 = handle;
    result = state.num_open_wadfiles;
    state.num_open_wadfiles += 1;
    return result;
}
unsafe fn ChecksumAddLump(
    state: &mut WChecksumState,
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
        GetFileNumber(state, (*lump).wad_file) as u32,
    );
    SHA1_UpdateInt32(sha1_context, (*lump).position as u32);
    SHA1_UpdateInt32(sha1_context, (*lump).size as u32);
}
pub unsafe fn W_Checksum(state: &mut WChecksumState, mut digest: *mut byte) {
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
    state.num_open_wadfiles = 0 as i32;
    i = 0 as u32;
    while i < numlumps {
        ChecksumAddLump(
            state,
            &raw mut sha1_context,
            lumpinfo.offset(i as isize) as *mut lumpinfo_t,
        );
        i = i.wrapping_add(1);
    }
    SHA1_Final(digest, &raw mut sha1_context);
}
