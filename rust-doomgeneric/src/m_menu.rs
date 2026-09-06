use crate::src::i_system::FILE;
use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::i_system::I_Error;
use crate::src::dstrings::{doom1_endmsg, doom2_endmsg};
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName};
use crate::src::i_timer::I_WaitVBL;
use crate::src::d_main::D_StartTitle;
use crate::src::i_input::vanilla_keyboard_mapping;
use crate::src::i_video::usegamma;
use crate::src::r_main::R_SetViewSize;
use crate::src::g_game::G_SaveGame;
use crate::src::g_game::G_ScreenShot;
use crate::src::m_controls::key_menu_activate;
use crate::src::m_controls::key_menu_up;
use crate::src::m_controls::key_menu_down;
use crate::src::m_controls::key_menu_left;
use crate::src::m_controls::key_menu_right;
use crate::src::m_controls::key_menu_back;
use crate::src::m_controls::key_menu_forward;
use crate::src::m_controls::key_menu_confirm;
use crate::src::m_controls::key_menu_abort;
use crate::src::m_controls::key_menu_help;
use crate::src::m_controls::key_menu_save;
use crate::src::m_controls::key_menu_load;
use crate::src::m_controls::key_menu_volume;
use crate::src::m_controls::key_menu_detail;
use crate::src::m_controls::key_menu_qsave;
use crate::src::m_controls::key_menu_endgame;
use crate::src::m_controls::key_menu_messages;
use crate::src::m_controls::key_menu_qload;
use crate::src::m_controls::key_menu_quit;
use crate::src::m_controls::key_menu_gamma;
use crate::src::m_controls::key_menu_incscreen;
use crate::src::m_controls::key_menu_decscreen;
use crate::src::m_controls::key_menu_screenshot;
use crate::src::m_controls::joybmenu;
use crate::src::s_sound::S_SetMusicVolume;
use crate::src::s_sound::S_SetSfxVolume;
use crate::src::d_main::devparm;
use crate::src::hu_stuff::message_dontfuckwithme;
use crate::src::hu_stuff::chat_on;
use crate::src::g_game::G_LoadGame;
use crate::src::g_game::G_DeferedInitNew;
use crate::src::g_game::usergame;
use crate::src::g_game::testcontrols;
use crate::src::hu_stuff::hu_font;
use crate::src::i_system::I_Quit;
use crate::src::s_sound::sfxVolume;
use crate::src::s_sound::musicVolume;
use crate::src::g_game::gamestate;
use crate::src::i_video::I_SetPalette;
use crate::src::p_saveg::P_SaveGameFile;
use crate::src::v_video::V_DrawPatchDirect;
use crate::src::d_loop::gametic;
use crate::src::g_game::demoplayback;
use crate::src::doomstat::gamemission;
use crate::src::am_map::automapactive;
use crate::src::m_misc::M_StringCopy;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::i_timer::I_GetTime;
use crate::src::z_zone::PU_CACHE;
use crate::src::sounds::{sfx_boscub, sfx_bspact, sfx_dmpain, sfx_getpow, sfx_kntdth, sfx_oof, sfx_pistol, sfx_pldeth, sfx_popain, sfx_posit1, sfx_posit3, sfx_pstop, sfx_sgtatk, sfx_skeswg, sfx_slop, sfx_stnmov, sfx_swtchn, sfx_swtchx, sfx_telept, sfx_vilact};
use libc::toupper;
use libc::snprintf;
use crate::src::m_misc::__ctype_toupper_loc;
use crate::src::i_system::{fclose, fopen, fprintf, fread, stderr};
use crate::src::p_mobj::mobjtype_t;
use crate::src::d_mode::{commercial, registered, retail, shareware};
use crate::src::d_mode::{exe_chex, exe_doom_1_9, exe_ultimate};
use crate::src::d_mode::{doom, doom2, pack_chex, pack_hacx};
use crate::src::d_mode::skill_t;
use crate::src::d_event::{ev_joystick, ev_keydown, ev_mouse, ev_quit};
use crate::src::d_event::GS_LEVEL;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::__int32_t;
use crate::src::stdint_types::size_t;

pub const NUMMOBJTYPES: mobjtype_t = 137;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuitem_t {
    pub status: i16,
    pub name: [::core::ffi::c_char; 10],
    pub routine: Option<unsafe extern "C" fn(i32) -> ()>,
    pub alphaKey: ::core::ffi::c_char,
}
pub type menu_t = menu_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_s {
    pub numitems: i16,
    pub prevMenu: *mut menu_s,
    pub menuitems: *mut menuitem_t,
    pub routine: Option<unsafe extern "C" fn() -> ()>,
    pub x: i16,
    pub y: i16,
    pub lastOn: i16,
}
pub const read2_end: C2RustUnnamed_6 = 1;
pub const read1_end: C2RustUnnamed_5 = 1;
pub const load_end: C2RustUnnamed_8 = 6;
pub const scrnsize: C2RustUnnamed_4 = 3;
pub const mousesens: C2RustUnnamed_4 = 5;
pub const messages: C2RustUnnamed_4 = 1;
pub const detail: C2RustUnnamed_4 = 2;
pub const music_vol: C2RustUnnamed_7 = 2;
pub const sfx_vol: C2RustUnnamed_7 = 0;
pub const sound_end: C2RustUnnamed_7 = 4;
pub const opt_end: C2RustUnnamed_4 = 8;
pub const ep1: C2RustUnnamed_2 = 0;
pub const hurtme: C2RustUnnamed_3 = 2;
pub const nightmare: C2RustUnnamed_3 = 4;
pub const newg_end: C2RustUnnamed_3 = 5;
pub const ep_end: C2RustUnnamed_2 = 4;
pub const main_end: C2RustUnnamed_1 = 6;
pub const quitdoom: C2RustUnnamed_1 = 5;
pub const readthis: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_1 = u32;
pub const savegame: C2RustUnnamed_1 = 3;
pub const loadgame: C2RustUnnamed_1 = 2;
pub const options: C2RustUnnamed_1 = 1;
pub const newgame: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = u32;
pub const ep4: C2RustUnnamed_2 = 3;
pub const ep3: C2RustUnnamed_2 = 2;
pub const ep2: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = u32;
pub const violence: C2RustUnnamed_3 = 3;
pub const toorough: C2RustUnnamed_3 = 1;
pub const killthings: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = u32;
pub const soundvol: C2RustUnnamed_4 = 7;
pub const option_empty2: C2RustUnnamed_4 = 6;
pub const option_empty1: C2RustUnnamed_4 = 4;
pub const endgame: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = u32;
pub const rdthsempty1: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = u32;
pub const rdthsempty2: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = u32;
pub const sfx_empty2: C2RustUnnamed_7 = 3;
pub const sfx_empty1: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_8 = u32;
pub const load6: C2RustUnnamed_8 = 5;
pub const load5: C2RustUnnamed_8 = 4;
pub const load4: C2RustUnnamed_8 = 3;
pub const load3: C2RustUnnamed_8 = 2;
pub const load2: C2RustUnnamed_8 = 1;
pub const load1: C2RustUnnamed_8 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const KEY_ESCAPE: i32 = 27 as i32;
pub const KEY_ENTER: i32 = 13;
pub const KEY_BACKSPACE: i32 = 127;
pub const KEY_PAUSE: i32 = 0xff as i32;
pub const KEY_CAPSLOCK: i32 = 0x80 as i32
    + 0x3a as i32;
