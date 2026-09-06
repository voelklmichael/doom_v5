use crate::src::i_system::FILE;
use crate::src::d_event::event_t;
use crate::src::wi_stuff::{wbplayerstruct_t, wbstartstruct_t};
use crate::src::p_mobj::{mapthing_t, state_t, subsector_t};
use crate::src::d_player::{player_s, player_t, PST_LIVE, PST_DEAD, PST_REBORN};
use crate::src::p_mobj::{mobj_t, pspdef_t};
use crate::src::d_ticcmd::{ticcmd_t};
use crate::src::i_system::I_Error;
use crate::src::m_argv::{myargv, M_CheckParm, M_CheckParmWithArgs};
use crate::src::w_wad::{
    wad_name8_to_string, W_CacheLumpName, W_CheckNumForName, W_ReleaseLumpName,
};
use crate::src::d_loop::singletics;
use crate::src::d_loop::ticdup;
use crate::src::m_random::rndindex;
use crate::src::d_net::netcmds;
use crate::src::m_controls::key_right;
use crate::src::m_controls::key_left;
use crate::src::m_controls::key_up;
use crate::src::m_controls::key_down;
use crate::src::m_controls::key_strafeleft;
use crate::src::m_controls::key_straferight;
use crate::src::m_controls::key_fire;
use crate::src::m_controls::key_use;
use crate::src::m_controls::key_strafe;
use crate::src::m_controls::key_speed;
use crate::src::m_controls::key_pause;
use crate::src::m_controls::key_weapon1;
use crate::src::m_controls::key_weapon2;
use crate::src::m_controls::key_weapon3;
use crate::src::m_controls::key_weapon4;
use crate::src::m_controls::key_weapon5;
use crate::src::m_controls::key_weapon6;
use crate::src::m_controls::key_weapon7;
use crate::src::m_controls::key_weapon8;
use crate::src::m_controls::key_demo_quit;
use crate::src::m_controls::key_spy;
use crate::src::m_controls::key_prevweapon;
use crate::src::m_controls::key_nextweapon;
use crate::src::m_controls::mousebfire;
use crate::src::m_controls::mousebstrafe;
use crate::src::m_controls::mousebforward;
use crate::src::m_controls::mousebstrafeleft;
use crate::src::m_controls::mousebstraferight;
use crate::src::m_controls::mousebbackward;
use crate::src::m_controls::mousebuse;
use crate::src::m_controls::mousebprevweapon;
use crate::src::m_controls::mousebnextweapon;
use crate::src::m_controls::joybfire;
use crate::src::m_controls::joybstrafe;
use crate::src::m_controls::joybuse;
use crate::src::m_controls::joybspeed;
use crate::src::m_controls::joybstrafeleft;
use crate::src::m_controls::joybstraferight;
use crate::src::m_controls::joybprevweapon;
use crate::src::m_controls::joybnextweapon;
use crate::src::m_controls::dclick_use;
use crate::src::m_misc::M_TempFile;
use crate::src::m_menu::M_StartControlPanel;
use crate::src::m_random::M_ClearRandom;
use crate::src::p_setup::P_SetupLevel;
use crate::src::p_saveg::P_TempSaveGameFile;
use crate::src::p_saveg::P_ReadSaveGameHeader;
use crate::src::p_saveg::P_WriteSaveGameHeader;
use crate::src::p_saveg::P_ReadSaveGameEOF;
use crate::src::p_saveg::P_WriteSaveGameEOF;
use crate::src::p_saveg::P_ArchivePlayers;
use crate::src::p_saveg::P_UnArchivePlayers;
use crate::src::p_saveg::P_ArchiveWorld;
use crate::src::p_saveg::P_UnArchiveWorld;
use crate::src::p_saveg::P_ArchiveThinkers;
use crate::src::p_saveg::P_UnArchiveThinkers;
use crate::src::p_saveg::P_ArchiveSpecials;
use crate::src::p_saveg::P_UnArchiveSpecials;
use crate::src::p_saveg::save_stream;
use crate::src::p_saveg::savegame_error;
use crate::src::p_tick::P_Ticker;
use crate::src::d_main::D_PageTicker;
use crate::src::d_main::D_AdvanceDemo;
use crate::src::wi_stuff::WI_Ticker;
use crate::src::wi_stuff::WI_Start;
use crate::src::wi_stuff::WI_End;
use crate::src::hu_stuff::HU_Responder;
use crate::src::hu_stuff::HU_Ticker;
use crate::src::hu_stuff::HU_dequeueChatChar;
use crate::src::st_stuff::ST_Ticker;
use crate::src::am_map::AM_Responder;
use crate::src::am_map::AM_Ticker;
use crate::src::statdump::StatCopy;
use crate::src::p_inter::maxammo;
use crate::src::s_sound::S_PauseSound;
use crate::src::s_sound::S_ResumeSound;
use crate::src::f_finale::F_Responder;
use crate::src::f_finale::F_Ticker;
use crate::src::f_finale::F_StartFinale;
use crate::src::hu_stuff::player_names;
use crate::src::am_map::AM_Stop;
use crate::src::d_main::respawnparm;
use crate::src::d_main::wipegamestate;
use crate::src::i_system::I_Quit;
use crate::src::m_menu::mouseSensitivity;
use crate::src::m_misc::M_WriteFile;
use crate::src::p_setup::deathmatchstarts;
use crate::src::p_setup::deathmatch_p;
use crate::src::r_draw::R_FillBackScreen;
use crate::src::r_main::R_ExecuteSetViewSize;
use crate::src::r_main::setsizeneeded;
use crate::src::st_stuff::ST_Responder;
use crate::src::d_main::nomonsters;
use crate::src::d_main::fastparm;
use crate::src::p_map::P_CheckPosition;
use crate::src::p_saveg::P_SaveGameFile;
use crate::src::p_setup::playerstarts;
use crate::src::r_sky::skytexture;
use crate::src::tables::finetangent;
use crate::src::d_loop::gametic;
use crate::src::r_main::R_PointInSubsector;
use crate::src::info::mobjinfo;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::r_sky::skyflatnum;
use crate::src::doomstat::gamemission;
use crate::src::info::states;
use crate::src::am_map::automapactive;
use crate::src::m_misc::M_StringCopy;
use crate::src::m_random::P_Random;
use crate::src::m_misc::M_snprintf;
use crate::src::doomstat::gameversion;
use crate::src::p_tick::leveltime;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::p_mobj::P_SpawnPlayer;
use crate::src::v_video::V_ScreenShot;
use crate::src::z_zone::Z_CheckHeap;
use crate::src::r_data::R_FlatNumForName;
use crate::src::i_timer::I_GetTime;
use crate::src::r_data::R_TextureNumForName;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use crate::src::p_mobj::MF_SHADOW;
use crate::src::sounds::sfx_telept;
use crate::src::d_ticcmd::{BTS_PAUSE, BTS_SAVEGAME, BTS_SAVEMASK, BTS_SAVESHIFT, BT_ATTACK, BT_CHANGE, BT_SPECIAL, BT_SPECIALMASK, BT_USE, BT_WEAPONSHIFT};
use crate::src::d_player::pw_strength;
use libc::{memcpy, memset};
use libc::{atoi, strlen};
use libc::printf;
use crate::src::i_system::{fclose, fopen, ftell};
use crate::src::p_mobj::{MT_BRUISERSHOT, MT_HEADSHOT, MT_TFOG, MT_TROOPSHOT};
use crate::src::d_mode::{commercial, shareware};
use crate::src::d_mode::{exe_chex, exe_final2, exe_ultimate};
use crate::src::d_mode::{doom, doom2, pack_chex, pack_hacx};
use crate::src::d_mode::{sk_baby, sk_nightmare, skill_t};
use crate::src::d_event::{ev_joystick, ev_keydown, ev_mouse};
use crate::src::d_event::{GS_DEMOSCREEN, GS_FINALE, GS_INTERMISSION, GS_LEVEL, gamestate_t};
use crate::src::d_event::{ga_completed, ga_loadgame, ga_loadlevel, ga_newgame, ga_nothing, ga_playdemo, ga_savegame, ga_screenshot, ga_victory, ga_worlddone, gameaction_t};
use crate::src::d_player::{weapontype_t, wp_bfg, wp_chaingun, wp_chainsaw, wp_fist, wp_missile, wp_nochange, wp_pistol, wp_plasma, wp_shotgun, wp_supershotgun};
use crate::src::m_fixed::fixed_t;
use crate::src::doomdef::boolean;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::info::{S_SARG_PAIN2, S_SARG_RUN1};
use crate::src::d_player::{NUMAMMO, am_clip};
use crate::src::doomdef::NULL;
use crate::src::doomdef::true_0;
use crate::src::doomdef::false_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::doomdef::TICRATE;
use crate::src::m_fixed::FRACUNIT;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG45;
use crate::src::d_loop::BACKUPTICS;
use crate::src::m_fixed::FRACBITS;
use crate::src::game_state::game_state;

