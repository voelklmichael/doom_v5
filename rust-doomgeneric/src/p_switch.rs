use crate::src::p_spec::{button_t};
use crate::src::p_mobj::{degenmobj_t, line_t};
use crate::src::p_mobj::{mobj_t};
use crate::src::i_system::I_Error;
use crate::src::p_doors::EV_DoLockedDoor;
use crate::src::g_game::G_SecretExitLevel;
use crate::src::p_ceilng::EV_DoCeiling;
use crate::src::p_floor::EV_BuildStairs;
use crate::src::p_lights::EV_LightTurnOn;
use crate::src::p_plats::EV_DoPlat;
use crate::src::g_game::G_ExitLevel;
use crate::src::p_doors::EV_DoDoor;
use crate::src::p_floor::EV_DoFloor;
use crate::src::p_setup::sides;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::p_doors::EV_VerticalDoor;
use crate::src::p_spec::EV_DoDonut;
use crate::src::r_data::R_TextureNumForName;
use crate::src::sounds::{sfx_swtchn, sfx_swtchx};
use crate::src::d_mode::{commercial, registered, retail};
use crate::src::p_plats::{blazeDWUS, downWaitUpStay, raiseAndChange, raiseToNearestAndChange};
use crate::src::p_doors::{vld_blazeClose, vld_blazeOpen, vld_blazeRaise, vld_close, vld_normal, vld_open};
use crate::src::p_floor::{lowerFloor, lowerFloorToLowest, raiseFloor, raiseFloor512, raiseFloorCrush, raiseFloorToNearest, raiseFloorTurbo, turboLower};
use crate::src::p_floor::{build8, turbo16};
use crate::src::p_ceilng::{crushAndRaise, lowerToFloor};
use crate::src::p_spec::ML_SECRET;


