use crate::src::i_system::I_Error;
use crate::src::m_argv::{myargv, M_CheckParmWithArgs};
use crate::src::m_misc::M_FileExists;
extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
}
pub type size_t = usize;
pub type GameMission_t = u32;
pub const none: GameMission_t = 9;
pub const strife: GameMission_t = 8;
pub const hexen: GameMission_t = 7;
pub const heretic: GameMission_t = 6;
pub const pack_hacx: GameMission_t = 5;
pub const pack_chex: GameMission_t = 4;
pub const pack_plut: GameMission_t = 3;
pub const pack_tnt: GameMission_t = 2;
pub const doom2: GameMission_t = 1;
pub const doom: GameMission_t = 0;
pub type GameMode_t = u32;
pub const indetermined: GameMode_t = 4;
pub const retail: GameMode_t = 3;
pub const commercial: GameMode_t = 2;
pub const registered: GameMode_t = 1;
pub const shareware: GameMode_t = 0;
#[derive(Copy, Clone)]
pub struct iwad_t {
    pub name: &'static str,
    pub mission: GameMission_t,
    pub mode: GameMode_t,
    pub description: &'static str,
}
pub const FILES_DIR: &str = ".";
pub const DIR_SEPARATOR: char = '/';
pub const DIR_SEPARATOR_S: &str = "/";
static iwads: [iwad_t; 14] = [
    iwad_t {
        name: "doom2.wad",
        mission: doom2,
        mode: commercial,
        description: "Doom II",
    },
    iwad_t {
        name: "plutonia.wad",
        mission: pack_plut,
        mode: commercial,
        description: "Final Doom: Plutonia Experiment",
    },
    iwad_t {
        name: "tnt.wad",
        mission: pack_tnt,
        mode: commercial,
        description: "Final Doom: TNT: Evilution",
    },
    iwad_t {
        name: "doom.wad",
        mission: doom,
        mode: retail,
        description: "Doom",
    },
    iwad_t {
        name: "doom1.wad",
        mission: doom,
        mode: shareware,
        description: "Doom Shareware",
    },
    iwad_t {
        name: "chex.wad",
        mission: pack_chex,
        mode: shareware,
        description: "Chex Quest",
    },
    iwad_t {
        name: "hacx.wad",
        mission: pack_hacx,
        mode: commercial,
        description: "Hacx",
    },
    iwad_t {
        name: "freedm.wad",
        mission: doom2,
        mode: commercial,
        description: "FreeDM",
    },
    iwad_t {
        name: "freedoom2.wad",
        mission: doom2,
        mode: commercial,
        description: "Freedoom: Phase 2",
    },
    iwad_t {
        name: "freedoom1.wad",
        mission: doom,
        mode: retail,
        description: "Freedoom: Phase 1",
    },
    iwad_t {
        name: "heretic.wad",
        mission: heretic,
        mode: retail,
        description: "Heretic",
    },
    iwad_t {
        name: "heretic1.wad",
        mission: heretic,
        mode: shareware,
        description: "Heretic Shareware",
    },
    iwad_t {
        name: "hexen.wad",
        mission: hexen,
        mode: commercial,
        description: "Hexen",
    },
    iwad_t {
        name: "strife1.wad",
        mission: strife,
        mode: commercial,
        description: "Strife",
    },
];
pub const MAX_IWAD_DIRS: i32 = 128 as i32;
static mut iwad_dirs_built: bool = false;
static mut iwad_dirs: Vec<String> = Vec::new();

