use crate::src::w_file::wad_file_t;
use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::d_player::{player_t, PST_LIVE};
use crate::src::p_mobj::{actionf_t};
use crate::src::i_system::I_Error;
use crate::src::m_argv::{myargv, M_CheckParm, M_CheckParmWithArgs};
use crate::src::m_config::M_BindVariable;
use crate::src::m_misc::M_StringEndsWith;
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName, W_CheckNumForName};
use crate::src::i_timer::I_InitTimer;
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
use crate::src::m_controls::M_ApplyPlatformDefaults;
use crate::src::m_menu::M_Responder;
use crate::src::m_menu::M_Drawer;
use crate::src::i_endoom::I_Endoom;
use crate::src::i_joystick::I_InitJoystick;
use crate::src::i_joystick::I_BindJoystickVariables;
use crate::src::i_system::I_PrintStartupBanner;
use crate::src::i_system::I_PrintBanner;
use crate::src::i_system::I_PrintDivider;
use crate::src::i_video::I_GraphicsCheckCommandLine;
use crate::src::i_video::I_UpdateNoBlit;
use crate::src::i_video::I_FinishUpdate;
use crate::src::i_video::I_SetWindowTitle;
use crate::src::i_video::I_CheckIsScreensaver;
use crate::src::i_video::I_SetGrabMouseCallback;
use crate::src::i_video::I_DisplayFPSDots;
use crate::src::i_video::I_BindVideoVariables;
use crate::src::i_video::I_StartFrame;
use crate::src::i_video::I_EnableLoadingDisk;
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
use crate::src::v_video::V_Init;
use crate::src::z_zone::Z_Init;
use crate::src::i_timer::I_GetTime;
use crate::src::v_video::V_DrawPatch;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};

