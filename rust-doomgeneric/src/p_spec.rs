use crate::src::r_defs::{side_t};
use crate::src::p_mobj::{thinker_t, sector_t, degenmobj_t, line_t};
use crate::src::d_player::{player_t};
use crate::src::p_mobj::{mobj_t};
use crate::src::i_system::I_Error;
use crate::src::m_argv::{myargv, M_CheckParmWithArgs};
use crate::src::w_wad::{wad_name8_to_string, W_CheckNumForName};
use crate::src::r_data::R_CheckTextureNumForName;
use crate::src::p_lights::P_SpawnFireFlicker;
use crate::src::p_lights::P_SpawnLightFlash;
use crate::src::p_lights::P_SpawnStrobeFlash;
use crate::src::p_lights::EV_StartLightStrobing;
use crate::src::p_lights::EV_TurnTagLightsOff;
use crate::src::p_lights::P_SpawnGlowingLight;
use crate::src::p_switch::buttonlist;
use crate::src::p_switch::P_ChangeSwitchTexture;
use crate::src::p_plats::activeplats;
use crate::src::p_plats::EV_StopPlat;
use crate::src::p_doors::P_SpawnDoorCloseIn30;
use crate::src::p_doors::P_SpawnDoorRaiseIn5Mins;
use crate::src::p_ceilng::EV_CeilingCrushStop;
use crate::src::p_telept::EV_Teleport;
use crate::src::r_data::numflats;
use crate::src::g_game::G_SecretExitLevel;
use crate::src::g_game::totalsecret;
use crate::src::p_ceilng::EV_DoCeiling;
use crate::src::p_ceilng::activeceilings;
use crate::src::p_floor::EV_BuildStairs;
use crate::src::p_lights::EV_LightTurnOn;
use crate::src::p_plats::EV_DoPlat;
use crate::src::r_data::flattranslation;
use crate::src::r_data::texturetranslation;
use crate::src::g_game::G_ExitLevel;
use crate::src::g_game::timelimit;
use crate::src::m_misc::M_StrToInt;
use crate::src::p_doors::EV_DoDoor;
use crate::src::p_floor::EV_DoFloor;
use crate::src::p_setup::numlines;
use crate::src::p_inter::P_DamageMobj;
use crate::src::p_setup::lines;
use crate::src::p_setup::numsectors;
use crate::src::p_setup::sides;
use crate::src::p_tick::P_AddThinker;
use crate::src::g_game::deathmatch;
use crate::src::m_random::P_Random;
use crate::src::p_setup::sectors;
use crate::src::p_tick::leveltime;
use crate::src::s_sound::S_StartSound;
use crate::src::r_data::R_FlatNumForName;
use crate::src::r_data::R_TextureNumForName;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVSPEC;
use crate::src::sounds::sfx_swtchn;
use crate::src::d_player::pw_ironfeet;
use crate::src::d_player::CF_GODMODE;
use libc::memset;
use crate::src::i_system::{fprintf, stderr};
use crate::src::p_mobj::mobjtype_t;
use crate::src::p_mobj::ThinkerFn;
use crate::src::p_plats::{blazeDWUS, downWaitUpStay, perpetualRaise, plattype_e, raiseToNearestAndChange};
use crate::src::p_plats::plat_e;
use crate::src::p_doors::{vld_blazeClose, vld_blazeOpen, vld_blazeRaise, vld_close, vld_close30ThenOpen, vld_normal, vld_open};
use crate::src::p_floor::{donutRaise, floor_e, lowerAndChange, lowerFloor, lowerFloorToLowest, raiseFloor, raiseFloor24, raiseFloor24AndChange, raiseFloorCrush, raiseFloorToNearest, raiseFloorTurbo, raiseToTexture, turboLower};
use crate::src::p_floor::{build8, turbo16};
use crate::src::p_ceilng::{ceiling_e, crushAndRaise, fastCrushAndRaise, lowerAndCrush, raiseToHighest, silentCrushAndRaise};
use crate::src::p_switch::bwhere_e;
use crate::src::m_fixed::fixed_t;
use crate::src::doomdef::boolean;
use crate::src::stdint_types::size_t;

