use crate::src::am_map::AM_Responder;
use crate::src::am_map::AM_Stop;
use crate::src::am_map::AM_Ticker;
use crate::src::d_event::event_t;
use crate::src::d_event::{ev_joystick, ev_keydown, ev_mouse};
use crate::src::d_event::{
    ga_completed, ga_loadgame, ga_loadlevel, ga_newgame, ga_nothing, ga_playdemo, ga_savegame,
    ga_screenshot, ga_victory, ga_worlddone, gameaction_t,
};
use crate::src::d_event::{gamestate_t, GS_DEMOSCREEN, GS_FINALE, GS_INTERMISSION, GS_LEVEL};
use crate::src::d_loop::BACKUPTICS;
use crate::src::d_main::D_AdvanceDemo;
use crate::src::d_main::D_PageTicker;
use crate::src::d_mode::{commercial, shareware};
use crate::src::d_mode::{doom, doom2, pack_chex, pack_hacx};
use crate::src::d_mode::{exe_chex, exe_final2, exe_ultimate};
use crate::src::d_mode::{sk_baby, sk_nightmare, skill_t};
use crate::src::d_player::pw_strength;
use crate::src::d_player::{am_clip, NUMAMMO};
use crate::src::d_player::{player_s, player_t, PST_DEAD, PST_LIVE, PST_REBORN};
use crate::src::d_player::{
    weapontype_t, wp_bfg, wp_chaingun, wp_chainsaw, wp_fist, wp_missile, wp_nochange, wp_pistol,
    wp_plasma, wp_shotgun, wp_supershotgun,
};
use crate::src::d_ticcmd::ticcmd_t;
use crate::src::d_ticcmd::{
    BTS_PAUSE, BTS_SAVEGAME, BTS_SAVEMASK, BTS_SAVESHIFT, BT_ATTACK, BT_CHANGE, BT_SPECIAL,
    BT_SPECIALMASK, BT_USE, BT_WEAPONSHIFT,
};
use crate::src::doomdef::boolean;
use crate::src::doomdef::false_0;
use crate::src::doomdef::true_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::doomdef::NULL;
use crate::src::doomdef::TICRATE;
use crate::src::f_finale::F_Responder;
use crate::src::f_finale::F_StartFinale;
use crate::src::f_finale::F_Ticker;
use crate::src::doomstat::DoomstatState;
use crate::src::game_state::game_state;
use crate::src::game_state::GameState;
use crate::src::hu_stuff::player_names;
use crate::src::hu_stuff::HU_Responder;
use crate::src::hu_stuff::HU_Ticker;
use crate::src::hu_stuff::HU_dequeueChatChar;
use crate::src::i_system::I_Error;
use crate::src::i_system::I_Quit;
use crate::src::i_system::FILE;
use crate::src::i_system::{fclose, fopen, ftell};
use crate::src::i_timer::I_GetTime;
use crate::src::info::{S_SARG_PAIN2, S_SARG_RUN1};
use crate::src::m_argv::{M_CheckParm, M_CheckParmWithArgs};
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FRACBITS;
use crate::src::m_fixed::FRACUNIT;
use crate::src::m_menu::M_StartControlPanel;
use crate::src::m_misc::M_StringCopy;
use crate::src::m_misc::M_TempFile;
use crate::src::m_misc::M_WriteFile;
use crate::src::m_misc::M_snprintf;
use crate::src::m_random::M_ClearRandom;
use crate::src::m_random::P_Random;
use crate::src::p_inter::maxammo;
use crate::src::p_map::P_CheckPosition;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::p_mobj::P_SpawnPlayer;
use crate::src::p_mobj::MF_SHADOW;
use crate::src::p_mobj::{mapthing_t, state_t, subsector_t};
use crate::src::p_mobj::{mobj_t, pspdef_t};
use crate::src::p_mobj::{MT_BRUISERSHOT, MT_HEADSHOT, MT_TFOG, MT_TROOPSHOT};
use crate::src::p_saveg::P_ArchivePlayers;
use crate::src::p_saveg::P_ArchiveSpecials;
use crate::src::p_saveg::P_ArchiveThinkers;
use crate::src::p_saveg::P_ArchiveWorld;
use crate::src::p_saveg::P_ReadSaveGameEOF;
use crate::src::p_saveg::P_ReadSaveGameHeader;
use crate::src::p_saveg::P_SaveGameFile;
use crate::src::p_saveg::P_TempSaveGameFile;
use crate::src::p_saveg::P_UnArchivePlayers;
use crate::src::p_saveg::P_UnArchiveSpecials;
use crate::src::p_saveg::P_UnArchiveThinkers;
use crate::src::p_saveg::P_UnArchiveWorld;
use crate::src::p_saveg::P_WriteSaveGameEOF;
use crate::src::p_saveg::P_WriteSaveGameHeader;
use crate::src::p_setup::P_SetupLevel;
use crate::src::p_tick::P_Ticker;
use crate::src::r_data::R_FlatNumForName;
use crate::src::r_data::R_TextureNumForName;
use crate::src::r_draw::R_FillBackScreen;
use crate::src::r_main::R_ExecuteSetViewSize;
use crate::src::r_main::R_PointInSubsector;
use crate::src::s_sound::S_PauseSound;
use crate::src::s_sound::S_ResumeSound;
use crate::src::s_sound::S_StartSound;
use crate::src::sounds::sfx_telept;
use crate::src::st_stuff::ST_Responder;
use crate::src::st_stuff::ST_Ticker;
use crate::src::statdump::StatCopy;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::tables::finetangent;
use crate::src::tables::ANG45;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::v_video::V_ScreenShot;
use crate::src::w_wad::{
    wad_name8_to_string, W_CacheLumpName, W_CheckNumForName, W_ReleaseLumpName,
};
use crate::src::wi_stuff::WI_End;
use crate::src::wi_stuff::WI_Start;
use crate::src::wi_stuff::WI_Ticker;
use crate::src::wi_stuff::{wbplayerstruct_t, wbstartstruct_t};
use crate::src::z_zone::Z_CheckHeap;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use libc::printf;
use libc::{atoi, strlen};
use libc::{memcpy, memset};

pub struct GGameState {
    pub oldgamestate: gamestate_t,
    pub gameaction: gameaction_t,
    pub gamestate: gamestate_t,
    pub gameskill: skill_t,
    pub respawnmonsters: bool,
    pub gameepisode: i32,
    pub gamemap: i32,
    pub timelimit: i32,
    pub paused: bool,
    pub sendpause: bool,
    pub sendsave: bool,
    pub usergame: bool,
    pub timingdemo: bool,
    pub nodrawers: bool,
    pub starttime: i32,
    pub viewactive: bool,
    pub deathmatch: i32,
    pub netgame: bool,
    pub playeringame: [boolean; 4],
    pub players: [player_t; 4],
    pub turbodetected: [boolean; 4],
    pub consoleplayer: i32,
    pub displayplayer: i32,
    pub levelstarttic: i32,
    pub totalsecret: i32,
    pub totalkills: i32,
    pub totalitems: i32,
    pub demoname: *mut ::core::ffi::c_char,
    pub demorecording: bool,
    pub longtics: bool,
    pub lowres_turn: bool,
    pub demoplayback: bool,
    pub netdemo: bool,
    pub demobuffer: *mut byte,
    pub demo_p: *mut byte,
    pub demoend: *mut byte,
    pub singledemo: bool,
    pub precache: bool,
    pub testcontrols: bool,
    pub testcontrols_mousespeed: i32,
    pub wminfo: wbstartstruct_t,
    pub consistancy: [[byte; 128]; 4],
    pub forwardmove: [fixed_t; 2],
    pub sidemove: [fixed_t; 2],
    pub next_weapon: i32,
    pub gamekeydown: [boolean; 256],
    pub turnheld: i32,
    pub mousearray: [boolean; 9],
    pub mousebuttons: *mut boolean,
    pub mousex: i32,
    pub mousey: i32,
    pub dclicktime: i32,
    pub dclickstate: boolean,
    pub dclicks: i32,
    pub dclicktime2: i32,
    pub dclickstate2: boolean,
    pub dclicks2: i32,
    pub joyxmove: i32,
    pub joyymove: i32,
    pub joystrafemove: i32,
    pub joyarray: [boolean; 21],
    pub joybuttons: *mut boolean,
    pub savegameslot: i32,
    pub savedescription: [::core::ffi::c_char; 32],
    pub bodyque: [*mut mobj_t; 32],
    pub bodyqueslot: i32,
    pub vanilla_savegame_limit: i32,
    pub vanilla_demo_limit: i32,
    pub secretexit: bool,
    pub savename: [::core::ffi::c_char; 256],
    pub d_skill: skill_t,
    pub d_episode: i32,
    pub d_map: i32,
    pub defdemoname: *mut ::core::ffi::c_char,
    pub g_build_ticcmd_carry: i16,
    pub g_ticker_turbomessage: [::core::ffi::c_char; 80],
    pub demo_version_description_resultbuf: [::core::ffi::c_char; 16],
}