pub const KEY_NUMLOCK: i32 = 0x80 as i32
    + 0x45 as i32;
pub const KEY_SCRLCK: i32 = 0x80 as i32
    + 0x46 as i32;
pub const GAMMALVL0: &str = "Gamma correction OFF\0";
pub const GAMMALVL1: &str = "Gamma correction level 1\0";
pub const GAMMALVL2: &str = "Gamma correction level 2\0";
pub const GAMMALVL3: &str = "Gamma correction level 3\0";
pub const GAMMALVL4: &str = "Gamma correction level 4\0";
pub const EMPTYSTRING: &str = "empty slot\0";
pub const NUM_QUITMESSAGES: i32 = 8 as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const HU_FONTSTART: i32 = '!' as i32;
pub const HU_FONTEND: i32 = '_' as i32;
pub const HU_FONTSIZE: i32 = HU_FONTEND - HU_FONTSTART
    + 1 as i32;
pub const SAVESTRINGSIZE: i32 = 24 as i32;
pub static mut mouseSensitivity: i32 = 5 as i32;
pub static mut showMessages: i32 = 1 as i32;
pub static mut detailLevel: i32 = 0 as i32;
pub static mut screenblocks: i32 = 10 as i32;
#[no_mangle]
pub static mut screenSize: i32 = 0;
#[no_mangle]
pub static mut quickSaveSlot: i32 = 0;
#[no_mangle]
pub static mut messageToPrint: i32 = 0;
pub static mut messageString: String = String::new();
#[no_mangle]
pub static mut messx: i32 = 0;
#[no_mangle]
pub static mut messy: i32 = 0;
#[no_mangle]
pub static mut messageLastMenuActive: i32 = 0;
#[no_mangle]
pub static mut messageNeedsInput: bool = false;
#[no_mangle]
pub static mut messageRoutine: Option<unsafe extern "C" fn(i32) -> ()> = None;
pub static gammamsg: [&str; 5] = [GAMMALVL0, GAMMALVL1, GAMMALVL2, GAMMALVL3, GAMMALVL4];
#[no_mangle]
pub static mut saveStringEnter: i32 = 0;
#[no_mangle]
pub static mut saveSlot: i32 = 0;
#[no_mangle]
pub static mut saveCharIndex: i32 = 0;
#[no_mangle]
pub static mut saveOldString: String = String::new();
pub static mut inhelpscreens: bool = false;
pub static mut menuactive: bool = false;
pub const SKULLXOFF: i32 = -(32 as i32);
pub const LINEHEIGHT: i32 = 16 as i32;
pub static mut savegamestrings: [String; 10] = [
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
];
#[no_mangle]
pub static mut endstring: [::core::ffi::c_char; 160] = [0; 160];
#[no_mangle]
pub static mut itemOn: i16 = 0;
#[no_mangle]
pub static mut skullAnimCounter: i16 = 0;
#[no_mangle]
pub static mut whichSkull: i16 = 0;
#[no_mangle]
pub static skullName: [&str; 2] = ["M_SKULL1", "M_SKULL2"];
#[no_mangle]
pub static mut currentMenu: *mut menu_t = ::core::ptr::null::<menu_t>() as *mut menu_t;
#[no_mangle]
pub static mut main_e: C2RustUnnamed_1 = newgame;
#[no_mangle]
pub static mut MainMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_NGAME\0\0\0"),
            routine: Some(M_NewGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'n' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_OPTION\0\0"),
            routine: Some(M_Options as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'o' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_LOADG\0\0\0"),
            routine: Some(M_LoadGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'l' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SAVEG\0\0\0"),
            routine: Some(M_SaveGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_RDTHIS\0\0"),
            routine: Some(M_ReadThis as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'r' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_QUITG\0\0\0"),
            routine: Some(M_QuitDOOM as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'q' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut MainDef: menu_t = unsafe {
    menu_s {
        numitems: main_end as i32 as i16,
        prevMenu: ::core::ptr::null::<menu_s>() as *mut menu_s,
        menuitems: &raw const MainMenu as *mut menuitem_t,
        routine: Some(M_DrawMainMenu as unsafe extern "C" fn() -> ()),
        x: 97 as i16,
        y: 64 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut episodes_e: C2RustUnnamed_2 = ep1;
#[no_mangle]
pub static mut EpisodeMenu: [menuitem_t; 4] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI1\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'k' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI2\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 't' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI3\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'i' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI4\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 't' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut EpiDef: menu_t = unsafe {
    menu_s {
        numitems: ep_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const EpisodeMenu as *mut menuitem_t,
        routine: Some(M_DrawEpisode as unsafe extern "C" fn() -> ()),
        x: 48 as i16,
        y: 63 as i16,
        lastOn: ep1 as i32 as i16,
    }
};
#[no_mangle]
pub static mut newgame_e: C2RustUnnamed_3 = killthings;
#[no_mangle]
pub static mut NewGameMenu: [menuitem_t; 5] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_JKILL\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'i' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_ROUGH\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'h' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_HURT\0\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'h' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_ULTRA\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'u' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_NMARE\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'n' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut NewDef: menu_t = unsafe {
    menu_s {
        numitems: newg_end as i32 as i16,
        prevMenu: &raw const EpiDef as *mut menu_s,
        menuitems: &raw const NewGameMenu as *mut menuitem_t,
        routine: Some(M_DrawNewGame as unsafe extern "C" fn() -> ()),
        x: 48 as i16,
        y: 63 as i16,
        lastOn: hurtme as i32 as i16,
    }
};
#[no_mangle]
pub static mut options_e: C2RustUnnamed_4 = endgame;
#[no_mangle]
pub static mut OptionsMenu: [menuitem_t; 8] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_ENDGAM\0\0"),
            routine: Some(M_EndGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'e' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_MESSG\0\0\0"),
            routine: Some(
                M_ChangeMessages as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'm' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_DETAIL\0\0"),
            routine: Some(
                M_ChangeDetail as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'g' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SCRNSZ\0\0"),
            routine: Some(
                M_SizeDisplay as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_MSENS\0\0\0"),
            routine: Some(
                M_ChangeSensitivity as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'm' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SVOL\0\0\0\0"),
            routine: Some(M_Sound as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut OptionsDef: menu_t = unsafe {
    menu_s {
        numitems: opt_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const OptionsMenu as *mut menuitem_t,
        routine: Some(M_DrawOptions as unsafe extern "C" fn() -> ()),
        x: 60 as i16,
        y: 37 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut read_e: C2RustUnnamed_5 = rdthsempty1;
#[no_mangle]
pub static mut ReadMenu1: [menuitem_t; 1] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(M_ReadThis2 as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 0 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut ReadDef1: menu_t = unsafe {
    menu_s {
        numitems: read1_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const ReadMenu1 as *mut menuitem_t,
        routine: Some(M_DrawReadThis1 as unsafe extern "C" fn() -> ()),
        x: 280 as i16,
        y: 185 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut read_e2: C2RustUnnamed_6 = rdthsempty2;
#[no_mangle]
pub static mut ReadMenu2: [menuitem_t; 1] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_FinishReadThis as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 0 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut ReadDef2: menu_t = unsafe {
    menu_s {
        numitems: read2_end as i32 as i16,
        prevMenu: &raw const ReadDef1 as *mut menu_s,
        menuitems: &raw const ReadMenu2 as *mut menuitem_t,
        routine: Some(M_DrawReadThis2 as unsafe extern "C" fn() -> ()),
        x: 330 as i16,
        y: 175 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut sound_e: C2RustUnnamed_7 = sfx_vol;
#[no_mangle]
pub static mut SoundMenu: [menuitem_t; 4] = unsafe {
    [
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SFXVOL\0\0"),
            routine: Some(M_SfxVol as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_MUSVOL\0\0"),
            routine: Some(M_MusicVol as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'm' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut SoundDef: menu_t = unsafe {
    menu_s {
        numitems: sound_end as i32 as i16,
        prevMenu: &raw const OptionsDef as *mut menu_s,
        menuitems: &raw const SoundMenu as *mut menuitem_t,
        routine: Some(M_DrawSound as unsafe extern "C" fn() -> ()),
        x: 80 as i16,
        y: 64 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut load_e: C2RustUnnamed_8 = load1;
#[no_mangle]
pub static mut LoadMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '1' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '2' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '3' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '4' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '5' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '6' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut LoadDef: menu_t = unsafe {
    menu_s {
        numitems: load_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const LoadMenu as *mut menuitem_t,
        routine: Some(M_DrawLoad as unsafe extern "C" fn() -> ()),
        x: 80 as i16,
        y: 54 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut SaveMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '1' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '2' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '3' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '4' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '5' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '6' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut SaveDef: menu_t = unsafe {
    menu_s {
        numitems: load_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const SaveMenu as *mut menuitem_t,
        routine: Some(M_DrawSave as unsafe extern "C" fn() -> ()),
        x: 80 as i16,
        y: 54 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub unsafe extern "C" fn M_ReadSaveStrings() {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut i: i32 = 0;
    let mut name: [::core::ffi::c_char; 256] = [0; 256];
    i = 0 as i32;
    while i < load_end as i32 {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            P_SaveGameFile(i),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        );
        handle = fopen(
            &raw mut name as *mut ::core::ffi::c_char,
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if handle.is_null() {
            savegamestrings[i as usize] = EMPTYSTRING.trim_end_matches('\0').to_string();
            LoadMenu[i as usize].status = 0 as i16;
        } else {
            let mut buf: [u8; 24] = [0; 24];
            fread(
                buf.as_mut_ptr() as *mut ::core::ffi::c_void,
                1 as size_t,
                SAVESTRINGSIZE as size_t,
                handle,
            );
            let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            savegamestrings[i as usize] = String::from_utf8_lossy(&buf[..len]).into_owned();
            fclose(handle);
            LoadMenu[i as usize].status = 1 as i16;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawLoad() {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        72 as i32,
        28 as i32,
        W_CacheLumpName("M_LOADG",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    i = 0 as i32;
    while i < load_end as i32 {
        M_DrawSaveLoadBorder(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
        );
        M_WriteText(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
            &savegamestrings[i as usize],
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSaveLoadBorder(
    mut x: i32,
    mut y: i32,
) {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        x - 8 as i32,
        y + 7 as i32,
        W_CacheLumpName("M_LSLEFT",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    i = 0 as i32;
    while i < 24 as i32 {
        V_DrawPatchDirect(
            x,
            y + 7 as i32,
            W_CacheLumpName("M_LSCNTR",
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
        x += 8 as i32;
        i += 1;
    }
    V_DrawPatchDirect(
        x,
        y + 7 as i32,
        W_CacheLumpName("M_LSRGHT",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_LoadSelect(mut choice: i32) {
    let mut name: [::core::ffi::c_char; 256] = [0; 256];
    M_StringCopy(
        &raw mut name as *mut ::core::ffi::c_char,
        P_SaveGameFile(choice),
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
    );
    G_LoadGame(&raw mut name as *mut ::core::ffi::c_char);
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_LoadGame(mut choice: i32) {
    if netgame {
        M_StartMessage(
            "you can't do load while in a net game!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    M_SetupNextMenu(&raw mut LoadDef);
    M_ReadSaveStrings();
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSave() {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        72 as i32,
        28 as i32,
        W_CacheLumpName("M_SAVEG",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    i = 0 as i32;
    while i < load_end as i32 {
        M_DrawSaveLoadBorder(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
        );
        M_WriteText(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
            &savegamestrings[i as usize],
        );
        i += 1;
    }
    if saveStringEnter != 0 {
        i = M_StringWidth(&savegamestrings[saveSlot as usize]);
        M_WriteText(
            LoadDef.x as i32 + i,
            LoadDef.y as i32 + LINEHEIGHT * saveSlot,
            "_",
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DoSave(mut slot: i32) {
    let name_cstring = ::std::ffi::CString::new(savegamestrings[slot as usize].as_str())
        .unwrap();
    G_SaveGame(slot, name_cstring.as_ptr() as *mut ::core::ffi::c_char);
    M_ClearMenus();
    if quickSaveSlot == -(2 as i32) {
        quickSaveSlot = slot;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveSelect(mut choice: i32) {
    saveStringEnter = 1 as i32;
    saveSlot = choice;
    saveOldString = savegamestrings[choice as usize].clone();
    if savegamestrings[choice as usize] == EMPTYSTRING.trim_end_matches('\0') {
        savegamestrings[choice as usize].clear();
    }
    saveCharIndex = savegamestrings[choice as usize].len() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveGame(mut choice: i32) {
    if !usergame {
        M_StartMessage(
            "you can't save if you aren't playing!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if gamestate as u32
        != GS_LEVEL as i32 as u32
    {
        return;
    }
    M_SetupNextMenu(&raw mut SaveDef);
    M_ReadSaveStrings();
}
#[no_mangle]
pub static mut tempstring: [::core::ffi::c_char; 80] = [0; 80];
#[no_mangle]
pub unsafe extern "C" fn M_QuickSaveResponse(mut key: i32) {
    if key == key_menu_confirm {
        M_DoSave(quickSaveSlot);
        S_StartSound(NULL, sfx_swtchx as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickSave() {
    if !usergame {
        S_StartSound(NULL, sfx_oof as i32);
        return;
    }
    if gamestate as u32
        != GS_LEVEL as i32 as u32
    {
        return;
    }
    if quickSaveSlot < 0 as i32 {
        M_StartControlPanel();
        M_ReadSaveStrings();
        M_SetupNextMenu(&raw mut SaveDef);
        quickSaveSlot = -(2 as i32);
        return;
    }
    let quicksave_name_cstring = ::std::ffi::CString::new(
        savegamestrings[quickSaveSlot as usize].as_str(),
    )
        .unwrap();
    snprintf(
        &raw mut tempstring as *mut ::core::ffi::c_char,
        80 as size_t,
        b"quicksave over your game named\n\n'%s'?\n\npress y or n.\0" as *const u8
            as *const ::core::ffi::c_char,
        quicksave_name_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(&raw mut tempstring as *mut ::core::ffi::c_char)
            .to_str()
            .unwrap(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_QuickSaveResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickLoadResponse(mut key: i32) {
    if key == key_menu_confirm {
        M_LoadSelect(quickSaveSlot);
        S_StartSound(NULL, sfx_swtchx as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickLoad() {
    if netgame {
        M_StartMessage(
            "you can't quickload during a netgame!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if quickSaveSlot < 0 as i32 {
        M_StartMessage(
            "you haven't picked a quicksave slot yet!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    let quickload_name_cstring = ::std::ffi::CString::new(
        savegamestrings[quickSaveSlot as usize].as_str(),
    )
        .unwrap();
    snprintf(
        &raw mut tempstring as *mut ::core::ffi::c_char,
        80 as size_t,
        b"do you want to quickload the game named\n\n'%s'?\n\npress y or n.\0"
            as *const u8 as *const ::core::ffi::c_char,
        quickload_name_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(&raw mut tempstring as *mut ::core::ffi::c_char)
            .to_str()
            .unwrap(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_QuickLoadResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawReadThis1() {
    let mut lumpname: *mut ::core::ffi::c_char = b"CREDIT\0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut skullx: i32 = 330 as i32;
    let mut skully: i32 = 175 as i32;
    inhelpscreens = true;
    match gameversion as u32 {
        1 | 2 | 3 | 4 | 5 => {
            if gamemode as u32
                == commercial as i32 as u32
            {
                lumpname = b"HELP\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                skullx = 330 as i32;
                skully = 165 as i32;
            } else {
                lumpname = b"HELP2\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                skullx = 280 as i32;
                skully = 185 as i32;
            }
        }
        6 | 9 => {
            lumpname = b"HELP1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 | 8 => {
            lumpname = b"HELP\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            I_Error("Unhandled game version");
        }
    }
    lumpname = lumpname;
    V_DrawPatchDirect(
        0 as i32,
        0 as i32,
        W_CacheLumpName(
            &wad_name8_to_string(lumpname),
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    ReadDef1.x = skullx as i16;
    ReadDef1.y = skully as i16;
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawReadThis2() {
    inhelpscreens = true;
    V_DrawPatchDirect(
        0 as i32,
        0 as i32,
        W_CacheLumpName("HELP1",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSound() {
    V_DrawPatchDirect(
        60 as i32,
        38 as i32,
        W_CacheLumpName("M_SVOL",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    M_DrawThermo(
        SoundDef.x as i32,
        SoundDef.y as i32
            + LINEHEIGHT * (sfx_vol as i32 + 1 as i32),
        16 as i32,
        sfxVolume,
    );
    M_DrawThermo(
        SoundDef.x as i32,
        SoundDef.y as i32
            + LINEHEIGHT * (music_vol as i32 + 1 as i32),
        16 as i32,
        musicVolume,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_Sound(mut choice: i32) {
    M_SetupNextMenu(&raw mut SoundDef);
}
#[no_mangle]
pub unsafe extern "C" fn M_SfxVol(mut choice: i32) {
    match choice {
        0 => {
            if sfxVolume != 0 {
                sfxVolume -= 1;
            }
        }
        1 => {
            if sfxVolume < 15 as i32 {
                sfxVolume += 1;
            }
        }
        _ => {}
    }
    S_SetSfxVolume(sfxVolume * 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn M_MusicVol(mut choice: i32) {
    match choice {
        0 => {
            if musicVolume != 0 {
                musicVolume -= 1;
            }
        }
        1 => {
            if musicVolume < 15 as i32 {
                musicVolume += 1;
            }
        }
        _ => {}
    }
    S_SetMusicVolume(musicVolume * 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawMainMenu() {
    V_DrawPatchDirect(
        94 as i32,
        2 as i32,
        W_CacheLumpName("M_DOOM",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawNewGame() {
    V_DrawPatchDirect(
        96 as i32,
        14 as i32,
        W_CacheLumpName("M_NEWG",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        54 as i32,
        38 as i32,
        W_CacheLumpName("M_SKILL",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_NewGame(mut choice: i32) {
    if netgame && !demoplayback {
        M_StartMessage(
            "you can't start a new game\nwhile in a network game.\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if gamemode as u32
        == commercial as i32 as u32
        || gameversion as u32
            == exe_chex as i32 as u32
    {
        M_SetupNextMenu(&raw mut NewDef);
    } else {
        M_SetupNextMenu(&raw mut EpiDef);
    };
}
#[no_mangle]
pub static mut epi: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn M_DrawEpisode() {
    V_DrawPatchDirect(
        54 as i32,
        38 as i32,
        W_CacheLumpName("M_EPISOD",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_VerifyNightmare(mut key: i32) {
    if key != key_menu_confirm {
        return;
    }
    G_DeferedInitNew(
        nightmare as i32 as skill_t,
        epi + 1 as i32,
        1 as i32,
    );
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_ChooseSkill(mut choice: i32) {
    if choice == nightmare as i32 {
        M_StartMessage(
            "are you sure? this skill level\nisn't even remotely fair.\n\npress y or n.",
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(i32) -> ()>,
                *mut ::core::ffi::c_void,
            >(Some(M_VerifyNightmare as unsafe extern "C" fn(i32) -> ())),
            true,
        );
        return;
    }
    G_DeferedInitNew(
        choice as skill_t,
        epi + 1 as i32,
        1 as i32,
    );
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_Episode(mut choice: i32) {
    if gamemode as u32
        == shareware as i32 as u32 && choice != 0
    {
        M_StartMessage(
            "this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\npress a key.",
            NULL,
            false,
        );
        M_SetupNextMenu(&raw mut ReadDef1);
        return;
    }
    if gamemode as u32
        == registered as i32 as u32
        && choice > 2 as i32
    {
        fprintf(
            stderr,
            b"M_Episode: 4th episode requires UltimateDOOM\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        choice = 0 as i32;
    }
    epi = choice;
    M_SetupNextMenu(&raw mut NewDef);
}
static detailNames: [&str; 2] = ["M_GDHIGH", "M_GDLOW"];
static msgNames: [&str; 2] = ["M_MSGOFF", "M_MSGON"];
#[no_mangle]
pub unsafe extern "C" fn M_DrawOptions() {
    V_DrawPatchDirect(
        108 as i32,
        15 as i32,
        W_CacheLumpName("M_OPTTTL",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        OptionsDef.x as i32 + 175 as i32,
        OptionsDef.y as i32 + LINEHEIGHT * detail as i32,
        W_CacheLumpName(
            detailNames[detailLevel as usize],
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        OptionsDef.x as i32 + 120 as i32,
        OptionsDef.y as i32 + LINEHEIGHT * messages as i32,
        W_CacheLumpName(msgNames[showMessages as usize], PU_CACHE as i32)
            as *mut patch_t,
    );
    M_DrawThermo(
        OptionsDef.x as i32,
        OptionsDef.y as i32
            + LINEHEIGHT * (mousesens as i32 + 1 as i32),
        10 as i32,
        mouseSensitivity,
    );
    M_DrawThermo(
        OptionsDef.x as i32,
        OptionsDef.y as i32
            + LINEHEIGHT * (scrnsize as i32 + 1 as i32),
        9 as i32,
        screenSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_Options(mut choice: i32) {
    M_SetupNextMenu(&raw mut OptionsDef);
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeMessages(mut choice: i32) {
    choice = 0 as i32;
    showMessages = 1 as i32 - showMessages;
    if showMessages == 0 {
        players[consoleplayer as usize].message = b"Messages OFF\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        players[consoleplayer as usize].message = b"Messages ON\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    message_dontfuckwithme = true;
}
#[no_mangle]
pub unsafe extern "C" fn M_EndGameResponse(mut key: i32) {
    if key != key_menu_confirm {
        return;
    }
    (*currentMenu).lastOn = itemOn;
    M_ClearMenus();
    D_StartTitle();
}
#[no_mangle]
pub unsafe extern "C" fn M_EndGame(mut choice: i32) {
    choice = 0 as i32;
    if !usergame {
        S_StartSound(NULL, sfx_oof as i32);
        return;
    }
    if netgame {
        M_StartMessage(
            "you can't end a netgame!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    M_StartMessage(
            "are you sure you want to end the game?\n\npress y or n.",
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_EndGameResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadThis(mut choice: i32) {
    choice = 0 as i32;
    M_SetupNextMenu(&raw mut ReadDef1);
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadThis2(mut choice: i32) {
    if gameversion as u32
        <= exe_doom_1_9 as i32 as u32
        && gamemode as u32
            != commercial as i32 as u32
    {
        choice = 0 as i32;
        M_SetupNextMenu(&raw mut ReadDef2);
    } else {
        M_FinishReadThis(0 as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_FinishReadThis(mut choice: i32) {
    choice = 0 as i32;
    M_SetupNextMenu(&raw mut MainDef);
}
#[no_mangle]
pub static mut quitsounds: [i32; 8] = [
    sfx_pldeth as i32,
    sfx_dmpain as i32,
    sfx_popain as i32,
    sfx_slop as i32,
    sfx_telept as i32,
    sfx_posit1 as i32,
    sfx_posit3 as i32,
    sfx_sgtatk as i32,
];
#[no_mangle]
pub static mut quitsounds2: [i32; 8] = [
    sfx_vilact as i32,
    sfx_getpow as i32,
    sfx_boscub as i32,
    sfx_slop as i32,
    sfx_skeswg as i32,
    sfx_kntdth as i32,
    sfx_bspact as i32,
    sfx_sgtatk as i32,
];
#[no_mangle]
pub unsafe extern "C" fn M_QuitResponse(mut key: i32) {
    if key != key_menu_confirm {
        return;
    }
    if !netgame {
        if gamemode as u32
            == commercial as i32 as u32
        {
            S_StartSound(
                NULL,
                quitsounds2[(gametic >> 2 as i32
                    & 7 as i32) as usize],
            );
        } else {
            S_StartSound(
                NULL,
                quitsounds[(gametic >> 2 as i32 & 7 as i32)
                    as usize],
            );
        }
        I_WaitVBL(105 as i32);
    }
    I_Quit();
}
unsafe fn M_SelectEndMessage() -> &'static str {
    let endmsg: &'static [&'static str; 8] = if (if gamemission as u32
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
        &doom1_endmsg
    } else {
        &doom2_endmsg
    };
    endmsg[(gametic % NUM_QUITMESSAGES) as usize]
}
#[no_mangle]
pub unsafe extern "C" fn M_QuitDOOM(mut choice: i32) {
    let endmsg_cstring = ::std::ffi::CString::new(M_SelectEndMessage()).unwrap();
    snprintf(
        &raw mut endstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>() as size_t,
        b"%s\n\n(press y to quit to dos.)\0" as *const u8 as *const ::core::ffi::c_char,
        endmsg_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(&raw mut endstring as *mut ::core::ffi::c_char)
            .to_str()
            .unwrap(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_QuitResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeSensitivity(mut choice: i32) {
    match choice {
        0 => {
            if mouseSensitivity != 0 {
                mouseSensitivity -= 1;
            }
        }
        1 => {
            if mouseSensitivity < 9 as i32 {
                mouseSensitivity += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeDetail(mut choice: i32) {
    choice = 0 as i32;
    detailLevel = 1 as i32 - detailLevel;
    R_SetViewSize(screenblocks, detailLevel);
    if detailLevel == 0 {
        players[consoleplayer as usize].message = b"High detail\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        players[consoleplayer as usize].message = b"Low detail\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_SizeDisplay(mut choice: i32) {
    match choice {
        0 => {
            if screenSize > 0 as i32 {
                screenblocks -= 1;
                screenSize -= 1;
            }
        }
        1 => {
            if screenSize < 8 as i32 {
                screenblocks += 1;
                screenSize += 1;
            }
        }
        _ => {}
    }
    R_SetViewSize(screenblocks, detailLevel);
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawThermo(
    mut x: i32,
    mut y: i32,
    mut thermWidth: i32,
    mut thermDot: i32,
) {
    let mut xx: i32 = 0;
    let mut i: i32 = 0;
    xx = x;
    V_DrawPatchDirect(
        xx,
        y,
        W_CacheLumpName("M_THERML",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    xx += 8 as i32;
    i = 0 as i32;
    while i < thermWidth {
        V_DrawPatchDirect(
            xx,
            y,
            W_CacheLumpName("M_THERMM",
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
        xx += 8 as i32;
        i += 1;
    }
    V_DrawPatchDirect(
        xx,
        y,
        W_CacheLumpName("M_THERMR",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        x + 8 as i32 + thermDot * 8 as i32,
        y,
        W_CacheLumpName("M_THERMO",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawEmptyCell(
    mut menu: *mut menu_t,
    mut item: i32,
) {
    V_DrawPatchDirect(
        (*menu).x as i32 - 10 as i32,
        (*menu).y as i32 + item * LINEHEIGHT - 1 as i32,
        W_CacheLumpName("M_CELL1",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSelCell(
    mut menu: *mut menu_t,
    mut item: i32,
) {
    V_DrawPatchDirect(
        (*menu).x as i32 - 10 as i32,
        (*menu).y as i32 + item * LINEHEIGHT - 1 as i32,
        W_CacheLumpName("M_CELL2",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
pub unsafe fn M_StartMessage(
    string: &str,
    mut routine: *mut ::core::ffi::c_void,
    mut input: bool,
) {
    messageLastMenuActive = menuactive as i32;
    messageToPrint = 1 as i32;
    messageString = string.to_string();
    messageRoutine = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(i32) -> ()>,
    >(routine);
    messageNeedsInput = input;
    menuactive = true;
}
#[no_mangle]
pub unsafe extern "C" fn M_StopMessage() {
    menuactive = messageLastMenuActive != 0;
    messageToPrint = 0 as i32;
}
pub unsafe fn M_StringWidth(string: &str) -> i32 {
    let mut w: i32 = 0 as i32;
    let mut c: i32 = 0;
    for b in string.bytes() {
        c = toupper(b as i32) - HU_FONTSTART;
        if c < 0 as i32 || c >= HU_FONTSIZE {
            w += 4 as i32;
        } else {
            w += (*hu_font[c as usize]).width as i32;
        }
    }
    return w;
}
pub unsafe fn M_StringHeight(string: &str) -> i32 {
    let mut h: i32 = 0;
    let height: i32 = (*hu_font[0 as i32 as usize])
        .height as i32;
    h = height;
    for b in string.bytes() {
        if b == b'\n' {
            h += height;
        }
    }
    return h;
}
pub unsafe fn M_WriteText(x: i32, y: i32, string: &str) {
    let mut w: i32 = 0;
    let mut c: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    cx = x;
    cy = y;
    'outer: for b in string.bytes() {
        c = b as i32;
        if c == '\n' as i32 {
            cx = x;
            cy += 12 as i32;
        } else {
            c = toupper(c) - HU_FONTSTART;
            if c < 0 as i32 || c >= HU_FONTSIZE {
                cx += 4 as i32;
            } else {
                w = (*hu_font[c as usize]).width as i32;
                if cx + w > SCREENWIDTH {
                    break 'outer;
                }
                V_DrawPatchDirect(cx, cy, hu_font[c as usize]);
                cx += w;
            }
        }
    };
}
unsafe extern "C" fn IsNullKey(mut key: i32) -> bool {
    return key == KEY_PAUSE || key == KEY_CAPSLOCK || key == KEY_SCRLCK
        || key == KEY_NUMLOCK;
}
pub unsafe fn M_Responder(mut ev: *mut event_t) -> bool {
    let mut ch: i32 = 0;
    let mut key: i32 = 0;
    let mut i: i32 = 0;
    static mut joywait: i32 = 0 as i32;
    static mut mousewait: i32 = 0 as i32;
    static mut mousey: i32 = 0 as i32;
    static mut lasty: i32 = 0 as i32;
    static mut mousex: i32 = 0 as i32;
    static mut lastx: i32 = 0 as i32;
    if testcontrols {
        if (*ev).type_0 as u32
            == ev_quit as i32 as u32
            || (*ev).type_0 as u32
                == ev_keydown as i32 as u32
                && ((*ev).data1 == key_menu_activate || (*ev).data1 == key_menu_quit)
        {
            I_Quit();
            return true;
        }
        return false;
    }
    if (*ev).type_0 as u32
        == ev_quit as i32 as u32
    {
        if menuactive && messageToPrint != 0
            && messageRoutine
                == Some(M_QuitResponse as unsafe extern "C" fn(i32) -> ())
        {
            M_QuitResponse(key_menu_confirm);
        } else {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuitDOOM(0 as i32);
        }
        return true;
    }
    ch = 0 as i32;
    key = -(1 as i32);
    if (*ev).type_0 as u32
        == ev_joystick as i32 as u32
        && joywait < I_GetTime()
    {
        if (*ev).data3 < 0 as i32 {
            key = key_menu_up;
            joywait = I_GetTime() + 5 as i32;
        } else if (*ev).data3 > 0 as i32 {
            key = key_menu_down;
            joywait = I_GetTime() + 5 as i32;
        }
        if (*ev).data2 < 0 as i32 {
            key = key_menu_left;
            joywait = I_GetTime() + 2 as i32;
        } else if (*ev).data2 > 0 as i32 {
            key = key_menu_right;
            joywait = I_GetTime() + 2 as i32;
        }
        if (*ev).data1 & 1 as i32 != 0 {
            key = key_menu_forward;
            joywait = I_GetTime() + 5 as i32;
        }
        if (*ev).data1 & 2 as i32 != 0 {
            key = key_menu_back;
            joywait = I_GetTime() + 5 as i32;
        }
        if joybmenu >= 0 as i32
            && (*ev).data1 & (1 as i32) << joybmenu
                != 0 as i32
        {
            key = key_menu_activate;
            joywait = I_GetTime() + 5 as i32;
        }
    } else if (*ev).type_0 as u32
        == ev_mouse as i32 as u32
        && mousewait < I_GetTime()
    {
        mousey += (*ev).data3;
        if mousey < lasty - 30 as i32 {
            key = key_menu_down;
            mousewait = I_GetTime() + 5 as i32;
            lasty -= 30 as i32;
            mousey = lasty;
        } else if mousey > lasty + 30 as i32 {
            key = key_menu_up;
            mousewait = I_GetTime() + 5 as i32;
            lasty += 30 as i32;
            mousey = lasty;
        }
        mousex += (*ev).data2;
        if mousex < lastx - 30 as i32 {
            key = key_menu_left;
            mousewait = I_GetTime() + 5 as i32;
            lastx -= 30 as i32;
            mousex = lastx;
        } else if mousex > lastx + 30 as i32 {
            key = key_menu_right;
            mousewait = I_GetTime() + 5 as i32;
            lastx += 30 as i32;
            mousex = lastx;
        }
        if (*ev).data1 & 1 as i32 != 0 {
            key = key_menu_forward;
            mousewait = I_GetTime() + 15 as i32;
        }
        if (*ev).data1 & 2 as i32 != 0 {
            key = key_menu_back;
            mousewait = I_GetTime() + 15 as i32;
        }
    } else if (*ev).type_0 as u32
        == ev_keydown as i32 as u32
    {
        key = (*ev).data1;
        ch = (*ev).data2;
    }
    if key == -(1 as i32) {
        return false;
    }
    if saveStringEnter != 0 {
        match key {
            KEY_BACKSPACE => {
                if saveCharIndex > 0 as i32 {
                    saveCharIndex -= 1;
                    savegamestrings[saveSlot as usize].truncate(saveCharIndex as usize);
                }
            }
            KEY_ESCAPE => {
                saveStringEnter = 0 as i32;
                savegamestrings[saveSlot as usize] = saveOldString.clone();
            }
            KEY_ENTER => {
                saveStringEnter = 0 as i32;
                if !savegamestrings[saveSlot as usize].is_empty() {
                    M_DoSave(saveSlot);
                }
            }
            _ => {
                if vanilla_keyboard_mapping != 0 {
                    ch = key;
                }
                ch = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<i32>() as usize > 1 as usize
                    {
                        if 0 != 0 {
                            let mut __c: i32 = ch;
                            __res = (if __c < -(128 as i32)
                                || __c > 255 as i32
                            {
                                __c as __int32_t
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            }) as i32;
                        } else {
                            __res = toupper(ch);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(ch as isize)
                            as i32;
                    }
                    __res
                });
                if !(ch != ' ' as i32
                    && (ch - HU_FONTSTART < 0 as i32
                        || ch - HU_FONTSTART >= HU_FONTSIZE))
                {
                    if ch >= 32 as i32 && ch <= 127 as i32
                        && saveCharIndex < SAVESTRINGSIZE - 1 as i32
                        && M_StringWidth(&savegamestrings[saveSlot as usize])
                            < (SAVESTRINGSIZE - 2 as i32)
                                * 8 as i32
                    {
                        saveCharIndex += 1;
                        savegamestrings[saveSlot as usize].push(ch as u8 as char);
                    }
                }
            }
        }
        return true;
    }
    if messageToPrint != 0 {
        if messageNeedsInput {
            if key != ' ' as i32 && key != KEY_ESCAPE && key != key_menu_confirm
                && key != key_menu_abort
            {
                return false;
            }
        }
        menuactive = messageLastMenuActive != 0;
        messageToPrint = 0 as i32;
        if messageRoutine.is_some() {
            messageRoutine.expect("non-null function pointer")(key);
        }
        menuactive = false;
        S_StartSound(NULL, sfx_swtchx as i32);
        return true;
    }
    if devparm && key == key_menu_help
        || key != 0 as i32 && key == key_menu_screenshot
    {
        G_ScreenShot();
        return true;
    }
    if !menuactive {
        if key == key_menu_decscreen {
            if automapactive || chat_on {
                return false;
            }
            M_SizeDisplay(0 as i32);
            S_StartSound(NULL, sfx_stnmov as i32);
            return true;
        } else if key == key_menu_incscreen {
            if automapactive || chat_on {
                return false;
            }
            M_SizeDisplay(1 as i32);
            S_StartSound(NULL, sfx_stnmov as i32);
            return true;
        } else if key == key_menu_help {
            M_StartControlPanel();
            if gamemode as u32
                == retail as i32 as u32
            {
                currentMenu = &raw mut ReadDef2;
            } else {
                currentMenu = &raw mut ReadDef1;
            }
            itemOn = 0 as i16;
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_save {
            M_StartControlPanel();
            S_StartSound(NULL, sfx_swtchn as i32);
            M_SaveGame(0 as i32);
            return true;
        } else if key == key_menu_load {
            M_StartControlPanel();
            S_StartSound(NULL, sfx_swtchn as i32);
            M_LoadGame(0 as i32);
            return true;
        } else if key == key_menu_volume {
            M_StartControlPanel();
            currentMenu = &raw mut SoundDef;
            itemOn = sfx_vol as i32 as i16;
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_detail {
            M_ChangeDetail(0 as i32);
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_qsave {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuickSave();
            return true;
        } else if key == key_menu_endgame {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_EndGame(0 as i32);
            return true;
        } else if key == key_menu_messages {
            M_ChangeMessages(0 as i32);
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_qload {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuickLoad();
            return true;
        } else if key == key_menu_quit {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuitDOOM(0 as i32);
            return true;
        } else if key == key_menu_gamma {
            usegamma += 1;
            if usegamma > 4 as i32 {
                usegamma = 0 as i32;
            }
            players[consoleplayer as usize].message = gammamsg[usegamma as usize]
                .as_ptr() as *mut ::core::ffi::c_char;
            I_SetPalette(
                W_CacheLumpName("PLAYPAL",
                    PU_CACHE as i32,
                ) as *mut byte,
            );
            return true;
        }
    }
    if !menuactive {
        if key == key_menu_activate {
            M_StartControlPanel();
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        }
        return false;
    }
    if key == key_menu_down {
        loop {
            if itemOn as i32 + 1 as i32
                > (*currentMenu).numitems as i32 - 1 as i32
            {
                itemOn = 0 as i16;
            } else {
                itemOn += 1;
            }
            S_StartSound(NULL, sfx_pstop as i32);
            if !((*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == -(1 as i32))
            {
                break;
            }
        }
        return true;
    } else if key == key_menu_up {
        loop {
            if itemOn == 0 {
                itemOn = ((*currentMenu).numitems as i32
                    - 1 as i32) as i16;
            } else {
                itemOn -= 1;
            }
            S_StartSound(NULL, sfx_pstop as i32);
            if !((*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == -(1 as i32))
            {
                break;
            }
        }
        return true;
    } else if key == key_menu_left {
        if (*(*currentMenu).menuitems.offset(itemOn as isize)).routine.is_some()
            && (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == 2 as i32
        {
            S_StartSound(NULL, sfx_stnmov as i32);
            (*(*currentMenu).menuitems.offset(itemOn as isize))
                .routine
                .expect("non-null function pointer")(0 as i32);
        }
        return true;
    } else if key == key_menu_right {
        if (*(*currentMenu).menuitems.offset(itemOn as isize)).routine.is_some()
            && (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == 2 as i32
        {
            S_StartSound(NULL, sfx_stnmov as i32);
            (*(*currentMenu).menuitems.offset(itemOn as isize))
                .routine
                .expect("non-null function pointer")(1 as i32);
        }
        return true;
    } else if key == key_menu_forward {
        if (*(*currentMenu).menuitems.offset(itemOn as isize)).routine.is_some()
            && (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 != 0
        {
            (*currentMenu).lastOn = itemOn;
            if (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == 2 as i32
            {
                (*(*currentMenu).menuitems.offset(itemOn as isize))
                    .routine
                    .expect("non-null function pointer")(1 as i32);
                S_StartSound(NULL, sfx_stnmov as i32);
            } else {
                (*(*currentMenu).menuitems.offset(itemOn as isize))
                    .routine
                    .expect("non-null function pointer")(itemOn as i32);
                S_StartSound(NULL, sfx_pistol as i32);
            }
        }
        return true;
    } else if key == key_menu_activate {
        (*currentMenu).lastOn = itemOn;
        M_ClearMenus();
        S_StartSound(NULL, sfx_swtchx as i32);
        return true;
    } else if key == key_menu_back {
        (*currentMenu).lastOn = itemOn;
        if !(*currentMenu).prevMenu.is_null() {
            currentMenu = (*currentMenu).prevMenu as *mut menu_t;
            itemOn = (*currentMenu).lastOn;
            S_StartSound(NULL, sfx_swtchn as i32);
        }
        return true;
    } else if ch != 0 as i32 || IsNullKey(key) {
        i = itemOn as i32 + 1 as i32;
        while i < (*currentMenu).numitems as i32 {
            if (*(*currentMenu).menuitems.offset(i as isize)).alphaKey
                as i32 == ch
            {
                itemOn = i as i16;
                S_StartSound(NULL, sfx_pstop as i32);
                return true;
            }
            i += 1;
        }
        i = 0 as i32;
        while i <= itemOn as i32 {
            if (*(*currentMenu).menuitems.offset(i as isize)).alphaKey
                as i32 == ch
            {
                itemOn = i as i16;
                S_StartSound(NULL, sfx_pstop as i32);
                return true;
            }
            i += 1;
        }
    }
    return false;
}
pub unsafe fn M_StartControlPanel() {
    if menuactive {
        return;
    }
    menuactive = true;
    currentMenu = &raw mut MainDef;
    itemOn = (*currentMenu).lastOn;
}
pub unsafe fn M_Drawer() {
    static mut x: i16 = 0;
    static mut y: i16 = 0;
    let mut i: u32 = 0;
    let mut max: u32 = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    inhelpscreens = false;
    if messageToPrint != 0 {
        y = (SCREENHEIGHT / 2 as i32
            - M_StringHeight(&messageString) / 2 as i32) as i16;
        for line in messageString.split('\n') {
            let line = if line.len() > 79 { &line[..79] } else { line };
            x = (SCREENWIDTH / 2 as i32
                - M_StringWidth(line) / 2 as i32) as i16;
            M_WriteText(x as i32, y as i32, line);
            y = (y as i32
                + (*hu_font[0 as i32 as usize]).height
                    as i32) as i16;
        }
        return;
    }
    if !menuactive {
        return;
    }
    if (*currentMenu).routine.is_some() {
        ::core::mem::transmute::<
            _,
            fn(),
        >((*currentMenu).routine.expect("non-null function pointer"))();
    }
    x = (*currentMenu).x;
    y = (*currentMenu).y;
    max = (*currentMenu).numitems as u32;
    i = 0 as u32;
    while i < max {
        name = &raw mut (*(*currentMenu).menuitems.offset(i as isize)).name
            as *mut ::core::ffi::c_char;
        if *name.offset(0 as i32 as isize) != 0 {
            V_DrawPatchDirect(
                x as i32,
                y as i32,
                W_CacheLumpName(
                    &wad_name8_to_string(name),
                    PU_CACHE as i32,
                ) as *mut patch_t,
            );
        }
        y = (y as i32 + LINEHEIGHT) as i16;
        i = i.wrapping_add(1);
    }
    V_DrawPatchDirect(
        x as i32 + SKULLXOFF,
        (*currentMenu).y as i32 - 5 as i32
            + itemOn as i32 * LINEHEIGHT,
        W_CacheLumpName(skullName[whichSkull as usize], PU_CACHE as i32)
            as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ClearMenus() {
    menuactive = false;
}
#[no_mangle]
pub unsafe extern "C" fn M_SetupNextMenu(mut menudef: *mut menu_t) {
    currentMenu = menudef;
    itemOn = (*currentMenu).lastOn;
}
pub unsafe fn M_Ticker() {
    skullAnimCounter -= 1;
    if skullAnimCounter as i32 <= 0 as i32 {
        whichSkull = (whichSkull as i32 ^ 1 as i32)
            as i16;
        skullAnimCounter = 8 as i16;
    }
}
pub unsafe fn M_Init() {
    currentMenu = &raw mut MainDef;
    menuactive = false;
    itemOn = (*currentMenu).lastOn;
    whichSkull = 0 as i16;
    skullAnimCounter = 10 as i16;
    screenSize = screenblocks - 3 as i32;
    messageToPrint = 0 as i32;
    messageString = String::new();
    messageLastMenuActive = menuactive as i32;
    quickSaveSlot = -(1 as i32);
    match gamemode as u32 {
        2 => {
            MainMenu[readthis as i32 as usize] = MainMenu[quitdoom
                as i32 as usize];
            MainDef.numitems -= 1;
            MainDef.y = (MainDef.y as i32 + 8 as i32)
                as i16;
            NewDef.prevMenu = &raw mut MainDef as *mut menu_s;
        }
        0 => {}
        1 | 3 | _ => {}
    }
    if (gameversion as u32)
        < exe_ultimate as i32 as u32
    {
        EpiDef.numitems -= 1;
    }
}
