use crate::src::hu_lib::patch_t;
use crate::src::st_lib::{st_number_t, st_percent_t, st_multicon_t, st_binicon_t};
use crate::src::m_cheat::cheatseq_t;
use crate::src::d_items::{weaponinfo_t, weaponinfo};
use crate::src::d_event::event_t;
use crate::src::d_player::{player_t};
use crate::src::w_wad::{
    wad_name8_to_string, W_CacheLumpName, W_GetNumForName, W_ReleaseLumpName,
};
use crate::src::m_cheat::cht_GetParam;
use crate::src::st_lib::STlib_initNum;
use crate::src::st_lib::STlib_updateNum;
use crate::src::st_lib::STlib_initPercent;
use crate::src::st_lib::STlib_updatePercent;
use crate::src::st_lib::STlib_initMultIcon;
use crate::src::st_lib::STlib_updateMultIcon;
use crate::src::st_lib::STlib_initBinIcon;
use crate::src::st_lib::STlib_updateBinIcon;
use crate::src::p_inter::P_GivePower;
use crate::src::g_game::G_DeferedInitNew;
use crate::src::m_cheat::cht_CheckCheat;
use crate::src::v_video::V_UseBuffer;
use crate::src::i_video::I_SetPalette;
use crate::src::m_random::M_Random;
use crate::src::s_sound::S_ChangeMusic;
use crate::src::v_video::V_RestoreBuffer;
use crate::src::g_game::gameskill;
use crate::src::doomstat::gamemission;
use crate::src::am_map::automapactive;
use crate::src::r_main::R_PointToAngle2;
use crate::src::g_game::deathmatch;
use crate::src::m_misc::M_snprintf;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::st_lib::STlib_init;
use crate::src::v_video::V_CopyRect;
use crate::src::v_video::V_DrawPatch;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use crate::src::sounds::{mus_e1m1, mus_runnin};
use crate::src::d_player::{pw_invulnerability, pw_ironfeet, pw_strength};
use crate::src::d_player::{CF_GODMODE, CF_NOCLIP};
use crate::src::p_mobj::mobjtype_t;
use crate::src::d_mode::{commercial, registered, retail, shareware};
use crate::src::d_mode::{exe_chex, exe_ultimate};
use crate::src::d_mode::{doom, doom2, pack_chex, pack_hacx};
use crate::src::d_mode::sk_nightmare;
use crate::src::d_event::{ev_keydown, ev_keyup};
use crate::src::d_player::{NUMWEAPONS, wp_chainsaw};
use crate::src::d_player::{NUMAMMO, am_noammo};
use crate::src::tables::angle_t;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use libc::snprintf;

pub type C2RustUnnamed_0 = u32;
pub const NUMCARDS: C2RustUnnamed_0 = 6;
pub const it_redskull: C2RustUnnamed_0 = 5;
pub const it_yellowskull: C2RustUnnamed_0 = 4;
pub const it_blueskull: C2RustUnnamed_0 = 3;
pub const it_redcard: C2RustUnnamed_0 = 2;
pub const it_yellowcard: C2RustUnnamed_0 = 1;
pub const it_bluecard: C2RustUnnamed_0 = 0;
pub const NUMMOBJTYPES: mobjtype_t = 137;
pub type st_stateenum_t = u32;
pub const FirstPersonState: st_stateenum_t = 1;
pub const AutomapState: st_stateenum_t = 0;
pub type st_chatstateenum_t = u32;
pub const GetChatState: st_chatstateenum_t = 2;
pub const WaitDestState: st_chatstateenum_t = 1;
pub const StartChatState: st_chatstateenum_t = 0;
pub type load_callback_t = Option<
    unsafe fn(*mut ::core::ffi::c_char, *mut *mut patch_t) -> (),
>;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const DEH_DEFAULT_GOD_MODE_HEALTH: i32 = 100 as i32;
pub const DEH_DEFAULT_IDFA_ARMOR: i32 = 200 as i32;
pub const DEH_DEFAULT_IDFA_ARMOR_CLASS: i32 = 2 as i32;
pub const DEH_DEFAULT_IDKFA_ARMOR: i32 = 200 as i32;
pub const DEH_DEFAULT_IDKFA_ARMOR_CLASS: i32 = 2 as i32;
pub const deh_god_mode_health: i32 = DEH_DEFAULT_GOD_MODE_HEALTH;
pub const deh_idfa_armor: i32 = DEH_DEFAULT_IDFA_ARMOR;
pub const deh_idfa_armor_class: i32 = DEH_DEFAULT_IDFA_ARMOR_CLASS;
pub const deh_idkfa_armor: i32 = DEH_DEFAULT_IDKFA_ARMOR;
pub const deh_idkfa_armor_class: i32 = DEH_DEFAULT_IDKFA_ARMOR_CLASS;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TICRATE: i32 = 35 as i32;
pub const MAXPLAYERS: i32 = 4 as i32;
pub const ST_HEIGHT: i32 = 32 as i32;
pub const ST_WIDTH: i32 = SCREENWIDTH;
pub const ST_Y: i32 = SCREENHEIGHT - ST_HEIGHT;
pub const ANG45: i32 = 0x20000000 as i32;
pub const ANG180: u32 = 0x80000000 as u32;
pub const AM_MSGHEADER: i32 = (('a' as i32) << 24 as i32)
    + (('m' as i32) << 16 as i32);
pub const AM_MSGENTERED: i32 = 1634559232;
pub const AM_MSGEXITED: i32 = 1634564096;
pub const STARTREDPALS: i32 = 1 as i32;
pub const STARTBONUSPALS: i32 = 9 as i32;
pub const NUMREDPALS: i32 = 8 as i32;
pub const NUMBONUSPALS: i32 = 4 as i32;
pub const RADIATIONPAL: i32 = 13 as i32;
pub const ST_X: i32 = 0 as i32;
pub const ST_FX: i32 = 143 as i32;
pub const ST_NUMPAINFACES: i32 = 5 as i32;
pub const ST_NUMSTRAIGHTFACES: i32 = 3 as i32;
pub const ST_NUMTURNFACES: i32 = 2 as i32;
pub const ST_NUMSPECIALFACES: i32 = 3 as i32;
pub const ST_FACESTRIDE: i32 = ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES
    + ST_NUMSPECIALFACES;
