use crate::src::p_mobj::{thinker_t, sector_t, line_t};
use crate::src::p_mobj::{mobj_t};
use crate::src::p_map::P_TeleportMove;
use crate::src::p_tick::thinkercap;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::p_setup::numsectors;
use crate::src::doomstat::gameversion;
use crate::src::p_setup::sectors;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::s_sound::S_StartSound;
use crate::src::p_mobj::MF_MISSILE;
use crate::src::sounds::sfx_telept;
use crate::src::p_mobj::{MT_TELEPORTMAN, MT_TFOG, mobjtype_t};
use crate::src::p_mobj::ThinkerFn;
use crate::src::d_mode::exe_final;
use crate::src::m_fixed::fixed_t;
pub const NUMMOBJTYPES: mobjtype_t = 137;
pub const ANGLETOFINESHIFT: i32 = 19 as i32;
pub unsafe fn EV_Teleport(
    mut line: *mut line_t,
    mut side: i32,
    mut thing: *mut mobj_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut tag: i32 = 0;
    let mut m: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut fog: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: u32 = 0;
    let mut thinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut sector: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut oldx: fixed_t = 0;
    let mut oldy: fixed_t = 0;
    let mut oldz: fixed_t = 0;
    if (*thing).flags & MF_MISSILE as i32 != 0 {
        return 0 as i32;
    }
    if side == 1 as i32 {
        return 0 as i32;
    }
    tag = (*line).tag as i32;
    i = 0 as i32;
    while i < numsectors {
        if (*sectors.offset(i as isize)).tag as i32 == tag {
            thinker = thinkercap.next as *mut thinker_t;
            thinker = thinkercap.next as *mut thinker_t;
            while thinker != &raw mut thinkercap {
                if matches!((*thinker).function, ThinkerFn::Mobj(_))
                {
                    m = thinker as *mut mobj_t;
                    if !((*m).type_0 as u32
                        != MT_TELEPORTMAN as i32 as u32)
                    {
                        sector = (*(*m).subsector).sector;
                        if !(sector.offset_from(sectors) as i64
                            != i as i64)
                        {
                            oldx = (*thing).x;
                            oldy = (*thing).y;
                            oldz = (*thing).z;
                            if !P_TeleportMove(thing, (*m).x, (*m).y) {
                                return 0 as i32;
                            }
                            if gameversion as u32
                                != exe_final as i32 as u32
                            {
                                (*thing).z = (*thing).floorz;
                            }
                            if !(*thing).player.is_null() {
                                (*(*thing).player).viewz = (*thing).z
                                    + (*(*thing).player).viewheight;
                            }
                            fog = P_SpawnMobj(oldx, oldy, oldz, MT_TFOG);
                            S_StartSound(
                                fog as *mut ::core::ffi::c_void,
                                sfx_telept as i32,
                            );
                            an = ((*m).angle >> ANGLETOFINESHIFT) as u32;
                            fog = P_SpawnMobj(
                                (*m).x + 20 as fixed_t * finecosine[an as isize],
                                (*m).y + 20 as fixed_t * finesine[an as usize],
                                (*thing).z,
                                MT_TFOG,
                            );
                            S_StartSound(
                                fog as *mut ::core::ffi::c_void,
                                sfx_telept as i32,
                            );
                            if !(*thing).player.is_null() {
                                (*thing).reactiontime = 18 as i32;
                            }
                            (*thing).angle = (*m).angle;
                            (*thing).momz = 0 as i32 as fixed_t;
                            (*thing).momy = (*thing).momz;
                            (*thing).momx = (*thing).momy;
                            return 1 as i32;
                        }
                    }
                }
                thinker = (*thinker).next as *mut thinker_t;
            }
        }
        i += 1;
    }
    return 0 as i32;
}