impl GGameState {
    pub const fn new() -> Self {
        GGameState {
            oldgamestate: GS_LEVEL,
            gameaction: ga_nothing,
            gamestate: GS_LEVEL,
            gameskill: sk_baby,
            respawnmonsters: false,
            gameepisode: 0,
            gamemap: 0,
            timelimit: 0,
            paused: false,
            sendpause: false,
            sendsave: false,
            usergame: false,
            timingdemo: false,
            nodrawers: false,
            starttime: 0,
            viewactive: false,
            deathmatch: 0,
            netgame: false,
            playeringame: [0; 4],
            players: [player_s {
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
                attacker: None,
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
            }; 4],
            turbodetected: [0; 4],
            consoleplayer: 0,
            displayplayer: 0,
            levelstarttic: 0,
            totalsecret: 0,
            totalkills: 0,
            totalitems: 0,
            demoname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            demorecording: false,
            longtics: false,
            lowres_turn: false,
            demoplayback: false,
            netdemo: false,
            demobuffer: ::core::ptr::null::<byte>() as *mut byte,
            demo_p: ::core::ptr::null::<byte>() as *mut byte,
            demoend: ::core::ptr::null::<byte>() as *mut byte,
            singledemo: false,
            precache: true,
            testcontrols: false,
            testcontrols_mousespeed: 0,
            wminfo: wbstartstruct_t {
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
            },
            consistancy: [[0; 128]; 4],
            forwardmove: [0x19 as i32, 0x32 as i32],
            sidemove: [0x18 as i32, 0x28 as i32],
            next_weapon: 0,
            gamekeydown: [0; 256],
            turnheld: 0,
            mousearray: [0; 9],
            mousebuttons: ::core::ptr::null::<boolean>() as *mut boolean,
            mousex: 0,
            mousey: 0,
            dclicktime: 0,
            dclickstate: 0,
            dclicks: 0,
            dclicktime2: 0,
            dclickstate2: 0,
            dclicks2: 0,
            joyxmove: 0,
            joyymove: 0,
            joystrafemove: 0,
            joyarray: [0; 21],
            joybuttons: ::core::ptr::null::<boolean>() as *mut boolean,
            savegameslot: 0,
            savedescription: [0; 32],
            bodyque: [::core::ptr::null::<mobj_t>() as *mut mobj_t; 32],
            bodyqueslot: 0,
            vanilla_savegame_limit: 1,
            vanilla_demo_limit: 1,
            secretexit: false,
            savename: [0; 256],
            d_skill: sk_baby,
            d_episode: 0,
            d_map: 0,
            defdemoname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            g_build_ticcmd_carry: 0,
            g_ticker_turbomessage: [0; 80],
            demo_version_description_resultbuf: [0; 16],
        }
    }
}

