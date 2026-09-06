use crate::src::p_spec::{ceiling_t};
use crate::src::p_mobj::{sector_t, line_t};
use crate::src::p_spec::P_FindHighestCeilingSurrounding;
use crate::src::p_floor::T_MovePlane;
use crate::src::p_spec::P_FindSectorFromLineTag;
use crate::src::p_tick::P_RemoveThinker;
use crate::src::p_tick::P_AddThinker;
use crate::src::p_setup::sectors;
use crate::src::p_tick::leveltime;
use crate::src::s_sound::S_StartSound;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVSPEC;
use crate::src::sounds::{sfx_pstop, sfx_stnmov};
use crate::src::p_mobj::mobjtype_t;
use crate::src::p_mobj::ThinkerFn;
use crate::src::p_floor::{crushed, ok, pastdest, result_e};
use crate::src::m_fixed::fixed_t;


pub const NUMMOBJTYPES: mobjtype_t = 137;
pub type ceiling_e = u32;
pub const silentCrushAndRaise: ceiling_e = 5;
pub const fastCrushAndRaise: ceiling_e = 4;
pub const crushAndRaise: ceiling_e = 3;
pub const lowerAndCrush: ceiling_e = 2;
pub const raiseToHighest: ceiling_e = 1;
pub const lowerToFloor: ceiling_e = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub const FRACUNIT: i32 = (1 as i32) << FRACBITS;
pub const CEILSPEED: i32 = FRACUNIT;
pub const MAXCEILINGS: i32 = 30 as i32;
pub static mut activeceilings: [*mut ceiling_t; 30] = [::core::ptr::null::<ceiling_t>()
    as *mut ceiling_t; 30];