extern "C" {
    fn remove(__filename: *const ::core::ffi::c_char) -> i32;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub weapon: weapontype_t,
    pub weapon_num: weapontype_t,
}
pub const DEH_DEFAULT_INITIAL_HEALTH: i32 = 100;
pub const DEH_DEFAULT_INITIAL_BULLETS: i32 = 50;
pub const deh_initial_health: i32 = DEH_DEFAULT_INITIAL_HEALTH;
pub const deh_initial_bullets: i32 = DEH_DEFAULT_INITIAL_BULLETS;
pub const DOOM_191_VERSION: i32 = 111;
pub const SAVEGAMESIZE: i32 = 0x2c000;
#[no_mangle]
pub static mut oldgamestate: gamestate_t = GS_LEVEL;
pub static mut gameaction: gameaction_t = ga_nothing;
pub static mut gamestate: gamestate_t = GS_LEVEL;
pub static mut gameskill: skill_t = sk_baby;
pub static mut respawnmonsters: bool = false;
pub static mut gameepisode: i32 = 0;
pub static mut gamemap: i32 = 0;
pub static mut timelimit: i32 = 0;
pub static mut paused: bool = false;
#[no_mangle]
pub static mut sendpause: bool = false;
#[no_mangle]
pub static mut sendsave: bool = false;
pub static mut usergame: bool = false;
#[no_mangle]
pub static mut timingdemo: bool = false;
pub static mut nodrawers: bool = false;
#[no_mangle]
pub static mut starttime: i32 = 0;
pub static mut viewactive: bool = false;
pub static mut deathmatch: i32 = 0;
pub static mut netgame: bool = false;
pub static mut playeringame: [boolean; 4] = [0; 4];
pub static mut players: [player_t; 4] = [player_s {
    mo: ::core::ptr::null::<mobj_t>() as *mut mobj_t,
    playerstate: PST_LIVE,
    cmd: ticcmd_t {
        forwardmove: 0,
        sidemove: 0,
        angleturn: 0,
        chatchar: 0,
        buttons: 0,
        consistancy: 0,
        buttons2: 0,
        inventory: 0,
        lookfly: 0,
        arti: 0,
    },
    viewz: 0,
    viewheight: 0,
    deltaviewheight: 0,
    bob: 0,
    health: 0,
    armorpoints: 0,
    armortype: 0,
    powers: [0; 6],
    cards: [false; 6],
    backpack: false,
    frags: [0; 4],
    readyweapon: wp_fist,
    pendingweapon: wp_fist,
    weaponowned: [false; 9],
    ammo: [0; 4],
    maxammo: [0; 4],
    attackdown: 0,
    usedown: 0,
    cheats: 0,
    refire: 0,
    killcount: 0,
    itemcount: 0,
    secretcount: 0,
    message: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    damagecount: 0,
    bonuscount: 0,
    attacker: ::core::ptr::null::<mobj_t>() as *mut mobj_t,
    extralight: 0,
    fixedcolormap: 0,
    colormap: 0,
    psprites: [pspdef_t {
        state: ::core::ptr::null::<state_t>() as *mut state_t,
        tics: 0,
        sx: 0,
        sy: 0,
    }; 2],
    didsecret: false,
}; 4];
#[no_mangle]
pub static mut turbodetected: [boolean; 4] = [0; 4];
pub static mut consoleplayer: i32 = 0;
pub static mut displayplayer: i32 = 0;
#[no_mangle]
pub static mut levelstarttic: i32 = 0;
pub static mut totalsecret: i32 = 0;
pub static mut totalkills: i32 = 0;
pub static mut totalitems: i32 = 0;
#[no_mangle]
pub static mut demoname: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
pub static mut demorecording: bool = false;
#[no_mangle]
pub static mut longtics: bool = false;
pub static mut lowres_turn: bool = false;
pub static mut demoplayback: bool = false;
#[no_mangle]
pub static mut netdemo: bool = false;
#[no_mangle]
pub static mut demobuffer: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub static mut demo_p: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
#[no_mangle]
pub static mut demoend: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
pub static mut singledemo: bool = false;
pub static mut precache: bool = true;
pub static mut testcontrols: bool = false;
pub static mut testcontrols_mousespeed: i32 = 0;
pub static mut wminfo: wbstartstruct_t = wbstartstruct_t {
    epsd: 0,
    didsecret: false,
    last: 0,
    next: 0,
    maxkills: 0,
    maxitems: 0,
    maxsecret: 0,
    maxfrags: 0,
    partime: 0,
    pnum: 0,
    plyr: [wbplayerstruct_t {
        in_0: false,
        skills: 0,
        sitems: 0,
        ssecret: 0,
        stime: 0,
        frags: [0; 4],
        score: 0,
    }; 4],
};
#[no_mangle]
pub static mut consistancy: [[byte; 128]; 4] = [[0; 128]; 4];
pub const TURBOTHRESHOLD: i32 = 0x32;
pub static mut forwardmove: [fixed_t; 2] = [
    0x19 as i32,
    0x32 as i32,
];
pub static mut sidemove: [fixed_t; 2] = [
    0x18 as i32,
    0x28 as i32,
];
#[no_mangle]
pub static mut angleturn: [fixed_t; 3] = [
    640 as i32,
    1280 as i32,
    320 as i32,
];
static mut weapon_keys: [*mut i32; 8] = unsafe {
    [
        &raw const key_weapon1 as *mut i32,
        &raw const key_weapon2 as *mut i32,
        &raw const key_weapon3 as *mut i32,
        &raw const key_weapon4 as *mut i32,
        &raw const key_weapon5 as *mut i32,
        &raw const key_weapon6 as *mut i32,
        &raw const key_weapon7 as *mut i32,
        &raw const key_weapon8 as *mut i32,
    ]
};
static mut next_weapon: i32 = 0;
static mut weapon_order_table: [C2RustUnnamed_5; 9] = [
    C2RustUnnamed_5 {
        weapon: wp_fist,
        weapon_num: wp_fist,
    },
    C2RustUnnamed_5 {
        weapon: wp_chainsaw,
        weapon_num: wp_fist,
    },
    C2RustUnnamed_5 {
        weapon: wp_pistol,
        weapon_num: wp_pistol,
    },
    C2RustUnnamed_5 {
        weapon: wp_shotgun,
        weapon_num: wp_shotgun,
    },
    C2RustUnnamed_5 {
        weapon: wp_supershotgun,
        weapon_num: wp_shotgun,
    },
    C2RustUnnamed_5 {
        weapon: wp_chaingun,
        weapon_num: wp_chaingun,
    },
    C2RustUnnamed_5 {
        weapon: wp_missile,
        weapon_num: wp_missile,
    },
    C2RustUnnamed_5 {
        weapon: wp_plasma,
        weapon_num: wp_plasma,
    },
    C2RustUnnamed_5 {
        weapon: wp_bfg,
        weapon_num: wp_bfg,
    },
];
pub const SLOWTURNTICS: i32 = 6;
pub const NUMKEYS: i32 = 256;
pub const MAX_JOY_BUTTONS: i32 = 20;
static mut gamekeydown: [boolean; 256] = [0; 256];
static mut turnheld: i32 = 0;
static mut mousearray: [boolean; 9] = [0; 9];
static mut mousebuttons: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut mousex: i32 = 0;
#[no_mangle]
pub static mut mousey: i32 = 0;
static mut dclicktime: i32 = 0;
static mut dclickstate: boolean = 0;
static mut dclicks: i32 = 0;
static mut dclicktime2: i32 = 0;
static mut dclickstate2: boolean = 0;
static mut dclicks2: i32 = 0;
static mut joyxmove: i32 = 0;
static mut joyymove: i32 = 0;
static mut joystrafemove: i32 = 0;
static mut joyarray: [boolean; 21] = [0; 21];
static mut joybuttons: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
static mut savegameslot: i32 = 0;
static mut savedescription: [::core::ffi::c_char; 32] = [0; 32];
pub const BODYQUESIZE: i32 = 32;
#[no_mangle]
pub static mut bodyque: [*mut mobj_t; 32] = [::core::ptr::null::<mobj_t>()
    as *mut mobj_t; 32];
