use crate::src::w_file::wad_file_t;
use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::d_player::{player_t, PST_LIVE};
use crate::src::i_system::I_Error;
use crate::src::m_argv::{myargv, M_CheckParm, M_CheckParmWithArgs};
use crate::src::m_config::M_BindVariable;
use crate::src::m_misc::M_StringEndsWith;
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName, W_CheckNumForName};
use crate::src::d_loop::D_StartGameLoop;
use crate::src::doomstat::gamedescription;
use crate::src::g_game::nodrawers;
use crate::src::g_game::testcontrols_mousespeed;
use crate::src::g_game::displayplayer;
use crate::src::i_sound::I_InitSound;
use crate::src::i_sound::I_InitMusic;
use crate::src::i_sound::I_BindSoundVariables;
use crate::src::d_iwad::D_FindIWAD;
use crate::src::d_iwad::D_SaveGameIWADName;
use crate::src::w_main::W_ParseCommandLine;
use crate::src::w_wad::W_GenerateHashTable;
use crate::src::w_wad::W_CheckCorrectIWAD;
use crate::src::s_sound::S_UpdateSounds;
use crate::src::s_sound::snd_channels;
use crate::src::v_video::V_DrawMouseSpeedBox;
use crate::src::d_event::D_PopEvent;
use crate::src::game_state::game_state;
use crate::src::f_finale::F_Drawer;
use crate::src::f_wipe::wipe_StartScreen;
use crate::src::f_wipe::wipe_EndScreen;
use crate::src::f_wipe::wipe_ScreenWipe;
use crate::src::m_config::M_SetConfigDir;
use crate::src::m_config::M_SetConfigFilenames;
use crate::src::m_config::M_GetSaveGameDir;
use crate::src::m_controls::M_BindBaseControls;
use crate::src::m_controls::M_BindWeaponControls;
use crate::src::m_controls::M_BindMapControls;
use crate::src::m_controls::M_BindMenuControls;
use crate::src::m_controls::M_BindChatControls;
use crate::src::m_menu::M_Responder;
use crate::src::m_menu::M_Drawer;
use crate::src::i_joystick::I_BindJoystickVariables;
use crate::src::i_system::I_PrintStartupBanner;
use crate::src::i_system::I_PrintBanner;
use crate::src::i_system::I_PrintDivider;
use crate::src::i_video::I_FinishUpdate;
use crate::src::i_video::I_SetWindowTitle;
use crate::src::i_video::I_SetGrabMouseCallback;
use crate::src::i_video::screenvisible;
use crate::src::g_game::G_InitNew;
use crate::src::g_game::G_DeferedPlayDemo;
use crate::src::g_game::G_RecordDemo;
use crate::src::g_game::G_BeginRecording;
use crate::src::g_game::G_TimeDemo;
use crate::src::g_game::G_Responder;
use crate::src::g_game::vanilla_savegame_limit;
use crate::src::g_game::vanilla_demo_limit;
use crate::src::hu_stuff::HU_Drawer;
use crate::src::hu_stuff::HU_Erase;
use crate::src::hu_stuff::chat_macros;
use crate::src::wi_stuff::WI_Drawer;
use crate::src::st_stuff::ST_Drawer;
use crate::src::am_map::AM_Drawer;
use crate::src::r_main::R_RenderPlayerView;
use crate::src::r_draw::R_DrawViewBorder;
use crate::src::m_menu::inhelpscreens;
use crate::src::d_net::D_ConnectNetGame;
use crate::src::g_game::forwardmove;
use crate::src::g_game::sidemove;
use crate::src::d_loop::NetUpdate;
use crate::src::doomstat::modifiedgame;
use crate::src::dummy::drone;
use crate::src::g_game::G_LoadGame;
use crate::src::g_game::G_VanillaVersionCode;
use crate::src::g_game::gameaction;
use crate::src::g_game::paused;
use crate::src::g_game::usergame;
use crate::src::g_game::demorecording;
use crate::src::g_game::singledemo;
use crate::src::g_game::testcontrols;
use crate::src::i_timer::I_Sleep;
use crate::src::i_video::screensaver_mode;
use crate::src::m_controls::key_multi_msgplayer;
use crate::src::m_menu::mouseSensitivity;
use crate::src::m_menu::showMessages;
use crate::src::m_menu::detailLevel;
use crate::src::m_menu::screenblocks;
use crate::src::m_menu::menuactive;
use crate::src::r_draw::R_FillBackScreen;
use crate::src::r_draw::scaledviewwidth;
use crate::src::r_draw::viewwindowx;
use crate::src::r_draw::viewwindowy;
use crate::src::r_main::R_ExecuteSetViewSize;
use crate::src::r_main::setsizeneeded;
use crate::src::s_sound::S_StartMusic;
use crate::src::s_sound::sfxVolume;
use crate::src::s_sound::musicVolume;
use crate::src::w_wad::W_AddFile;
use crate::src::w_wad::numlumps;
use crate::src::g_game::gamestate;
use crate::src::g_game::timelimit;
use crate::src::g_game::viewactive;
use crate::src::i_system::I_AtExit;
use crate::src::i_video::I_SetPalette;
use crate::src::p_saveg::P_SaveGameFile;
use crate::src::v_video::V_DrawPatchDirect;
use crate::src::v_video::V_RestoreBuffer;
use crate::src::d_loop::gametic;
use crate::src::w_wad::lumpinfo;
use crate::src::g_game::demoplayback;
use crate::src::r_draw::viewheight;
use crate::src::doomstat::gamemission;
use crate::src::am_map::automapactive;
use crate::src::m_misc::M_StringCopy;
use crate::src::g_game::deathmatch;
use crate::src::m_misc::M_snprintf;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::d_loop::TryRunTics;
use crate::src::d_net::D_CheckNetGame;
use crate::src::hu_stuff::HU_Init;
use crate::src::i_video::I_InitGraphics;
use crate::src::m_config::M_LoadDefaults;
use crate::src::m_menu::M_Init;
use crate::src::p_setup::P_Init;
use crate::src::r_main::R_Init;
use crate::src::s_sound::S_Init;
use crate::src::st_stuff::ST_Init;
use crate::src::z_zone::Z_Init;
use crate::src::i_timer::I_GetTime;
use crate::src::v_video::V_DrawPatch;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use crate::src::sounds::{mus_dm2ttl, mus_intro};
use crate::src::d_mode::{commercial, registered, retail, shareware};
use crate::src::d_mode::{GameVersion_t, exe_chex, exe_doom_1_2, exe_doom_1_666, exe_doom_1_7, exe_doom_1_8, exe_doom_1_9, exe_final, exe_final2, exe_hacx, exe_ultimate};
use crate::src::d_mode::{GameMission_t, doom, doom2, none, pack_chex, pack_hacx, pack_plut, pack_tnt};
use crate::src::d_mode::{sk_baby, sk_medium, skill_t};
use crate::src::d_event::{GS_DEMOSCREEN, GS_LEVEL, gamestate_t};
use crate::src::d_event::{ga_loadgame, ga_nothing, ga_playdemo};
use crate::src::i_system::atexit_func_t;
use crate::src::doomdef::boolean;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use libc::{atoi, strcasecmp, strcmp, strlen, strncasecmp};
use libc::{exit, printf, snprintf};
use crate::src::doomdef::NULL;
use crate::src::doomdef::false_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::doomdef::TICRATE;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::doomdef::SCREENHEIGHT;

