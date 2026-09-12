use crate::src::doomdef::NULL;
use crate::src::game_state::game_state;
use crate::src::game_state::GameState;
use crate::src::i_system::I_Error;
use crate::src::m_argv::M_CheckParmWithArgs;
use crate::src::m_controls::KEY_BACKSPACE;
use crate::src::m_controls::KEY_CAPSLOCK;
use crate::src::m_controls::KEY_DEL;
use crate::src::m_controls::KEY_DOWNARROW;
use crate::src::m_controls::KEY_END;
use crate::src::m_controls::KEY_F1;
use crate::src::m_controls::KEY_F10;
use crate::src::m_controls::KEY_F11;
use crate::src::m_controls::KEY_F12;
use crate::src::m_controls::KEY_F2;
use crate::src::m_controls::KEY_F3;
use crate::src::m_controls::KEY_F4;
use crate::src::m_controls::KEY_F5;
use crate::src::m_controls::KEY_F6;
use crate::src::m_controls::KEY_F7;
use crate::src::m_controls::KEY_F8;
use crate::src::m_controls::KEY_F9;
use crate::src::m_controls::KEY_HOME;
use crate::src::m_controls::KEY_INS;
use crate::src::m_controls::KEY_LEFTARROW;
use crate::src::m_controls::KEY_MINUS;
use crate::src::m_controls::KEY_PAUSE;
use crate::src::m_controls::KEY_PGDN;
use crate::src::m_controls::KEY_PGUP;
use crate::src::m_controls::KEY_RALT;
use crate::src::m_controls::KEY_RIGHTARROW;
use crate::src::m_controls::KEY_RSHIFT;
use crate::src::m_controls::KEY_SCRLCK;
use crate::src::m_controls::KEY_UPARROW;
use crate::src::m_misc::M_MakeDirectory;
use crate::src::m_misc::M_StrToInt;

pub type default_type_t = u32;
pub const DEFAULT_KEY: default_type_t = 4;
pub const DEFAULT_FLOAT: default_type_t = 3;
pub const DEFAULT_STRING: default_type_t = 2;
pub const DEFAULT_INT_HEX: default_type_t = 1;
pub const DEFAULT_INT: default_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct default_t {
    pub name: &'static str,
    pub location: *mut ::core::ffi::c_void,
    pub type_0: default_type_t,
    pub untranslated: i32,
    pub original_translated: i32,
    pub bound: bool,
}
#[derive(Clone)]
#[repr(C)]
pub struct default_collection_t {
    pub defaults: *mut default_t,
    pub numdefaults: i32,
    pub filename: String,
}
pub const DIR_SEPARATOR_S: &str = "/";
pub const KEY_RCTRL: i32 = 0x80 + 0x1d as i32;
pub const KEY_PRTSCR: i32 = 0x80 + 0x59 as i32;
pub const KEYP_5: i32 = '5' as i32;
pub const KEYP_PLUS: i32 = '+' as i32;
pub const KEYP_MULTIPLY: i32 = '*' as i32;
pub struct MConfigState {
    configdir: String,
    default_main_config: &'static str,
    default_extra_config: &'static str,
    doom_defaults_list: [default_t; 76],
    doom_defaults: default_collection_t,
    extra_defaults_list: [default_t; 119],
    extra_defaults: default_collection_t,
}