pub type bwhere_e = u32;
pub const bottom: bwhere_e = 2;
pub const middle: bwhere_e = 1;
pub const top: bwhere_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switchlist_t {
    pub name1: [::core::ffi::c_char; 9],
    pub name2: [::core::ffi::c_char; 9],
    pub episode: i16,
}
pub const MAXSWITCHES: i32 = 50;
pub const MAXBUTTONS: i32 = 16;
pub const BUTTONTIME: i32 = 35;
#[no_mangle]
pub static mut alphSwitchList: [switchlist_t; 41] = unsafe {
    [
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BRCOM\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BRCOM\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BRN1\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BRN1\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BRN2\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BRN2\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BRNGN\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BRNGN\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BROWN\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BROWN\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1COMM\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2COMM\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1COMP\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2COMP\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1DIRT\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2DIRT\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1EXIT\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2EXIT\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1GRAY\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2GRAY\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1GRAY1\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2GRAY1\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1METAL\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2METAL\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1PIPE\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2PIPE\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1SLAD\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2SLAD\0\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1STARG\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2STARG\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1STON1\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2STON1\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1STON2\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2STON2\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1STONE\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2STONE\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1STRTN\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2STRTN\0"),
            episode: 1 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BLUE\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BLUE\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1CMT\0\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2CMT\0\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1GARG\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2GARG\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1GSTON\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2GSTON\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1HOT\0\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2HOT\0\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1LION\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2LION\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1SATYR\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2SATYR\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1SKIN\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2SKIN\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1VINE\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2VINE\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1WOOD\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2WOOD\0\0"),
            episode: 2 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1PANEL\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2PANEL\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1ROCK\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2ROCK\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1MET2\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2MET2\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1WDMET\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2WDMET\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1BRIK\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2BRIK\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1MOD1\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2MOD1\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1ZIM\0\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2ZIM\0\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1STON6\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2STON6\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1TEK\0\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2TEK\0\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1MARB\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2MARB\0\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW1SKULL\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SW2SKULL\0"),
            episode: 3 as i16,
        },
        switchlist_t {
            name1: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"\0\0\0\0\0\0\0\0\0"),
            name2: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"\0\0\0\0\0\0\0\0\0"),
            episode: 0 as i16,
        },
    ]
};
#[no_mangle]
pub static mut switchlist: [i32; 100] = [0; 100];
#[no_mangle]
pub static mut numswitches: i32 = 0;
pub static mut buttonlist: [button_t; 16] = [button_t {
    line: ::core::ptr::null::<line_t>() as *mut line_t,
    where_0: top,
    btexture: 0,
    btimer: 0,
    soundorg: ::core::ptr::null::<degenmobj_t>() as *mut degenmobj_t,
}; 16];
pub unsafe fn P_InitSwitchList() {
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    let mut episode: i32 = 0;
    episode = 1 as i32;
    if gamemode as u32
        == registered as i32 as u32
        || gamemode as u32
            == retail as i32 as u32
    {
        episode = 2 as i32;
    } else if gamemode as u32
        == commercial as i32 as u32
    {
        episode = 3 as i32;
    }
    index = 0 as i32;
    i = 0 as i32;
    while i < MAXSWITCHES {
        if alphSwitchList[i as usize].episode == 0 {
            numswitches = index / 2 as i32;
            switchlist[index as usize] = -(1 as i32);
            break;
        } else {
            if alphSwitchList[i as usize].episode as i32 <= episode {
                let fresh0 = index;
                index = index + 1;
                switchlist[fresh0 as usize] = R_TextureNumForName(
                    &raw mut (*(&raw mut alphSwitchList as *mut switchlist_t)
                        .offset(i as isize))
                        .name1 as *mut ::core::ffi::c_char,
                );
                let fresh1 = index;
                index = index + 1;
                switchlist[fresh1 as usize] = R_TextureNumForName(
                    &raw mut (*(&raw mut alphSwitchList as *mut switchlist_t)
                        .offset(i as isize))
                        .name2 as *mut ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    }
}
pub unsafe fn P_StartButton(
    mut line: *mut line_t,
    mut w: bwhere_e,
    mut texture: i32,
    mut time: i32,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXBUTTONS {
        if buttonlist[i as usize].btimer != 0 && buttonlist[i as usize].line == line {
            return;
        }
        i += 1;
    }
    i = 0 as i32;
    while i < MAXBUTTONS {
        if buttonlist[i as usize].btimer == 0 {
            buttonlist[i as usize].line = line;
            buttonlist[i as usize].where_0 = w;
            buttonlist[i as usize].btexture = texture;
            buttonlist[i as usize].btimer = time;
            buttonlist[i as usize].soundorg = &raw mut (*(*line).frontsector).soundorg;
            return;
        }
        i += 1;
    }
    I_Error("P_StartButton: no button slots left!");
}
pub unsafe fn P_ChangeSwitchTexture(
    mut line: *mut line_t,
    mut useAgain: i32,
) {
    let mut texTop: i32 = 0;
    let mut texMid: i32 = 0;
    let mut texBot: i32 = 0;
    let mut i: i32 = 0;
    let mut sound: i32 = 0;
    if useAgain == 0 {
        (*line).special = 0 as i16;
    }
    texTop = (*sides.offset((*line).sidenum[0 as i32 as usize] as isize))
        .toptexture as i32;
    texMid = (*sides.offset((*line).sidenum[0 as i32 as usize] as isize))
        .midtexture as i32;
    texBot = (*sides.offset((*line).sidenum[0 as i32 as usize] as isize))
        .bottomtexture as i32;
    sound = sfx_swtchn as i32;
    if (*line).special as i32 == 11 as i32 {
        sound = sfx_swtchx as i32;
    }
    i = 0 as i32;
    while i < numswitches * 2 as i32 {
        if switchlist[i as usize] == texTop {
            S_StartSound(
                (*(&raw mut buttonlist as *mut button_t)).soundorg
                    as *mut ::core::ffi::c_void,
                sound,
            );
            (*sides.offset((*line).sidenum[0 as i32 as usize] as isize))
                .toptexture = switchlist[(i ^ 1 as i32) as usize]
                as i16;
            if useAgain != 0 {
                P_StartButton(line, top, switchlist[i as usize], BUTTONTIME);
            }
            return;
        } else if switchlist[i as usize] == texMid {
            S_StartSound(
                (*(&raw mut buttonlist as *mut button_t)).soundorg
                    as *mut ::core::ffi::c_void,
                sound,
            );
            (*sides.offset((*line).sidenum[0 as i32 as usize] as isize))
                .midtexture = switchlist[(i ^ 1 as i32) as usize]
                as i16;
            if useAgain != 0 {
                P_StartButton(line, middle, switchlist[i as usize], BUTTONTIME);
            }
            return;
        } else if switchlist[i as usize] == texBot {
            S_StartSound(
                (*(&raw mut buttonlist as *mut button_t)).soundorg
                    as *mut ::core::ffi::c_void,
                sound,
            );
            (*sides.offset((*line).sidenum[0 as i32 as usize] as isize))
                .bottomtexture = switchlist[(i ^ 1 as i32) as usize]
                as i16;
            if useAgain != 0 {
                P_StartButton(line, bottom, switchlist[i as usize], BUTTONTIME);
            }
            return;
        }
        i += 1;
    }
}
pub unsafe fn P_UseSpecialLine(
    mut thing: *mut mobj_t,
    mut line: *mut line_t,
    mut side: i32,
) -> bool {
    if side != 0 {
        match (*line).special as i32 {
            124 => {}
            _ => return false,
        }
    }
    if (*thing).player.is_null() {
        if (*line).flags as i32 & ML_SECRET != 0 {
            return false;
        }
        let mut current_block_6: u64;
        match (*line).special as i32 {
            1 => {
                current_block_6 = 3640593987805443782;
            }
            32 => {
                current_block_6 = 12497116384748537712;
            }
            33 => {
                current_block_6 = 12497116384748537712;
            }
            34 => {
                current_block_6 = 3514215265213398008;
            }
            _ => return false,
        }
        match current_block_6 {
            12497116384748537712 => {
                current_block_6 = 3514215265213398008;
            }
            _ => {}
        }
        match current_block_6 {
            3514215265213398008 => {}
            _ => {}
        }
    }
    let mut current_block_108: u64;
    match (*line).special as i32 {
        1 => {
            current_block_108 = 9599537275859496075;
        }
        26 => {
            current_block_108 = 9599537275859496075;
        }
        27 => {
            current_block_108 = 4020771665460505868;
        }
        28 => {
            current_block_108 = 14541717319785412967;
        }
        31 => {
            current_block_108 = 9823790708098527527;
        }
        32 => {
            current_block_108 = 16744586327905341259;
        }
        33 => {
            current_block_108 = 346168964044254087;
        }
        34 => {
            current_block_108 = 7551167036031974558;
        }
        117 => {
            current_block_108 = 16004491317120851547;
        }
        118 => {
            current_block_108 = 6634390297149606533;
        }
        7 => {
            if EV_BuildStairs(line, build8) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        9 => {
            if EV_DoDonut(line) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        11 => {
            P_ChangeSwitchTexture(line, 0 as i32);
            G_ExitLevel();
            current_block_108 = 16981061190961355901;
        }
        14 => {
            if EV_DoPlat(line, raiseAndChange, 32 as i32) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        15 => {
            if EV_DoPlat(line, raiseAndChange, 24 as i32) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        18 => {
            if EV_DoFloor(line, raiseFloorToNearest) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        20 => {
            if EV_DoPlat(line, raiseToNearestAndChange, 0 as i32) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        21 => {
            if EV_DoPlat(line, downWaitUpStay, 0 as i32) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        23 => {
            if EV_DoFloor(line, lowerFloorToLowest) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        29 => {
            if EV_DoDoor(line, vld_normal) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        41 => {
            if EV_DoCeiling(line, lowerToFloor) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        71 => {
            if EV_DoFloor(line, turboLower) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        49 => {
            if EV_DoCeiling(line, crushAndRaise) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        50 => {
            if EV_DoDoor(line, vld_close) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        51 => {
            P_ChangeSwitchTexture(line, 0 as i32);
            G_SecretExitLevel();
            current_block_108 = 16981061190961355901;
        }
        55 => {
            if EV_DoFloor(line, raiseFloorCrush) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        101 => {
            if EV_DoFloor(line, raiseFloor) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        102 => {
            if EV_DoFloor(line, lowerFloor) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        103 => {
            if EV_DoDoor(line, vld_open) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        111 => {
            if EV_DoDoor(line, vld_blazeRaise) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        112 => {
            if EV_DoDoor(line, vld_blazeOpen) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        113 => {
            if EV_DoDoor(line, vld_blazeClose) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        122 => {
            if EV_DoPlat(line, blazeDWUS, 0 as i32) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        127 => {
            if EV_BuildStairs(line, turbo16) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        131 => {
            if EV_DoFloor(line, raiseFloorTurbo) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        133 => {
            current_block_108 = 6707790765423050264;
        }
        135 | 137 => {
            current_block_108 = 6707790765423050264;
        }
        140 => {
            if EV_DoFloor(line, raiseFloor512) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        42 => {
            if EV_DoDoor(line, vld_close) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        43 => {
            if EV_DoCeiling(line, lowerToFloor) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        45 => {
            if EV_DoFloor(line, lowerFloor) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        60 => {
            if EV_DoFloor(line, lowerFloorToLowest) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        61 => {
            if EV_DoDoor(line, vld_open) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        62 => {
            if EV_DoPlat(line, downWaitUpStay, 1 as i32) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        63 => {
            if EV_DoDoor(line, vld_normal) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        64 => {
            if EV_DoFloor(line, raiseFloor) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        66 => {
            if EV_DoPlat(line, raiseAndChange, 24 as i32) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        67 => {
            if EV_DoPlat(line, raiseAndChange, 32 as i32) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        65 => {
            if EV_DoFloor(line, raiseFloorCrush) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        68 => {
            if EV_DoPlat(line, raiseToNearestAndChange, 0 as i32) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        69 => {
            if EV_DoFloor(line, raiseFloorToNearest) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        70 => {
            if EV_DoFloor(line, turboLower) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        114 => {
            if EV_DoDoor(line, vld_blazeRaise) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        115 => {
            if EV_DoDoor(line, vld_blazeOpen) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        116 => {
            if EV_DoDoor(line, vld_blazeClose) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        123 => {
            if EV_DoPlat(line, blazeDWUS, 0 as i32) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        132 => {
            if EV_DoFloor(line, raiseFloorTurbo) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        99 => {
            current_block_108 = 16848555411549253182;
        }
        134 | 136 => {
            current_block_108 = 16848555411549253182;
        }
        138 => {
            EV_LightTurnOn(line, 255 as i32);
            P_ChangeSwitchTexture(line, 1 as i32);
            current_block_108 = 16981061190961355901;
        }
        139 => {
            EV_LightTurnOn(line, 35 as i32);
            P_ChangeSwitchTexture(line, 1 as i32);
            current_block_108 = 16981061190961355901;
        }
        _ => {
            current_block_108 = 16981061190961355901;
        }
    }
    match current_block_108 {
        9599537275859496075 => {
            current_block_108 = 4020771665460505868;
        }
        6707790765423050264 => {
            if EV_DoLockedDoor(line, vld_blazeOpen, thing) != 0 {
                P_ChangeSwitchTexture(line, 0 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        16848555411549253182 => {
            if EV_DoLockedDoor(line, vld_blazeOpen, thing) != 0 {
                P_ChangeSwitchTexture(line, 1 as i32);
            }
            current_block_108 = 16981061190961355901;
        }
        _ => {}
    }
    match current_block_108 {
        4020771665460505868 => {
            current_block_108 = 14541717319785412967;
        }
        _ => {}
    }
    match current_block_108 {
        14541717319785412967 => {
            current_block_108 = 9823790708098527527;
        }
        _ => {}
    }
    match current_block_108 {
        9823790708098527527 => {
            current_block_108 = 16744586327905341259;
        }
        _ => {}
    }
    match current_block_108 {
        16744586327905341259 => {
            current_block_108 = 346168964044254087;
        }
        _ => {}
    }
    match current_block_108 {
        346168964044254087 => {
            current_block_108 = 7551167036031974558;
        }
        _ => {}
    }
    match current_block_108 {
        7551167036031974558 => {
            current_block_108 = 16004491317120851547;
        }
        _ => {}
    }
    match current_block_108 {
        16004491317120851547 => {
            current_block_108 = 6634390297149606533;
        }
        _ => {}
    }
    match current_block_108 {
        6634390297149606533 => {
            EV_VerticalDoor(line, thing);
        }
        _ => {}
    }
    return true;
}