unsafe fn file_exists(path: &str) -> bool {
    let path_cstring = ::std::ffi::CString::new(path).unwrap();
    M_FileExists(path_cstring.as_ptr() as *mut ::core::ffi::c_char) != 0
}
unsafe fn add_iwad_dir(dir: &str) {
    iwad_dirs.push(dir.to_string());
}
fn dir_is_file(path: &str, filename: &str) -> bool {
    path.len() >= filename.len() + 1
        && path.as_bytes()[path.len() - filename.len() - 1] == DIR_SEPARATOR as u8
        && path[path.len() - filename.len()..].eq_ignore_ascii_case(filename)
}
unsafe fn check_directory_has_iwad(dir: &str, iwadname: &str) -> Option<String> {
    if dir_is_file(dir, iwadname) && file_exists(dir) {
        return Some(dir.to_string());
    }
    let filename = if dir == "." {
        iwadname.to_string()
    } else {
        format!("{}{}{}", dir, DIR_SEPARATOR_S, iwadname)
    };
    let filename_cstring = ::std::ffi::CString::new(filename.as_str()).unwrap();
    printf(
        b"Trying IWAD file:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        filename_cstring.as_ptr(),
    );
    if file_exists(&filename) { Some(filename) } else { None }
}
unsafe fn search_directory_for_iwad(
    dir: &str,
    mask: i32,
    mission: *mut GameMission_t,
) -> Option<String> {
    for iwad in iwads.iter() {
        if (1 as i32) << iwad.mission & mask == 0 as i32 {
            continue;
        }
        if let Some(filename) = check_directory_has_iwad(dir, iwad.name) {
            *mission = iwad.mission;
            return Some(filename);
        }
    }
    None
}
fn identify_iwad_by_name(name: &str, mask: i32) -> GameMission_t {
    let name = match name.rfind(DIR_SEPARATOR) {
        Some(pos) => &name[pos + 1..],
        None => name,
    };
    for iwad in iwads.iter() {
        if (1 as i32) << iwad.mission & mask == 0 as i32 {
            continue;
        }
        if name.eq_ignore_ascii_case(iwad.name) {
            return iwad.mission;
        }
    }
    none
}
unsafe fn build_iwad_dir_list() {
    add_iwad_dir(FILES_DIR);
    iwad_dirs_built = true;
}
#[no_mangle]
pub unsafe extern "C" fn D_FindWADByName(
    mut name: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let name_str = ::std::ffi::CStr::from_ptr(name).to_str().unwrap();
    if file_exists(name_str) {
        return name;
    }
    build_iwad_dir_list();
    for dir in iwad_dirs.iter() {
        if dir_is_file(dir, name_str) && file_exists(dir) {
            return ::std::ffi::CString::new(dir.as_str()).unwrap().into_raw();
        }
        let path = format!("{}{}{}", dir, DIR_SEPARATOR_S, name_str);
        if file_exists(&path) {
            return ::std::ffi::CString::new(path).unwrap().into_raw();
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
pub unsafe fn D_TryFindWADByName(
    mut filename: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let result = D_FindWADByName(filename);
    if !result.is_null() { result } else { filename }
}
pub unsafe fn D_FindIWAD(
    mut mask: i32,
    mut mission: *mut GameMission_t,
) -> *mut ::core::ffi::c_char {
    let iwadparm = M_CheckParmWithArgs("-iwad", 1 as i32);
    if iwadparm != 0 {
        let iwadfile = myargv[(iwadparm + 1 as i32) as usize].as_ptr()
            as *mut ::core::ffi::c_char;
        let result = D_FindWADByName(iwadfile);
        if result.is_null() {
            I_Error(&format!(
                "IWAD file '{}' not found!",
                myargv[(iwadparm + 1 as i32) as usize].to_str().unwrap(),
            ));
        }
        *mission = identify_iwad_by_name(
            ::std::ffi::CStr::from_ptr(result).to_str().unwrap(),
            mask,
        );
        result
    } else {
        printf(
            b"-iwad not specified, trying a few iwad names\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        build_iwad_dir_list();
        for dir in iwad_dirs.iter() {
            if let Some(found) = search_directory_for_iwad(dir, mask, mission) {
                return ::std::ffi::CString::new(found).unwrap().into_raw();
            }
        }
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_FindAllIWADs(
    mut mask: i32,
) -> *mut *const iwad_t {
    let mut result: Vec<*const iwad_t> = Vec::new();
    for iwad in iwads.iter() {
        if (1 as i32) << iwad.mission & mask == 0 as i32 {
            continue;
        }
        let name = ::std::ffi::CString::new(iwad.name).unwrap();
        if !D_FindWADByName(name.as_ptr() as *mut ::core::ffi::c_char).is_null() {
            result.push(iwad as *const iwad_t);
        }
    }
    result.push(::core::ptr::null());
    Box::leak(result.into_boxed_slice()).as_mut_ptr()
}
pub unsafe fn D_SaveGameIWADName(
    mut gamemission: GameMission_t,
) -> *mut ::core::ffi::c_char {
    for iwad in iwads.iter() {
        if gamemission == iwad.mission {
            return ::std::ffi::CString::new(iwad.name).unwrap().into_raw();
        }
    }
    ::std::ffi::CString::new("unknown.wad").unwrap().into_raw()
}
#[no_mangle]
pub unsafe extern "C" fn D_SuggestIWADName(
    mut mission: GameMission_t,
    mut mode: GameMode_t,
) -> *mut ::core::ffi::c_char {
    for iwad in iwads.iter() {
        if iwad.mission == mission && iwad.mode == mode {
            return ::std::ffi::CString::new(iwad.name).unwrap().into_raw();
        }
    }
    ::std::ffi::CString::new("unknown.wad").unwrap().into_raw()
}
pub unsafe fn D_SuggestGameName(
    mut mission: GameMission_t,
    mut mode: GameMode_t,
) -> *mut ::core::ffi::c_char {
    for iwad in iwads.iter() {
        if iwad.mission == mission && (mode == indetermined || iwad.mode == mode) {
            return ::std::ffi::CString::new(iwad.description).unwrap().into_raw();
        }
    }
    ::std::ffi::CString::new("Unknown game?").unwrap().into_raw()
}