impl MConfigState {
    pub const fn new() -> Self {
        MConfigState {
            configdir: String::new(),
            default_main_config: "",
            default_extra_config: "",
            doom_defaults_list: [
                default_t {
                    name: "mouse_sensitivity",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "sfx_volume",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "music_volume",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "show_talk",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "voice_volume",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "show_messages",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_right",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_left",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_up",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_down",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_strafeleft",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_straferight",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_useHealth",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_jump",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_flyup",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_flydown",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_flycenter",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_lookup",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_lookdown",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_lookcenter",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invquery",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_mission",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invPop",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invKey",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invHome",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invEnd",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invleft",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invright",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invLeft",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invRight",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_useartifact",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invUse",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_invDrop",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_lookUp",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_lookDown",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_fire",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_use",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_strafe",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_speed",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "use_mouse",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_fire",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_strafe",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_forward",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_jump",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "use_joystick",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_fire",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_strafe",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_use",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_speed",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_jump",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "screenblocks",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "screensize",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "detaillevel",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_channels",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_musicdevice",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_sfxdevice",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_sbport",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_sbirq",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_sbdma",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_mport",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "usegamma",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "savedir",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "messageson",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "back_flat",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "nickname",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro0",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro1",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro2",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro3",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro4",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro5",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro6",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro7",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro8",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "chatmacro9",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "comport",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
            ],
            doom_defaults: default_collection_t {
                defaults: ::core::ptr::null::<default_t>() as *mut default_t,
                numdefaults: 0,
                filename: String::new(),
            },
            extra_defaults_list: [
                default_t {
                    name: "graphical_startup",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "autoadjust_video_settings",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "fullscreen",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "aspect_ratio_correct",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "startup_delay",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "screen_width",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "screen_height",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "screen_bpp",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "grabmouse",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "novert",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouse_acceleration",
                    location: NULL,
                    type_0: DEFAULT_FLOAT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouse_threshold",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_samplerate",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_cachesize",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_maxslicetime_ms",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "snd_musiccmd",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "opl_io_port",
                    location: NULL,
                    type_0: DEFAULT_INT_HEX,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "show_endoom",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "png_screenshots",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "vanilla_savegame_limit",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "vanilla_demo_limit",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "vanilla_keyboard_mapping",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "video_driver",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "window_position",
                    location: NULL,
                    type_0: DEFAULT_STRING,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_index",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_x_axis",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_x_invert",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_y_axis",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_y_invert",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_strafe_axis",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_strafe_invert",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button0",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button1",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button2",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button3",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button4",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button5",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button6",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button7",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button8",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joystick_physical_button9",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_strafeleft",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_straferight",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_menu_activate",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_prevweapon",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "joyb_nextweapon",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_strafeleft",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_straferight",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_use",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_backward",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_prevweapon",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "mouseb_nextweapon",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "dclick_use",
                    location: NULL,
                    type_0: DEFAULT_INT,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_pause",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_activate",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_up",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_down",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_left",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_right",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_back",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_forward",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_confirm",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_abort",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_help",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_save",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_load",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_volume",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_detail",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_qsave",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_endgame",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_messages",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_qload",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_quit",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_gamma",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_spy",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_incscreen",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_decscreen",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_menu_screenshot",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_toggle",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_north",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_south",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_east",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_west",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_zoomin",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_zoomout",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_maxzoom",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_follow",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_grid",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_mark",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_map_clearmark",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon1",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon2",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon3",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon4",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon5",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon6",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon7",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_weapon8",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_prevweapon",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_nextweapon",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_all",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_health",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_poisonbag",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_blastradius",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_teleport",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_teleportother",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_egg",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_arti_invulnerability",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_message_refresh",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_demo_quit",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msg",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer1",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer2",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer3",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer4",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer5",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer6",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer7",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
                default_t {
                    name: "key_multi_msgplayer8",
                    location: NULL,
                    type_0: DEFAULT_KEY,
                    untranslated: 0 as i32,
                    original_translated: 0 as i32,
                    bound: false,
                },
            ],
            extra_defaults: default_collection_t {
                defaults: ::core::ptr::null::<default_t>() as *mut default_t,
                numdefaults: 0,
                filename: String::new(),
            },
        }
    }

    // `doom_defaults`/`extra_defaults`'s `.defaults` must point at this same
    // struct's own `_list` array field. That address isn't known until this
    // value is at its final, permanently-stable 'static home (inside
    // `GameState`, behind `OnceLock`) -- computing it inline above, during
    // construction, would capture the address of a temporary that gets moved
    // at least twice more before settling. Called once from `game_state()`
    // itself, strictly after `OnceLock::get_or_init` returns, same pattern as
    // `sounds::fixup_self_links`/`p_maputl::fixup_intercepts_overrun`.
    pub fn fixup_defaults(&mut self) {
        self.doom_defaults.defaults = &raw mut self.doom_defaults_list as *mut default_t;
        self.doom_defaults.numdefaults = self.doom_defaults_list.len() as i32;
        self.extra_defaults.defaults = &raw mut self.extra_defaults_list as *mut default_t;
        self.extra_defaults.numdefaults = self.extra_defaults_list.len() as i32;
    }
}

unsafe fn SearchCollection(
    mut collection: *mut default_collection_t,
    name: &str,
) -> *mut default_t {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*collection).numdefaults {
        if (*(*collection).defaults.offset(i as isize)).name == name {
            return (*collection).defaults.offset(i as isize) as *mut default_t;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<default_t>();
}
static scantokey: [i32; 128] = [
    0 as i32,
    27 as i32,
    '1' as i32,
    '2' as i32,
    '3' as i32,
    '4' as i32,
    '5' as i32,
    '6' as i32,
    '7' as i32,
    '8' as i32,
    '9' as i32,
    '0' as i32,
    '-' as i32,
    '=' as i32,
    KEY_BACKSPACE,
    9 as i32,
    'q' as i32,
    'w' as i32,
    'e' as i32,
    'r' as i32,
    't' as i32,
    'y' as i32,
    'u' as i32,
    'i' as i32,
    'o' as i32,
    'p' as i32,
    '[' as i32,
    ']' as i32,
    13 as i32,
    KEY_RCTRL,
    'a' as i32,
    's' as i32,
    'd' as i32,
    'f' as i32,
    'g' as i32,
    'h' as i32,
    'j' as i32,
    'k' as i32,
    'l' as i32,
    ';' as i32,
    '\'' as i32,
    '`' as i32,
    KEY_RSHIFT,
    '\\' as i32,
    'z' as i32,
    'x' as i32,
    'c' as i32,
    'v' as i32,
    'b' as i32,
    'n' as i32,
    'm' as i32,
    ',' as i32,
    '.' as i32,
    '/' as i32,
    KEY_RSHIFT,
    KEYP_MULTIPLY,
    KEY_RALT,
    ' ' as i32,
    KEY_CAPSLOCK,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_PAUSE,
    KEY_SCRLCK,
    KEY_HOME,
    KEY_UPARROW,
    KEY_PGUP,
    KEY_MINUS,
    KEY_LEFTARROW,
    KEYP_5,
    KEY_RIGHTARROW,
    KEYP_PLUS,
    KEY_END,
    KEY_DOWNARROW,
    KEY_PGDN,
    KEY_INS,
    KEY_DEL,
    0 as i32,
    0 as i32,
    0 as i32,
    KEY_F11,
    KEY_F12,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    KEY_PRTSCR,
    0 as i32,
];
unsafe fn ParseIntParameter(strparm: &str) -> i32 {
    let mut parm: i32 = 0;
    M_StrToInt(strparm, &mut parm);
    return parm;
}
unsafe fn SetVariable(mut def: *mut default_t, mut value: *mut ::core::ffi::c_char) {
    let mut intparm: i32 = 0;
    match (*def).type_0 as u32 {
        2 => {
            let ref mut fresh0 = *((*def).location as *mut *mut ::core::ffi::c_char);
            *fresh0 = ::std::ffi::CStr::from_ptr(value).to_owned().into_raw();
        }
        0 | 1 => {
            *((*def).location as *mut i32) =
                ParseIntParameter(::std::ffi::CStr::from_ptr(value).to_str().unwrap());
        }
        4 => {
            intparm = ParseIntParameter(::std::ffi::CStr::from_ptr(value).to_str().unwrap());
            (*def).untranslated = intparm;
            if intparm >= 0 as i32 && intparm < 128 as i32 {
                intparm = scantokey[intparm as usize];
            } else {
                intparm = 0 as i32;
            }
            (*def).original_translated = intparm;
            *((*def).location as *mut i32) = intparm;
        }
        3 => {
            let value_str = ::std::ffi::CStr::from_ptr(value).to_str().unwrap();
            *((*def).location as *mut f32) = value_str.trim().parse::<f64>().unwrap_or(0.0) as f32;
        }
        _ => {}
    };
}
pub unsafe fn M_SetConfigFilenames(
    state: &mut MConfigState,
    main_config: &'static str,
    extra_config: &'static str,
) {
    state.default_main_config = main_config;
    state.default_extra_config = extra_config;
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveDefaults(_state: &mut GameState) {}
pub unsafe fn M_SaveDefaultsAlternate(state: &mut MConfigState, main_0: &str, extra: &str) {
    let orig_main = state.doom_defaults.filename.clone();
    let orig_extra = state.extra_defaults.filename.clone();
    state.doom_defaults.filename = main_0.to_string();
    state.extra_defaults.filename = extra.to_string();
    M_SaveDefaults(unsafe { game_state() });
    state.doom_defaults.filename = orig_main;
    state.extra_defaults.filename = orig_extra;
}
pub unsafe fn M_LoadDefaults(state: &mut GameState) {
    let mut i: i32 = 0;
    i = M_CheckParmWithArgs(state, "-config", 1 as i32);
    if i != 0 {
        state.m_config.doom_defaults.filename = state.m_argv.myargv[(i + 1 as i32) as usize]
            .to_str()
            .unwrap()
            .to_string();
        println!(
            "\tdefault file: {}",
            state.m_config.doom_defaults.filename,
        );
    } else {
        state.m_config.doom_defaults.filename = format!(
            "{}{}",
            state.m_config.configdir, state.m_config.default_main_config
        );
    }
    println!("saving config in {}", state.m_config.doom_defaults.filename);
    i = M_CheckParmWithArgs(state, "-extraconfig", 1 as i32);
    if i != 0 {
        state.m_config.extra_defaults.filename = state.m_argv.myargv[(i + 1 as i32) as usize]
            .to_str()
            .unwrap()
            .to_string();
        println!(
            "        extra configuration file: {}",
            state.m_config.extra_defaults.filename,
        );
    } else {
        state.m_config.extra_defaults.filename = format!(
            "{}{}",
            state.m_config.configdir, state.m_config.default_extra_config
        );
    }
}
unsafe fn GetDefaultForName(state: &mut MConfigState, name: &str) -> *mut default_t {
    let mut result: *mut default_t = ::core::ptr::null_mut::<default_t>();
    result = SearchCollection(&raw mut state.doom_defaults, name);
    if result.is_null() {
        result = SearchCollection(&raw mut state.extra_defaults, name);
    }
    if result.is_null() {
        I_Error(&format!("Unknown configuration variable: '{}'", name));
    }
    return result;
}
pub unsafe fn M_BindVariable(
    state: &mut MConfigState,
    name: &str,
    mut location: *mut ::core::ffi::c_void,
) {
    let mut variable: *mut default_t = ::core::ptr::null_mut::<default_t>();
    variable = GetDefaultForName(state, name);
    (*variable).location = location;
    (*variable).bound = true;
}
pub unsafe fn M_SetVariable(
    state: &mut MConfigState,
    name: &str,
    mut value: *mut ::core::ffi::c_char,
) -> bool {
    let mut variable: *mut default_t = ::core::ptr::null_mut::<default_t>();
    variable = GetDefaultForName(state, name);
    if variable.is_null() || !(*variable).bound {
        return false;
    }
    SetVariable(variable, value);
    return true;
}
pub unsafe fn M_GetIntVariable(state: &mut MConfigState, name: &str) -> i32 {
    let mut variable: *mut default_t = ::core::ptr::null_mut::<default_t>();
    variable = GetDefaultForName(state, name);
    if variable.is_null()
        || !(*variable).bound
        || (*variable).type_0 as u32 != DEFAULT_INT as i32 as u32
            && (*variable).type_0 as u32 != DEFAULT_INT_HEX as i32 as u32
    {
        return 0 as i32;
    }
    return *((*variable).location as *mut i32);
}
pub unsafe fn M_GetStrVariable(state: &mut MConfigState, name: &str) -> *const ::core::ffi::c_char {
    let mut variable: *mut default_t = ::core::ptr::null_mut::<default_t>();
    variable = GetDefaultForName(state, name);
    if variable.is_null()
        || !(*variable).bound
        || (*variable).type_0 as u32 != DEFAULT_STRING as i32 as u32
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return *((*variable).location as *mut *const ::core::ffi::c_char);
}
pub unsafe fn M_GetFloatVariable(state: &mut MConfigState, name: &str) -> f32 {
    let mut variable: *mut default_t = ::core::ptr::null_mut::<default_t>();
    variable = GetDefaultForName(state, name);
    if variable.is_null()
        || !(*variable).bound
        || (*variable).type_0 as u32 != DEFAULT_FLOAT as i32 as u32
    {
        return 0 as i32 as f32;
    }
    return *((*variable).location as *mut f32);
}
fn GetDefaultConfigDir() -> String {
    ".".to_string()
}
pub unsafe fn M_SetConfigDir(state: &mut MConfigState, dir: Option<&str>) {
    if let Some(dir) = dir {
        state.configdir = dir.to_string();
    } else {
        state.configdir = GetDefaultConfigDir();
    }
    if !state.configdir.is_empty() {
        println!("Using {} for configuration and saves", state.configdir);
    }
    M_MakeDirectory(&state.configdir);
}
pub unsafe fn M_GetSaveGameDir(state: &mut MConfigState, _iwadname: &'static str) -> String {
    let savegamedir;
    if state.configdir.is_empty() {
        savegamedir = String::new();
    } else {
        savegamedir = format!("{}{}.savegame/", state.configdir, DIR_SEPARATOR_S);
        M_MakeDirectory(&savegamedir);
        println!("Using {} for savegames", savegamedir);
    }
    savegamedir
}
