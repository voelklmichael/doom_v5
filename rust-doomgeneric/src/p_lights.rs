use crate::src::p_mobj::{thinker_t, sector_t, line_t};
use crate::src::p_spec::P_FindMinSurroundingLight;
use crate::src::p_spec::getNextSector;
use crate::src::p_spec::P_FindSectorFromLineTag;
use crate::src::p_setup::numsectors;
use crate::src::p_tick::P_AddThinker;
use crate::src::m_random::P_Random;
use crate::src::p_setup::sectors;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVSPEC;
use crate::src::p_mobj::ThinkerFn;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct fireflicker_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightflash_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
    pub maxtime: i32,
    pub mintime: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strobe_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub minlight: i32,
    pub maxlight: i32,
    pub darktime: i32,
    pub brighttime: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glow_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub minlight: i32,
    pub maxlight: i32,
    pub direction: i32,
}
pub const GLOWSPEED: i32 = 8;
pub const STROBEBRIGHT: i32 = 5;
pub const SLOWDARK: i32 = 35;
pub unsafe fn T_FireFlicker(mut flick: *mut fireflicker_t) {
    let mut amount: i32 = 0;
    (*flick).count -= 1;
    if (*flick).count != 0 {
        return;
    }
    amount = (P_Random() & 3 as i32) * 16 as i32;
    if (*(*flick).sector).lightlevel as i32 - amount < (*flick).minlight {
        (*(*flick).sector).lightlevel = (*flick).minlight as i16;
    } else {
        (*(*flick).sector).lightlevel = ((*flick).maxlight - amount)
            as i16;
    }
    (*flick).count = 4 as i32;
}
pub unsafe fn P_SpawnFireFlicker(mut sector: *mut sector_t) {
    let mut flick: *mut fireflicker_t = ::core::ptr::null_mut::<fireflicker_t>();
    (*sector).special = 0 as i16;
    flick = Z_Malloc(
        ::core::mem::size_of::<fireflicker_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut fireflicker_t;
    P_AddThinker(&raw mut (*flick).thinker);
    (*flick).thinker.function = ThinkerFn::FireFlicker(T_FireFlicker);
    (*flick).sector = sector;
    (*flick).maxlight = (*sector).lightlevel as i32;
    (*flick).minlight = P_FindMinSurroundingLight(
        sector,
        (*sector).lightlevel as i32,
    ) + 16 as i32;
    (*flick).count = 4 as i32;
}
pub unsafe fn T_LightFlash(mut flash: *mut lightflash_t) {
    (*flash).count -= 1;
    if (*flash).count != 0 {
        return;
    }
    if (*(*flash).sector).lightlevel as i32 == (*flash).maxlight {
        (*(*flash).sector).lightlevel = (*flash).minlight as i16;
        (*flash).count = (P_Random() & (*flash).mintime) + 1 as i32;
    } else {
        (*(*flash).sector).lightlevel = (*flash).maxlight as i16;
        (*flash).count = (P_Random() & (*flash).maxtime) + 1 as i32;
    };
}
pub unsafe fn P_SpawnLightFlash(mut sector: *mut sector_t) {
    let mut flash: *mut lightflash_t = ::core::ptr::null_mut::<lightflash_t>();
    (*sector).special = 0 as i16;
    flash = Z_Malloc(
        ::core::mem::size_of::<lightflash_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut lightflash_t;
    P_AddThinker(&raw mut (*flash).thinker);
    (*flash).thinker.function = ThinkerFn::LightFlash(T_LightFlash);
    (*flash).sector = sector;
    (*flash).maxlight = (*sector).lightlevel as i32;
    (*flash).minlight = P_FindMinSurroundingLight(
        sector,
        (*sector).lightlevel as i32,
    );
    (*flash).maxtime = 64 as i32;
    (*flash).mintime = 7 as i32;
    (*flash).count = (P_Random() & (*flash).maxtime) + 1 as i32;
}
pub unsafe fn T_StrobeFlash(mut flash: *mut strobe_t) {
    (*flash).count -= 1;
    if (*flash).count != 0 {
        return;
    }
    if (*(*flash).sector).lightlevel as i32 == (*flash).minlight {
        (*(*flash).sector).lightlevel = (*flash).maxlight as i16;
        (*flash).count = (*flash).brighttime;
    } else {
        (*(*flash).sector).lightlevel = (*flash).minlight as i16;
        (*flash).count = (*flash).darktime;
    };
}
pub unsafe fn P_SpawnStrobeFlash(
    mut sector: *mut sector_t,
    mut fastOrSlow: i32,
    mut inSync: i32,
) {
    let mut flash: *mut strobe_t = ::core::ptr::null_mut::<strobe_t>();
    flash = Z_Malloc(
        ::core::mem::size_of::<strobe_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut strobe_t;
    P_AddThinker(&raw mut (*flash).thinker);
    (*flash).sector = sector;
    (*flash).darktime = fastOrSlow;
    (*flash).brighttime = STROBEBRIGHT;
    (*flash).thinker.function = ThinkerFn::Strobe(T_StrobeFlash);
    (*flash).maxlight = (*sector).lightlevel as i32;
    (*flash).minlight = P_FindMinSurroundingLight(
        sector,
        (*sector).lightlevel as i32,
    );
    if (*flash).minlight == (*flash).maxlight {
        (*flash).minlight = 0 as i32;
    }
    (*sector).special = 0 as i16;
    if inSync == 0 {
        (*flash).count = (P_Random() & 7 as i32)
            + 1 as i32;
    } else {
        (*flash).count = 1 as i32;
    };
}
pub unsafe fn EV_StartLightStrobing(mut line: *mut line_t) {
    let mut secnum: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    secnum = -(1 as i32);
    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if !(secnum >= 0 as i32) {
            break;
        }
        sec = sectors.offset(secnum as isize) as *mut sector_t;
        if !(*sec).specialdata.is_null() {
            continue;
        }
        P_SpawnStrobeFlash(sec, SLOWDARK, 0 as i32);
    };
}
pub unsafe fn EV_TurnTagLightsOff(mut line: *mut line_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut min: i32 = 0;
    let mut sector: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut tsec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut templine: *mut line_t = ::core::ptr::null_mut::<line_t>();
    sector = sectors;
    j = 0 as i32;
    while j < numsectors {
        if (*sector).tag as i32 == (*line).tag as i32 {
            min = (*sector).lightlevel as i32;
            i = 0 as i32;
            while i < (*sector).linecount {
                templine = *(*sector).lines.offset(i as isize) as *mut line_t;
                tsec = getNextSector(templine, sector);
                if !tsec.is_null() {
                    if ((*tsec).lightlevel as i32) < min {
                        min = (*tsec).lightlevel as i32;
                    }
                }
                i += 1;
            }
            (*sector).lightlevel = min as i16;
        }
        j += 1;
        sector = sector.offset(1);
    }
}
pub unsafe fn EV_LightTurnOn(
    mut line: *mut line_t,
    mut bright: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sector: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut temp: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut templine: *mut line_t = ::core::ptr::null_mut::<line_t>();
    sector = sectors;
    i = 0 as i32;
    while i < numsectors {
        if (*sector).tag as i32 == (*line).tag as i32 {
            if bright == 0 {
                j = 0 as i32;
                while j < (*sector).linecount {
                    templine = *(*sector).lines.offset(j as isize) as *mut line_t;
                    temp = getNextSector(templine, sector);
                    if !temp.is_null() {
                        if (*temp).lightlevel as i32 > bright {
                            bright = (*temp).lightlevel as i32;
                        }
                    }
                    j += 1;
                }
            }
            (*sector).lightlevel = bright as i16;
        }
        i += 1;
        sector = sector.offset(1);
    }
}
pub unsafe fn T_Glow(mut g: *mut glow_t) {
    match (*g).direction {
        -1 => {
            (*(*g).sector).lightlevel = ((*(*g).sector).lightlevel as i32
                - GLOWSPEED) as i16;
            if (*(*g).sector).lightlevel as i32 <= (*g).minlight {
                (*(*g).sector).lightlevel = ((*(*g).sector).lightlevel
                    as i32 + GLOWSPEED) as i16;
                (*g).direction = 1 as i32;
            }
        }
        1 => {
            (*(*g).sector).lightlevel = ((*(*g).sector).lightlevel as i32
                + GLOWSPEED) as i16;
            if (*(*g).sector).lightlevel as i32 >= (*g).maxlight {
                (*(*g).sector).lightlevel = ((*(*g).sector).lightlevel
                    as i32 - GLOWSPEED) as i16;
                (*g).direction = -(1 as i32);
            }
        }
        _ => {}
    };
}
pub unsafe fn P_SpawnGlowingLight(mut sector: *mut sector_t) {
    let mut g: *mut glow_t = ::core::ptr::null_mut::<glow_t>();
    g = Z_Malloc(
        ::core::mem::size_of::<glow_t>() as i32,
        PU_LEVSPEC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut glow_t;
    P_AddThinker(&raw mut (*g).thinker);
    (*g).sector = sector;
    (*g).minlight = P_FindMinSurroundingLight(
        sector,
        (*sector).lightlevel as i32,
    );
    (*g).maxlight = (*sector).lightlevel as i32;
    (*g).thinker.function = ThinkerFn::Glow(T_Glow);
    (*g).direction = -(1 as i32);
    (*sector).special = 0 as i16;
}