pub static mut bodyqueslot: i32 = 0;
pub static mut vanilla_savegame_limit: i32 = 1;
pub static mut vanilla_demo_limit: i32 = 1;
pub unsafe fn G_CmdChecksum(mut cmd: *mut ticcmd_t) -> i32 {
    let mut i: size_t = 0;
    let mut sum: i32 = 0 as i32;
    i = 0 as size_t;
    while i
        < (::core::mem::size_of::<ticcmd_t>() as usize)
            .wrapping_div(4 as usize)
            .wrapping_sub(1 as usize)
    {
        sum += *(cmd as *mut i32).offset(i as isize);
        i = i.wrapping_add(1);
    }
    return sum;
}
unsafe fn WeaponSelectable(mut weapon: weapontype_t) -> bool {
    if weapon as u32
        == wp_supershotgun as i32 as u32
        && (if gamemission as u32
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
    {
        return false;
    }
    if (weapon as u32
        == wp_plasma as i32 as u32
        || weapon as u32
            == wp_bfg as i32 as u32)
        && gamemission as u32
            == doom as i32 as u32
        && gamemode as u32
            == shareware as i32 as u32
    {
        return false;
    }
    if !players[consoleplayer as usize].weaponowned[weapon as usize] {
        return false;
    }
    if weapon as u32
        == wp_fist as i32 as u32
        && players[consoleplayer as usize]
            .weaponowned[wp_chainsaw as i32 as usize]
        && players[consoleplayer as usize]
            .powers[pw_strength as i32 as usize] == 0
    {
        return false;
    }
    return true;
}
unsafe fn G_NextWeapon(
    mut direction: i32,
) -> i32 {
    let mut weapon: weapontype_t = wp_fist;
    let mut start_i: i32 = 0;
    let mut i: i32 = 0;
    if players[consoleplayer as usize].pendingweapon as u32
        == wp_nochange as i32 as u32
    {
        weapon = players[consoleplayer as usize].readyweapon;
    } else {
        weapon = players[consoleplayer as usize].pendingweapon;
    }
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_5; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as usize)
    {
        if weapon_order_table[i as usize].weapon as u32
            == weapon as u32
        {
            break;
        }
        i += 1;
    }
    start_i = i;
    loop {
        i += direction;
        i = (i as usize)
            .wrapping_add(
                (::core::mem::size_of::<[C2RustUnnamed_5; 9]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as usize),
            )
            .wrapping_rem(
                (::core::mem::size_of::<[C2RustUnnamed_5; 9]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as usize),
            ) as i32;
        if !(i != start_i
            && !WeaponSelectable(weapon_order_table[i as usize].weapon))
        {
            break;
        }
    }
    return weapon_order_table[i as usize].weapon_num as i32;
}
pub unsafe fn G_BuildTiccmd(
    mut cmd: *mut ticcmd_t,
    mut maketic: i32,
) {
    let mut i: i32 = 0;
    let mut strafe: bool = false;
    let mut bstrafe: boolean = 0;
    let mut speed: i32 = 0;
    let mut tspeed: i32 = 0;
    let mut forward: i32 = 0;
    let mut side: i32 = 0;
    memset(
        cmd as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<ticcmd_t>() as size_t,
    );
    (*cmd).consistancy = consistancy[consoleplayer
        as usize][(maketic % BACKUPTICS) as usize];
    strafe = gamekeydown[key_strafe as usize] != 0
        || *mousebuttons.offset(mousebstrafe as isize) != 0
        || *joybuttons.offset(joybstrafe as isize) != 0;
    speed = (key_speed >= NUMKEYS || joybspeed >= MAX_JOY_BUTTONS
        || gamekeydown[key_speed as usize] != 0
        || *joybuttons.offset(joybspeed as isize) != 0) as i32;
    side = 0 as i32;
    forward = side;
    if joyxmove < 0 as i32 || joyxmove > 0 as i32
        || gamekeydown[key_right as usize] != 0 || gamekeydown[key_left as usize] != 0
    {
        turnheld += ticdup;
    } else {
        turnheld = 0 as i32;
    }
    if turnheld < SLOWTURNTICS {
        tspeed = 2 as i32;
    } else {
        tspeed = speed;
    }
    if strafe {
        if gamekeydown[key_right as usize] != 0 {
            side += sidemove[speed as usize] as i32;
        }
        if gamekeydown[key_left as usize] != 0 {
            side -= sidemove[speed as usize] as i32;
        }
        if joyxmove > 0 as i32 {
            side += sidemove[speed as usize] as i32;
        }
        if joyxmove < 0 as i32 {
            side -= sidemove[speed as usize] as i32;
        }
    } else {
        if gamekeydown[key_right as usize] != 0 {
            (*cmd).angleturn = ((*cmd).angleturn as i32
                - angleturn[tspeed as usize] as i32)
                as i16;
        }
        if gamekeydown[key_left as usize] != 0 {
            (*cmd).angleturn = ((*cmd).angleturn as i32
                + angleturn[tspeed as usize] as i32)
                as i16;
        }
        if joyxmove > 0 as i32 {
            (*cmd).angleturn = ((*cmd).angleturn as i32
                - angleturn[tspeed as usize] as i32)
                as i16;
        }
        if joyxmove < 0 as i32 {
            (*cmd).angleturn = ((*cmd).angleturn as i32
                + angleturn[tspeed as usize] as i32)
                as i16;
        }
    }
    if gamekeydown[key_up as usize] != 0 {
        forward += forwardmove[speed as usize] as i32;
    }
    if gamekeydown[key_down as usize] != 0 {
        forward -= forwardmove[speed as usize] as i32;
    }
    if joyymove < 0 as i32 {
        forward += forwardmove[speed as usize] as i32;
    }
    if joyymove > 0 as i32 {
        forward -= forwardmove[speed as usize] as i32;
    }
    if gamekeydown[key_strafeleft as usize] != 0
        || *joybuttons.offset(joybstrafeleft as isize) != 0
        || *mousebuttons.offset(mousebstrafeleft as isize) != 0
        || joystrafemove < 0 as i32
    {
        side -= sidemove[speed as usize] as i32;
    }
    if gamekeydown[key_straferight as usize] != 0
        || *joybuttons.offset(joybstraferight as isize) != 0
        || *mousebuttons.offset(mousebstraferight as isize) != 0
        || joystrafemove > 0 as i32
    {
        side += sidemove[speed as usize] as i32;
    }
    (*cmd).chatchar = HU_dequeueChatChar() as byte;
    if gamekeydown[key_fire as usize] != 0
        || *mousebuttons.offset(mousebfire as isize) != 0
        || *joybuttons.offset(joybfire as isize) != 0
    {
        (*cmd).buttons = ((*cmd).buttons as i32
            | BT_ATTACK as i32) as byte;
    }
    if gamekeydown[key_use as usize] != 0 || *joybuttons.offset(joybuse as isize) != 0
        || *mousebuttons.offset(mousebuse as isize) != 0
    {
        (*cmd).buttons = ((*cmd).buttons as i32
            | BT_USE as i32) as byte;
        dclicks = 0 as i32;
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32
        && next_weapon != 0 as i32
    {
        i = G_NextWeapon(next_weapon);
        (*cmd).buttons = ((*cmd).buttons as i32
            | BT_CHANGE as i32) as byte;
        (*cmd).buttons = ((*cmd).buttons as i32
            | i << BT_WEAPONSHIFT as i32) as byte;
    } else {
        i = 0 as i32;
        while (i as usize)
            < (::core::mem::size_of::<[*mut i32; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut i32>() as usize)
        {
            let mut key: i32 = *weapon_keys[i as usize];
            if gamekeydown[key as usize] != 0 {
                (*cmd).buttons = ((*cmd).buttons as i32
                    | BT_CHANGE as i32) as byte;
                (*cmd).buttons = ((*cmd).buttons as i32
                    | i << BT_WEAPONSHIFT as i32) as byte;
                break;
            } else {
                i += 1;
            }
        }
    }
    next_weapon = 0 as i32;
    if *mousebuttons.offset(mousebforward as isize) != 0 {
        forward += forwardmove[speed as usize] as i32;
    }
    if *mousebuttons.offset(mousebbackward as isize) != 0 {
        forward -= forwardmove[speed as usize] as i32;
    }
    if dclick_use != 0 {
        if *mousebuttons.offset(mousebforward as isize) != dclickstate
            && dclicktime > 1 as i32
        {
            dclickstate = *mousebuttons.offset(mousebforward as isize);
            if dclickstate != 0 {
                dclicks += 1;
            }
            if dclicks == 2 as i32 {
                (*cmd).buttons = ((*cmd).buttons as i32
                    | BT_USE as i32) as byte;
                dclicks = 0 as i32;
            } else {
                dclicktime = 0 as i32;
            }
        } else {
            dclicktime += ticdup;
            if dclicktime > 20 as i32 {
                dclicks = 0 as i32;
                dclickstate = 0 as boolean;
            }
        }
        bstrafe = (*mousebuttons.offset(mousebstrafe as isize) != 0
            || *joybuttons.offset(joybstrafe as isize) != 0) as i32
            as boolean;
        if bstrafe != dclickstate2 && dclicktime2 > 1 as i32 {
            dclickstate2 = bstrafe;
            if dclickstate2 != 0 {
                dclicks2 += 1;
            }
            if dclicks2 == 2 as i32 {
                (*cmd).buttons = ((*cmd).buttons as i32
                    | BT_USE as i32) as byte;
                dclicks2 = 0 as i32;
            } else {
                dclicktime2 = 0 as i32;
            }
        } else {
            dclicktime2 += ticdup;
            if dclicktime2 > 20 as i32 {
                dclicks2 = 0 as i32;
                dclickstate2 = 0 as boolean;
            }
        }
    }
    forward += mousey;
    if strafe {
        side += mousex * 2 as i32;
    } else {
        (*cmd).angleturn = ((*cmd).angleturn as i32
            - mousex * 0x8 as i32) as i16;
    }
    if mousex == 0 as i32 {
        testcontrols_mousespeed = 0 as i32;
    }
    mousey = 0 as i32;
    mousex = mousey;
    if forward > forwardmove[1 as i32 as usize] {
        forward = forwardmove[1 as i32 as usize] as i32;
    } else if forward < -forwardmove[1 as i32 as usize] {
        forward = -forwardmove[1 as i32 as usize] as i32;
    }
    if side > forwardmove[1 as i32 as usize] {
        side = forwardmove[1 as i32 as usize] as i32;
    } else if side < -forwardmove[1 as i32 as usize] {
        side = -forwardmove[1 as i32 as usize] as i32;
    }
    (*cmd).forwardmove = ((*cmd).forwardmove as i32 + forward)
        as i8;
    (*cmd).sidemove = ((*cmd).sidemove as i32 + side)
        as i8;
    if sendpause {
        sendpause = false;
        (*cmd).buttons = (BT_SPECIAL as i32
            | BTS_PAUSE as i32) as byte;
    }
    if sendsave {
        sendsave = false;
        (*cmd).buttons = (BT_SPECIAL as i32
            | BTS_SAVEGAME as i32
            | savegameslot << BTS_SAVESHIFT as i32) as byte;
    }
    if lowres_turn {
        static mut carry: i16 = 0;
        let mut desired_angleturn: i16 = 0;
        desired_angleturn = ((*cmd).angleturn as i32
            + carry as i32) as i16;
        (*cmd).angleturn = (desired_angleturn as i32
            + 128 as i32 & 0xff00 as i32)
            as i16;
        carry = (desired_angleturn as i32
            - (*cmd).angleturn as i32) as i16;
    }
}
pub unsafe fn G_DoLoadLevel() {
    let mut i: i32 = 0;
    skyflatnum = R_FlatNumForName(
        b"F_SKY1\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    if gamemode as u32
        == commercial as i32 as u32
        && (gameversion as u32
            == exe_final2 as i32 as u32
            || gameversion as u32
                == exe_chex as i32 as u32)
    {
        let mut skytexturename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if gamemap < 12 as i32 {
            skytexturename = b"SKY1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else if gamemap < 21 as i32 {
            skytexturename = b"SKY2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else {
            skytexturename = b"SKY3\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        skytexturename = skytexturename;
        skytexture = R_TextureNumForName(skytexturename);
    }
    levelstarttic = gametic;
    if wipegamestate as u32
        == GS_LEVEL as i32 as u32
    {
        wipegamestate = 4294967295 as gamestate_t;
    }
    gamestate = GS_LEVEL;
    i = 0 as i32;
    while i < MAXPLAYERS {
        turbodetected[i as usize] = false_0 as boolean;
        if playeringame[i as usize] != 0
            && players[i as usize].playerstate as u32
                == PST_DEAD as i32 as u32
        {
            players[i as usize].playerstate = PST_REBORN;
        }
        memset(
            &raw mut (*(&raw mut players as *mut player_t).offset(i as isize)).frags
                as *mut i32 as *mut ::core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<[i32; 4]>() as size_t,
        );
        i += 1;
    }
    P_SetupLevel(gameepisode, gamemap, 0 as i32, gameskill);
    displayplayer = consoleplayer;
    gameaction = ga_nothing;
    Z_CheckHeap();
    memset(
        &raw mut gamekeydown as *mut boolean as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[boolean; 256]>() as size_t,
    );
    joystrafemove = 0 as i32;
    joyymove = joystrafemove;
    joyxmove = joyymove;
    mousey = 0 as i32;
    mousex = mousey;
    paused = false;
    sendsave = paused;
    sendpause = sendsave;
    memset(
        &raw mut mousearray as *mut boolean as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[boolean; 9]>() as size_t,
    );
    memset(
        &raw mut joyarray as *mut boolean as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[boolean; 21]>() as size_t,
    );
    if testcontrols {
        players[consoleplayer as usize].message = b"Press escape to quit.\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
}
unsafe fn SetJoyButtons(mut buttons_mask: u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAX_JOY_BUTTONS {
        let mut button_on: i32 = (buttons_mask
            & ((1 as i32) << i) as u32
            != 0 as u32) as i32;
        if *joybuttons.offset(i as isize) == 0 && button_on != 0 {
            if i == joybprevweapon {
                next_weapon = -(1 as i32);
            } else if i == joybnextweapon {
                next_weapon = 1 as i32;
            }
        }
        *joybuttons.offset(i as isize) = button_on as boolean;
        i += 1;
    }
}
unsafe fn SetMouseButtons(mut buttons_mask: u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAX_MOUSE_BUTTONS {
        let mut button_on: u32 = (buttons_mask
            & ((1 as i32) << i) as u32
            != 0 as u32) as i32 as u32;
        if *mousebuttons.offset(i as isize) == 0 && button_on != 0 {
            if i == mousebprevweapon {
                next_weapon = -(1 as i32);
            } else if i == mousebnextweapon {
                next_weapon = 1 as i32;
            }
        }
        *mousebuttons.offset(i as isize) = button_on as boolean;
        i += 1;
    }
}
pub unsafe fn G_Responder(mut ev: *mut event_t) -> bool {
    if gamestate as u32
        == GS_LEVEL as i32 as u32
        && (*ev).type_0 as u32
            == ev_keydown as i32 as u32
        && (*ev).data1 == key_spy && (singledemo || deathmatch == 0)
    {
        loop {
            displayplayer += 1;
            if displayplayer == MAXPLAYERS {
                displayplayer = 0 as i32;
            }
            if !(playeringame[displayplayer as usize] == 0
                && displayplayer != consoleplayer)
            {
                break;
            }
        }
        return true;
    }
    if gameaction as u32
        == ga_nothing as i32 as u32 && !singledemo
        && (demoplayback
            || gamestate as u32
                == GS_DEMOSCREEN as i32 as u32)
    {
        if (*ev).type_0 as u32
            == ev_keydown as i32 as u32
            || (*ev).type_0 as u32
                == ev_mouse as i32 as u32
                && (*ev).data1 != 0
            || (*ev).type_0 as u32
                == ev_joystick as i32 as u32
                && (*ev).data1 != 0
        {
            M_StartControlPanel();
            return true;
        }
        return false;
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32
    {
        if HU_Responder(ev) {
            return true;
        }
        if ST_Responder(ev) {
            return true;
        }
        if AM_Responder(ev) {
            return true;
        }
    }
    if gamestate as u32
        == GS_FINALE as i32 as u32
    {
        if F_Responder(ev) {
            return true;
        }
    }
    if testcontrols
        && (*ev).type_0 as u32
            == ev_mouse as i32 as u32
    {
        testcontrols_mousespeed = ((*ev).data2).abs();
    }
    if (*ev).type_0 as u32
        == ev_keydown as i32 as u32
        && (*ev).data1 == key_prevweapon
    {
        next_weapon = -(1 as i32);
    } else if (*ev).type_0 as u32
        == ev_keydown as i32 as u32
        && (*ev).data1 == key_nextweapon
    {
        next_weapon = 1 as i32;
    }
    match (*ev).type_0 as u32 {
        0 => {
            if (*ev).data1 == key_pause {
                sendpause = true;
            } else if (*ev).data1 < NUMKEYS {
                gamekeydown[(*ev).data1 as usize] = true_0 as boolean;
            }
            return true;
        }
        1 => {
            if (*ev).data1 < NUMKEYS {
                gamekeydown[(*ev).data1 as usize] = false_0 as boolean;
            }
            return false;
        }
        2 => {
            SetMouseButtons((*ev).data1 as u32);
            mousex = (*ev).data2 * (mouseSensitivity + 5 as i32)
                / 10 as i32;
            mousey = (*ev).data3 * (mouseSensitivity + 5 as i32)
                / 10 as i32;
            return true;
        }
        3 => {
            SetJoyButtons((*ev).data1 as u32);
            joyxmove = (*ev).data2;
            joyymove = (*ev).data3;
            joystrafemove = (*ev).data4;
            return true;
        }
        _ => {}
    }
    return false;
}
pub unsafe fn G_Ticker() {
    let mut i: i32 = 0;
    let mut buf: i32 = 0;
    let mut cmd: *mut ticcmd_t = ::core::ptr::null_mut::<ticcmd_t>();
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0
            && players[i as usize].playerstate as u32
                == PST_REBORN as i32 as u32
        {
            G_DoReborn(i);
        }
        i += 1;
    }
    while gameaction as u32
        != ga_nothing as i32 as u32
    {
        match gameaction as u32 {
            1 => {
                G_DoLoadLevel();
            }
            2 => {
                G_DoNewGame();
            }
            3 => {
                G_DoLoadGame();
            }
            4 => {
                G_DoSaveGame();
            }
            5 => {
                G_DoPlayDemo();
            }
            6 => {
                G_DoCompleted();
            }
            7 => {
                F_StartFinale();
            }
            8 => {
                G_DoWorldDone();
            }
            9 => {
                V_ScreenShot(
                    b"DOOM%02i.%s\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                players[consoleplayer as usize].message = b"screen shot\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                gameaction = ga_nothing;
            }
            0 | _ => {}
        }
    }
    buf = gametic / ticdup % BACKUPTICS;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            cmd = &raw mut (*(&raw mut players as *mut player_t).offset(i as isize)).cmd;
            memcpy(
                cmd as *mut ::core::ffi::c_void,
                netcmds.offset(i as isize) as *mut ticcmd_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<ticcmd_t>() as size_t,
            );
            if demoplayback {
                G_ReadDemoTiccmd(cmd);
            }
            if demorecording {
                G_WriteDemoTiccmd(cmd);
            }
            if (*cmd).forwardmove as i32 > TURBOTHRESHOLD {
                turbodetected[i as usize] = true_0 as boolean;
            }
            if gametic & 31 as i32 == 0 as i32
                && (gametic >> 5 as i32) % MAXPLAYERS == i
                && turbodetected[i as usize] != 0
            {
                static mut turbomessage: [::core::ffi::c_char; 80] = [0; 80];
                M_snprintf(
                    &raw mut turbomessage as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
                    b"%s is turbo!\0" as *const u8 as *const ::core::ffi::c_char,
                    player_names[i as usize],
                );
                players[consoleplayer as usize].message = &raw mut turbomessage
                    as *mut ::core::ffi::c_char;
                turbodetected[i as usize] = false_0 as boolean;
            }
            if netgame && !netdemo && gametic % ticdup == 0 {
                if gametic > BACKUPTICS
                    && consistancy[i as usize][buf as usize] as i32
                        != (*cmd).consistancy as i32
                {
                    I_Error(&format!(
                        "consistency failure ({} should be {})",
                        (*cmd).consistancy as i32,
                        consistancy[i as usize][buf as usize] as i32,
                    ));
                }
                if !players[i as usize].mo.is_null() {
                    consistancy[i as usize][buf as usize] = (*players[i as usize].mo).x
                        as byte;
                } else {
                    consistancy[i as usize][buf as usize] = rndindex as byte;
                }
            }
        }
        i += 1;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            if players[i as usize].cmd.buttons as i32
                & BT_SPECIAL as i32 != 0
            {
                match players[i as usize].cmd.buttons as i32
                    & BT_SPECIALMASK as i32
                {
                    1 => {
                        paused = !paused;
                        if paused {
                            S_PauseSound();
                        } else {
                            S_ResumeSound();
                        }
                    }
                    2 => {
                        if savedescription[0 as i32 as usize] == 0 {
                            M_StringCopy(
                                &raw mut savedescription as *mut ::core::ffi::c_char,
                                b"NET GAME\0" as *const u8 as *const ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 32]>()
                                    as size_t,
                            );
                        }
                        savegameslot = (players[i as usize].cmd.buttons
                            as i32 & BTS_SAVEMASK as i32)
                            >> BTS_SAVESHIFT as i32;
                        gameaction = ga_savegame;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    if oldgamestate as u32
        == GS_INTERMISSION as i32 as u32
        && gamestate as u32
            != GS_INTERMISSION as i32 as u32
    {
        WI_End();
    }
    oldgamestate = gamestate;
    match gamestate as u32 {
        0 => {
            P_Ticker();
            ST_Ticker();
            AM_Ticker();
            HU_Ticker();
        }
        1 => {
            WI_Ticker();
        }
        2 => {
            F_Ticker();
        }
        3 => {
            D_PageTicker();
        }
        _ => {}
    };
}
pub unsafe fn G_InitPlayer(mut player: i32) {
    G_PlayerReborn(player);
}
pub unsafe fn G_PlayerFinishLevel(mut player: i32) {
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    p = (&raw mut players as *mut player_t).offset(player as isize) as *mut player_t;
    memset(
        &raw mut (*p).powers as *mut i32 as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[i32; 6]>() as size_t,
    );
    memset(
        &raw mut (*p).cards as *mut bool as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[bool; 6]>() as size_t,
    );
    (*(*p).mo).flags &= !(MF_SHADOW as i32);
    (*p).extralight = 0 as i32;
    (*p).fixedcolormap = 0 as i32;
    (*p).damagecount = 0 as i32;
    (*p).bonuscount = 0 as i32;
}
pub unsafe fn G_PlayerReborn(mut player: i32) {
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut i: i32 = 0;
    let mut frags: [i32; 4] = [0; 4];
    let mut killcount: i32 = 0;
    let mut itemcount: i32 = 0;
    let mut secretcount: i32 = 0;
    memcpy(
        &raw mut frags as *mut i32 as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut players as *mut player_t).offset(player as isize)).frags
            as *mut i32 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[i32; 4]>() as size_t,
    );
    killcount = players[player as usize].killcount;
    itemcount = players[player as usize].itemcount;
    secretcount = players[player as usize].secretcount;
    p = (&raw mut players as *mut player_t).offset(player as isize) as *mut player_t;
    memset(
        p as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<player_t>() as size_t,
    );
    memcpy(
        &raw mut (*(&raw mut players as *mut player_t).offset(player as isize)).frags
            as *mut i32 as *mut ::core::ffi::c_void,
        &raw mut frags as *mut i32 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[i32; 4]>() as size_t,
    );
    players[player as usize].killcount = killcount;
    players[player as usize].itemcount = itemcount;
    players[player as usize].secretcount = secretcount;
    (*p).attackdown = true_0;
    (*p).usedown = (*p).attackdown;
    (*p).playerstate = PST_LIVE;
    (*p).health = deh_initial_health;
    (*p).pendingweapon = wp_pistol;
    (*p).readyweapon = (*p).pendingweapon;
    (*p).weaponowned[wp_fist as i32 as usize] = true;
    (*p).weaponowned[wp_pistol as i32 as usize] = true;
    (*p).ammo[am_clip as i32 as usize] = deh_initial_bullets;
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        (*p).maxammo[i as usize] = maxammo[i as usize];
        i += 1;
    }
}
pub unsafe fn G_CheckSpot(
    mut playernum: i32,
    mut mthing: *mut mapthing_t,
) -> bool {
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut i: i32 = 0;
    if players[playernum as usize].mo.is_null() {
        i = 0 as i32;
        while i < playernum {
            if (*players[i as usize].mo).x
                == ((*mthing).x as i32) << FRACBITS
                && (*players[i as usize].mo).y
                    == ((*mthing).y as i32) << FRACBITS
            {
                return false;
            }
            i += 1;
        }
        return true;
    }
    x = (((*mthing).x as i32) << FRACBITS) as fixed_t;
    y = (((*mthing).y as i32) << FRACBITS) as fixed_t;
    if !P_CheckPosition(players[playernum as usize].mo, x, y) {
        return false;
    }
    if bodyqueslot >= BODYQUESIZE {
        P_RemoveMobj(bodyque[(bodyqueslot % BODYQUESIZE) as usize]);
    }
    bodyque[(bodyqueslot % BODYQUESIZE) as usize] = players[playernum as usize].mo;
    bodyqueslot += 1;
    ss = R_PointInSubsector(x, y);
    let mut xa: fixed_t = 0;
    let mut ya: fixed_t = 0;
    let mut an: i32 = 0;
    an = (ANG45 >> ANGLETOFINESHIFT)
        * ((*mthing).angle as i32 / 45 as i32);
    match an {
        4096 => {
            xa = finetangent[2048 as i32 as usize];
            ya = finetangent[0 as i32 as usize];
        }
        5120 => {
            xa = finetangent[3072 as i32 as usize];
            ya = finetangent[1024 as i32 as usize];
        }
        6144 => {
            xa = finesine[0 as i32 as usize];
            ya = finetangent[2048 as i32 as usize];
        }
        7168 => {
            xa = finesine[1024 as i32 as usize];
            ya = finetangent[3072 as i32 as usize];
        }
        0 | 1024 | 2048 | 3072 => {
            xa = finecosine[an as isize];
            ya = finesine[an as usize];
        }
        _ => {
            I_Error(&format!("G_CheckSpot: unexpected angle {}\n", an));
            ya = 0 as i32 as fixed_t;
            xa = ya;
        }
    }
    mo = P_SpawnMobj(
        x + 20 as fixed_t * xa,
        y + 20 as fixed_t * ya,
        (*(*ss).sector).floorheight,
        MT_TFOG,
    );
    if players[consoleplayer as usize].viewz != 1 as i32 {
        S_StartSound(mo as *mut ::core::ffi::c_void, sfx_telept as i32);
    }
    return true;
}
pub unsafe fn G_DeathMatchSpawnPlayer(mut playernum: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut selections: i32 = 0;
    selections = deathmatch_p.offset_from(&raw mut deathmatchstarts as *mut mapthing_t)
        as i64 as i32;
    if selections < 4 as i32 {
        I_Error(&format!("Only {} deathmatch spots, 4 required", selections));
    }
    j = 0 as i32;
    while j < 20 as i32 {
        i = P_Random() % selections;
        if G_CheckSpot(
            playernum,
            (&raw mut deathmatchstarts as *mut mapthing_t).offset(i as isize)
                as *mut mapthing_t,
        )
        {
            deathmatchstarts[i as usize].type_0 = (playernum + 1 as i32)
                as i16;
            P_SpawnPlayer(
                (&raw mut deathmatchstarts as *mut mapthing_t).offset(i as isize)
                    as *mut mapthing_t,
            );
            return;
        }
        j += 1;
    }
    P_SpawnPlayer(
        (&raw mut playerstarts as *mut mapthing_t).offset(playernum as isize)
            as *mut mapthing_t,
    );
}
pub unsafe fn G_DoReborn(mut playernum: i32) {
    let mut i: i32 = 0;
    if !netgame {
        gameaction = ga_loadlevel;
    } else {
        (*players[playernum as usize].mo).player = ::core::ptr::null_mut::<player_s>();
        if deathmatch != 0 {
            G_DeathMatchSpawnPlayer(playernum);
            return;
        }
        if G_CheckSpot(
            playernum,
            (&raw mut playerstarts as *mut mapthing_t).offset(playernum as isize)
                as *mut mapthing_t,
        )
        {
            P_SpawnPlayer(
                (&raw mut playerstarts as *mut mapthing_t).offset(playernum as isize)
                    as *mut mapthing_t,
            );
            return;
        }
        i = 0 as i32;
        while i < MAXPLAYERS {
            if G_CheckSpot(
                playernum,
                (&raw mut playerstarts as *mut mapthing_t).offset(i as isize)
                    as *mut mapthing_t,
            )
            {
                playerstarts[i as usize].type_0 = (playernum + 1 as i32)
                    as i16;
                P_SpawnPlayer(
                    (&raw mut playerstarts as *mut mapthing_t).offset(i as isize)
                        as *mut mapthing_t,
                );
                playerstarts[i as usize].type_0 = (i + 1 as i32)
                    as i16;
                return;
            }
            i += 1;
        }
        P_SpawnPlayer(
            (&raw mut playerstarts as *mut mapthing_t).offset(playernum as isize)
                as *mut mapthing_t,
        );
    };
}
pub unsafe fn G_ScreenShot() {
    gameaction = ga_screenshot;
}
#[no_mangle]
pub static mut pars: [[i32; 10]; 4] = [
    [0 as i32; 10],
    [
        0 as i32,
        30 as i32,
        75 as i32,
        120 as i32,
        90 as i32,
        165 as i32,
        180 as i32,
        180 as i32,
        30 as i32,
        165 as i32,
    ],
    [
        0 as i32,
        90 as i32,
        90 as i32,
        90 as i32,
        120 as i32,
        90 as i32,
        360 as i32,
        240 as i32,
        30 as i32,
        170 as i32,
    ],
    [
        0 as i32,
        90 as i32,
        45 as i32,
        90 as i32,
        150 as i32,
        90 as i32,
        90 as i32,
        165 as i32,
        30 as i32,
        135 as i32,
    ],
];
#[no_mangle]
pub static mut cpars: [i32; 32] = [
    30 as i32,
    90 as i32,
    120 as i32,
    120 as i32,
    90 as i32,
    150 as i32,
    120 as i32,
    120 as i32,
    270 as i32,
    90 as i32,
    210 as i32,
    150 as i32,
    150 as i32,
    150 as i32,
    210 as i32,
    150 as i32,
    420 as i32,
    150 as i32,
    210 as i32,
    150 as i32,
    240 as i32,
    150 as i32,
    180 as i32,
    150 as i32,
    150 as i32,
    300 as i32,
    330 as i32,
    420 as i32,
    300 as i32,
    180 as i32,
    120 as i32,
    30 as i32,
];
#[no_mangle]
pub static mut secretexit: bool = false;
pub unsafe fn G_ExitLevel() {
    secretexit = false;
    gameaction = ga_completed;
}
pub unsafe fn G_SecretExitLevel() {
    if gamemode as u32
        == commercial as i32 as u32
        && W_CheckNumForName("map31",
        ) < 0 as i32
    {
        secretexit = false;
    } else {
        secretexit = true;
    }
    gameaction = ga_completed;
}
pub unsafe fn G_DoCompleted() {
    let mut i: i32 = 0;
    gameaction = ga_nothing;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            G_PlayerFinishLevel(i);
        }
        i += 1;
    }
    if automapactive {
        AM_Stop();
    }
    if gamemode as u32
        != commercial as i32 as u32
    {
        if gameversion as u32
            == exe_chex as i32 as u32
        {
            if gamemap == 5 as i32 {
                gameaction = ga_victory;
                return;
            }
        } else {
            match gamemap {
                8 => {
                    gameaction = ga_victory;
                    return;
                }
                9 => {
                    i = 0 as i32;
                    while i < MAXPLAYERS {
                        players[i as usize].didsecret = true;
                        i += 1;
                    }
                }
                _ => {}
            }
        }
    }
    if gamemap == 8 as i32
        && gamemode as u32
            != commercial as i32 as u32
    {
        gameaction = ga_victory;
        return;
    }
    if gamemap == 9 as i32
        && gamemode as u32
            != commercial as i32 as u32
    {
        i = 0 as i32;
        while i < MAXPLAYERS {
            players[i as usize].didsecret = true;
            i += 1;
        }
    }
    wminfo.didsecret = players[consoleplayer as usize].didsecret;
    wminfo.epsd = gameepisode - 1 as i32;
    wminfo.last = gamemap - 1 as i32;
    if gamemode as u32
        == commercial as i32 as u32
    {
        if secretexit {
            match gamemap {
                15 => {
                    wminfo.next = 30 as i32;
                }
                31 => {
                    wminfo.next = 31 as i32;
                }
                _ => {}
            }
        } else {
            match gamemap {
                31 | 32 => {
                    wminfo.next = 15 as i32;
                }
                _ => {
                    wminfo.next = gamemap;
                }
            }
        }
    } else if secretexit {
        wminfo.next = 8 as i32;
    } else if gamemap == 9 as i32 {
        match gameepisode {
            1 => {
                wminfo.next = 3 as i32;
            }
            2 => {
                wminfo.next = 5 as i32;
            }
            3 => {
                wminfo.next = 6 as i32;
            }
            4 => {
                wminfo.next = 2 as i32;
            }
            _ => {}
        }
    } else {
        wminfo.next = gamemap;
    }
    wminfo.maxkills = totalkills;
    wminfo.maxitems = totalitems;
    wminfo.maxsecret = totalsecret;
    wminfo.maxfrags = 0 as i32;
    if gamemode as u32
        == commercial as i32 as u32
    {
        wminfo.partime = TICRATE * cpars[(gamemap - 1 as i32) as usize];
    } else if gameepisode < 4 as i32 {
        wminfo.partime = TICRATE * pars[gameepisode as usize][gamemap as usize];
    } else {
        wminfo.partime = TICRATE * cpars[gamemap as usize];
    }
    wminfo.pnum = consoleplayer;
    i = 0 as i32;
    while i < MAXPLAYERS {
        wminfo.plyr[i as usize].in_0 = playeringame[i as usize] != 0;
        wminfo.plyr[i as usize].skills = players[i as usize].killcount;
        wminfo.plyr[i as usize].sitems = players[i as usize].itemcount;
        wminfo.plyr[i as usize].ssecret = players[i as usize].secretcount;
        wminfo.plyr[i as usize].stime = leveltime;
        memcpy(
            &raw mut (*(&raw mut wminfo.plyr as *mut wbplayerstruct_t)
                .offset(i as isize))
                .frags as *mut i32 as *mut ::core::ffi::c_void,
            &raw mut (*(&raw mut players as *mut player_t).offset(i as isize)).frags
                as *mut i32 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[i32; 4]>() as size_t,
        );
        i += 1;
    }
    gamestate = GS_INTERMISSION;
    viewactive = false;
    automapactive = false;
    StatCopy(unsafe { &mut game_state().statdump }, &raw mut wminfo);
    WI_Start(&raw mut wminfo);
}
pub unsafe fn G_WorldDone() {
    gameaction = ga_worlddone;
    if secretexit {
        players[consoleplayer as usize].didsecret = true;
    }
    if gamemode as u32
        == commercial as i32 as u32
    {
        let mut current_block_3: u64;
        match gamemap {
            15 | 31 => {
                if !secretexit {
                    current_block_3 = 6937071982253665452;
                } else {
                    current_block_3 = 9744923308842414524;
                }
            }
            6 | 11 | 20 | 30 => {
                current_block_3 = 9744923308842414524;
            }
            _ => {
                current_block_3 = 6937071982253665452;
            }
        }
        match current_block_3 {
            9744923308842414524 => {
                F_StartFinale();
            }
            _ => {}
        }
    }
}
pub unsafe fn G_DoWorldDone() {
    gamestate = GS_LEVEL;
    gamemap = wminfo.next + 1 as i32;
    G_DoLoadLevel();
    gameaction = ga_nothing;
    viewactive = true;
}
#[no_mangle]
pub static mut savename: [::core::ffi::c_char; 256] = [0; 256];
pub unsafe fn G_LoadGame(mut name: *mut ::core::ffi::c_char) {
    M_StringCopy(
        &raw mut savename as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
    );
    gameaction = ga_loadgame;
}
pub unsafe fn G_DoLoadGame() {
    let mut savedleveltime: i32 = 0;
    gameaction = ga_nothing;
    save_stream = fopen(
        &raw mut savename as *mut ::core::ffi::c_char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if save_stream.is_null() {
        return;
    }
    savegame_error = false;
    if !P_ReadSaveGameHeader() {
        fclose(save_stream);
        return;
    }
    savedleveltime = leveltime;
    G_InitNew(gameskill, gameepisode, gamemap);
    leveltime = savedleveltime;
    P_UnArchivePlayers();
    P_UnArchiveWorld();
    P_UnArchiveThinkers();
    P_UnArchiveSpecials();
    if !P_ReadSaveGameEOF() {
        I_Error("Bad savegame");
    }
    fclose(save_stream);
    if setsizeneeded {
        R_ExecuteSetViewSize();
    }
    R_FillBackScreen();
}
pub unsafe fn G_SaveGame(
    mut slot: i32,
    mut description: *mut ::core::ffi::c_char,
) {
    savegameslot = slot;
    M_StringCopy(
        &raw mut savedescription as *mut ::core::ffi::c_char,
        description,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
    );
    sendsave = true;
}
pub unsafe fn G_DoSaveGame() {
    let mut savegame_file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut temp_savegame_file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut recovery_savegame_file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    recovery_savegame_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    temp_savegame_file = P_TempSaveGameFile();
    savegame_file = P_SaveGameFile(savegameslot);
    save_stream = fopen(
        temp_savegame_file,
        b"wb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if save_stream.is_null() {
        recovery_savegame_file = M_TempFile(
            b"recovery.dsg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        save_stream = fopen(
            recovery_savegame_file,
            b"wb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if save_stream.is_null() {
            I_Error(&format!(
                "Failed to open either '{}' or '{}' to write savegame.",
                ::std::ffi::CStr::from_ptr(temp_savegame_file).to_str().unwrap(),
                ::std::ffi::CStr::from_ptr(recovery_savegame_file).to_str().unwrap(),
            ));
        }
    }
    savegame_error = false;
    P_WriteSaveGameHeader(&raw mut savedescription as *mut ::core::ffi::c_char);
    P_ArchivePlayers();
    P_ArchiveWorld();
    P_ArchiveThinkers();
    P_ArchiveSpecials();
    P_WriteSaveGameEOF();
    if vanilla_savegame_limit != 0
        && ftell(save_stream) > SAVEGAMESIZE as i64
    {
        I_Error("Savegame buffer overrun");
    }
    fclose(save_stream);
    if !recovery_savegame_file.is_null() {
        I_Error(&format!(
            "Failed to open savegame file '{}' for writing.\nBut your game has been saved to '{}' for recovery.",
            ::std::ffi::CStr::from_ptr(temp_savegame_file).to_str().unwrap(),
            ::std::ffi::CStr::from_ptr(recovery_savegame_file).to_str().unwrap(),
        ));
    }
    remove(savegame_file);
    rename(temp_savegame_file, savegame_file);
    gameaction = ga_nothing;
    M_StringCopy(
        &raw mut savedescription as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
    );
    players[consoleplayer as usize].message = b"game saved.\0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    R_FillBackScreen();
}
#[no_mangle]
pub static mut d_skill: skill_t = sk_baby;
#[no_mangle]
pub static mut d_episode: i32 = 0;
#[no_mangle]
pub static mut d_map: i32 = 0;
pub unsafe fn G_DeferedInitNew(
    mut skill: skill_t,
    mut episode: i32,
    mut map: i32,
) {
    d_skill = skill;
    d_episode = episode;
    d_map = map;
    gameaction = ga_newgame;
}
pub unsafe fn G_DoNewGame() {
    demoplayback = false;
    netdemo = false;
    netgame = false;
    deathmatch = false_0;
    playeringame[3 as i32 as usize] = 0 as boolean;
    playeringame[2 as i32 as usize] = playeringame[3 as i32
        as usize];
    playeringame[1 as i32 as usize] = playeringame[2 as i32
        as usize];
    respawnparm = false;
    fastparm = false;
    nomonsters = false;
    consoleplayer = 0 as i32;
    G_InitNew(d_skill, d_episode, d_map);
    gameaction = ga_nothing;
}
pub unsafe fn G_InitNew(
    mut skill: skill_t,
    mut episode: i32,
    mut map: i32,
) {
    let mut skytexturename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: i32 = 0;
    if paused {
        paused = false;
        S_ResumeSound();
    }
    if skill as i32 > sk_nightmare as i32 {
        skill = sk_nightmare;
    }
    if gameversion as u32
        >= exe_ultimate as i32 as u32
    {
        if episode == 0 as i32 {
            episode = 4 as i32;
        }
    } else {
        if episode < 1 as i32 {
            episode = 1 as i32;
        }
        if episode > 3 as i32 {
            episode = 3 as i32;
        }
    }
    if episode > 1 as i32
        && gamemode as u32
            == shareware as i32 as u32
    {
        episode = 1 as i32;
    }
    if map < 1 as i32 {
        map = 1 as i32;
    }
    if map > 9 as i32
        && gamemode as u32
            != commercial as i32 as u32
    {
        map = 9 as i32;
    }
    M_ClearRandom();
    if skill as i32 == sk_nightmare as i32
        || respawnparm
    {
        respawnmonsters = true;
    } else {
        respawnmonsters = false;
    }
    if fastparm
        || skill as i32 == sk_nightmare as i32
            && gameskill as i32 != sk_nightmare as i32
    {
        i = S_SARG_RUN1 as i32;
        while i <= S_SARG_PAIN2 as i32 {
            states[i as usize].tics >>= 1 as i32;
            i += 1;
        }
        mobjinfo[MT_BRUISERSHOT as i32 as usize].speed = 20
            as i32 * FRACUNIT;
        mobjinfo[MT_HEADSHOT as i32 as usize].speed = 20
            as i32 * FRACUNIT;
        mobjinfo[MT_TROOPSHOT as i32 as usize].speed = 20
            as i32 * FRACUNIT;
    } else if skill as i32 != sk_nightmare as i32
        && gameskill as i32 == sk_nightmare as i32
    {
        i = S_SARG_RUN1 as i32;
        while i <= S_SARG_PAIN2 as i32 {
            states[i as usize].tics <<= 1 as i32;
            i += 1;
        }
        mobjinfo[MT_BRUISERSHOT as i32 as usize].speed = 15
            as i32 * FRACUNIT;
        mobjinfo[MT_HEADSHOT as i32 as usize].speed = 10
            as i32 * FRACUNIT;
        mobjinfo[MT_TROOPSHOT as i32 as usize].speed = 10
            as i32 * FRACUNIT;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        players[i as usize].playerstate = PST_REBORN;
        i += 1;
    }
    usergame = true;
    paused = false;
    demoplayback = false;
    automapactive = false;
    viewactive = true;
    gameepisode = episode;
    gamemap = map;
    gameskill = skill;
    viewactive = true;
    if gamemode as u32
        == commercial as i32 as u32
    {
        if gamemap < 12 as i32 {
            skytexturename = b"SKY1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else if gamemap < 21 as i32 {
            skytexturename = b"SKY2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else {
            skytexturename = b"SKY3\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    } else {
        match gameepisode {
            2 => {
                skytexturename = b"SKY2\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            3 => {
                skytexturename = b"SKY3\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            4 => {
                skytexturename = b"SKY4\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            1 | _ => {
                skytexturename = b"SKY1\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        }
    }
    skytexturename = skytexturename;
    skytexture = R_TextureNumForName(skytexturename);
    G_DoLoadLevel();
}
pub const DEMOMARKER: i32 = 0x80;
pub unsafe fn G_ReadDemoTiccmd(mut cmd: *mut ticcmd_t) {
    if *demo_p as i32 == DEMOMARKER {
        G_CheckDemoStatus();
        return;
    }
    let fresh18 = demo_p;
    demo_p = demo_p.offset(1);
    (*cmd).forwardmove = *fresh18 as i8;
    let fresh19 = demo_p;
    demo_p = demo_p.offset(1);
    (*cmd).sidemove = *fresh19 as i8;
    if longtics {
        let fresh20 = demo_p;
        demo_p = demo_p.offset(1);
        (*cmd).angleturn = *fresh20 as i16;
        let fresh21 = demo_p;
        demo_p = demo_p.offset(1);
        (*cmd).angleturn = ((*cmd).angleturn as i32
            | (*fresh21 as i32) << 8 as i32)
            as i16;
    } else {
        let fresh22 = demo_p;
        demo_p = demo_p.offset(1);
        (*cmd).angleturn = ((*fresh22 as u8 as i32)
            << 8 as i32) as i16;
    }
    let fresh23 = demo_p;
    demo_p = demo_p.offset(1);
    (*cmd).buttons = *fresh23 as u8 as byte;
}
unsafe fn IncreaseDemoBuffer() {
    let mut current_length: i32 = 0;
    let mut new_demobuffer: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut new_demop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut new_length: i32 = 0;
    current_length = demoend.offset_from(demobuffer) as i64
        as i32;
    new_length = current_length * 2 as i32;
    new_demobuffer = Z_Malloc(
        new_length,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut byte;
    new_demop = new_demobuffer
        .offset(demo_p.offset_from(demobuffer) as i64 as isize);
    memcpy(
        new_demobuffer as *mut ::core::ffi::c_void,
        demobuffer as *const ::core::ffi::c_void,
        current_length as size_t,
    );
    Z_Free(demobuffer as *mut ::core::ffi::c_void);
    demobuffer = new_demobuffer;
    demo_p = new_demop;
    demoend = demobuffer.offset(new_length as isize);
}
pub unsafe fn G_WriteDemoTiccmd(mut cmd: *mut ticcmd_t) {
    let mut demo_start: *mut byte = ::core::ptr::null_mut::<byte>();
    if gamekeydown[key_demo_quit as usize] != 0 {
        G_CheckDemoStatus();
    }
    demo_start = demo_p;
    let fresh12 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh12 = (*cmd).forwardmove as byte;
    let fresh13 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh13 = (*cmd).sidemove as byte;
    if longtics {
        let fresh14 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh14 = ((*cmd).angleturn as i32 & 0xff as i32)
            as byte;
        let fresh15 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh15 = ((*cmd).angleturn as i32 >> 8 as i32
            & 0xff as i32) as byte;
    } else {
        let fresh16 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh16 = ((*cmd).angleturn as i32 >> 8 as i32)
            as byte;
    }
    let fresh17 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh17 = (*cmd).buttons;
    demo_p = demo_start;
    if demo_p > demoend.offset(-(16 as i32 as isize)) {
        if vanilla_demo_limit != 0 {
            G_CheckDemoStatus();
            return;
        } else {
            IncreaseDemoBuffer();
        }
    }
    G_ReadDemoTiccmd(cmd);
}
pub unsafe fn G_RecordDemo(mut name: *mut ::core::ffi::c_char) {
    let mut demoname_size: size_t = 0;
    let mut i: i32 = 0;
    let mut maxsize: i32 = 0;
    usergame = false;
    demoname_size = strlen(name).wrapping_add(5 as size_t);
    demoname = Z_Malloc(
        demoname_size as i32,
        PU_STATIC as i32,
        NULL,
    ) as *mut ::core::ffi::c_char;
    M_snprintf(
        demoname,
        demoname_size,
        b"%s.lmp\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    maxsize = 0x20000 as i32;
    i = M_CheckParmWithArgs("-maxdemo", 1 as i32);
    if i != 0 {
        maxsize = atoi(
            myargv[(i + 1 as i32) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        ) * 1024 as i32;
    }
    demobuffer = Z_Malloc(maxsize, PU_STATIC as i32, NULL) as *mut byte;
    demoend = demobuffer.offset(maxsize as isize);
    demorecording = true;
}
pub unsafe fn G_VanillaVersionCode() -> i32 {
    match gameversion as u32 {
        0 => {
            I_Error("Doom 1.2 does not have a version code!");
        }
        1 => {}
        2 => return 107 as i32,
        3 => return 108 as i32,
        4 | _ => return 109 as i32,
    }
    return 106 as i32;
}
pub unsafe fn G_BeginRecording() {
    let mut i: i32 = 0;
    longtics = M_CheckParm("-longtics") != 0 as i32;
    lowres_turn = !longtics;
    demo_p = demobuffer;
    if longtics {
        let fresh0 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh0 = DOOM_191_VERSION as byte;
    } else {
        let fresh1 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh1 = G_VanillaVersionCode() as byte;
    }
    let fresh2 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh2 = gameskill as byte;
    let fresh3 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh3 = gameepisode as byte;
    let fresh4 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh4 = gamemap as byte;
    let fresh5 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh5 = deathmatch as byte;
    let fresh6 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh6 = respawnparm as byte;
    let fresh7 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh7 = fastparm as byte;
    let fresh8 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh8 = nomonsters as byte;
    let fresh9 = demo_p;
    demo_p = demo_p.offset(1);
    *fresh9 = consoleplayer as byte;
    i = 0 as i32;
    while i < MAXPLAYERS {
        let fresh10 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh10 = playeringame[i as usize] as byte;
        i += 1;
    }
}
#[no_mangle]
pub static mut defdemoname: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
pub unsafe fn G_DeferedPlayDemo(mut name: *mut ::core::ffi::c_char) {
    defdemoname = name;
    gameaction = ga_playdemo;
}
unsafe fn DemoVersionDescription(
    mut version: i32,
) -> *mut ::core::ffi::c_char {
    static mut resultbuf: [::core::ffi::c_char; 16] = [0; 16];
    match version {
        104 => {
            return b"v1.4\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        105 => {
            return b"v1.5\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        106 => {
            return b"v1.6/v1.666\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        107 => {
            return b"v1.7/v1.7a\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        108 => {
            return b"v1.8\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        109 => {
            return b"v1.9\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {}
    }
    if version >= 0 as i32 && version <= 4 as i32 {
        return b"v1.0/v1.1/v1.2\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char
    } else {
        M_snprintf(
            &raw mut resultbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
            b"%i.%i (unknown)\0" as *const u8 as *const ::core::ffi::c_char,
            version / 100 as i32,
            version % 100 as i32,
        );
        return &raw mut resultbuf as *mut ::core::ffi::c_char;
    };
}
pub unsafe fn G_DoPlayDemo() {
    let mut skill: skill_t = sk_baby;
    let mut i: i32 = 0;
    let mut episode: i32 = 0;
    let mut map: i32 = 0;
    let mut demoversion: i32 = 0;
    gameaction = ga_nothing;
    demo_p = W_CacheLumpName(
        &wad_name8_to_string(defdemoname),
        PU_STATIC as i32,
    ) as *mut byte;
    demobuffer = demo_p;
    let fresh24 = demo_p;
    demo_p = demo_p.offset(1);
    demoversion = *fresh24 as i32;
    if demoversion == G_VanillaVersionCode() {
        longtics = false;
    } else if demoversion == DOOM_191_VERSION {
        longtics = true;
    } else {
        let mut message: *mut ::core::ffi::c_char = b"Demo is from a different game version!\n(read %i, should be %i)\n\n*** You may need to upgrade your version of Doom to v1.9. ***\n    See: https://www.doomworld.com/classicdoom/info/patches.php\n    This appears to be %s.\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        printf(
            message,
            demoversion,
            G_VanillaVersionCode(),
            DemoVersionDescription(demoversion),
        );
    }
    let fresh25 = demo_p;
    demo_p = demo_p.offset(1);
    skill = *fresh25 as skill_t;
    let fresh26 = demo_p;
    demo_p = demo_p.offset(1);
    episode = *fresh26 as i32;
    let fresh27 = demo_p;
    demo_p = demo_p.offset(1);
    map = *fresh27 as i32;
    let fresh28 = demo_p;
    demo_p = demo_p.offset(1);
    deathmatch = *fresh28 as i32;
    let fresh29 = demo_p;
    demo_p = demo_p.offset(1);
    respawnparm = *fresh29 != 0;
    let fresh30 = demo_p;
    demo_p = demo_p.offset(1);
    fastparm = *fresh30 != 0;
    let fresh31 = demo_p;
    demo_p = demo_p.offset(1);
    nomonsters = *fresh31 != 0;
    let fresh32 = demo_p;
    demo_p = demo_p.offset(1);
    consoleplayer = *fresh32 as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        let fresh33 = demo_p;
        demo_p = demo_p.offset(1);
        playeringame[i as usize] = *fresh33 as boolean;
        i += 1;
    }
    if playeringame[1 as i32 as usize] != 0
        || M_CheckParm("-solo-net") > 0 as i32
        || M_CheckParm("-netdemo") > 0 as i32
    {
        netgame = true;
        netdemo = true;
    }
    precache = false;
    G_InitNew(skill, episode, map);
    precache = true;
    starttime = I_GetTime(unsafe { &mut game_state().i_timer });
    usergame = false;
    demoplayback = true;
}
pub unsafe fn G_TimeDemo(mut name: *mut ::core::ffi::c_char) {
    nodrawers = M_CheckParm("-nodraw") != 0;
    timingdemo = true;
    singletics = true;
    defdemoname = name;
    gameaction = ga_playdemo;
}
#[no_mangle]
pub unsafe extern "C" fn G_CheckDemoStatus() -> boolean {
    let mut endtime: i32 = 0;
    if timingdemo {
        let mut fps: f32 = 0.;
        let mut realtics: i32 = 0;
        endtime = I_GetTime(unsafe { &mut game_state().i_timer });
        realtics = endtime - starttime;
        fps = gametic as f32 * TICRATE as f32
            / realtics as f32;
        timingdemo = false;
        demoplayback = false;
        I_Error(&format!(
            "timed {} gametics in {} realtics ({:.6} fps)",
            gametic,
            realtics,
            fps as f64,
        ));
    }
    if demoplayback {
        W_ReleaseLumpName(&wad_name8_to_string(defdemoname));
        demoplayback = false;
        netdemo = false;
        netgame = false;
        deathmatch = false_0;
        playeringame[3 as i32 as usize] = 0 as boolean;
        playeringame[2 as i32 as usize] = playeringame[3
            as i32 as usize];
        playeringame[1 as i32 as usize] = playeringame[2
            as i32 as usize];
        respawnparm = false;
        fastparm = false;
        nomonsters = false;
        consoleplayer = 0 as i32;
        if singledemo {
            I_Quit();
        } else {
            D_AdvanceDemo();
        }
        return true_0 as boolean;
    }
    if demorecording {
        let fresh11 = demo_p;
        demo_p = demo_p.offset(1);
        *fresh11 = DEMOMARKER as byte;
        M_WriteFile(
            demoname,
            demobuffer as *mut ::core::ffi::c_void,
            demo_p.offset_from(demobuffer) as i64 as i32,
        );
        Z_Free(demobuffer as *mut ::core::ffi::c_void);
        demorecording = false;
        I_Error(&format!(
            "Demo {} recorded",
            ::std::ffi::CStr::from_ptr(demoname).to_str().unwrap(),
        ));
    }
    return false_0 as boolean;
}
pub const MAX_MOUSE_BUTTONS: i32 = 8;
unsafe extern "C" fn run_static_initializers() {
    joybuttons = (&raw mut joyarray as *mut boolean)
        .offset(1 as i32 as isize) as *mut boolean;
    mousebuttons = (&raw mut mousearray as *mut boolean)
        .offset(1 as i32 as isize) as *mut boolean;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
