use crate::src::w_file::wad_file_t;
use crate::src::i_system::I_Error;
use crate::src::d_mode::D_GameMissionString;
use crate::src::d_iwad::D_SuggestGameName;
use crate::src::i_video::I_BeginRead;
use crate::src::i_video::I_EndRead;
use crate::src::m_misc::M_ExtractFileBase;
use crate::src::w_file::W_OpenFile;
use crate::src::z_zone::Z_ChangeTag2;
use crate::src::w_file::W_Read;
use crate::src::z_zone::Z_ChangeUser;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use libc::{memcpy, memset};
use libc::{strcasecmp, strlen, strncasecmp, strncmp, strncpy, toupper};
use libc::{free, printf};
use crate::src::m_misc::__ctype_toupper_loc;
use crate::src::d_mode::indetermined;
use crate::src::d_mode::{GameMission_t, doom, heretic, hexen, strife};
use crate::src::stdint_types::byte;
use crate::src::stdint_types::__int32_t;
use crate::src::stdint_types::size_t;
use crate::src::doomdef::NULL;

extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lumpinfo_s {
    pub name: [::core::ffi::c_char; 8],
    pub wad_file: *mut wad_file_t,
    pub position: i32,
    pub size: i32,
    pub cache: *mut ::core::ffi::c_void,
    pub next: *mut lumpinfo_t,
}
pub type lumpinfo_t = lumpinfo_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct filelump_t {
    pub filepos: i32,
    pub size: i32,
    pub name: [::core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct wadinfo_t {
    pub identification: [::core::ffi::c_char; 4],
    pub numlumps: i32,
    pub infotableofs: i32,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub mission: GameMission_t,
    pub lumpname: &'static str,
}
pub const PROGRAM_PREFIX: [::core::ffi::c_char; 12] = unsafe {
    ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"doomgeneric\0")
};
pub static mut lumpinfo: *mut lumpinfo_t = ::core::ptr::null::<lumpinfo_t>()
    as *mut lumpinfo_t;
pub static mut numlumps: u32 = 0 as u32;
static mut lumphash: *mut *mut lumpinfo_t = ::core::ptr::null::<*mut lumpinfo_t>()
    as *mut *mut lumpinfo_t;
pub unsafe fn W_LumpNameHash(
    mut s: *const ::core::ffi::c_char,
) -> u32 {
    let mut result: u32 = 5381 as u32;
    let mut i: u32 = 0;
    i = 0 as u32;
    while i < 8 as u32
        && *s.offset(i as isize) as i32 != '\0' as i32
    {
        result = result << 5 as i32 ^ result
            ^ ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i32>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: i32 = *s.offset(i as isize)
                            as i32;
                        __res = (if __c < -(128 as i32)
                            || __c > 255 as i32
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as i32;
                    } else {
                        __res = toupper(*s.offset(i as isize) as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*s.offset(i as isize) as i32 as isize)
                        as i32;
                }
                __res
            }) as u32;
        i = i.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn ExtendLumpInfo(mut newnumlumps: i32) {
    let mut newlumpinfo: *mut lumpinfo_t = ::core::ptr::null_mut::<lumpinfo_t>();
    let mut i: u32 = 0;
    newlumpinfo = calloc(
        newnumlumps as size_t,
        ::core::mem::size_of::<lumpinfo_t>() as size_t,
    ) as *mut lumpinfo_t;
    if newlumpinfo.is_null() {
        I_Error("Couldn't realloc lumpinfo");
    }
    i = 0 as u32;
    while i < numlumps && i < newnumlumps as u32 {
        memcpy(
            newlumpinfo.offset(i as isize) as *mut lumpinfo_t
                as *mut ::core::ffi::c_void,
            lumpinfo.offset(i as isize) as *mut lumpinfo_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<lumpinfo_t>() as size_t,
        );
        if !(*newlumpinfo.offset(i as isize)).cache.is_null() {
            Z_ChangeUser(
                (*newlumpinfo.offset(i as isize)).cache,
                &raw mut (*newlumpinfo.offset(i as isize)).cache,
            );
        }
        if !(*lumpinfo.offset(i as isize)).next.is_null() {
            let mut nextlumpnum: i32 = (*lumpinfo.offset(i as isize))
                .next
                .offset_from(lumpinfo) as i64 as i32;
            let ref mut fresh0 = (*newlumpinfo.offset(i as isize)).next;
            *fresh0 = newlumpinfo.offset(nextlumpnum as isize) as *mut lumpinfo_t;
        }
        i = i.wrapping_add(1);
    }
    free(lumpinfo as *mut ::core::ffi::c_void);
    lumpinfo = newlumpinfo;
    numlumps = newnumlumps as u32;
}
pub unsafe fn W_AddFile(
    mut filename: *mut ::core::ffi::c_char,
) -> *mut wad_file_t {
    let mut header: wadinfo_t = wadinfo_t {
        identification: [0; 4],
        numlumps: 0,
        infotableofs: 0,
    };
    let mut lump_p: *mut lumpinfo_t = ::core::ptr::null_mut::<lumpinfo_t>();
    let mut i: u32 = 0;
    let mut wad_file: *mut wad_file_t = ::core::ptr::null_mut::<wad_file_t>();
    let mut length: i32 = 0;
    let mut startlump: i32 = 0;
    let mut fileinfo: *mut filelump_t = ::core::ptr::null_mut::<filelump_t>();
    let mut filerover: *mut filelump_t = ::core::ptr::null_mut::<filelump_t>();
    let mut newnumlumps: i32 = 0;
    wad_file = W_OpenFile(filename);
    if wad_file.is_null() {
        printf(
            b" couldn't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        return ::core::ptr::null_mut::<wad_file_t>();
    }
    newnumlumps = numlumps as i32;
    if strcasecmp(
        filename
            .offset(strlen(filename) as isize)
            .offset(-(3 as i32 as isize)),
        b"wad\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        fileinfo = Z_Malloc(
            ::core::mem::size_of::<filelump_t>() as i32,
            PU_STATIC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut filelump_t;
        (*fileinfo).filepos = 0 as i32;
        (*fileinfo).size = (*wad_file).length as i32;
        M_ExtractFileBase(
            filename,
            &raw mut (*fileinfo).name as *mut ::core::ffi::c_char,
        );
        newnumlumps += 1;
    } else {
        W_Read(
            wad_file,
            0 as u32,
            &raw mut header as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<wadinfo_t>() as size_t,
        );
        if strncmp(
            &raw mut header.identification as *mut ::core::ffi::c_char,
            b"IWAD\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0
        {
            if strncmp(
                &raw mut header.identification as *mut ::core::ffi::c_char,
                b"PWAD\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) != 0
            {
                I_Error(&format!(
                    "Wad file {} doesn't have IWAD or PWAD id\n",
                    ::std::ffi::CStr::from_ptr(filename).to_str().unwrap(),
                ));
            }
        }
        header.numlumps = header.numlumps;
        header.infotableofs = header.infotableofs;
        length = (header.numlumps as usize)
            .wrapping_mul(::core::mem::size_of::<filelump_t>() as usize)
            as i32;
        fileinfo = Z_Malloc(
            length,
            PU_STATIC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut filelump_t;
        W_Read(
            wad_file,
            header.infotableofs as u32,
            fileinfo as *mut ::core::ffi::c_void,
            length as size_t,
        );
        newnumlumps += header.numlumps;
    }
    startlump = numlumps as i32;
    ExtendLumpInfo(newnumlumps);
    lump_p = lumpinfo.offset(startlump as isize) as *mut lumpinfo_t;
    filerover = fileinfo;
    i = startlump as u32;
    while i < numlumps {
        (*lump_p).wad_file = wad_file;
        (*lump_p).position = (*filerover).filepos;
        (*lump_p).size = (*filerover).size;
        (*lump_p).cache = NULL;
        strncpy(
            &raw mut (*lump_p).name as *mut ::core::ffi::c_char,
            &raw mut (*filerover).name as *mut ::core::ffi::c_char,
            8 as size_t,
        );
        lump_p = lump_p.offset(1);
        filerover = filerover.offset(1);
        i = i.wrapping_add(1);
    }
    Z_Free(fileinfo as *mut ::core::ffi::c_void);
    if !lumphash.is_null() {
        Z_Free(lumphash as *mut ::core::ffi::c_void);
        lumphash = ::core::ptr::null_mut::<*mut lumpinfo_t>();
    }
    return wad_file;
}
#[no_mangle]
pub unsafe extern "C" fn W_NumLumps() -> i32 {
    return numlumps as i32;
}
/// Reads up to 8 bytes at `ptr` as a WAD lump name and converts it to an
/// owned `String`, stopping at the first nul (if any). WAD lump names are a
/// fixed 8-byte field with no guaranteed nul terminator, so unlike
/// `CStr::from_ptr` this never reads past the 8th byte; invalid UTF-8 is
/// lossily replaced rather than panicking, since arbitrary WAD/PWAD data is
/// not guaranteed to be valid UTF-8 (or even ASCII).
pub unsafe fn wad_name8_to_string(ptr: *const ::core::ffi::c_char) -> String {
    let bytes = ::core::slice::from_raw_parts(ptr as *const u8, 8);
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(8);
    String::from_utf8_lossy(&bytes[..len]).into_owned()
}
pub unsafe fn W_CheckNumForName(name: &str) -> i32 {
    let name_cstring = ::std::ffi::CString::new(name).unwrap();
    let name = name_cstring.as_ptr() as *mut ::core::ffi::c_char;
    let mut lump_p: *mut lumpinfo_t = ::core::ptr::null_mut::<lumpinfo_t>();
    let mut i: i32 = 0;
    if !lumphash.is_null() {
        let mut hash: i32 = 0;
        hash = W_LumpNameHash(name).wrapping_rem(numlumps) as i32;
        lump_p = *lumphash.offset(hash as isize);
        while !lump_p.is_null() {
            if strncasecmp(
                &raw mut (*lump_p).name as *mut ::core::ffi::c_char,
                name,
                8 as size_t,
            ) == 0
            {
                return lump_p.offset_from(lumpinfo) as i64
                    as i32;
            }
            lump_p = (*lump_p).next;
        }
    } else {
        i = numlumps.wrapping_sub(1 as u32) as i32;
        while i >= 0 as i32 {
            if strncasecmp(
                &raw mut (*lumpinfo.offset(i as isize)).name as *mut ::core::ffi::c_char,
                name,
                8 as size_t,
            ) == 0
            {
                return i;
            }
            i -= 1;
        }
    }
    return -(1 as i32);
}
pub unsafe fn W_GetNumForName(name: &str) -> i32 {
    let mut i: i32 = 0;
    i = W_CheckNumForName(name);
    if i < 0 as i32 {
        I_Error(&format!("W_GetNumForName: {} not found!", name));
    }
    return i;
}
pub unsafe fn W_LumpLength(
    mut lump: u32,
) -> i32 {
    if lump >= numlumps {
        I_Error(&format!("W_LumpLength: {} >= numlumps", lump));
    }
    return (*lumpinfo.offset(lump as isize)).size;
}
pub unsafe fn W_ReadLump(
    mut lump: u32,
    mut dest: *mut ::core::ffi::c_void,
) {
    let mut c: i32 = 0;
    let mut l: *mut lumpinfo_t = ::core::ptr::null_mut::<lumpinfo_t>();
    if lump >= numlumps {
        I_Error(&format!("W_ReadLump: {} >= numlumps", lump));
    }
    l = lumpinfo.offset(lump as isize);
    I_BeginRead();
    c = W_Read(
        (*l).wad_file,
        (*l).position as u32,
        dest,
        (*l).size as size_t,
    ) as i32;
    if c < (*l).size {
        I_Error(&format!("W_ReadLump: only read {} of {} on lump {}", c, (*l).size, lump));
    }
    I_EndRead();
}
pub unsafe fn W_CacheLumpNum(
    mut lumpnum: i32,
    mut tag: i32,
) -> *mut ::core::ffi::c_void {
    let mut result: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut lump: *mut lumpinfo_t = ::core::ptr::null_mut::<lumpinfo_t>();
    if lumpnum as u32 >= numlumps {
        I_Error(&format!("W_CacheLumpNum: {} >= numlumps", lumpnum));
    }
    lump = lumpinfo.offset(lumpnum as isize) as *mut lumpinfo_t;
    if !(*(*lump).wad_file).mapped.is_null() {
        result = (*(*lump).wad_file).mapped.offset((*lump).position as isize);
    } else if !(*lump).cache.is_null() {
        result = (*lump).cache as *mut byte;
        Z_ChangeTag2(
            (*lump).cache,
            tag,
            b"w_wad.c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            410 as i32,
        );
    } else {
        (*lump).cache = Z_Malloc(
            W_LumpLength(lumpnum as u32),
            tag,
            &raw mut (*lump).cache as *mut ::core::ffi::c_void,
        );
        W_ReadLump(lumpnum as u32, (*lump).cache);
        result = (*lump).cache as *mut byte;
    }
    return result as *mut ::core::ffi::c_void;
}
pub unsafe fn W_CacheLumpName(
    name: &str,
    mut tag: i32,
) -> *mut ::core::ffi::c_void {
    return W_CacheLumpNum(W_GetNumForName(name), tag);
}
pub unsafe fn W_ReleaseLumpNum(mut lumpnum: i32) {
    let mut lump: *mut lumpinfo_t = ::core::ptr::null_mut::<lumpinfo_t>();
    if lumpnum as u32 >= numlumps {
        I_Error(&format!("W_ReleaseLumpNum: {} >= numlumps", lumpnum));
    }
    lump = lumpinfo.offset(lumpnum as isize) as *mut lumpinfo_t;
    if (*(*lump).wad_file).mapped.is_null() {
        Z_ChangeTag2(
            (*lump).cache,
            PU_CACHE as i32,
            b"w_wad.c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            461 as i32,
        );
    }
}
pub unsafe fn W_ReleaseLumpName(name: &str) {
    W_ReleaseLumpNum(W_GetNumForName(name));
}
pub unsafe fn W_GenerateHashTable() {
    let mut i: u32 = 0;
    if !lumphash.is_null() {
        Z_Free(lumphash as *mut ::core::ffi::c_void);
    }
    if numlumps > 0 as u32 {
        lumphash = Z_Malloc(
            (::core::mem::size_of::<*mut lumpinfo_t>() as usize)
                .wrapping_mul(numlumps as usize) as i32,
            PU_STATIC as i32,
            NULL,
        ) as *mut *mut lumpinfo_t;
        memset(
            lumphash as *mut ::core::ffi::c_void,
            0 as i32,
            (::core::mem::size_of::<*mut lumpinfo_t>() as size_t)
                .wrapping_mul(numlumps as size_t),
        );
        i = 0 as u32;
        while i < numlumps {
            let mut hash: u32 = 0;
            hash = W_LumpNameHash(
                    &raw mut (*lumpinfo.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                )
                .wrapping_rem(numlumps);
            let ref mut fresh1 = (*lumpinfo.offset(i as isize)).next;
            *fresh1 = *lumphash.offset(hash as isize);
            let ref mut fresh2 = *lumphash.offset(hash as isize);
            *fresh2 = lumpinfo.offset(i as isize) as *mut lumpinfo_t;
            i = i.wrapping_add(1);
        }
    }
}
static unique_lumps: [C2RustUnnamed_0; 4] = [
    C2RustUnnamed_0 { mission: doom, lumpname: "POSSA1" },
    C2RustUnnamed_0 { mission: heretic, lumpname: "IMPXA1" },
    C2RustUnnamed_0 { mission: hexen, lumpname: "ETTNA1" },
    C2RustUnnamed_0 { mission: strife, lumpname: "AGRDA1" },
];
pub unsafe fn W_CheckCorrectIWAD(mut mission: GameMission_t) {
    let mut i: i32 = 0;
    let mut lumpnum: i32 = 0;
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as usize)
    {
        if mission as u32
            != unique_lumps[i as usize].mission as u32
        {
            lumpnum = W_CheckNumForName(unique_lumps[i as usize].lumpname);
            if lumpnum >= 0 as i32 {
                I_Error(&format!(
                    "\nYou are trying to use a {} IWAD file with the {}{} binary.\nThis isn't going to work.\nYou probably want to use the {}{} binary.",
                    ::std::ffi::CStr::from_ptr(
                        D_SuggestGameName(unique_lumps[i as usize].mission, indetermined),
                    ).to_str().unwrap(),
                    ::std::ffi::CStr::from_ptr(PROGRAM_PREFIX.as_ptr()).to_str().unwrap(),
                    ::std::ffi::CStr::from_ptr(D_GameMissionString(mission)).to_str().unwrap(),
                    ::std::ffi::CStr::from_ptr(PROGRAM_PREFIX.as_ptr()).to_str().unwrap(),
                    ::std::ffi::CStr::from_ptr(
                        D_GameMissionString(unique_lumps[i as usize].mission),
                    ).to_str().unwrap(),
                ));
            }
        }
        i += 1;
    }
}
