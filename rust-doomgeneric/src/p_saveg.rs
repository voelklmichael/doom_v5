use crate::src::i_system::FILE;
use crate::src::r_defs::{side_t};
use crate::src::p_spec::{plat_t, ceiling_t, floormove_t};
use crate::src::p_lights::{lightflash_t, strobe_t, glow_t};
use crate::src::p_doors::{vldoor_t};
use crate::src::p_mobj::{thinker_s, thinker_t, mapthing_t, state_t, mobjinfo_t, subsector_s, sector_t, line_t, ThinkerFn};
use crate::src::d_player::{player_s, player_t, playerstate_t};
use crate::src::p_mobj::{mobj_s, mobj_t, pspdef_t};
use crate::src::d_ticcmd::{ticcmd_t};
use crate::src::i_system::I_Error;
use crate::src::p_ceilng::P_AddActiveCeiling;
use crate::src::d_main::savegamedir;
use crate::src::g_game::G_VanillaVersionCode;
use crate::src::p_ceilng::activeceilings;
use crate::src::p_tick::P_InitThinkers;
use crate::src::p_setup::numlines;
use crate::src::p_maputl::P_SetThingPosition;
use crate::src::p_tick::thinkercap;
use crate::src::g_game::gameskill;
use crate::src::info::mobjinfo;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::p_setup::lines;
use crate::src::g_game::gameepisode;
use crate::src::g_game::gamemap;
use crate::src::info::states;
use crate::src::p_setup::numsectors;
use crate::src::p_setup::sides;
use crate::src::p_tick::P_AddThinker;
use crate::src::g_game::playeringame;
use crate::src::m_misc::M_snprintf;
use crate::src::p_setup::sectors;
use crate::src::p_tick::leveltime;
use crate::src::g_game::players;
use crate::src::p_plats::P_AddActivePlat;
use crate::src::m_misc::M_StringJoin;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVEL;
use crate::src::d_player::NUMPOWERS;
use crate::src::d_player::NUMPSPRITES;
use libc::memset;
use libc::{strcmp, strlen};
use libc::{malloc, snprintf};
use crate::src::i_system::{fprintf, fread, ftell, fwrite, stderr};
use crate::src::p_mobj::spritenum_t;
use crate::src::p_mobj::mobjtype_t;
use crate::src::d_mode::skill_t;
use crate::src::d_player::{NUMWEAPONS, weapontype_t};
use crate::src::p_plats::plattype_e;
use crate::src::p_plats::plat_e;
use crate::src::p_doors::vldoor_e;
use crate::src::p_floor::floor_e;
use crate::src::p_ceilng::ceiling_e;
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::doomdef::boolean;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;