pub const ST_TURNOFFSET: i32 = 3 as i32;
pub const ST_OUCHOFFSET: i32 = ST_TURNOFFSET + ST_NUMTURNFACES;
pub const ST_EVILGRINOFFSET: i32 = ST_OUCHOFFSET
    + 1 as i32;
pub const ST_RAMPAGEOFFSET: i32 = ST_EVILGRINOFFSET
    + 1 as i32;
pub const ST_GODFACE: i32 = ST_NUMPAINFACES * ST_FACESTRIDE;
pub const ST_DEADFACE: i32 = ST_GODFACE + 1 as i32;
pub const ST_FACESX: i32 = 143 as i32;
pub const ST_FACESY: i32 = 168 as i32;
pub const ST_EVILGRINCOUNT: i32 = 2 as i32 * TICRATE;
pub const ST_STRAIGHTFACECOUNT: i32 = TICRATE / 2 as i32;
pub const ST_TURNCOUNT: i32 = 1 as i32 * TICRATE;
pub const ST_RAMPAGEDELAY: i32 = 2 as i32 * TICRATE;
pub const ST_MUCHPAIN: i32 = 20 as i32;
pub const ST_AMMOWIDTH: i32 = 3 as i32;
pub const ST_AMMOX: i32 = 44 as i32;
pub const ST_AMMOY: i32 = 171 as i32;
pub const ST_HEALTHX: i32 = 90 as i32;
pub const ST_HEALTHY: i32 = 171 as i32;
pub const ST_ARMSX: i32 = 111 as i32;
pub const ST_ARMSY: i32 = 172 as i32;
pub const ST_ARMSBGX: i32 = 104 as i32;
pub const ST_ARMSBGY: i32 = 168 as i32;
pub const ST_ARMSXSPACE: i32 = 12 as i32;
pub const ST_ARMSYSPACE: i32 = 10 as i32;
pub const ST_FRAGSX: i32 = 138 as i32;
pub const ST_FRAGSY: i32 = 171 as i32;
pub const ST_FRAGSWIDTH: i32 = 2 as i32;
pub const ST_ARMORX: i32 = 221 as i32;
pub const ST_ARMORY: i32 = 171 as i32;
pub const ST_KEY0X: i32 = 239 as i32;
pub const ST_KEY0Y: i32 = 171 as i32;
pub const ST_KEY1X: i32 = 239 as i32;
pub const ST_KEY1Y: i32 = 181 as i32;
pub const ST_KEY2X: i32 = 239 as i32;
pub const ST_KEY2Y: i32 = 191 as i32;
pub const ST_AMMO0WIDTH: i32 = 3 as i32;
pub const ST_AMMO0X: i32 = 288 as i32;
pub const ST_AMMO0Y: i32 = 173 as i32;
pub const ST_AMMO1WIDTH: i32 = ST_AMMO0WIDTH;
pub const ST_AMMO1X: i32 = 288 as i32;
pub const ST_AMMO1Y: i32 = 179 as i32;
pub const ST_AMMO2WIDTH: i32 = ST_AMMO0WIDTH;
pub const ST_AMMO2X: i32 = 288 as i32;
pub const ST_AMMO2Y: i32 = 191 as i32;
pub const ST_AMMO3WIDTH: i32 = ST_AMMO0WIDTH;
pub const ST_AMMO3X: i32 = 288 as i32;
pub const ST_AMMO3Y: i32 = 185 as i32;
pub const ST_MAXAMMO0WIDTH: i32 = 3 as i32;
pub const ST_MAXAMMO0X: i32 = 314 as i32;
pub const ST_MAXAMMO0Y: i32 = 173 as i32;
pub const ST_MAXAMMO1WIDTH: i32 = ST_MAXAMMO0WIDTH;
pub const ST_MAXAMMO1X: i32 = 314 as i32;
pub const ST_MAXAMMO1Y: i32 = 179 as i32;
pub const ST_MAXAMMO2WIDTH: i32 = ST_MAXAMMO0WIDTH;
pub const ST_MAXAMMO2X: i32 = 314 as i32;
pub const ST_MAXAMMO2Y: i32 = 191 as i32;
pub const ST_MAXAMMO3WIDTH: i32 = ST_MAXAMMO0WIDTH;
pub const ST_MAXAMMO3X: i32 = 314 as i32;
pub const ST_MAXAMMO3Y: i32 = 185 as i32;
pub static mut st_backing_screen: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut plyr: *mut player_t = ::core::ptr::null::<player_t>() as *mut player_t;
static mut st_firsttime: bool = false;
static mut lu_palette: i32 = 0;
static mut st_clock: u32 = 0;
static mut st_msgcounter: i32 = 0 as i32;
static mut st_chatstate: st_chatstateenum_t = StartChatState;
static mut st_gamestate: st_stateenum_t = AutomapState;
static mut st_statusbaron: bool = false;
static mut st_chat: bool = false;
static mut st_oldchat: bool = false;
static mut st_cursoron: bool = false;
static mut st_notdeathmatch: bool = false;
static mut st_armson: bool = false;
static mut st_fragson: bool = false;
static mut sbar: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut tallnum: [*mut patch_t; 10] = [::core::ptr::null::<patch_t>()
    as *mut patch_t; 10];
static mut tallpercent: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut shortnum: [*mut patch_t; 10] = [::core::ptr::null::<patch_t>()
    as *mut patch_t; 10];
static mut keys: [*mut patch_t; 6] = [::core::ptr::null::<patch_t>() as *mut patch_t; 6];
static mut faces: [*mut patch_t; 42] = [::core::ptr::null::<patch_t>()
    as *mut patch_t; 42];
static mut faceback: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut armsbg: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut arms: [[*mut patch_t; 2]; 6] = [[::core::ptr::null::<patch_t>()
    as *mut patch_t; 2]; 6];
