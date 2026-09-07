use crate::src::d_event::event_t;
use crate::src::d_main::D_StartTitle;
use crate::src::dstrings::{doom1_endmsg, doom2_endmsg};
use crate::src::hu_lib::patch_t;
use crate::src::i_system::I_Error;
use crate::src::i_system::FILE;
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName};

use crate::src::d_event::GS_LEVEL;
use crate::src::d_event::{ev_joystick, ev_keydown, ev_mouse, ev_quit};
use crate::src::d_mode::skill_t;
use crate::src::d_mode::{commercial, registered, retail, shareware};
use crate::src::d_mode::{doom, doom2, pack_chex, pack_hacx};
use crate::src::d_mode::{exe_chex, exe_doom_1_9, exe_ultimate};
use crate::src::doomdef::NULL;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::g_game::G_DeferedInitNew;
use crate::src::g_game::G_LoadGame;
use crate::src::g_game::G_SaveGame;
use crate::src::g_game::G_ScreenShot;
use crate::src::game_state::game_state;
use crate::src::hu_stuff::HU_FONTSIZE;
use crate::src::hu_stuff::HU_FONTSTART;
use crate::src::i_system::I_Quit;
use crate::src::i_system::{fclose, fopen, fprintf, fread, stderr};
use crate::src::i_timer::I_GetTime;
use crate::src::i_video::I_SetPalette;
use crate::src::m_controls::KEY_BACKSPACE;
use crate::src::m_controls::KEY_CAPSLOCK;
use crate::src::m_controls::KEY_ENTER;
use crate::src::m_controls::KEY_ESCAPE;
use crate::src::m_controls::KEY_PAUSE;
use crate::src::m_controls::KEY_SCRLCK;
use crate::src::m_misc::M_StringCopy;
use crate::src::m_misc::__ctype_toupper_loc;
use crate::src::p_saveg::P_SaveGameFile;
use crate::src::r_main::R_SetViewSize;
use crate::src::s_sound::S_SetMusicVolume;
use crate::src::s_sound::S_SetSfxVolume;
use crate::src::s_sound::S_StartSound;
use crate::src::sounds::{
    sfx_boscub, sfx_bspact, sfx_dmpain, sfx_getpow, sfx_kntdth, sfx_oof, sfx_pistol, sfx_pldeth,
    sfx_popain, sfx_posit1, sfx_posit3, sfx_pstop, sfx_sgtatk, sfx_skeswg, sfx_slop, sfx_stnmov,
    sfx_swtchn, sfx_swtchx, sfx_telept, sfx_vilact,
};
use crate::src::stdint_types::__int32_t;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::v_video::V_DrawPatchDirect;
use crate::src::z_zone::PU_CACHE;
use libc::snprintf;
use libc::toupper;

pub struct MMenuDefsHolder {
    pub MainDef: menu_t,
    pub EpiDef: menu_t,
    pub NewDef: menu_t,
    pub OptionsDef: menu_t,
    pub ReadDef1: menu_t,
    pub ReadDef2: menu_t,
    pub SoundDef: menu_t,
    pub LoadDef: menu_t,
    pub SaveDef: menu_t,
}

impl MMenuDefsHolder {
    pub const fn new() -> Self {
        MMenuDefsHolder {
            MainDef: menu_s {
                numitems: main_end as i32 as i16,
                prevMenu: ::core::ptr::null::<menu_s>() as *mut menu_s,
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawMainMenu as unsafe extern "C" fn() -> ()),
                x: 97 as i16,
                y: 64 as i16,
                lastOn: 0 as i16,
            },
            EpiDef: menu_s {
                numitems: ep_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawEpisode as unsafe extern "C" fn() -> ()),
                x: 48 as i16,
                y: 63 as i16,
                lastOn: ep1 as i32 as i16,
            },
            NewDef: menu_s {
                numitems: newg_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawNewGame as unsafe extern "C" fn() -> ()),
                x: 48 as i16,
                y: 63 as i16,
                lastOn: hurtme as i32 as i16,
            },
            OptionsDef: menu_s {
                numitems: opt_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawOptions as unsafe extern "C" fn() -> ()),
                x: 60 as i16,
                y: 37 as i16,
                lastOn: 0 as i16,
            },
            ReadDef1: menu_s {
                numitems: read1_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawReadThis1 as unsafe extern "C" fn() -> ()),
                x: 280 as i16,
                y: 185 as i16,
                lastOn: 0 as i16,
            },
            ReadDef2: menu_s {
                numitems: read2_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawReadThis2 as unsafe extern "C" fn() -> ()),
                x: 330 as i16,
                y: 175 as i16,
                lastOn: 0 as i16,
            },
            SoundDef: menu_s {
                numitems: sound_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawSound as unsafe extern "C" fn() -> ()),
                x: 80 as i16,
                y: 64 as i16,
                lastOn: 0 as i16,
            },
            LoadDef: menu_s {
                numitems: load_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawLoad as unsafe extern "C" fn() -> ()),
                x: 80 as i16,
                y: 54 as i16,
                lastOn: 0 as i16,
            },
            SaveDef: menu_s {
                numitems: load_end as i32 as i16,
                prevMenu: ::core::ptr::null_mut::<menu_s>(),
                menuitems: ::core::ptr::null_mut::<menuitem_t>(),
                routine: Some(M_DrawSave as unsafe extern "C" fn() -> ()),
                x: 80 as i16,
                y: 54 as i16,
                lastOn: 0 as i16,
            },
        }
    }
}

pub struct MMenuMenusHolder {
    pub MainMenu: [menuitem_t; 6],
    pub EpisodeMenu: [menuitem_t; 4],
    pub NewGameMenu: [menuitem_t; 5],
    pub OptionsMenu: [menuitem_t; 8],
    pub ReadMenu1: [menuitem_t; 1],
    pub ReadMenu2: [menuitem_t; 1],
    pub SoundMenu: [menuitem_t; 4],
    pub LoadMenu: [menuitem_t; 6],
    pub SaveMenu: [menuitem_t; 6],
}