pub unsafe fn T_MoveCeiling(mut ceiling: *mut ceiling_t) {
    let mut res: result_e = ok;
    match (*ceiling).direction {
        1 => {
            res = T_MovePlane(
                (*ceiling).sector,
                (*ceiling).speed,
                (*ceiling).topheight,
                false,
                1 as i32,
                (*ceiling).direction,
            );
            if leveltime & 7 as i32 == 0 {
                match (*ceiling).type_0 as u32 {
                    5 => {}
                    _ => {
                        S_StartSound(
                            &raw mut (*(*ceiling).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_stnmov as i32,
                        );
                    }
                }
            }
            if res as u32
                == pastdest as i32 as u32
            {
                let mut current_block_7: u64;
                match (*ceiling).type_0 as u32 {
                    1 => {
                        P_RemoveActiveCeiling(ceiling);
                        current_block_7 = 10599921512955367680;
                    }
                    5 => {
                        S_StartSound(
                            &raw mut (*(*ceiling).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_pstop as i32,
                        );
                        current_block_7 = 16040908003852494439;
                    }
                    4 | 3 => {
                        current_block_7 = 16040908003852494439;
                    }
                    _ => {
                        current_block_7 = 10599921512955367680;
                    }
                }
                match current_block_7 {
                    16040908003852494439 => {
                        (*ceiling).direction = -(1 as i32);
                    }
                    _ => {}
                }
            }
        }
        -1 => {
            res = T_MovePlane(
                (*ceiling).sector,
                (*ceiling).speed,
                (*ceiling).bottomheight,
                (*ceiling).crush,
                1 as i32,
                (*ceiling).direction,
            );
            if leveltime & 7 as i32 == 0 {
                match (*ceiling).type_0 as u32 {
                    5 => {}
                    _ => {
                        S_StartSound(
                            &raw mut (*(*ceiling).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_stnmov as i32,
                        );
                    }
                }
            }
            if res as u32
                == pastdest as i32 as u32
            {
                let mut current_block_19: u64;
                match (*ceiling).type_0 as u32 {
                    5 => {
                        S_StartSound(
                            &raw mut (*(*ceiling).sector).soundorg
                                as *mut ::core::ffi::c_void,
                            sfx_pstop as i32,
                        );
                        current_block_19 = 3850642056257311267;
                    }
                    3 => {
                        current_block_19 = 3850642056257311267;
                    }
                    4 => {
                        current_block_19 = 14600216857840559743;
                    }
                    2 | 0 => {
                        P_RemoveActiveCeiling(ceiling);
                        current_block_19 = 16924917904204750491;
                    }
                    _ => {
                        current_block_19 = 16924917904204750491;
                    }
                }
                match current_block_19 {
                    3850642056257311267 => {
                        (*ceiling).speed = CEILSPEED as fixed_t;
                        current_block_19 = 14600216857840559743;
                    }
                    _ => {}
                }
                match current_block_19 {
                    14600216857840559743 => {
                        (*ceiling).direction = 1 as i32;
                    }
                    _ => {}
                }
            } else if res as u32
                == crushed as i32 as u32
            {
                match (*ceiling).type_0 as u32 {
                    5 | 3 | 2 => {
                        (*ceiling).speed = (CEILSPEED / 8 as i32)
                            as fixed_t;
                    }
                    _ => {}
                }
            }
        }
        0 | _ => {}
    };
}
pub unsafe fn EV_DoCeiling(
    mut line: *mut line_t,
    mut type_0: ceiling_e,
) -> i32 {
    let mut secnum: i32 = 0;
    let mut rtn: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut ceiling: *mut ceiling_t = ::core::ptr::null_mut::<ceiling_t>();
    secnum = -(1 as i32);
    rtn = 0 as i32;
    match type_0 as u32 {
        4 | 5 | 3 => {
            P_ActivateInStasisCeiling(line);
        }
        _ => {}
    }
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
        ceiling = Z_Malloc(
            ::core::mem::size_of::<ceiling_t>() as i32,
            PU_LEVSPEC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut ceiling_t;
        P_AddThinker(&raw mut (*ceiling).thinker);
        (*sec).specialdata = ceiling as *mut ::core::ffi::c_void;
        (*ceiling).thinker.function = ThinkerFn::Ceiling(T_MoveCeiling);
        (*ceiling).sector = sec;
        (*ceiling).crush = false;
        let mut current_block_26: u64;
        match type_0 as u32 {
            4 => {
                (*ceiling).crush = true;
                (*ceiling).topheight = (*sec).ceilingheight;
                (*ceiling).bottomheight = ((*sec).floorheight as i32
                    + 8 as i32 * FRACUNIT) as fixed_t;
                (*ceiling).direction = -(1 as i32);
                (*ceiling).speed = (CEILSPEED * 2 as i32) as fixed_t;
                current_block_26 = 7056779235015430508;
            }
            5 | 3 => {
                (*ceiling).crush = true;
                (*ceiling).topheight = (*sec).ceilingheight;
                current_block_26 = 6994972524166957283;
            }
            2 | 0 => {
                current_block_26 = 6994972524166957283;
            }
            1 => {
                (*ceiling).topheight = P_FindHighestCeilingSurrounding(sec);
                (*ceiling).direction = 1 as i32;
                (*ceiling).speed = CEILSPEED as fixed_t;
                current_block_26 = 7056779235015430508;
            }
            _ => {
                current_block_26 = 7056779235015430508;
            }
        }
        match current_block_26 {
            6994972524166957283 => {
                (*ceiling).bottomheight = (*sec).floorheight;
                if type_0 as u32
                    != lowerToFloor as i32 as u32
                {
                    (*ceiling).bottomheight += 8 as i32 * FRACUNIT;
                }
                (*ceiling).direction = -(1 as i32);
                (*ceiling).speed = CEILSPEED as fixed_t;
            }
            _ => {}
        }
        (*ceiling).tag = (*sec).tag as i32;
        (*ceiling).type_0 = type_0;
        P_AddActiveCeiling(ceiling);
    }
    return rtn;
}
pub unsafe fn P_AddActiveCeiling(mut c: *mut ceiling_t) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXCEILINGS {
        if activeceilings[i as usize].is_null() {
            activeceilings[i as usize] = c;
            return;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_RemoveActiveCeiling(mut c: *mut ceiling_t) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXCEILINGS {
        if activeceilings[i as usize] == c {
            (*(*activeceilings[i as usize]).sector).specialdata = NULL;
            P_RemoveThinker(
                &raw mut (**(&raw mut activeceilings as *mut *mut ceiling_t)
                    .offset(i as isize))
                    .thinker,
            );
            activeceilings[i as usize] = ::core::ptr::null_mut::<ceiling_t>();
            break;
        } else {
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_ActivateInStasisCeiling(mut line: *mut line_t) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXCEILINGS {
        if !activeceilings[i as usize].is_null()
            && (*activeceilings[i as usize]).tag == (*line).tag as i32
            && (*activeceilings[i as usize]).direction == 0 as i32
        {
            (*activeceilings[i as usize]).direction = (*activeceilings[i as usize])
                .olddirection;
            (*activeceilings[i as usize]).thinker.function = ThinkerFn::Ceiling(T_MoveCeiling);
        }
        i += 1;
    }
}
pub unsafe fn EV_CeilingCrushStop(
    mut line: *mut line_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut rtn: i32 = 0;
    rtn = 0 as i32;
    i = 0 as i32;
    while i < MAXCEILINGS {
        if !activeceilings[i as usize].is_null()
            && (*activeceilings[i as usize]).tag == (*line).tag as i32
            && (*activeceilings[i as usize]).direction != 0 as i32
        {
            (*activeceilings[i as usize]).olddirection = (*activeceilings[i as usize])
                .direction;
            (*activeceilings[i as usize]).thinker.function = ThinkerFn::Paused;
            (*activeceilings[i as usize]).direction = 0 as i32;
            rtn = 1 as i32;
        }
        i += 1;
    }
    return rtn;
}