extern "C" {
    fn remove(__filename: *const ::core::ffi::c_char) -> i32;
    fn rename(__old: *const ::core::ffi::c_char, __new: *const ::core::ffi::c_char) -> i32;
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
pub const TURBOTHRESHOLD: i32 = 0x32;
#[no_mangle]
pub static angleturn: [fixed_t; 3] = [640 as i32, 1280 as i32, 320 as i32];
static weapon_order_table: [C2RustUnnamed_5; 9] = [
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
pub const BODYQUESIZE: i32 = 32;
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
    if weapon as u32 == wp_supershotgun as u32
        && (if unsafe { game_state() }.doomstat.gamemission as u32 == pack_chex as u32 {
            doom as u32
        } else {
            (if unsafe { game_state() }.doomstat.gamemission as u32 == pack_hacx as u32 {
                doom2 as u32
            } else {
                unsafe { game_state() }.doomstat.gamemission as u32
            })
        }) == doom as u32
    {
        return false;
    }
    if (weapon as u32 == wp_plasma as u32 || weapon as u32 == wp_bfg as u32)
        && unsafe { game_state() }.doomstat.gamemission as u32 == doom as u32
        && unsafe { game_state() }.doomstat.gamemode as u32 == shareware as u32
    {
        return false;
    }
    if !unsafe { game_state() }.g_game.players
        [unsafe { game_state() }.g_game.consoleplayer as usize]
        .weaponowned[weapon as usize]
    {
        return false;
    }
    if weapon as u32 == wp_fist as u32
        && unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .weaponowned[wp_chainsaw as i32 as usize]
        && unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .powers[pw_strength as i32 as usize]
            == 0
    {
        return false;
    }
    return true;
}
unsafe fn G_NextWeapon(mut direction: i32) -> i32 {
    let mut weapon: weapontype_t = wp_fist;
    let mut start_i: i32 = 0;
    let mut i: i32 = 0;
    if unsafe { game_state() }.g_game.players[unsafe { game_state() }.g_game.consoleplayer as usize]
        .pendingweapon as u32
        == wp_nochange as u32
    {
        weapon = unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .readyweapon;
    } else {
        weapon = unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .pendingweapon;
    }
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_5; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as usize)
    {
        if weapon_order_table[i as usize].weapon as u32 == weapon as u32 {
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
        if !(i != start_i && !WeaponSelectable(weapon_order_table[i as usize].weapon)) {
            break;
        }
    }
    return weapon_order_table[i as usize].weapon_num as i32;
}
pub unsafe fn G_BuildTiccmd(mut cmd: *mut ticcmd_t, mut maketic: i32) {
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
    (*cmd).consistancy = unsafe { game_state() }.g_game.consistancy
        [unsafe { game_state() }.g_game.consoleplayer as usize][(maketic % BACKUPTICS) as usize];
    strafe = unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_strafe as usize]
        != 0
        || *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebstrafe as isize)
            != 0
        || *unsafe { game_state() }
            .g_game
            .joybuttons
            .offset(unsafe { game_state() }.m_controls.joybstrafe as isize)
            != 0;
    speed = (unsafe { game_state() }.m_controls.key_speed >= NUMKEYS
        || unsafe { game_state() }.m_controls.joybspeed >= MAX_JOY_BUTTONS
        || unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_speed as usize]
            != 0
        || *unsafe { game_state() }
            .g_game
            .joybuttons
            .offset(unsafe { game_state() }.m_controls.joybspeed as isize)
            != 0) as i32;
    side = 0 as i32;
    forward = side;
    if unsafe { game_state() }.g_game.joyxmove < 0 as i32
        || unsafe { game_state() }.g_game.joyxmove > 0 as i32
        || unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_right as usize]
            != 0
        || unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_left as usize]
            != 0
    {
        unsafe { game_state() }.g_game.turnheld += unsafe { game_state() }.d_loop.ticdup;
    } else {
        unsafe { game_state() }.g_game.turnheld = 0 as i32;
    }
    if unsafe { game_state() }.g_game.turnheld < SLOWTURNTICS {
        tspeed = 2 as i32;
    } else {
        tspeed = speed;
    }
    if strafe {
        if unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_right as usize]
            != 0
        {
            side += unsafe { game_state() }.g_game.sidemove[speed as usize] as i32;
        }
        if unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_left as usize]
            != 0
        {
            side -= unsafe { game_state() }.g_game.sidemove[speed as usize] as i32;
        }
        if unsafe { game_state() }.g_game.joyxmove > 0 as i32 {
            side += unsafe { game_state() }.g_game.sidemove[speed as usize] as i32;
        }
        if unsafe { game_state() }.g_game.joyxmove < 0 as i32 {
            side -= unsafe { game_state() }.g_game.sidemove[speed as usize] as i32;
        }
    } else {
        if unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_right as usize]
            != 0
        {
            (*cmd).angleturn = ((*cmd).angleturn as i32 - angleturn[tspeed as usize] as i32) as i16;
        }
        if unsafe { game_state() }.g_game.gamekeydown
            [unsafe { game_state() }.m_controls.key_left as usize]
            != 0
        {
            (*cmd).angleturn = ((*cmd).angleturn as i32 + angleturn[tspeed as usize] as i32) as i16;
        }
        if unsafe { game_state() }.g_game.joyxmove > 0 as i32 {
            (*cmd).angleturn = ((*cmd).angleturn as i32 - angleturn[tspeed as usize] as i32) as i16;
        }
        if unsafe { game_state() }.g_game.joyxmove < 0 as i32 {
            (*cmd).angleturn = ((*cmd).angleturn as i32 + angleturn[tspeed as usize] as i32) as i16;
        }
    }
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_up as usize]
        != 0
    {
        forward += unsafe { game_state() }.g_game.forwardmove[speed as usize] as i32;
    }
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_down as usize]
        != 0
    {
        forward -= unsafe { game_state() }.g_game.forwardmove[speed as usize] as i32;
    }
    if unsafe { game_state() }.g_game.joyymove < 0 as i32 {
        forward += unsafe { game_state() }.g_game.forwardmove[speed as usize] as i32;
    }
    if unsafe { game_state() }.g_game.joyymove > 0 as i32 {
        forward -= unsafe { game_state() }.g_game.forwardmove[speed as usize] as i32;
    }
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_strafeleft as usize]
        != 0
        || *unsafe { game_state() }
            .g_game
            .joybuttons
            .offset(unsafe { game_state() }.m_controls.joybstrafeleft as isize)
            != 0
        || *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebstrafeleft as isize)
            != 0
        || unsafe { game_state() }.g_game.joystrafemove < 0 as i32
    {
        side -= unsafe { game_state() }.g_game.sidemove[speed as usize] as i32;
    }
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_straferight as usize]
        != 0
        || *unsafe { game_state() }
            .g_game
            .joybuttons
            .offset(unsafe { game_state() }.m_controls.joybstraferight as isize)
            != 0
        || *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebstraferight as isize)
            != 0
        || unsafe { game_state() }.g_game.joystrafemove > 0 as i32
    {
        side += unsafe { game_state() }.g_game.sidemove[speed as usize] as i32;
    }
    (*cmd).chatchar = HU_dequeueChatChar() as byte;
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_fire as usize]
        != 0
        || *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebfire as isize)
            != 0
        || *unsafe { game_state() }
            .g_game
            .joybuttons
            .offset(unsafe { game_state() }.m_controls.joybfire as isize)
            != 0
    {
        (*cmd).buttons = ((*cmd).buttons as i32 | BT_ATTACK as i32) as byte;
    }
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_use as usize]
        != 0
        || *unsafe { game_state() }
            .g_game
            .joybuttons
            .offset(unsafe { game_state() }.m_controls.joybuse as isize)
            != 0
        || *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebuse as isize)
            != 0
    {
        (*cmd).buttons = ((*cmd).buttons as i32 | BT_USE as i32) as byte;
        unsafe { game_state() }.g_game.dclicks = 0 as i32;
    }
    if unsafe { game_state() }.g_game.gamestate as u32 == GS_LEVEL as u32
        && unsafe { game_state() }.g_game.next_weapon != 0 as i32
    {
        i = G_NextWeapon(unsafe { game_state() }.g_game.next_weapon);
        (*cmd).buttons = ((*cmd).buttons as i32 | BT_CHANGE as i32) as byte;
        (*cmd).buttons = ((*cmd).buttons as i32 | i << BT_WEAPONSHIFT as i32) as byte;
    } else {
        i = 0 as i32;
        while (i as usize)
            < (::core::mem::size_of::<[*mut i32; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut i32>() as usize)
        {
            let mut key: i32 = *unsafe { game_state() }.m_controls.weapon_keys[i as usize];
            if unsafe { game_state() }.g_game.gamekeydown[key as usize] != 0 {
                (*cmd).buttons = ((*cmd).buttons as i32 | BT_CHANGE as i32) as byte;
                (*cmd).buttons = ((*cmd).buttons as i32 | i << BT_WEAPONSHIFT as i32) as byte;
                break;
            } else {
                i += 1;
            }
        }
    }
    unsafe { game_state() }.g_game.next_weapon = 0 as i32;
    if *unsafe { game_state() }
        .g_game
        .mousebuttons
        .offset(unsafe { game_state() }.m_controls.mousebforward as isize)
        != 0
    {
        forward += unsafe { game_state() }.g_game.forwardmove[speed as usize] as i32;
    }
    if *unsafe { game_state() }
        .g_game
        .mousebuttons
        .offset(unsafe { game_state() }.m_controls.mousebbackward as isize)
        != 0
    {
        forward -= unsafe { game_state() }.g_game.forwardmove[speed as usize] as i32;
    }
    if unsafe { game_state() }.m_controls.dclick_use != 0 {
        if *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebforward as isize)
            != unsafe { game_state() }.g_game.dclickstate
            && unsafe { game_state() }.g_game.dclicktime > 1 as i32
        {
            unsafe { game_state() }.g_game.dclickstate = *unsafe { game_state() }
                .g_game
                .mousebuttons
                .offset(unsafe { game_state() }.m_controls.mousebforward as isize);
            if unsafe { game_state() }.g_game.dclickstate != 0 {
                unsafe { game_state() }.g_game.dclicks += 1;
            }
            if unsafe { game_state() }.g_game.dclicks == 2 as i32 {
                (*cmd).buttons = ((*cmd).buttons as i32 | BT_USE as i32) as byte;
                unsafe { game_state() }.g_game.dclicks = 0 as i32;
            } else {
                unsafe { game_state() }.g_game.dclicktime = 0 as i32;
            }
        } else {
            unsafe { game_state() }.g_game.dclicktime += unsafe { game_state() }.d_loop.ticdup;
            if unsafe { game_state() }.g_game.dclicktime > 20 as i32 {
                unsafe { game_state() }.g_game.dclicks = 0 as i32;
                unsafe { game_state() }.g_game.dclickstate = 0 as boolean;
            }
        }
        bstrafe = (*unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(unsafe { game_state() }.m_controls.mousebstrafe as isize)
            != 0
            || *unsafe { game_state() }
                .g_game
                .joybuttons
                .offset(unsafe { game_state() }.m_controls.joybstrafe as isize)
                != 0) as i32 as boolean;
        if bstrafe != unsafe { game_state() }.g_game.dclickstate2
            && unsafe { game_state() }.g_game.dclicktime2 > 1 as i32
        {
            unsafe { game_state() }.g_game.dclickstate2 = bstrafe;
            if unsafe { game_state() }.g_game.dclickstate2 != 0 {
                unsafe { game_state() }.g_game.dclicks2 += 1;
            }
            if unsafe { game_state() }.g_game.dclicks2 == 2 as i32 {
                (*cmd).buttons = ((*cmd).buttons as i32 | BT_USE as i32) as byte;
                unsafe { game_state() }.g_game.dclicks2 = 0 as i32;
            } else {
                unsafe { game_state() }.g_game.dclicktime2 = 0 as i32;
            }
        } else {
            unsafe { game_state() }.g_game.dclicktime2 += unsafe { game_state() }.d_loop.ticdup;
            if unsafe { game_state() }.g_game.dclicktime2 > 20 as i32 {
                unsafe { game_state() }.g_game.dclicks2 = 0 as i32;
                unsafe { game_state() }.g_game.dclickstate2 = 0 as boolean;
            }
        }
    }
    forward += unsafe { game_state() }.g_game.mousey;
    if strafe {
        side += unsafe { game_state() }.g_game.mousex * 2 as i32;
    } else {
        (*cmd).angleturn =
            ((*cmd).angleturn as i32 - unsafe { game_state() }.g_game.mousex * 0x8 as i32) as i16;
    }
    if unsafe { game_state() }.g_game.mousex == 0 as i32 {
        unsafe { game_state() }.g_game.testcontrols_mousespeed = 0 as i32;
    }
    unsafe { game_state() }.g_game.mousey = 0 as i32;
    unsafe { game_state() }.g_game.mousex = unsafe { game_state() }.g_game.mousey;
    if forward > unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] {
        forward = unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] as i32;
    } else if forward < -unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] {
        forward = -unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] as i32;
    }
    if side > unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] {
        side = unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] as i32;
    } else if side < -unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] {
        side = -unsafe { game_state() }.g_game.forwardmove[1 as i32 as usize] as i32;
    }
    (*cmd).forwardmove = ((*cmd).forwardmove as i32 + forward) as i8;
    (*cmd).sidemove = ((*cmd).sidemove as i32 + side) as i8;
    if unsafe { game_state() }.g_game.sendpause {
        unsafe { game_state() }.g_game.sendpause = false;
        (*cmd).buttons = (BT_SPECIAL as i32 | BTS_PAUSE as i32) as byte;
    }
    if unsafe { game_state() }.g_game.sendsave {
        unsafe { game_state() }.g_game.sendsave = false;
        (*cmd).buttons = (BT_SPECIAL as i32
            | BTS_SAVEGAME as i32
            | unsafe { game_state() }.g_game.savegameslot << BTS_SAVESHIFT as i32)
            as byte;
    }
    if unsafe { game_state() }.g_game.lowres_turn {
        let mut desired_angleturn: i16 = 0;
        desired_angleturn = ((*cmd).angleturn as i32
            + unsafe { game_state() }.g_game.g_build_ticcmd_carry as i32)
            as i16;
        (*cmd).angleturn = (desired_angleturn as i32 + 128 as i32 & 0xff00 as i32) as i16;
        unsafe { game_state() }.g_game.g_build_ticcmd_carry =
            (desired_angleturn as i32 - (*cmd).angleturn as i32) as i16;
    }
}
pub unsafe fn G_DoLoadLevel(state: &mut GameState) {
    let mut i: i32 = 0;
    state.r_sky.skyflatnum = R_FlatNumForName(
        &mut state.r_data,
        b"F_SKY1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if state.doomstat.gamemode as u32 == commercial as u32
        && (state.doomstat.gameversion as u32 == exe_final2 as u32
            || state.doomstat.gameversion as u32 == exe_chex as u32)
    {
        let mut skytexturename: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        if state.g_game.gamemap < 12 as i32 {
            skytexturename =
                b"SKY1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if state.g_game.gamemap < 21 as i32 {
            skytexturename =
                b"SKY2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else {
            skytexturename =
                b"SKY3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        skytexturename = skytexturename;
        state.r_sky.skytexture = R_TextureNumForName(&mut state.r_data, skytexturename);
    }
    state.g_game.levelstarttic = state.d_loop.gametic;
    if state.d_main.wipegamestate as u32 == GS_LEVEL as u32 {
        state.d_main.wipegamestate = 4294967295 as gamestate_t;
    }
    state.g_game.gamestate = GS_LEVEL;
    i = 0 as i32;
    while i < MAXPLAYERS {
        state.g_game.turbodetected[i as usize] = false_0 as boolean;
        if state.g_game.playeringame[i as usize] != 0
            && state.g_game.players[i as usize].playerstate as u32
                == PST_DEAD as u32
        {
            state.g_game.players[i as usize].playerstate = PST_REBORN;
        }
        memset(
            &raw mut (*(&raw mut state.g_game.players as *mut player_t)
                .offset(i as isize))
            .frags as *mut i32 as *mut ::core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<[i32; 4]>() as size_t,
        );
        i += 1;
    }
    P_SetupLevel(
        state.g_game.gameepisode,
        state.g_game.gamemap,
        0 as i32,
        state.g_game.gameskill,
    );
    state.g_game.displayplayer = state.g_game.consoleplayer;
    state.g_game.gameaction = ga_nothing;
    Z_CheckHeap(&mut state.z_zone);
    memset(
        &raw mut state.g_game.gamekeydown as *mut boolean
            as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[boolean; 256]>() as size_t,
    );
    state.g_game.joystrafemove = 0 as i32;
    state.g_game.joyymove = state.g_game.joystrafemove;
    state.g_game.joyxmove = state.g_game.joyymove;
    state.g_game.mousey = 0 as i32;
    state.g_game.mousex = state.g_game.mousey;
    state.g_game.paused = false;
    state.g_game.sendsave = state.g_game.paused;
    state.g_game.sendpause = state.g_game.sendsave;
    memset(
        &raw mut state.g_game.mousearray as *mut boolean
            as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[boolean; 9]>() as size_t,
    );
    memset(
        &raw mut state.g_game.joyarray as *mut boolean
            as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[boolean; 21]>() as size_t,
    );
    if state.g_game.testcontrols {
        state.g_game.players
            [state.g_game.consoleplayer as usize]
            .message = b"Press escape to quit.\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
}
unsafe fn SetJoyButtons(mut buttons_mask: u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAX_JOY_BUTTONS {
        let mut button_on: i32 = (buttons_mask & ((1 as i32) << i) as u32 != 0 as u32) as i32;
        if *unsafe { game_state() }.g_game.joybuttons.offset(i as isize) == 0 && button_on != 0 {
            if i == unsafe { game_state() }.m_controls.joybprevweapon {
                unsafe { game_state() }.g_game.next_weapon = -(1 as i32);
            } else if i == unsafe { game_state() }.m_controls.joybnextweapon {
                unsafe { game_state() }.g_game.next_weapon = 1 as i32;
            }
        }
        *unsafe { game_state() }.g_game.joybuttons.offset(i as isize) = button_on as boolean;
        i += 1;
    }
}
unsafe fn SetMouseButtons(mut buttons_mask: u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAX_MOUSE_BUTTONS {
        let mut button_on: u32 = (buttons_mask & ((1 as i32) << i) as u32 != 0 as u32) as u32;
        if *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(i as isize)
            == 0
            && button_on != 0
        {
            if i == unsafe { game_state() }.m_controls.mousebprevweapon {
                unsafe { game_state() }.g_game.next_weapon = -(1 as i32);
            } else if i == unsafe { game_state() }.m_controls.mousebnextweapon {
                unsafe { game_state() }.g_game.next_weapon = 1 as i32;
            }
        }
        *unsafe { game_state() }
            .g_game
            .mousebuttons
            .offset(i as isize) = button_on as boolean;
        i += 1;
    }
}
pub unsafe fn G_Responder(mut ev: event_t) -> bool {
    if unsafe { game_state() }.g_game.gamestate == GS_LEVEL
        && ev.type_0 as u32 == ev_keydown as u32
        && ev.data1 == unsafe { game_state() }.m_controls.key_spy
        && (unsafe { game_state() }.g_game.singledemo
            || unsafe { game_state() }.g_game.deathmatch == 0)
    {
        loop {
            unsafe { game_state() }.g_game.displayplayer += 1;
            if unsafe { game_state() }.g_game.displayplayer == MAXPLAYERS {
                unsafe { game_state() }.g_game.displayplayer = 0 as i32;
            }
            if !(unsafe { game_state() }.g_game.playeringame
                [unsafe { game_state() }.g_game.displayplayer as usize]
                == 0
                && unsafe { game_state() }.g_game.displayplayer
                    != unsafe { game_state() }.g_game.consoleplayer)
            {
                break;
            }
        }
        return true;
    }
    if unsafe { game_state() }.g_game.gameaction == ga_nothing
        && !unsafe { game_state() }.g_game.singledemo
        && (unsafe { game_state() }.g_game.demoplayback
            || unsafe { game_state() }.g_game.gamestate == GS_DEMOSCREEN)
    {
        if ev.type_0 == ev_keydown
            || ev.type_0 == ev_mouse && ev.data1 != 0
            || ev.type_0 == ev_joystick && ev.data1 != 0
        {
            M_StartControlPanel();
            return true;
        }
        return false;
    }
    if unsafe { game_state() }.g_game.gamestate == GS_LEVEL {
        if HU_Responder(&ev) {
            return true;
        }
        if ST_Responder(&ev) {
            return true;
        }
        if AM_Responder(&ev) {
            return true;
        }
    }
    if unsafe { game_state() }.g_game.gamestate == GS_FINALE {
        if F_Responder(unsafe { game_state() }, &ev) {
            return true;
        }
    }
    if unsafe { game_state() }.g_game.testcontrols && ev.type_0 == ev_mouse {
        unsafe { game_state() }.g_game.testcontrols_mousespeed = (ev.data2).abs();
    }
    if ev.type_0 == ev_keydown && ev.data1 == unsafe { game_state() }.m_controls.key_prevweapon {
        unsafe { game_state() }.g_game.next_weapon = -1;
    } else if ev.type_0 == ev_keydown
        && ev.data1 == unsafe { game_state() }.m_controls.key_nextweapon
    {
        unsafe { game_state() }.g_game.next_weapon = 1;
    }
    match ev.type_0 as u32 {
        0 => {
            if ev.data1 == unsafe { game_state() }.m_controls.key_pause {
                unsafe { game_state() }.g_game.sendpause = true;
            } else if ev.data1 < NUMKEYS {
                unsafe { game_state() }.g_game.gamekeydown[ev.data1 as usize] = true_0 as boolean;
            }
            return true;
        }
        1 => {
            if ev.data1 < NUMKEYS {
                unsafe { game_state() }.g_game.gamekeydown[ev.data1 as usize] = false_0 as boolean;
            }
            return false;
        }
        2 => {
            SetMouseButtons(ev.data1 as u32);
            unsafe { game_state() }.g_game.mousex =
                ev.data2 * (unsafe { game_state() }.m_menu.mouseSensitivity + 5 as i32) / 10 as i32;
            unsafe { game_state() }.g_game.mousey =
                ev.data3 * (unsafe { game_state() }.m_menu.mouseSensitivity + 5 as i32) / 10 as i32;
            return true;
        }
        3 => {
            SetJoyButtons(ev.data1 as u32);
            unsafe { game_state() }.g_game.joyxmove = ev.data2;
            unsafe { game_state() }.g_game.joyymove = ev.data3;
            unsafe { game_state() }.g_game.joystrafemove = ev.data4;
            return true;
        }
        _ => {}
    }
    return false;
}
pub unsafe fn G_Ticker(state: &mut GameState) {
    let mut i: i32 = 0;
    let mut buf: i32 = 0;
    let mut cmd: *mut ticcmd_t = ::core::ptr::null_mut::<ticcmd_t>();
    i = 0 as i32;
    while i < MAXPLAYERS {
        if state.g_game.playeringame[i as usize] != 0
            && state.g_game.players[i as usize].playerstate as u32
                == PST_REBORN as u32
        {
            G_DoReborn(state, i);
        }
        i += 1;
    }
    while state.g_game.gameaction as u32 != ga_nothing as u32 {
        match state.g_game.gameaction as u32 {
            1 => {
                G_DoLoadLevel(state);
            }
            2 => {
                G_DoNewGame(state);
            }
            3 => {
                G_DoLoadGame(state);
            }
            4 => {
                G_DoSaveGame(state);
            }
            5 => {
                G_DoPlayDemo();
            }
            6 => {
                G_DoCompleted();
            }
            7 => {
                F_StartFinale(state);
            }
            8 => {
                G_DoWorldDone(state);
            }
            9 => {
                V_ScreenShot(
                    state,
                    b"DOOM%02i.%s\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                state.g_game.players
                    [state.g_game.consoleplayer as usize]
                    .message = b"screen shot\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state.g_game.gameaction = ga_nothing;
            }
            0 | _ => {}
        }
    }
    buf =
        state.d_loop.gametic / state.d_loop.ticdup % BACKUPTICS;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if state.g_game.playeringame[i as usize] != 0 {
            cmd = &raw mut (*(&raw mut state.g_game.players as *mut player_t)
                .offset(i as isize))
            .cmd;
            memcpy(
                cmd as *mut ::core::ffi::c_void,
                state.d_net.netcmds.offset(i as isize) as *mut ticcmd_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<ticcmd_t>() as size_t,
            );
            if state.g_game.demoplayback {
                G_ReadDemoTiccmd(cmd);
            }
            if state.g_game.demorecording {
                G_WriteDemoTiccmd(cmd);
            }
            if (*cmd).forwardmove as i32 > TURBOTHRESHOLD {
                state.g_game.turbodetected[i as usize] = true_0 as boolean;
            }
            if state.d_loop.gametic & 31 as i32 == 0 as i32
                && (state.d_loop.gametic >> 5 as i32) % MAXPLAYERS == i
                && state.g_game.turbodetected[i as usize] != 0
            {
                M_snprintf(
                    &raw mut state.g_game.g_ticker_turbomessage
                        as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
                    b"%s is turbo!\0" as *const u8 as *const ::core::ffi::c_char,
                    player_names[i as usize],
                );
                state.g_game.players
                    [state.g_game.consoleplayer as usize]
                    .message = &raw mut state.g_game.g_ticker_turbomessage
                    as *mut ::core::ffi::c_char;
                state.g_game.turbodetected[i as usize] = false_0 as boolean;
            }
            if state.g_game.netgame
                && !state.g_game.netdemo
                && state.d_loop.gametic % state.d_loop.ticdup
                    == 0
            {
                if state.d_loop.gametic > BACKUPTICS
                    && state.g_game.consistancy[i as usize][buf as usize] as i32
                        != (*cmd).consistancy as i32
                {
                    I_Error(&format!(
                        "consistency failure ({} should be {})",
                        (*cmd).consistancy as i32,
                        state.g_game.consistancy[i as usize][buf as usize] as i32,
                    ));
                }
                if !state.g_game.players[i as usize]
                    .mo
                    .is_null()
                {
                    state.g_game.consistancy[i as usize][buf as usize] =
                        (*state.g_game.players[i as usize].mo).x as byte;
                } else {
                    state.g_game.consistancy[i as usize][buf as usize] =
                        state.m_random.rndindex as byte;
                }
            }
        }
        i += 1;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        if state.g_game.playeringame[i as usize] != 0 {
            if state.g_game.players[i as usize]
                .cmd
                .buttons as i32
                & BT_SPECIAL as i32
                != 0
            {
                match state.g_game.players[i as usize]
                    .cmd
                    .buttons as i32
                    & BT_SPECIALMASK as i32
                {
                    1 => {
                        state.g_game.paused =
                            !state.g_game.paused;
                        if state.g_game.paused {
                            S_PauseSound(state);
                        } else {
                            S_ResumeSound(state);
                        }
                    }
                    2 => {
                        if state.g_game.savedescription[0 as i32 as usize] == 0 {
                            M_StringCopy(
                                &raw mut state.g_game.savedescription
                                    as *mut ::core::ffi::c_char,
                                b"NET GAME\0" as *const u8 as *const ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
                            );
                        }
                        state.g_game.savegameslot =
                            (state.g_game.players[i as usize]
                                .cmd
                                .buttons as i32
                                & BTS_SAVEMASK as i32)
                                >> BTS_SAVESHIFT as i32;
                        state.g_game.gameaction = ga_savegame;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    if state.g_game.oldgamestate as u32 == GS_INTERMISSION as u32
        && state.g_game.gamestate as u32 != GS_INTERMISSION as u32
    {
        WI_End();
    }
    state.g_game.oldgamestate = state.g_game.gamestate;
    match state.g_game.gamestate as u32 {
        0 => {
            P_Ticker(state);
            ST_Ticker(state);
            AM_Ticker(state);
            HU_Ticker();
        }
        1 => {
            WI_Ticker(state);
        }
        2 => {
            F_Ticker(state);
        }
        3 => {
            D_PageTicker(state);
        }
        _ => {}
    };
}
pub unsafe fn G_InitPlayer(mut player: i32) {
    G_PlayerReborn(&mut unsafe { game_state() }.g_game, player);
}
pub unsafe fn G_PlayerFinishLevel(state: &mut GGameState, mut player: i32) {
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    p = (&raw mut state.players as *mut player_t).offset(player as isize)
        as *mut player_t;
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
pub unsafe fn G_PlayerReborn(state: &mut GGameState, mut player: i32) {
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut i: i32 = 0;
    let mut frags: [i32; 4] = [0; 4];
    let mut killcount: i32 = 0;
    let mut itemcount: i32 = 0;
    let mut secretcount: i32 = 0;
    memcpy(
        &raw mut frags as *mut i32 as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut state.players as *mut player_t)
            .offset(player as isize))
        .frags as *mut i32 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[i32; 4]>() as size_t,
    );
    killcount = state.players[player as usize].killcount;
    itemcount = state.players[player as usize].itemcount;
    secretcount = state.players[player as usize].secretcount;
    p = (&raw mut state.players as *mut player_t).offset(player as isize)
        as *mut player_t;
    memset(
        p as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<player_t>() as size_t,
    );
    memcpy(
        &raw mut (*(&raw mut state.players as *mut player_t)
            .offset(player as isize))
        .frags as *mut i32 as *mut ::core::ffi::c_void,
        &raw mut frags as *mut i32 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[i32; 4]>() as size_t,
    );
    state.players[player as usize].killcount = killcount;
    state.players[player as usize].itemcount = itemcount;
    state.players[player as usize].secretcount = secretcount;
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
pub unsafe fn G_CheckSpot(state: &mut GameState, mut playernum: i32, mut mthing: *mut mapthing_t) -> bool {
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut i: i32 = 0;
    if state.g_game.players[playernum as usize]
        .mo
        .is_null()
    {
        i = 0 as i32;
        while i < playernum {
            if (*state.g_game.players[i as usize].mo).x
                == ((*mthing).x as i32) << FRACBITS
                && (*state.g_game.players[i as usize].mo).y
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
    if !P_CheckPosition(
        state.g_game.players[playernum as usize].mo,
        x,
        y,
    ) {
        return false;
    }
    if state.g_game.bodyqueslot >= BODYQUESIZE {
        P_RemoveMobj(
            &mut state.p_mobj,
            state.g_game.bodyque
                [(state.g_game.bodyqueslot % BODYQUESIZE) as usize],
        );
    }
    state.g_game.bodyque
        [(state.g_game.bodyqueslot % BODYQUESIZE) as usize] =
        state.g_game.players[playernum as usize].mo;
    state.g_game.bodyqueslot += 1;
    ss = R_PointInSubsector(x, y);
    let mut xa: fixed_t = 0;
    let mut ya: fixed_t = 0;
    let mut an: i32 = 0;
    an = (ANG45 >> ANGLETOFINESHIFT) * ((*mthing).angle as i32 / 45 as i32);
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
        (*state.p_setup.sector_mut((*ss).sector)).floorheight,
        MT_TFOG,
    );
    if state.g_game.players[state.g_game.consoleplayer as usize]
        .viewz
        != 1 as i32
    {
        S_StartSound(
            &mut state.sounds,
            mo as *mut ::core::ffi::c_void,
            sfx_telept as i32,
        );
    }
    return true;
}
pub unsafe fn G_DeathMatchSpawnPlayer(state: &mut GameState, mut playernum: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut selections: i32 = 0;
    selections =
        state.p_setup.deathmatch_p.offset_from(&raw mut state.p_setup.deathmatchstarts as *mut mapthing_t) as i64 as i32;
    if selections < 4 as i32 {
        I_Error(&format!("Only {} deathmatch spots, 4 required", selections));
    }
    j = 0 as i32;
    while j < 20 as i32 {
        i = P_Random(&mut state.m_random) % selections;
        let dm_spot = (&raw mut state.p_setup.deathmatchstarts as *mut mapthing_t).offset(i as isize) as *mut mapthing_t;
        if G_CheckSpot(state, playernum, dm_spot) {
            state.p_setup.deathmatchstarts[i as usize].type_0 = (playernum + 1 as i32) as i16;
            let dm_spot = (&raw mut state.p_setup.deathmatchstarts as *mut mapthing_t).offset(i as isize)
                as *mut mapthing_t;
            P_SpawnPlayer(state, dm_spot);
            return;
        }
        j += 1;
    }
    let spot = (&raw mut state.p_setup.playerstarts as *mut mapthing_t).offset(playernum as isize) as *mut mapthing_t;
    P_SpawnPlayer(state, spot);
}
pub unsafe fn G_DoReborn(state: &mut GameState, mut playernum: i32) {
    let mut i: i32 = 0;
    if !state.g_game.netgame {
        state.g_game.gameaction = ga_loadlevel;
    } else {
        (*state.g_game.players[playernum as usize].mo).player =
            ::core::ptr::null_mut::<player_s>();
        if state.g_game.deathmatch != 0 {
            G_DeathMatchSpawnPlayer(state, playernum);
            return;
        }
        let spot = (&raw mut state.p_setup.playerstarts as *mut mapthing_t).offset(playernum as isize)
            as *mut mapthing_t;
        if G_CheckSpot(state, playernum, spot) {
            let spot = (&raw mut state.p_setup.playerstarts as *mut mapthing_t).offset(playernum as isize)
                as *mut mapthing_t;
            P_SpawnPlayer(state, spot);
            return;
        }
        i = 0 as i32;
        while i < MAXPLAYERS {
            let spot = (&raw mut state.p_setup.playerstarts as *mut mapthing_t).offset(i as isize) as *mut mapthing_t;
            if G_CheckSpot(state, playernum, spot) {
                state.p_setup.playerstarts[i as usize].type_0 = (playernum + 1 as i32) as i16;
                let spot = (&raw mut state.p_setup.playerstarts as *mut mapthing_t).offset(i as isize)
                    as *mut mapthing_t;
                P_SpawnPlayer(state, spot);
                state.p_setup.playerstarts[i as usize].type_0 = (i + 1 as i32) as i16;
                return;
            }
            i += 1;
        }
        let spot = (&raw mut state.p_setup.playerstarts as *mut mapthing_t).offset(playernum as isize)
            as *mut mapthing_t;
        P_SpawnPlayer(state, spot);
    };
}
pub unsafe fn G_ScreenShot(state: &mut GameState) {
    state.g_game.gameaction = ga_screenshot;
}
#[no_mangle]
pub static pars: [[i32; 10]; 4] = [
    [0 as i32; 10],
    [
        0 as i32, 30 as i32, 75 as i32, 120 as i32, 90 as i32, 165 as i32, 180 as i32, 180 as i32,
        30 as i32, 165 as i32,
    ],
    [
        0 as i32, 90 as i32, 90 as i32, 90 as i32, 120 as i32, 90 as i32, 360 as i32, 240 as i32,
        30 as i32, 170 as i32,
    ],
    [
        0 as i32, 90 as i32, 45 as i32, 90 as i32, 150 as i32, 90 as i32, 90 as i32, 165 as i32,
        30 as i32, 135 as i32,
    ],
];
#[no_mangle]
pub static cpars: [i32; 32] = [
    30 as i32, 90 as i32, 120 as i32, 120 as i32, 90 as i32, 150 as i32, 120 as i32, 120 as i32,
    270 as i32, 90 as i32, 210 as i32, 150 as i32, 150 as i32, 150 as i32, 210 as i32, 150 as i32,
    420 as i32, 150 as i32, 210 as i32, 150 as i32, 240 as i32, 150 as i32, 180 as i32, 150 as i32,
    150 as i32, 300 as i32, 330 as i32, 420 as i32, 300 as i32, 180 as i32, 120 as i32, 30 as i32,
];
pub unsafe fn G_ExitLevel(state: &mut GameState) {
    state.g_game.secretexit = false;
    state.g_game.gameaction = ga_completed;
}
pub unsafe fn G_SecretExitLevel(state: &mut GameState) {
    if state.doomstat.gamemode as u32 == commercial as u32 && W_CheckNumForName("map31") < 0 as i32
    {
        state.g_game.secretexit = false;
    } else {
        state.g_game.secretexit = true;
    }
    state.g_game.gameaction = ga_completed;
}
pub unsafe fn G_DoCompleted() {
    let mut i: i32 = 0;
    unsafe { game_state() }.g_game.gameaction = ga_nothing;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if unsafe { game_state() }.g_game.playeringame[i as usize] != 0 {
            G_PlayerFinishLevel(&mut unsafe { game_state() }.g_game, i);
        }
        i += 1;
    }
    if unsafe { game_state() }.am_map.automapactive {
        AM_Stop();
    }
    if unsafe { game_state() }.doomstat.gamemode as u32 != commercial as u32 {
        if unsafe { game_state() }.doomstat.gameversion as u32 == exe_chex as u32 {
            if unsafe { game_state() }.g_game.gamemap == 5 as i32 {
                unsafe { game_state() }.g_game.gameaction = ga_victory;
                return;
            }
        } else {
            match unsafe { game_state() }.g_game.gamemap {
                8 => {
                    unsafe { game_state() }.g_game.gameaction = ga_victory;
                    return;
                }
                9 => {
                    i = 0 as i32;
                    while i < MAXPLAYERS {
                        unsafe { game_state() }.g_game.players[i as usize].didsecret = true;
                        i += 1;
                    }
                }
                _ => {}
            }
        }
    }
    if unsafe { game_state() }.g_game.gamemap == 8 as i32
        && unsafe { game_state() }.doomstat.gamemode as u32 != commercial as u32
    {
        unsafe { game_state() }.g_game.gameaction = ga_victory;
        return;
    }
    if unsafe { game_state() }.g_game.gamemap == 9 as i32
        && unsafe { game_state() }.doomstat.gamemode as u32 != commercial as u32
    {
        i = 0 as i32;
        while i < MAXPLAYERS {
            unsafe { game_state() }.g_game.players[i as usize].didsecret = true;
            i += 1;
        }
    }
    unsafe { game_state() }.g_game.wminfo.didsecret = unsafe { game_state() }.g_game.players
        [unsafe { game_state() }.g_game.consoleplayer as usize]
        .didsecret;
    unsafe { game_state() }.g_game.wminfo.epsd =
        unsafe { game_state() }.g_game.gameepisode - 1 as i32;
    unsafe { game_state() }.g_game.wminfo.last = unsafe { game_state() }.g_game.gamemap - 1 as i32;
    if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as u32 {
        if unsafe { game_state() }.g_game.secretexit {
            match unsafe { game_state() }.g_game.gamemap {
                15 => {
                    unsafe { game_state() }.g_game.wminfo.next = 30 as i32;
                }
                31 => {
                    unsafe { game_state() }.g_game.wminfo.next = 31 as i32;
                }
                _ => {}
            }
        } else {
            match unsafe { game_state() }.g_game.gamemap {
                31 | 32 => {
                    unsafe { game_state() }.g_game.wminfo.next = 15 as i32;
                }
                _ => {
                    unsafe { game_state() }.g_game.wminfo.next =
                        unsafe { game_state() }.g_game.gamemap;
                }
            }
        }
    } else if unsafe { game_state() }.g_game.secretexit {
        unsafe { game_state() }.g_game.wminfo.next = 8 as i32;
    } else if unsafe { game_state() }.g_game.gamemap == 9 as i32 {
        match unsafe { game_state() }.g_game.gameepisode {
            1 => {
                unsafe { game_state() }.g_game.wminfo.next = 3 as i32;
            }
            2 => {
                unsafe { game_state() }.g_game.wminfo.next = 5 as i32;
            }
            3 => {
                unsafe { game_state() }.g_game.wminfo.next = 6 as i32;
            }
            4 => {
                unsafe { game_state() }.g_game.wminfo.next = 2 as i32;
            }
            _ => {}
        }
    } else {
        unsafe { game_state() }.g_game.wminfo.next = unsafe { game_state() }.g_game.gamemap;
    }
    unsafe { game_state() }.g_game.wminfo.maxkills = unsafe { game_state() }.g_game.totalkills;
    unsafe { game_state() }.g_game.wminfo.maxitems = unsafe { game_state() }.g_game.totalitems;
    unsafe { game_state() }.g_game.wminfo.maxsecret = unsafe { game_state() }.g_game.totalsecret;
    unsafe { game_state() }.g_game.wminfo.maxfrags = 0 as i32;
    if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as u32 {
        unsafe { game_state() }.g_game.wminfo.partime =
            TICRATE * cpars[(unsafe { game_state() }.g_game.gamemap - 1 as i32) as usize];
    } else if unsafe { game_state() }.g_game.gameepisode < 4 as i32 {
        unsafe { game_state() }.g_game.wminfo.partime = TICRATE
            * pars[unsafe { game_state() }.g_game.gameepisode as usize]
                [unsafe { game_state() }.g_game.gamemap as usize];
    } else {
        unsafe { game_state() }.g_game.wminfo.partime =
            TICRATE * cpars[unsafe { game_state() }.g_game.gamemap as usize];
    }
    unsafe { game_state() }.g_game.wminfo.pnum = unsafe { game_state() }.g_game.consoleplayer;
    i = 0 as i32;
    while i < MAXPLAYERS {
        unsafe { game_state() }.g_game.wminfo.plyr[i as usize].in_0 =
            unsafe { game_state() }.g_game.playeringame[i as usize] != 0;
        unsafe { game_state() }.g_game.wminfo.plyr[i as usize].skills =
            unsafe { game_state() }.g_game.players[i as usize].killcount;
        unsafe { game_state() }.g_game.wminfo.plyr[i as usize].sitems =
            unsafe { game_state() }.g_game.players[i as usize].itemcount;
        unsafe { game_state() }.g_game.wminfo.plyr[i as usize].ssecret =
            unsafe { game_state() }.g_game.players[i as usize].secretcount;
        unsafe { game_state() }.g_game.wminfo.plyr[i as usize].stime = unsafe { game_state() }.p_tick.leveltime;
        memcpy(
            &raw mut (*(&raw mut unsafe { game_state() }.g_game.wminfo.plyr
                as *mut wbplayerstruct_t)
                .offset(i as isize))
            .frags as *mut i32 as *mut ::core::ffi::c_void,
            &raw mut (*(&raw mut unsafe { game_state() }.g_game.players as *mut player_t)
                .offset(i as isize))
            .frags as *mut i32 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[i32; 4]>() as size_t,
        );
        i += 1;
    }
    unsafe { game_state() }.g_game.gamestate = GS_INTERMISSION;
    unsafe { game_state() }.g_game.viewactive = false;
    unsafe { game_state() }.am_map.automapactive = false;
    StatCopy(
        unsafe { &mut game_state().statdump },
        &raw mut unsafe { game_state() }.g_game.wminfo,
    );
    WI_Start(&raw mut unsafe { game_state() }.g_game.wminfo);
}
pub unsafe fn G_WorldDone(state: &mut GameState) {
    state.g_game.gameaction = ga_worlddone;
    if state.g_game.secretexit {
        state.g_game.players
            [state.g_game.consoleplayer as usize]
            .didsecret = true;
    }
    if state.doomstat.gamemode as u32 == commercial as u32 {
        let mut current_block_3: u64;
        match state.g_game.gamemap {
            15 | 31 => {
                if !state.g_game.secretexit {
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
                F_StartFinale(state);
            }
            _ => {}
        }
    }
}
pub unsafe fn G_DoWorldDone(state: &mut GameState) {
    state.g_game.gamestate = GS_LEVEL;
    state.g_game.gamemap = state.g_game.wminfo.next + 1 as i32;
    G_DoLoadLevel(state);
    state.g_game.gameaction = ga_nothing;
    state.g_game.viewactive = true;
}
pub unsafe fn G_LoadGame(state: &mut GameState, mut name: *mut ::core::ffi::c_char) {
    M_StringCopy(
        &raw mut state.g_game.savename as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
    );
    state.g_game.gameaction = ga_loadgame;
}
pub unsafe fn G_DoLoadGame(state: &mut GameState) {
    let mut savedleveltime: i32 = 0;
    state.g_game.gameaction = ga_nothing;
    state.p_saveg.save_stream = fopen(
        &raw mut state.g_game.savename as *mut ::core::ffi::c_char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if state.p_saveg.save_stream.is_null() {
        return;
    }
    state.p_saveg.savegame_error = false;
    if !P_ReadSaveGameHeader() {
        fclose(state.p_saveg.save_stream);
        return;
    }
    savedleveltime = state.p_tick.leveltime;
    G_InitNew(
        state.g_game.gameskill,
        state.g_game.gameepisode,
        state.g_game.gamemap,
    );
    state.p_tick.leveltime = savedleveltime;
    P_UnArchivePlayers();
    P_UnArchiveWorld();
    P_UnArchiveThinkers();
    P_UnArchiveSpecials(&mut state.p_ceilng);
    if !P_ReadSaveGameEOF() {
        I_Error("Bad savegame");
    }
    fclose(state.p_saveg.save_stream);
    if state.r_main.setsizeneeded {
        R_ExecuteSetViewSize(state);
    }
    R_FillBackScreen(state);
}
pub unsafe fn G_SaveGame(state: &mut GameState, mut slot: i32, mut description: *mut ::core::ffi::c_char) {
    state.g_game.savegameslot = slot;
    M_StringCopy(
        &raw mut state.g_game.savedescription as *mut ::core::ffi::c_char,
        description,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
    );
    state.g_game.sendsave = true;
}
pub unsafe fn G_DoSaveGame(state: &mut GameState) {
    let mut savegame_file: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut temp_savegame_file: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut recovery_savegame_file: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    recovery_savegame_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    temp_savegame_file = P_TempSaveGameFile();
    savegame_file = P_SaveGameFile(state.g_game.savegameslot);
    state.p_saveg.save_stream = fopen(
        temp_savegame_file,
        b"wb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if state.p_saveg.save_stream.is_null() {
        recovery_savegame_file = M_TempFile(
            b"recovery.dsg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        state.p_saveg.save_stream = fopen(
            recovery_savegame_file,
            b"wb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if state.p_saveg.save_stream.is_null() {
            I_Error(&format!(
                "Failed to open either '{}' or '{}' to write savegame.",
                ::std::ffi::CStr::from_ptr(temp_savegame_file)
                    .to_str()
                    .unwrap(),
                ::std::ffi::CStr::from_ptr(recovery_savegame_file)
                    .to_str()
                    .unwrap(),
            ));
        }
    }
    state.p_saveg.savegame_error = false;
    P_WriteSaveGameHeader(
        &raw mut state.g_game.savedescription as *mut ::core::ffi::c_char,
    );
    P_ArchivePlayers();
    P_ArchiveWorld();
    P_ArchiveThinkers();
    P_ArchiveSpecials(&mut state.p_ceilng);
    P_WriteSaveGameEOF();
    if state.g_game.vanilla_savegame_limit != 0
        && ftell(state.p_saveg.save_stream) > SAVEGAMESIZE as i64
    {
        I_Error("Savegame buffer overrun");
    }
    fclose(state.p_saveg.save_stream);
    if !recovery_savegame_file.is_null() {
        I_Error(&format!(
            "Failed to open savegame file '{}' for writing.\nBut your game has been saved to '{}' for recovery.",
            ::std::ffi::CStr::from_ptr(temp_savegame_file).to_str().unwrap(),
            ::std::ffi::CStr::from_ptr(recovery_savegame_file).to_str().unwrap(),
        ));
    }
    remove(savegame_file);
    rename(temp_savegame_file, savegame_file);
    state.g_game.gameaction = ga_nothing;
    M_StringCopy(
        &raw mut state.g_game.savedescription as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
    );
    state.g_game.players[state.g_game.consoleplayer as usize]
        .message =
        b"game saved.\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    R_FillBackScreen(state);
}
pub unsafe fn G_DeferedInitNew(state: &mut GameState, mut skill: skill_t, mut episode: i32, mut map: i32) {
    state.g_game.d_skill = skill;
    state.g_game.d_episode = episode;
    state.g_game.d_map = map;
    state.g_game.gameaction = ga_newgame;
}
pub unsafe fn G_DoNewGame(state: &mut GameState) {
    state.g_game.demoplayback = false;
    state.g_game.netdemo = false;
    state.g_game.netgame = false;
    state.g_game.deathmatch = false_0;
    state.g_game.playeringame[3 as i32 as usize] = 0 as boolean;
    state.g_game.playeringame[2 as i32 as usize] =
        state.g_game.playeringame[3 as i32 as usize];
    state.g_game.playeringame[1 as i32 as usize] =
        state.g_game.playeringame[2 as i32 as usize];
    state.d_main.respawnparm = false;
    state.d_main.fastparm = false;
    state.d_main.nomonsters = false;
    state.g_game.consoleplayer = 0 as i32;
    G_InitNew(
        state.g_game.d_skill,
        state.g_game.d_episode,
        state.g_game.d_map,
    );
    state.g_game.gameaction = ga_nothing;
}
pub unsafe fn G_InitNew(mut skill: skill_t, mut episode: i32, mut map: i32) {
    let mut skytexturename: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: i32 = 0;
    if unsafe { game_state() }.g_game.paused {
        unsafe { game_state() }.g_game.paused = false;
        S_ResumeSound(unsafe { game_state() });
    }
    if skill as i32 > sk_nightmare as i32 {
        skill = sk_nightmare;
    }
    if unsafe { game_state() }.doomstat.gameversion as u32 >= exe_ultimate as u32 {
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
    if episode > 1 as i32 && unsafe { game_state() }.doomstat.gamemode as u32 == shareware as u32 {
        episode = 1 as i32;
    }
    if map < 1 as i32 {
        map = 1 as i32;
    }
    if map > 9 as i32 && unsafe { game_state() }.doomstat.gamemode as u32 != commercial as u32 {
        map = 9 as i32;
    }
    M_ClearRandom(unsafe { &mut game_state().m_random });
    if skill as i32 == sk_nightmare as i32 || unsafe { game_state() }.d_main.respawnparm {
        unsafe { game_state() }.g_game.respawnmonsters = true;
    } else {
        unsafe { game_state() }.g_game.respawnmonsters = false;
    }
    if unsafe { game_state() }.d_main.fastparm
        || skill as i32 == sk_nightmare as i32
            && unsafe { game_state() }.g_game.gameskill as i32 != sk_nightmare as i32
    {
        i = S_SARG_RUN1 as i32;
        while i <= S_SARG_PAIN2 as i32 {
            unsafe { game_state() }.info.states[i as usize].tics >>= 1 as i32;
            i += 1;
        }
        unsafe { game_state() }.info.mobjinfo[MT_BRUISERSHOT as i32 as usize].speed = 20 as i32 * FRACUNIT;
        unsafe { game_state() }.info.mobjinfo[MT_HEADSHOT as i32 as usize].speed = 20 as i32 * FRACUNIT;
        unsafe { game_state() }.info.mobjinfo[MT_TROOPSHOT as i32 as usize].speed = 20 as i32 * FRACUNIT;
    } else if skill as i32 != sk_nightmare as i32
        && unsafe { game_state() }.g_game.gameskill as i32 == sk_nightmare as i32
    {
        i = S_SARG_RUN1 as i32;
        while i <= S_SARG_PAIN2 as i32 {
            unsafe { game_state() }.info.states[i as usize].tics <<= 1 as i32;
            i += 1;
        }
        unsafe { game_state() }.info.mobjinfo[MT_BRUISERSHOT as i32 as usize].speed = 15 as i32 * FRACUNIT;
        unsafe { game_state() }.info.mobjinfo[MT_HEADSHOT as i32 as usize].speed = 10 as i32 * FRACUNIT;
        unsafe { game_state() }.info.mobjinfo[MT_TROOPSHOT as i32 as usize].speed = 10 as i32 * FRACUNIT;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        unsafe { game_state() }.g_game.players[i as usize].playerstate = PST_REBORN;
        i += 1;
    }
    unsafe { game_state() }.g_game.usergame = true;
    unsafe { game_state() }.g_game.paused = false;
    unsafe { game_state() }.g_game.demoplayback = false;
    unsafe { game_state() }.am_map.automapactive = false;
    unsafe { game_state() }.g_game.viewactive = true;
    unsafe { game_state() }.g_game.gameepisode = episode;
    unsafe { game_state() }.g_game.gamemap = map;
    unsafe { game_state() }.g_game.gameskill = skill;
    unsafe { game_state() }.g_game.viewactive = true;
    if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as u32 {
        if unsafe { game_state() }.g_game.gamemap < 12 as i32 {
            skytexturename =
                b"SKY1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if unsafe { game_state() }.g_game.gamemap < 21 as i32 {
            skytexturename =
                b"SKY2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else {
            skytexturename =
                b"SKY3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
    } else {
        match unsafe { game_state() }.g_game.gameepisode {
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
    unsafe { game_state() }.r_sky.skytexture = R_TextureNumForName(unsafe { &mut game_state().r_data }, skytexturename);
    G_DoLoadLevel(unsafe { game_state() });
}
pub const DEMOMARKER: i32 = 0x80;
pub unsafe fn G_ReadDemoTiccmd(mut cmd: *mut ticcmd_t) {
    if *unsafe { game_state() }.g_game.demo_p as i32 == DEMOMARKER {
        G_CheckDemoStatus();
        return;
    }
    let fresh18 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    (*cmd).forwardmove = *fresh18 as i8;
    let fresh19 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    (*cmd).sidemove = *fresh19 as i8;
    if unsafe { game_state() }.g_game.longtics {
        let fresh20 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        (*cmd).angleturn = *fresh20 as i16;
        let fresh21 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        (*cmd).angleturn = ((*cmd).angleturn as i32 | (*fresh21 as i32) << 8 as i32) as i16;
    } else {
        let fresh22 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        (*cmd).angleturn = ((*fresh22 as u8 as i32) << 8 as i32) as i16;
    }
    let fresh23 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    (*cmd).buttons = *fresh23 as u8 as byte;
}
unsafe fn IncreaseDemoBuffer() {
    let mut current_length: i32 = 0;
    let mut new_demobuffer: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut new_demop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut new_length: i32 = 0;
    current_length = unsafe { game_state() }
        .g_game
        .demoend
        .offset_from(unsafe { game_state() }.g_game.demobuffer) as i64 as i32;
    new_length = current_length * 2 as i32;
    new_demobuffer = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        new_length,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut byte;
    new_demop = new_demobuffer.offset(
        unsafe { game_state() }
            .g_game
            .demo_p
            .offset_from(unsafe { game_state() }.g_game.demobuffer) as i64 as isize,
    );
    memcpy(
        new_demobuffer as *mut ::core::ffi::c_void,
        unsafe { game_state() }.g_game.demobuffer as *const ::core::ffi::c_void,
        current_length as size_t,
    );
    Z_Free(
        unsafe { &mut game_state().z_zone },
        unsafe { game_state() }.g_game.demobuffer as *mut ::core::ffi::c_void,
    );
    unsafe { game_state() }.g_game.demobuffer = new_demobuffer;
    unsafe { game_state() }.g_game.demo_p = new_demop;
    unsafe { game_state() }.g_game.demoend = unsafe { game_state() }
        .g_game
        .demobuffer
        .offset(new_length as isize);
}
pub unsafe fn G_WriteDemoTiccmd(mut cmd: *mut ticcmd_t) {
    let mut demo_start: *mut byte = ::core::ptr::null_mut::<byte>();
    if unsafe { game_state() }.g_game.gamekeydown
        [unsafe { game_state() }.m_controls.key_demo_quit as usize]
        != 0
    {
        G_CheckDemoStatus();
    }
    demo_start = unsafe { game_state() }.g_game.demo_p;
    let fresh12 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh12 = (*cmd).forwardmove as byte;
    let fresh13 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh13 = (*cmd).sidemove as byte;
    if unsafe { game_state() }.g_game.longtics {
        let fresh14 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh14 = ((*cmd).angleturn as i32 & 0xff as i32) as byte;
        let fresh15 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh15 = ((*cmd).angleturn as i32 >> 8 as i32 & 0xff as i32) as byte;
    } else {
        let fresh16 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh16 = ((*cmd).angleturn as i32 >> 8 as i32) as byte;
    }
    let fresh17 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh17 = (*cmd).buttons;
    unsafe { game_state() }.g_game.demo_p = demo_start;
    if unsafe { game_state() }.g_game.demo_p
        > unsafe { game_state() }
            .g_game
            .demoend
            .offset(-(16 as i32 as isize))
    {
        if unsafe { game_state() }.g_game.vanilla_demo_limit != 0 {
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
    unsafe { game_state() }.g_game.usergame = false;
    demoname_size = strlen(name).wrapping_add(5 as size_t);
    unsafe { game_state() }.g_game.demoname = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        demoname_size as i32,
        PU_STATIC as i32,
        NULL,
    ) as *mut ::core::ffi::c_char;
    M_snprintf(
        unsafe { game_state() }.g_game.demoname,
        demoname_size,
        b"%s.lmp\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    maxsize = 0x20000 as i32;
    i = M_CheckParmWithArgs("-maxdemo", 1 as i32);
    if i != 0 {
        maxsize = atoi(
            unsafe { game_state() }.m_argv.myargv[(i + 1 as i32) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        ) * 1024 as i32;
    }
    unsafe { game_state() }.g_game.demobuffer = Z_Malloc(
        unsafe { &mut game_state().z_zone },
        maxsize,
        PU_STATIC as i32,
        NULL,
    ) as *mut byte;
    unsafe { game_state() }.g_game.demoend = unsafe { game_state() }
        .g_game
        .demobuffer
        .offset(maxsize as isize);
    unsafe { game_state() }.g_game.demorecording = true;
}
pub unsafe fn G_VanillaVersionCode(state: &mut DoomstatState) -> i32 {
    match state.gameversion as u32 {
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
    unsafe { game_state() }.g_game.longtics = M_CheckParm("-longtics") != 0 as i32;
    unsafe { game_state() }.g_game.lowres_turn = !unsafe { game_state() }.g_game.longtics;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demobuffer;
    if unsafe { game_state() }.g_game.longtics {
        let fresh0 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh0 = DOOM_191_VERSION as byte;
    } else {
        let fresh1 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh1 = G_VanillaVersionCode(&mut unsafe { game_state() }.doomstat) as byte;
    }
    let fresh2 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh2 = unsafe { game_state() }.g_game.gameskill as byte;
    let fresh3 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh3 = unsafe { game_state() }.g_game.gameepisode as byte;
    let fresh4 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh4 = unsafe { game_state() }.g_game.gamemap as byte;
    let fresh5 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh5 = unsafe { game_state() }.g_game.deathmatch as byte;
    let fresh6 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh6 = unsafe { game_state() }.d_main.respawnparm as byte;
    let fresh7 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh7 = unsafe { game_state() }.d_main.fastparm as byte;
    let fresh8 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh8 = unsafe { game_state() }.d_main.nomonsters as byte;
    let fresh9 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    *fresh9 = unsafe { game_state() }.g_game.consoleplayer as byte;
    i = 0 as i32;
    while i < MAXPLAYERS {
        let fresh10 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh10 = unsafe { game_state() }.g_game.playeringame[i as usize] as byte;
        i += 1;
    }
}
pub unsafe fn G_DeferedPlayDemo(mut name: *mut ::core::ffi::c_char) {
    unsafe { game_state() }.g_game.defdemoname = name;
    unsafe { game_state() }.g_game.gameaction = ga_playdemo;
}
unsafe fn DemoVersionDescription(mut version: i32) -> *mut ::core::ffi::c_char {
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
            as *mut ::core::ffi::c_char;
    } else {
        M_snprintf(
            &raw mut unsafe { game_state() }
                .g_game
                .demo_version_description_resultbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
            b"%i.%i (unknown)\0" as *const u8 as *const ::core::ffi::c_char,
            version / 100 as i32,
            version % 100 as i32,
        );
        return &raw mut unsafe { game_state() }
            .g_game
            .demo_version_description_resultbuf as *mut ::core::ffi::c_char;
    };
}
pub unsafe fn G_DoPlayDemo() {
    let mut skill: skill_t = sk_baby;
    let mut i: i32 = 0;
    let mut episode: i32 = 0;
    let mut map: i32 = 0;
    let mut demoversion: i32 = 0;
    unsafe { game_state() }.g_game.gameaction = ga_nothing;
    unsafe { game_state() }.g_game.demo_p = W_CacheLumpName(
        &wad_name8_to_string(unsafe { game_state() }.g_game.defdemoname),
        PU_STATIC as i32,
    ) as *mut byte;
    unsafe { game_state() }.g_game.demobuffer = unsafe { game_state() }.g_game.demo_p;
    let fresh24 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    demoversion = *fresh24 as i32;
    if demoversion == G_VanillaVersionCode(&mut unsafe { game_state() }.doomstat) {
        unsafe { game_state() }.g_game.longtics = false;
    } else if demoversion == DOOM_191_VERSION {
        unsafe { game_state() }.g_game.longtics = true;
    } else {
        let mut message: *mut ::core::ffi::c_char = b"Demo is from a different game version!\n(read %i, should be %i)\n\n*** You may need to upgrade your version of Doom to v1.9. ***\n    See: https://www.doomworld.com/classicdoom/info/patches.php\n    This appears to be %s.\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        printf(
            message,
            demoversion,
            G_VanillaVersionCode(&mut unsafe { game_state() }.doomstat),
            DemoVersionDescription(demoversion),
        );
    }
    let fresh25 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    skill = *fresh25 as skill_t;
    let fresh26 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    episode = *fresh26 as i32;
    let fresh27 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    map = *fresh27 as i32;
    let fresh28 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    unsafe { game_state() }.g_game.deathmatch = *fresh28 as i32;
    let fresh29 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    unsafe { game_state() }.d_main.respawnparm = *fresh29 != 0;
    let fresh30 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    unsafe { game_state() }.d_main.fastparm = *fresh30 != 0;
    let fresh31 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    unsafe { game_state() }.d_main.nomonsters = *fresh31 != 0;
    let fresh32 = unsafe { game_state() }.g_game.demo_p;
    unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
    unsafe { game_state() }.g_game.consoleplayer = *fresh32 as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        let fresh33 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        unsafe { game_state() }.g_game.playeringame[i as usize] = *fresh33 as boolean;
        i += 1;
    }
    if unsafe { game_state() }.g_game.playeringame[1 as i32 as usize] != 0
        || M_CheckParm("-solo-net") > 0 as i32
        || M_CheckParm("-netdemo") > 0 as i32
    {
        unsafe { game_state() }.g_game.netgame = true;
        unsafe { game_state() }.g_game.netdemo = true;
    }
    unsafe { game_state() }.g_game.precache = false;
    G_InitNew(skill, episode, map);
    unsafe { game_state() }.g_game.precache = true;
    unsafe { game_state() }.g_game.starttime = I_GetTime(unsafe { &mut game_state().i_timer });
    unsafe { game_state() }.g_game.usergame = false;
    unsafe { game_state() }.g_game.demoplayback = true;
}
pub unsafe fn G_TimeDemo(mut name: *mut ::core::ffi::c_char) {
    unsafe { game_state() }.g_game.nodrawers = M_CheckParm("-nodraw") != 0;
    unsafe { game_state() }.g_game.timingdemo = true;
    unsafe { game_state() }.d_loop.singletics = true;
    unsafe { game_state() }.g_game.defdemoname = name;
    unsafe { game_state() }.g_game.gameaction = ga_playdemo;
}
#[no_mangle]
pub unsafe extern "C" fn G_CheckDemoStatus() -> boolean {
    let mut endtime: i32 = 0;
    if unsafe { game_state() }.g_game.timingdemo {
        let mut fps: f32 = 0.;
        let mut realtics: i32 = 0;
        endtime = I_GetTime(unsafe { &mut game_state().i_timer });
        realtics = endtime - unsafe { game_state() }.g_game.starttime;
        fps = unsafe { game_state() }.d_loop.gametic as f32 * TICRATE as f32 / realtics as f32;
        unsafe { game_state() }.g_game.timingdemo = false;
        unsafe { game_state() }.g_game.demoplayback = false;
        I_Error(&format!(
            "timed {} gametics in {} realtics ({:.6} fps)",
            unsafe { game_state() }.d_loop.gametic,
            realtics,
            fps as f64,
        ));
    }
    if unsafe { game_state() }.g_game.demoplayback {
        W_ReleaseLumpName(&wad_name8_to_string(
            unsafe { game_state() }.g_game.defdemoname,
        ));
        unsafe { game_state() }.g_game.demoplayback = false;
        unsafe { game_state() }.g_game.netdemo = false;
        unsafe { game_state() }.g_game.netgame = false;
        unsafe { game_state() }.g_game.deathmatch = false_0;
        unsafe { game_state() }.g_game.playeringame[3 as i32 as usize] = 0 as boolean;
        unsafe { game_state() }.g_game.playeringame[2 as i32 as usize] =
            unsafe { game_state() }.g_game.playeringame[3 as i32 as usize];
        unsafe { game_state() }.g_game.playeringame[1 as i32 as usize] =
            unsafe { game_state() }.g_game.playeringame[2 as i32 as usize];
        unsafe { game_state() }.d_main.respawnparm = false;
        unsafe { game_state() }.d_main.fastparm = false;
        unsafe { game_state() }.d_main.nomonsters = false;
        unsafe { game_state() }.g_game.consoleplayer = 0 as i32;
        if unsafe { game_state() }.g_game.singledemo {
            I_Quit();
        } else {
            D_AdvanceDemo(unsafe { game_state() });
        }
        return true_0 as boolean;
    }
    if unsafe { game_state() }.g_game.demorecording {
        let fresh11 = unsafe { game_state() }.g_game.demo_p;
        unsafe { game_state() }.g_game.demo_p = unsafe { game_state() }.g_game.demo_p.offset(1);
        *fresh11 = DEMOMARKER as byte;
        M_WriteFile(
            unsafe { game_state() }.g_game.demoname,
            unsafe { game_state() }.g_game.demobuffer as *mut ::core::ffi::c_void,
            unsafe { game_state() }
                .g_game
                .demo_p
                .offset_from(unsafe { game_state() }.g_game.demobuffer) as i64 as i32,
        );
        Z_Free(
            unsafe { &mut game_state().z_zone },
            unsafe { game_state() }.g_game.demobuffer as *mut ::core::ffi::c_void,
        );
        unsafe { game_state() }.g_game.demorecording = false;
        I_Error(&format!(
            "Demo {} recorded",
            ::std::ffi::CStr::from_ptr(unsafe { game_state() }.g_game.demoname)
                .to_str()
                .unwrap(),
        ));
    }
    return false_0 as boolean;
}
pub const MAX_MOUSE_BUTTONS: i32 = 8;
unsafe extern "C" fn run_static_initializers() {
    unsafe { game_state() }.g_game.joybuttons = (&raw mut unsafe { game_state() }.g_game.joyarray
        as *mut boolean)
        .offset(1 as i32 as isize) as *mut boolean;
    unsafe { game_state() }.g_game.mousebuttons =
        (&raw mut unsafe { game_state() }.g_game.mousearray as *mut boolean)
            .offset(1 as i32 as isize) as *mut boolean;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
