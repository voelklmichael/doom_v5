use crate::src::p_spec::{plat_t};
use crate::src::p_mobj::{thinker_t, sector_t, line_t};
use crate::src::d_player::{player_t};
use crate::src::p_mobj::{mobj_t};
use crate::src::p_spec::P_FindLowestCeilingSurrounding;
use crate::src::p_floor::T_MovePlane;
use crate::src::p_spec::P_FindSectorFromLineTag;
use crate::src::p_tick::P_RemoveThinker;
use crate::src::p_setup::sides;
use crate::src::p_tick::P_AddThinker;
use crate::src::p_setup::sectors;
use crate::src::s_sound::S_StartSound;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVSPEC;
use crate::src::sounds::{sfx_bdcls, sfx_bdopn, sfx_dorcls, sfx_doropn, sfx_oof};
use crate::src::i_system::{fprintf, stderr};
use crate::src::p_mobj::ThinkerFn;
use crate::src::p_floor::{crushed, ok, pastdest, result_e};
use crate::src::m_fixed::fixed_t;
use crate::src::p_inter::{it_bluecard, it_blueskull, it_redcard, it_redskull, it_yellowcard, it_yellowskull};
use crate::src::doomdef::NULL;
use crate::src::doomdef::TICRATE;
use crate::src::m_fixed::FRACUNIT;
pub type vldoor_e = u32;
pub const vld_blazeClose: vldoor_e = 7;
pub const vld_blazeOpen: vldoor_e = 6;
pub const vld_blazeRaise: vldoor_e = 5;
pub const vld_raiseIn5Mins: vldoor_e = 4;
pub const vld_open: vldoor_e = 3;
pub const vld_close: vldoor_e = 2;
pub const vld_close30ThenOpen: vldoor_e = 1;
pub const vld_normal: vldoor_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vldoor_t {
    pub thinker: thinker_t,
    pub type_0: vldoor_e,
    pub sector: *mut sector_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub direction: i32,
    pub topwait: i32,
    pub topcountdown: i32,
}
pub const VDOORWAIT: i32 = 150 as i32;
pub unsafe fn T_VerticalDoor(mut door: *mut vldoor_t) {
    let mut res: result_e = ok;
    match (*door).direction {
        0 => {
            (*door).topcountdown -= 1;
            if (*door).topcountdown == 0 {
                match (*door).type_0 as u32 {
                    5 => {
                        (*door).direction = -(1 as i32);
                        S_StartSound(
                            &raw mut (*(*door).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_bdcls as i32,
                        );
                    }
                    0 => {
                        (*door).direction = -(1 as i32);
                        S_StartSound(
                            &raw mut (*(*door).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_dorcls as i32,
                        );
                    }
                    1 => {
                        (*door).direction = 1 as i32;
                        S_StartSound(
                            &raw mut (*(*door).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_doropn as i32,
                        );
                    }
                    _ => {}
                }
            }
        }
        2 => {
            (*door).topcountdown -= 1;
            if (*door).topcountdown == 0 {
                match (*door).type_0 as u32 {
                    4 => {
                        (*door).direction = 1 as i32;
                        (*door).type_0 = vld_normal;
                        S_StartSound(
                            &raw mut (*(*door).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_doropn as i32,
                        );
                    }
                    _ => {}
                }
            }
        }
        -1 => {
            res = T_MovePlane(
                (*door).sector,
                (*door).speed,
                (*(*door).sector).floorheight,
                false,
                1 as i32,
                (*door).direction,
            );
            if res as u32
                == pastdest as i32 as u32
            {
                match (*door).type_0 as u32 {
                    5 | 7 => {
                        (*(*door).sector).specialdata = NULL;
                        P_RemoveThinker(&raw mut (*door).thinker);
                        S_StartSound(
                            &raw mut (*(*door).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_bdcls as i32,
                        );
                    }
                    0 | 2 => {
                        (*(*door).sector).specialdata = NULL;
                        P_RemoveThinker(&raw mut (*door).thinker);
                    }
                    1 => {
                        (*door).direction = 0 as i32;
                        (*door).topcountdown = TICRATE * 30 as i32;
                    }
                    _ => {}
                }
            } else if res as u32
                == crushed as i32 as u32
            {
                match (*door).type_0 as u32 {
                    7 | 2 => {}
                    _ => {
                        (*door).direction = 1 as i32;
                        S_StartSound(
                            &raw mut (*(*door).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_doropn as i32,
                        );
                    }
                }
            }
        }
        1 => {
            res = T_MovePlane(
                (*door).sector,
                (*door).speed,
                (*door).topheight,
                false,
                1 as i32,
                (*door).direction,
            );
            if res as u32
                == pastdest as i32 as u32
            {
                match (*door).type_0 as u32 {
                    5 | 0 => {
                        (*door).direction = 0 as i32;
                        (*door).topcountdown = (*door).topwait;
                    }
                    1 | 6 | 3 => {
                        (*(*door).sector).specialdata = NULL;
                        P_RemoveThinker(&raw mut (*door).thinker);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };
}
pub unsafe fn EV_DoLockedDoor(
    mut line: *mut line_t,
    mut type_0: vldoor_e,
    mut thing: *mut mobj_t,
) -> i32 {
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    p = (*thing).player as *mut player_t;
    if p.is_null() {
        return 0 as i32;
    }
    match (*line).special as i32 {
        99 | 133 => {
            if p.is_null() {
                return 0 as i32;
            }
            if !(*p).cards[it_bluecard as i32 as usize]
                && !(*p).cards[it_blueskull as i32 as usize]
            {
                (*p).message = b"You need a blue key to activate this object\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartSound(NULL, sfx_oof as i32);
                return 0 as i32;
            }
        }
        134 | 135 => {
            if p.is_null() {
                return 0 as i32;
            }
            if !(*p).cards[it_redcard as i32 as usize]
                && !(*p).cards[it_redskull as i32 as usize]
            {
                (*p).message = b"You need a red key to activate this object\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartSound(NULL, sfx_oof as i32);
                return 0 as i32;
            }
        }
        136 | 137 => {
            if p.is_null() {
                return 0 as i32;
            }
            if !(*p).cards[it_yellowcard as i32 as usize]
                && !(*p).cards[it_yellowskull as i32 as usize]
            {
                (*p).message = b"You need a yellow key to activate this object\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartSound(NULL, sfx_oof as i32);
                return 0 as i32;
            }
        }
        _ => {}
    }
    return EV_DoDoor(line, type_0);
}
pub unsafe fn EV_DoDoor(
    mut line: *mut line_t,
    mut type_0: vldoor_e,
) -> i32 {
    let mut secnum: i32 = 0;
    let mut rtn: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut door: *mut vldoor_t = ::core::ptr::null_mut::<vldoor_t>();
    secnum = -(1 as i32);
    rtn = 0 as i32;
    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if !(secnum >= 0 as i32) {
            break;
        }
        sec = sectors.offset(secnum as isize) as *mut sector_t;
        if !(*sec).specialdata.is_null() {
            continue;
        }
        rtn = 1 as i32;
        door = Z_Malloc(
            ::core::mem::size_of::<vldoor_t>() as i32,
            PU_LEVSPEC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut vldoor_t;
        P_AddThinker(&raw mut (*door).thinker);
        (*sec).specialdata = door as *mut ::core::ffi::c_void;
        (*door).thinker.function = ThinkerFn::Door(T_VerticalDoor);
        (*door).sector = sec;
        (*door).type_0 = type_0;
        (*door).topwait = VDOORWAIT;
        (*door).speed = (FRACUNIT * 2 as i32) as fixed_t;
        match type_0 as u32 {
            7 => {
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 as i32 * FRACUNIT;
                (*door).direction = -(1 as i32);
                (*door).speed = (FRACUNIT * 2 as i32
                    * 4 as i32) as fixed_t;
                S_StartSound(
                    &raw mut (*(*door).sector).soundorg as *mut ::core::ffi::c_void,
                    sfx_bdcls as i32,
                );
            }
            2 => {
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 as i32 * FRACUNIT;
                (*door).direction = -(1 as i32);
                S_StartSound(
                    &raw mut (*(*door).sector).soundorg as *mut ::core::ffi::c_void,
                    sfx_dorcls as i32,
                );
            }
            1 => {
                (*door).topheight = (*sec).ceilingheight;
                (*door).direction = -(1 as i32);
                S_StartSound(
                    &raw mut (*(*door).sector).soundorg as *mut ::core::ffi::c_void,
                    sfx_dorcls as i32,
                );
            }
            5 | 6 => {
                (*door).direction = 1 as i32;
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 as i32 * FRACUNIT;
                (*door).speed = (FRACUNIT * 2 as i32
                    * 4 as i32) as fixed_t;
                if (*door).topheight != (*sec).ceilingheight {
                    S_StartSound(
                        &raw mut (*(*door).sector).soundorg as *mut ::core::ffi::c_void,
                        sfx_bdopn as i32,
                    );
                }
            }
            0 | 3 => {
                (*door).direction = 1 as i32;
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 as i32 * FRACUNIT;
                if (*door).topheight != (*sec).ceilingheight {
                    S_StartSound(
                        &raw mut (*(*door).sector).soundorg as *mut ::core::ffi::c_void,
                        sfx_doropn as i32,
                    );
                }
            }
            _ => {}
        }
    }
    return rtn;
}
pub unsafe fn EV_VerticalDoor(mut line: *mut line_t, mut thing: *mut mobj_t) {
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut door: *mut vldoor_t = ::core::ptr::null_mut::<vldoor_t>();
    let mut side: i32 = 0;
    side = 0 as i32;
    player = (*thing).player as *mut player_t;
    match (*line).special as i32 {
        26 | 32 => {
            if player.is_null() {
                return;
            }
            if !(*player).cards[it_bluecard as i32 as usize]
                && !(*player).cards[it_blueskull as i32 as usize]
            {
                (*player).message = b"You need a blue key to open this door\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartSound(NULL, sfx_oof as i32);
                return;
            }
        }
        27 | 34 => {
            if player.is_null() {
                return;
            }
            if !(*player).cards[it_yellowcard as i32 as usize]
                && !(*player).cards[it_yellowskull as i32 as usize]
            {
                (*player).message = b"You need a yellow key to open this door\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartSound(NULL, sfx_oof as i32);
                return;
            }
        }
        28 | 33 => {
            if player.is_null() {
                return;
            }
            if !(*player).cards[it_redcard as i32 as usize]
                && !(*player).cards[it_redskull as i32 as usize]
            {
                (*player).message = b"You need a red key to open this door\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartSound(NULL, sfx_oof as i32);
                return;
            }
        }
        _ => {}
    }
    sec = (*sides
        .offset((*line).sidenum[(side ^ 1 as i32) as usize] as isize))
        .sector;
    if !(*sec).specialdata.is_null() {
        door = (*sec).specialdata as *mut vldoor_t;
        match (*line).special as i32 {
            1 | 26 | 27 | 28 | 117 => {
                if (*door).direction == -(1 as i32) {
                    (*door).direction = 1 as i32;
                } else {
                    if (*thing).player.is_null() {
                        return;
                    }
                    if matches!((*door).thinker.function, ThinkerFn::Door(_)) {
                        (*door).direction = -(1 as i32);
                    } else if matches!((*door).thinker.function, ThinkerFn::Plat(_)) {
                        let mut plat: *mut plat_t = ::core::ptr::null_mut::<plat_t>();
                        plat = door as *mut plat_t;
                        (*plat).wait = -(1 as i32);
                    } else {
                        fprintf(
                            stderr,
                            b"EV_VerticalDoor: Tried to close something that wasn't a door.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        (*door).direction = -(1 as i32);
                    }
                }
                return;
            }
            _ => {}
        }
    }
    match (*line).special as i32 {
        117 | 118 => {
            S_StartSound(
                &raw mut (*sec).soundorg as *mut ::core::ffi::c_void,
                sfx_bdopn as i32,
            );
        }
        1 | 31 => {
            S_StartSound(
                &raw mut (*sec).soundorg as *mut ::core::ffi::c_void,
                sfx_doropn as i32,
            );
        }
        _ => {
            S_StartSound(
                &raw mut (*sec).soundorg as *mut ::core::ffi::c_void,
                sfx_doropn as i32,
            );
        }
    }
    door = Z_Malloc(
        ::core::mem::size_of::<vldoor_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut vldoor_t;
    P_AddThinker(&raw mut (*door).thinker);
    (*sec).specialdata = door as *mut ::core::ffi::c_void;
    (*door).thinker.function = ThinkerFn::Door(T_VerticalDoor);
    (*door).sector = sec;
    (*door).direction = 1 as i32;
    (*door).speed = (FRACUNIT * 2 as i32) as fixed_t;
    (*door).topwait = VDOORWAIT;
    match (*line).special as i32 {
        1 | 26 | 27 | 28 => {
            (*door).type_0 = vld_normal;
        }
        31 | 32 | 33 | 34 => {
            (*door).type_0 = vld_open;
            (*line).special = 0 as i16;
        }
        117 => {
            (*door).type_0 = vld_blazeRaise;
            (*door).speed = (FRACUNIT * 2 as i32
                * 4 as i32) as fixed_t;
        }
        118 => {
            (*door).type_0 = vld_blazeOpen;
            (*line).special = 0 as i16;
            (*door).speed = (FRACUNIT * 2 as i32
                * 4 as i32) as fixed_t;
        }
        _ => {}
    }
    (*door).topheight = P_FindLowestCeilingSurrounding(sec);
    (*door).topheight -= 4 as i32 * FRACUNIT;
}
pub unsafe fn P_SpawnDoorCloseIn30(mut sec: *mut sector_t) {
    let mut door: *mut vldoor_t = ::core::ptr::null_mut::<vldoor_t>();
    door = Z_Malloc(
        ::core::mem::size_of::<vldoor_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut vldoor_t;
    P_AddThinker(&raw mut (*door).thinker);
    (*sec).specialdata = door as *mut ::core::ffi::c_void;
    (*sec).special = 0 as i16;
    (*door).thinker.function = ThinkerFn::Door(T_VerticalDoor);
    (*door).sector = sec;
    (*door).direction = 0 as i32;
    (*door).type_0 = vld_normal;
    (*door).speed = (FRACUNIT * 2 as i32) as fixed_t;
    (*door).topcountdown = 30 as i32 * TICRATE;
}
pub unsafe fn P_SpawnDoorRaiseIn5Mins(
    mut sec: *mut sector_t,
    mut secnum: i32,
) {
    let mut door: *mut vldoor_t = ::core::ptr::null_mut::<vldoor_t>();
    door = Z_Malloc(
        ::core::mem::size_of::<vldoor_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut vldoor_t;
    P_AddThinker(&raw mut (*door).thinker);
    (*sec).specialdata = door as *mut ::core::ffi::c_void;
    (*sec).special = 0 as i16;
    (*door).thinker.function = ThinkerFn::Door(T_VerticalDoor);
    (*door).sector = sec;
    (*door).direction = 2 as i32;
    (*door).type_0 = vld_raiseIn5Mins;
    (*door).speed = (FRACUNIT * 2 as i32) as fixed_t;
    (*door).topheight = P_FindLowestCeilingSurrounding(sec);
    (*door).topheight -= 4 as i32 * FRACUNIT;
    (*door).topwait = VDOORWAIT;
    (*door).topcountdown = 5 as i32 * 60 as i32 * TICRATE;
}