static mut w_ready: st_number_t = st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
};
static mut w_frags: st_number_t = st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
};
static mut w_health: st_percent_t = st_percent_t {
    n: st_number_t {
        x: 0,
        y: 0,
        width: 0,
        oldnum: 0,
        num: ::core::ptr::null::<i32>() as *mut i32,
        on: ::core::ptr::null::<bool>() as *mut bool,
        p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
        data: 0,
    },
    p: ::core::ptr::null::<patch_t>() as *mut patch_t,
};
static mut w_armsbg: st_binicon_t = st_binicon_t {
    x: 0,
    y: 0,
    oldval: false,
    val: ::core::ptr::null::<bool>() as *mut bool,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<patch_t>() as *mut patch_t,
    data: 0,
};
static mut w_arms_owned: [i32; 6] = [0; 6];
static mut w_arms: [st_multicon_t; 6] = [st_multicon_t {
    x: 0,
    y: 0,
    oldinum: 0,
    inum: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
}; 6];
static mut w_faces: st_multicon_t = st_multicon_t {
    x: 0,
    y: 0,
    oldinum: 0,
    inum: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
};
static mut w_keyboxes: [st_multicon_t; 3] = [st_multicon_t {
    x: 0,
    y: 0,
    oldinum: 0,
    inum: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
}; 3];
static mut w_armor: st_percent_t = st_percent_t {
    n: st_number_t {
        x: 0,
        y: 0,
        width: 0,
        oldnum: 0,
        num: ::core::ptr::null::<i32>() as *mut i32,
        on: ::core::ptr::null::<bool>() as *mut bool,
        p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
        data: 0,
    },
    p: ::core::ptr::null::<patch_t>() as *mut patch_t,
};
static mut w_ammo: [st_number_t; 4] = [st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
}; 4];
static mut w_maxammo: [st_number_t; 4] = [st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: ::core::ptr::null::<i32>() as *mut i32,
    on: ::core::ptr::null::<bool>() as *mut bool,
    p: ::core::ptr::null::<*mut patch_t>() as *mut *mut patch_t,
    data: 0,
}; 4];
static mut st_fragscount: i32 = 0;
static mut st_oldhealth: i32 = -(1 as i32);
static mut oldweaponsowned: [bool; 9] = [false; 9];
static mut st_facecount: i32 = 0 as i32;
static mut st_faceindex: i32 = 0 as i32;
static mut keyboxes: [i32; 3] = [0; 3];
static mut st_randomnumber: i32 = 0;
#[no_mangle]
pub static mut cheat_mus: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_god: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_ammo: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_ammonokey: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_noclip: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_commercial_noclip: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_powerup: [cheatseq_t; 7] = [cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
}; 7];
#[no_mangle]
pub static mut cheat_choppers: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_clev: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub static mut cheat_mypos: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
#[no_mangle]
pub unsafe extern "C" fn ST_refreshBackground() {
    if st_statusbaron {
        V_UseBuffer(st_backing_screen);
        V_DrawPatch(ST_X, 0 as i32, sbar);
        if netgame {
            V_DrawPatch(ST_FX, 0 as i32, faceback);
        }
        V_RestoreBuffer();
        V_CopyRect(
            ST_X,
            0 as i32,
            st_backing_screen,
            ST_WIDTH,
            ST_HEIGHT,
            ST_X,
            ST_Y,
        );
    }
}
pub unsafe fn ST_Responder(mut ev: *mut event_t) -> bool {
    let mut i: i32 = 0;
    if (*ev).type_0 as u32
        == ev_keyup as i32 as u32
        && (*ev).data1 as u32 & 0xffff0000 as u32
            == AM_MSGHEADER as u32
    {
        match (*ev).data1 {
            AM_MSGENTERED => {
                st_gamestate = AutomapState;
                st_firsttime = true;
            }
            AM_MSGEXITED => {
                st_gamestate = FirstPersonState;
            }
            _ => {}
        }
    } else if (*ev).type_0 as u32
        == ev_keydown as i32 as u32
    {
        if !netgame
            && gameskill as i32 != sk_nightmare as i32
        {
            if cht_CheckCheat(&raw mut cheat_god, (*ev).data2 as ::core::ffi::c_char)
                != 0
            {
                (*plyr).cheats ^= CF_GODMODE as i32;
                if (*plyr).cheats & CF_GODMODE as i32 != 0 {
                    if !(*plyr).mo.is_null() {
                        (*(*plyr).mo).health = 100 as i32;
                    }
                    (*plyr).health = deh_god_mode_health;
                    (*plyr).message = b"Degreelessness Mode On\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                } else {
                    (*plyr).message = b"Degreelessness Mode Off\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                }
            } else if cht_CheckCheat(
                &raw mut cheat_ammonokey,
                (*ev).data2 as ::core::ffi::c_char,
            ) != 0
            {
                (*plyr).armorpoints = deh_idfa_armor;
                (*plyr).armortype = deh_idfa_armor_class;
                i = 0 as i32;
                while i < NUMWEAPONS as i32 {
                    (*plyr).weaponowned[i as usize] = true;
                    i += 1;
                }
                i = 0 as i32;
                while i < NUMAMMO as i32 {
                    (*plyr).ammo[i as usize] = (*plyr).maxammo[i as usize];
                    i += 1;
                }
                (*plyr).message = b"Ammo (no keys) Added\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            } else if cht_CheckCheat(
                &raw mut cheat_ammo,
                (*ev).data2 as ::core::ffi::c_char,
            ) != 0
            {
                (*plyr).armorpoints = deh_idkfa_armor;
                (*plyr).armortype = deh_idkfa_armor_class;
                i = 0 as i32;
                while i < NUMWEAPONS as i32 {
                    (*plyr).weaponowned[i as usize] = true;
                    i += 1;
                }
                i = 0 as i32;
                while i < NUMAMMO as i32 {
                    (*plyr).ammo[i as usize] = (*plyr).maxammo[i as usize];
                    i += 1;
                }
                i = 0 as i32;
                while i < NUMCARDS as i32 {
                    (*plyr).cards[i as usize] = true;
                    i += 1;
                }
                (*plyr).message = b"Very Happy Ammo Added\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            } else if cht_CheckCheat(
                &raw mut cheat_mus,
                (*ev).data2 as ::core::ffi::c_char,
            ) != 0
            {
                let mut buf: [::core::ffi::c_char; 3] = [0; 3];
                let mut musnum: i32 = 0;
                (*plyr).message = b"Music Change\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                cht_GetParam(
                    &raw mut cheat_mus,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
                if gamemode as u32
                    == commercial as i32 as u32
                    || (gameversion as u32)
                        < exe_ultimate as i32 as u32
                {
                    musnum = mus_runnin as i32
                        + (buf[0 as i32 as usize] as i32
                            - '0' as i32) * 10 as i32
                        + buf[1 as i32 as usize] as i32
                        - '0' as i32 - 1 as i32;
                    if (buf[0 as i32 as usize] as i32
                        - '0' as i32) * 10 as i32
                        + buf[1 as i32 as usize] as i32
                        - '0' as i32 > 35 as i32
                    {
                        (*plyr).message = b"IMPOSSIBLE SELECTION\0" as *const u8
                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                    } else {
                        S_ChangeMusic(musnum, 1 as i32);
                    }
                } else {
                    musnum = mus_e1m1 as i32
                        + (buf[0 as i32 as usize] as i32
                            - '1' as i32) * 9 as i32
                        + (buf[1 as i32 as usize] as i32
                            - '1' as i32);
                    if (buf[0 as i32 as usize] as i32
                        - '1' as i32) * 9 as i32
                        + buf[1 as i32 as usize] as i32
                        - '1' as i32 > 31 as i32
                    {
                        (*plyr).message = b"IMPOSSIBLE SELECTION\0" as *const u8
                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                    } else {
                        S_ChangeMusic(musnum, 1 as i32);
                    }
                }
            } else if (if gamemission as u32
                == pack_chex as i32 as u32
            {
                doom as i32 as u32
            } else {
                (if gamemission as u32
                    == pack_hacx as i32 as u32
                {
                    doom2 as i32 as u32
                } else {
                    gamemission as u32
                })
            }) == doom as i32 as u32
                && cht_CheckCheat(
                    &raw mut cheat_noclip,
                    (*ev).data2 as ::core::ffi::c_char,
                ) != 0
                || (if gamemission as u32
                    == pack_chex as i32 as u32
                {
                    doom as i32 as u32
                } else {
                    (if gamemission as u32
                        == pack_hacx as i32 as u32
                    {
                        doom2 as i32 as u32
                    } else {
                        gamemission as u32
                    })
                }) != doom as i32 as u32
                    && cht_CheckCheat(
                        &raw mut cheat_commercial_noclip,
                        (*ev).data2 as ::core::ffi::c_char,
                    ) != 0
            {
                (*plyr).cheats ^= CF_NOCLIP as i32;
                if (*plyr).cheats & CF_NOCLIP as i32 != 0 {
                    (*plyr).message = b"No Clipping Mode ON\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                } else {
                    (*plyr).message = b"No Clipping Mode OFF\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                }
            }
            i = 0 as i32;
            while i < 6 as i32 {
                if cht_CheckCheat(
                    (&raw mut cheat_powerup as *mut cheatseq_t).offset(i as isize)
                        as *mut cheatseq_t,
                    (*ev).data2 as ::core::ffi::c_char,
                ) != 0
                {
                    if (*plyr).powers[i as usize] == 0 {
                        P_GivePower(plyr, i);
                    } else if i != pw_strength as i32 {
                        (*plyr).powers[i as usize] = 1 as i32;
                    } else {
                        (*plyr).powers[i as usize] = 0 as i32;
                    }
                    (*plyr).message = b"Power-up Toggled\0" as *const u8
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                }
                i += 1;
            }
            if cht_CheckCheat(
                (&raw mut cheat_powerup as *mut cheatseq_t)
                    .offset(6 as i32 as isize) as *mut cheatseq_t,
                (*ev).data2 as ::core::ffi::c_char,
            ) != 0
            {
                (*plyr).message = b"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else if cht_CheckCheat(
                &raw mut cheat_choppers,
                (*ev).data2 as ::core::ffi::c_char,
            ) != 0
            {
                (*plyr).weaponowned[wp_chainsaw as i32 as usize] = true;
                (*plyr).powers[pw_invulnerability as i32 as usize] = true_0;
                (*plyr).message = b"... doesn't suck - GM\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            } else if cht_CheckCheat(
                &raw mut cheat_mypos,
                (*ev).data2 as ::core::ffi::c_char,
            ) != 0
            {
                static mut buf_0: [::core::ffi::c_char; 52] = [0; 52];
                M_snprintf(
                    &raw mut buf_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 52]>() as size_t,
                    b"ang=0x%x;x,y=(0x%x,0x%x)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*players[consoleplayer as usize].mo).angle,
                    (*players[consoleplayer as usize].mo).x,
                    (*players[consoleplayer as usize].mo).y,
                );
                (*plyr).message = &raw mut buf_0 as *mut ::core::ffi::c_char;
            }
        }
        if !netgame
            && cht_CheckCheat(&raw mut cheat_clev, (*ev).data2 as ::core::ffi::c_char)
                != 0
        {
            let mut buf_1: [::core::ffi::c_char; 3] = [0; 3];
            let mut epsd: i32 = 0;
            let mut map: i32 = 0;
            cht_GetParam(
                &raw mut cheat_clev,
                &raw mut buf_1 as *mut ::core::ffi::c_char,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                epsd = 1 as i32;
                map = (buf_1[0 as i32 as usize] as i32
                    - '0' as i32) * 10 as i32
                    + buf_1[1 as i32 as usize] as i32
                    - '0' as i32;
            } else {
                epsd = buf_1[0 as i32 as usize] as i32
                    - '0' as i32;
                map = buf_1[1 as i32 as usize] as i32
                    - '0' as i32;
            }
            if gameversion as u32
                == exe_chex as i32 as u32
            {
                epsd = 1 as i32;
            }
            if epsd < 1 as i32 {
                return false;
            }
            if map < 1 as i32 {
                return false;
            }
            if gamemode as u32
                == retail as i32 as u32
                && (epsd > 4 as i32 || map > 9 as i32)
            {
                return false;
            }
            if gamemode as u32
                == registered as i32 as u32
                && (epsd > 3 as i32 || map > 9 as i32)
            {
                return false;
            }
            if gamemode as u32
                == shareware as i32 as u32
                && (epsd > 1 as i32 || map > 9 as i32)
            {
                return false;
            }
            if gamemode as u32
                == commercial as i32 as u32
                && (epsd > 1 as i32 || map > 40 as i32)
            {
                return false;
            }
            (*plyr).message = b"Changing Level...\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            G_DeferedInitNew(gameskill, epsd, map);
        }
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn ST_calcPainOffset() -> i32 {
    let mut health: i32 = 0;
    static mut lastcalc: i32 = 0;
    static mut oldhealth: i32 = -(1 as i32);
    health = if (*plyr).health > 100 as i32 {
        100 as i32
    } else {
        (*plyr).health
    };
    if health != oldhealth {
        lastcalc = ST_FACESTRIDE
            * ((100 as i32 - health) * ST_NUMPAINFACES
                / 101 as i32);
        oldhealth = health;
    }
    return lastcalc;
}
#[no_mangle]
pub unsafe extern "C" fn ST_updateFaceWidget() {
    let mut i: i32 = 0;
    let mut badguyangle: angle_t = 0;
    let mut diffang: angle_t = 0;
    static mut lastattackdown: i32 = -(1 as i32);
    static mut priority: i32 = 0 as i32;
    let mut doevilgrin: bool = false;
    if priority < 10 as i32 {
        if (*plyr).health == 0 {
            priority = 9 as i32;
            st_faceindex = ST_DEADFACE;
            st_facecount = 1 as i32;
        }
    }
    if priority < 9 as i32 {
        if (*plyr).bonuscount != 0 {
            doevilgrin = false;
            i = 0 as i32;
            while i < NUMWEAPONS as i32 {
                if oldweaponsowned[i as usize] != (*plyr).weaponowned[i as usize] {
                    doevilgrin = true;
                    oldweaponsowned[i as usize] = (*plyr).weaponowned[i as usize];
                }
                i += 1;
            }
            if doevilgrin {
                priority = 8 as i32;
                st_facecount = ST_EVILGRINCOUNT;
                st_faceindex = ST_calcPainOffset() + ST_EVILGRINOFFSET;
            }
        }
    }
    if priority < 8 as i32 {
        if (*plyr).damagecount != 0 && !(*plyr).attacker.is_null()
            && (*plyr).attacker != (*plyr).mo
        {
            priority = 7 as i32;
            if (*plyr).health - st_oldhealth > ST_MUCHPAIN {
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
            } else {
                badguyangle = R_PointToAngle2(
                    (*(*plyr).mo).x,
                    (*(*plyr).mo).y,
                    (*(*plyr).attacker).x,
                    (*(*plyr).attacker).y,
                );
                if badguyangle > (*(*plyr).mo).angle {
                    diffang = badguyangle.wrapping_sub((*(*plyr).mo).angle);
                    i = (diffang > ANG180) as i32;
                } else {
                    diffang = (*(*plyr).mo).angle.wrapping_sub(badguyangle);
                    i = (diffang <= ANG180) as i32;
                }
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset();
                if diffang < ANG45 as angle_t {
                    st_faceindex += ST_RAMPAGEOFFSET;
                } else if i != 0 {
                    st_faceindex += ST_TURNOFFSET;
                } else {
                    st_faceindex += ST_TURNOFFSET + 1 as i32;
                }
            }
        }
    }
    if priority < 7 as i32 {
        if (*plyr).damagecount != 0 {
            if (*plyr).health - st_oldhealth > ST_MUCHPAIN {
                priority = 7 as i32;
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
            } else {
                priority = 6 as i32;
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
            }
        }
    }
    if priority < 6 as i32 {
        if (*plyr).attackdown != 0 {
            if lastattackdown == -(1 as i32) {
                lastattackdown = ST_RAMPAGEDELAY;
            } else {
                lastattackdown -= 1;
                if lastattackdown == 0 {
                    priority = 5 as i32;
                    st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
                    st_facecount = 1 as i32;
                    lastattackdown = 1 as i32;
                }
            }
        } else {
            lastattackdown = -(1 as i32);
        }
    }
    if priority < 5 as i32 {
        if (*plyr).cheats & CF_GODMODE as i32 != 0
            || (*plyr).powers[pw_invulnerability as i32 as usize] != 0
        {
            priority = 4 as i32;
            st_faceindex = ST_GODFACE;
            st_facecount = 1 as i32;
        }
    }
    if st_facecount == 0 {
        st_faceindex = ST_calcPainOffset() + st_randomnumber % 3 as i32;
        st_facecount = ST_STRAIGHTFACECOUNT;
        priority = 0 as i32;
    }
    st_facecount -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn ST_updateWidgets() {
    static mut largeammo: i32 = 1994 as i32;
    let mut i: i32 = 0;
    if weaponinfo[(*plyr).readyweapon as usize].ammo as u32
        == am_noammo as i32 as u32
    {
        w_ready.num = &raw mut largeammo;
    } else {
        w_ready.num = (&raw mut (*plyr).ammo as *mut i32)
            .offset(
                (*(&raw mut weaponinfo as *mut weaponinfo_t)
                    .offset((*plyr).readyweapon as isize))
                    .ammo as isize,
            ) as *mut i32;
    }
    w_ready.data = (*plyr).readyweapon as i32;
    i = 0 as i32;
    while i < 6 as i32 {
        w_arms_owned[i as usize] = (*plyr)
            .weaponowned[(i + 1 as i32) as usize] as i32;
        i += 1;
    }
    i = 0 as i32;
    while i < 3 as i32 {
        keyboxes[i as usize] = if (*plyr).cards[i as usize] {
            i
        } else {
            -(1 as i32)
        };
        if (*plyr).cards[(i + 3 as i32) as usize] {
            keyboxes[i as usize] = i + 3 as i32;
        }
        i += 1;
    }
    ST_updateFaceWidget();
    st_notdeathmatch = deathmatch == 0;
    st_armson = st_statusbaron && deathmatch == 0;
    st_fragson = deathmatch != 0 && st_statusbaron;
    st_fragscount = 0 as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if i != consoleplayer {
            st_fragscount += (*plyr).frags[i as usize];
        } else {
            st_fragscount -= (*plyr).frags[i as usize];
        }
        i += 1;
    }
    st_msgcounter -= 1;
    if st_msgcounter == 0 {
        st_chat = st_oldchat;
    }
}
pub unsafe fn ST_Ticker() {
    st_clock = st_clock.wrapping_add(1);
    st_randomnumber = M_Random();
    ST_updateWidgets();
    st_oldhealth = (*plyr).health;
}
static mut st_palette: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn ST_doPaletteStuff() {
    let mut palette: i32 = 0;
    let mut pal: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut cnt: i32 = 0;
    let mut bzc: i32 = 0;
    cnt = (*plyr).damagecount;
    if (*plyr).powers[pw_strength as i32 as usize] != 0 {
        bzc = 12 as i32
            - ((*plyr).powers[pw_strength as i32 as usize]
                >> 6 as i32);
        if bzc > cnt {
            cnt = bzc;
        }
    }
    if cnt != 0 {
        palette = cnt + 7 as i32 >> 3 as i32;
        if palette >= NUMREDPALS {
            palette = NUMREDPALS - 1 as i32;
        }
        palette += STARTREDPALS;
    } else if (*plyr).bonuscount != 0 {
        palette = (*plyr).bonuscount + 7 as i32
            >> 3 as i32;
        if palette >= NUMBONUSPALS {
            palette = NUMBONUSPALS - 1 as i32;
        }
        palette += STARTBONUSPALS;
    } else if (*plyr).powers[pw_ironfeet as i32 as usize]
        > 4 as i32 * 32 as i32
        || (*plyr).powers[pw_ironfeet as i32 as usize]
            & 8 as i32 != 0
    {
        palette = RADIATIONPAL;
    } else {
        palette = 0 as i32;
    }
    if gameversion as u32
        == exe_chex as i32 as u32
        && palette >= STARTREDPALS && palette < STARTREDPALS + NUMREDPALS
    {
        palette = RADIATIONPAL;
    }
    if palette != st_palette {
        st_palette = palette;
        pal = (W_CacheLumpNum(lu_palette, PU_CACHE as i32) as *mut byte)
            .offset((palette * 768 as i32) as isize);
        I_SetPalette(pal);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ST_drawWidgets(mut refresh: bool) {
    let mut i: i32 = 0;
    st_armson = st_statusbaron && deathmatch == 0;
    st_fragson = deathmatch != 0 && st_statusbaron;
    STlib_updateNum(&raw mut w_ready, refresh);
    i = 0 as i32;
    while i < 4 as i32 {
        STlib_updateNum(
            (&raw mut w_ammo as *mut st_number_t).offset(i as isize) as *mut st_number_t,
            refresh,
        );
        STlib_updateNum(
            (&raw mut w_maxammo as *mut st_number_t).offset(i as isize)
                as *mut st_number_t,
            refresh,
        );
        i += 1;
    }
    STlib_updatePercent(&raw mut w_health, refresh as i32);
    STlib_updatePercent(&raw mut w_armor, refresh as i32);
    STlib_updateBinIcon(&raw mut w_armsbg, refresh);
    i = 0 as i32;
    while i < 6 as i32 {
        STlib_updateMultIcon(
            (&raw mut w_arms as *mut st_multicon_t).offset(i as isize)
                as *mut st_multicon_t,
            refresh,
        );
        i += 1;
    }
    STlib_updateMultIcon(&raw mut w_faces, refresh);
    i = 0 as i32;
    while i < 3 as i32 {
        STlib_updateMultIcon(
            (&raw mut w_keyboxes as *mut st_multicon_t).offset(i as isize)
                as *mut st_multicon_t,
            refresh,
        );
        i += 1;
    }
    STlib_updateNum(&raw mut w_frags, refresh);
}
#[no_mangle]
pub unsafe extern "C" fn ST_doRefresh() {
    st_firsttime = false;
    ST_refreshBackground();
    ST_drawWidgets(true);
}
#[no_mangle]
pub unsafe extern "C" fn ST_diffDraw() {
    ST_drawWidgets(false);
}
pub unsafe fn ST_Drawer(mut fullscreen: bool, mut refresh: bool) {
    st_statusbaron = !fullscreen || automapactive;
    st_firsttime = st_firsttime || refresh;
    ST_doPaletteStuff();
    if st_firsttime {
        ST_doRefresh();
    } else {
        ST_diffDraw();
    };
}
unsafe fn ST_loadUnloadGraphics(mut callback: load_callback_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut facenum: i32 = 0;
    let mut namebuf: [::core::ffi::c_char; 9] = [0; 9];
    i = 0 as i32;
    while i < 10 as i32 {
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STTNUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut tallnum as *mut *mut patch_t).offset(i as isize)
                as *mut *mut patch_t,
        );
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STYSNUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut shortnum as *mut *mut patch_t).offset(i as isize)
                as *mut *mut patch_t,
        );
        i += 1;
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"STTPRCNT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut tallpercent,
    );
    i = 0 as i32;
    while i < NUMCARDS as i32 {
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STKEYS%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut keys as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        i += 1;
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"STARMS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut armsbg,
    );
    i = 0 as i32;
    while i < 6 as i32 {
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STGNUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i + 2 as i32,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut *(&raw mut arms as *mut [*mut patch_t; 2]).offset(i as isize)
                as *mut *mut patch_t)
                .offset(0 as i32 as isize) as *mut *mut patch_t,
        );
        arms[i as usize][1 as i32 as usize] = shortnum[(i
            + 2 as i32) as usize];
        i += 1;
    }
    snprintf(
        &raw mut namebuf as *mut ::core::ffi::c_char,
        9 as size_t,
        b"STFB%d\0" as *const u8 as *const ::core::ffi::c_char,
        consoleplayer,
    );
    callback
        .expect(
            "non-null function pointer",
        )(&raw mut namebuf as *mut ::core::ffi::c_char, &raw mut faceback);
    callback
        .expect(
            "non-null function pointer",
        )(
        b"STBAR\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut sbar,
    );
    facenum = 0 as i32;
    i = 0 as i32;
    while i < ST_NUMPAINFACES {
        j = 0 as i32;
        while j < ST_NUMSTRAIGHTFACES {
            snprintf(
                &raw mut namebuf as *mut ::core::ffi::c_char,
                9 as size_t,
                b"STFST%d%d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
                j,
            );
            callback
                .expect(
                    "non-null function pointer",
                )(
                &raw mut namebuf as *mut ::core::ffi::c_char,
                (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
                    as *mut *mut patch_t,
            );
            facenum += 1;
            j += 1;
        }
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STFTR%d0\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
                as *mut *mut patch_t,
        );
        facenum += 1;
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STFTL%d0\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
                as *mut *mut patch_t,
        );
        facenum += 1;
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STFOUCH%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
                as *mut *mut patch_t,
        );
        facenum += 1;
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STFEVL%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
                as *mut *mut patch_t,
        );
        facenum += 1;
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STFKILL%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
                as *mut *mut patch_t,
        );
        facenum += 1;
        i += 1;
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"STFGOD0\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
            as *mut *mut patch_t,
    );
    facenum += 1;
    callback
        .expect(
            "non-null function pointer",
        )(
        b"STFDEAD0\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (&raw mut faces as *mut *mut patch_t).offset(facenum as isize)
            as *mut *mut patch_t,
    );
    facenum += 1;
}
unsafe fn ST_loadCallback(
    mut lumpname: *mut ::core::ffi::c_char,
    mut variable: *mut *mut patch_t,
) {
    *variable = W_CacheLumpName(
        &wad_name8_to_string(lumpname),
        PU_STATIC as i32,
    ) as *mut patch_t;
}
#[no_mangle]
pub unsafe extern "C" fn ST_loadGraphics() {
    ST_loadUnloadGraphics(
        Some(
            ST_loadCallback
                as unsafe fn(
                    *mut ::core::ffi::c_char,
                    *mut *mut patch_t,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ST_loadData() {
    lu_palette = W_GetNumForName("PLAYPAL",
    );
    ST_loadGraphics();
}
unsafe fn ST_unloadCallback(
    mut lumpname: *mut ::core::ffi::c_char,
    mut variable: *mut *mut patch_t,
) {
    W_ReleaseLumpName(&wad_name8_to_string(lumpname));
    *variable = ::core::ptr::null_mut::<patch_t>();
}
#[no_mangle]
pub unsafe extern "C" fn ST_unloadGraphics() {
    ST_loadUnloadGraphics(
        Some(
            ST_unloadCallback
                as unsafe fn(
                    *mut ::core::ffi::c_char,
                    *mut *mut patch_t,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ST_unloadData() {
    ST_unloadGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn ST_initData() {
    let mut i: i32 = 0;
    st_firsttime = true;
    plyr = (&raw mut players as *mut player_t).offset(consoleplayer as isize)
        as *mut player_t;
    st_clock = 0 as u32;
    st_chatstate = StartChatState;
    st_gamestate = FirstPersonState;
    st_statusbaron = true;
    st_chat = false;
    st_oldchat = st_chat;
    st_cursoron = false;
    st_faceindex = 0 as i32;
    st_palette = -(1 as i32);
    st_oldhealth = -(1 as i32);
    i = 0 as i32;
    while i < NUMWEAPONS as i32 {
        oldweaponsowned[i as usize] = (*plyr).weaponowned[i as usize];
        i += 1;
    }
    i = 0 as i32;
    while i < 3 as i32 {
        keyboxes[i as usize] = -(1 as i32);
        i += 1;
    }
    STlib_init();
}
#[no_mangle]
pub unsafe extern "C" fn ST_createWidgets() {
    let mut i: i32 = 0;
    STlib_initNum(
        &raw mut w_ready,
        ST_AMMOX,
        ST_AMMOY,
        &raw mut tallnum as *mut *mut patch_t,
        (&raw mut (*plyr).ammo as *mut i32)
            .offset(
                (*(&raw mut weaponinfo as *mut weaponinfo_t)
                    .offset((*plyr).readyweapon as isize))
                    .ammo as isize,
            ) as *mut i32,
        &raw mut st_statusbaron,
        ST_AMMOWIDTH,
    );
    w_ready.data = (*plyr).readyweapon as i32;
    STlib_initPercent(
        &raw mut w_health,
        ST_HEALTHX,
        ST_HEALTHY,
        &raw mut tallnum as *mut *mut patch_t,
        &raw mut (*plyr).health,
        &raw mut st_statusbaron,
        tallpercent,
    );
    STlib_initBinIcon(
        &raw mut w_armsbg,
        ST_ARMSBGX,
        ST_ARMSBGY,
        armsbg,
        &raw mut st_notdeathmatch,
        &raw mut st_statusbaron,
    );
    i = 0 as i32;
    while i < 6 as i32 {
        STlib_initMultIcon(
            (&raw mut w_arms as *mut st_multicon_t).offset(i as isize)
                as *mut st_multicon_t,
            ST_ARMSX + i % 3 as i32 * ST_ARMSXSPACE,
            ST_ARMSY + i / 3 as i32 * ST_ARMSYSPACE,
            &raw mut *(&raw mut arms as *mut [*mut patch_t; 2]).offset(i as isize)
                as *mut *mut patch_t,
            (&raw mut w_arms_owned as *mut i32)
                .offset(i as isize),
            &raw mut st_armson,
        );
        i += 1;
    }
    STlib_initNum(
        &raw mut w_frags,
        ST_FRAGSX,
        ST_FRAGSY,
        &raw mut tallnum as *mut *mut patch_t,
        &raw mut st_fragscount,
        &raw mut st_fragson,
        ST_FRAGSWIDTH,
    );
    STlib_initMultIcon(
        &raw mut w_faces,
        ST_FACESX,
        ST_FACESY,
        &raw mut faces as *mut *mut patch_t,
        &raw mut st_faceindex,
        &raw mut st_statusbaron,
    );
    STlib_initPercent(
        &raw mut w_armor,
        ST_ARMORX,
        ST_ARMORY,
        &raw mut tallnum as *mut *mut patch_t,
        &raw mut (*plyr).armorpoints,
        &raw mut st_statusbaron,
        tallpercent,
    );
    STlib_initMultIcon(
        (&raw mut w_keyboxes as *mut st_multicon_t)
            .offset(0 as i32 as isize) as *mut st_multicon_t,
        ST_KEY0X,
        ST_KEY0Y,
        &raw mut keys as *mut *mut patch_t,
        (&raw mut keyboxes as *mut i32)
            .offset(0 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
    );
    STlib_initMultIcon(
        (&raw mut w_keyboxes as *mut st_multicon_t)
            .offset(1 as i32 as isize) as *mut st_multicon_t,
        ST_KEY1X,
        ST_KEY1Y,
        &raw mut keys as *mut *mut patch_t,
        (&raw mut keyboxes as *mut i32)
            .offset(1 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
    );
    STlib_initMultIcon(
        (&raw mut w_keyboxes as *mut st_multicon_t)
            .offset(2 as i32 as isize) as *mut st_multicon_t,
        ST_KEY2X,
        ST_KEY2Y,
        &raw mut keys as *mut *mut patch_t,
        (&raw mut keyboxes as *mut i32)
            .offset(2 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
    );
    STlib_initNum(
        (&raw mut w_ammo as *mut st_number_t).offset(0 as i32 as isize)
            as *mut st_number_t,
        ST_AMMO0X,
        ST_AMMO0Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).ammo as *mut i32)
            .offset(0 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_AMMO0WIDTH,
    );
    STlib_initNum(
        (&raw mut w_ammo as *mut st_number_t).offset(1 as i32 as isize)
            as *mut st_number_t,
        ST_AMMO1X,
        ST_AMMO1Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).ammo as *mut i32)
            .offset(1 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_AMMO1WIDTH,
    );
    STlib_initNum(
        (&raw mut w_ammo as *mut st_number_t).offset(2 as i32 as isize)
            as *mut st_number_t,
        ST_AMMO2X,
        ST_AMMO2Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).ammo as *mut i32)
            .offset(2 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_AMMO2WIDTH,
    );
    STlib_initNum(
        (&raw mut w_ammo as *mut st_number_t).offset(3 as i32 as isize)
            as *mut st_number_t,
        ST_AMMO3X,
        ST_AMMO3Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).ammo as *mut i32)
            .offset(3 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_AMMO3WIDTH,
    );
    STlib_initNum(
        (&raw mut w_maxammo as *mut st_number_t).offset(0 as i32 as isize)
            as *mut st_number_t,
        ST_MAXAMMO0X,
        ST_MAXAMMO0Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).maxammo as *mut i32)
            .offset(0 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_MAXAMMO0WIDTH,
    );
    STlib_initNum(
        (&raw mut w_maxammo as *mut st_number_t).offset(1 as i32 as isize)
            as *mut st_number_t,
        ST_MAXAMMO1X,
        ST_MAXAMMO1Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).maxammo as *mut i32)
            .offset(1 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_MAXAMMO1WIDTH,
    );
    STlib_initNum(
        (&raw mut w_maxammo as *mut st_number_t).offset(2 as i32 as isize)
            as *mut st_number_t,
        ST_MAXAMMO2X,
        ST_MAXAMMO2Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).maxammo as *mut i32)
            .offset(2 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_MAXAMMO2WIDTH,
    );
    STlib_initNum(
        (&raw mut w_maxammo as *mut st_number_t).offset(3 as i32 as isize)
            as *mut st_number_t,
        ST_MAXAMMO3X,
        ST_MAXAMMO3Y,
        &raw mut shortnum as *mut *mut patch_t,
        (&raw mut (*plyr).maxammo as *mut i32)
            .offset(3 as i32 as isize) as *mut i32,
        &raw mut st_statusbaron,
        ST_MAXAMMO3WIDTH,
    );
}
static mut st_stopped: bool = true;
pub unsafe fn ST_Start() {
    if !st_stopped {
        ST_Stop();
    }
    ST_initData();
    ST_createWidgets();
    st_stopped = false;
}
#[no_mangle]
pub unsafe extern "C" fn ST_Stop() {
    if st_stopped {
        return;
    }
    I_SetPalette(
        W_CacheLumpNum(lu_palette, PU_CACHE as i32) as *mut byte,
    );
    st_stopped = true;
}
pub unsafe fn ST_Init() {
    ST_loadData();
    st_backing_screen = Z_Malloc(
        ST_WIDTH * ST_HEIGHT,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut byte;
}
unsafe extern "C" fn run_static_initializers() {
    cheat_clev = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idclev\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 2 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_mypos = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idmypos\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_choppers = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idchoppers\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_powerup = [
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbeholdv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbeholds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbeholdi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbeholdr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbeholda\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbeholdl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
        cheatseq_t {
            sequence: ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"idbehold\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                .wrapping_sub(1 as size_t),
            parameter_chars: 0 as i32,
            chars_read: 0 as size_t,
            param_chars_read: 0 as i32,
            parameter_buf: ::core::mem::transmute::<
                [u8; 5],
                [::core::ffi::c_char; 5],
            >(*b"\0\0\0\0\0"),
        },
    ];
    cheat_commercial_noclip = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idclip\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_noclip = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idspispopd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_mus = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idmus\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 2 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_ammo = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idkfa\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_ammonokey = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"idfa\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
    cheat_god = cheatseq_t {
        sequence: ::core::mem::transmute::<
            [u8; 25],
            [::core::ffi::c_char; 25],
        >(*b"iddqd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<
            [u8; 5],
            [::core::ffi::c_char; 5],
        >(*b"\0\0\0\0\0"),
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
