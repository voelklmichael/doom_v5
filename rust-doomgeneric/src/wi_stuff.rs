use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::d_player::{player_t};
use crate::src::w_wad::{
    wad_name8_to_string, W_CacheLumpName, W_CheckNumForName, W_ReleaseLumpName,
};
use crate::src::g_game::G_WorldDone;
use crate::src::m_random::M_Random;
use crate::src::s_sound::S_ChangeMusic;
use crate::src::m_misc::M_StringCopy;
use crate::src::g_game::deathmatch;
use crate::src::g_game::playeringame;
use crate::src::g_game::netgame;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::v_video::V_DrawPatch;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use crate::src::sounds::{sfx_barexp, sfx_pistol, sfx_pldeth, sfx_sgcock, sfx_slop};
use crate::src::sounds::{mus_dm2int, mus_inter};
use crate::src::d_ticcmd::{BT_ATTACK, BT_USE};
use crate::src::p_mobj::mobjtype_t;
use crate::src::d_mode::{commercial, retail};
use crate::src::stdint_types::size_t;
use libc::{printf, snprintf};
use crate::src::st_stuff::load_callback_t;

pub const NUMMOBJTYPES: mobjtype_t = 137;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wbplayerstruct_t {
    pub in_0: bool,
    pub skills: i32,
    pub sitems: i32,
    pub ssecret: i32,
    pub stime: i32,
    pub frags: [i32; 4],
    pub score: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wbstartstruct_t {
    pub epsd: i32,
    pub didsecret: bool,
    pub last: i32,
    pub next: i32,
    pub maxkills: i32,
    pub maxitems: i32,
    pub maxsecret: i32,
    pub maxfrags: i32,
    pub partime: i32,
    pub pnum: i32,
    pub plyr: [wbplayerstruct_t; 4],
}
pub type stateenum_t = i32;
pub const ShowNextLoc: stateenum_t = 1;
pub const StatCount: stateenum_t = 0;
pub const NoState: stateenum_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct anim_t {
    pub type_0: animenum_t,
    pub period: i32,
    pub nanims: i32,
    pub loc: point_t,
    pub data1: i32,
    pub data2: i32,
    pub p: [*mut patch_t; 3],
    pub nexttic: i32,
    pub lastdrawn: i32,
    pub ctr: i32,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_t {
    pub x: i32,
    pub y: i32,
}
pub type animenum_t = u32;
pub const ANIM_LEVEL: animenum_t = 2;
pub const ANIM_RANDOM: animenum_t = 1;
pub const ANIM_ALWAYS: animenum_t = 0;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TICRATE: i32 = 35 as i32;
pub const MAXPLAYERS: i32 = 4 as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const NUMMAPS: i32 = 9 as i32;
pub const WI_TITLEY: i32 = 2 as i32;
pub const WI_SPACINGY: i32 = 33 as i32;
pub const SP_STATSX: i32 = 50 as i32;
pub const SP_STATSY: i32 = 50 as i32;
pub const SP_TIMEX: i32 = 16 as i32;
pub const SP_TIMEY: i32 = SCREENHEIGHT - 32 as i32;
pub const NG_STATSY: i32 = 50 as i32;
pub const NG_SPACINGX: i32 = 64 as i32;
pub const DM_MATRIXX: i32 = 42 as i32;
pub const DM_MATRIXY: i32 = 68 as i32;
pub const DM_SPACINGX: i32 = 40 as i32;
pub const DM_TOTALSX: i32 = 269 as i32;
pub const DM_KILLERSX: i32 = 10 as i32;
pub const DM_KILLERSY: i32 = 100 as i32;
pub const DM_VICTIMSX: i32 = 5 as i32;
pub const DM_VICTIMSY: i32 = 50 as i32;
static mut lnodes: [[point_t; 9]; 4] = [
    [
        point_t {
            x: 185 as i32,
            y: 164 as i32,
        },
        point_t {
            x: 148 as i32,
            y: 143 as i32,
        },
        point_t {
            x: 69 as i32,
            y: 122 as i32,
        },
        point_t {
            x: 209 as i32,
            y: 102 as i32,
        },
        point_t {
            x: 116 as i32,
            y: 89 as i32,
        },
        point_t {
            x: 166 as i32,
            y: 55 as i32,
        },
        point_t {
            x: 71 as i32,
            y: 56 as i32,
        },
        point_t {
            x: 135 as i32,
            y: 29 as i32,
        },
        point_t {
            x: 71 as i32,
            y: 24 as i32,
        },
    ],
    [
        point_t {
            x: 254 as i32,
            y: 25 as i32,
        },
        point_t {
            x: 97 as i32,
            y: 50 as i32,
        },
        point_t {
            x: 188 as i32,
            y: 64 as i32,
        },
        point_t {
            x: 128 as i32,
            y: 78 as i32,
        },
        point_t {
            x: 214 as i32,
            y: 92 as i32,
        },
        point_t {
            x: 133 as i32,
            y: 130 as i32,
        },
        point_t {
            x: 208 as i32,
            y: 136 as i32,
        },
        point_t {
            x: 148 as i32,
            y: 140 as i32,
        },
        point_t {
            x: 235 as i32,
            y: 158 as i32,
        },
    ],
    [
        point_t {
            x: 156 as i32,
            y: 168 as i32,
        },
        point_t {
            x: 48 as i32,
            y: 154 as i32,
        },
        point_t {
            x: 174 as i32,
            y: 95 as i32,
        },
        point_t {
            x: 265 as i32,
            y: 75 as i32,
        },
        point_t {
            x: 130 as i32,
            y: 48 as i32,
        },
        point_t {
            x: 279 as i32,
            y: 23 as i32,
        },
        point_t {
            x: 198 as i32,
            y: 48 as i32,
        },
        point_t {
            x: 140 as i32,
            y: 25 as i32,
        },
        point_t {
            x: 281 as i32,
            y: 136 as i32,
        },
    ],
    [point_t { x: 0, y: 0 }; 9],
];
static mut epsd0animinfo: [anim_t; 10] = [
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 224 as i32,
            y: 104 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 184 as i32,
            y: 160 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 112 as i32,
            y: 136 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 72 as i32,
            y: 112 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 88 as i32,
            y: 96 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 64 as i32,
            y: 48 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 192 as i32,
            y: 40 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 136 as i32,
            y: 16 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 80 as i32,
            y: 16 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 64 as i32,
            y: 24 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
];
static mut epsd1animinfo: [anim_t; 9] = [
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 1 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 2 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 3 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 4 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 5 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 6 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 7 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 192 as i32,
            y: 144 as i32,
        },
        data1: 8 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 8 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
];
static mut epsd2animinfo: [anim_t; 6] = [
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 104 as i32,
            y: 168 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 40 as i32,
            y: 136 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 160 as i32,
            y: 96 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 104 as i32,
            y: 80 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 120 as i32,
            y: 32 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 4 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 40 as i32,
            y: 0 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
];
static mut NUMANIMS: [i32; 4] = [0; 4];
static mut anims: [*mut anim_t; 4] = unsafe {
    [
        &raw const epsd0animinfo as *mut anim_t,
        &raw const epsd1animinfo as *mut anim_t,
        &raw const epsd2animinfo as *mut anim_t,
        ::core::ptr::null::<anim_t>() as *mut anim_t,
    ]
};
pub const SHOWNEXTLOCDELAY: i32 = 4 as i32;
static mut acceleratestage: i32 = 0;
static mut me: i32 = 0;
static mut state: stateenum_t = StatCount;
static mut wbs: *mut wbstartstruct_t = ::core::ptr::null::<wbstartstruct_t>()
    as *mut wbstartstruct_t;
static mut plrs: *mut wbplayerstruct_t = ::core::ptr::null::<wbplayerstruct_t>()
    as *mut wbplayerstruct_t;
static mut cnt: i32 = 0;
static mut bcnt: i32 = 0;
static mut firstrefresh: i32 = 0;
static mut cnt_kills: [i32; 4] = [0; 4];
static mut cnt_items: [i32; 4] = [0; 4];
static mut cnt_secret: [i32; 4] = [0; 4];
static mut cnt_time: i32 = 0;
static mut cnt_par: i32 = 0;
static mut cnt_pause: i32 = 0;
static mut NUMCMAPS: i32 = 0;
static mut yah: [*mut patch_t; 3] = [
    ::core::ptr::null::<patch_t>() as *mut patch_t,
    ::core::ptr::null::<patch_t>() as *mut patch_t,
    ::core::ptr::null::<patch_t>() as *mut patch_t,
];
static mut splat: [*mut patch_t; 2] = [
    ::core::ptr::null::<patch_t>() as *mut patch_t,
    ::core::ptr::null::<patch_t>() as *mut patch_t,
];
static mut percent: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut colon: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut num: [*mut patch_t; 10] = [::core::ptr::null::<patch_t>()
    as *mut patch_t; 10];
static mut wiminus: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut finished: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut entering: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut sp_secret: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut kills: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut secret: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut items: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut frags: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut timepatch: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut par: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut sucks: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut killers: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut victims: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut total: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut star: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut bstar: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut p: [*mut patch_t; 4] = [::core::ptr::null::<patch_t>() as *mut patch_t; 4];
static mut bp: [*mut patch_t; 4] = [::core::ptr::null::<patch_t>() as *mut patch_t; 4];
static mut lnames: *mut *mut patch_t = ::core::ptr::null::<*mut patch_t>()
    as *mut *mut patch_t;
static mut background: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
#[no_mangle]
pub unsafe extern "C" fn WI_slamBackground() {
    V_DrawPatch(0 as i32, 0 as i32, background);
}
#[no_mangle]
pub unsafe extern "C" fn WI_Responder(mut ev: *mut event_t) -> bool {
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawLF() {
    let mut y: i32 = WI_TITLEY;
    if gamemode as u32
        != commercial as i32 as u32
        || (*wbs).last < NUMCMAPS
    {
        V_DrawPatch(
            (SCREENWIDTH
                - (**lnames.offset((*wbs).last as isize)).width as i32)
                / 2 as i32,
            y,
            *lnames.offset((*wbs).last as isize),
        );
        y
            += 5 as i32
                * (**lnames.offset((*wbs).last as isize)).height as i32
                / 4 as i32;
        V_DrawPatch(
            (SCREENWIDTH - (*finished).width as i32)
                / 2 as i32,
            y,
            finished,
        );
    } else if !((*wbs).last == NUMCMAPS) {
        if (*wbs).last > NUMCMAPS {
            let mut tmp: patch_t = patch_t {
                width: SCREENWIDTH as i16,
                height: SCREENHEIGHT as i16,
                leftoffset: 1 as i16,
                topoffset: 1 as i16,
                columnofs: [
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                ],
            };
            V_DrawPatch(0 as i32, y, &raw mut tmp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawEL() {
    let mut y: i32 = WI_TITLEY;
    V_DrawPatch(
        (SCREENWIDTH - (*entering).width as i32)
            / 2 as i32,
        y,
        entering,
    );
    y
        += 5 as i32
            * (**lnames.offset((*wbs).next as isize)).height as i32
            / 4 as i32;
    V_DrawPatch(
        (SCREENWIDTH
            - (**lnames.offset((*wbs).next as isize)).width as i32)
            / 2 as i32,
        y,
        *lnames.offset((*wbs).next as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawOnLnode(
    mut n: i32,
    mut c: *mut *mut patch_t,
) {
    let mut i: i32 = 0;
    let mut left: i32 = 0;
    let mut top: i32 = 0;
    let mut right: i32 = 0;
    let mut bottom: i32 = 0;
    let mut fits: bool = false;
    i = 0 as i32;
    loop {
        left = lnodes[(*wbs).epsd as usize][n as usize].x
            - (**c.offset(i as isize)).leftoffset as i32;
        top = lnodes[(*wbs).epsd as usize][n as usize].y
            - (**c.offset(i as isize)).topoffset as i32;
        right = left + (**c.offset(i as isize)).width as i32;
        bottom = top + (**c.offset(i as isize)).height as i32;
        if left >= 0 as i32 && right < SCREENWIDTH
            && top >= 0 as i32 && bottom < SCREENHEIGHT
        {
            fits = true;
        } else {
            i += 1;
        }
        if !(!fits && i != 2 as i32
            && !(*c.offset(i as isize)).is_null())
        {
            break;
        }
    }
    if fits && i < 2 as i32 {
        V_DrawPatch(
            lnodes[(*wbs).epsd as usize][n as usize].x,
            lnodes[(*wbs).epsd as usize][n as usize].y,
            *c.offset(i as isize),
        );
    } else {
        printf(
            b"Could not place patch on level %d\0" as *const u8
                as *const ::core::ffi::c_char,
            n + 1 as i32,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn WI_initAnimatedBack() {
    let mut i: i32 = 0;
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if (*wbs).epsd > 2 as i32 {
        return;
    }
    i = 0 as i32;
    while i < NUMANIMS[(*wbs).epsd as usize] {
        a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
            .offset(i as isize) as *mut anim_t;
        (*a).ctr = -(1 as i32);
        if (*a).type_0 as u32
            == ANIM_ALWAYS as i32 as u32
        {
            (*a).nexttic = bcnt + 1 as i32 + M_Random() % (*a).period;
        } else if (*a).type_0 as u32
            == ANIM_RANDOM as i32 as u32
        {
            (*a).nexttic = bcnt + 1 as i32 + (*a).data2
                + M_Random() % (*a).data1;
        } else if (*a).type_0 as u32
            == ANIM_LEVEL as i32 as u32
        {
            (*a).nexttic = bcnt + 1 as i32;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateAnimatedBack() {
    let mut i: i32 = 0;
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if (*wbs).epsd > 2 as i32 {
        return;
    }
    i = 0 as i32;
    while i < NUMANIMS[(*wbs).epsd as usize] {
        a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
            .offset(i as isize) as *mut anim_t;
        if bcnt == (*a).nexttic {
            match (*a).type_0 as u32 {
                0 => {
                    (*a).ctr += 1;
                    if (*a).ctr >= (*a).nanims {
                        (*a).ctr = 0 as i32;
                    }
                    (*a).nexttic = bcnt + (*a).period;
                }
                1 => {
                    (*a).ctr += 1;
                    if (*a).ctr == (*a).nanims {
                        (*a).ctr = -(1 as i32);
                        (*a).nexttic = bcnt + (*a).data2 + M_Random() % (*a).data1;
                    } else {
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                2 => {
                    if !(state as i32 == StatCount as i32
                        && i == 7 as i32) && (*wbs).next == (*a).data1
                    {
                        (*a).ctr += 1;
                        if (*a).ctr == (*a).nanims {
                            (*a).ctr -= 1;
                        }
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawAnimatedBack() {
    let mut i: i32 = 0;
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if (*wbs).epsd > 2 as i32 {
        return;
    }
    i = 0 as i32;
    while i < NUMANIMS[(*wbs).epsd as usize] {
        a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
            .offset(i as isize) as *mut anim_t;
        if (*a).ctr >= 0 as i32 {
            V_DrawPatch((*a).loc.x, (*a).loc.y, (*a).p[(*a).ctr as usize]);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawNum(
    mut x: i32,
    mut y: i32,
    mut n: i32,
    mut digits: i32,
) -> i32 {
    let mut fontwidth: i32 = (*num[0 as i32 as usize])
        .width as i32;
    let mut neg: i32 = 0;
    let mut temp: i32 = 0;
    if digits < 0 as i32 {
        if n == 0 {
            digits = 1 as i32;
        } else {
            digits = 0 as i32;
            temp = n;
            while temp != 0 {
                temp /= 10 as i32;
                digits += 1;
            }
        }
    }
    neg = (n < 0 as i32) as i32;
    if neg != 0 {
        n = -n;
    }
    if n == 1994 as i32 {
        return 0 as i32;
    }
    loop {
        let fresh0 = digits;
        digits = digits - 1;
        if !(fresh0 != 0) {
            break;
        }
        x -= fontwidth;
        V_DrawPatch(x, y, num[(n % 10 as i32) as usize]);
        n /= 10 as i32;
    }
    if neg != 0 {
        x -= 8 as i32;
        V_DrawPatch(x, y, wiminus);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawPercent(
    mut x: i32,
    mut y: i32,
    mut p_0: i32,
) {
    if p_0 < 0 as i32 {
        return;
    }
    V_DrawPatch(x, y, percent);
    WI_drawNum(x, y, p_0, -(1 as i32));
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawTime(
    mut x: i32,
    mut y: i32,
    mut t: i32,
) {
    let mut div: i32 = 0;
    let mut n: i32 = 0;
    if t < 0 as i32 {
        return;
    }
    if t <= 61 as i32 * 59 as i32 {
        div = 1 as i32;
        loop {
            n = t / div % 60 as i32;
            x = WI_drawNum(x, y, n, 2 as i32)
                - (*colon).width as i32;
            div *= 60 as i32;
            if div == 60 as i32 || t / div != 0 {
                V_DrawPatch(x, y, colon);
            }
            if !(t / div != 0) {
                break;
            }
        }
    } else {
        V_DrawPatch(x - (*sucks).width as i32, y, sucks);
    };
}
pub unsafe fn WI_End() {
    #[export_name = "WI_unloadData"]
    pub unsafe extern "C" fn WI_unloadData_0() {
        WI_loadUnloadData(
            Some(
                WI_unloadCallback
                    as unsafe fn(*mut ::core::ffi::c_char, *mut *mut patch_t) -> (),
            ),
        );
    }
    WI_unloadData_0();
}
#[no_mangle]
pub unsafe extern "C" fn WI_initNoState() {
    state = NoState;
    acceleratestage = 0 as i32;
    cnt = 10 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateNoState() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 {
        G_WorldDone();
    }
}
static mut snl_pointeron: bool = false;
#[no_mangle]
pub unsafe extern "C" fn WI_initShowNextLoc() {
    state = ShowNextLoc;
    acceleratestage = 0 as i32;
    cnt = SHOWNEXTLOCDELAY * TICRATE;
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateShowNextLoc() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 || acceleratestage != 0 {
        WI_initNoState();
    } else {
        snl_pointeron = (cnt & 31 as i32) < 20 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawShowNextLoc() {
    let mut i: i32 = 0;
    let mut last: i32 = 0;
    WI_slamBackground();
    WI_drawAnimatedBack();
    if gamemode as u32
        != commercial as i32 as u32
    {
        if (*wbs).epsd > 2 as i32 {
            WI_drawEL();
            return;
        }
        last = if (*wbs).last == 8 as i32 {
            (*wbs).next - 1 as i32
        } else {
            (*wbs).last
        };
        i = 0 as i32;
        while i <= last {
            WI_drawOnLnode(i, &raw mut splat as *mut *mut patch_t);
            i += 1;
        }
        if (*wbs).didsecret {
            WI_drawOnLnode(8 as i32, &raw mut splat as *mut *mut patch_t);
        }
        if snl_pointeron {
            WI_drawOnLnode((*wbs).next, &raw mut yah as *mut *mut patch_t);
        }
    }
    if gamemode as u32
        != commercial as i32 as u32
        || (*wbs).next != 30 as i32
    {
        WI_drawEL();
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawNoState() {
    snl_pointeron = true;
    WI_drawShowNextLoc();
}
#[no_mangle]
pub unsafe extern "C" fn WI_fragSum(
    mut playernum: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut frags_0: i32 = 0 as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 && i != playernum {
            frags_0 += (*plrs.offset(playernum as isize)).frags[i as usize];
        }
        i += 1;
    }
    frags_0 -= (*plrs.offset(playernum as isize)).frags[playernum as usize];
    return frags_0;
}
static mut dm_state: i32 = 0;
static mut dm_frags: [[i32; 4]; 4] = [[0; 4]; 4];
static mut dm_totals: [i32; 4] = [0; 4];
#[no_mangle]
pub unsafe extern "C" fn WI_initDeathmatchStats() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    state = StatCount;
    acceleratestage = 0 as i32;
    dm_state = 1 as i32;
    cnt_pause = TICRATE;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            j = 0 as i32;
            while j < MAXPLAYERS {
                if playeringame[j as usize] != 0 {
                    dm_frags[i as usize][j as usize] = 0 as i32;
                }
                j += 1;
            }
            dm_totals[i as usize] = 0 as i32;
        }
        i += 1;
    }
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateDeathmatchStats() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut stillticking: bool = false;
    WI_updateAnimatedBack();
    if acceleratestage != 0 && dm_state != 4 as i32 {
        acceleratestage = 0 as i32;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if playeringame[i as usize] != 0 {
                j = 0 as i32;
                while j < MAXPLAYERS {
                    if playeringame[j as usize] != 0 {
                        dm_frags[i as usize][j as usize] = (*plrs.offset(i as isize))
                            .frags[j as usize];
                    }
                    j += 1;
                }
                dm_totals[i as usize] = WI_fragSum(i);
            }
            i += 1;
        }
        S_StartSound(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            sfx_barexp as i32,
        );
        dm_state = 4 as i32;
    }
    if dm_state == 2 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if playeringame[i as usize] != 0 {
                j = 0 as i32;
                while j < MAXPLAYERS {
                    if playeringame[j as usize] != 0
                        && dm_frags[i as usize][j as usize]
                            != (*plrs.offset(i as isize)).frags[j as usize]
                    {
                        if (*plrs.offset(i as isize)).frags[j as usize]
                            < 0 as i32
                        {
                            dm_frags[i as usize][j as usize] -= 1;
                        } else {
                            dm_frags[i as usize][j as usize] += 1;
                        }
                        if dm_frags[i as usize][j as usize] > 99 as i32 {
                            dm_frags[i as usize][j as usize] = 99 as i32;
                        }
                        if dm_frags[i as usize][j as usize] < -(99 as i32)
                        {
                            dm_frags[i as usize][j as usize] = -(99
                                as i32);
                        }
                        stillticking = true;
                    }
                    j += 1;
                }
                dm_totals[i as usize] = WI_fragSum(i);
                if dm_totals[i as usize] > 99 as i32 {
                    dm_totals[i as usize] = 99 as i32;
                }
                if dm_totals[i as usize] < -(99 as i32) {
                    dm_totals[i as usize] = -(99 as i32);
                }
            }
            i += 1;
        }
        if !stillticking {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            dm_state += 1;
        }
    } else if dm_state == 4 as i32 {
        if acceleratestage != 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_slop as i32,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if dm_state & 1 as i32 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            dm_state += 1;
            cnt_pause = TICRATE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawDeathmatchStats() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();
    V_DrawPatch(
        DM_TOTALSX - (*total).width as i32 / 2 as i32,
        DM_MATRIXY - WI_SPACINGY + 10 as i32,
        total,
    );
    V_DrawPatch(DM_KILLERSX, DM_KILLERSY, killers);
    V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, victims);
    x = DM_MATRIXX + DM_SPACINGX;
    y = DM_MATRIXY;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            V_DrawPatch(
                x
                    - (*p[i as usize]).width as i32
                        / 2 as i32,
                DM_MATRIXY - WI_SPACINGY,
                p[i as usize],
            );
            V_DrawPatch(
                DM_MATRIXX
                    - (*p[i as usize]).width as i32
                        / 2 as i32,
                y,
                p[i as usize],
            );
            if i == me {
                V_DrawPatch(
                    x
                        - (*p[i as usize]).width as i32
                            / 2 as i32,
                    DM_MATRIXY - WI_SPACINGY,
                    bstar,
                );
                V_DrawPatch(
                    DM_MATRIXX
                        - (*p[i as usize]).width as i32
                            / 2 as i32,
                    y,
                    star,
                );
            }
        }
        x += DM_SPACINGX;
        y += WI_SPACINGY;
        i += 1;
    }
    y = DM_MATRIXY + 10 as i32;
    w = (*num[0 as i32 as usize]).width as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        x = DM_MATRIXX + DM_SPACINGX;
        if playeringame[i as usize] != 0 {
            j = 0 as i32;
            while j < MAXPLAYERS {
                if playeringame[j as usize] != 0 {
                    WI_drawNum(
                        x + w,
                        y,
                        dm_frags[i as usize][j as usize],
                        2 as i32,
                    );
                }
                x += DM_SPACINGX;
                j += 1;
            }
            WI_drawNum(
                DM_TOTALSX + w,
                y,
                dm_totals[i as usize],
                2 as i32,
            );
        }
        y += WI_SPACINGY;
        i += 1;
    }
}
static mut cnt_frags: [i32; 4] = [0; 4];
static mut dofrags: i32 = 0;
static mut ng_state: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn WI_initNetgameStats() {
    let mut i: i32 = 0;
    state = StatCount;
    acceleratestage = 0 as i32;
    ng_state = 1 as i32;
    cnt_pause = TICRATE;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            cnt_frags[i as usize] = 0 as i32;
            cnt_secret[i as usize] = cnt_frags[i as usize];
            cnt_items[i as usize] = cnt_secret[i as usize];
            cnt_kills[i as usize] = cnt_items[i as usize];
            dofrags += WI_fragSum(i);
        }
        i += 1;
    }
    dofrags = (dofrags != 0) as i32;
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateNetgameStats() {
    let mut i: i32 = 0;
    let mut fsum: i32 = 0;
    let mut stillticking: bool = false;
    WI_updateAnimatedBack();
    if acceleratestage != 0 && ng_state != 10 as i32 {
        acceleratestage = 0 as i32;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_kills[i as usize] = (*plrs.offset(i as isize)).skills
                    * 100 as i32 / (*wbs).maxkills;
                cnt_items[i as usize] = (*plrs.offset(i as isize)).sitems
                    * 100 as i32 / (*wbs).maxitems;
                cnt_secret[i as usize] = (*plrs.offset(i as isize)).ssecret
                    * 100 as i32 / (*wbs).maxsecret;
                if dofrags != 0 {
                    cnt_frags[i as usize] = WI_fragSum(i);
                }
            }
            i += 1;
        }
        S_StartSound(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            sfx_barexp as i32,
        );
        ng_state = 10 as i32;
    }
    if ng_state == 2 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_kills[i as usize] += 2 as i32;
                if cnt_kills[i as usize]
                    >= (*plrs.offset(i as isize)).skills * 100 as i32
                        / (*wbs).maxkills
                {
                    cnt_kills[i as usize] = (*plrs.offset(i as isize)).skills
                        * 100 as i32 / (*wbs).maxkills;
                } else {
                    stillticking = true;
                }
            }
            i += 1;
        }
        if !stillticking {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            ng_state += 1;
        }
    } else if ng_state == 4 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_items[i as usize] += 2 as i32;
                if cnt_items[i as usize]
                    >= (*plrs.offset(i as isize)).sitems * 100 as i32
                        / (*wbs).maxitems
                {
                    cnt_items[i as usize] = (*plrs.offset(i as isize)).sitems
                        * 100 as i32 / (*wbs).maxitems;
                } else {
                    stillticking = true;
                }
            }
            i += 1;
        }
        if !stillticking {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            ng_state += 1;
        }
    } else if ng_state == 6 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_secret[i as usize] += 2 as i32;
                if cnt_secret[i as usize]
                    >= (*plrs.offset(i as isize)).ssecret * 100 as i32
                        / (*wbs).maxsecret
                {
                    cnt_secret[i as usize] = (*plrs.offset(i as isize)).ssecret
                        * 100 as i32 / (*wbs).maxsecret;
                } else {
                    stillticking = true;
                }
            }
            i += 1;
        }
        if !stillticking {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            ng_state
                += 1 as i32
                    + 2 as i32 * (dofrags == 0) as i32;
        }
    } else if ng_state == 8 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_frags[i as usize] += 1 as i32;
                fsum = WI_fragSum(i);
                if cnt_frags[i as usize] >= fsum {
                    cnt_frags[i as usize] = fsum;
                } else {
                    stillticking = true;
                }
            }
            i += 1;
        }
        if !stillticking {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pldeth as i32,
            );
            ng_state += 1;
        }
    } else if ng_state == 10 as i32 {
        if acceleratestage != 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_sgcock as i32,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if ng_state & 1 as i32 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            ng_state += 1;
            cnt_pause = TICRATE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawNetgameStats() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut pwidth: i32 = (*percent).width as i32;
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();
    V_DrawPatch(
        32 as i32
            + (*star).width as i32 / 2 as i32
            + 32 as i32 * (dofrags == 0) as i32
            + NG_SPACINGX - (*kills).width as i32,
        NG_STATSY,
        kills,
    );
    V_DrawPatch(
        32 as i32
            + (*star).width as i32 / 2 as i32
            + 32 as i32 * (dofrags == 0) as i32
            + 2 as i32 * NG_SPACINGX
            - (*items).width as i32,
        NG_STATSY,
        items,
    );
    V_DrawPatch(
        32 as i32
            + (*star).width as i32 / 2 as i32
            + 32 as i32 * (dofrags == 0) as i32
            + 3 as i32 * NG_SPACINGX
            - (*secret).width as i32,
        NG_STATSY,
        secret,
    );
    if dofrags != 0 {
        V_DrawPatch(
            32 as i32
                + (*star).width as i32 / 2 as i32
                + 32 as i32 * (dofrags == 0) as i32
                + 4 as i32 * NG_SPACINGX
                - (*frags).width as i32,
            NG_STATSY,
            frags,
        );
    }
    y = NG_STATSY + (*kills).height as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            x = 32 as i32
                + (*star).width as i32 / 2 as i32
                + 32 as i32 * (dofrags == 0) as i32;
            V_DrawPatch(
                x - (*p[i as usize]).width as i32,
                y,
                p[i as usize],
            );
            if i == me {
                V_DrawPatch(x - (*p[i as usize]).width as i32, y, star);
            }
            x += NG_SPACINGX;
            WI_drawPercent(
                x - pwidth,
                y + 10 as i32,
                cnt_kills[i as usize],
            );
            x += NG_SPACINGX;
            WI_drawPercent(
                x - pwidth,
                y + 10 as i32,
                cnt_items[i as usize],
            );
            x += NG_SPACINGX;
            WI_drawPercent(
                x - pwidth,
                y + 10 as i32,
                cnt_secret[i as usize],
            );
            x += NG_SPACINGX;
            if dofrags != 0 {
                WI_drawNum(
                    x,
                    y + 10 as i32,
                    cnt_frags[i as usize],
                    -(1 as i32),
                );
            }
            y += WI_SPACINGY;
        }
        i += 1;
    }
}
static mut sp_state: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn WI_initStats() {
    state = StatCount;
    acceleratestage = 0 as i32;
    sp_state = 1 as i32;
    cnt_secret[0 as i32 as usize] = -(1 as i32);
    cnt_items[0 as i32 as usize] = cnt_secret[0 as i32
        as usize];
    cnt_kills[0 as i32 as usize] = cnt_items[0 as i32
        as usize];
    cnt_par = -(1 as i32);
    cnt_time = cnt_par;
    cnt_pause = TICRATE;
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateStats() {
    WI_updateAnimatedBack();
    if acceleratestage != 0 && sp_state != 10 as i32 {
        acceleratestage = 0 as i32;
        cnt_kills[0 as i32 as usize] = (*plrs.offset(me as isize)).skills
            * 100 as i32 / (*wbs).maxkills;
        cnt_items[0 as i32 as usize] = (*plrs.offset(me as isize)).sitems
            * 100 as i32 / (*wbs).maxitems;
        cnt_secret[0 as i32 as usize] = (*plrs.offset(me as isize))
            .ssecret * 100 as i32 / (*wbs).maxsecret;
        cnt_time = (*plrs.offset(me as isize)).stime / TICRATE;
        cnt_par = (*wbs).partime / TICRATE;
        S_StartSound(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            sfx_barexp as i32,
        );
        sp_state = 10 as i32;
    }
    if sp_state == 2 as i32 {
        cnt_kills[0 as i32 as usize] += 2 as i32;
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        if cnt_kills[0 as i32 as usize]
            >= (*plrs.offset(me as isize)).skills * 100 as i32
                / (*wbs).maxkills
        {
            cnt_kills[0 as i32 as usize] = (*plrs.offset(me as isize))
                .skills * 100 as i32 / (*wbs).maxkills;
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            sp_state += 1;
        }
    } else if sp_state == 4 as i32 {
        cnt_items[0 as i32 as usize] += 2 as i32;
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        if cnt_items[0 as i32 as usize]
            >= (*plrs.offset(me as isize)).sitems * 100 as i32
                / (*wbs).maxitems
        {
            cnt_items[0 as i32 as usize] = (*plrs.offset(me as isize))
                .sitems * 100 as i32 / (*wbs).maxitems;
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            sp_state += 1;
        }
    } else if sp_state == 6 as i32 {
        cnt_secret[0 as i32 as usize] += 2 as i32;
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        if cnt_secret[0 as i32 as usize]
            >= (*plrs.offset(me as isize)).ssecret * 100 as i32
                / (*wbs).maxsecret
        {
            cnt_secret[0 as i32 as usize] = (*plrs.offset(me as isize))
                .ssecret * 100 as i32 / (*wbs).maxsecret;
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            sp_state += 1;
        }
    } else if sp_state == 8 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        cnt_time += 3 as i32;
        if cnt_time >= (*plrs.offset(me as isize)).stime / TICRATE {
            cnt_time = (*plrs.offset(me as isize)).stime / TICRATE;
        }
        cnt_par += 3 as i32;
        if cnt_par >= (*wbs).partime / TICRATE {
            cnt_par = (*wbs).partime / TICRATE;
            if cnt_time >= (*plrs.offset(me as isize)).stime / TICRATE {
                S_StartSound(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    sfx_barexp as i32,
                );
                sp_state += 1;
            }
        }
    } else if sp_state == 10 as i32 {
        if acceleratestage != 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_sgcock as i32,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if sp_state & 1 as i32 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            sp_state += 1;
            cnt_pause = TICRATE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawStats() {
    let mut lh: i32 = 0;
    lh = 3 as i32
        * (*num[0 as i32 as usize]).height as i32
        / 2 as i32;
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();
    V_DrawPatch(SP_STATSX, SP_STATSY, kills);
    WI_drawPercent(
        SCREENWIDTH - SP_STATSX,
        SP_STATSY,
        cnt_kills[0 as i32 as usize],
    );
    V_DrawPatch(SP_STATSX, SP_STATSY + lh, items);
    WI_drawPercent(
        SCREENWIDTH - SP_STATSX,
        SP_STATSY + lh,
        cnt_items[0 as i32 as usize],
    );
    V_DrawPatch(SP_STATSX, SP_STATSY + 2 as i32 * lh, sp_secret);
    WI_drawPercent(
        SCREENWIDTH - SP_STATSX,
        SP_STATSY + 2 as i32 * lh,
        cnt_secret[0 as i32 as usize],
    );
    V_DrawPatch(SP_TIMEX, SP_TIMEY, timepatch);
    WI_drawTime(SCREENWIDTH / 2 as i32 - SP_TIMEX, SP_TIMEY, cnt_time);
    if (*wbs).epsd < 3 as i32 {
        V_DrawPatch(SCREENWIDTH / 2 as i32 + SP_TIMEX, SP_TIMEY, par);
        WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_checkForAccelerate() {
    let mut i: i32 = 0;
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    i = 0 as i32;
    player = &raw mut players as *mut player_t;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            if (*player).cmd.buttons as i32
                & BT_ATTACK as i32 != 0
            {
                if (*player).attackdown == 0 {
                    acceleratestage = 1 as i32;
                }
                (*player).attackdown = true_0;
            } else {
                (*player).attackdown = false_0;
            }
            if (*player).cmd.buttons as i32 & BT_USE as i32
                != 0
            {
                if (*player).usedown == 0 {
                    acceleratestage = 1 as i32;
                }
                (*player).usedown = true_0;
            } else {
                (*player).usedown = false_0;
            }
        }
        i += 1;
        player = player.offset(1);
    }
}
pub unsafe fn WI_Ticker() {
    bcnt += 1;
    if bcnt == 1 as i32 {
        if gamemode as u32
            == commercial as i32 as u32
        {
            S_ChangeMusic(mus_dm2int as i32, true_0);
        } else {
            S_ChangeMusic(mus_inter as i32, true_0);
        }
    }
    WI_checkForAccelerate();
    match state as i32 {
        0 => {
            if deathmatch != 0 {
                WI_updateDeathmatchStats();
            } else if netgame {
                WI_updateNetgameStats();
            } else {
                WI_updateStats();
            }
        }
        1 => {
            WI_updateShowNextLoc();
        }
        -1 => {
            WI_updateNoState();
        }
        _ => {}
    };
}
unsafe fn WI_loadUnloadData(mut callback: load_callback_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut name: [::core::ffi::c_char; 9] = [0; 9];
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        i = 0 as i32;
        while i < NUMCMAPS {
            snprintf(
                &raw mut name as *mut ::core::ffi::c_char,
                9 as size_t,
                b"CWILV%2.2d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            callback
                .expect(
                    "non-null function pointer",
                )(
                &raw mut name as *mut ::core::ffi::c_char,
                lnames.offset(i as isize) as *mut *mut patch_t,
            );
            i += 1;
        }
    } else {
        i = 0 as i32;
        while i < NUMMAPS {
            snprintf(
                &raw mut name as *mut ::core::ffi::c_char,
                9 as size_t,
                b"WILV%d%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*wbs).epsd,
                i,
            );
            callback
                .expect(
                    "non-null function pointer",
                )(
                &raw mut name as *mut ::core::ffi::c_char,
                lnames.offset(i as isize) as *mut *mut patch_t,
            );
            i += 1;
        }
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WIURH0\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (&raw mut yah as *mut *mut patch_t).offset(0 as i32 as isize)
                as *mut *mut patch_t,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WIURH1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (&raw mut yah as *mut *mut patch_t).offset(1 as i32 as isize)
                as *mut *mut patch_t,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WISPLAT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (&raw mut splat as *mut *mut patch_t)
                .offset(0 as i32 as isize) as *mut *mut patch_t,
        );
        if (*wbs).epsd < 3 as i32 {
            j = 0 as i32;
            while j < NUMANIMS[(*wbs).epsd as usize] {
                a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
                    .offset(j as isize) as *mut anim_t;
                i = 0 as i32;
                while i < (*a).nanims {
                    if (*wbs).epsd != 1 as i32
                        || j != 8 as i32
                    {
                        snprintf(
                            &raw mut name as *mut ::core::ffi::c_char,
                            9 as size_t,
                            b"WIA%d%.2d%.2d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*wbs).epsd,
                            j,
                            i,
                        );
                        callback
                            .expect(
                                "non-null function pointer",
                            )(
                            &raw mut name as *mut ::core::ffi::c_char,
                            (&raw mut (*a).p as *mut *mut patch_t).offset(i as isize)
                                as *mut *mut patch_t,
                        );
                    } else {
                        (*a).p[i as usize] = (*anims[1 as i32 as usize]
                            .offset(4 as i32 as isize))
                            .p[i as usize];
                    }
                    i += 1;
                }
                j += 1;
            }
        }
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIMINUS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut wiminus,
    );
    i = 0 as i32;
    while i < 10 as i32 {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            9 as size_t,
            b"WINUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut name as *mut ::core::ffi::c_char,
            (&raw mut num as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        i += 1;
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIPCNT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut percent,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut finished,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIENTER\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut entering,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIOSTK\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut kills,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIOSTS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut secret,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WISCRT2\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut sp_secret,
    );
    if W_CheckNumForName("WIOBJ",
    ) >= 0 as i32
    {
        if netgame && deathmatch == 0 {
            callback
                .expect(
                    "non-null function pointer",
                )(
                b"WIOBJ\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut items,
            );
        } else {
            callback
                .expect(
                    "non-null function pointer",
                )(
                b"WIOSTI\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut items,
            );
        }
    } else {
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WIOSTI\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut items,
        );
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIFRGS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut frags,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WICOLON\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut colon,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WITIME\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut timepatch,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WISUCKS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut sucks,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIPAR\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut par,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIKILRS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut killers,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIVCTMS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut victims,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIMSTT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut total,
    );
    i = 0 as i32;
    while i < MAXPLAYERS {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STPB%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut name as *mut ::core::ffi::c_char,
            (&raw mut p as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            9 as size_t,
            b"WIBP%d\0" as *const u8 as *const ::core::ffi::c_char,
            i + 1 as i32,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut name as *mut ::core::ffi::c_char,
            (&raw mut bp as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        i += 1;
    }
    if gamemode as u32
        == commercial as i32 as u32
    {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            b"INTERPIC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
        );
    } else if gamemode as u32
        == retail as i32 as u32
        && (*wbs).epsd == 3 as i32
    {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            b"INTERPIC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
        );
    } else {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
            b"WIMAP%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*wbs).epsd,
        );
    }
    callback
        .expect(
            "non-null function pointer",
        )(&raw mut name as *mut ::core::ffi::c_char, &raw mut background);
}
unsafe fn WI_loadCallback(
    mut name: *mut ::core::ffi::c_char,
    mut variable: *mut *mut patch_t,
) {
    *variable = W_CacheLumpName(
        &wad_name8_to_string(name),
        PU_STATIC as i32,
    ) as *mut patch_t;
}
#[no_mangle]
pub unsafe extern "C" fn WI_loadData() {
    if gamemode as u32
        == commercial as i32 as u32
    {
        NUMCMAPS = 32 as i32;
        lnames = Z_Malloc(
            (::core::mem::size_of::<*mut patch_t>() as usize)
                .wrapping_mul(NUMCMAPS as usize) as i32,
            PU_STATIC as i32,
            NULL,
        ) as *mut *mut patch_t;
    } else {
        lnames = Z_Malloc(
            (::core::mem::size_of::<*mut patch_t>() as usize)
                .wrapping_mul(NUMMAPS as usize) as i32,
            PU_STATIC as i32,
            NULL,
        ) as *mut *mut patch_t;
    }
    WI_loadUnloadData(
        Some(
            WI_loadCallback
                as unsafe fn(*mut ::core::ffi::c_char, *mut *mut patch_t) -> (),
        ),
    );
    star = W_CacheLumpName("STFST01",
        PU_STATIC as i32,
    ) as *mut patch_t;
    bstar = W_CacheLumpName("STFDEAD0",
        PU_STATIC as i32,
    ) as *mut patch_t;
}
unsafe fn WI_unloadCallback(
    mut name: *mut ::core::ffi::c_char,
    mut variable: *mut *mut patch_t,
) {
    W_ReleaseLumpName(&wad_name8_to_string(name));
    *variable = ::core::ptr::null_mut::<patch_t>();
}
pub unsafe fn WI_Drawer() {
    match state as i32 {
        0 => {
            if deathmatch != 0 {
                WI_drawDeathmatchStats();
            } else if netgame {
                WI_drawNetgameStats();
            } else {
                WI_drawStats();
            }
        }
        1 => {
            WI_drawShowNextLoc();
        }
        -1 => {
            WI_drawNoState();
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn WI_initVariables(mut wbstartstruct: *mut wbstartstruct_t) {
    wbs = wbstartstruct;
    acceleratestage = 0 as i32;
    bcnt = 0 as i32;
    cnt = bcnt;
    firstrefresh = 1 as i32;
    me = (*wbs).pnum;
    plrs = &raw mut (*wbs).plyr as *mut wbplayerstruct_t;
    if (*wbs).maxkills == 0 {
        (*wbs).maxkills = 1 as i32;
    }
    if (*wbs).maxitems == 0 {
        (*wbs).maxitems = 1 as i32;
    }
    if (*wbs).maxsecret == 0 {
        (*wbs).maxsecret = 1 as i32;
    }
    if gamemode as u32
        != retail as i32 as u32
    {
        if (*wbs).epsd > 2 as i32 {
            (*wbs).epsd -= 3 as i32;
        }
    }
}
pub unsafe fn WI_Start(mut wbstartstruct: *mut wbstartstruct_t) {
    WI_initVariables(wbstartstruct);
    WI_loadData();
    if deathmatch != 0 {
        WI_initDeathmatchStats();
    } else if netgame {
        WI_initNetgameStats();
    } else {
        WI_initStats();
    };
}
unsafe extern "C" fn run_static_initializers() {
    NUMANIMS = [
        (::core::mem::size_of::<[anim_t; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<anim_t>() as usize)
            as i32,
        (::core::mem::size_of::<[anim_t; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<anim_t>() as usize)
            as i32,
        (::core::mem::size_of::<[anim_t; 6]>() as usize)
            .wrapping_div(::core::mem::size_of::<anim_t>() as usize)
            as i32,
        0,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