extern "C" {
    fn __ctype_b_loc() -> *mut *const u16;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn M_SaveDefaults();
    fn G_CheckDemoStatus() -> boolean;
    fn StatDump();
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type C2RustUnnamed_2 = u32;
pub const wipe_NUMWIPES: C2RustUnnamed_2 = 2;
pub const wipe_Melt: C2RustUnnamed_2 = 1;
pub const wipe_ColorXForm: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub name: *mut ::core::ffi::c_char,
    pub mission: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub description: *mut ::core::ffi::c_char,
    pub cmdline: *mut ::core::ffi::c_char,
    pub version: GameVersion_t,
}
pub const PACKAGE_STRING: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"Doom Generic 0.1\0")
};
pub const D_DEVSTR: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<
        [u8; 22],
        [::core::ffi::c_char; 22],
    >(*b"Development mode ON.\n\0")
};
pub const HUSTR_KEYGREEN: i32 = 'g' as i32;
pub const HUSTR_KEYINDIGO: i32 = 'i' as i32;
pub const HUSTR_KEYBROWN: i32 = 'b' as i32;
pub const HUSTR_KEYRED: i32 = 'r' as i32;
pub static mut savegamedir: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut iwadfile: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
pub static mut devparm: bool = false;
pub static mut nomonsters: bool = false;
pub static mut respawnparm: bool = false;
pub static mut fastparm: bool = false;
pub static mut startskill: skill_t = sk_baby;
pub static mut startepisode: i32 = 0;
pub static mut startmap: i32 = 0;
pub static mut autostart: bool = false;
pub static mut startloadgame: i32 = 0;
pub static mut advancedemo: bool = false;
#[no_mangle]
pub static mut storedemo: bool = false;
#[no_mangle]
pub static mut bfgedition: bool = false;
#[no_mangle]
pub static mut main_loop_started: bool = false;
#[no_mangle]
pub static mut wadfile: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut mapdir: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut show_endoom: i32 = 1;
pub unsafe fn D_ProcessEvents() {
    let mut ev: *mut event_t = ::core::ptr::null_mut::<event_t>();
    if storedemo {
        return;
    }
    loop {
        ev = D_PopEvent(&mut game_state().d_event);
        if ev.is_null() {
            break;
        }
        if M_Responder(ev) {
            continue;
        }
        G_Responder(ev);
    };
}
pub static mut wipegamestate: gamestate_t = GS_DEMOSCREEN;
pub unsafe fn D_Display() {
    static mut viewactivestate: bool = false;
    static mut menuactivestate: bool = false;
    static mut inhelpscreensstate: bool = false;
    static mut fullscreen: bool = false;
    static mut oldgamestate: gamestate_t = 4294967295;
    static mut borderdrawcount: i32 = 0;
    let mut nowtime: i32 = 0;
    let mut tics: i32 = 0;
    let mut wipestart: i32 = 0;
    let mut y: i32 = 0;
    let mut done: bool = false;
    let mut wipe: bool = false;
    let mut redrawsbar: bool = false;
    if nodrawers {
        return;
    }
    redrawsbar = false;
    if setsizeneeded {
        R_ExecuteSetViewSize();
        oldgamestate = 4294967295 as gamestate_t;
        borderdrawcount = 3 as i32;
    }
    if gamestate as u32 != wipegamestate as u32 {
        wipe = true;
        wipe_StartScreen(
            0 as i32,
            0 as i32,
            SCREENWIDTH,
            SCREENHEIGHT,
        );
    } else {
        wipe = false;
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32 && gametic != 0
    {
        HU_Erase();
    }
    match gamestate as u32 {
        0 => {
            if !(gametic == 0) {
                if automapactive {
                    AM_Drawer();
                }
                if wipe
                    || viewheight != 200 as i32 && fullscreen
                {
                    redrawsbar = true;
                }
                if inhelpscreensstate && !inhelpscreens {
                    redrawsbar = true;
                }
                ST_Drawer(
                    viewheight == 200 as i32,
                    redrawsbar,
                );
                fullscreen = viewheight == 200 as i32;
            }
        }
        1 => {
            WI_Drawer();
        }
        2 => {
            F_Drawer(unsafe { &mut game_state().f_finale });
        }
        3 => {
            D_PageDrawer();
        }
        _ => {}
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32 && !automapactive
        && gametic != 0
    {
        R_RenderPlayerView(
            (&raw mut players as *mut player_t).offset(displayplayer as isize)
                as *mut player_t,
        );
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32 && gametic != 0
    {
        HU_Drawer();
    }
    if gamestate as u32 != oldgamestate as u32
        && gamestate as u32
            != GS_LEVEL as i32 as u32
    {
        I_SetPalette(
            W_CacheLumpName("PLAYPAL",
                PU_CACHE as i32,
            ) as *mut byte,
        );
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32
        && oldgamestate as u32
            != GS_LEVEL as i32 as u32
    {
        viewactivestate = false;
        R_FillBackScreen();
    }
    if gamestate as u32
        == GS_LEVEL as i32 as u32 && !automapactive
        && scaledviewwidth != 320 as i32
    {
        if menuactive || menuactivestate || !viewactivestate {
            borderdrawcount = 3 as i32;
        }
        if borderdrawcount != 0 {
            R_DrawViewBorder();
            borderdrawcount -= 1;
        }
    }
    if testcontrols {
        V_DrawMouseSpeedBox(testcontrols_mousespeed);
    }
    menuactivestate = menuactive;
    viewactivestate = viewactive;
    inhelpscreensstate = inhelpscreens;
    wipegamestate = gamestate;
    oldgamestate = wipegamestate;
    if paused {
        if automapactive {
            y = 4 as i32;
        } else {
            y = viewwindowy + 4 as i32;
        }
        V_DrawPatchDirect(
            viewwindowx
                + (scaledviewwidth - 68 as i32) / 2 as i32,
            y,
            W_CacheLumpName("M_PAUSE",
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
    }
    M_Drawer();
    NetUpdate();
    if !wipe {
        I_FinishUpdate();
        return;
    }
    wipe_EndScreen(
        0 as i32,
        0 as i32,
        SCREENWIDTH,
        SCREENHEIGHT,
    );
    wipestart = I_GetTime(unsafe { &mut game_state().i_timer }) - 1 as i32;
    loop {
        loop {
            nowtime = I_GetTime(unsafe { &mut game_state().i_timer });
            tics = nowtime - wipestart;
            I_Sleep(1 as i32);
            if !(tics <= 0 as i32) {
                break;
            }
        }
        wipestart = nowtime;
        done = wipe_ScreenWipe(
            wipe_Melt as i32,
            0 as i32,
            0 as i32,
            SCREENWIDTH,
            SCREENHEIGHT,
            tics,
        ) != 0;
        M_Drawer();
        I_FinishUpdate();
        if done {
            break;
        }
    };
}
pub unsafe fn D_BindVariables() {
    let mut i: i32 = 0;
    I_BindJoystickVariables(unsafe { &mut game_state().i_joystick });
    I_BindSoundVariables();
    M_BindBaseControls();
    M_BindWeaponControls();
    M_BindMapControls();
    M_BindMenuControls();
    M_BindChatControls(MAXPLAYERS as u32);
    key_multi_msgplayer[0 as i32 as usize] = HUSTR_KEYGREEN;
    key_multi_msgplayer[1 as i32 as usize] = HUSTR_KEYINDIGO;
    key_multi_msgplayer[2 as i32 as usize] = HUSTR_KEYBROWN;
    key_multi_msgplayer[3 as i32 as usize] = HUSTR_KEYRED;
    M_BindVariable("mouse_sensitivity",
        &raw mut mouseSensitivity as *mut ::core::ffi::c_void,
    );
    M_BindVariable("sfx_volume",
        &raw mut sfxVolume as *mut ::core::ffi::c_void,
    );
    M_BindVariable("music_volume",
        &raw mut musicVolume as *mut ::core::ffi::c_void,
    );
    M_BindVariable("show_messages",
        &raw mut showMessages as *mut ::core::ffi::c_void,
    );
    M_BindVariable("screenblocks",
        &raw mut screenblocks as *mut ::core::ffi::c_void,
    );
    M_BindVariable("detaillevel",
        &raw mut detailLevel as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_channels",
        &raw mut snd_channels as *mut ::core::ffi::c_void,
    );
    M_BindVariable("vanilla_savegame_limit",
        &raw mut vanilla_savegame_limit as *mut ::core::ffi::c_void,
    );
    M_BindVariable("vanilla_demo_limit",
        &raw mut vanilla_demo_limit as *mut ::core::ffi::c_void,
    );
    M_BindVariable("show_endoom",
        &raw mut show_endoom as *mut ::core::ffi::c_void,
    );
    i = 0 as i32;
    while i < 10 as i32 {
        let mut buf: [::core::ffi::c_char; 12] = [0; 12];
        M_snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
            b"chatmacro%i\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        M_BindVariable(
            ::std::ffi::CStr::from_ptr(&raw mut buf as *mut ::core::ffi::c_char)
                .to_str()
                .unwrap(),
            (&raw mut chat_macros as *mut *mut ::core::ffi::c_char).offset(i as isize)
                as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
}
pub unsafe fn D_GrabMouseCallback() -> boolean {
    if drone {
        return false_0 as boolean;
    }
    if menuactive || paused {
        return false_0 as boolean;
    }
    return (gamestate as u32
        == GS_LEVEL as i32 as u32 && !demoplayback
        && !advancedemo) as i32 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn doomgeneric_Tick() {
    TryRunTics();
    S_UpdateSounds(players[consoleplayer as usize].mo);
    if screenvisible {
        D_Display();
    }
}
pub unsafe fn D_DoomLoop() {
    if bfgedition
        && (demorecording
            || gameaction as u32
                == ga_playdemo as i32 as u32
            || netgame)
    {
        printf(
            b" WARNING: You are playing using one of the Doom Classic\n IWAD files shipped with the Doom 3: BFG Edition. These are\n known to be incompatible with the regular IWAD files and\n may cause demos and network games to get out of sync.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if demorecording {
        G_BeginRecording();
    }
    main_loop_started = true;
    TryRunTics();
    I_SetWindowTitle(gamedescription);
    I_SetGrabMouseCallback(
        Some(D_GrabMouseCallback as unsafe fn() -> boolean),
    );
    I_InitGraphics();
    V_RestoreBuffer();
    R_ExecuteSetViewSize();
    D_StartGameLoop();
    if testcontrols {
        wipegamestate = gamestate;
    }
    doomgeneric_Tick();
}
#[no_mangle]
pub static mut demosequence: i32 = 0;
#[no_mangle]
pub static mut pagetic: i32 = 0;
#[no_mangle]
pub static mut pagename: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
pub unsafe fn D_PageTicker() {
    pagetic -= 1;
    if pagetic < 0 as i32 {
        D_AdvanceDemo();
    }
}
pub unsafe fn D_PageDrawer() {
    V_DrawPatch(
        0 as i32,
        0 as i32,
        W_CacheLumpName(
            &wad_name8_to_string(pagename),
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
pub unsafe fn D_AdvanceDemo() {
    advancedemo = true;
}
pub unsafe fn D_DoAdvanceDemo() {
    players[consoleplayer as usize].playerstate = PST_LIVE;
    advancedemo = false;
    usergame = false;
    paused = false;
    gameaction = ga_nothing;
    if gameversion as u32
        == exe_ultimate as i32 as u32
        || gameversion as u32
            == exe_final as i32 as u32
    {
        demosequence = (demosequence + 1 as i32)
            % 7 as i32;
    } else {
        demosequence = (demosequence + 1 as i32)
            % 6 as i32;
    }
    match demosequence {
        0 => {
            if gamemode as u32
                == commercial as i32 as u32
            {
                pagetic = TICRATE * 11 as i32;
            } else {
                pagetic = 170 as i32;
            }
            gamestate = GS_DEMOSCREEN;
            pagename = b"TITLEPIC\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            if gamemode as u32
                == commercial as i32 as u32
            {
                S_StartMusic(mus_dm2ttl as i32);
            } else {
                S_StartMusic(mus_intro as i32);
            }
        }
        1 => {
            G_DeferedPlayDemo(
                b"demo1\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        2 => {
            pagetic = 200 as i32;
            gamestate = GS_DEMOSCREEN;
            pagename = b"CREDIT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3 => {
            G_DeferedPlayDemo(
                b"demo2\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        4 => {
            gamestate = GS_DEMOSCREEN;
            if gamemode as u32
                == commercial as i32 as u32
            {
                pagetic = TICRATE * 11 as i32;
                pagename = b"TITLEPIC\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                S_StartMusic(mus_dm2ttl as i32);
            } else {
                pagetic = 200 as i32;
                if gamemode as u32
                    == retail as i32 as u32
                {
                    pagename = b"CREDIT\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else {
                    pagename = b"HELP2\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
            }
        }
        5 => {
            G_DeferedPlayDemo(
                b"demo3\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        6 => {
            G_DeferedPlayDemo(
                b"demo4\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    if bfgedition
        && strcasecmp(pagename, b"TITLEPIC\0" as *const u8 as *const ::core::ffi::c_char)
            == 0
        && W_CheckNumForName("titlepic",
        ) < 0 as i32
    {
        pagename = b"INTERPIC\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
}
pub unsafe fn D_StartTitle() {
    gameaction = ga_nothing;
    demosequence = -(1 as i32);
    D_AdvanceDemo();
}
static banners: [&str; 7] = [
    "                         DOOM 2: Hell on Earth v%i.%i                           ",
    "                            DOOM Shareware Startup v%i.%i                           ",
    "                            DOOM Registered Startup v%i.%i                           ",
    "                          DOOM System Startup v%i.%i                          ",
    "                         The Ultimate DOOM Startup v%i.%i                        ",
    "                     DOOM 2: TNT - Evilution v%i.%i                           ",
    "                   DOOM 2: Plutonia Experiment v%i.%i                           ",
];
unsafe fn GetGameName(
    mut gamename: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < banners.len() as size_t {
        let deh_sub_str: &str = banners[i as usize];
        if deh_sub_str != banners[i as usize] {
            let deh_sub_cstring = ::std::ffi::CString::new(deh_sub_str).unwrap();
            let deh_sub: *mut ::core::ffi::c_char = deh_sub_cstring.as_ptr()
                as *mut ::core::ffi::c_char;
            let mut gamename_size: size_t = 0;
            let mut version: i32 = 0;
            gamename_size = strlen(deh_sub).wrapping_add(10 as size_t);
            gamename = Z_Malloc(
                gamename_size as i32,
                PU_STATIC as i32,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ) as *mut ::core::ffi::c_char;
            version = G_VanillaVersionCode();
            M_snprintf(
                gamename,
                gamename_size,
                deh_sub,
                version / 100 as i32,
                version % 100 as i32,
            );
            while *gamename.offset(0 as i32 as isize)
                as i32 != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *gamename.offset(0 as i32 as isize)
                            as i32 as isize,
                    ) as i32
                    & _ISspace as i32 as u16
                        as i32 != 0
            {
                memmove(
                    gamename as *mut ::core::ffi::c_void,
                    gamename.offset(1 as i32 as isize)
                        as *const ::core::ffi::c_void,
                    gamename_size.wrapping_sub(1 as size_t),
                );
            }
            while *gamename.offset(0 as i32 as isize)
                as i32 != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *gamename
                            .offset(strlen(gamename).wrapping_sub(1 as size_t) as isize)
                            as i32 as isize,
                    ) as i32
                    & _ISspace as i32 as u16
                        as i32 != 0
            {
                *gamename.offset(strlen(gamename).wrapping_sub(1 as size_t) as isize) = '\0'
                    as i32 as ::core::ffi::c_char;
            }
            return gamename;
        }
        i = i.wrapping_add(1);
    }
    return gamename;
}
unsafe fn SetMissionForPackName(mut pack_name: *mut ::core::ffi::c_char) {
    let mut i: i32 = 0;
    static mut packs: [C2RustUnnamed_3; 3] = [
        C2RustUnnamed_3 {
            name: b"doom2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            mission: doom2 as i32,
        },
        C2RustUnnamed_3 {
            name: b"tnt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            mission: pack_tnt as i32,
        },
        C2RustUnnamed_3 {
            name: b"plutonia\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            mission: pack_plut as i32,
        },
    ];
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_3; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_3>() as usize)
    {
        if strcasecmp(pack_name, packs[i as usize].name) == 0 {
            gamemission = packs[i as usize].mission as GameMission_t;
            return;
        }
        i += 1;
    }
    printf(b"Valid mission packs are:\n\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as i32;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_3; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_3>() as usize)
    {
        printf(
            b"\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            packs[i as usize].name,
        );
        i += 1;
    }
    I_Error(&format!(
        "Unknown mission pack name: {}",
        ::std::ffi::CStr::from_ptr(pack_name).to_str().unwrap(),
    ));
}
pub unsafe fn D_IdentifyVersion() {
    if gamemission as u32
        == none as i32 as u32
    {
        let mut i: u32 = 0;
        i = 0 as u32;
        while i < numlumps {
            if strncasecmp(
                &raw mut (*lumpinfo.offset(i as isize)).name as *mut ::core::ffi::c_char,
                b"MAP01\0" as *const u8 as *const ::core::ffi::c_char,
                8 as size_t,
            ) == 0
            {
                gamemission = doom2;
                break;
            } else if strncasecmp(
                &raw mut (*lumpinfo.offset(i as isize)).name as *mut ::core::ffi::c_char,
                b"E1M1\0" as *const u8 as *const ::core::ffi::c_char,
                8 as size_t,
            ) == 0
            {
                gamemission = doom;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if gamemission as u32
            == none as i32 as u32
        {
            I_Error("Unknown or invalid IWAD file.");
        }
    }
    if (if gamemission as u32
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
        if W_CheckNumForName("E4M1",
        ) > 0 as i32
        {
            gamemode = retail;
        } else if W_CheckNumForName("E3M1",
        ) > 0 as i32
        {
            gamemode = registered;
        } else {
            gamemode = shareware;
        }
    } else {
        let mut p: i32 = 0;
        gamemode = commercial;
        p = M_CheckParmWithArgs("-pack", 1 as i32);
        if p > 0 as i32 {
            SetMissionForPackName(
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
            );
        }
    };
}
pub unsafe fn D_SetGameDescription() {
    let mut is_freedoom: bool = W_CheckNumForName("FREEDOOM",
    ) >= 0 as i32;
    let mut is_freedm: bool = W_CheckNumForName("FREEDM",
    ) >= 0 as i32;
    gamedescription = b"Unknown\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    if (if gamemission as u32
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
        if is_freedoom {
            gamedescription = GetGameName(
                b"Freedoom: Phase 1\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else if gamemode as u32
            == retail as i32 as u32
        {
            gamedescription = GetGameName(
                b"The Ultimate DOOM\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else if gamemode as u32
            == registered as i32 as u32
        {
            gamedescription = GetGameName(
                b"DOOM Registered\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else if gamemode as u32
            == shareware as i32 as u32
        {
            gamedescription = GetGameName(
                b"DOOM Shareware\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    } else if is_freedoom {
        if is_freedm {
            gamedescription = GetGameName(
                b"FreeDM\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            gamedescription = GetGameName(
                b"Freedoom: Phase 2\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
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
    }) == doom2 as i32 as u32
    {
        gamedescription = GetGameName(
            b"DOOM 2: Hell on Earth\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
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
    }) == pack_plut as i32 as u32
    {
        gamedescription = GetGameName(
            b"DOOM 2: Plutonia Experiment\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
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
    }) == pack_tnt as i32 as u32
    {
        gamedescription = GetGameName(
            b"DOOM 2: TNT - Evilution\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub static mut title: [::core::ffi::c_char; 128] = [0; 128];
unsafe fn D_AddFile(mut filename: *mut ::core::ffi::c_char) -> bool {
    let mut handle: *mut wad_file_t = ::core::ptr::null_mut::<wad_file_t>();
    printf(b" adding %s\n\0" as *const u8 as *const ::core::ffi::c_char, filename);
    handle = W_AddFile(filename);
    return handle != NULL as *mut wad_file_t;
}
static copyright_banners: [&str; 3] = [
    "===========================================================================\nATTENTION:  This version of DOOM has been modified.  If you would like to\nget a copy of the original game, call 1-800-IDGAMES or see the readme file.\n        You will not receive technical support for modified games.\n                      press enter to continue\n===========================================================================\n",
    "===========================================================================\n                 Commercial product - do not distribute!\n         Please report software piracy to the SPA: 1-800-388-PIR8\n===========================================================================\n",
    "===========================================================================\n                                Shareware!\n===========================================================================\n",
];
pub unsafe fn PrintDehackedBanners() {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < copyright_banners.len() as size_t {
        let deh_s_str: &str = copyright_banners[i as usize];
        if deh_s_str != copyright_banners[i as usize] {
            let deh_s_cstring = ::std::ffi::CString::new(deh_s_str).unwrap();
            let deh_s: *mut ::core::ffi::c_char = deh_s_cstring.as_ptr()
                as *mut ::core::ffi::c_char;
            printf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, deh_s);
            if *deh_s.offset(strlen(deh_s).wrapping_sub(1 as size_t) as isize)
                as i32 != '\n' as i32
            {
                printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        i = i.wrapping_add(1);
    }
}
static mut gameversions: [C2RustUnnamed_4; 10] = [
    C2RustUnnamed_4 {
        description: b"Doom 1.666\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"1.666\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_doom_1_666,
    },
    C2RustUnnamed_4 {
        description: b"Doom 1.7/1.7a\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"1.7\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_doom_1_7,
    },
    C2RustUnnamed_4 {
        description: b"Doom 1.8\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"1.8\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_doom_1_8,
    },
    C2RustUnnamed_4 {
        description: b"Doom 1.9\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"1.9\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_doom_1_9,
    },
    C2RustUnnamed_4 {
        description: b"Hacx\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"hacx\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_hacx,
    },
    C2RustUnnamed_4 {
        description: b"Ultimate Doom\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"ultimate\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_ultimate,
    },
    C2RustUnnamed_4 {
        description: b"Final Doom\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"final\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_final,
    },
    C2RustUnnamed_4 {
        description: b"Final Doom (alt)\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"final2\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_final2,
    },
    C2RustUnnamed_4 {
        description: b"Chex Quest\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cmdline: b"chex\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        version: exe_chex,
    },
    C2RustUnnamed_4 {
        description: ::core::ptr::null::<::core::ffi::c_char>()
            as *mut ::core::ffi::c_char,
        cmdline: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        version: exe_doom_1_2,
    },
];
unsafe fn InitGameVersion() {
    let mut p: i32 = 0;
    let mut i: i32 = 0;
    p = M_CheckParmWithArgs("-gameversion", 1 as i32);
    if p != 0 {
        i = 0 as i32;
        while !gameversions[i as usize].description.is_null() {
            if strcmp(
                myargv[(p + 1 as i32) as usize].as_ptr(),
                gameversions[i as usize].cmdline,
            ) == 0
            {
                gameversion = gameversions[i as usize].version;
                break;
            } else {
                i += 1;
            }
        }
        if gameversions[i as usize].description.is_null() {
            printf(
                b"Supported game versions:\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            i = 0 as i32;
            while !gameversions[i as usize].description.is_null() {
                printf(
                    b"\t%s (%s)\n\0" as *const u8 as *const ::core::ffi::c_char,
                    gameversions[i as usize].cmdline,
                    gameversions[i as usize].description,
                );
                i += 1;
            }
            I_Error(&format!(
                "Unknown game version '{}'",
                myargv[(p + 1 as i32) as usize].to_str().unwrap(),
            ));
        }
    } else if gamemission as u32
        == pack_chex as i32 as u32
    {
        gameversion = exe_chex;
    } else if gamemission as u32
        == pack_hacx as i32 as u32
    {
        gameversion = exe_hacx;
    } else if gamemode as u32
        == shareware as i32 as u32
        || gamemode as u32
            == registered as i32 as u32
    {
        gameversion = exe_doom_1_9;
    } else if gamemode as u32
        == retail as i32 as u32
    {
        gameversion = exe_ultimate;
    } else if gamemode as u32
        == commercial as i32 as u32
    {
        if gamemission as u32
            == doom2 as i32 as u32
        {
            gameversion = exe_doom_1_9;
        } else {
            gameversion = exe_final;
        }
    }
    if (gameversion as u32)
        < exe_ultimate as i32 as u32
        && gamemode as u32
            == retail as i32 as u32
    {
        gamemode = registered;
    }
    if (gameversion as u32)
        < exe_final as i32 as u32
        && gamemode as u32
            == commercial as i32 as u32
        && (gamemission as u32
            == pack_tnt as i32 as u32
            || gamemission as u32
                == pack_plut as i32 as u32)
    {
        gamemission = doom2;
    }
}
pub unsafe fn PrintGameVersion() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !gameversions[i as usize].description.is_null() {
        if gameversions[i as usize].version as u32
            == gameversion as u32
        {
            printf(
                b"Emulating the behavior of the '%s' executable.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                gameversions[i as usize].description,
            );
            break;
        } else {
            i += 1;
        }
    }
}
unsafe extern "C" fn D_Endoom() {
    if show_endoom == 0 || !main_loop_started || screensaver_mode
        || M_CheckParm("-testcontrols") > 0 as i32
    {
        return;
    }
    exit(0 as i32);
}
pub unsafe fn D_DoomMain() {
    let mut p: i32 = 0;
    let mut file: [::core::ffi::c_char; 256] = [0; 256];
    let mut demolumpname: [::core::ffi::c_char; 9] = [0; 9];
    I_AtExit(Some(D_Endoom as unsafe extern "C" fn() -> ()), false);
    I_PrintBanner(PACKAGE_STRING.as_ptr() as *mut ::core::ffi::c_char);
    printf(
        b"Z_Init: Init zone memory allocation daemon. \n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    Z_Init();
    nomonsters = M_CheckParm("-nomonsters") != 0;
    respawnparm = M_CheckParm("-respawn") != 0;
    fastparm = M_CheckParm("-fast") != 0;
    devparm = M_CheckParm("-devparm") != 0;
    if M_CheckParm("-deathmatch") != 0 {
        deathmatch = 1 as i32;
    }
    if M_CheckParm("-altdeath") != 0 {
        deathmatch = 2 as i32;
    }
    if devparm {
        printf(D_DEVSTR.as_ptr());
    }
    M_SetConfigDir(::core::ptr::null_mut::<::core::ffi::c_char>());
    p = M_CheckParm("-turbo");
    if p != 0 {
        let mut scale: i32 = 200 as i32;
        if p < myargv.len() as i32 - 1 as i32 {
            scale = atoi(
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
            );
        }
        if scale < 10 as i32 {
            scale = 10 as i32;
        }
        if scale > 400 as i32 {
            scale = 400 as i32;
        }
        printf(
            b"turbo scale: %i%%\n\0" as *const u8 as *const ::core::ffi::c_char,
            scale,
        );
        forwardmove[0 as i32 as usize] = forwardmove[0
            as i32 as usize] * scale / 100 as i32;
        forwardmove[1 as i32 as usize] = forwardmove[1
            as i32 as usize] * scale / 100 as i32;
        sidemove[0 as i32 as usize] = sidemove[0 as i32
            as usize] * scale / 100 as i32;
        sidemove[1 as i32 as usize] = sidemove[1 as i32
            as usize] * scale / 100 as i32;
    }
    printf(b"V_Init: allocate screens.\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"M_LoadDefaults: Load system defaults.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    M_SetConfigFilenames(
        b"default.cfg\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"doomgenericdoom.cfg\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    D_BindVariables();
    M_LoadDefaults();
    I_AtExit(Some(M_SaveDefaults as unsafe extern "C" fn() -> ()), false);
    iwadfile = D_FindIWAD(unsafe { &mut game_state().d_iwad }, 
        (1 as i32) << doom as i32
            | (1 as i32) << doom2 as i32
            | (1 as i32) << pack_tnt as i32
            | (1 as i32) << pack_plut as i32
            | (1 as i32) << pack_chex as i32
            | (1 as i32) << pack_hacx as i32,
        &raw mut gamemission,
    );
    if iwadfile.is_null() {
        I_Error(
            "Game mode indeterminate.  No IWAD file was found.  Try\nspecifying one with the '-iwad' command line parameter.\n",
        );
    }
    modifiedgame = false;
    printf(b"W_Init: Init WADfiles.\n\0" as *const u8 as *const ::core::ffi::c_char);
    D_AddFile(iwadfile);
    W_CheckCorrectIWAD(doom);
    D_IdentifyVersion();
    InitGameVersion();
    if W_CheckNumForName("dmenupic",
    ) >= 0 as i32
    {
        printf(
            b"BFG Edition: Using workarounds as needed.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        bfgedition = true;
    }
    modifiedgame = W_ParseCommandLine();
    p = M_CheckParmWithArgs("-playdemo", 1 as i32);
    if p == 0 {
        p = M_CheckParmWithArgs("-timedemo", 1 as i32);
    }
    if p != 0 {
        if M_StringEndsWith(
            myargv[(p + 1 as i32) as usize].to_str().unwrap(),
            ".lmp",
        )
        {
            M_StringCopy(
                &raw mut file as *mut ::core::ffi::c_char,
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            );
        } else {
            snprintf(
                &raw mut file as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s.lmp\0" as *const u8 as *const ::core::ffi::c_char,
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
            );
        }
        if D_AddFile(&raw mut file as *mut ::core::ffi::c_char) {
            M_StringCopy(
                &raw mut demolumpname as *mut ::core::ffi::c_char,
                &raw mut (*lumpinfo
                    .offset(numlumps.wrapping_sub(1 as u32) as isize))
                    .name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
            );
        } else {
            M_StringCopy(
                &raw mut demolumpname as *mut ::core::ffi::c_char,
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
            );
        }
        printf(
            b"Playing demo %s.\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut file as *mut ::core::ffi::c_char,
        );
    }
    I_AtExit(
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> boolean>,
            atexit_func_t,
        >(Some(G_CheckDemoStatus as unsafe extern "C" fn() -> boolean)),
        true,
    );
    W_GenerateHashTable();
    D_SetGameDescription();
    savegamedir = M_GetSaveGameDir(D_SaveGameIWADName(gamemission));
    if modifiedgame {
        let mut name: [[::core::ffi::c_char; 8]; 23] = [
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m1\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m2\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m3\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m4\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m5\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m6\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m7\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m8\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e2m9\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m1\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m3\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m3\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m4\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m5\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m6\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m7\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m8\0\0\0\0"),
            ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e3m9\0\0\0\0"),
            ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"dphoof\0\0"),
            ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"bfgga0\0\0"),
            ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"heada1\0\0"),
            ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"cybra1\0\0"),
            ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"spida1d1"),
        ];
        let mut i: i32 = 0;
        if gamemode as u32
            == shareware as i32 as u32
        {
            I_Error("\nYou cannot -file with the shareware version. Register!");
        }
        if gamemode as u32
            == registered as i32 as u32
        {
            i = 0 as i32;
            while i < 23 as i32 {
                if W_CheckNumForName(
                    &wad_name8_to_string(
                        &raw mut *(&raw mut name as *mut [::core::ffi::c_char; 8])
                            .offset(i as isize) as *mut ::core::ffi::c_char,
                    ),
                ) < 0 as i32
                {
                    I_Error("\nThis is not the registered version.");
                }
                i += 1;
            }
        }
    }
    if W_CheckNumForName("SS_START",
    ) >= 0 as i32
        || W_CheckNumForName("FF_END",
        ) >= 0 as i32
    {
        I_PrintDivider();
        printf(
            b" WARNING: The loaded WAD file contains modified sprites or\n floor textures.  You may want to use the '-merge' command\n line option instead of '-file'.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    I_PrintStartupBanner(gamedescription);
    PrintDehackedBanners();
    if W_CheckNumForName("FREEDOOM",
    ) >= 0 as i32
        && W_CheckNumForName("FREEDM",
        ) < 0 as i32
    {
        printf(
            b" WARNING: You are playing using one of the Freedoom IWAD\n files, which might not work in this port. See this page\n for more information on how to play using Freedoom:\n   http://www.chocolate-doom.org/wiki/index.php/Freedoom\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        I_PrintDivider();
    }
    printf(
        b"I_Init: Setting up machine state.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    I_InitSound(true);
    I_InitMusic();
    D_ConnectNetGame();
    startskill = sk_medium;
    startepisode = 1 as i32;
    startmap = 1 as i32;
    autostart = false;
    p = M_CheckParmWithArgs("-skill", 1 as i32);
    if p != 0 {
        startskill = (myargv[(p + 1 as i32) as usize].as_bytes().first().copied().unwrap_or(0)
            as i32 - '1' as i32) as skill_t;
        autostart = true;
    }
    p = M_CheckParmWithArgs("-episode", 1 as i32);
    if p != 0 {
        startepisode = myargv[(p + 1 as i32) as usize].as_bytes().first().copied().unwrap_or(0)
            as i32 - '0' as i32;
        startmap = 1 as i32;
        autostart = true;
    }
    timelimit = 0 as i32;
    p = M_CheckParmWithArgs("-timer", 1 as i32);
    if p != 0 {
        timelimit = atoi(
            myargv[(p + 1 as i32) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        );
    }
    p = M_CheckParm("-avg");
    if p != 0 {
        timelimit = 20 as i32;
    }
    p = M_CheckParmWithArgs("-warp", 1 as i32);
    if p != 0 {
        if gamemode as u32
            == commercial as i32 as u32
        {
            startmap = atoi(
                myargv[(p + 1 as i32) as usize].as_ptr()
                    as *mut ::core::ffi::c_char,
            );
        } else {
            startepisode = myargv[(p + 1 as i32) as usize].as_bytes().first().copied().unwrap_or(0)
                as i32 - '0' as i32;
            if (p + 2 as i32) < myargv.len() as i32 {
                startmap = myargv[(p + 2 as i32) as usize].as_bytes().first().copied().unwrap_or(0)
                    as i32 - '0' as i32;
            } else {
                startmap = 1 as i32;
            }
        }
        autostart = true;
    }
    p = M_CheckParm("-testcontrols");
    if p > 0 as i32 {
        startepisode = 1 as i32;
        startmap = 1 as i32;
        autostart = true;
        testcontrols = true;
    }
    p = M_CheckParmWithArgs("-loadgame", 1 as i32);
    if p != 0 {
        startloadgame = atoi(
            myargv[(p + 1 as i32) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        );
    } else {
        startloadgame = -(1 as i32);
    }
    printf(
        b"M_Init: Init miscellaneous info.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    M_Init();
    printf(
        b"R_Init: Init DOOM refresh daemon - \0" as *const u8
            as *const ::core::ffi::c_char,
    );
    R_Init();
    printf(
        b"\nP_Init: Init Playloop state.\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    P_Init();
    printf(b"S_Init: Setting up sound.\n\0" as *const u8 as *const ::core::ffi::c_char);
    S_Init(sfxVolume * 8 as i32, musicVolume * 8 as i32);
    printf(
        b"D_CheckNetGame: Checking network game status.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    D_CheckNetGame();
    PrintGameVersion();
    printf(
        b"HU_Init: Setting up heads up display.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    HU_Init();
    printf(b"ST_Init: Init status bar.\n\0" as *const u8 as *const ::core::ffi::c_char);
    ST_Init();
    if gamemode as u32
        == commercial as i32 as u32
        && W_CheckNumForName("map01",
        ) < 0 as i32
    {
        storedemo = true;
    }
    if M_CheckParmWithArgs("-statdump", 1 as i32) != 0 {
        I_AtExit(Some(StatDump as unsafe extern "C" fn() -> ()), true);
        printf(
            b"External statistics registered.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    p = M_CheckParmWithArgs("-record", 1 as i32);
    if p != 0 {
        G_RecordDemo(
            myargv[(p + 1 as i32) as usize].as_ptr()
                as *mut ::core::ffi::c_char,
        );
        autostart = true;
    }
    p = M_CheckParmWithArgs("-playdemo", 1 as i32);
    if p != 0 {
        singledemo = true;
        G_DeferedPlayDemo(&raw mut demolumpname as *mut ::core::ffi::c_char);
        D_DoomLoop();
        return;
    }
    p = M_CheckParmWithArgs("-timedemo", 1 as i32);
    if p != 0 {
        G_TimeDemo(&raw mut demolumpname as *mut ::core::ffi::c_char);
        D_DoomLoop();
        return;
    }
    if startloadgame >= 0 as i32 {
        M_StringCopy(
            &raw mut file as *mut ::core::ffi::c_char,
            P_SaveGameFile(startloadgame),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        );
        G_LoadGame(&raw mut file as *mut ::core::ffi::c_char);
    }
    if gameaction as u32
        != ga_loadgame as i32 as u32
    {
        if autostart || netgame {
            G_InitNew(startskill, startepisode, startmap);
        } else {
            D_StartTitle();
        }
    }
    D_DoomLoop();
}