extern "C" {
    fn __ctype_b_loc() -> *mut *const u16;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> i32;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> i32;
    fn exit(__status: i32) -> !;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> i32;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> i32;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> i32;
    fn M_SaveDefaults();
    fn G_CheckDemoStatus() -> boolean;
    fn StatDump();
}
pub type __uint8_t = u8;
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
pub type size_t = usize;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub type GameMission_t = u32;
pub const none: GameMission_t = 9;
pub const strife: GameMission_t = 8;
pub const hexen: GameMission_t = 7;
pub const heretic: GameMission_t = 6;
pub const pack_hacx: GameMission_t = 5;
pub const pack_chex: GameMission_t = 4;
pub const pack_plut: GameMission_t = 3;
pub const pack_tnt: GameMission_t = 2;
pub const doom2: GameMission_t = 1;
pub const doom: GameMission_t = 0;
pub type GameMode_t = u32;
pub const indetermined: GameMode_t = 4;
pub const retail: GameMode_t = 3;
pub const commercial: GameMode_t = 2;
pub const registered: GameMode_t = 1;
pub const shareware: GameMode_t = 0;
pub type GameVersion_t = u32;
pub const exe_strife_1_31: GameVersion_t = 13;
pub const exe_strife_1_2: GameVersion_t = 12;
pub const exe_hexen_1_1: GameVersion_t = 11;
pub const exe_heretic_1_3: GameVersion_t = 10;
pub const exe_chex: GameVersion_t = 9;
pub const exe_final2: GameVersion_t = 8;
pub const exe_final: GameVersion_t = 7;
pub const exe_ultimate: GameVersion_t = 6;
pub const exe_hacx: GameVersion_t = 5;
pub const exe_doom_1_9: GameVersion_t = 4;
pub const exe_doom_1_8: GameVersion_t = 3;
pub const exe_doom_1_7: GameVersion_t = 2;
pub const exe_doom_1_666: GameVersion_t = 1;
pub const exe_doom_1_2: GameVersion_t = 0;
pub type skill_t = i32;
pub const sk_nightmare: skill_t = 4;
pub const sk_hard: skill_t = 3;
pub const sk_medium: skill_t = 2;
pub const sk_easy: skill_t = 1;
pub const sk_baby: skill_t = 0;
pub const sk_noitems: skill_t = -1;
pub type gamestate_t = u32;
pub const GS_DEMOSCREEN: gamestate_t = 3;
pub const GS_FINALE: gamestate_t = 2;
pub const GS_INTERMISSION: gamestate_t = 1;
pub const GS_LEVEL: gamestate_t = 0;
pub type gameaction_t = u32;
pub const ga_screenshot: gameaction_t = 9;
pub const ga_worlddone: gameaction_t = 8;
pub const ga_victory: gameaction_t = 7;
pub const ga_completed: gameaction_t = 6;
pub const ga_playdemo: gameaction_t = 5;
pub const ga_savegame: gameaction_t = 4;
pub const ga_loadgame: gameaction_t = 3;
pub const ga_newgame: gameaction_t = 2;
pub const ga_loadlevel: gameaction_t = 1;
pub const ga_nothing: gameaction_t = 0;
pub type weapontype_t = u32;
pub const wp_nochange: weapontype_t = 10;
pub const NUMWEAPONS: weapontype_t = 9;
pub const wp_supershotgun: weapontype_t = 8;
pub const wp_chainsaw: weapontype_t = 7;
pub const wp_bfg: weapontype_t = 6;
pub const wp_plasma: weapontype_t = 5;
pub const wp_missile: weapontype_t = 4;
pub const wp_chaingun: weapontype_t = 3;
pub const wp_shotgun: weapontype_t = 2;
pub const wp_pistol: weapontype_t = 1;
pub const wp_fist: weapontype_t = 0;
pub type fixed_t = i32;
pub type angle_t = u32;
pub type actionf_v = Option<unsafe extern "C" fn() -> ()>;
pub type actionf_p1 = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type actionf_p2 = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
>;
pub type think_t = actionf_t;
pub type spritenum_t = u32;
pub const NUMSPRITES: spritenum_t = 138;
pub const SPR_TLP2: spritenum_t = 137;
pub const SPR_TLMP: spritenum_t = 136;
pub const SPR_BRS1: spritenum_t = 135;
pub const SPR_POB2: spritenum_t = 134;
pub const SPR_POB1: spritenum_t = 133;
pub const SPR_HDB6: spritenum_t = 132;
pub const SPR_HDB5: spritenum_t = 131;
pub const SPR_HDB4: spritenum_t = 130;
pub const SPR_HDB3: spritenum_t = 129;
pub const SPR_HDB2: spritenum_t = 128;
pub const SPR_HDB1: spritenum_t = 127;
pub const SPR_SMRT: spritenum_t = 126;
pub const SPR_SMGT: spritenum_t = 125;
pub const SPR_SMBT: spritenum_t = 124;
pub const SPR_TRED: spritenum_t = 123;
pub const SPR_TGRN: spritenum_t = 122;
pub const SPR_TBLU: spritenum_t = 121;
pub const SPR_COL5: spritenum_t = 120;
pub const SPR_FSKU: spritenum_t = 119;
pub const SPR_CEYE: spritenum_t = 118;
pub const SPR_ELEC: spritenum_t = 117;
pub const SPR_TRE2: spritenum_t = 116;
pub const SPR_TRE1: spritenum_t = 115;
pub const SPR_COL6: spritenum_t = 114;
pub const SPR_CBRA: spritenum_t = 113;
pub const SPR_CAND: spritenum_t = 112;
pub const SPR_COL4: spritenum_t = 111;
pub const SPR_COL3: spritenum_t = 110;
pub const SPR_COL2: spritenum_t = 109;
pub const SPR_COL1: spritenum_t = 108;
pub const SPR_SMIT: spritenum_t = 107;
pub const SPR_GOR5: spritenum_t = 106;
pub const SPR_GOR4: spritenum_t = 105;
pub const SPR_GOR3: spritenum_t = 104;
pub const SPR_GOR2: spritenum_t = 103;
pub const SPR_POL6: spritenum_t = 102;
pub const SPR_POL1: spritenum_t = 101;
pub const SPR_POL3: spritenum_t = 100;
pub const SPR_POL4: spritenum_t = 99;
pub const SPR_POL5: spritenum_t = 98;
pub const SPR_POL2: spritenum_t = 97;
pub const SPR_GOR1: spritenum_t = 96;
pub const SPR_SMT2: spritenum_t = 95;
pub const SPR_COLU: spritenum_t = 94;
pub const SPR_SGN2: spritenum_t = 93;
pub const SPR_SHOT: spritenum_t = 92;
pub const SPR_PLAS: spritenum_t = 91;
pub const SPR_LAUN: spritenum_t = 90;
pub const SPR_CSAW: spritenum_t = 89;
pub const SPR_MGUN: spritenum_t = 88;
pub const SPR_BFUG: spritenum_t = 87;
pub const SPR_BPAK: spritenum_t = 86;
pub const SPR_SBOX: spritenum_t = 85;
pub const SPR_SHEL: spritenum_t = 84;
pub const SPR_CELP: spritenum_t = 83;
pub const SPR_CELL: spritenum_t = 82;
pub const SPR_BROK: spritenum_t = 81;
pub const SPR_ROCK: spritenum_t = 80;
pub const SPR_AMMO: spritenum_t = 79;
pub const SPR_CLIP: spritenum_t = 78;
pub const SPR_PVIS: spritenum_t = 77;
pub const SPR_PMAP: spritenum_t = 76;
pub const SPR_SUIT: spritenum_t = 75;
pub const SPR_MEGA: spritenum_t = 74;
pub const SPR_PINS: spritenum_t = 73;
pub const SPR_PSTR: spritenum_t = 72;
pub const SPR_PINV: spritenum_t = 71;
pub const SPR_SOUL: spritenum_t = 70;
pub const SPR_MEDI: spritenum_t = 69;
pub const SPR_STIM: spritenum_t = 68;
pub const SPR_YSKU: spritenum_t = 67;
pub const SPR_RSKU: spritenum_t = 66;
pub const SPR_BSKU: spritenum_t = 65;
pub const SPR_YKEY: spritenum_t = 64;
pub const SPR_RKEY: spritenum_t = 63;
pub const SPR_BKEY: spritenum_t = 62;
pub const SPR_BON2: spritenum_t = 61;
pub const SPR_BON1: spritenum_t = 60;
pub const SPR_FCAN: spritenum_t = 59;
pub const SPR_BEXP: spritenum_t = 58;
pub const SPR_BAR1: spritenum_t = 57;
pub const SPR_ARM2: spritenum_t = 56;
pub const SPR_ARM1: spritenum_t = 55;
pub const SPR_BOSF: spritenum_t = 54;
pub const SPR_BBRN: spritenum_t = 53;
pub const SPR_KEEN: spritenum_t = 52;
pub const SPR_SSWV: spritenum_t = 51;
pub const SPR_PAIN: spritenum_t = 50;
pub const SPR_CYBR: spritenum_t = 49;
pub const SPR_APBX: spritenum_t = 48;
pub const SPR_APLS: spritenum_t = 47;
pub const SPR_BSPI: spritenum_t = 46;
pub const SPR_SPID: spritenum_t = 45;
pub const SPR_SKUL: spritenum_t = 44;
pub const SPR_BOS2: spritenum_t = 43;
pub const SPR_BOSS: spritenum_t = 42;
pub const SPR_BAL7: spritenum_t = 41;
pub const SPR_HEAD: spritenum_t = 40;
pub const SPR_SARG: spritenum_t = 39;
pub const SPR_CPOS: spritenum_t = 38;
pub const SPR_FATT: spritenum_t = 37;
pub const SPR_MANF: spritenum_t = 36;
pub const SPR_SKEL: spritenum_t = 35;
pub const SPR_FBXP: spritenum_t = 34;
pub const SPR_FATB: spritenum_t = 33;
pub const SPR_FIRE: spritenum_t = 32;
pub const SPR_VILE: spritenum_t = 31;
pub const SPR_SPOS: spritenum_t = 30;
pub const SPR_POSS: spritenum_t = 29;
pub const SPR_PLAY: spritenum_t = 28;
pub const SPR_IFOG: spritenum_t = 27;
pub const SPR_TFOG: spritenum_t = 26;
pub const SPR_BFE2: spritenum_t = 25;
pub const SPR_BFE1: spritenum_t = 24;
pub const SPR_BFS1: spritenum_t = 23;
pub const SPR_MISL: spritenum_t = 22;
pub const SPR_PLSE: spritenum_t = 21;
pub const SPR_PLSS: spritenum_t = 20;
pub const SPR_BAL2: spritenum_t = 19;
pub const SPR_BAL1: spritenum_t = 18;
pub const SPR_PUFF: spritenum_t = 17;
pub const SPR_BLUD: spritenum_t = 16;
pub const SPR_BFGF: spritenum_t = 15;
pub const SPR_BFGG: spritenum_t = 14;
pub const SPR_PLSF: spritenum_t = 13;
pub const SPR_PLSG: spritenum_t = 12;
pub const SPR_SAWG: spritenum_t = 11;
pub const SPR_MISF: spritenum_t = 10;
pub const SPR_MISG: spritenum_t = 9;
pub const SPR_CHGF: spritenum_t = 8;
pub const SPR_CHGG: spritenum_t = 7;
pub const SPR_SHT2: spritenum_t = 6;
pub const SPR_SHTF: spritenum_t = 5;
pub const SPR_PISF: spritenum_t = 4;
pub const SPR_PISG: spritenum_t = 3;
pub const SPR_PUNG: spritenum_t = 2;
pub const SPR_SHTG: spritenum_t = 1;
pub const SPR_TROO: spritenum_t = 0;
pub type statenum_t = u32;
pub const NUMSTATES: statenum_t = 967;
pub const S_TECH2LAMP4: statenum_t = 966;
pub const S_TECH2LAMP3: statenum_t = 965;
pub const S_TECH2LAMP2: statenum_t = 964;
pub const S_TECH2LAMP: statenum_t = 963;
pub const S_TECHLAMP4: statenum_t = 962;
pub const S_TECHLAMP3: statenum_t = 961;
pub const S_TECHLAMP2: statenum_t = 960;
pub const S_TECHLAMP: statenum_t = 959;
pub const S_BRAINSTEM: statenum_t = 958;
pub const S_SMALLPOOL: statenum_t = 957;
pub const S_COLONGIBS: statenum_t = 956;
pub const S_HANGTNOBRAIN: statenum_t = 955;
pub const S_HANGTLOOKUP: statenum_t = 954;
pub const S_HANGTSKULL: statenum_t = 953;
pub const S_HANGTLOOKDN: statenum_t = 952;
pub const S_HANGBNOBRAIN: statenum_t = 951;
pub const S_HANGNOGUTS: statenum_t = 950;
pub const S_RTORCHSHRT4: statenum_t = 949;
pub const S_RTORCHSHRT3: statenum_t = 948;
pub const S_RTORCHSHRT2: statenum_t = 947;
pub const S_RTORCHSHRT: statenum_t = 946;
pub const S_GTORCHSHRT4: statenum_t = 945;
pub const S_GTORCHSHRT3: statenum_t = 944;
pub const S_GTORCHSHRT2: statenum_t = 943;
pub const S_GTORCHSHRT: statenum_t = 942;
pub const S_BTORCHSHRT4: statenum_t = 941;
pub const S_BTORCHSHRT3: statenum_t = 940;
pub const S_BTORCHSHRT2: statenum_t = 939;
pub const S_BTORCHSHRT: statenum_t = 938;
pub const S_REDTORCH4: statenum_t = 937;
pub const S_REDTORCH3: statenum_t = 936;
pub const S_REDTORCH2: statenum_t = 935;
pub const S_REDTORCH: statenum_t = 934;
pub const S_GREENTORCH4: statenum_t = 933;
pub const S_GREENTORCH3: statenum_t = 932;
pub const S_GREENTORCH2: statenum_t = 931;
pub const S_GREENTORCH: statenum_t = 930;
pub const S_BLUETORCH4: statenum_t = 929;
pub const S_BLUETORCH3: statenum_t = 928;
pub const S_BLUETORCH2: statenum_t = 927;
pub const S_BLUETORCH: statenum_t = 926;
pub const S_HEARTCOL2: statenum_t = 925;
pub const S_HEARTCOL: statenum_t = 924;
pub const S_FLOATSKULL3: statenum_t = 923;
pub const S_FLOATSKULL2: statenum_t = 922;
pub const S_FLOATSKULL: statenum_t = 921;
pub const S_EVILEYE4: statenum_t = 920;
pub const S_EVILEYE3: statenum_t = 919;
pub const S_EVILEYE2: statenum_t = 918;
pub const S_EVILEYE: statenum_t = 917;
pub const S_TECHPILLAR: statenum_t = 916;
pub const S_BIGTREE: statenum_t = 915;
pub const S_TORCHTREE: statenum_t = 914;
pub const S_SKULLCOL: statenum_t = 913;
pub const S_CANDELABRA: statenum_t = 912;
pub const S_CANDLESTIK: statenum_t = 911;
pub const S_SHRTREDCOL: statenum_t = 910;
pub const S_TALLREDCOL: statenum_t = 909;
pub const S_SHRTGRNCOL: statenum_t = 908;
pub const S_TALLGRNCOL: statenum_t = 907;
pub const S_STALAGTITE: statenum_t = 906;
pub const S_MEAT5: statenum_t = 905;
pub const S_MEAT4: statenum_t = 904;
pub const S_MEAT3: statenum_t = 903;
pub const S_MEAT2: statenum_t = 902;
pub const S_LIVESTICK2: statenum_t = 901;
pub const S_LIVESTICK: statenum_t = 900;
pub const S_DEADSTICK: statenum_t = 899;
pub const S_HEADCANDLES2: statenum_t = 898;
pub const S_HEADCANDLES: statenum_t = 897;
pub const S_HEADONASTICK: statenum_t = 896;
pub const S_GIBS: statenum_t = 895;
pub const S_HEADSONSTICK: statenum_t = 894;
pub const S_DEADBOTTOM: statenum_t = 893;
pub const S_DEADTORSO: statenum_t = 892;
pub const S_BLOODYTWITCH4: statenum_t = 891;
pub const S_BLOODYTWITCH3: statenum_t = 890;
pub const S_BLOODYTWITCH2: statenum_t = 889;
pub const S_BLOODYTWITCH: statenum_t = 888;
pub const S_STALAG: statenum_t = 887;
pub const S_COLU: statenum_t = 886;
pub const S_SHOT2: statenum_t = 885;
pub const S_SHOT: statenum_t = 884;
pub const S_PLAS: statenum_t = 883;
pub const S_LAUN: statenum_t = 882;
pub const S_CSAW: statenum_t = 881;
pub const S_MGUN: statenum_t = 880;
pub const S_BFUG: statenum_t = 879;
pub const S_BPAK: statenum_t = 878;
pub const S_SBOX: statenum_t = 877;
pub const S_SHEL: statenum_t = 876;
pub const S_CELP: statenum_t = 875;
pub const S_CELL: statenum_t = 874;
pub const S_BROK: statenum_t = 873;
pub const S_ROCK: statenum_t = 872;
pub const S_AMMO: statenum_t = 871;
pub const S_CLIP: statenum_t = 870;
pub const S_PVIS2: statenum_t = 869;
pub const S_PVIS: statenum_t = 868;
pub const S_PMAP6: statenum_t = 867;
pub const S_PMAP5: statenum_t = 866;
pub const S_PMAP4: statenum_t = 865;
pub const S_PMAP3: statenum_t = 864;
pub const S_PMAP2: statenum_t = 863;
pub const S_PMAP: statenum_t = 862;
pub const S_SUIT: statenum_t = 861;
pub const S_MEGA4: statenum_t = 860;
pub const S_MEGA3: statenum_t = 859;
pub const S_MEGA2: statenum_t = 858;
pub const S_MEGA: statenum_t = 857;
pub const S_PINS4: statenum_t = 856;
pub const S_PINS3: statenum_t = 855;
pub const S_PINS2: statenum_t = 854;
pub const S_PINS: statenum_t = 853;
pub const S_PSTR: statenum_t = 852;
pub const S_PINV4: statenum_t = 851;
pub const S_PINV3: statenum_t = 850;
pub const S_PINV2: statenum_t = 849;
pub const S_PINV: statenum_t = 848;
pub const S_SOUL6: statenum_t = 847;
pub const S_SOUL5: statenum_t = 846;
pub const S_SOUL4: statenum_t = 845;
pub const S_SOUL3: statenum_t = 844;
pub const S_SOUL2: statenum_t = 843;
pub const S_SOUL: statenum_t = 842;
pub const S_MEDI: statenum_t = 841;
pub const S_STIM: statenum_t = 840;
pub const S_YSKULL2: statenum_t = 839;
pub const S_YSKULL: statenum_t = 838;
pub const S_RSKULL2: statenum_t = 837;
pub const S_RSKULL: statenum_t = 836;
pub const S_BSKULL2: statenum_t = 835;
pub const S_BSKULL: statenum_t = 834;
pub const S_YKEY2: statenum_t = 833;
pub const S_YKEY: statenum_t = 832;
pub const S_RKEY2: statenum_t = 831;
pub const S_RKEY: statenum_t = 830;
pub const S_BKEY2: statenum_t = 829;
pub const S_BKEY: statenum_t = 828;
pub const S_BON2E: statenum_t = 827;
pub const S_BON2D: statenum_t = 826;
pub const S_BON2C: statenum_t = 825;
pub const S_BON2B: statenum_t = 824;
pub const S_BON2A: statenum_t = 823;
pub const S_BON2: statenum_t = 822;
pub const S_BON1E: statenum_t = 821;
pub const S_BON1D: statenum_t = 820;
pub const S_BON1C: statenum_t = 819;
pub const S_BON1B: statenum_t = 818;
pub const S_BON1A: statenum_t = 817;
pub const S_BON1: statenum_t = 816;
pub const S_BBAR3: statenum_t = 815;
pub const S_BBAR2: statenum_t = 814;
pub const S_BBAR1: statenum_t = 813;
pub const S_BEXP5: statenum_t = 812;
pub const S_BEXP4: statenum_t = 811;
pub const S_BEXP3: statenum_t = 810;
pub const S_BEXP2: statenum_t = 809;
pub const S_BEXP: statenum_t = 808;
pub const S_BAR2: statenum_t = 807;
pub const S_BAR1: statenum_t = 806;
pub const S_ARM2A: statenum_t = 805;
pub const S_ARM2: statenum_t = 804;
pub const S_ARM1A: statenum_t = 803;
pub const S_ARM1: statenum_t = 802;
pub const S_BRAINEXPLODE3: statenum_t = 801;
pub const S_BRAINEXPLODE2: statenum_t = 800;
pub const S_BRAINEXPLODE1: statenum_t = 799;
pub const S_SPAWNFIRE8: statenum_t = 798;
pub const S_SPAWNFIRE7: statenum_t = 797;
pub const S_SPAWNFIRE6: statenum_t = 796;
pub const S_SPAWNFIRE5: statenum_t = 795;
pub const S_SPAWNFIRE4: statenum_t = 794;
pub const S_SPAWNFIRE3: statenum_t = 793;
pub const S_SPAWNFIRE2: statenum_t = 792;
pub const S_SPAWNFIRE1: statenum_t = 791;
pub const S_SPAWN4: statenum_t = 790;
pub const S_SPAWN3: statenum_t = 789;
pub const S_SPAWN2: statenum_t = 788;
pub const S_SPAWN1: statenum_t = 787;
pub const S_BRAINEYE1: statenum_t = 786;
pub const S_BRAINEYESEE: statenum_t = 785;
pub const S_BRAINEYE: statenum_t = 784;
pub const S_BRAIN_DIE4: statenum_t = 783;
pub const S_BRAIN_DIE3: statenum_t = 782;
pub const S_BRAIN_DIE2: statenum_t = 781;
pub const S_BRAIN_DIE1: statenum_t = 780;
pub const S_BRAIN_PAIN: statenum_t = 779;
pub const S_BRAIN: statenum_t = 778;
pub const S_KEENPAIN2: statenum_t = 777;
pub const S_KEENPAIN: statenum_t = 776;
pub const S_COMMKEEN12: statenum_t = 775;
pub const S_COMMKEEN11: statenum_t = 774;
pub const S_COMMKEEN10: statenum_t = 773;
pub const S_COMMKEEN9: statenum_t = 772;
pub const S_COMMKEEN8: statenum_t = 771;
pub const S_COMMKEEN7: statenum_t = 770;
pub const S_COMMKEEN6: statenum_t = 769;
pub const S_COMMKEEN5: statenum_t = 768;
pub const S_COMMKEEN4: statenum_t = 767;
pub const S_COMMKEEN3: statenum_t = 766;
pub const S_COMMKEEN2: statenum_t = 765;
pub const S_COMMKEEN: statenum_t = 764;
pub const S_KEENSTND: statenum_t = 763;
pub const S_SSWV_RAISE5: statenum_t = 762;
pub const S_SSWV_RAISE4: statenum_t = 761;
pub const S_SSWV_RAISE3: statenum_t = 760;
pub const S_SSWV_RAISE2: statenum_t = 759;
pub const S_SSWV_RAISE1: statenum_t = 758;
pub const S_SSWV_XDIE9: statenum_t = 757;
pub const S_SSWV_XDIE8: statenum_t = 756;
pub const S_SSWV_XDIE7: statenum_t = 755;
pub const S_SSWV_XDIE6: statenum_t = 754;
pub const S_SSWV_XDIE5: statenum_t = 753;
pub const S_SSWV_XDIE4: statenum_t = 752;
pub const S_SSWV_XDIE3: statenum_t = 751;
pub const S_SSWV_XDIE2: statenum_t = 750;
pub const S_SSWV_XDIE1: statenum_t = 749;
pub const S_SSWV_DIE5: statenum_t = 748;
pub const S_SSWV_DIE4: statenum_t = 747;
pub const S_SSWV_DIE3: statenum_t = 746;
pub const S_SSWV_DIE2: statenum_t = 745;
pub const S_SSWV_DIE1: statenum_t = 744;
pub const S_SSWV_PAIN2: statenum_t = 743;
pub const S_SSWV_PAIN: statenum_t = 742;
pub const S_SSWV_ATK6: statenum_t = 741;
pub const S_SSWV_ATK5: statenum_t = 740;
pub const S_SSWV_ATK4: statenum_t = 739;
pub const S_SSWV_ATK3: statenum_t = 738;
pub const S_SSWV_ATK2: statenum_t = 737;
pub const S_SSWV_ATK1: statenum_t = 736;
pub const S_SSWV_RUN8: statenum_t = 735;
pub const S_SSWV_RUN7: statenum_t = 734;
pub const S_SSWV_RUN6: statenum_t = 733;
pub const S_SSWV_RUN5: statenum_t = 732;
pub const S_SSWV_RUN4: statenum_t = 731;
pub const S_SSWV_RUN3: statenum_t = 730;
pub const S_SSWV_RUN2: statenum_t = 729;
pub const S_SSWV_RUN1: statenum_t = 728;
pub const S_SSWV_STND2: statenum_t = 727;
pub const S_SSWV_STND: statenum_t = 726;
pub const S_PAIN_RAISE6: statenum_t = 725;
pub const S_PAIN_RAISE5: statenum_t = 724;
pub const S_PAIN_RAISE4: statenum_t = 723;
pub const S_PAIN_RAISE3: statenum_t = 722;
pub const S_PAIN_RAISE2: statenum_t = 721;
pub const S_PAIN_RAISE1: statenum_t = 720;
pub const S_PAIN_DIE6: statenum_t = 719;
pub const S_PAIN_DIE5: statenum_t = 718;
pub const S_PAIN_DIE4: statenum_t = 717;
pub const S_PAIN_DIE3: statenum_t = 716;
pub const S_PAIN_DIE2: statenum_t = 715;
pub const S_PAIN_DIE1: statenum_t = 714;
pub const S_PAIN_PAIN2: statenum_t = 713;
pub const S_PAIN_PAIN: statenum_t = 712;
pub const S_PAIN_ATK4: statenum_t = 711;
pub const S_PAIN_ATK3: statenum_t = 710;
pub const S_PAIN_ATK2: statenum_t = 709;
pub const S_PAIN_ATK1: statenum_t = 708;
pub const S_PAIN_RUN6: statenum_t = 707;
pub const S_PAIN_RUN5: statenum_t = 706;
pub const S_PAIN_RUN4: statenum_t = 705;
pub const S_PAIN_RUN3: statenum_t = 704;
pub const S_PAIN_RUN2: statenum_t = 703;
pub const S_PAIN_RUN1: statenum_t = 702;
pub const S_PAIN_STND: statenum_t = 701;
pub const S_CYBER_DIE10: statenum_t = 700;
pub const S_CYBER_DIE9: statenum_t = 699;
pub const S_CYBER_DIE8: statenum_t = 698;
pub const S_CYBER_DIE7: statenum_t = 697;
pub const S_CYBER_DIE6: statenum_t = 696;
pub const S_CYBER_DIE5: statenum_t = 695;
pub const S_CYBER_DIE4: statenum_t = 694;
pub const S_CYBER_DIE3: statenum_t = 693;
pub const S_CYBER_DIE2: statenum_t = 692;
pub const S_CYBER_DIE1: statenum_t = 691;
pub const S_CYBER_PAIN: statenum_t = 690;
pub const S_CYBER_ATK6: statenum_t = 689;
pub const S_CYBER_ATK5: statenum_t = 688;
pub const S_CYBER_ATK4: statenum_t = 687;
pub const S_CYBER_ATK3: statenum_t = 686;
pub const S_CYBER_ATK2: statenum_t = 685;
pub const S_CYBER_ATK1: statenum_t = 684;
pub const S_CYBER_RUN8: statenum_t = 683;
pub const S_CYBER_RUN7: statenum_t = 682;
pub const S_CYBER_RUN6: statenum_t = 681;
pub const S_CYBER_RUN5: statenum_t = 680;
pub const S_CYBER_RUN4: statenum_t = 679;
pub const S_CYBER_RUN3: statenum_t = 678;
pub const S_CYBER_RUN2: statenum_t = 677;
pub const S_CYBER_RUN1: statenum_t = 676;
pub const S_CYBER_STND2: statenum_t = 675;
pub const S_CYBER_STND: statenum_t = 674;
pub const S_ARACH_PLEX5: statenum_t = 673;
pub const S_ARACH_PLEX4: statenum_t = 672;
pub const S_ARACH_PLEX3: statenum_t = 671;
pub const S_ARACH_PLEX2: statenum_t = 670;
pub const S_ARACH_PLEX: statenum_t = 669;
pub const S_ARACH_PLAZ2: statenum_t = 668;
pub const S_ARACH_PLAZ: statenum_t = 667;
pub const S_BSPI_RAISE7: statenum_t = 666;
pub const S_BSPI_RAISE6: statenum_t = 665;
pub const S_BSPI_RAISE5: statenum_t = 664;
pub const S_BSPI_RAISE4: statenum_t = 663;
pub const S_BSPI_RAISE3: statenum_t = 662;
pub const S_BSPI_RAISE2: statenum_t = 661;
pub const S_BSPI_RAISE1: statenum_t = 660;
pub const S_BSPI_DIE7: statenum_t = 659;
pub const S_BSPI_DIE6: statenum_t = 658;
pub const S_BSPI_DIE5: statenum_t = 657;
pub const S_BSPI_DIE4: statenum_t = 656;
pub const S_BSPI_DIE3: statenum_t = 655;
pub const S_BSPI_DIE2: statenum_t = 654;
pub const S_BSPI_DIE1: statenum_t = 653;
pub const S_BSPI_PAIN2: statenum_t = 652;
pub const S_BSPI_PAIN: statenum_t = 651;
pub const S_BSPI_ATK4: statenum_t = 650;
pub const S_BSPI_ATK3: statenum_t = 649;
pub const S_BSPI_ATK2: statenum_t = 648;
pub const S_BSPI_ATK1: statenum_t = 647;
pub const S_BSPI_RUN12: statenum_t = 646;
pub const S_BSPI_RUN11: statenum_t = 645;
pub const S_BSPI_RUN10: statenum_t = 644;
pub const S_BSPI_RUN9: statenum_t = 643;
pub const S_BSPI_RUN8: statenum_t = 642;
pub const S_BSPI_RUN7: statenum_t = 641;
pub const S_BSPI_RUN6: statenum_t = 640;
pub const S_BSPI_RUN5: statenum_t = 639;
pub const S_BSPI_RUN4: statenum_t = 638;
pub const S_BSPI_RUN3: statenum_t = 637;
pub const S_BSPI_RUN2: statenum_t = 636;
pub const S_BSPI_RUN1: statenum_t = 635;
pub const S_BSPI_SIGHT: statenum_t = 634;
pub const S_BSPI_STND2: statenum_t = 633;
pub const S_BSPI_STND: statenum_t = 632;
pub const S_SPID_DIE11: statenum_t = 631;
pub const S_SPID_DIE10: statenum_t = 630;
pub const S_SPID_DIE9: statenum_t = 629;
pub const S_SPID_DIE8: statenum_t = 628;
pub const S_SPID_DIE7: statenum_t = 627;
pub const S_SPID_DIE6: statenum_t = 626;
pub const S_SPID_DIE5: statenum_t = 625;
pub const S_SPID_DIE4: statenum_t = 624;
pub const S_SPID_DIE3: statenum_t = 623;
pub const S_SPID_DIE2: statenum_t = 622;
pub const S_SPID_DIE1: statenum_t = 621;
pub const S_SPID_PAIN2: statenum_t = 620;
pub const S_SPID_PAIN: statenum_t = 619;
pub const S_SPID_ATK4: statenum_t = 618;
pub const S_SPID_ATK3: statenum_t = 617;
pub const S_SPID_ATK2: statenum_t = 616;
pub const S_SPID_ATK1: statenum_t = 615;
pub const S_SPID_RUN12: statenum_t = 614;
pub const S_SPID_RUN11: statenum_t = 613;
pub const S_SPID_RUN10: statenum_t = 612;
pub const S_SPID_RUN9: statenum_t = 611;
pub const S_SPID_RUN8: statenum_t = 610;
pub const S_SPID_RUN7: statenum_t = 609;
pub const S_SPID_RUN6: statenum_t = 608;
pub const S_SPID_RUN5: statenum_t = 607;
pub const S_SPID_RUN4: statenum_t = 606;
pub const S_SPID_RUN3: statenum_t = 605;
pub const S_SPID_RUN2: statenum_t = 604;
pub const S_SPID_RUN1: statenum_t = 603;
pub const S_SPID_STND2: statenum_t = 602;
pub const S_SPID_STND: statenum_t = 601;
pub const S_SKULL_DIE6: statenum_t = 600;
pub const S_SKULL_DIE5: statenum_t = 599;
pub const S_SKULL_DIE4: statenum_t = 598;
pub const S_SKULL_DIE3: statenum_t = 597;
pub const S_SKULL_DIE2: statenum_t = 596;
pub const S_SKULL_DIE1: statenum_t = 595;
pub const S_SKULL_PAIN2: statenum_t = 594;
pub const S_SKULL_PAIN: statenum_t = 593;
pub const S_SKULL_ATK4: statenum_t = 592;
pub const S_SKULL_ATK3: statenum_t = 591;
pub const S_SKULL_ATK2: statenum_t = 590;
pub const S_SKULL_ATK1: statenum_t = 589;
pub const S_SKULL_RUN2: statenum_t = 588;
pub const S_SKULL_RUN1: statenum_t = 587;
pub const S_SKULL_STND2: statenum_t = 586;
pub const S_SKULL_STND: statenum_t = 585;
pub const S_BOS2_RAISE7: statenum_t = 584;
pub const S_BOS2_RAISE6: statenum_t = 583;
pub const S_BOS2_RAISE5: statenum_t = 582;
pub const S_BOS2_RAISE4: statenum_t = 581;
pub const S_BOS2_RAISE3: statenum_t = 580;
pub const S_BOS2_RAISE2: statenum_t = 579;
pub const S_BOS2_RAISE1: statenum_t = 578;
pub const S_BOS2_DIE7: statenum_t = 577;
pub const S_BOS2_DIE6: statenum_t = 576;
pub const S_BOS2_DIE5: statenum_t = 575;
pub const S_BOS2_DIE4: statenum_t = 574;
pub const S_BOS2_DIE3: statenum_t = 573;
pub const S_BOS2_DIE2: statenum_t = 572;
pub const S_BOS2_DIE1: statenum_t = 571;
pub const S_BOS2_PAIN2: statenum_t = 570;
pub const S_BOS2_PAIN: statenum_t = 569;
pub const S_BOS2_ATK3: statenum_t = 568;
pub const S_BOS2_ATK2: statenum_t = 567;
pub const S_BOS2_ATK1: statenum_t = 566;
pub const S_BOS2_RUN8: statenum_t = 565;
pub const S_BOS2_RUN7: statenum_t = 564;
pub const S_BOS2_RUN6: statenum_t = 563;
pub const S_BOS2_RUN5: statenum_t = 562;
pub const S_BOS2_RUN4: statenum_t = 561;
pub const S_BOS2_RUN3: statenum_t = 560;
pub const S_BOS2_RUN2: statenum_t = 559;
pub const S_BOS2_RUN1: statenum_t = 558;
pub const S_BOS2_STND2: statenum_t = 557;
pub const S_BOS2_STND: statenum_t = 556;
pub const S_BOSS_RAISE7: statenum_t = 555;
pub const S_BOSS_RAISE6: statenum_t = 554;
pub const S_BOSS_RAISE5: statenum_t = 553;
pub const S_BOSS_RAISE4: statenum_t = 552;
pub const S_BOSS_RAISE3: statenum_t = 551;
pub const S_BOSS_RAISE2: statenum_t = 550;
pub const S_BOSS_RAISE1: statenum_t = 549;
pub const S_BOSS_DIE7: statenum_t = 548;
pub const S_BOSS_DIE6: statenum_t = 547;
pub const S_BOSS_DIE5: statenum_t = 546;
pub const S_BOSS_DIE4: statenum_t = 545;
pub const S_BOSS_DIE3: statenum_t = 544;
pub const S_BOSS_DIE2: statenum_t = 543;
pub const S_BOSS_DIE1: statenum_t = 542;
pub const S_BOSS_PAIN2: statenum_t = 541;
pub const S_BOSS_PAIN: statenum_t = 540;
pub const S_BOSS_ATK3: statenum_t = 539;
pub const S_BOSS_ATK2: statenum_t = 538;
pub const S_BOSS_ATK1: statenum_t = 537;
pub const S_BOSS_RUN8: statenum_t = 536;
pub const S_BOSS_RUN7: statenum_t = 535;
pub const S_BOSS_RUN6: statenum_t = 534;
pub const S_BOSS_RUN5: statenum_t = 533;
pub const S_BOSS_RUN4: statenum_t = 532;
pub const S_BOSS_RUN3: statenum_t = 531;
pub const S_BOSS_RUN2: statenum_t = 530;
pub const S_BOSS_RUN1: statenum_t = 529;
pub const S_BOSS_STND2: statenum_t = 528;
pub const S_BOSS_STND: statenum_t = 527;
pub const S_BRBALLX3: statenum_t = 526;
pub const S_BRBALLX2: statenum_t = 525;
pub const S_BRBALLX1: statenum_t = 524;
pub const S_BRBALL2: statenum_t = 523;
pub const S_BRBALL1: statenum_t = 522;
pub const S_HEAD_RAISE6: statenum_t = 521;
pub const S_HEAD_RAISE5: statenum_t = 520;
pub const S_HEAD_RAISE4: statenum_t = 519;
pub const S_HEAD_RAISE3: statenum_t = 518;
pub const S_HEAD_RAISE2: statenum_t = 517;
pub const S_HEAD_RAISE1: statenum_t = 516;
pub const S_HEAD_DIE6: statenum_t = 515;
pub const S_HEAD_DIE5: statenum_t = 514;
pub const S_HEAD_DIE4: statenum_t = 513;
pub const S_HEAD_DIE3: statenum_t = 512;
pub const S_HEAD_DIE2: statenum_t = 511;
pub const S_HEAD_DIE1: statenum_t = 510;
pub const S_HEAD_PAIN3: statenum_t = 509;
pub const S_HEAD_PAIN2: statenum_t = 508;
pub const S_HEAD_PAIN: statenum_t = 507;
pub const S_HEAD_ATK3: statenum_t = 506;
pub const S_HEAD_ATK2: statenum_t = 505;
pub const S_HEAD_ATK1: statenum_t = 504;
pub const S_HEAD_RUN1: statenum_t = 503;
pub const S_HEAD_STND: statenum_t = 502;
pub const S_SARG_RAISE6: statenum_t = 501;
pub const S_SARG_RAISE5: statenum_t = 500;
pub const S_SARG_RAISE4: statenum_t = 499;
pub const S_SARG_RAISE3: statenum_t = 498;
pub const S_SARG_RAISE2: statenum_t = 497;
pub const S_SARG_RAISE1: statenum_t = 496;
pub const S_SARG_DIE6: statenum_t = 495;
pub const S_SARG_DIE5: statenum_t = 494;
pub const S_SARG_DIE4: statenum_t = 493;
pub const S_SARG_DIE3: statenum_t = 492;
pub const S_SARG_DIE2: statenum_t = 491;
pub const S_SARG_DIE1: statenum_t = 490;
pub const S_SARG_PAIN2: statenum_t = 489;
pub const S_SARG_PAIN: statenum_t = 488;
pub const S_SARG_ATK3: statenum_t = 487;
pub const S_SARG_ATK2: statenum_t = 486;
pub const S_SARG_ATK1: statenum_t = 485;
pub const S_SARG_RUN8: statenum_t = 484;
pub const S_SARG_RUN7: statenum_t = 483;
pub const S_SARG_RUN6: statenum_t = 482;
pub const S_SARG_RUN5: statenum_t = 481;
pub const S_SARG_RUN4: statenum_t = 480;
pub const S_SARG_RUN3: statenum_t = 479;
pub const S_SARG_RUN2: statenum_t = 478;
pub const S_SARG_RUN1: statenum_t = 477;
pub const S_SARG_STND2: statenum_t = 476;
pub const S_SARG_STND: statenum_t = 475;
pub const S_TROO_RAISE5: statenum_t = 474;
pub const S_TROO_RAISE4: statenum_t = 473;
pub const S_TROO_RAISE3: statenum_t = 472;
pub const S_TROO_RAISE2: statenum_t = 471;
pub const S_TROO_RAISE1: statenum_t = 470;
pub const S_TROO_XDIE8: statenum_t = 469;
pub const S_TROO_XDIE7: statenum_t = 468;
pub const S_TROO_XDIE6: statenum_t = 467;
pub const S_TROO_XDIE5: statenum_t = 466;
pub const S_TROO_XDIE4: statenum_t = 465;
pub const S_TROO_XDIE3: statenum_t = 464;
pub const S_TROO_XDIE2: statenum_t = 463;
pub const S_TROO_XDIE1: statenum_t = 462;
pub const S_TROO_DIE5: statenum_t = 461;
pub const S_TROO_DIE4: statenum_t = 460;
pub const S_TROO_DIE3: statenum_t = 459;
pub const S_TROO_DIE2: statenum_t = 458;
pub const S_TROO_DIE1: statenum_t = 457;
pub const S_TROO_PAIN2: statenum_t = 456;
pub const S_TROO_PAIN: statenum_t = 455;
pub const S_TROO_ATK3: statenum_t = 454;
pub const S_TROO_ATK2: statenum_t = 453;
pub const S_TROO_ATK1: statenum_t = 452;
pub const S_TROO_RUN8: statenum_t = 451;
pub const S_TROO_RUN7: statenum_t = 450;
pub const S_TROO_RUN6: statenum_t = 449;
pub const S_TROO_RUN5: statenum_t = 448;
pub const S_TROO_RUN4: statenum_t = 447;
pub const S_TROO_RUN3: statenum_t = 446;
pub const S_TROO_RUN2: statenum_t = 445;
pub const S_TROO_RUN1: statenum_t = 444;
pub const S_TROO_STND2: statenum_t = 443;
pub const S_TROO_STND: statenum_t = 442;
pub const S_CPOS_RAISE7: statenum_t = 441;
pub const S_CPOS_RAISE6: statenum_t = 440;
pub const S_CPOS_RAISE5: statenum_t = 439;
pub const S_CPOS_RAISE4: statenum_t = 438;
pub const S_CPOS_RAISE3: statenum_t = 437;
pub const S_CPOS_RAISE2: statenum_t = 436;
pub const S_CPOS_RAISE1: statenum_t = 435;
pub const S_CPOS_XDIE6: statenum_t = 434;
pub const S_CPOS_XDIE5: statenum_t = 433;
pub const S_CPOS_XDIE4: statenum_t = 432;
pub const S_CPOS_XDIE3: statenum_t = 431;
pub const S_CPOS_XDIE2: statenum_t = 430;
pub const S_CPOS_XDIE1: statenum_t = 429;
pub const S_CPOS_DIE7: statenum_t = 428;
pub const S_CPOS_DIE6: statenum_t = 427;
pub const S_CPOS_DIE5: statenum_t = 426;
pub const S_CPOS_DIE4: statenum_t = 425;
pub const S_CPOS_DIE3: statenum_t = 424;
pub const S_CPOS_DIE2: statenum_t = 423;
pub const S_CPOS_DIE1: statenum_t = 422;
pub const S_CPOS_PAIN2: statenum_t = 421;
pub const S_CPOS_PAIN: statenum_t = 420;
pub const S_CPOS_ATK4: statenum_t = 419;
pub const S_CPOS_ATK3: statenum_t = 418;
pub const S_CPOS_ATK2: statenum_t = 417;
pub const S_CPOS_ATK1: statenum_t = 416;
pub const S_CPOS_RUN8: statenum_t = 415;
pub const S_CPOS_RUN7: statenum_t = 414;
pub const S_CPOS_RUN6: statenum_t = 413;
pub const S_CPOS_RUN5: statenum_t = 412;
pub const S_CPOS_RUN4: statenum_t = 411;
pub const S_CPOS_RUN3: statenum_t = 410;
pub const S_CPOS_RUN2: statenum_t = 409;
pub const S_CPOS_RUN1: statenum_t = 408;
pub const S_CPOS_STND2: statenum_t = 407;
pub const S_CPOS_STND: statenum_t = 406;
pub const S_FATT_RAISE8: statenum_t = 405;
pub const S_FATT_RAISE7: statenum_t = 404;
pub const S_FATT_RAISE6: statenum_t = 403;
pub const S_FATT_RAISE5: statenum_t = 402;
pub const S_FATT_RAISE4: statenum_t = 401;
pub const S_FATT_RAISE3: statenum_t = 400;
pub const S_FATT_RAISE2: statenum_t = 399;
pub const S_FATT_RAISE1: statenum_t = 398;
pub const S_FATT_DIE10: statenum_t = 397;
pub const S_FATT_DIE9: statenum_t = 396;
pub const S_FATT_DIE8: statenum_t = 395;
pub const S_FATT_DIE7: statenum_t = 394;
pub const S_FATT_DIE6: statenum_t = 393;
pub const S_FATT_DIE5: statenum_t = 392;
pub const S_FATT_DIE4: statenum_t = 391;
pub const S_FATT_DIE3: statenum_t = 390;
pub const S_FATT_DIE2: statenum_t = 389;
pub const S_FATT_DIE1: statenum_t = 388;
pub const S_FATT_PAIN2: statenum_t = 387;
pub const S_FATT_PAIN: statenum_t = 386;
pub const S_FATT_ATK10: statenum_t = 385;
pub const S_FATT_ATK9: statenum_t = 384;
pub const S_FATT_ATK8: statenum_t = 383;
pub const S_FATT_ATK7: statenum_t = 382;
pub const S_FATT_ATK6: statenum_t = 381;
pub const S_FATT_ATK5: statenum_t = 380;
pub const S_FATT_ATK4: statenum_t = 379;
pub const S_FATT_ATK3: statenum_t = 378;
pub const S_FATT_ATK2: statenum_t = 377;
pub const S_FATT_ATK1: statenum_t = 376;
pub const S_FATT_RUN12: statenum_t = 375;
pub const S_FATT_RUN11: statenum_t = 374;
pub const S_FATT_RUN10: statenum_t = 373;
pub const S_FATT_RUN9: statenum_t = 372;
pub const S_FATT_RUN8: statenum_t = 371;
pub const S_FATT_RUN7: statenum_t = 370;
pub const S_FATT_RUN6: statenum_t = 369;
pub const S_FATT_RUN5: statenum_t = 368;
pub const S_FATT_RUN4: statenum_t = 367;
pub const S_FATT_RUN3: statenum_t = 366;
pub const S_FATT_RUN2: statenum_t = 365;
pub const S_FATT_RUN1: statenum_t = 364;
pub const S_FATT_STND2: statenum_t = 363;
pub const S_FATT_STND: statenum_t = 362;
pub const S_FATSHOTX3: statenum_t = 361;
pub const S_FATSHOTX2: statenum_t = 360;
pub const S_FATSHOTX1: statenum_t = 359;
pub const S_FATSHOT2: statenum_t = 358;
pub const S_FATSHOT1: statenum_t = 357;
pub const S_SKEL_RAISE6: statenum_t = 356;
pub const S_SKEL_RAISE5: statenum_t = 355;
pub const S_SKEL_RAISE4: statenum_t = 354;
pub const S_SKEL_RAISE3: statenum_t = 353;
pub const S_SKEL_RAISE2: statenum_t = 352;
pub const S_SKEL_RAISE1: statenum_t = 351;
pub const S_SKEL_DIE6: statenum_t = 350;
pub const S_SKEL_DIE5: statenum_t = 349;
pub const S_SKEL_DIE4: statenum_t = 348;
pub const S_SKEL_DIE3: statenum_t = 347;
pub const S_SKEL_DIE2: statenum_t = 346;
pub const S_SKEL_DIE1: statenum_t = 345;
pub const S_SKEL_PAIN2: statenum_t = 344;
pub const S_SKEL_PAIN: statenum_t = 343;
pub const S_SKEL_MISS4: statenum_t = 342;
pub const S_SKEL_MISS3: statenum_t = 341;
pub const S_SKEL_MISS2: statenum_t = 340;
pub const S_SKEL_MISS1: statenum_t = 339;
pub const S_SKEL_FIST4: statenum_t = 338;
pub const S_SKEL_FIST3: statenum_t = 337;
pub const S_SKEL_FIST2: statenum_t = 336;
pub const S_SKEL_FIST1: statenum_t = 335;
pub const S_SKEL_RUN12: statenum_t = 334;
pub const S_SKEL_RUN11: statenum_t = 333;
pub const S_SKEL_RUN10: statenum_t = 332;
pub const S_SKEL_RUN9: statenum_t = 331;
pub const S_SKEL_RUN8: statenum_t = 330;
pub const S_SKEL_RUN7: statenum_t = 329;
pub const S_SKEL_RUN6: statenum_t = 328;
pub const S_SKEL_RUN5: statenum_t = 327;
pub const S_SKEL_RUN4: statenum_t = 326;
pub const S_SKEL_RUN3: statenum_t = 325;
pub const S_SKEL_RUN2: statenum_t = 324;
pub const S_SKEL_RUN1: statenum_t = 323;
pub const S_SKEL_STND2: statenum_t = 322;
pub const S_SKEL_STND: statenum_t = 321;
pub const S_TRACEEXP3: statenum_t = 320;
pub const S_TRACEEXP2: statenum_t = 319;
pub const S_TRACEEXP1: statenum_t = 318;
pub const S_TRACER2: statenum_t = 317;
pub const S_TRACER: statenum_t = 316;
pub const S_SMOKE5: statenum_t = 315;
pub const S_SMOKE4: statenum_t = 314;
pub const S_SMOKE3: statenum_t = 313;
pub const S_SMOKE2: statenum_t = 312;
pub const S_SMOKE1: statenum_t = 311;
pub const S_FIRE30: statenum_t = 310;
pub const S_FIRE29: statenum_t = 309;
pub const S_FIRE28: statenum_t = 308;
pub const S_FIRE27: statenum_t = 307;
pub const S_FIRE26: statenum_t = 306;
pub const S_FIRE25: statenum_t = 305;
pub const S_FIRE24: statenum_t = 304;
pub const S_FIRE23: statenum_t = 303;
pub const S_FIRE22: statenum_t = 302;
pub const S_FIRE21: statenum_t = 301;
pub const S_FIRE20: statenum_t = 300;
pub const S_FIRE19: statenum_t = 299;
pub const S_FIRE18: statenum_t = 298;
pub const S_FIRE17: statenum_t = 297;
pub const S_FIRE16: statenum_t = 296;
pub const S_FIRE15: statenum_t = 295;
pub const S_FIRE14: statenum_t = 294;
pub const S_FIRE13: statenum_t = 293;
pub const S_FIRE12: statenum_t = 292;
pub const S_FIRE11: statenum_t = 291;
pub const S_FIRE10: statenum_t = 290;
pub const S_FIRE9: statenum_t = 289;
pub const S_FIRE8: statenum_t = 288;
pub const S_FIRE7: statenum_t = 287;
pub const S_FIRE6: statenum_t = 286;
pub const S_FIRE5: statenum_t = 285;
pub const S_FIRE4: statenum_t = 284;
pub const S_FIRE3: statenum_t = 283;
pub const S_FIRE2: statenum_t = 282;
pub const S_FIRE1: statenum_t = 281;
pub const S_VILE_DIE10: statenum_t = 280;
pub const S_VILE_DIE9: statenum_t = 279;
pub const S_VILE_DIE8: statenum_t = 278;
pub const S_VILE_DIE7: statenum_t = 277;
pub const S_VILE_DIE6: statenum_t = 276;
pub const S_VILE_DIE5: statenum_t = 275;
pub const S_VILE_DIE4: statenum_t = 274;
pub const S_VILE_DIE3: statenum_t = 273;
pub const S_VILE_DIE2: statenum_t = 272;
pub const S_VILE_DIE1: statenum_t = 271;
pub const S_VILE_PAIN2: statenum_t = 270;
pub const S_VILE_PAIN: statenum_t = 269;
pub const S_VILE_HEAL3: statenum_t = 268;
pub const S_VILE_HEAL2: statenum_t = 267;
pub const S_VILE_HEAL1: statenum_t = 266;
pub const S_VILE_ATK11: statenum_t = 265;
pub const S_VILE_ATK10: statenum_t = 264;
pub const S_VILE_ATK9: statenum_t = 263;
pub const S_VILE_ATK8: statenum_t = 262;
pub const S_VILE_ATK7: statenum_t = 261;
pub const S_VILE_ATK6: statenum_t = 260;
pub const S_VILE_ATK5: statenum_t = 259;
pub const S_VILE_ATK4: statenum_t = 258;
pub const S_VILE_ATK3: statenum_t = 257;
pub const S_VILE_ATK2: statenum_t = 256;
pub const S_VILE_ATK1: statenum_t = 255;
pub const S_VILE_RUN12: statenum_t = 254;
pub const S_VILE_RUN11: statenum_t = 253;
pub const S_VILE_RUN10: statenum_t = 252;
pub const S_VILE_RUN9: statenum_t = 251;
pub const S_VILE_RUN8: statenum_t = 250;
pub const S_VILE_RUN7: statenum_t = 249;
pub const S_VILE_RUN6: statenum_t = 248;
pub const S_VILE_RUN5: statenum_t = 247;
pub const S_VILE_RUN4: statenum_t = 246;
pub const S_VILE_RUN3: statenum_t = 245;
pub const S_VILE_RUN2: statenum_t = 244;
pub const S_VILE_RUN1: statenum_t = 243;
pub const S_VILE_STND2: statenum_t = 242;
pub const S_VILE_STND: statenum_t = 241;
pub const S_SPOS_RAISE5: statenum_t = 240;
pub const S_SPOS_RAISE4: statenum_t = 239;
pub const S_SPOS_RAISE3: statenum_t = 238;
pub const S_SPOS_RAISE2: statenum_t = 237;
pub const S_SPOS_RAISE1: statenum_t = 236;
pub const S_SPOS_XDIE9: statenum_t = 235;
pub const S_SPOS_XDIE8: statenum_t = 234;
pub const S_SPOS_XDIE7: statenum_t = 233;
pub const S_SPOS_XDIE6: statenum_t = 232;
pub const S_SPOS_XDIE5: statenum_t = 231;
pub const S_SPOS_XDIE4: statenum_t = 230;
pub const S_SPOS_XDIE3: statenum_t = 229;
pub const S_SPOS_XDIE2: statenum_t = 228;
pub const S_SPOS_XDIE1: statenum_t = 227;
pub const S_SPOS_DIE5: statenum_t = 226;
pub const S_SPOS_DIE4: statenum_t = 225;
pub const S_SPOS_DIE3: statenum_t = 224;
pub const S_SPOS_DIE2: statenum_t = 223;
pub const S_SPOS_DIE1: statenum_t = 222;
pub const S_SPOS_PAIN2: statenum_t = 221;
pub const S_SPOS_PAIN: statenum_t = 220;
pub const S_SPOS_ATK3: statenum_t = 219;
pub const S_SPOS_ATK2: statenum_t = 218;
pub const S_SPOS_ATK1: statenum_t = 217;
pub const S_SPOS_RUN8: statenum_t = 216;
pub const S_SPOS_RUN7: statenum_t = 215;
pub const S_SPOS_RUN6: statenum_t = 214;
pub const S_SPOS_RUN5: statenum_t = 213;
pub const S_SPOS_RUN4: statenum_t = 212;
pub const S_SPOS_RUN3: statenum_t = 211;
pub const S_SPOS_RUN2: statenum_t = 210;
pub const S_SPOS_RUN1: statenum_t = 209;
pub const S_SPOS_STND2: statenum_t = 208;
pub const S_SPOS_STND: statenum_t = 207;
pub const S_POSS_RAISE4: statenum_t = 206;
pub const S_POSS_RAISE3: statenum_t = 205;
pub const S_POSS_RAISE2: statenum_t = 204;
pub const S_POSS_RAISE1: statenum_t = 203;
pub const S_POSS_XDIE9: statenum_t = 202;
pub const S_POSS_XDIE8: statenum_t = 201;
pub const S_POSS_XDIE7: statenum_t = 200;
pub const S_POSS_XDIE6: statenum_t = 199;
pub const S_POSS_XDIE5: statenum_t = 198;
pub const S_POSS_XDIE4: statenum_t = 197;
pub const S_POSS_XDIE3: statenum_t = 196;
pub const S_POSS_XDIE2: statenum_t = 195;
pub const S_POSS_XDIE1: statenum_t = 194;
pub const S_POSS_DIE5: statenum_t = 193;
pub const S_POSS_DIE4: statenum_t = 192;
pub const S_POSS_DIE3: statenum_t = 191;
pub const S_POSS_DIE2: statenum_t = 190;
pub const S_POSS_DIE1: statenum_t = 189;
pub const S_POSS_PAIN2: statenum_t = 188;
pub const S_POSS_PAIN: statenum_t = 187;
pub const S_POSS_ATK3: statenum_t = 186;
pub const S_POSS_ATK2: statenum_t = 185;
pub const S_POSS_ATK1: statenum_t = 184;
pub const S_POSS_RUN8: statenum_t = 183;
pub const S_POSS_RUN7: statenum_t = 182;
pub const S_POSS_RUN6: statenum_t = 181;
pub const S_POSS_RUN5: statenum_t = 180;
pub const S_POSS_RUN4: statenum_t = 179;
pub const S_POSS_RUN3: statenum_t = 178;
pub const S_POSS_RUN2: statenum_t = 177;
pub const S_POSS_RUN1: statenum_t = 176;
pub const S_POSS_STND2: statenum_t = 175;
pub const S_POSS_STND: statenum_t = 174;
pub const S_PLAY_XDIE9: statenum_t = 173;
pub const S_PLAY_XDIE8: statenum_t = 172;
pub const S_PLAY_XDIE7: statenum_t = 171;
pub const S_PLAY_XDIE6: statenum_t = 170;
pub const S_PLAY_XDIE5: statenum_t = 169;
pub const S_PLAY_XDIE4: statenum_t = 168;
pub const S_PLAY_XDIE3: statenum_t = 167;
pub const S_PLAY_XDIE2: statenum_t = 166;
pub const S_PLAY_XDIE1: statenum_t = 165;
pub const S_PLAY_DIE7: statenum_t = 164;
pub const S_PLAY_DIE6: statenum_t = 163;
pub const S_PLAY_DIE5: statenum_t = 162;
pub const S_PLAY_DIE4: statenum_t = 161;
pub const S_PLAY_DIE3: statenum_t = 160;
pub const S_PLAY_DIE2: statenum_t = 159;
pub const S_PLAY_DIE1: statenum_t = 158;
pub const S_PLAY_PAIN2: statenum_t = 157;
pub const S_PLAY_PAIN: statenum_t = 156;
pub const S_PLAY_ATK2: statenum_t = 155;
pub const S_PLAY_ATK1: statenum_t = 154;
pub const S_PLAY_RUN4: statenum_t = 153;
pub const S_PLAY_RUN3: statenum_t = 152;
pub const S_PLAY_RUN2: statenum_t = 151;
pub const S_PLAY_RUN1: statenum_t = 150;
pub const S_PLAY: statenum_t = 149;
pub const S_IFOG5: statenum_t = 148;
pub const S_IFOG4: statenum_t = 147;
pub const S_IFOG3: statenum_t = 146;
pub const S_IFOG2: statenum_t = 145;
pub const S_IFOG02: statenum_t = 144;
pub const S_IFOG01: statenum_t = 143;
pub const S_IFOG: statenum_t = 142;
pub const S_TFOG10: statenum_t = 141;
pub const S_TFOG9: statenum_t = 140;
pub const S_TFOG8: statenum_t = 139;
pub const S_TFOG7: statenum_t = 138;
pub const S_TFOG6: statenum_t = 137;
pub const S_TFOG5: statenum_t = 136;
pub const S_TFOG4: statenum_t = 135;
pub const S_TFOG3: statenum_t = 134;
pub const S_TFOG2: statenum_t = 133;
pub const S_TFOG02: statenum_t = 132;
pub const S_TFOG01: statenum_t = 131;
pub const S_TFOG: statenum_t = 130;
pub const S_EXPLODE3: statenum_t = 129;
pub const S_EXPLODE2: statenum_t = 128;
pub const S_EXPLODE1: statenum_t = 127;
pub const S_BFGEXP4: statenum_t = 126;
pub const S_BFGEXP3: statenum_t = 125;
pub const S_BFGEXP2: statenum_t = 124;
pub const S_BFGEXP: statenum_t = 123;
pub const S_BFGLAND6: statenum_t = 122;
pub const S_BFGLAND5: statenum_t = 121;
pub const S_BFGLAND4: statenum_t = 120;
pub const S_BFGLAND3: statenum_t = 119;
pub const S_BFGLAND2: statenum_t = 118;
pub const S_BFGLAND: statenum_t = 117;
pub const S_BFGSHOT2: statenum_t = 116;
pub const S_BFGSHOT: statenum_t = 115;
pub const S_ROCKET: statenum_t = 114;
pub const S_PLASEXP5: statenum_t = 113;
pub const S_PLASEXP4: statenum_t = 112;
pub const S_PLASEXP3: statenum_t = 111;
pub const S_PLASEXP2: statenum_t = 110;
pub const S_PLASEXP: statenum_t = 109;
pub const S_PLASBALL2: statenum_t = 108;
pub const S_PLASBALL: statenum_t = 107;
pub const S_RBALLX3: statenum_t = 106;
pub const S_RBALLX2: statenum_t = 105;
pub const S_RBALLX1: statenum_t = 104;
pub const S_RBALL2: statenum_t = 103;
pub const S_RBALL1: statenum_t = 102;
pub const S_TBALLX3: statenum_t = 101;
pub const S_TBALLX2: statenum_t = 100;
pub const S_TBALLX1: statenum_t = 99;
pub const S_TBALL2: statenum_t = 98;
pub const S_TBALL1: statenum_t = 97;
pub const S_PUFF4: statenum_t = 96;
pub const S_PUFF3: statenum_t = 95;
pub const S_PUFF2: statenum_t = 94;
pub const S_PUFF1: statenum_t = 93;
pub const S_BLOOD3: statenum_t = 92;
pub const S_BLOOD2: statenum_t = 91;
pub const S_BLOOD1: statenum_t = 90;
pub const S_BFGFLASH2: statenum_t = 89;
pub const S_BFGFLASH1: statenum_t = 88;
pub const S_BFG4: statenum_t = 87;
pub const S_BFG3: statenum_t = 86;
pub const S_BFG2: statenum_t = 85;
pub const S_BFG1: statenum_t = 84;
pub const S_BFGUP: statenum_t = 83;
pub const S_BFGDOWN: statenum_t = 82;
pub const S_BFG: statenum_t = 81;
pub const S_PLASMAFLASH2: statenum_t = 80;
pub const S_PLASMAFLASH1: statenum_t = 79;
pub const S_PLASMA2: statenum_t = 78;
pub const S_PLASMA1: statenum_t = 77;
pub const S_PLASMAUP: statenum_t = 76;
pub const S_PLASMADOWN: statenum_t = 75;
pub const S_PLASMA: statenum_t = 74;
pub const S_SAW3: statenum_t = 73;
pub const S_SAW2: statenum_t = 72;
pub const S_SAW1: statenum_t = 71;
pub const S_SAWUP: statenum_t = 70;
pub const S_SAWDOWN: statenum_t = 69;
pub const S_SAWB: statenum_t = 68;
pub const S_SAW: statenum_t = 67;
pub const S_MISSILEFLASH4: statenum_t = 66;
pub const S_MISSILEFLASH3: statenum_t = 65;
pub const S_MISSILEFLASH2: statenum_t = 64;
pub const S_MISSILEFLASH1: statenum_t = 63;
pub const S_MISSILE3: statenum_t = 62;
pub const S_MISSILE2: statenum_t = 61;
pub const S_MISSILE1: statenum_t = 60;
pub const S_MISSILEUP: statenum_t = 59;
pub const S_MISSILEDOWN: statenum_t = 58;
pub const S_MISSILE: statenum_t = 57;
pub const S_CHAINFLASH2: statenum_t = 56;
pub const S_CHAINFLASH1: statenum_t = 55;
pub const S_CHAIN3: statenum_t = 54;
pub const S_CHAIN2: statenum_t = 53;
pub const S_CHAIN1: statenum_t = 52;
pub const S_CHAINUP: statenum_t = 51;
pub const S_CHAINDOWN: statenum_t = 50;
pub const S_CHAIN: statenum_t = 49;
pub const S_DSGUNFLASH2: statenum_t = 48;
pub const S_DSGUNFLASH1: statenum_t = 47;
pub const S_DSNR2: statenum_t = 46;
pub const S_DSNR1: statenum_t = 45;
pub const S_DSGUN10: statenum_t = 44;
pub const S_DSGUN9: statenum_t = 43;
pub const S_DSGUN8: statenum_t = 42;
pub const S_DSGUN7: statenum_t = 41;
pub const S_DSGUN6: statenum_t = 40;
pub const S_DSGUN5: statenum_t = 39;
pub const S_DSGUN4: statenum_t = 38;
pub const S_DSGUN3: statenum_t = 37;
pub const S_DSGUN2: statenum_t = 36;
pub const S_DSGUN1: statenum_t = 35;
pub const S_DSGUNUP: statenum_t = 34;
pub const S_DSGUNDOWN: statenum_t = 33;
pub const S_DSGUN: statenum_t = 32;
pub const S_SGUNFLASH2: statenum_t = 31;
pub const S_SGUNFLASH1: statenum_t = 30;
pub const S_SGUN9: statenum_t = 29;
pub const S_SGUN8: statenum_t = 28;
pub const S_SGUN7: statenum_t = 27;
pub const S_SGUN6: statenum_t = 26;
pub const S_SGUN5: statenum_t = 25;
pub const S_SGUN4: statenum_t = 24;
pub const S_SGUN3: statenum_t = 23;
pub const S_SGUN2: statenum_t = 22;
pub const S_SGUN1: statenum_t = 21;
pub const S_SGUNUP: statenum_t = 20;
pub const S_SGUNDOWN: statenum_t = 19;
pub const S_SGUN: statenum_t = 18;
pub const S_PISTOLFLASH: statenum_t = 17;
pub const S_PISTOL4: statenum_t = 16;
pub const S_PISTOL3: statenum_t = 15;
pub const S_PISTOL2: statenum_t = 14;
pub const S_PISTOL1: statenum_t = 13;
pub const S_PISTOLUP: statenum_t = 12;
pub const S_PISTOLDOWN: statenum_t = 11;
pub const S_PISTOL: statenum_t = 10;
pub const S_PUNCH5: statenum_t = 9;
pub const S_PUNCH4: statenum_t = 8;
pub const S_PUNCH3: statenum_t = 7;
pub const S_PUNCH2: statenum_t = 6;
pub const S_PUNCH1: statenum_t = 5;
pub const S_PUNCHUP: statenum_t = 4;
pub const S_PUNCHDOWN: statenum_t = 3;
pub const S_PUNCH: statenum_t = 2;
pub const S_LIGHTDONE: statenum_t = 1;
pub const S_NULL: statenum_t = 0;
pub type mobjtype_t = u32;
pub const NUMMOBJTYPES: mobjtype_t = 137;
pub const MT_MISC86: mobjtype_t = 136;
pub const MT_MISC85: mobjtype_t = 135;
pub const MT_MISC84: mobjtype_t = 134;
pub const MT_MISC83: mobjtype_t = 133;
pub const MT_MISC82: mobjtype_t = 132;
pub const MT_MISC81: mobjtype_t = 131;
pub const MT_MISC80: mobjtype_t = 130;
pub const MT_MISC79: mobjtype_t = 129;
pub const MT_MISC78: mobjtype_t = 128;
pub const MT_MISC77: mobjtype_t = 127;
pub const MT_MISC76: mobjtype_t = 126;
pub const MT_MISC75: mobjtype_t = 125;
pub const MT_MISC74: mobjtype_t = 124;
pub const MT_MISC73: mobjtype_t = 123;
pub const MT_MISC72: mobjtype_t = 122;
pub const MT_MISC71: mobjtype_t = 121;
pub const MT_MISC70: mobjtype_t = 120;
pub const MT_MISC69: mobjtype_t = 119;
pub const MT_MISC68: mobjtype_t = 118;
pub const MT_MISC67: mobjtype_t = 117;
pub const MT_MISC66: mobjtype_t = 116;
pub const MT_MISC65: mobjtype_t = 115;
pub const MT_MISC64: mobjtype_t = 114;
pub const MT_MISC63: mobjtype_t = 113;
pub const MT_MISC62: mobjtype_t = 112;
pub const MT_MISC61: mobjtype_t = 111;
pub const MT_MISC60: mobjtype_t = 110;
pub const MT_MISC59: mobjtype_t = 109;
pub const MT_MISC58: mobjtype_t = 108;
pub const MT_MISC57: mobjtype_t = 107;
pub const MT_MISC56: mobjtype_t = 106;
pub const MT_MISC55: mobjtype_t = 105;
pub const MT_MISC54: mobjtype_t = 104;
pub const MT_MISC53: mobjtype_t = 103;
pub const MT_MISC52: mobjtype_t = 102;
pub const MT_MISC51: mobjtype_t = 101;
pub const MT_MISC50: mobjtype_t = 100;
pub const MT_MISC49: mobjtype_t = 99;
pub const MT_MISC48: mobjtype_t = 98;
pub const MT_MISC47: mobjtype_t = 97;
pub const MT_MISC46: mobjtype_t = 96;
pub const MT_MISC45: mobjtype_t = 95;
pub const MT_MISC44: mobjtype_t = 94;
pub const MT_MISC43: mobjtype_t = 93;
pub const MT_MISC42: mobjtype_t = 92;
pub const MT_MISC41: mobjtype_t = 91;
pub const MT_MISC40: mobjtype_t = 90;
pub const MT_MISC39: mobjtype_t = 89;
pub const MT_MISC38: mobjtype_t = 88;
pub const MT_MISC37: mobjtype_t = 87;
pub const MT_MISC36: mobjtype_t = 86;
pub const MT_MISC35: mobjtype_t = 85;
pub const MT_MISC34: mobjtype_t = 84;
pub const MT_MISC33: mobjtype_t = 83;
pub const MT_MISC32: mobjtype_t = 82;
pub const MT_MISC31: mobjtype_t = 81;
pub const MT_MISC30: mobjtype_t = 80;
pub const MT_MISC29: mobjtype_t = 79;
pub const MT_SUPERSHOTGUN: mobjtype_t = 78;
pub const MT_SHOTGUN: mobjtype_t = 77;
pub const MT_MISC28: mobjtype_t = 76;
pub const MT_MISC27: mobjtype_t = 75;
pub const MT_MISC26: mobjtype_t = 74;
pub const MT_CHAINGUN: mobjtype_t = 73;
pub const MT_MISC25: mobjtype_t = 72;
pub const MT_MISC24: mobjtype_t = 71;
pub const MT_MISC23: mobjtype_t = 70;
pub const MT_MISC22: mobjtype_t = 69;
pub const MT_MISC21: mobjtype_t = 68;
pub const MT_MISC20: mobjtype_t = 67;
pub const MT_MISC19: mobjtype_t = 66;
pub const MT_MISC18: mobjtype_t = 65;
pub const MT_MISC17: mobjtype_t = 64;
pub const MT_CLIP: mobjtype_t = 63;
pub const MT_MEGA: mobjtype_t = 62;
pub const MT_MISC16: mobjtype_t = 61;
pub const MT_MISC15: mobjtype_t = 60;
pub const MT_MISC14: mobjtype_t = 59;
pub const MT_INS: mobjtype_t = 58;
pub const MT_MISC13: mobjtype_t = 57;
pub const MT_INV: mobjtype_t = 56;
pub const MT_MISC12: mobjtype_t = 55;
pub const MT_MISC11: mobjtype_t = 54;
pub const MT_MISC10: mobjtype_t = 53;
pub const MT_MISC9: mobjtype_t = 52;
pub const MT_MISC8: mobjtype_t = 51;
pub const MT_MISC7: mobjtype_t = 50;
pub const MT_MISC6: mobjtype_t = 49;
pub const MT_MISC5: mobjtype_t = 48;
pub const MT_MISC4: mobjtype_t = 47;
pub const MT_MISC3: mobjtype_t = 46;
pub const MT_MISC2: mobjtype_t = 45;
pub const MT_MISC1: mobjtype_t = 44;
pub const MT_MISC0: mobjtype_t = 43;
pub const MT_EXTRABFG: mobjtype_t = 42;
pub const MT_TELEPORTMAN: mobjtype_t = 41;
pub const MT_IFOG: mobjtype_t = 40;
pub const MT_TFOG: mobjtype_t = 39;
pub const MT_BLOOD: mobjtype_t = 38;
pub const MT_PUFF: mobjtype_t = 37;
pub const MT_ARACHPLAZ: mobjtype_t = 36;
pub const MT_BFG: mobjtype_t = 35;
pub const MT_PLASMA: mobjtype_t = 34;
pub const MT_ROCKET: mobjtype_t = 33;
pub const MT_HEADSHOT: mobjtype_t = 32;
pub const MT_TROOPSHOT: mobjtype_t = 31;
pub const MT_BARREL: mobjtype_t = 30;
pub const MT_SPAWNFIRE: mobjtype_t = 29;
pub const MT_SPAWNSHOT: mobjtype_t = 28;
pub const MT_BOSSTARGET: mobjtype_t = 27;
pub const MT_BOSSSPIT: mobjtype_t = 26;
pub const MT_BOSSBRAIN: mobjtype_t = 25;
pub const MT_KEEN: mobjtype_t = 24;
pub const MT_WOLFSS: mobjtype_t = 23;
pub const MT_PAIN: mobjtype_t = 22;
pub const MT_CYBORG: mobjtype_t = 21;
pub const MT_BABY: mobjtype_t = 20;
pub const MT_SPIDER: mobjtype_t = 19;
pub const MT_SKULL: mobjtype_t = 18;
pub const MT_KNIGHT: mobjtype_t = 17;
pub const MT_BRUISERSHOT: mobjtype_t = 16;
pub const MT_BRUISER: mobjtype_t = 15;
pub const MT_HEAD: mobjtype_t = 14;
pub const MT_SHADOWS: mobjtype_t = 13;
pub const MT_SERGEANT: mobjtype_t = 12;
pub const MT_TROOP: mobjtype_t = 11;
pub const MT_CHAINGUY: mobjtype_t = 10;
pub const MT_FATSHOT: mobjtype_t = 9;
pub const MT_FATSO: mobjtype_t = 8;
pub const MT_SMOKE: mobjtype_t = 7;
pub const MT_TRACER: mobjtype_t = 6;
pub const MT_UNDEAD: mobjtype_t = 5;
pub const MT_FIRE: mobjtype_t = 4;
pub const MT_VILE: mobjtype_t = 3;
pub const MT_SHOTGUY: mobjtype_t = 2;
pub const MT_POSSESSED: mobjtype_t = 1;
pub const MT_PLAYER: mobjtype_t = 0;
pub type C2RustUnnamed_0 = u32;
pub const NUMMUSIC: C2RustUnnamed_0 = 68;
pub const mus_dm2int: C2RustUnnamed_0 = 67;
pub const mus_dm2ttl: C2RustUnnamed_0 = 66;
pub const mus_read_m: C2RustUnnamed_0 = 65;
pub const mus_ultima: C2RustUnnamed_0 = 64;
pub const mus_evil: C2RustUnnamed_0 = 63;
pub const mus_openin: C2RustUnnamed_0 = 62;
pub const mus_shawn3: C2RustUnnamed_0 = 61;
pub const mus_tense: C2RustUnnamed_0 = 60;
pub const mus_romer2: C2RustUnnamed_0 = 59;
pub const mus_messg2: C2RustUnnamed_0 = 58;
pub const mus_adrian: C2RustUnnamed_0 = 57;
pub const mus_theda3: C2RustUnnamed_0 = 56;
pub const mus_ampie: C2RustUnnamed_0 = 55;
pub const mus_ddtbl3: C2RustUnnamed_0 = 54;
pub const mus_count2: C2RustUnnamed_0 = 53;
pub const mus_messag: C2RustUnnamed_0 = 52;
pub const mus_shawn2: C2RustUnnamed_0 = 51;
pub const mus_romero: C2RustUnnamed_0 = 50;
pub const mus_stlks3: C2RustUnnamed_0 = 49;
pub const mus_dead2: C2RustUnnamed_0 = 48;
pub const mus_runni2: C2RustUnnamed_0 = 47;
pub const mus_ddtbl2: C2RustUnnamed_0 = 46;
pub const mus_doom2: C2RustUnnamed_0 = 45;
pub const mus_theda2: C2RustUnnamed_0 = 44;
pub const mus_stlks2: C2RustUnnamed_0 = 43;
pub const mus_dead: C2RustUnnamed_0 = 42;
pub const mus_in_cit: C2RustUnnamed_0 = 41;
pub const mus_ddtblu: C2RustUnnamed_0 = 40;
pub const mus_shawn: C2RustUnnamed_0 = 39;
pub const mus_the_da: C2RustUnnamed_0 = 38;
pub const mus_doom: C2RustUnnamed_0 = 37;
pub const mus_betwee: C2RustUnnamed_0 = 36;
pub const mus_countd: C2RustUnnamed_0 = 35;
pub const mus_stalks: C2RustUnnamed_0 = 34;
pub const mus_runnin: C2RustUnnamed_0 = 33;
pub const mus_introa: C2RustUnnamed_0 = 32;
pub const mus_victor: C2RustUnnamed_0 = 31;
pub const mus_bunny: C2RustUnnamed_0 = 30;
pub const mus_intro: C2RustUnnamed_0 = 29;
pub const mus_inter: C2RustUnnamed_0 = 28;
pub const mus_e3m9: C2RustUnnamed_0 = 27;
pub const mus_e3m8: C2RustUnnamed_0 = 26;
pub const mus_e3m7: C2RustUnnamed_0 = 25;
pub const mus_e3m6: C2RustUnnamed_0 = 24;
pub const mus_e3m5: C2RustUnnamed_0 = 23;
pub const mus_e3m4: C2RustUnnamed_0 = 22;
pub const mus_e3m3: C2RustUnnamed_0 = 21;
pub const mus_e3m2: C2RustUnnamed_0 = 20;
pub const mus_e3m1: C2RustUnnamed_0 = 19;
pub const mus_e2m9: C2RustUnnamed_0 = 18;
pub const mus_e2m8: C2RustUnnamed_0 = 17;
pub const mus_e2m7: C2RustUnnamed_0 = 16;
pub const mus_e2m6: C2RustUnnamed_0 = 15;
pub const mus_e2m5: C2RustUnnamed_0 = 14;
pub const mus_e2m4: C2RustUnnamed_0 = 13;
pub const mus_e2m3: C2RustUnnamed_0 = 12;
pub const mus_e2m2: C2RustUnnamed_0 = 11;
pub const mus_e2m1: C2RustUnnamed_0 = 10;
pub const mus_e1m9: C2RustUnnamed_0 = 9;
pub const mus_e1m8: C2RustUnnamed_0 = 8;
pub const mus_e1m7: C2RustUnnamed_0 = 7;
pub const mus_e1m6: C2RustUnnamed_0 = 6;
pub const mus_e1m5: C2RustUnnamed_0 = 5;
pub const mus_e1m4: C2RustUnnamed_0 = 4;
pub const mus_e1m3: C2RustUnnamed_0 = 3;
pub const mus_e1m2: C2RustUnnamed_0 = 2;
pub const mus_e1m1: C2RustUnnamed_0 = 1;
pub const mus_None: C2RustUnnamed_0 = 0;
pub type evtype_t = u32;
pub const ev_quit: evtype_t = 4;
pub const ev_joystick: evtype_t = 3;
pub const ev_mouse: evtype_t = 2;
pub const ev_keyup: evtype_t = 1;
pub const ev_keydown: evtype_t = 0;
pub type C2RustUnnamed_2 = u32;
pub const wipe_NUMWIPES: C2RustUnnamed_2 = 2;
pub const wipe_Melt: C2RustUnnamed_2 = 1;
pub const wipe_ColorXForm: C2RustUnnamed_2 = 0;
pub type atexit_func_t = Option<unsafe extern "C" fn() -> ()>;
pub type grabmouse_callback_t = Option<unsafe extern "C" fn() -> boolean>;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PACKAGE_STRING: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"Doom Generic 0.1\0")
};
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const TICRATE: i32 = 35 as i32;
pub const MAXPLAYERS: i32 = 4 as i32;
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
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
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
pub static mut show_endoom: i32 = 1 as i32;
#[no_mangle]
pub unsafe extern "C" fn D_ProcessEvents() {
    let mut ev: *mut event_t = ::core::ptr::null_mut::<event_t>();
    if storedemo {
        return;
    }
    loop {
        ev = D_PopEvent();
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
#[no_mangle]
pub unsafe extern "C" fn D_Display() {
    static mut viewactivestate: bool = false;
    static mut menuactivestate: bool = false;
    static mut inhelpscreensstate: bool = false;
    static mut fullscreen: bool = false;
    static mut oldgamestate: gamestate_t = 4294967295 as gamestate_t;
    static mut borderdrawcount: i32 = 0;
    let mut nowtime: i32 = 0;
    let mut tics: i32 = 0;
    let mut wipestart: i32 = 0;
    let mut y: i32 = 0;
    let mut done: boolean = 0;
    let mut wipe: boolean = 0;
    let mut redrawsbar: boolean = 0;
    if nodrawers {
        return;
    }
    redrawsbar = false_0 as boolean;
    if setsizeneeded {
        R_ExecuteSetViewSize();
        oldgamestate = 4294967295 as gamestate_t;
        borderdrawcount = 3 as i32;
    }
    if gamestate as u32 != wipegamestate as u32 {
        wipe = true_0 as boolean;
        wipe_StartScreen(
            0 as i32,
            0 as i32,
            SCREENWIDTH,
            SCREENHEIGHT,
        );
    } else {
        wipe = false_0 as boolean;
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
                if wipe != 0
                    || viewheight != 200 as i32 && fullscreen
                {
                    redrawsbar = true_0 as boolean;
                }
                if inhelpscreensstate && !inhelpscreens {
                    redrawsbar = true_0 as boolean;
                }
                ST_Drawer(
                    viewheight == 200 as i32,
                    redrawsbar != 0,
                );
                fullscreen = viewheight == 200 as i32;
            }
        }
        1 => {
            WI_Drawer();
        }
        2 => {
            F_Drawer();
        }
        3 => {
            D_PageDrawer();
        }
        _ => {}
    }
    I_UpdateNoBlit();
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
    if wipe == 0 {
        I_FinishUpdate();
        return;
    }
    wipe_EndScreen(
        0 as i32,
        0 as i32,
        SCREENWIDTH,
        SCREENHEIGHT,
    );
    wipestart = I_GetTime() - 1 as i32;
    loop {
        loop {
            nowtime = I_GetTime();
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
        ) as boolean;
        I_UpdateNoBlit();
        M_Drawer();
        I_FinishUpdate();
        if !(done == 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn D_BindVariables() {
    let mut i: i32 = 0;
    M_ApplyPlatformDefaults();
    I_BindVideoVariables();
    I_BindJoystickVariables();
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
#[no_mangle]
pub unsafe extern "C" fn D_GrabMouseCallback() -> boolean {
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
    I_StartFrame();
    TryRunTics();
    S_UpdateSounds(players[consoleplayer as usize].mo);
    if screenvisible {
        D_Display();
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_DoomLoop() {
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
    I_GraphicsCheckCommandLine();
    I_SetGrabMouseCallback(
        Some(D_GrabMouseCallback as unsafe extern "C" fn() -> boolean),
    );
    I_InitGraphics();
    I_EnableLoadingDisk();
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
#[no_mangle]
pub unsafe extern "C" fn D_PageDrawer() {
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
unsafe extern "C" fn GetGameName(
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
unsafe extern "C" fn SetMissionForPackName(mut pack_name: *mut ::core::ffi::c_char) {
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
#[no_mangle]
pub unsafe extern "C" fn D_IdentifyVersion() {
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
#[no_mangle]
pub unsafe extern "C" fn D_SetGameDescription() {
    let mut is_freedoom: boolean = (W_CheckNumForName("FREEDOOM",
    ) >= 0 as i32) as i32 as boolean;
    let mut is_freedm: boolean = (W_CheckNumForName("FREEDM",
    ) >= 0 as i32) as i32 as boolean;
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
        if is_freedoom != 0 {
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
    } else if is_freedoom != 0 {
        if is_freedm != 0 {
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
unsafe extern "C" fn D_AddFile(mut filename: *mut ::core::ffi::c_char) -> bool {
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
#[no_mangle]
pub unsafe extern "C" fn PrintDehackedBanners() {
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
unsafe extern "C" fn InitGameVersion() {
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
#[no_mangle]
pub unsafe extern "C" fn PrintGameVersion() {
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
    let mut endoom: *mut byte = ::core::ptr::null_mut::<byte>();
    if show_endoom == 0 || !main_loop_started || screensaver_mode
        || M_CheckParm("-testcontrols") > 0 as i32
    {
        return;
    }
    endoom = W_CacheLumpName("ENDOOM",
        PU_STATIC as i32,
    ) as *mut byte;
    I_Endoom(endoom);
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
    I_DisplayFPSDots(devparm);
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
    V_Init();
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
    iwadfile = D_FindIWAD(
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
    I_CheckIsScreensaver();
    I_InitTimer();
    I_InitJoystick();
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