impl MMenuMenusHolder {
    pub const fn new() -> Self {
        MMenuMenusHolder {
            MainMenu: unsafe {
                [
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_NGAME\0\0\0",
                        ),
                        routine: Some(M_NewGame as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'n' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_OPTION\0\0",
                        ),
                        routine: Some(M_Options as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'o' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_LOADG\0\0\0",
                        ),
                        routine: Some(M_LoadGame as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'l' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_SAVEG\0\0\0",
                        ),
                        routine: Some(M_SaveGame as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 's' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_RDTHIS\0\0",
                        ),
                        routine: Some(M_ReadThis as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'r' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_QUITG\0\0\0",
                        ),
                        routine: Some(M_QuitDOOM as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'q' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
            EpisodeMenu: unsafe {
                [
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_EPI1\0\0\0\0",
                        ),
                        routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'k' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_EPI2\0\0\0\0",
                        ),
                        routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 't' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_EPI3\0\0\0\0",
                        ),
                        routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'i' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_EPI4\0\0\0\0",
                        ),
                        routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 't' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
            NewGameMenu: unsafe {
                [
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_JKILL\0\0\0",
                        ),
                        routine: Some(M_ChooseSkill as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'i' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_ROUGH\0\0\0",
                        ),
                        routine: Some(M_ChooseSkill as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'h' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_HURT\0\0\0\0",
                        ),
                        routine: Some(M_ChooseSkill as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'h' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_ULTRA\0\0\0",
                        ),
                        routine: Some(M_ChooseSkill as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'u' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_NMARE\0\0\0",
                        ),
                        routine: Some(M_ChooseSkill as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'n' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
            OptionsMenu: unsafe {
                [
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_ENDGAM\0\0",
                        ),
                        routine: Some(M_EndGame as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'e' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_MESSG\0\0\0",
                        ),
                        routine: Some(M_ChangeMessages as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'm' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_DETAIL\0\0",
                        ),
                        routine: Some(M_ChangeDetail as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'g' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 2 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_SCRNSZ\0\0",
                        ),
                        routine: Some(M_SizeDisplay as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 's' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: -(1 as i32) as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: None,
                        alphaKey: '\0' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 2 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_MSENS\0\0\0",
                        ),
                        routine: Some(M_ChangeSensitivity as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'm' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: -(1 as i32) as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: None,
                        alphaKey: '\0' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_SVOL\0\0\0\0",
                        ),
                        routine: Some(M_Sound as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 's' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
            ReadMenu1: unsafe {
                [menuitem_t {
                    status: 1 as i16,
                    name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                        *b"\0\0\0\0\0\0\0\0\0\0",
                    ),
                    routine: Some(M_ReadThis2 as unsafe extern "C" fn(i32) -> ()),
                    alphaKey: 0 as ::core::ffi::c_char,
                }]
            },
            ReadMenu2: unsafe {
                [menuitem_t {
                    status: 1 as i16,
                    name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                        *b"\0\0\0\0\0\0\0\0\0\0",
                    ),
                    routine: Some(M_FinishReadThis as unsafe extern "C" fn(i32) -> ()),
                    alphaKey: 0 as ::core::ffi::c_char,
                }]
            },
            SoundMenu: unsafe {
                [
                    menuitem_t {
                        status: 2 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_SFXVOL\0\0",
                        ),
                        routine: Some(M_SfxVol as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 's' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: -(1 as i32) as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: None,
                        alphaKey: '\0' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 2 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"M_MUSVOL\0\0",
                        ),
                        routine: Some(M_MusicVol as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: 'm' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: -(1 as i32) as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: None,
                        alphaKey: '\0' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
            LoadMenu: unsafe {
                [
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_LoadSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '1' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_LoadSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '2' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_LoadSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '3' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_LoadSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '4' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_LoadSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '5' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_LoadSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '6' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
            SaveMenu: unsafe {
                [
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_SaveSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '1' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_SaveSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '2' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_SaveSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '3' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_SaveSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '4' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_SaveSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '5' as i32 as ::core::ffi::c_char,
                    },
                    menuitem_t {
                        status: 1 as i16,
                        name: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"\0\0\0\0\0\0\0\0\0\0",
                        ),
                        routine: Some(M_SaveSelect as unsafe extern "C" fn(i32) -> ()),
                        alphaKey: '6' as i32 as ::core::ffi::c_char,
                    },
                ]
            },
        }
    }
}

pub struct MMenuState {
    pub menus: MMenuMenusHolder,
    pub defs: MMenuDefsHolder,
    pub mouseSensitivity: i32,
    pub showMessages: i32,
    pub detailLevel: i32,
    pub screenblocks: i32,
    pub screenSize: i32,
    pub quickSaveSlot: i32,
    pub messageToPrint: i32,
    pub messageString: String,
    pub messx: i32,
    pub messy: i32,
    pub messageLastMenuActive: i32,
    pub messageNeedsInput: bool,
    pub messageRoutine: Option<unsafe extern "C" fn(i32) -> ()>,
    pub saveStringEnter: i32,
    pub saveSlot: i32,
    pub saveCharIndex: i32,
    pub saveOldString: String,
    pub inhelpscreens: bool,
    pub menuactive: bool,
    pub savegamestrings: [String; 10],
    pub endstring: [::core::ffi::c_char; 160],
    pub itemOn: i16,
    pub skullAnimCounter: i16,
    pub whichSkull: i16,
    pub currentMenu: *mut menu_t,
    pub tempstring: [::core::ffi::c_char; 80],
    pub epi: i32,
    pub responder_joywait: i32,
    pub responder_mousewait: i32,
    pub responder_mousey: i32,
    pub responder_lasty: i32,
    pub responder_mousex: i32,
    pub responder_lastx: i32,
    pub drawer_x: i16,
    pub drawer_y: i16,
}

impl MMenuState {
    pub const fn new() -> Self {
        MMenuState {
            menus: MMenuMenusHolder::new(),
            defs: MMenuDefsHolder::new(),
            mouseSensitivity: 5,
            showMessages: 1,
            detailLevel: 0,
            screenblocks: 10,
            screenSize: 0,
            quickSaveSlot: 0,
            messageToPrint: 0,
            messageString: String::new(),
            messx: 0,
            messy: 0,
            messageLastMenuActive: 0,
            messageNeedsInput: false,
            messageRoutine: None,
            saveStringEnter: 0,
            saveSlot: 0,
            saveCharIndex: 0,
            saveOldString: String::new(),
            inhelpscreens: false,
            menuactive: false,
            savegamestrings: [
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
            ],
            endstring: [0; 160],
            itemOn: 0,
            skullAnimCounter: 0,
            whichSkull: 0,
            currentMenu: ::core::ptr::null::<menu_t>() as *mut menu_t,
            tempstring: [0; 80],
            epi: 0,
            responder_joywait: 0,
            responder_mousewait: 0,
            responder_mousey: 0,
            responder_lasty: 0,
            responder_mousex: 0,
            responder_lastx: 0,
            drawer_x: 0,
            drawer_y: 0,
        }
    }

    // Each menu_t's `menuitems` points at this same struct's own XxxMenu
    // array, and most also `prevMenu`-link to a sibling menu_t -- both are
    // only known once this value is at its final, permanently-stable
    // 'static address (inside GameState, behind OnceLock). Called once
    // from `game_state()` itself, strictly after `OnceLock::get_or_init`
    // returns, same pattern as `sounds::fixup_self_links`/
    // `p_maputl::fixup_intercepts_overrun`/`m_controls::fixup_weapon_keys`.
    pub fn fixup_menu_links(&mut self) {
        self.defs.MainDef.menuitems = &raw mut self.menus.MainMenu as *mut menuitem_t;
        self.defs.EpiDef.menuitems = &raw mut self.menus.EpisodeMenu as *mut menuitem_t;
        self.defs.EpiDef.prevMenu = &raw mut self.defs.MainDef as *mut menu_s;
        self.defs.NewDef.menuitems = &raw mut self.menus.NewGameMenu as *mut menuitem_t;
        self.defs.NewDef.prevMenu = &raw mut self.defs.EpiDef as *mut menu_s;
        self.defs.OptionsDef.menuitems = &raw mut self.menus.OptionsMenu as *mut menuitem_t;
        self.defs.OptionsDef.prevMenu = &raw mut self.defs.MainDef as *mut menu_s;
        self.defs.ReadDef1.menuitems = &raw mut self.menus.ReadMenu1 as *mut menuitem_t;
        self.defs.ReadDef1.prevMenu = &raw mut self.defs.MainDef as *mut menu_s;
        self.defs.ReadDef2.menuitems = &raw mut self.menus.ReadMenu2 as *mut menuitem_t;
        self.defs.ReadDef2.prevMenu = &raw mut self.defs.ReadDef1 as *mut menu_s;
        self.defs.SoundDef.menuitems = &raw mut self.menus.SoundMenu as *mut menuitem_t;
        self.defs.SoundDef.prevMenu = &raw mut self.defs.OptionsDef as *mut menu_s;
        self.defs.LoadDef.menuitems = &raw mut self.menus.LoadMenu as *mut menuitem_t;
        self.defs.LoadDef.prevMenu = &raw mut self.defs.MainDef as *mut menu_s;
        self.defs.SaveDef.menuitems = &raw mut self.menus.SaveMenu as *mut menuitem_t;
        self.defs.SaveDef.prevMenu = &raw mut self.defs.MainDef as *mut menu_s;
    }
}

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
pub const KEY_NUMLOCK: i32 = 0x80 + 0x45 as i32;
pub const GAMMALVL0: &str = "Gamma correction OFF\0";
pub const GAMMALVL1: &str = "Gamma correction level 1\0";
pub const GAMMALVL2: &str = "Gamma correction level 2\0";
pub const GAMMALVL3: &str = "Gamma correction level 3\0";
pub const GAMMALVL4: &str = "Gamma correction level 4\0";
pub const EMPTYSTRING: &str = "empty slot\0";
pub const NUM_QUITMESSAGES: i32 = 8;
pub const SAVESTRINGSIZE: i32 = 24;
pub static gammamsg: [&str; 5] = [GAMMALVL0, GAMMALVL1, GAMMALVL2, GAMMALVL3, GAMMALVL4];
pub const SKULLXOFF: i32 = -(32 as i32);
pub const LINEHEIGHT: i32 = 16;
#[no_mangle]
pub static skullName: [&str; 2] = ["M_SKULL1", "M_SKULL2"];
#[no_mangle]
pub static main_e: C2RustUnnamed_1 = newgame;
#[no_mangle]
pub static episodes_e: C2RustUnnamed_2 = ep1;
#[no_mangle]
pub static newgame_e: C2RustUnnamed_3 = killthings;
#[no_mangle]
pub static options_e: C2RustUnnamed_4 = endgame;
#[no_mangle]
pub static read_e: C2RustUnnamed_5 = rdthsempty1;
#[no_mangle]
pub static read_e2: C2RustUnnamed_6 = rdthsempty2;
#[no_mangle]
pub static sound_e: C2RustUnnamed_7 = sfx_vol;
#[no_mangle]
pub static load_e: C2RustUnnamed_8 = load1;
pub unsafe fn M_ReadSaveStrings() {
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
            unsafe { game_state() }.m_menu.savegamestrings[i as usize] =
                EMPTYSTRING.trim_end_matches('\0').to_string();
            unsafe { game_state() }.m_menu.menus.LoadMenu[i as usize].status = 0 as i16;
        } else {
            let mut buf: [u8; 24] = [0; 24];
            fread(
                buf.as_mut_ptr() as *mut ::core::ffi::c_void,
                1 as size_t,
                SAVESTRINGSIZE as size_t,
                handle,
            );
            let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            unsafe { game_state() }.m_menu.savegamestrings[i as usize] =
                String::from_utf8_lossy(&buf[..len]).into_owned();
            fclose(handle);
            unsafe { game_state() }.m_menu.menus.LoadMenu[i as usize].status = 1 as i16;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawLoad() {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        72 as i32,
        28 as i32,
        W_CacheLumpName("M_LOADG", PU_CACHE as i32) as *mut patch_t,
    );
    i = 0 as i32;
    while i < load_end as i32 {
        M_DrawSaveLoadBorder(
            unsafe { game_state() }.m_menu.defs.LoadDef.x as i32,
            unsafe { game_state() }.m_menu.defs.LoadDef.y as i32 + LINEHEIGHT * i,
        );
        M_WriteText(
            unsafe { game_state() }.m_menu.defs.LoadDef.x as i32,
            unsafe { game_state() }.m_menu.defs.LoadDef.y as i32 + LINEHEIGHT * i,
            &unsafe { game_state() }.m_menu.savegamestrings[i as usize],
        );
        i += 1;
    }
}
pub unsafe fn M_DrawSaveLoadBorder(mut x: i32, mut y: i32) {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        x - 8 as i32,
        y + 7 as i32,
        W_CacheLumpName("M_LSLEFT", PU_CACHE as i32) as *mut patch_t,
    );
    i = 0 as i32;
    while i < 24 as i32 {
        V_DrawPatchDirect(
            unsafe { &mut game_state().v_video },
            x,
            y + 7 as i32,
            W_CacheLumpName("M_LSCNTR", PU_CACHE as i32) as *mut patch_t,
        );
        x += 8 as i32;
        i += 1;
    }
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        x,
        y + 7 as i32,
        W_CacheLumpName("M_LSRGHT", PU_CACHE as i32) as *mut patch_t,
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
    if unsafe { game_state() }.g_game.netgame {
        M_StartMessage(
            "you can't do load while in a net game!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.LoadDef);
    M_ReadSaveStrings();
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSave() {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        72 as i32,
        28 as i32,
        W_CacheLumpName("M_SAVEG", PU_CACHE as i32) as *mut patch_t,
    );
    i = 0 as i32;
    while i < load_end as i32 {
        M_DrawSaveLoadBorder(
            unsafe { game_state() }.m_menu.defs.LoadDef.x as i32,
            unsafe { game_state() }.m_menu.defs.LoadDef.y as i32 + LINEHEIGHT * i,
        );
        M_WriteText(
            unsafe { game_state() }.m_menu.defs.LoadDef.x as i32,
            unsafe { game_state() }.m_menu.defs.LoadDef.y as i32 + LINEHEIGHT * i,
            &unsafe { game_state() }.m_menu.savegamestrings[i as usize],
        );
        i += 1;
    }
    if unsafe { game_state() }.m_menu.saveStringEnter != 0 {
        i = M_StringWidth(
            &unsafe { game_state() }.m_menu.savegamestrings
                [unsafe { game_state() }.m_menu.saveSlot as usize],
        );
        M_WriteText(
            unsafe { game_state() }.m_menu.defs.LoadDef.x as i32 + i,
            unsafe { game_state() }.m_menu.defs.LoadDef.y as i32
                + LINEHEIGHT * unsafe { game_state() }.m_menu.saveSlot,
            "_",
        );
    }
}
pub unsafe fn M_DoSave(mut slot: i32) {
    let name_cstring = ::std::ffi::CString::new(
        unsafe { game_state() }.m_menu.savegamestrings[slot as usize].as_str(),
    )
    .unwrap();
    G_SaveGame(slot, name_cstring.as_ptr() as *mut ::core::ffi::c_char);
    M_ClearMenus();
    if unsafe { game_state() }.m_menu.quickSaveSlot == -(2 as i32) {
        unsafe { game_state() }.m_menu.quickSaveSlot = slot;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveSelect(mut choice: i32) {
    unsafe { game_state() }.m_menu.saveStringEnter = 1 as i32;
    unsafe { game_state() }.m_menu.saveSlot = choice;
    unsafe { game_state() }.m_menu.saveOldString =
        unsafe { game_state() }.m_menu.savegamestrings[choice as usize].clone();
    if unsafe { game_state() }.m_menu.savegamestrings[choice as usize]
        == EMPTYSTRING.trim_end_matches('\0')
    {
        unsafe { game_state() }.m_menu.savegamestrings[choice as usize].clear();
    }
    unsafe { game_state() }.m_menu.saveCharIndex =
        unsafe { game_state() }.m_menu.savegamestrings[choice as usize].len() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveGame(mut choice: i32) {
    if !unsafe { game_state() }.g_game.usergame {
        M_StartMessage(
            "you can't save if you aren't playing!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if unsafe { game_state() }.g_game.gamestate as u32 != GS_LEVEL as i32 as u32 {
        return;
    }
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.SaveDef);
    M_ReadSaveStrings();
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickSaveResponse(mut key: i32) {
    if key == unsafe { game_state() }.m_controls.key_menu_confirm {
        M_DoSave(unsafe { game_state() }.m_menu.quickSaveSlot);
        S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchx as i32);
    }
}
pub unsafe fn M_QuickSave() {
    if !unsafe { game_state() }.g_game.usergame {
        S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_oof as i32);
        return;
    }
    if unsafe { game_state() }.g_game.gamestate as u32 != GS_LEVEL as i32 as u32 {
        return;
    }
    if unsafe { game_state() }.m_menu.quickSaveSlot < 0 as i32 {
        M_StartControlPanel();
        M_ReadSaveStrings();
        M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.SaveDef);
        unsafe { game_state() }.m_menu.quickSaveSlot = -(2 as i32);
        return;
    }
    let quicksave_name_cstring = ::std::ffi::CString::new(
        unsafe { game_state() }.m_menu.savegamestrings
            [unsafe { game_state() }.m_menu.quickSaveSlot as usize]
            .as_str(),
    )
    .unwrap();
    snprintf(
        &raw mut unsafe { game_state() }.m_menu.tempstring as *mut ::core::ffi::c_char,
        80 as size_t,
        b"quicksave over your game named\n\n'%s'?\n\npress y or n.\0" as *const u8
            as *const ::core::ffi::c_char,
        quicksave_name_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(
            &raw mut unsafe { game_state() }.m_menu.tempstring as *mut ::core::ffi::c_char,
        )
        .to_str()
        .unwrap(),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(i32) -> ()>, *mut ::core::ffi::c_void>(
            Some(M_QuickSaveResponse as unsafe extern "C" fn(i32) -> ()),
        ),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickLoadResponse(mut key: i32) {
    if key == unsafe { game_state() }.m_controls.key_menu_confirm {
        M_LoadSelect(unsafe { game_state() }.m_menu.quickSaveSlot);
        S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchx as i32);
    }
}
pub unsafe fn M_QuickLoad() {
    if unsafe { game_state() }.g_game.netgame {
        M_StartMessage(
            "you can't quickload during a netgame!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if unsafe { game_state() }.m_menu.quickSaveSlot < 0 as i32 {
        M_StartMessage(
            "you haven't picked a quicksave slot yet!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    let quickload_name_cstring = ::std::ffi::CString::new(
        unsafe { game_state() }.m_menu.savegamestrings
            [unsafe { game_state() }.m_menu.quickSaveSlot as usize]
            .as_str(),
    )
    .unwrap();
    snprintf(
        &raw mut unsafe { game_state() }.m_menu.tempstring as *mut ::core::ffi::c_char,
        80 as size_t,
        b"do you want to quickload the game named\n\n'%s'?\n\npress y or n.\0" as *const u8
            as *const ::core::ffi::c_char,
        quickload_name_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(
            &raw mut unsafe { game_state() }.m_menu.tempstring as *mut ::core::ffi::c_char,
        )
        .to_str()
        .unwrap(),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(i32) -> ()>, *mut ::core::ffi::c_void>(
            Some(M_QuickLoadResponse as unsafe extern "C" fn(i32) -> ()),
        ),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawReadThis1() {
    let mut lumpname: *mut ::core::ffi::c_char =
        b"CREDIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut skullx: i32 = 330 as i32;
    let mut skully: i32 = 175 as i32;
    unsafe { game_state() }.m_menu.inhelpscreens = true;
    match unsafe { game_state() }.doomstat.gameversion as u32 {
        1 | 2 | 3 | 4 | 5 => {
            if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as i32 as u32 {
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
            lumpname =
                b"HELP1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        7 | 8 => {
            lumpname =
                b"HELP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        _ => {
            I_Error("Unhandled game version");
        }
    }
    lumpname = lumpname;
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        0 as i32,
        0 as i32,
        W_CacheLumpName(&wad_name8_to_string(lumpname), PU_CACHE as i32) as *mut patch_t,
    );
    unsafe { game_state() }.m_menu.defs.ReadDef1.x = skullx as i16;
    unsafe { game_state() }.m_menu.defs.ReadDef1.y = skully as i16;
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawReadThis2() {
    unsafe { game_state() }.m_menu.inhelpscreens = true;
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        0 as i32,
        0 as i32,
        W_CacheLumpName("HELP1", PU_CACHE as i32) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSound() {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        60 as i32,
        38 as i32,
        W_CacheLumpName("M_SVOL", PU_CACHE as i32) as *mut patch_t,
    );
    M_DrawThermo(
        unsafe { game_state() }.m_menu.defs.SoundDef.x as i32,
        unsafe { game_state() }.m_menu.defs.SoundDef.y as i32
            + LINEHEIGHT * (sfx_vol as i32 + 1 as i32),
        16 as i32,
        unsafe { game_state() }.s_sound.sfxVolume,
    );
    M_DrawThermo(
        unsafe { game_state() }.m_menu.defs.SoundDef.x as i32,
        unsafe { game_state() }.m_menu.defs.SoundDef.y as i32
            + LINEHEIGHT * (music_vol as i32 + 1 as i32),
        16 as i32,
        unsafe { game_state() }.s_sound.musicVolume,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_Sound(mut choice: i32) {
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.SoundDef);
}
#[no_mangle]
pub unsafe extern "C" fn M_SfxVol(mut choice: i32) {
    match choice {
        0 => {
            if unsafe { game_state() }.s_sound.sfxVolume != 0 {
                unsafe { game_state() }.s_sound.sfxVolume -= 1;
            }
        }
        1 => {
            if unsafe { game_state() }.s_sound.sfxVolume < 15 as i32 {
                unsafe { game_state() }.s_sound.sfxVolume += 1;
            }
        }
        _ => {}
    }
    S_SetSfxVolume(unsafe { game_state() }.s_sound.sfxVolume * 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn M_MusicVol(mut choice: i32) {
    match choice {
        0 => {
            if unsafe { game_state() }.s_sound.musicVolume != 0 {
                unsafe { game_state() }.s_sound.musicVolume -= 1;
            }
        }
        1 => {
            if unsafe { game_state() }.s_sound.musicVolume < 15 as i32 {
                unsafe { game_state() }.s_sound.musicVolume += 1;
            }
        }
        _ => {}
    }
    S_SetMusicVolume(unsafe { game_state() }.s_sound.musicVolume * 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawMainMenu() {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        94 as i32,
        2 as i32,
        W_CacheLumpName("M_DOOM", PU_CACHE as i32) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawNewGame() {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        96 as i32,
        14 as i32,
        W_CacheLumpName("M_NEWG", PU_CACHE as i32) as *mut patch_t,
    );
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        54 as i32,
        38 as i32,
        W_CacheLumpName("M_SKILL", PU_CACHE as i32) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_NewGame(mut choice: i32) {
    if unsafe { game_state() }.g_game.netgame && !unsafe { game_state() }.g_game.demoplayback {
        M_StartMessage(
            "you can't start a new game\nwhile in a network game.\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as i32 as u32
        || unsafe { game_state() }.doomstat.gameversion as u32 == exe_chex as i32 as u32
    {
        M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.NewDef);
    } else {
        M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.EpiDef);
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawEpisode() {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        54 as i32,
        38 as i32,
        W_CacheLumpName("M_EPISOD", PU_CACHE as i32) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_VerifyNightmare(mut key: i32) {
    if key != unsafe { game_state() }.m_controls.key_menu_confirm {
        return;
    }
    G_DeferedInitNew(
        nightmare as i32 as skill_t,
        unsafe { game_state() }.m_menu.epi + 1 as i32,
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
        unsafe { game_state() }.m_menu.epi + 1 as i32,
        1 as i32,
    );
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_Episode(mut choice: i32) {
    if unsafe { game_state() }.doomstat.gamemode as u32 == shareware as i32 as u32 && choice != 0 {
        M_StartMessage(
            "this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\npress a key.",
            NULL,
            false,
        );
        M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.ReadDef1);
        return;
    }
    if unsafe { game_state() }.doomstat.gamemode as u32 == registered as i32 as u32
        && choice > 2 as i32
    {
        fprintf(
            stderr,
            b"M_Episode: 4th episode requires UltimateDOOM\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        choice = 0 as i32;
    }
    unsafe { game_state() }.m_menu.epi = choice;
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.NewDef);
}
static detailNames: [&str; 2] = ["M_GDHIGH", "M_GDLOW"];
static msgNames: [&str; 2] = ["M_MSGOFF", "M_MSGON"];
#[no_mangle]
pub unsafe extern "C" fn M_DrawOptions() {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        108 as i32,
        15 as i32,
        W_CacheLumpName("M_OPTTTL", PU_CACHE as i32) as *mut patch_t,
    );
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.m_menu.defs.OptionsDef.x as i32 + 175 as i32,
        unsafe { game_state() }.m_menu.defs.OptionsDef.y as i32 + LINEHEIGHT * detail as i32,
        W_CacheLumpName(
            detailNames[unsafe { game_state() }.m_menu.detailLevel as usize],
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.m_menu.defs.OptionsDef.x as i32 + 120 as i32,
        unsafe { game_state() }.m_menu.defs.OptionsDef.y as i32 + LINEHEIGHT * messages as i32,
        W_CacheLumpName(
            msgNames[unsafe { game_state() }.m_menu.showMessages as usize],
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    M_DrawThermo(
        unsafe { game_state() }.m_menu.defs.OptionsDef.x as i32,
        unsafe { game_state() }.m_menu.defs.OptionsDef.y as i32
            + LINEHEIGHT * (mousesens as i32 + 1 as i32),
        10 as i32,
        unsafe { game_state() }.m_menu.mouseSensitivity,
    );
    M_DrawThermo(
        unsafe { game_state() }.m_menu.defs.OptionsDef.x as i32,
        unsafe { game_state() }.m_menu.defs.OptionsDef.y as i32
            + LINEHEIGHT * (scrnsize as i32 + 1 as i32),
        9 as i32,
        unsafe { game_state() }.m_menu.screenSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_Options(mut choice: i32) {
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.OptionsDef);
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeMessages(mut choice: i32) {
    choice = 0 as i32;
    unsafe { game_state() }.m_menu.showMessages =
        1 as i32 - unsafe { game_state() }.m_menu.showMessages;
    if unsafe { game_state() }.m_menu.showMessages == 0 {
        unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .message = b"Messages OFF\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else {
        unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .message =
            b"Messages ON\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    unsafe { game_state() }.hu_stuff.message_dontfuckwithme = true;
}
#[no_mangle]
pub unsafe extern "C" fn M_EndGameResponse(mut key: i32) {
    if key != unsafe { game_state() }.m_controls.key_menu_confirm {
        return;
    }
    (*unsafe { game_state() }.m_menu.currentMenu).lastOn = unsafe { game_state() }.m_menu.itemOn;
    M_ClearMenus();
    D_StartTitle();
}
#[no_mangle]
pub unsafe extern "C" fn M_EndGame(mut choice: i32) {
    choice = 0 as i32;
    if !unsafe { game_state() }.g_game.usergame {
        S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_oof as i32);
        return;
    }
    if unsafe { game_state() }.g_game.netgame {
        M_StartMessage("you can't end a netgame!\n\npress a key.", NULL, false);
        return;
    }
    M_StartMessage(
        "are you sure you want to end the game?\n\npress y or n.",
        ::core::mem::transmute::<Option<unsafe extern "C" fn(i32) -> ()>, *mut ::core::ffi::c_void>(
            Some(M_EndGameResponse as unsafe extern "C" fn(i32) -> ()),
        ),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadThis(mut choice: i32) {
    choice = 0 as i32;
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.ReadDef1);
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadThis2(mut choice: i32) {
    if unsafe { game_state() }.doomstat.gameversion as u32 <= exe_doom_1_9 as i32 as u32
        && unsafe { game_state() }.doomstat.gamemode as u32 != commercial as i32 as u32
    {
        choice = 0 as i32;
        M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.ReadDef2);
    } else {
        M_FinishReadThis(0 as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_FinishReadThis(mut choice: i32) {
    choice = 0 as i32;
    M_SetupNextMenu(&raw mut unsafe { game_state() }.m_menu.defs.MainDef);
}
#[no_mangle]
pub static quitsounds: [i32; 8] = [
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
pub static quitsounds2: [i32; 8] = [
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
    if key != unsafe { game_state() }.m_controls.key_menu_confirm {
        return;
    }
    if !unsafe { game_state() }.g_game.netgame {
        if unsafe { game_state() }.doomstat.gamemode as u32 == commercial as i32 as u32 {
            S_StartSound(
                unsafe { &mut game_state().sounds },
                NULL,
                quitsounds2
                    [(unsafe { game_state() }.d_loop.gametic >> 2 as i32 & 7 as i32) as usize],
            );
        } else {
            S_StartSound(
                unsafe { &mut game_state().sounds },
                NULL,
                quitsounds
                    [(unsafe { game_state() }.d_loop.gametic >> 2 as i32 & 7 as i32) as usize],
            );
        }
    }
    I_Quit();
}
unsafe fn M_SelectEndMessage() -> &'static str {
    let endmsg: &'static [&'static str; 8] =
        if (if unsafe { game_state() }.doomstat.gamemission as u32 == pack_chex as i32 as u32 {
            doom as i32 as u32
        } else {
            (if unsafe { game_state() }.doomstat.gamemission as u32 == pack_hacx as i32 as u32 {
                doom2 as i32 as u32
            } else {
                unsafe { game_state() }.doomstat.gamemission as u32
            })
        }) == doom as i32 as u32
        {
            &doom1_endmsg
        } else {
            &doom2_endmsg
        };
    endmsg[(unsafe { game_state() }.d_loop.gametic % NUM_QUITMESSAGES) as usize]
}
#[no_mangle]
pub unsafe extern "C" fn M_QuitDOOM(mut choice: i32) {
    let endmsg_cstring = ::std::ffi::CString::new(M_SelectEndMessage()).unwrap();
    snprintf(
        &raw mut unsafe { game_state() }.m_menu.endstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>() as size_t,
        b"%s\n\n(press y to quit to dos.)\0" as *const u8 as *const ::core::ffi::c_char,
        endmsg_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(
            &raw mut unsafe { game_state() }.m_menu.endstring as *mut ::core::ffi::c_char,
        )
        .to_str()
        .unwrap(),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(i32) -> ()>, *mut ::core::ffi::c_void>(
            Some(M_QuitResponse as unsafe extern "C" fn(i32) -> ()),
        ),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeSensitivity(mut choice: i32) {
    match choice {
        0 => {
            if unsafe { game_state() }.m_menu.mouseSensitivity != 0 {
                unsafe { game_state() }.m_menu.mouseSensitivity -= 1;
            }
        }
        1 => {
            if unsafe { game_state() }.m_menu.mouseSensitivity < 9 as i32 {
                unsafe { game_state() }.m_menu.mouseSensitivity += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeDetail(mut choice: i32) {
    choice = 0 as i32;
    unsafe { game_state() }.m_menu.detailLevel =
        1 as i32 - unsafe { game_state() }.m_menu.detailLevel;
    R_SetViewSize(
        unsafe { game_state() }.m_menu.screenblocks,
        unsafe { game_state() }.m_menu.detailLevel,
    );
    if unsafe { game_state() }.m_menu.detailLevel == 0 {
        unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .message =
            b"High detail\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        unsafe { game_state() }.g_game.players
            [unsafe { game_state() }.g_game.consoleplayer as usize]
            .message =
            b"Low detail\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_SizeDisplay(mut choice: i32) {
    match choice {
        0 => {
            if unsafe { game_state() }.m_menu.screenSize > 0 as i32 {
                unsafe { game_state() }.m_menu.screenblocks -= 1;
                unsafe { game_state() }.m_menu.screenSize -= 1;
            }
        }
        1 => {
            if unsafe { game_state() }.m_menu.screenSize < 8 as i32 {
                unsafe { game_state() }.m_menu.screenblocks += 1;
                unsafe { game_state() }.m_menu.screenSize += 1;
            }
        }
        _ => {}
    }
    R_SetViewSize(
        unsafe { game_state() }.m_menu.screenblocks,
        unsafe { game_state() }.m_menu.detailLevel,
    );
}
pub unsafe fn M_DrawThermo(mut x: i32, mut y: i32, mut thermWidth: i32, mut thermDot: i32) {
    let mut xx: i32 = 0;
    let mut i: i32 = 0;
    xx = x;
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        xx,
        y,
        W_CacheLumpName("M_THERML", PU_CACHE as i32) as *mut patch_t,
    );
    xx += 8 as i32;
    i = 0 as i32;
    while i < thermWidth {
        V_DrawPatchDirect(
            unsafe { &mut game_state().v_video },
            xx,
            y,
            W_CacheLumpName("M_THERMM", PU_CACHE as i32) as *mut patch_t,
        );
        xx += 8 as i32;
        i += 1;
    }
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        xx,
        y,
        W_CacheLumpName("M_THERMR", PU_CACHE as i32) as *mut patch_t,
    );
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        x + 8 as i32 + thermDot * 8 as i32,
        y,
        W_CacheLumpName("M_THERMO", PU_CACHE as i32) as *mut patch_t,
    );
}
pub unsafe fn M_DrawEmptyCell(mut menu: *mut menu_t, mut item: i32) {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        (*menu).x as i32 - 10 as i32,
        (*menu).y as i32 + item * LINEHEIGHT - 1 as i32,
        W_CacheLumpName("M_CELL1", PU_CACHE as i32) as *mut patch_t,
    );
}
pub unsafe fn M_DrawSelCell(mut menu: *mut menu_t, mut item: i32) {
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        (*menu).x as i32 - 10 as i32,
        (*menu).y as i32 + item * LINEHEIGHT - 1 as i32,
        W_CacheLumpName("M_CELL2", PU_CACHE as i32) as *mut patch_t,
    );
}
pub unsafe fn M_StartMessage(string: &str, mut routine: *mut ::core::ffi::c_void, mut input: bool) {
    unsafe { game_state() }.m_menu.messageLastMenuActive =
        unsafe { game_state() }.m_menu.menuactive as i32;
    unsafe { game_state() }.m_menu.messageToPrint = 1 as i32;
    unsafe { game_state() }.m_menu.messageString = string.to_string();
    unsafe { game_state() }.m_menu.messageRoutine = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(i32) -> ()>,
    >(routine);
    unsafe { game_state() }.m_menu.messageNeedsInput = input;
    unsafe { game_state() }.m_menu.menuactive = true;
}
pub unsafe fn M_StopMessage() {
    unsafe { game_state() }.m_menu.menuactive =
        unsafe { game_state() }.m_menu.messageLastMenuActive != 0;
    unsafe { game_state() }.m_menu.messageToPrint = 0 as i32;
}
pub unsafe fn M_StringWidth(string: &str) -> i32 {
    let mut w: i32 = 0 as i32;
    let mut c: i32 = 0;
    for b in string.bytes() {
        c = toupper(b as i32) - HU_FONTSTART;
        if c < 0 as i32 || c >= HU_FONTSIZE {
            w += 4 as i32;
        } else {
            w += (*unsafe { game_state() }.hu_stuff.hu_font[c as usize]).width as i32;
        }
    }
    return w;
}
pub unsafe fn M_StringHeight(string: &str) -> i32 {
    let mut h: i32 = 0;
    let height: i32 = (*unsafe { game_state() }.hu_stuff.hu_font[0 as i32 as usize]).height as i32;
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
                w = (*unsafe { game_state() }.hu_stuff.hu_font[c as usize]).width as i32;
                if cx + w > SCREENWIDTH {
                    break 'outer;
                }
                V_DrawPatchDirect(
                    unsafe { &mut game_state().v_video },
                    cx,
                    cy,
                    unsafe { game_state() }.hu_stuff.hu_font[c as usize],
                );
                cx += w;
            }
        }
    }
}
unsafe fn IsNullKey(mut key: i32) -> bool {
    return key == KEY_PAUSE || key == KEY_CAPSLOCK || key == KEY_SCRLCK || key == KEY_NUMLOCK;
}
pub unsafe fn M_Responder(ev: &mut event_t) -> bool {
    let mut ch: i32 = 0;
    let mut key: i32 = 0;
    let mut i: i32 = 0;
    if unsafe { game_state() }.g_game.testcontrols {
        if (*ev).type_0 as u32 == ev_quit as i32 as u32
            || (*ev).type_0 as u32 == ev_keydown as i32 as u32
                && ((*ev).data1 == unsafe { game_state() }.m_controls.key_menu_activate
                    || (*ev).data1 == unsafe { game_state() }.m_controls.key_menu_quit)
        {
            I_Quit();
            return true;
        }
        return false;
    }
    if (*ev).type_0 as u32 == ev_quit as i32 as u32 {
        if unsafe { game_state() }.m_menu.menuactive
            && unsafe { game_state() }.m_menu.messageToPrint != 0
            && unsafe { game_state() }.m_menu.messageRoutine
                == Some(M_QuitResponse as unsafe extern "C" fn(i32) -> ())
        {
            M_QuitResponse(unsafe { game_state() }.m_controls.key_menu_confirm);
        } else {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_QuitDOOM(0 as i32);
        }
        return true;
    }
    ch = 0 as i32;
    key = -(1 as i32);
    if (*ev).type_0 as u32 == ev_joystick as i32 as u32
        && unsafe { game_state() }.m_menu.responder_joywait
            < I_GetTime(unsafe { &mut game_state().i_timer })
    {
        if (*ev).data3 < 0 as i32 {
            key = unsafe { game_state() }.m_controls.key_menu_up;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
        } else if (*ev).data3 > 0 as i32 {
            key = unsafe { game_state() }.m_controls.key_menu_down;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
        }
        if (*ev).data2 < 0 as i32 {
            key = unsafe { game_state() }.m_controls.key_menu_left;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 2 as i32;
        } else if (*ev).data2 > 0 as i32 {
            key = unsafe { game_state() }.m_controls.key_menu_right;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 2 as i32;
        }
        if (*ev).data1 & 1 as i32 != 0 {
            key = unsafe { game_state() }.m_controls.key_menu_forward;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
        }
        if (*ev).data1 & 2 as i32 != 0 {
            key = unsafe { game_state() }.m_controls.key_menu_back;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
        }
        if unsafe { game_state() }.m_controls.joybmenu >= 0 as i32
            && (*ev).data1 & (1 as i32) << unsafe { game_state() }.m_controls.joybmenu != 0 as i32
        {
            key = unsafe { game_state() }.m_controls.key_menu_activate;
            unsafe { game_state() }.m_menu.responder_joywait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
        }
    } else if (*ev).type_0 as u32 == ev_mouse as i32 as u32
        && unsafe { game_state() }.m_menu.responder_mousewait
            < I_GetTime(unsafe { &mut game_state().i_timer })
    {
        unsafe { game_state() }.m_menu.responder_mousey += (*ev).data3;
        if unsafe { game_state() }.m_menu.responder_mousey
            < unsafe { game_state() }.m_menu.responder_lasty - 30 as i32
        {
            key = unsafe { game_state() }.m_controls.key_menu_down;
            unsafe { game_state() }.m_menu.responder_mousewait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
            unsafe { game_state() }.m_menu.responder_lasty -= 30 as i32;
            unsafe { game_state() }.m_menu.responder_mousey =
                unsafe { game_state() }.m_menu.responder_lasty;
        } else if unsafe { game_state() }.m_menu.responder_mousey
            > unsafe { game_state() }.m_menu.responder_lasty + 30 as i32
        {
            key = unsafe { game_state() }.m_controls.key_menu_up;
            unsafe { game_state() }.m_menu.responder_mousewait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
            unsafe { game_state() }.m_menu.responder_lasty += 30 as i32;
            unsafe { game_state() }.m_menu.responder_mousey =
                unsafe { game_state() }.m_menu.responder_lasty;
        }
        unsafe { game_state() }.m_menu.responder_mousex += (*ev).data2;
        if unsafe { game_state() }.m_menu.responder_mousex
            < unsafe { game_state() }.m_menu.responder_lastx - 30 as i32
        {
            key = unsafe { game_state() }.m_controls.key_menu_left;
            unsafe { game_state() }.m_menu.responder_mousewait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
            unsafe { game_state() }.m_menu.responder_lastx -= 30 as i32;
            unsafe { game_state() }.m_menu.responder_mousex =
                unsafe { game_state() }.m_menu.responder_lastx;
        } else if unsafe { game_state() }.m_menu.responder_mousex
            > unsafe { game_state() }.m_menu.responder_lastx + 30 as i32
        {
            key = unsafe { game_state() }.m_controls.key_menu_right;
            unsafe { game_state() }.m_menu.responder_mousewait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 5 as i32;
            unsafe { game_state() }.m_menu.responder_lastx += 30 as i32;
            unsafe { game_state() }.m_menu.responder_mousex =
                unsafe { game_state() }.m_menu.responder_lastx;
        }
        if (*ev).data1 & 1 as i32 != 0 {
            key = unsafe { game_state() }.m_controls.key_menu_forward;
            unsafe { game_state() }.m_menu.responder_mousewait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 15 as i32;
        }
        if (*ev).data1 & 2 as i32 != 0 {
            key = unsafe { game_state() }.m_controls.key_menu_back;
            unsafe { game_state() }.m_menu.responder_mousewait =
                I_GetTime(unsafe { &mut game_state().i_timer }) + 15 as i32;
        }
    } else if (*ev).type_0 as u32 == ev_keydown as i32 as u32 {
        key = (*ev).data1;
        ch = (*ev).data2;
    }
    if key == -(1 as i32) {
        return false;
    }
    if unsafe { game_state() }.m_menu.saveStringEnter != 0 {
        match key {
            KEY_BACKSPACE => {
                if unsafe { game_state() }.m_menu.saveCharIndex > 0 as i32 {
                    unsafe { game_state() }.m_menu.saveCharIndex -= 1;
                    unsafe { game_state() }.m_menu.savegamestrings
                        [unsafe { game_state() }.m_menu.saveSlot as usize]
                        .truncate(unsafe { game_state() }.m_menu.saveCharIndex as usize);
                }
            }
            KEY_ESCAPE => {
                unsafe { game_state() }.m_menu.saveStringEnter = 0 as i32;
                unsafe { game_state() }.m_menu.savegamestrings
                    [unsafe { game_state() }.m_menu.saveSlot as usize] =
                    unsafe { game_state() }.m_menu.saveOldString.clone();
            }
            KEY_ENTER => {
                unsafe { game_state() }.m_menu.saveStringEnter = 0 as i32;
                if !unsafe { game_state() }.m_menu.savegamestrings
                    [unsafe { game_state() }.m_menu.saveSlot as usize]
                    .is_empty()
                {
                    M_DoSave(unsafe { game_state() }.m_menu.saveSlot);
                }
            }
            _ => {
                if game_state().i_input.vanilla_keyboard_mapping != 0 {
                    ch = key;
                }
                ch = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<i32>() as usize > 1 as usize {
                        if 0 != 0 {
                            let mut __c: i32 = ch;
                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                __c as __int32_t
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            }) as i32;
                        } else {
                            __res = toupper(ch);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(ch as isize) as i32;
                    }
                    __res
                });
                if !(ch != ' ' as i32
                    && (ch - HU_FONTSTART < 0 as i32 || ch - HU_FONTSTART >= HU_FONTSIZE))
                {
                    if ch >= 32 as i32
                        && ch <= 127 as i32
                        && unsafe { game_state() }.m_menu.saveCharIndex < SAVESTRINGSIZE - 1 as i32
                        && M_StringWidth(
                            &unsafe { game_state() }.m_menu.savegamestrings
                                [unsafe { game_state() }.m_menu.saveSlot as usize],
                        ) < (SAVESTRINGSIZE - 2 as i32) * 8 as i32
                    {
                        unsafe { game_state() }.m_menu.saveCharIndex += 1;
                        unsafe { game_state() }.m_menu.savegamestrings
                            [unsafe { game_state() }.m_menu.saveSlot as usize]
                            .push(ch as u8 as char);
                    }
                }
            }
        }
        return true;
    }
    if unsafe { game_state() }.m_menu.messageToPrint != 0 {
        if unsafe { game_state() }.m_menu.messageNeedsInput {
            if key != ' ' as i32
                && key != KEY_ESCAPE
                && key != unsafe { game_state() }.m_controls.key_menu_confirm
                && key != unsafe { game_state() }.m_controls.key_menu_abort
            {
                return false;
            }
        }
        unsafe { game_state() }.m_menu.menuactive =
            unsafe { game_state() }.m_menu.messageLastMenuActive != 0;
        unsafe { game_state() }.m_menu.messageToPrint = 0 as i32;
        if unsafe { game_state() }.m_menu.messageRoutine.is_some() {
            unsafe { game_state() }
                .m_menu
                .messageRoutine
                .expect("non-null function pointer")(key);
        }
        unsafe { game_state() }.m_menu.menuactive = false;
        S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchx as i32);
        return true;
    }
    if unsafe { game_state() }.d_main.devparm && key == unsafe { game_state() }.m_controls.key_menu_help
        || key != 0 as i32 && key == unsafe { game_state() }.m_controls.key_menu_screenshot
    {
        G_ScreenShot();
        return true;
    }
    if !unsafe { game_state() }.m_menu.menuactive {
        if key == unsafe { game_state() }.m_controls.key_menu_decscreen {
            if unsafe { game_state() }.am_map.automapactive || unsafe { game_state() }.hu_stuff.chat_on {
                return false;
            }
            M_SizeDisplay(0 as i32);
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_stnmov as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_incscreen {
            if unsafe { game_state() }.am_map.automapactive || unsafe { game_state() }.hu_stuff.chat_on {
                return false;
            }
            M_SizeDisplay(1 as i32);
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_stnmov as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_help {
            M_StartControlPanel();
            if unsafe { game_state() }.doomstat.gamemode as u32 == retail as i32 as u32 {
                unsafe { game_state() }.m_menu.currentMenu =
                    &raw mut unsafe { game_state() }.m_menu.defs.ReadDef2;
            } else {
                unsafe { game_state() }.m_menu.currentMenu =
                    &raw mut unsafe { game_state() }.m_menu.defs.ReadDef1;
            }
            unsafe { game_state() }.m_menu.itemOn = 0 as i16;
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_save {
            M_StartControlPanel();
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_SaveGame(0 as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_load {
            M_StartControlPanel();
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_LoadGame(0 as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_volume {
            M_StartControlPanel();
            unsafe { game_state() }.m_menu.currentMenu =
                &raw mut unsafe { game_state() }.m_menu.defs.SoundDef;
            unsafe { game_state() }.m_menu.itemOn = sfx_vol as i32 as i16;
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_detail {
            M_ChangeDetail(0 as i32);
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_qsave {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_QuickSave();
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_endgame {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_EndGame(0 as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_messages {
            M_ChangeMessages(0 as i32);
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_qload {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_QuickLoad();
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_quit {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            M_QuitDOOM(0 as i32);
            return true;
        } else if key == unsafe { game_state() }.m_controls.key_menu_gamma {
            unsafe { game_state() }.i_video.usegamma += 1;
            if unsafe { game_state() }.i_video.usegamma > 4 as i32 {
                unsafe { game_state() }.i_video.usegamma = 0 as i32;
            }
            unsafe { game_state() }.g_game.players
                [unsafe { game_state() }.g_game.consoleplayer as usize]
                .message = gammamsg[unsafe { game_state() }.i_video.usegamma as usize].as_ptr() as *mut ::core::ffi::c_char;
            I_SetPalette(W_CacheLumpName("PLAYPAL", PU_CACHE as i32) as *mut byte);
            return true;
        }
    }
    if !unsafe { game_state() }.m_menu.menuactive {
        if key == unsafe { game_state() }.m_controls.key_menu_activate {
            M_StartControlPanel();
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
            return true;
        }
        return false;
    }
    if key == unsafe { game_state() }.m_controls.key_menu_down {
        loop {
            if unsafe { game_state() }.m_menu.itemOn as i32 + 1 as i32
                > (*unsafe { game_state() }.m_menu.currentMenu).numitems as i32 - 1 as i32
            {
                unsafe { game_state() }.m_menu.itemOn = 0 as i16;
            } else {
                unsafe { game_state() }.m_menu.itemOn += 1;
            }
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_pstop as i32);
            if !((*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .status as i32
                == -(1 as i32))
            {
                break;
            }
        }
        return true;
    } else if key == unsafe { game_state() }.m_controls.key_menu_up {
        loop {
            if unsafe { game_state() }.m_menu.itemOn == 0 {
                unsafe { game_state() }.m_menu.itemOn =
                    ((*unsafe { game_state() }.m_menu.currentMenu).numitems as i32 - 1 as i32)
                        as i16;
            } else {
                unsafe { game_state() }.m_menu.itemOn -= 1;
            }
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_pstop as i32);
            if !((*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .status as i32
                == -(1 as i32))
            {
                break;
            }
        }
        return true;
    } else if key == unsafe { game_state() }.m_controls.key_menu_left {
        if (*(*unsafe { game_state() }.m_menu.currentMenu)
            .menuitems
            .offset(unsafe { game_state() }.m_menu.itemOn as isize))
        .routine
        .is_some()
            && (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .status as i32
                == 2 as i32
        {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_stnmov as i32);
            (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .routine
            .expect("non-null function pointer")(0 as i32);
        }
        return true;
    } else if key == unsafe { game_state() }.m_controls.key_menu_right {
        if (*(*unsafe { game_state() }.m_menu.currentMenu)
            .menuitems
            .offset(unsafe { game_state() }.m_menu.itemOn as isize))
        .routine
        .is_some()
            && (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .status as i32
                == 2 as i32
        {
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_stnmov as i32);
            (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .routine
            .expect("non-null function pointer")(1 as i32);
        }
        return true;
    } else if key == unsafe { game_state() }.m_controls.key_menu_forward {
        if (*(*unsafe { game_state() }.m_menu.currentMenu)
            .menuitems
            .offset(unsafe { game_state() }.m_menu.itemOn as isize))
        .routine
        .is_some()
            && (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .status as i32
                != 0
        {
            (*unsafe { game_state() }.m_menu.currentMenu).lastOn =
                unsafe { game_state() }.m_menu.itemOn;
            if (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(unsafe { game_state() }.m_menu.itemOn as isize))
            .status as i32
                == 2 as i32
            {
                (*(*unsafe { game_state() }.m_menu.currentMenu)
                    .menuitems
                    .offset(unsafe { game_state() }.m_menu.itemOn as isize))
                .routine
                .expect("non-null function pointer")(1 as i32);
                S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_stnmov as i32);
            } else {
                (*(*unsafe { game_state() }.m_menu.currentMenu)
                    .menuitems
                    .offset(unsafe { game_state() }.m_menu.itemOn as isize))
                .routine
                .expect("non-null function pointer")(
                    unsafe { game_state() }.m_menu.itemOn as i32
                );
                S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_pistol as i32);
            }
        }
        return true;
    } else if key == unsafe { game_state() }.m_controls.key_menu_activate {
        (*unsafe { game_state() }.m_menu.currentMenu).lastOn =
            unsafe { game_state() }.m_menu.itemOn;
        M_ClearMenus();
        S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchx as i32);
        return true;
    } else if key == unsafe { game_state() }.m_controls.key_menu_back {
        (*unsafe { game_state() }.m_menu.currentMenu).lastOn =
            unsafe { game_state() }.m_menu.itemOn;
        if !(*unsafe { game_state() }.m_menu.currentMenu)
            .prevMenu
            .is_null()
        {
            unsafe { game_state() }.m_menu.currentMenu =
                (*unsafe { game_state() }.m_menu.currentMenu).prevMenu as *mut menu_t;
            unsafe { game_state() }.m_menu.itemOn =
                (*unsafe { game_state() }.m_menu.currentMenu).lastOn;
            S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_swtchn as i32);
        }
        return true;
    } else if ch != 0 as i32 || IsNullKey(key) {
        i = unsafe { game_state() }.m_menu.itemOn as i32 + 1 as i32;
        while i < (*unsafe { game_state() }.m_menu.currentMenu).numitems as i32 {
            if (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(i as isize))
            .alphaKey as i32
                == ch
            {
                unsafe { game_state() }.m_menu.itemOn = i as i16;
                S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_pstop as i32);
                return true;
            }
            i += 1;
        }
        i = 0 as i32;
        while i <= unsafe { game_state() }.m_menu.itemOn as i32 {
            if (*(*unsafe { game_state() }.m_menu.currentMenu)
                .menuitems
                .offset(i as isize))
            .alphaKey as i32
                == ch
            {
                unsafe { game_state() }.m_menu.itemOn = i as i16;
                S_StartSound(unsafe { &mut game_state().sounds }, NULL, sfx_pstop as i32);
                return true;
            }
            i += 1;
        }
    }
    return false;
}
pub unsafe fn M_StartControlPanel() {
    if unsafe { game_state() }.m_menu.menuactive {
        return;
    }
    unsafe { game_state() }.m_menu.menuactive = true;
    unsafe { game_state() }.m_menu.currentMenu =
        &raw mut unsafe { game_state() }.m_menu.defs.MainDef;
    unsafe { game_state() }.m_menu.itemOn = (*unsafe { game_state() }.m_menu.currentMenu).lastOn;
}
pub unsafe fn M_Drawer() {
    let mut i: u32 = 0;
    let mut max: u32 = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    unsafe { game_state() }.m_menu.inhelpscreens = false;
    if unsafe { game_state() }.m_menu.messageToPrint != 0 {
        unsafe { game_state() }.m_menu.drawer_y = (SCREENHEIGHT / 2 as i32
            - M_StringHeight(&unsafe { game_state() }.m_menu.messageString) / 2 as i32)
            as i16;
        for line in unsafe { game_state() }.m_menu.messageString.split('\n') {
            let line = if line.len() > 79 { &line[..79] } else { line };
            unsafe { game_state() }.m_menu.drawer_x =
                (SCREENWIDTH / 2 as i32 - M_StringWidth(line) / 2 as i32) as i16;
            M_WriteText(
                unsafe { game_state() }.m_menu.drawer_x as i32,
                unsafe { game_state() }.m_menu.drawer_y as i32,
                line,
            );
            unsafe { game_state() }.m_menu.drawer_y = (unsafe { game_state() }.m_menu.drawer_y
                as i32
                + (*unsafe { game_state() }.hu_stuff.hu_font[0 as i32 as usize]).height as i32)
                as i16;
        }
        return;
    }
    if !unsafe { game_state() }.m_menu.menuactive {
        return;
    }
    if (*unsafe { game_state() }.m_menu.currentMenu)
        .routine
        .is_some()
    {
        ::core::mem::transmute::<_, fn()>(
            (*unsafe { game_state() }.m_menu.currentMenu)
                .routine
                .expect("non-null function pointer"),
        )();
    }
    unsafe { game_state() }.m_menu.drawer_x = (*unsafe { game_state() }.m_menu.currentMenu).x;
    unsafe { game_state() }.m_menu.drawer_y = (*unsafe { game_state() }.m_menu.currentMenu).y;
    max = (*unsafe { game_state() }.m_menu.currentMenu).numitems as u32;
    i = 0 as u32;
    while i < max {
        name = &raw mut (*(*unsafe { game_state() }.m_menu.currentMenu)
            .menuitems
            .offset(i as isize))
        .name as *mut ::core::ffi::c_char;
        if *name.offset(0 as i32 as isize) != 0 {
            V_DrawPatchDirect(
                unsafe { &mut game_state().v_video },
                unsafe { game_state() }.m_menu.drawer_x as i32,
                unsafe { game_state() }.m_menu.drawer_y as i32,
                W_CacheLumpName(&wad_name8_to_string(name), PU_CACHE as i32) as *mut patch_t,
            );
        }
        unsafe { game_state() }.m_menu.drawer_y =
            (unsafe { game_state() }.m_menu.drawer_y as i32 + LINEHEIGHT) as i16;
        i = i.wrapping_add(1);
    }
    V_DrawPatchDirect(
        unsafe { &mut game_state().v_video },
        unsafe { game_state() }.m_menu.drawer_x as i32 + SKULLXOFF,
        (*unsafe { game_state() }.m_menu.currentMenu).y as i32 - 5 as i32
            + unsafe { game_state() }.m_menu.itemOn as i32 * LINEHEIGHT,
        W_CacheLumpName(
            skullName[unsafe { game_state() }.m_menu.whichSkull as usize],
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
pub unsafe fn M_ClearMenus() {
    unsafe { game_state() }.m_menu.menuactive = false;
}
pub unsafe fn M_SetupNextMenu(mut menudef: *mut menu_t) {
    unsafe { game_state() }.m_menu.currentMenu = menudef;
    unsafe { game_state() }.m_menu.itemOn = (*unsafe { game_state() }.m_menu.currentMenu).lastOn;
}
pub unsafe fn M_Ticker() {
    unsafe { game_state() }.m_menu.skullAnimCounter -= 1;
    if unsafe { game_state() }.m_menu.skullAnimCounter as i32 <= 0 as i32 {
        unsafe { game_state() }.m_menu.whichSkull =
            (unsafe { game_state() }.m_menu.whichSkull as i32 ^ 1 as i32) as i16;
        unsafe { game_state() }.m_menu.skullAnimCounter = 8 as i16;
    }
}
pub unsafe fn M_Init() {
    unsafe { game_state() }.m_menu.currentMenu =
        &raw mut unsafe { game_state() }.m_menu.defs.MainDef;
    unsafe { game_state() }.m_menu.menuactive = false;
    unsafe { game_state() }.m_menu.itemOn = (*unsafe { game_state() }.m_menu.currentMenu).lastOn;
    unsafe { game_state() }.m_menu.whichSkull = 0 as i16;
    unsafe { game_state() }.m_menu.skullAnimCounter = 10 as i16;
    unsafe { game_state() }.m_menu.screenSize =
        unsafe { game_state() }.m_menu.screenblocks - 3 as i32;
    unsafe { game_state() }.m_menu.messageToPrint = 0 as i32;
    unsafe { game_state() }.m_menu.messageString = String::new();
    unsafe { game_state() }.m_menu.messageLastMenuActive =
        unsafe { game_state() }.m_menu.menuactive as i32;
    unsafe { game_state() }.m_menu.quickSaveSlot = -(1 as i32);
    match unsafe { game_state() }.doomstat.gamemode as u32 {
        2 => {
            unsafe { game_state() }.m_menu.menus.MainMenu[readthis as i32 as usize] =
                unsafe { game_state() }.m_menu.menus.MainMenu[quitdoom as i32 as usize];
            unsafe { game_state() }.m_menu.defs.MainDef.numitems -= 1;
            unsafe { game_state() }.m_menu.defs.MainDef.y =
                (unsafe { game_state() }.m_menu.defs.MainDef.y as i32 + 8 as i32) as i16;
            unsafe { game_state() }.m_menu.defs.NewDef.prevMenu =
                &raw mut unsafe { game_state() }.m_menu.defs.MainDef as *mut menu_s;
        }
        0 => {}
        1 | 3 | _ => {}
    }
    if (unsafe { game_state() }.doomstat.gameversion as u32) < exe_ultimate as i32 as u32 {
        unsafe { game_state() }.m_menu.defs.EpiDef.numitems -= 1;
    }
}