use crate::src::p_floor::T_MoveFloor;
pub const NUMMOBJTYPES: mobjtype_t = 137;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct anim_t {
    pub istexture: boolean,
    pub picnum: i32,
    pub basepic: i32,
    pub numpics: i32,
    pub speed: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct animdef_t {
    pub istexture: i32,
    pub endname: [::core::ffi::c_char; 9],
    pub startname: [::core::ffi::c_char; 9],
    pub speed: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct button_t {
    pub line: *mut line_t,
    pub where_0: bwhere_e,
    pub btexture: i32,
    pub btimer: i32,
    pub soundorg: *mut degenmobj_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plat_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub speed: fixed_t,
    pub low: fixed_t,
    pub high: fixed_t,
    pub wait: i32,
    pub count: i32,
    pub status: plat_e,
    pub oldstatus: plat_e,
    pub crush: bool,
    pub tag: i32,
    pub type_0: plattype_e,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ceiling_t {
    pub thinker: thinker_t,
    pub type_0: ceiling_e,
    pub sector: *mut sector_t,
    pub bottomheight: fixed_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub crush: bool,
    pub direction: i32,
    pub tag: i32,
    pub olddirection: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floormove_t {
    pub thinker: thinker_t,
    pub type_0: floor_e,
    pub crush: bool,
    pub sector: *mut sector_t,
    pub direction: i32,
    pub newspecial: i32,
    pub texture: i16,
    pub floordestheight: fixed_t,
    pub speed: fixed_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const INT_MAX: i32 = __INT_MAX__;
pub const TICRATE: i32 = 35 as i32;
pub const ML_TWOSIDED: i32 = 4 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub const FRACUNIT: i32 = (1 as i32) << FRACBITS;
pub const FASTDARK: i32 = 15 as i32;
pub const SLOWDARK: i32 = 35 as i32;
pub const MAXBUTTONS: i32 = 16 as i32;
pub const MAXPLATS: i32 = 30 as i32;
pub const MAXCEILINGS: i32 = 30 as i32;
pub const FLOORSPEED: i32 = FRACUNIT;
#[no_mangle]
pub static mut animdefs: [animdef_t; 23] = unsafe {
    [
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"NUKAGE3\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"NUKAGE1\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FWATER4\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FWATER1\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SWATER4\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SWATER1\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"LAVA4\0\0\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"LAVA1\0\0\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BLOOD3\0\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BLOOD1\0\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"RROCK08\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"RROCK05\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLIME04\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLIME01\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLIME08\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLIME05\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: false_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLIME12\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLIME09\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BLODGR4\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BLODGR1\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLADRIP3\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SLADRIP1\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BLODRIP4\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BLODRIP1\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIREWALL\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIREWALA\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"GSTFONT3\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"GSTFONT1\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIRELAVA\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIRELAV3\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIREMAG3\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIREMAG1\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIREBLU2\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"FIREBLU1\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"ROCKRED3\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"ROCKRED1\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BFALL4\0\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"BFALL1\0\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SFALL4\0\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"SFALL1\0\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"WFALL4\0\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"WFALL1\0\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: true_0,
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"DBRAIN4\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"DBRAIN1\0\0"),
            speed: 8 as i32,
        },
        animdef_t {
            istexture: -(1 as i32),
            endname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"\0\0\0\0\0\0\0\0\0"),
            startname: ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"\0\0\0\0\0\0\0\0\0"),
            speed: 0 as i32,
        },
    ]
};
#[no_mangle]
pub static mut anims: [anim_t; 32] = [anim_t {
    istexture: 0,
    picnum: 0,
    basepic: 0,
    numpics: 0,
    speed: 0,
}; 32];
#[no_mangle]
pub static mut lastanim: *mut anim_t = ::core::ptr::null::<anim_t>() as *mut anim_t;
pub const MAXLINEANIMS: i32 = 64 as i32;
pub unsafe fn P_InitPicAnims() {
    let mut i: i32 = 0;
    lastanim = &raw mut anims as *mut anim_t;
    let mut current_block_13: u64;
    i = 0 as i32;
    while animdefs[i as usize].istexture != -(1 as i32) {
        let mut startname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut endname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        startname = &raw mut (*(&raw mut animdefs as *mut animdef_t).offset(i as isize))
            .startname as *mut ::core::ffi::c_char;
        endname = &raw mut (*(&raw mut animdefs as *mut animdef_t).offset(i as isize))
            .endname as *mut ::core::ffi::c_char;
        if animdefs[i as usize].istexture != 0 {
            if R_CheckTextureNumForName(startname) == -(1 as i32) {
                current_block_13 = 12237857397564741460;
            } else {
                (*lastanim).picnum = R_TextureNumForName(endname);
                (*lastanim).basepic = R_TextureNumForName(startname);
                current_block_13 = 11650488183268122163;
            }
        } else if W_CheckNumForName(
            &wad_name8_to_string(startname),
        ) == -(1 as i32)
        {
            current_block_13 = 12237857397564741460;
        } else {
            (*lastanim).picnum = R_FlatNumForName(endname);
            (*lastanim).basepic = R_FlatNumForName(startname);
            current_block_13 = 11650488183268122163;
        }
        match current_block_13 {
            11650488183268122163 => {
                (*lastanim).istexture = animdefs[i as usize].istexture as boolean;
                (*lastanim).numpics = (*lastanim).picnum - (*lastanim).basepic
                    + 1 as i32;
                if (*lastanim).numpics < 2 as i32 {
                    I_Error(&format!(
                        "P_InitPicAnims: bad cycle from {} to {}",
                        wad_name8_to_string(startname),
                        wad_name8_to_string(endname),
                    ));
                }
                (*lastanim).speed = animdefs[i as usize].speed;
                lastanim = lastanim.offset(1);
            }
            _ => {}
        }
        i += 1;
    }
}
pub unsafe fn getSide(
    mut currentSector: i32,
    mut line: i32,
    mut side: i32,
) -> *mut side_t {
    return sides
        .offset(
            *(&raw mut (**(*sectors.offset(currentSector as isize))
                .lines
                .offset(line as isize))
                .sidenum as *mut i16)
                .offset(side as isize) as isize,
        ) as *mut side_t;
}
pub unsafe fn getSector(
    mut currentSector: i32,
    mut line: i32,
    mut side: i32,
) -> *mut sector_t {
    return (*sides
        .offset(
            (**(*sectors.offset(currentSector as isize)).lines.offset(line as isize))
                .sidenum[side as usize] as isize,
        ))
        .sector;
}
pub unsafe fn twoSided(
    mut sector: i32,
    mut line: i32,
) -> i32 {
    return (**(*sectors.offset(sector as isize)).lines.offset(line as isize)).flags
        as i32 & ML_TWOSIDED;
}
pub unsafe fn getNextSector(
    mut line: *mut line_t,
    mut sec: *mut sector_t,
) -> *mut sector_t {
    if (*line).flags as i32 & ML_TWOSIDED == 0 {
        return ::core::ptr::null_mut::<sector_t>();
    }
    if (*line).frontsector == sec {
        return (*line).backsector;
    }
    return (*line).frontsector;
}
pub unsafe fn P_FindLowestFloorSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: i32 = 0;
    let mut check: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut other: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut floor: fixed_t = (*sec).floorheight;
    i = 0 as i32;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).floorheight < floor {
                floor = (*other).floorheight;
            }
        }
        i += 1;
    }
    return floor;
}
pub unsafe fn P_FindHighestFloorSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: i32 = 0;
    let mut check: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut other: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut floor: fixed_t = -(500 as fixed_t) * FRACUNIT;
    i = 0 as i32;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).floorheight > floor {
                floor = (*other).floorheight;
            }
        }
        i += 1;
    }
    return floor;
}
pub const MAX_ADJOINING_SECTORS: i32 = 20 as i32;
pub unsafe fn P_FindNextHighestFloor(
    mut sec: *mut sector_t,
    mut currentheight: i32,
) -> fixed_t {
    let mut i: i32 = 0;
    let mut h: i32 = 0;
    let mut min: i32 = 0;
    let mut check: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut other: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut height: fixed_t = currentheight as fixed_t;
    let mut heightlist: [fixed_t; 22] = [0; 22];
    i = 0 as i32;
    h = 0 as i32;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).floorheight > height {
                if h == MAX_ADJOINING_SECTORS + 1 as i32 {
                    height = (*other).floorheight;
                } else if h == MAX_ADJOINING_SECTORS + 2 as i32 {
                    I_Error("Sector with more than 22 adjoining sectors. Vanilla will crash here");
                }
                let fresh1 = h;
                h = h + 1;
                heightlist[fresh1 as usize] = (*other).floorheight;
            }
        }
        i += 1;
    }
    if h == 0 {
        return currentheight as fixed_t;
    }
    min = heightlist[0 as i32 as usize] as i32;
    i = 1 as i32;
    while i < h {
        if heightlist[i as usize] < min {
            min = heightlist[i as usize] as i32;
        }
        i += 1;
    }
    return min as fixed_t;
}
pub unsafe fn P_FindLowestCeilingSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: i32 = 0;
    let mut check: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut other: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut height: fixed_t = INT_MAX;
    i = 0 as i32;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).ceilingheight < height {
                height = (*other).ceilingheight;
            }
        }
        i += 1;
    }
    return height;
}
pub unsafe fn P_FindHighestCeilingSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: i32 = 0;
    let mut check: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut other: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut height: fixed_t = 0 as fixed_t;
    i = 0 as i32;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).ceilingheight > height {
                height = (*other).ceilingheight;
            }
        }
        i += 1;
    }
    return height;
}
pub unsafe fn P_FindSectorFromLineTag(
    mut line: *mut line_t,
    mut start: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = start + 1 as i32;
    while i < numsectors {
        if (*sectors.offset(i as isize)).tag as i32
            == (*line).tag as i32
        {
            return i;
        }
        i += 1;
    }
    return -(1 as i32);
}
pub unsafe fn P_FindMinSurroundingLight(
    mut sector: *mut sector_t,
    mut max: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut min: i32 = 0;
    let mut line: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut check: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    min = max;
    i = 0 as i32;
    while i < (*sector).linecount {
        line = *(*sector).lines.offset(i as isize) as *mut line_t;
        check = getNextSector(line, sector);
        if !check.is_null() {
            if ((*check).lightlevel as i32) < min {
                min = (*check).lightlevel as i32;
            }
        }
        i += 1;
    }
    return min;
}
pub unsafe fn P_CrossSpecialLine(
    mut linenum: i32,
    mut side: i32,
    mut thing: *mut mobj_t,
) {
    let mut line: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut ok: i32 = 0;
    line = lines.offset(linenum as isize) as *mut line_t;
    if (*thing).player.is_null() {
        match (*thing).type_0 as u32 {
            33 | 34 | 35 | 31 | 32 | 16 => return,
            _ => {}
        }
        ok = 0 as i32;
        match (*line).special as i32 {
            39 | 97 | 125 | 126 | 4 | 10 | 88 => {
                ok = 1 as i32;
            }
            _ => {}
        }
        if ok == 0 {
            return;
        }
    }
    match (*line).special as i32 {
        2 => {
            EV_DoDoor(line, vld_open);
            (*line).special = 0 as i16;
        }
        3 => {
            EV_DoDoor(line, vld_close);
            (*line).special = 0 as i16;
        }
        4 => {
            EV_DoDoor(line, vld_normal);
            (*line).special = 0 as i16;
        }
        5 => {
            EV_DoFloor(line, raiseFloor);
            (*line).special = 0 as i16;
        }
        6 => {
            EV_DoCeiling(line, fastCrushAndRaise);
            (*line).special = 0 as i16;
        }
        8 => {
            EV_BuildStairs(line, build8);
            (*line).special = 0 as i16;
        }
        10 => {
            EV_DoPlat(line, downWaitUpStay, 0 as i32);
            (*line).special = 0 as i16;
        }
        12 => {
            EV_LightTurnOn(line, 0 as i32);
            (*line).special = 0 as i16;
        }
        13 => {
            EV_LightTurnOn(line, 255 as i32);
            (*line).special = 0 as i16;
        }
        16 => {
            EV_DoDoor(line, vld_close30ThenOpen);
            (*line).special = 0 as i16;
        }
        17 => {
            EV_StartLightStrobing(line);
            (*line).special = 0 as i16;
        }
        19 => {
            EV_DoFloor(line, lowerFloor);
            (*line).special = 0 as i16;
        }
        22 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0 as i32);
            (*line).special = 0 as i16;
        }
        25 => {
            EV_DoCeiling(line, crushAndRaise);
            (*line).special = 0 as i16;
        }
        30 => {
            EV_DoFloor(line, raiseToTexture);
            (*line).special = 0 as i16;
        }
        35 => {
            EV_LightTurnOn(line, 35 as i32);
            (*line).special = 0 as i16;
        }
        36 => {
            EV_DoFloor(line, turboLower);
            (*line).special = 0 as i16;
        }
        37 => {
            EV_DoFloor(line, lowerAndChange);
            (*line).special = 0 as i16;
        }
        38 => {
            EV_DoFloor(line, lowerFloorToLowest);
            (*line).special = 0 as i16;
        }
        39 => {
            EV_Teleport(line, side, thing);
            (*line).special = 0 as i16;
        }
        40 => {
            EV_DoCeiling(line, raiseToHighest);
            EV_DoFloor(line, lowerFloorToLowest);
            (*line).special = 0 as i16;
        }
        44 => {
            EV_DoCeiling(line, lowerAndCrush);
            (*line).special = 0 as i16;
        }
        52 => {
            G_ExitLevel();
        }
        53 => {
            EV_DoPlat(line, perpetualRaise, 0 as i32);
            (*line).special = 0 as i16;
        }
        54 => {
            EV_StopPlat(line);
            (*line).special = 0 as i16;
        }
        56 => {
            EV_DoFloor(line, raiseFloorCrush);
            (*line).special = 0 as i16;
        }
        57 => {
            EV_CeilingCrushStop(line);
            (*line).special = 0 as i16;
        }
        58 => {
            EV_DoFloor(line, raiseFloor24);
            (*line).special = 0 as i16;
        }
        59 => {
            EV_DoFloor(line, raiseFloor24AndChange);
            (*line).special = 0 as i16;
        }
        104 => {
            EV_TurnTagLightsOff(line);
            (*line).special = 0 as i16;
        }
        108 => {
            EV_DoDoor(line, vld_blazeRaise);
            (*line).special = 0 as i16;
        }
        109 => {
            EV_DoDoor(line, vld_blazeOpen);
            (*line).special = 0 as i16;
        }
        100 => {
            EV_BuildStairs(line, turbo16);
            (*line).special = 0 as i16;
        }
        110 => {
            EV_DoDoor(line, vld_blazeClose);
            (*line).special = 0 as i16;
        }
        119 => {
            EV_DoFloor(line, raiseFloorToNearest);
            (*line).special = 0 as i16;
        }
        121 => {
            EV_DoPlat(line, blazeDWUS, 0 as i32);
            (*line).special = 0 as i16;
        }
        124 => {
            G_SecretExitLevel();
        }
        125 => {
            if (*thing).player.is_null() {
                EV_Teleport(line, side, thing);
                (*line).special = 0 as i16;
            }
        }
        130 => {
            EV_DoFloor(line, raiseFloorTurbo);
            (*line).special = 0 as i16;
        }
        141 => {
            EV_DoCeiling(line, silentCrushAndRaise);
            (*line).special = 0 as i16;
        }
        72 => {
            EV_DoCeiling(line, lowerAndCrush);
        }
        73 => {
            EV_DoCeiling(line, crushAndRaise);
        }
        74 => {
            EV_CeilingCrushStop(line);
        }
        75 => {
            EV_DoDoor(line, vld_close);
        }
        76 => {
            EV_DoDoor(line, vld_close30ThenOpen);
        }
        77 => {
            EV_DoCeiling(line, fastCrushAndRaise);
        }
        79 => {
            EV_LightTurnOn(line, 35 as i32);
        }
        80 => {
            EV_LightTurnOn(line, 0 as i32);
        }
        81 => {
            EV_LightTurnOn(line, 255 as i32);
        }
        82 => {
            EV_DoFloor(line, lowerFloorToLowest);
        }
        83 => {
            EV_DoFloor(line, lowerFloor);
        }
        84 => {
            EV_DoFloor(line, lowerAndChange);
        }
        86 => {
            EV_DoDoor(line, vld_open);
        }
        87 => {
            EV_DoPlat(line, perpetualRaise, 0 as i32);
        }
        88 => {
            EV_DoPlat(line, downWaitUpStay, 0 as i32);
        }
        89 => {
            EV_StopPlat(line);
        }
        90 => {
            EV_DoDoor(line, vld_normal);
        }
        91 => {
            EV_DoFloor(line, raiseFloor);
        }
        92 => {
            EV_DoFloor(line, raiseFloor24);
        }
        93 => {
            EV_DoFloor(line, raiseFloor24AndChange);
        }
        94 => {
            EV_DoFloor(line, raiseFloorCrush);
        }
        95 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0 as i32);
        }
        96 => {
            EV_DoFloor(line, raiseToTexture);
        }
        97 => {
            EV_Teleport(line, side, thing);
        }
        98 => {
            EV_DoFloor(line, turboLower);
        }
        105 => {
            EV_DoDoor(line, vld_blazeRaise);
        }
        106 => {
            EV_DoDoor(line, vld_blazeOpen);
        }
        107 => {
            EV_DoDoor(line, vld_blazeClose);
        }
        120 => {
            EV_DoPlat(line, blazeDWUS, 0 as i32);
        }
        126 => {
            if (*thing).player.is_null() {
                EV_Teleport(line, side, thing);
            }
        }
        128 => {
            EV_DoFloor(line, raiseFloorToNearest);
        }
        129 => {
            EV_DoFloor(line, raiseFloorTurbo);
        }
        _ => {}
    };
}
pub unsafe fn P_ShootSpecialLine(
    mut thing: *mut mobj_t,
    mut line: *mut line_t,
) {
    let mut ok: i32 = 0;
    if (*thing).player.is_null() {
        ok = 0 as i32;
        match (*line).special as i32 {
            46 => {
                ok = 1 as i32;
            }
            _ => {}
        }
        if ok == 0 {
            return;
        }
    }
    match (*line).special as i32 {
        24 => {
            EV_DoFloor(line, raiseFloor);
            P_ChangeSwitchTexture(line, 0 as i32);
        }
        46 => {
            EV_DoDoor(line, vld_open);
            P_ChangeSwitchTexture(line, 1 as i32);
        }
        47 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0 as i32);
            P_ChangeSwitchTexture(line, 0 as i32);
        }
        _ => {}
    };
}
pub unsafe fn P_PlayerInSpecialSector(mut player: *mut player_t) {
    let mut sector: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    sector = (*(*(*player).mo).subsector).sector;
    if (*(*player).mo).z != (*sector).floorheight {
        return;
    }
    match (*sector).special as i32 {
        5 => {
            if (*player).powers[pw_ironfeet as i32 as usize] == 0 {
                if leveltime & 0x1f as i32 == 0 {
                    P_DamageMobj(
                        (*player).mo,
                        ::core::ptr::null_mut::<mobj_t>(),
                        ::core::ptr::null_mut::<mobj_t>(),
                        10 as i32,
                    );
                }
            }
        }
        7 => {
            if (*player).powers[pw_ironfeet as i32 as usize] == 0 {
                if leveltime & 0x1f as i32 == 0 {
                    P_DamageMobj(
                        (*player).mo,
                        ::core::ptr::null_mut::<mobj_t>(),
                        ::core::ptr::null_mut::<mobj_t>(),
                        5 as i32,
                    );
                }
            }
        }
        16 | 4 => {
            if (*player).powers[pw_ironfeet as i32 as usize] == 0
                || P_Random() < 5 as i32
            {
                if leveltime & 0x1f as i32 == 0 {
                    P_DamageMobj(
                        (*player).mo,
                        ::core::ptr::null_mut::<mobj_t>(),
                        ::core::ptr::null_mut::<mobj_t>(),
                        20 as i32,
                    );
                }
            }
        }
        9 => {
            (*player).secretcount += 1;
            (*sector).special = 0 as i16;
        }
        11 => {
            (*player).cheats &= !(CF_GODMODE as i32);
            if leveltime & 0x1f as i32 == 0 {
                P_DamageMobj(
                    (*player).mo,
                    ::core::ptr::null_mut::<mobj_t>(),
                    ::core::ptr::null_mut::<mobj_t>(),
                    20 as i32,
                );
            }
            if (*player).health <= 10 as i32 {
                G_ExitLevel();
            }
        }
        _ => {
            I_Error(&format!(
                "P_PlayerInSpecialSector: unknown special {}",
                (*sector).special as i32,
            ));
        }
    };
}
#[no_mangle]
pub static mut levelTimer: bool = false;
#[no_mangle]
pub static mut levelTimeCount: i32 = 0;
pub unsafe fn P_UpdateSpecials() {
    let mut anim: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    let mut pic: i32 = 0;
    let mut i: i32 = 0;
    let mut line: *mut line_t = ::core::ptr::null_mut::<line_t>();
    if levelTimer {
        levelTimeCount -= 1;
        if levelTimeCount == 0 {
            G_ExitLevel();
        }
    }
    anim = &raw mut anims as *mut anim_t;
    while anim < lastanim {
        i = (*anim).basepic;
        while i < (*anim).basepic + (*anim).numpics {
            pic = (*anim).basepic + (leveltime / (*anim).speed + i) % (*anim).numpics;
            if (*anim).istexture != 0 {
                *texturetranslation.offset(i as isize) = pic;
            } else {
                *flattranslation.offset(i as isize) = pic;
            }
            i += 1;
        }
        anim = anim.offset(1);
    }
    i = 0 as i32;
    while i < numlinespecials as i32 {
        line = linespeciallist[i as usize];
        match (*line).special as i32 {
            48 => {
                let ref mut fresh0 = (*sides
                    .offset((*line).sidenum[0 as i32 as usize] as isize))
                    .textureoffset;
                *fresh0 += FRACUNIT;
            }
            _ => {}
        }
        i += 1;
    }
    i = 0 as i32;
    while i < MAXBUTTONS {
        if buttonlist[i as usize].btimer != 0 {
            buttonlist[i as usize].btimer -= 1;
            if buttonlist[i as usize].btimer == 0 {
                match buttonlist[i as usize].where_0 as u32 {
                    0 => {
                        (*sides
                            .offset(
                                (*buttonlist[i as usize].line)
                                    .sidenum[0 as i32 as usize] as isize,
                            ))
                            .toptexture = buttonlist[i as usize].btexture
                            as i16;
                    }
                    1 => {
                        (*sides
                            .offset(
                                (*buttonlist[i as usize].line)
                                    .sidenum[0 as i32 as usize] as isize,
                            ))
                            .midtexture = buttonlist[i as usize].btexture
                            as i16;
                    }
                    2 => {
                        (*sides
                            .offset(
                                (*buttonlist[i as usize].line)
                                    .sidenum[0 as i32 as usize] as isize,
                            ))
                            .bottomtexture = buttonlist[i as usize].btexture
                            as i16;
                    }
                    _ => {}
                }
                S_StartSound(
                    &raw mut (*(&raw mut buttonlist as *mut button_t).offset(i as isize))
                        .soundorg as *mut ::core::ffi::c_void,
                    sfx_swtchn as i32,
                );
                memset(
                    (&raw mut buttonlist as *mut button_t).offset(i as isize)
                        as *mut button_t as *mut ::core::ffi::c_void,
                    0 as i32,
                    ::core::mem::size_of::<button_t>() as size_t,
                );
            }
        }
        i += 1;
    }
}
pub const DONUT_FLOORHEIGHT_DEFAULT: i32 = 0 as i32;
pub const DONUT_FLOORPIC_DEFAULT: i32 = 0x16 as i32;
unsafe extern "C" fn DonutOverrun(
    mut s3_floorheight: *mut fixed_t,
    mut s3_floorpic: *mut i16,
    mut line: *mut line_t,
    mut pillar_sector: *mut sector_t,
) {
    static mut first: i32 = 1 as i32;
    static mut tmp_s3_floorheight: i32 = 0;
    static mut tmp_s3_floorpic: i32 = 0;
    if first != 0 {
        let mut p: i32 = 0;
        first = 0 as i32;
        tmp_s3_floorheight = DONUT_FLOORHEIGHT_DEFAULT;
        tmp_s3_floorpic = DONUT_FLOORPIC_DEFAULT;
        p = M_CheckParmWithArgs("-donut", 2 as i32);
        if p > 0 as i32 {
            M_StrToInt(
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
                &raw mut tmp_s3_floorheight,
            );
            M_StrToInt(
                myargv[(p + 2 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
                &raw mut tmp_s3_floorpic,
            );
            if tmp_s3_floorpic >= numflats {
                fprintf(
                    stderr,
                    b"DonutOverrun: The second parameter for \"-donut\" switch should be greater than 0 and less than number of flats (%d). Using default value (%d) instead. \n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    numflats,
                    DONUT_FLOORPIC_DEFAULT,
                );
                tmp_s3_floorpic = DONUT_FLOORPIC_DEFAULT;
            }
        }
    }
    *s3_floorheight = tmp_s3_floorheight;
    *s3_floorpic = tmp_s3_floorpic as i16;
}
pub unsafe fn EV_DoDonut(mut line: *mut line_t) -> i32 {
    let mut s1: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut s2: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut s3: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut secnum: i32 = 0;
    let mut rtn: i32 = 0;
    let mut i: i32 = 0;
    let mut floor: *mut floormove_t = ::core::ptr::null_mut::<floormove_t>();
    let mut s3_floorheight: fixed_t = 0;
    let mut s3_floorpic: i16 = 0;
    secnum = -(1 as i32);
    rtn = 0 as i32;
    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if !(secnum >= 0 as i32) {
            break;
        }
        s1 = sectors.offset(secnum as isize) as *mut sector_t;
        if !(*s1).specialdata.is_null() {
            continue;
        }
        rtn = 1 as i32;
        s2 = getNextSector(
            *(*s1).lines.offset(0 as i32 as isize) as *mut line_t,
            s1,
        );
        if s2.is_null() {
            fprintf(
                stderr,
                b"EV_DoDonut: linedef had no second sidedef! Unexpected behavior may occur in Vanilla Doom. \n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            break;
        } else {
            i = 0 as i32;
            while i < (*s2).linecount {
                s3 = (**(*s2).lines.offset(i as isize)).backsector;
                if s3 == s1 {
                    i += 1;
                } else {
                    if s3.is_null() {
                        fprintf(
                            stderr,
                            b"EV_DoDonut: WARNING: emulating buffer overrun due to NULL back sector. Unexpected behavior may occur in Vanilla Doom.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        DonutOverrun(
                            &raw mut s3_floorheight,
                            &raw mut s3_floorpic,
                            line,
                            s1,
                        );
                    } else {
                        s3_floorheight = (*s3).floorheight;
                        s3_floorpic = (*s3).floorpic;
                    }
                    floor = Z_Malloc(
                        ::core::mem::size_of::<floormove_t>() as i32,
                        PU_LEVSPEC as i32,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ) as *mut floormove_t;
                    P_AddThinker(&raw mut (*floor).thinker);
                    (*s2).specialdata = floor as *mut ::core::ffi::c_void;
                    (*floor).thinker.function = ThinkerFn::Floor(T_MoveFloor);
                    (*floor).type_0 = donutRaise;
                    (*floor).crush = false;
                    (*floor).direction = 1 as i32;
                    (*floor).sector = s2;
                    (*floor).speed = (FLOORSPEED / 2 as i32) as fixed_t;
                    (*floor).texture = s3_floorpic;
                    (*floor).newspecial = 0 as i32;
                    (*floor).floordestheight = s3_floorheight;
                    floor = Z_Malloc(
                        ::core::mem::size_of::<floormove_t>() as i32,
                        PU_LEVSPEC as i32,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ) as *mut floormove_t;
                    P_AddThinker(&raw mut (*floor).thinker);
                    (*s1).specialdata = floor as *mut ::core::ffi::c_void;
                    (*floor).thinker.function = ThinkerFn::Floor(T_MoveFloor);
                    (*floor).type_0 = lowerFloor;
                    (*floor).crush = false;
                    (*floor).direction = -(1 as i32);
                    (*floor).sector = s1;
                    (*floor).speed = (FLOORSPEED / 2 as i32) as fixed_t;
                    (*floor).floordestheight = s3_floorheight;
                    break;
                }
            }
        }
    }
    return rtn;
}
#[no_mangle]
pub static mut numlinespecials: i16 = 0;
#[no_mangle]
pub static mut linespeciallist: [*mut line_t; 64] = [::core::ptr::null::<line_t>()
    as *mut line_t; 64];
pub unsafe fn P_SpawnSpecials() {
    let mut sector: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut i: i32 = 0;
    if timelimit > 0 as i32 && deathmatch != 0 {
        levelTimer = true;
        levelTimeCount = timelimit * 60 as i32 * TICRATE;
    } else {
        levelTimer = false;
    }
    sector = sectors;
    i = 0 as i32;
    while i < numsectors {
        if !((*sector).special == 0) {
            match (*sector).special as i32 {
                1 => {
                    P_SpawnLightFlash(sector);
                }
                2 => {
                    P_SpawnStrobeFlash(sector, FASTDARK, 0 as i32);
                }
                3 => {
                    P_SpawnStrobeFlash(sector, SLOWDARK, 0 as i32);
                }
                4 => {
                    P_SpawnStrobeFlash(sector, FASTDARK, 0 as i32);
                    (*sector).special = 4 as i16;
                }
                8 => {
                    P_SpawnGlowingLight(sector);
                }
                9 => {
                    totalsecret += 1;
                }
                10 => {
                    P_SpawnDoorCloseIn30(sector);
                }
                12 => {
                    P_SpawnStrobeFlash(sector, SLOWDARK, 1 as i32);
                }
                13 => {
                    P_SpawnStrobeFlash(sector, FASTDARK, 1 as i32);
                }
                14 => {
                    P_SpawnDoorRaiseIn5Mins(sector, i);
                }
                17 => {
                    P_SpawnFireFlicker(sector);
                }
                _ => {}
            }
        }
        i += 1;
        sector = sector.offset(1);
    }
    numlinespecials = 0 as i16;
    i = 0 as i32;
    while i < numlines {
        match (*lines.offset(i as isize)).special as i32 {
            48 => {
                if numlinespecials as i32 >= MAXLINEANIMS {
                    I_Error("Too many scrolling wall linedefs! (Vanilla limit is 64)");
                }
                linespeciallist[numlinespecials as usize] = lines.offset(i as isize)
                    as *mut line_t;
                numlinespecials += 1;
            }
            _ => {}
        }
        i += 1;
    }
    i = 0 as i32;
    while i < MAXCEILINGS {
        activeceilings[i as usize] = ::core::ptr::null_mut::<ceiling_t>();
        i += 1;
    }
    i = 0 as i32;
    while i < MAXPLATS {
        activeplats[i as usize] = ::core::ptr::null_mut::<plat_t>();
        i += 1;
    }
    i = 0 as i32;
    while i < MAXBUTTONS {
        memset(
            (&raw mut buttonlist as *mut button_t).offset(i as isize) as *mut button_t
                as *mut ::core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<button_t>() as size_t,
        );
        i += 1;
    }
}
pub const __INT_MAX__: i32 = 2147483647 as i32;
