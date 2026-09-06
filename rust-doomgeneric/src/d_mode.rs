pub type skill_t = i32;
pub const sk_nightmare: skill_t = 4;
pub const sk_hard: skill_t = 3;
pub const sk_medium: skill_t = 2;
pub const sk_easy: skill_t = 1;
pub const sk_baby: skill_t = 0;
pub const sk_noitems: skill_t = -1;
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
pub type GameVersion_t = u32;
pub const exe_strife_1_31: GameVersion_t = 13;
pub const exe_strife_1_2: GameVersion_t = 12;
pub const exe_hexen_1_1: GameVersion_t = 11;
pub const exe_heretic_1_3: GameVersion_t = 10;
pub const exe_chex: GameVersion_t = 9;
pub const exe_final2: GameVersion_t = 8;
pub const exe_final: GameVersion_t = 7;
pub const exe_ultimate: GameVersion_t = 6;
pub const exe_hacx: GameVersion_t = 5;
pub const exe_doom_1_9: GameVersion_t = 4;
pub const exe_doom_1_8: GameVersion_t = 3;
pub const exe_doom_1_7: GameVersion_t = 2;
pub const exe_doom_1_666: GameVersion_t = 1;
pub const exe_doom_1_2: GameVersion_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub mission: GameMission_t,
    pub mode: GameMode_t,
    pub episode: i32,
    pub map: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub mission: GameMission_t,
    pub version: GameVersion_t,
}
static mut valid_modes: [C2RustUnnamed; 13] = [
    C2RustUnnamed {
        mission: pack_chex,
        mode: shareware,
        episode: 1 as i32,
        map: 5 as i32,
    },
    C2RustUnnamed {
        mission: doom,
        mode: shareware,
        episode: 1 as i32,
        map: 9 as i32,
    },
    C2RustUnnamed {
        mission: doom,
        mode: registered,
        episode: 3 as i32,
        map: 9 as i32,
    },
    C2RustUnnamed {
        mission: doom,
        mode: retail,
        episode: 4 as i32,
        map: 9 as i32,
    },
    C2RustUnnamed {
        mission: doom2,
        mode: commercial,
        episode: 1 as i32,
        map: 32 as i32,
    },
    C2RustUnnamed {
        mission: pack_tnt,
        mode: commercial,
        episode: 1 as i32,
        map: 32 as i32,
    },
    C2RustUnnamed {
        mission: pack_plut,
        mode: commercial,
        episode: 1 as i32,
        map: 32 as i32,
    },
    C2RustUnnamed {
        mission: pack_hacx,
        mode: commercial,
        episode: 1 as i32,
        map: 32 as i32,
    },
    C2RustUnnamed {
        mission: heretic,
        mode: shareware,
        episode: 1 as i32,
        map: 9 as i32,
    },
    C2RustUnnamed {
        mission: heretic,
        mode: registered,
        episode: 3 as i32,
        map: 9 as i32,
    },
    C2RustUnnamed {
        mission: heretic,
        mode: retail,
        episode: 5 as i32,
        map: 9 as i32,
    },
    C2RustUnnamed {
        mission: hexen,
        mode: commercial,
        episode: 1 as i32,
        map: 60 as i32,
    },
    C2RustUnnamed {
        mission: strife,
        mode: commercial,
        episode: 1 as i32,
        map: 34 as i32,
    },
];
#[no_mangle]
pub unsafe extern "C" fn D_ValidGameMode(
    mut mission: GameMission_t,
    mut mode: GameMode_t,
) -> bool {
    let mut i: i32 = 0;
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed; 13]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
    {
        if valid_modes[i as usize].mode as u32
            == mode as u32
            && valid_modes[i as usize].mission as u32
                == mission as u32
        {
            return true;
        }
        i += 1;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn D_ValidEpisodeMap(
    mut mission: GameMission_t,
    mut mode: GameMode_t,
    mut episode: i32,
    mut map: i32,
) -> bool {
    let mut i: i32 = 0;
    if mission as u32
        == heretic as i32 as u32
    {
        if mode as u32
            == retail as i32 as u32
            && episode == 6 as i32
        {
            return map >= 1 as i32 && map <= 3 as i32
        } else if mode as u32
            == registered as i32 as u32
            && episode == 4 as i32
        {
            return map == 1 as i32
        }
    }
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed; 13]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
    {
        if mission as u32
            == valid_modes[i as usize].mission as u32
            && mode as u32
                == valid_modes[i as usize].mode as u32
        {
            return episode >= 1 as i32
                && episode <= valid_modes[i as usize].episode
                && map >= 1 as i32 && map <= valid_modes[i as usize].map;
        }
        i += 1;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn D_GetNumEpisodes(
    mut mission: GameMission_t,
    mut mode: GameMode_t,
) -> i32 {
    let mut episode: i32 = 0;
    episode = 1 as i32;
    while D_ValidEpisodeMap(mission, mode, episode, 1 as i32) {
        episode += 1;
    }
    return episode - 1 as i32;
}
static mut valid_versions: [C2RustUnnamed_0; 10] = [
    C2RustUnnamed_0 {
        mission: doom,
        version: exe_doom_1_9,
    },
    C2RustUnnamed_0 {
        mission: doom,
        version: exe_hacx,
    },
    C2RustUnnamed_0 {
        mission: doom,
        version: exe_ultimate,
    },
    C2RustUnnamed_0 {
        mission: doom,
        version: exe_final,
    },
    C2RustUnnamed_0 {
        mission: doom,
        version: exe_final2,
    },
    C2RustUnnamed_0 {
        mission: doom,
        version: exe_chex,
    },
    C2RustUnnamed_0 {
        mission: heretic,
        version: exe_heretic_1_3,
    },
    C2RustUnnamed_0 {
        mission: hexen,
        version: exe_hexen_1_1,
    },
    C2RustUnnamed_0 {
        mission: strife,
        version: exe_strife_1_2,
    },
    C2RustUnnamed_0 {
        mission: strife,
        version: exe_strife_1_31,
    },
];
#[no_mangle]
pub unsafe extern "C" fn D_ValidGameVersion(
    mut mission: GameMission_t,
    mut version: GameVersion_t,
) -> bool {
    let mut i: i32 = 0;
    if mission as u32
        == doom2 as i32 as u32
        || mission as u32
            == pack_plut as i32 as u32
        || mission as u32
            == pack_tnt as i32 as u32
        || mission as u32
            == pack_hacx as i32 as u32
        || mission as u32
            == pack_chex as i32 as u32
    {
        mission = doom;
    }
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as usize)
    {
        if valid_versions[i as usize].mission as u32
            == mission as u32
            && valid_versions[i as usize].version as u32
                == version as u32
        {
            return true;
        }
        i += 1;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn D_IsEpisodeMap(mut mission: GameMission_t) -> bool {
    match mission as u32 {
        0 | 6 | 4 => return true,
        9 | 7 | 1 | 5 | 2 | 3 | 8 | _ => return false,
    };
}
pub unsafe fn D_GameMissionString(
    mut mission: GameMission_t,
) -> *mut ::core::ffi::c_char {
    match mission as u32 {
        0 => {
            return b"doom\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        1 => {
            return b"doom2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        2 => {
            return b"tnt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3 => {
            return b"plutonia\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        5 => {
            return b"hacx\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        4 => {
            return b"chex\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        6 => {
            return b"heretic\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 => {
            return b"hexen\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        8 => {
            return b"strife\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        9 | _ => {
            return b"none\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