use crate::src::p_mobj::P_MobjThinker;
use crate::src::p_lights::{T_LightFlash, T_StrobeFlash, T_Glow};
use crate::src::p_plats::T_PlatRaise;
use crate::src::p_doors::T_VerticalDoor;
use crate::src::p_ceilng::T_MoveCeiling;
use crate::src::p_floor::T_MoveFloor;
use crate::src::p_inter::NUMCARDS;
use crate::src::d_player::NUMAMMO;
pub type intptr_t = isize;
pub const NUMMOBJTYPES: mobjtype_t = 137;
pub const tc_end: C2RustUnnamed_4 = 0;
pub const tc_mobj: C2RustUnnamed_4 = 1;
pub const tc_endspecials: C2RustUnnamed_5 = 7;
pub const tc_glow: C2RustUnnamed_5 = 6;
pub const tc_strobe: C2RustUnnamed_5 = 5;
pub const tc_flash: C2RustUnnamed_5 = 4;
pub const tc_plat: C2RustUnnamed_5 = 3;
pub const tc_floor: C2RustUnnamed_5 = 2;
pub const tc_door: C2RustUnnamed_5 = 1;
pub const tc_ceiling: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_4 = u32;
pub type C2RustUnnamed_5 = u32;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXPLAYERS: i32 = 4 as i32;
pub const MAXCEILINGS: i32 = 30 as i32;
pub const SAVESTRINGSIZE: i32 = 24 as i32;
pub const SAVEGAME_EOF: i32 = 0x1d as i32;
pub const VERSIONSIZE: i32 = 16 as i32;
pub static mut save_stream: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut savegamelength: i32 = 0;
pub static mut savegame_error: bool = false;
pub unsafe fn P_TempSaveGameFile() -> *mut ::core::ffi::c_char {
    static mut filename: *mut ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >() as *mut ::core::ffi::c_char;
    if filename.is_null() {
        filename = M_StringJoin(
            savegamedir,
            b"temp.dsg\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
    }
    return filename;
}
pub unsafe fn P_SaveGameFile(
    mut slot: i32,
) -> *mut ::core::ffi::c_char {
    static mut filename: *mut ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >() as *mut ::core::ffi::c_char;
    static mut filename_size: size_t = 0 as size_t;
    let mut basename: [::core::ffi::c_char; 32] = [0; 32];
    if filename.is_null() {
        filename_size = strlen(savegamedir).wrapping_add(32 as size_t);
        filename = malloc(filename_size) as *mut ::core::ffi::c_char;
    }
    snprintf(
        &raw mut basename as *mut ::core::ffi::c_char,
        32 as size_t,
        b"doomsav%d.dsg\0" as *const u8 as *const ::core::ffi::c_char,
        slot,
    );
    M_snprintf(
        filename,
        filename_size,
        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        savegamedir,
        &raw mut basename as *mut ::core::ffi::c_char,
    );
    return filename;
}
unsafe extern "C" fn saveg_read8() -> byte {
    let mut result: byte = 0;
    if fread(
        &raw mut result as *mut ::core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        save_stream,
    ) < 1 as u64
    {
        if !savegame_error {
            fprintf(
                stderr,
                b"saveg_read8: Unexpected end of file while reading save game\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            savegame_error = true;
        }
    }
    return result;
}
unsafe extern "C" fn saveg_write8(mut value: byte) {
    if fwrite(
        &raw mut value as *const ::core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        save_stream,
    ) < 1 as u64
    {
        if !savegame_error {
            fprintf(
                stderr,
                b"saveg_write8: Error while writing save game\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            savegame_error = true;
        }
    }
}
unsafe extern "C" fn saveg_read16() -> i16 {
    let mut result: i32 = 0;
    result = saveg_read8() as i32;
    result |= (saveg_read8() as i32) << 8 as i32;
    return result as i16;
}
unsafe extern "C" fn saveg_write16(mut value: i16) {
    saveg_write8((value as i32 & 0xff as i32) as byte);
    saveg_write8(
        (value as i32 >> 8 as i32
            & 0xff as i32) as byte,
    );
}
unsafe extern "C" fn saveg_read32() -> i32 {
    let mut result: i32 = 0;
    result = saveg_read8() as i32;
    result |= (saveg_read8() as i32) << 8 as i32;
    result |= (saveg_read8() as i32) << 16 as i32;
    result |= (saveg_read8() as i32) << 24 as i32;
    return result;
}
unsafe extern "C" fn saveg_write32(mut value: i32) {
    saveg_write8((value & 0xff as i32) as byte);
    saveg_write8(
        (value >> 8 as i32 & 0xff as i32) as byte,
    );
    saveg_write8(
        (value >> 16 as i32 & 0xff as i32) as byte,
    );
    saveg_write8(
        (value >> 24 as i32 & 0xff as i32) as byte,
    );
}
unsafe extern "C" fn saveg_read_pad() {
    let mut pos: u64 = 0;
    let mut padding: i32 = 0;
    let mut i: i32 = 0;
    pos = ftell(save_stream) as u64;
    padding = ((4 as u64).wrapping_sub(pos & 3 as u64)
        & 3 as u64) as i32;
    i = 0 as i32;
    while i < padding {
        saveg_read8();
        i += 1;
    }
}
unsafe extern "C" fn saveg_write_pad() {
    let mut pos: u64 = 0;
    let mut padding: i32 = 0;
    let mut i: i32 = 0;
    pos = ftell(save_stream) as u64;
    padding = ((4 as u64).wrapping_sub(pos & 3 as u64)
        & 3 as u64) as i32;
    i = 0 as i32;
    while i < padding {
        saveg_write8(0 as byte);
        i += 1;
    }
}
unsafe extern "C" fn saveg_readp() -> *mut ::core::ffi::c_void {
    return saveg_read32() as intptr_t as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn saveg_writep(mut p: *mut ::core::ffi::c_void) {
    saveg_write32(p as intptr_t as i32);
}
unsafe extern "C" fn saveg_read_mapthing_t(mut str: *mut mapthing_t) {
    (*str).x = saveg_read16();
    (*str).y = saveg_read16();
    (*str).angle = saveg_read16();
    (*str).type_0 = saveg_read16();
    (*str).options = saveg_read16();
}
unsafe extern "C" fn saveg_write_mapthing_t(mut str: *mut mapthing_t) {
    saveg_write16((*str).x);
    saveg_write16((*str).y);
    saveg_write16((*str).angle);
    saveg_write16((*str).type_0);
    saveg_write16((*str).options);
}
unsafe extern "C" fn saveg_read_actionf_t(mut str: *mut ThinkerFn) {
    let word = saveg_readp();
    *str = if word.is_null() { ThinkerFn::Paused } else { ThinkerFn::Unresolved };
}
unsafe extern "C" fn saveg_write_actionf_t(mut str: *mut ThinkerFn) {
    let word: *mut ::core::ffi::c_void = if matches!(*str, ThinkerFn::Paused) {
        ::core::ptr::null_mut()
    } else {
        str as *mut ::core::ffi::c_void
    };
    saveg_writep(word);
}
unsafe extern "C" fn saveg_read_thinker_t(mut str: *mut thinker_t) {
    (*str).prev = saveg_readp() as *mut thinker_s;
    (*str).next = saveg_readp() as *mut thinker_s;
    saveg_read_actionf_t(&raw mut (*str).function);
}
unsafe extern "C" fn saveg_write_thinker_t(mut str: *mut thinker_t) {
    saveg_writep((*str).prev as *mut ::core::ffi::c_void);
    saveg_writep((*str).next as *mut ::core::ffi::c_void);
    saveg_write_actionf_t(&raw mut (*str).function);
}
unsafe extern "C" fn saveg_read_mobj_t(mut str: *mut mobj_t) {
    let mut pl: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).x = saveg_read32() as fixed_t;
    (*str).y = saveg_read32() as fixed_t;
    (*str).z = saveg_read32() as fixed_t;
    (*str).snext = saveg_readp() as *mut mobj_s;
    (*str).sprev = saveg_readp() as *mut mobj_s;
    (*str).angle = saveg_read32() as angle_t;
    (*str).sprite = saveg_read32() as spritenum_t;
    (*str).frame = saveg_read32();
    (*str).bnext = saveg_readp() as *mut mobj_s;
    (*str).bprev = saveg_readp() as *mut mobj_s;
    (*str).subsector = saveg_readp() as *mut subsector_s;
    (*str).floorz = saveg_read32() as fixed_t;
    (*str).ceilingz = saveg_read32() as fixed_t;
    (*str).radius = saveg_read32() as fixed_t;
    (*str).height = saveg_read32() as fixed_t;
    (*str).momx = saveg_read32() as fixed_t;
    (*str).momy = saveg_read32() as fixed_t;
    (*str).momz = saveg_read32() as fixed_t;
    (*str).validcount = saveg_read32();
    (*str).type_0 = saveg_read32() as mobjtype_t;
    (*str).info = saveg_readp() as *mut mobjinfo_t;
    (*str).tics = saveg_read32();
    (*str).state = (&raw mut states as *mut state_t)
        .offset(
            (saveg_read32 as unsafe extern "C" fn() -> i32)() as isize,
        ) as *mut state_t;
    (*str).flags = saveg_read32();
    (*str).health = saveg_read32();
    (*str).movedir = saveg_read32();
    (*str).movecount = saveg_read32();
    (*str).target = saveg_readp() as *mut mobj_s;
    (*str).reactiontime = saveg_read32();
    (*str).threshold = saveg_read32();
    pl = saveg_read32();
    if pl > 0 as i32 {
        (*str).player = (&raw mut players as *mut player_t)
            .offset((pl - 1 as i32) as isize) as *mut player_t
            as *mut player_s;
        (*(*str).player).mo = str;
    } else {
        (*str).player = ::core::ptr::null_mut::<player_s>();
    }
    (*str).lastlook = saveg_read32();
    saveg_read_mapthing_t(&raw mut (*str).spawnpoint);
    (*str).tracer = saveg_readp() as *mut mobj_s;
}
unsafe extern "C" fn saveg_write_mobj_t(mut str: *mut mobj_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).x as i32);
    saveg_write32((*str).y as i32);
    saveg_write32((*str).z as i32);
    saveg_writep((*str).snext as *mut ::core::ffi::c_void);
    saveg_writep((*str).sprev as *mut ::core::ffi::c_void);
    saveg_write32((*str).angle as i32);
    saveg_write32((*str).sprite as i32);
    saveg_write32((*str).frame);
    saveg_writep((*str).bnext as *mut ::core::ffi::c_void);
    saveg_writep((*str).bprev as *mut ::core::ffi::c_void);
    saveg_writep((*str).subsector as *mut ::core::ffi::c_void);
    saveg_write32((*str).floorz as i32);
    saveg_write32((*str).ceilingz as i32);
    saveg_write32((*str).radius as i32);
    saveg_write32((*str).height as i32);
    saveg_write32((*str).momx as i32);
    saveg_write32((*str).momy as i32);
    saveg_write32((*str).momz as i32);
    saveg_write32((*str).validcount);
    saveg_write32((*str).type_0 as i32);
    saveg_writep((*str).info as *mut ::core::ffi::c_void);
    saveg_write32((*str).tics);
    saveg_write32(
        (*str).state.offset_from(&raw mut states as *mut state_t) as i64
            as i32,
    );
    saveg_write32((*str).flags);
    saveg_write32((*str).health);
    saveg_write32((*str).movedir);
    saveg_write32((*str).movecount);
    saveg_writep((*str).target as *mut ::core::ffi::c_void);
    saveg_write32((*str).reactiontime);
    saveg_write32((*str).threshold);
    if !(*str).player.is_null() {
        saveg_write32(
            ((*str).player.offset_from(&raw mut players as *mut player_t)
                as i64 + 1 as i64) as i32,
        );
    } else {
        saveg_write32(0 as i32);
    }
    saveg_write32((*str).lastlook);
    saveg_write_mapthing_t(&raw mut (*str).spawnpoint);
    saveg_writep((*str).tracer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn saveg_read_ticcmd_t(mut str: *mut ticcmd_t) {
    (*str).forwardmove = saveg_read8() as i8;
    (*str).sidemove = saveg_read8() as i8;
    (*str).angleturn = saveg_read16();
    (*str).consistancy = saveg_read16() as byte;
    (*str).chatchar = saveg_read8();
    (*str).buttons = saveg_read8();
}
unsafe extern "C" fn saveg_write_ticcmd_t(mut str: *mut ticcmd_t) {
    saveg_write8((*str).forwardmove as byte);
    saveg_write8((*str).sidemove as byte);
    saveg_write16((*str).angleturn);
    saveg_write16((*str).consistancy as i16);
    saveg_write8((*str).chatchar);
    saveg_write8((*str).buttons);
}
unsafe extern "C" fn saveg_read_pspdef_t(mut str: *mut pspdef_t) {
    let mut state: i32 = 0;
    state = saveg_read32();
    if state > 0 as i32 {
        (*str).state = (&raw mut states as *mut state_t).offset(state as isize)
            as *mut state_t;
    } else {
        (*str).state = ::core::ptr::null_mut::<state_t>();
    }
    (*str).tics = saveg_read32();
    (*str).sx = saveg_read32() as fixed_t;
    (*str).sy = saveg_read32() as fixed_t;
}
unsafe extern "C" fn saveg_write_pspdef_t(mut str: *mut pspdef_t) {
    if !(*str).state.is_null() {
        saveg_write32(
            (*str).state.offset_from(&raw mut states as *mut state_t)
                as i64 as i32,
        );
    } else {
        saveg_write32(0 as i32);
    }
    saveg_write32((*str).tics);
    saveg_write32((*str).sx as i32);
    saveg_write32((*str).sy as i32);
}
unsafe extern "C" fn saveg_read_player_t(mut str: *mut player_t) {
    let mut i: i32 = 0;
    (*str).mo = saveg_readp() as *mut mobj_t;
    (*str).playerstate = saveg_read32() as playerstate_t;
    saveg_read_ticcmd_t(&raw mut (*str).cmd);
    (*str).viewz = saveg_read32() as fixed_t;
    (*str).viewheight = saveg_read32() as fixed_t;
    (*str).deltaviewheight = saveg_read32() as fixed_t;
    (*str).bob = saveg_read32() as fixed_t;
    (*str).health = saveg_read32();
    (*str).armorpoints = saveg_read32();
    (*str).armortype = saveg_read32();
    i = 0 as i32;
    while i < NUMPOWERS as i32 {
        (*str).powers[i as usize] = saveg_read32();
        i += 1;
    }
    i = 0 as i32;
    while i < NUMCARDS as i32 {
        (*str).cards[i as usize] = saveg_read32() != 0;
        i += 1;
    }
    (*str).backpack = saveg_read32() != 0;
    i = 0 as i32;
    while i < MAXPLAYERS {
        (*str).frags[i as usize] = saveg_read32();
        i += 1;
    }
    (*str).readyweapon = saveg_read32() as weapontype_t;
    (*str).pendingweapon = saveg_read32() as weapontype_t;
    i = 0 as i32;
    while i < NUMWEAPONS as i32 {
        (*str).weaponowned[i as usize] = saveg_read32() != 0;
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        (*str).ammo[i as usize] = saveg_read32();
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        (*str).maxammo[i as usize] = saveg_read32();
        i += 1;
    }
    (*str).attackdown = saveg_read32();
    (*str).usedown = saveg_read32();
    (*str).cheats = saveg_read32();
    (*str).refire = saveg_read32();
    (*str).killcount = saveg_read32();
    (*str).itemcount = saveg_read32();
    (*str).secretcount = saveg_read32();
    (*str).message = saveg_readp() as *mut ::core::ffi::c_char;
    (*str).damagecount = saveg_read32();
    (*str).bonuscount = saveg_read32();
    (*str).attacker = saveg_readp() as *mut mobj_t;
    (*str).extralight = saveg_read32();
    (*str).fixedcolormap = saveg_read32();
    (*str).colormap = saveg_read32();
    i = 0 as i32;
    while i < NUMPSPRITES as i32 {
        saveg_read_pspdef_t(
            (&raw mut (*str).psprites as *mut pspdef_t).offset(i as isize)
                as *mut pspdef_t,
        );
        i += 1;
    }
    (*str).didsecret = saveg_read32() != 0;
}
unsafe extern "C" fn saveg_write_player_t(mut str: *mut player_t) {
    let mut i: i32 = 0;
    saveg_writep((*str).mo as *mut ::core::ffi::c_void);
    saveg_write32((*str).playerstate as i32);
    saveg_write_ticcmd_t(&raw mut (*str).cmd);
    saveg_write32((*str).viewz as i32);
    saveg_write32((*str).viewheight as i32);
    saveg_write32((*str).deltaviewheight as i32);
    saveg_write32((*str).bob as i32);
    saveg_write32((*str).health);
    saveg_write32((*str).armorpoints);
    saveg_write32((*str).armortype);
    i = 0 as i32;
    while i < NUMPOWERS as i32 {
        saveg_write32((*str).powers[i as usize]);
        i += 1;
    }
    i = 0 as i32;
    while i < NUMCARDS as i32 {
        saveg_write32((*str).cards[i as usize] as i32);
        i += 1;
    }
    saveg_write32((*str).backpack as i32);
    i = 0 as i32;
    while i < MAXPLAYERS {
        saveg_write32((*str).frags[i as usize]);
        i += 1;
    }
    saveg_write32((*str).readyweapon as i32);
    saveg_write32((*str).pendingweapon as i32);
    i = 0 as i32;
    while i < NUMWEAPONS as i32 {
        saveg_write32((*str).weaponowned[i as usize] as i32);
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        saveg_write32((*str).ammo[i as usize]);
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        saveg_write32((*str).maxammo[i as usize]);
        i += 1;
    }
    saveg_write32((*str).attackdown);
    saveg_write32((*str).usedown);
    saveg_write32((*str).cheats);
    saveg_write32((*str).refire);
    saveg_write32((*str).killcount);
    saveg_write32((*str).itemcount);
    saveg_write32((*str).secretcount);
    saveg_writep((*str).message as *mut ::core::ffi::c_void);
    saveg_write32((*str).damagecount);
    saveg_write32((*str).bonuscount);
    saveg_writep((*str).attacker as *mut ::core::ffi::c_void);
    saveg_write32((*str).extralight);
    saveg_write32((*str).fixedcolormap);
    saveg_write32((*str).colormap);
    i = 0 as i32;
    while i < NUMPSPRITES as i32 {
        saveg_write_pspdef_t(
            (&raw mut (*str).psprites as *mut pspdef_t).offset(i as isize)
                as *mut pspdef_t,
        );
        i += 1;
    }
    saveg_write32((*str).didsecret as i32);
}
unsafe extern "C" fn saveg_read_ceiling_t(mut str: *mut ceiling_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).type_0 = saveg_read32() as ceiling_e;
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).bottomheight = saveg_read32() as fixed_t;
    (*str).topheight = saveg_read32() as fixed_t;
    (*str).speed = saveg_read32() as fixed_t;
    (*str).crush = saveg_read32() != 0;
    (*str).direction = saveg_read32();
    (*str).tag = saveg_read32();
    (*str).olddirection = saveg_read32();
}
unsafe extern "C" fn saveg_write_ceiling_t(mut str: *mut ceiling_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).type_0 as i32);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).bottomheight as i32);
    saveg_write32((*str).topheight as i32);
    saveg_write32((*str).speed as i32);
    saveg_write32((*str).crush as i32);
    saveg_write32((*str).direction);
    saveg_write32((*str).tag);
    saveg_write32((*str).olddirection);
}
unsafe extern "C" fn saveg_read_vldoor_t(mut str: *mut vldoor_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).type_0 = saveg_read32() as vldoor_e;
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).topheight = saveg_read32() as fixed_t;
    (*str).speed = saveg_read32() as fixed_t;
    (*str).direction = saveg_read32();
    (*str).topwait = saveg_read32();
    (*str).topcountdown = saveg_read32();
}
unsafe extern "C" fn saveg_write_vldoor_t(mut str: *mut vldoor_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).type_0 as i32);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).topheight as i32);
    saveg_write32((*str).speed as i32);
    saveg_write32((*str).direction);
    saveg_write32((*str).topwait);
    saveg_write32((*str).topcountdown);
}
unsafe extern "C" fn saveg_read_floormove_t(mut str: *mut floormove_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).type_0 = saveg_read32() as floor_e;
    (*str).crush = saveg_read32() != 0;
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).direction = saveg_read32();
    (*str).newspecial = saveg_read32();
    (*str).texture = saveg_read16();
    (*str).floordestheight = saveg_read32() as fixed_t;
    (*str).speed = saveg_read32() as fixed_t;
}
unsafe extern "C" fn saveg_write_floormove_t(mut str: *mut floormove_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).type_0 as i32);
    saveg_write32((*str).crush as i32);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).direction);
    saveg_write32((*str).newspecial);
    saveg_write16((*str).texture);
    saveg_write32((*str).floordestheight as i32);
    saveg_write32((*str).speed as i32);
}
unsafe extern "C" fn saveg_read_plat_t(mut str: *mut plat_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).speed = saveg_read32() as fixed_t;
    (*str).low = saveg_read32() as fixed_t;
    (*str).high = saveg_read32() as fixed_t;
    (*str).wait = saveg_read32();
    (*str).count = saveg_read32();
    (*str).status = saveg_read32() as plat_e;
    (*str).oldstatus = saveg_read32() as plat_e;
    (*str).crush = saveg_read32() != 0;
    (*str).tag = saveg_read32();
    (*str).type_0 = saveg_read32() as plattype_e;
}
unsafe extern "C" fn saveg_write_plat_t(mut str: *mut plat_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).speed as i32);
    saveg_write32((*str).low as i32);
    saveg_write32((*str).high as i32);
    saveg_write32((*str).wait);
    saveg_write32((*str).count);
    saveg_write32((*str).status as i32);
    saveg_write32((*str).oldstatus as i32);
    saveg_write32((*str).crush as i32);
    saveg_write32((*str).tag);
    saveg_write32((*str).type_0 as i32);
}
unsafe extern "C" fn saveg_read_lightflash_t(mut str: *mut lightflash_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).count = saveg_read32();
    (*str).maxlight = saveg_read32();
    (*str).minlight = saveg_read32();
    (*str).maxtime = saveg_read32();
    (*str).mintime = saveg_read32();
}
unsafe extern "C" fn saveg_write_lightflash_t(mut str: *mut lightflash_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).count);
    saveg_write32((*str).maxlight);
    saveg_write32((*str).minlight);
    saveg_write32((*str).maxtime);
    saveg_write32((*str).mintime);
}
unsafe extern "C" fn saveg_read_strobe_t(mut str: *mut strobe_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).count = saveg_read32();
    (*str).minlight = saveg_read32();
    (*str).maxlight = saveg_read32();
    (*str).darktime = saveg_read32();
    (*str).brighttime = saveg_read32();
}
unsafe extern "C" fn saveg_write_strobe_t(mut str: *mut strobe_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).count);
    saveg_write32((*str).minlight);
    saveg_write32((*str).maxlight);
    saveg_write32((*str).darktime);
    saveg_write32((*str).brighttime);
}
unsafe extern "C" fn saveg_read_glow_t(mut str: *mut glow_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).minlight = saveg_read32();
    (*str).maxlight = saveg_read32();
    (*str).direction = saveg_read32();
}
unsafe extern "C" fn saveg_write_glow_t(mut str: *mut glow_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).minlight);
    saveg_write32((*str).maxlight);
    saveg_write32((*str).direction);
}
pub unsafe fn P_WriteSaveGameHeader(
    mut description: *mut ::core::ffi::c_char,
) {
    let mut name: [::core::ffi::c_char; 16] = [0; 16];
    let mut i: i32 = 0;
    i = 0 as i32;
    while *description.offset(i as isize) as i32 != '\0' as i32 {
        saveg_write8(*description.offset(i as isize) as byte);
        i += 1;
    }
    while i < SAVESTRINGSIZE {
        saveg_write8(0 as byte);
        i += 1;
    }
    memset(
        &raw mut name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
    );
    M_snprintf(
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        b"version %i\0" as *const u8 as *const ::core::ffi::c_char,
        G_VanillaVersionCode(),
    );
    i = 0 as i32;
    while i < VERSIONSIZE {
        saveg_write8(name[i as usize] as byte);
        i += 1;
    }
    saveg_write8(gameskill as byte);
    saveg_write8(gameepisode as byte);
    saveg_write8(gamemap as byte);
    i = 0 as i32;
    while i < MAXPLAYERS {
        saveg_write8(playeringame[i as usize] as byte);
        i += 1;
    }
    saveg_write8(
        (leveltime >> 16 as i32 & 0xff as i32) as byte,
    );
    saveg_write8(
        (leveltime >> 8 as i32 & 0xff as i32) as byte,
    );
    saveg_write8((leveltime & 0xff as i32) as byte);
}
pub unsafe fn P_ReadSaveGameHeader() -> bool {
    let mut i: i32 = 0;
    let mut a: byte = 0;
    let mut b: byte = 0;
    let mut c: byte = 0;
    let mut vcheck: [::core::ffi::c_char; 16] = [0; 16];
    let mut read_vcheck: [::core::ffi::c_char; 16] = [0; 16];
    i = 0 as i32;
    while i < SAVESTRINGSIZE {
        saveg_read8();
        i += 1;
    }
    i = 0 as i32;
    while i < VERSIONSIZE {
        read_vcheck[i as usize] = saveg_read8() as ::core::ffi::c_char;
        i += 1;
    }
    memset(
        &raw mut vcheck as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
    );
    M_snprintf(
        &raw mut vcheck as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        b"version %i\0" as *const u8 as *const ::core::ffi::c_char,
        G_VanillaVersionCode(),
    );
    if strcmp(
        &raw mut read_vcheck as *mut ::core::ffi::c_char,
        &raw mut vcheck as *mut ::core::ffi::c_char,
    ) != 0 as i32
    {
        return false;
    }
    gameskill = saveg_read8() as skill_t;
    gameepisode = saveg_read8() as i32;
    gamemap = saveg_read8() as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        playeringame[i as usize] = saveg_read8() as boolean;
        i += 1;
    }
    a = saveg_read8();
    b = saveg_read8();
    c = saveg_read8();
    leveltime = ((a as i32) << 16 as i32)
        + ((b as i32) << 8 as i32)
        + c as i32;
    return true;
}
pub unsafe fn P_ReadSaveGameEOF() -> bool {
    let mut value: i32 = 0;
    value = saveg_read8() as i32;
    return value == SAVEGAME_EOF;
}
pub unsafe fn P_WriteSaveGameEOF() {
    saveg_write8(SAVEGAME_EOF as byte);
}
pub unsafe fn P_ArchivePlayers() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            saveg_write_pad();
            saveg_write_player_t(
                (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t,
            );
        }
        i += 1;
    }
}
pub unsafe fn P_UnArchivePlayers() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            saveg_read_pad();
            saveg_read_player_t(
                (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t,
            );
            players[i as usize].mo = ::core::ptr::null_mut::<mobj_t>();
            players[i as usize].message = ::core::ptr::null_mut::<::core::ffi::c_char>();
            players[i as usize].attacker = ::core::ptr::null_mut::<mobj_t>();
        }
        i += 1;
    }
}
pub unsafe fn P_ArchiveWorld() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut si: *mut side_t = ::core::ptr::null_mut::<side_t>();
    i = 0 as i32;
    sec = sectors;
    while i < numsectors {
        saveg_write16(((*sec).floorheight >> FRACBITS) as i16);
        saveg_write16(((*sec).ceilingheight >> FRACBITS) as i16);
        saveg_write16((*sec).floorpic);
        saveg_write16((*sec).ceilingpic);
        saveg_write16((*sec).lightlevel);
        saveg_write16((*sec).special);
        saveg_write16((*sec).tag);
        i += 1;
        sec = sec.offset(1);
    }
    i = 0 as i32;
    li = lines;
    while i < numlines {
        saveg_write16((*li).flags);
        saveg_write16((*li).special);
        saveg_write16((*li).tag);
        j = 0 as i32;
        while j < 2 as i32 {
            if !((*li).sidenum[j as usize] as i32
                == -(1 as i32))
            {
                si = sides
                    .offset(
                        *(&raw mut (*li).sidenum as *mut i16)
                            .offset(j as isize) as isize,
                    ) as *mut side_t;
                saveg_write16(((*si).textureoffset >> FRACBITS) as i16);
                saveg_write16(((*si).rowoffset >> FRACBITS) as i16);
                saveg_write16((*si).toptexture);
                saveg_write16((*si).bottomtexture);
                saveg_write16((*si).midtexture);
            }
            j += 1;
        }
        i += 1;
        li = li.offset(1);
    }
}
pub unsafe fn P_UnArchiveWorld() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut si: *mut side_t = ::core::ptr::null_mut::<side_t>();
    i = 0 as i32;
    sec = sectors;
    while i < numsectors {
        (*sec).floorheight = ((saveg_read16() as i32) << FRACBITS)
            as fixed_t;
        (*sec).ceilingheight = ((saveg_read16() as i32) << FRACBITS)
            as fixed_t;
        (*sec).floorpic = saveg_read16();
        (*sec).ceilingpic = saveg_read16();
        (*sec).lightlevel = saveg_read16();
        (*sec).special = saveg_read16();
        (*sec).tag = saveg_read16();
        (*sec).specialdata = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*sec).soundtarget = ::core::ptr::null_mut::<mobj_t>();
        i += 1;
        sec = sec.offset(1);
    }
    i = 0 as i32;
    li = lines;
    while i < numlines {
        (*li).flags = saveg_read16();
        (*li).special = saveg_read16();
        (*li).tag = saveg_read16();
        j = 0 as i32;
        while j < 2 as i32 {
            if !((*li).sidenum[j as usize] as i32
                == -(1 as i32))
            {
                si = sides
                    .offset(
                        *(&raw mut (*li).sidenum as *mut i16)
                            .offset(j as isize) as isize,
                    ) as *mut side_t;
                (*si).textureoffset = ((saveg_read16() as i32)
                    << FRACBITS) as fixed_t;
                (*si).rowoffset = ((saveg_read16() as i32) << FRACBITS)
                    as fixed_t;
                (*si).toptexture = saveg_read16();
                (*si).bottomtexture = saveg_read16();
                (*si).midtexture = saveg_read16();
            }
            j += 1;
        }
        i += 1;
        li = li.offset(1);
    }
}
pub unsafe fn P_ArchiveThinkers() {
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        if matches!((*th).function, ThinkerFn::Mobj(_))
        {
            saveg_write8(tc_mobj as i32 as byte);
            saveg_write_pad();
            saveg_write_mobj_t(th as *mut mobj_t);
        }
        th = (*th).next as *mut thinker_t;
    }
    saveg_write8(tc_end as i32 as byte);
}
pub unsafe fn P_UnArchiveThinkers() {
    let mut tclass: byte = 0;
    let mut currentthinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut next: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut mobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    currentthinker = thinkercap.next as *mut thinker_t;
    while currentthinker != &raw mut thinkercap {
        next = (*currentthinker).next as *mut thinker_t;
        if matches!((*currentthinker).function, ThinkerFn::Mobj(_))
        {
            P_RemoveMobj(currentthinker as *mut mobj_t);
        } else {
            Z_Free(currentthinker as *mut ::core::ffi::c_void);
        }
        currentthinker = next;
    }
    P_InitThinkers();
    loop {
        tclass = saveg_read8();
        match tclass as i32 {
            0 => return,
            1 => {
                saveg_read_pad();
                mobj = Z_Malloc(
                    ::core::mem::size_of::<mobj_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut mobj_t;
                saveg_read_mobj_t(mobj);
                (*mobj).target = ::core::ptr::null_mut::<mobj_s>();
                (*mobj).tracer = ::core::ptr::null_mut::<mobj_s>();
                P_SetThingPosition(mobj);
                (*mobj).info = (&raw mut mobjinfo as *mut mobjinfo_t)
                    .offset((*mobj).type_0 as isize) as *mut mobjinfo_t;
                (*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
                (*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
                (*mobj).thinker.function = ThinkerFn::Mobj(P_MobjThinker);
                P_AddThinker(&raw mut (*mobj).thinker);
            }
            _ => {
                I_Error(&format!(
                    "Unknown tclass {} in savegame",
                    tclass as i32,
                ));
            }
        }
    };
}
#[no_mangle]
pub static mut specials_e: C2RustUnnamed_5 = tc_ceiling;
pub unsafe fn P_ArchiveSpecials() {
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut i: i32 = 0;
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        match (*th).function {
            ThinkerFn::Paused => {
                i = 0 as i32;
                while i < MAXCEILINGS {
                    if activeceilings[i as usize] == th as *mut ceiling_t {
                        break;
                    }
                    i += 1;
                }
                if i < MAXCEILINGS {
                    saveg_write8(tc_ceiling as i32 as byte);
                    saveg_write_pad();
                    saveg_write_ceiling_t(th as *mut ceiling_t);
                }
            }
            ThinkerFn::Ceiling(_) => {
                saveg_write8(tc_ceiling as i32 as byte);
                saveg_write_pad();
                saveg_write_ceiling_t(th as *mut ceiling_t);
            }
            ThinkerFn::Door(_) => {
                saveg_write8(tc_door as i32 as byte);
                saveg_write_pad();
                saveg_write_vldoor_t(th as *mut vldoor_t);
            }
            ThinkerFn::Floor(_) => {
                saveg_write8(tc_floor as i32 as byte);
                saveg_write_pad();
                saveg_write_floormove_t(th as *mut floormove_t);
            }
            ThinkerFn::Plat(_) => {
                saveg_write8(tc_plat as i32 as byte);
                saveg_write_pad();
                saveg_write_plat_t(th as *mut plat_t);
            }
            ThinkerFn::LightFlash(_) => {
                saveg_write8(tc_flash as i32 as byte);
                saveg_write_pad();
                saveg_write_lightflash_t(th as *mut lightflash_t);
            }
            ThinkerFn::Strobe(_) => {
                saveg_write8(tc_strobe as i32 as byte);
                saveg_write_pad();
                saveg_write_strobe_t(th as *mut strobe_t);
            }
            ThinkerFn::Glow(_) => {
                saveg_write8(tc_glow as i32 as byte);
                saveg_write_pad();
                saveg_write_glow_t(th as *mut glow_t);
            }
            _ => {}
        }
        th = (*th).next as *mut thinker_t;
    }
    saveg_write8(tc_endspecials as i32 as byte);
}
pub unsafe fn P_UnArchiveSpecials() {
    let mut tclass: byte = 0;
    let mut ceiling: *mut ceiling_t = ::core::ptr::null_mut::<ceiling_t>();
    let mut door: *mut vldoor_t = ::core::ptr::null_mut::<vldoor_t>();
    let mut floor: *mut floormove_t = ::core::ptr::null_mut::<floormove_t>();
    let mut plat: *mut plat_t = ::core::ptr::null_mut::<plat_t>();
    let mut flash: *mut lightflash_t = ::core::ptr::null_mut::<lightflash_t>();
    let mut strobe: *mut strobe_t = ::core::ptr::null_mut::<strobe_t>();
    let mut glow: *mut glow_t = ::core::ptr::null_mut::<glow_t>();
    loop {
        tclass = saveg_read8();
        match tclass as i32 {
            7 => return,
            0 => {
                saveg_read_pad();
                ceiling = Z_Malloc(
                    ::core::mem::size_of::<ceiling_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut ceiling_t;
                saveg_read_ceiling_t(ceiling);
                (*(*ceiling).sector).specialdata = ceiling as *mut ::core::ffi::c_void;
                if matches!((*ceiling).thinker.function, ThinkerFn::Unresolved) {
                    (*ceiling).thinker.function = ThinkerFn::Ceiling(T_MoveCeiling);
                }
                P_AddThinker(&raw mut (*ceiling).thinker);
                P_AddActiveCeiling(ceiling);
            }
            1 => {
                saveg_read_pad();
                door = Z_Malloc(
                    ::core::mem::size_of::<vldoor_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut vldoor_t;
                saveg_read_vldoor_t(door);
                (*(*door).sector).specialdata = door as *mut ::core::ffi::c_void;
                (*door).thinker.function = ThinkerFn::Door(T_VerticalDoor);
                P_AddThinker(&raw mut (*door).thinker);
            }
            2 => {
                saveg_read_pad();
                floor = Z_Malloc(
                    ::core::mem::size_of::<floormove_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut floormove_t;
                saveg_read_floormove_t(floor);
                (*(*floor).sector).specialdata = floor as *mut ::core::ffi::c_void;
                (*floor).thinker.function = ThinkerFn::Floor(T_MoveFloor);
                P_AddThinker(&raw mut (*floor).thinker);
            }
            3 => {
                saveg_read_pad();
                plat = Z_Malloc(
                    ::core::mem::size_of::<plat_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut plat_t;
                saveg_read_plat_t(plat);
                (*(*plat).sector).specialdata = plat as *mut ::core::ffi::c_void;
                if matches!((*plat).thinker.function, ThinkerFn::Unresolved) {
                    (*plat).thinker.function = ThinkerFn::Plat(T_PlatRaise);
                }
                P_AddThinker(&raw mut (*plat).thinker);
                P_AddActivePlat(plat);
            }
            4 => {
                saveg_read_pad();
                flash = Z_Malloc(
                    ::core::mem::size_of::<lightflash_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut lightflash_t;
                saveg_read_lightflash_t(flash);
                (*flash).thinker.function = ThinkerFn::LightFlash(T_LightFlash);
                P_AddThinker(&raw mut (*flash).thinker);
            }
            5 => {
                saveg_read_pad();
                strobe = Z_Malloc(
                    ::core::mem::size_of::<strobe_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut strobe_t;
                saveg_read_strobe_t(strobe);
                (*strobe).thinker.function = ThinkerFn::Strobe(T_StrobeFlash);
                P_AddThinker(&raw mut (*strobe).thinker);
            }
            6 => {
                saveg_read_pad();
                glow = Z_Malloc(
                    ::core::mem::size_of::<glow_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut glow_t;
                saveg_read_glow_t(glow);
                (*glow).thinker.function = ThinkerFn::Glow(T_Glow);
                P_AddThinker(&raw mut (*glow).thinker);
            }
            _ => {
                I_Error(&format!(
                    "P_UnarchiveSpecials:Unknown tclass {} in savegame",
                    tclass as i32,
                ));
            }
        }
    };
}
