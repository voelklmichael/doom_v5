use crate::src::d_event::event_t;
use crate::src::d_event::{ev_keydown, ev_keyup};
use crate::src::d_player::player_t;
use crate::src::d_player::{pw_allmap, pw_invisibility};
use crate::src::doomdef::false_0;
use crate::src::doomdef::true_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::game_state::game_state;
use crate::src::hu_lib::patch_t;
use crate::src::i_system::{fprintf, stderr};
use crate::src::i_video::I_VideoBuffer;
use crate::src::m_cheat::cheatseq_t;
use crate::src::m_cheat::cht_CheckCheat;
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FixedDiv;
use crate::src::m_fixed::FixedMul;
use crate::src::m_fixed::FRACBITS;
use crate::src::m_fixed::FRACUNIT;
use crate::src::m_fixed::INT_MAX;
use crate::src::m_misc::M_snprintf;
use crate::src::p_maputl::MAPBLOCKUNITS;
use crate::src::p_mobj::mobj_t;
use crate::src::p_setup::bmaporgx;
use crate::src::p_setup::bmaporgy;
use crate::src::p_setup::lines;
use crate::src::p_setup::numlines;
use crate::src::p_setup::numsectors;
use crate::src::p_setup::numvertexes;
use crate::src::p_setup::sectors;
use crate::src::p_setup::vertexes;
use crate::src::p_spec::ML_MAPPED;
use crate::src::p_spec::ML_SECRET;
use crate::src::st_stuff::ST_Responder;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::tables::angle_t;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::v_video::V_DrawPatch;
use crate::src::v_video::V_MarkRect;
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName, W_ReleaseLumpName};
use crate::src::z_zone::PU_STATIC;
use libc::memset;
use libc::snprintf;

pub struct AmMapState {
    pub cheating: i32,
    pub grid: i32,
    pub leveljuststarted: i32,
    pub automapactive: bool,
    pub f_x: i32,
    pub f_y: i32,
    pub f_w: i32,
    pub f_h: i32,
    pub lightlev: i32,
    pub fb: *mut byte,
    pub amclock: i32,
    pub m_paninc: mpoint_t,
    pub mtof_zoommul: fixed_t,
    pub ftom_zoommul: fixed_t,
    pub m_y: fixed_t,
    pub m_x: fixed_t,
    pub m_x2: fixed_t,
    pub m_y2: fixed_t,
    pub m_w: fixed_t,
    pub m_h: fixed_t,
    pub min_x: fixed_t,
    pub min_y: fixed_t,
    pub max_x: fixed_t,
    pub max_y: fixed_t,
    pub max_w: fixed_t,
    pub max_h: fixed_t,
    pub min_w: fixed_t,
    pub min_h: fixed_t,
    pub min_scale_mtof: fixed_t,
    pub max_scale_mtof: fixed_t,
    pub old_m_h: fixed_t,
    pub old_m_w: fixed_t,
    pub old_m_y: fixed_t,
    pub old_m_x: fixed_t,
    pub f_oldloc: mpoint_t,
    pub scale_mtof: fixed_t,
    pub scale_ftom: fixed_t,
    pub plr: *mut player_t,
    pub marknums: [*mut patch_t; 10],
    pub markpoints: [mpoint_t; 10],
    pub markpointnum: i32,
    pub followplayer: i32,
    pub cheat_amap: cheatseq_t,
    pub stopped: bool,
    pub am_start_lastlevel: i32,
    pub am_start_lastepisode: i32,
    pub am_responder_bigstate: i32,
    pub am_responder_buffer: [::core::ffi::c_char; 20],
    pub am_drawfline_fuck: i32,
    pub am_updatelightlev_nexttic: i32,
    pub am_updatelightlev_litelevelscnt: i32,
}

impl AmMapState {
    pub const fn new() -> Self {
        AmMapState {
            cheating: 0,
            grid: 0,
            leveljuststarted: 1,
            automapactive: false,
            f_x: 0,
            f_y: 0,
            f_w: 0,
            f_h: 0,
            lightlev: 0,
            fb: ::core::ptr::null::<byte>() as *mut byte,
            amclock: 0,
            m_paninc: mpoint_t { x: 0, y: 0 },
            mtof_zoommul: 0,
            ftom_zoommul: 0,
            m_y: 0,
            m_x: 0,
            m_x2: 0,
            m_y2: 0,
            m_w: 0,
            m_h: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            max_w: 0,
            max_h: 0,
            min_w: 0,
            min_h: 0,
            min_scale_mtof: 0,
            max_scale_mtof: 0,
            old_m_h: 0,
            old_m_w: 0,
            old_m_y: 0,
            old_m_x: 0,
            f_oldloc: mpoint_t { x: 0, y: 0 },
            scale_mtof: INITSCALEMTOF as fixed_t,
            scale_ftom: 0,
            plr: ::core::ptr::null::<player_t>() as *mut player_t,
            marknums: [::core::ptr::null::<patch_t>() as *mut patch_t; 10],
            markpoints: [mpoint_t { x: 0, y: 0 }; 10],
            markpointnum: 0,
            followplayer: 1,
            cheat_amap: cheatseq_t {
        sequence: [0; 25],
        sequence_len: 0,
        parameter_chars: 0,
        chars_read: 0,
        param_chars_read: 0,
        parameter_buf: [0; 5],
    },
            stopped: true,
            am_start_lastlevel: -1,
            am_start_lastepisode: -1,
            am_responder_bigstate: 0,
            am_responder_buffer: [0; 20],
            am_drawfline_fuck: 0,
            am_updatelightlev_nexttic: 0,
            am_updatelightlev_litelevelscnt: 0,
        }
    }
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpoint_t {
    pub x: fixed_t,
    pub y: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mline_t {
    pub a: mpoint_t,
    pub b: mpoint_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fline_t {
    pub a: fpoint_t,
    pub b: fpoint_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fpoint_t {
    pub x: i32,
    pub y: i32,
}
pub const RIGHT: C2RustUnnamed_1 = 2;
pub const LEFT: C2RustUnnamed_1 = 1;
pub const BOTTOM: C2RustUnnamed_1 = 4;
pub const TOP: C2RustUnnamed_1 = 8;
pub type C2RustUnnamed_1 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct islope_t {
    pub slp: fixed_t,
    pub islp: fixed_t,
}
pub const ML_DONTDRAW: i32 = 128;
pub const AM_MSGHEADER: i32 = (('a' as i32) << 24 as i32) + (('m' as i32) << 16 as i32);
pub const AM_MSGENTERED: i32 = AM_MSGHEADER | ('e' as i32) << 8 as i32;
pub const AM_MSGEXITED: i32 = AM_MSGHEADER | ('x' as i32) << 8 as i32;
pub const REDS: i32 = 256 - 5 as i32 * 16 as i32;
pub const REDRANGE: i32 = 16;
pub const GREENS: i32 = 7 * 16 as i32;
pub const GREENRANGE: i32 = 16;
pub const GRAYS: i32 = 6 * 16 as i32;
pub const GRAYSRANGE: i32 = 16;
pub const BROWNS: i32 = 4 * 16 as i32;
pub const YELLOWS: i32 = 256 - 32 as i32 + 7 as i32;
pub const BLACK: i32 = 0;
pub const WHITE: i32 = 256 - 47 as i32;
pub const BACKGROUND: i32 = BLACK;
pub const WALLCOLORS: i32 = REDS;
pub const WALLRANGE: i32 = REDRANGE;
pub const TSWALLCOLORS: i32 = GRAYS;
pub const FDWALLCOLORS: i32 = BROWNS;
pub const CDWALLCOLORS: i32 = YELLOWS;
pub const THINGCOLORS: i32 = GREENS;
pub const THINGRANGE: i32 = GREENRANGE;
pub const SECRETWALLCOLORS: i32 = WALLCOLORS;
pub const GRIDCOLORS: i32 = GRAYS + GRAYSRANGE / 2 as i32;
pub const XHAIRCOLORS: i32 = GRAYS;
pub const AM_NUMMARKPOINTS: i32 = 10;
pub const INITSCALEMTOF: f64 = 0.2f64 * FRACUNIT as f64;
pub const M_ZOOMIN: i32 = (1.02f64 * FRACUNIT as f64) as i32;
pub const M_ZOOMOUT: i32 = (FRACUNIT as f64 / 1.02f64) as i32;
pub const LINE_NEVERSEE: i32 = ML_DONTDRAW;
pub const R_0: i32 = 8 * 16 as i32 * FRACUNIT / 7 as i32;
#[no_mangle]
pub static player_arrow: [mline_t; 7] = [
    mline_t {
        a: mpoint_t {
            x: -R_0 + R_0 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: R_0,
            y: 0 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_0,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: R_0 - R_0 / 2 as fixed_t,
            y: R_0 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_0,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: R_0 - R_0 / 2 as fixed_t,
            y: -R_0 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_0 + R_0 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_0 - R_0 / 8 as fixed_t,
            y: R_0 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_0 + R_0 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_0 - R_0 / 8 as fixed_t,
            y: -R_0 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_0 + 3 as fixed_t * R_0 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_0 + R_0 / 8 as fixed_t,
            y: R_0 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_0 + 3 as fixed_t * R_0 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_0 + R_0 / 8 as fixed_t,
            y: -R_0 / 4 as fixed_t,
        },
    },
];
pub const R_1: i32 = 8 * 16 as i32 * FRACUNIT / 7 as i32;
#[no_mangle]
pub static cheat_player_arrow: [mline_t; 16] = [
    mline_t {
        a: mpoint_t {
            x: -R_1 + R_1 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: R_1,
            y: 0 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_1,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: R_1 - R_1 / 2 as fixed_t,
            y: R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_1,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: R_1 - R_1 / 2 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 + R_1 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 - R_1 / 8 as fixed_t,
            y: R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 + R_1 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 - R_1 / 8 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 + 3 as fixed_t * R_1 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 + R_1 / 8 as fixed_t,
            y: R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 + 3 as fixed_t * R_1 / 8 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 + R_1 / 8 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 / 2 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 / 2 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 / 2 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 / 2 as fixed_t + R_1 / 6 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 / 2 as fixed_t + R_1 / 6 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 / 2 as fixed_t + R_1 / 6 as fixed_t,
            y: R_1 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 / 6 as fixed_t,
            y: 0 as fixed_t,
        },
        b: mpoint_t {
            x: -R_1 / 6 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_1 / 6 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
        b: mpoint_t {
            x: 0 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: 0 as fixed_t,
            y: -R_1 / 6 as fixed_t,
        },
        b: mpoint_t {
            x: 0 as fixed_t,
            y: R_1 / 4 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_1 / 6 as fixed_t,
            y: R_1 / 4 as fixed_t,
        },
        b: mpoint_t {
            x: R_1 / 6 as fixed_t,
            y: -R_1 / 7 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_1 / 6 as fixed_t,
            y: -R_1 / 7 as fixed_t,
        },
        b: mpoint_t {
            x: R_1 / 6 as fixed_t + R_1 / 32 as fixed_t,
            y: -R_1 / 7 as fixed_t - R_1 / 32 as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_1 / 6 as fixed_t + R_1 / 32 as fixed_t,
            y: -R_1 / 7 as fixed_t - R_1 / 32 as fixed_t,
        },
        b: mpoint_t {
            x: R_1 / 6 as fixed_t + R_1 / 10 as fixed_t,
            y: -R_1 / 7 as fixed_t,
        },
    },
];
pub const R_2: i32 = (1 as i32) << FRACBITS;
#[no_mangle]
pub static triangle_guy: [mline_t; 3] = [
    mline_t {
        a: mpoint_t {
            x: (-0.867f64 * R_2 as f64) as fixed_t,
            y: (-0.5f64 * R_2 as f64) as fixed_t,
        },
        b: mpoint_t {
            x: (0.867f64 * R_2 as f64) as fixed_t,
            y: (-0.5f64 * R_2 as f64) as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: (0.867f64 * R_2 as f64) as fixed_t,
            y: (-0.5f64 * R_2 as f64) as fixed_t,
        },
        b: mpoint_t {
            x: 0 as i32,
            y: (1 as i32) << FRACBITS,
        },
    },
    mline_t {
        a: mpoint_t {
            x: 0 as i32,
            y: (1 as i32) << FRACBITS,
        },
        b: mpoint_t {
            x: (-0.867f64 * R_2 as f64) as fixed_t,
            y: (-0.5f64 * R_2 as f64) as fixed_t,
        },
    },
];
pub const R: i32 = (1 as i32) << FRACBITS;
#[no_mangle]
pub static thintriangle_guy: [mline_t; 3] = [
    mline_t {
        a: mpoint_t {
            x: (-0.5f64 * R as f64) as fixed_t,
            y: (-0.7f64 * R as f64) as fixed_t,
        },
        b: mpoint_t {
            x: (1 as i32) << FRACBITS,
            y: 0 as i32,
        },
    },
    mline_t {
        a: mpoint_t {
            x: (1 as i32) << FRACBITS,
            y: 0 as i32,
        },
        b: mpoint_t {
            x: (-0.5f64 * R as f64) as fixed_t,
            y: (0.7f64 * R as f64) as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: (-0.5f64 * R as f64) as fixed_t,
            y: (0.7f64 * R as f64) as fixed_t,
        },
        b: mpoint_t {
            x: (-0.5f64 * R as f64) as fixed_t,
            y: (-0.7f64 * R as f64) as fixed_t,
        },
    },
];
static finit_width: i32 = SCREENWIDTH;
static finit_height: i32 = SCREENHEIGHT - 32;
pub unsafe fn AM_getIslope(mut ml: *mut mline_t, mut is: *mut islope_t) {
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    dy = ((*ml).a.y - (*ml).b.y) as i32;
    dx = ((*ml).b.x - (*ml).a.x) as i32;
    if dy == 0 {
        (*is).islp = (if dx < 0 as i32 { -INT_MAX } else { INT_MAX }) as fixed_t;
    } else {
        (*is).islp = FixedDiv(dx as fixed_t, dy as fixed_t);
    }
    if dx == 0 {
        (*is).slp = (if dy < 0 as i32 { -INT_MAX } else { INT_MAX }) as fixed_t;
    } else {
        (*is).slp = FixedDiv(dy as fixed_t, dx as fixed_t);
    };
}
pub unsafe fn AM_activateNewScale() {
    unsafe { game_state() }.am_map.m_x += unsafe { game_state() }.am_map.m_w as i32 / 2 as i32;
    unsafe { game_state() }.am_map.m_y += unsafe { game_state() }.am_map.m_h as i32 / 2 as i32;
    unsafe { game_state() }.am_map.m_w = FixedMul((unsafe { game_state() }.am_map.f_w as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
    unsafe { game_state() }.am_map.m_h = FixedMul((unsafe { game_state() }.am_map.f_h as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
    unsafe { game_state() }.am_map.m_x -= unsafe { game_state() }.am_map.m_w as i32 / 2 as i32;
    unsafe { game_state() }.am_map.m_y -= unsafe { game_state() }.am_map.m_h as i32 / 2 as i32;
    unsafe { game_state() }.am_map.m_x2 = unsafe { game_state() }.am_map.m_x + unsafe { game_state() }.am_map.m_w;
    unsafe { game_state() }.am_map.m_y2 = unsafe { game_state() }.am_map.m_y + unsafe { game_state() }.am_map.m_h;
}
pub unsafe fn AM_saveScaleAndLoc() {
    unsafe { game_state() }.am_map.old_m_x = unsafe { game_state() }.am_map.m_x;
    unsafe { game_state() }.am_map.old_m_y = unsafe { game_state() }.am_map.m_y;
    unsafe { game_state() }.am_map.old_m_w = unsafe { game_state() }.am_map.m_w;
    unsafe { game_state() }.am_map.old_m_h = unsafe { game_state() }.am_map.m_h;
}
pub unsafe fn AM_restoreScaleAndLoc() {
    unsafe { game_state() }.am_map.m_w = unsafe { game_state() }.am_map.old_m_w;
    unsafe { game_state() }.am_map.m_h = unsafe { game_state() }.am_map.old_m_h;
    if unsafe { game_state() }.am_map.followplayer == 0 {
        unsafe { game_state() }.am_map.m_x = unsafe { game_state() }.am_map.old_m_x;
        unsafe { game_state() }.am_map.m_y = unsafe { game_state() }.am_map.old_m_y;
    } else {
        unsafe { game_state() }.am_map.m_x = ((*(*unsafe { game_state() }.am_map.plr).mo).x as i32 - unsafe { game_state() }.am_map.m_w as i32 / 2 as i32)
            as fixed_t;
        unsafe { game_state() }.am_map.m_y = ((*(*unsafe { game_state() }.am_map.plr).mo).y as i32 - unsafe { game_state() }.am_map.m_h as i32 / 2 as i32)
            as fixed_t;
    }
    unsafe { game_state() }.am_map.m_x2 = unsafe { game_state() }.am_map.m_x + unsafe { game_state() }.am_map.m_w;
    unsafe { game_state() }.am_map.m_y2 = unsafe { game_state() }.am_map.m_y + unsafe { game_state() }.am_map.m_h;
    unsafe { game_state() }.am_map.scale_mtof = FixedDiv((unsafe { game_state() }.am_map.f_w as fixed_t) << FRACBITS, unsafe { game_state() }.am_map.m_w);
    unsafe { game_state() }.am_map.scale_ftom = FixedDiv(FRACUNIT, unsafe { game_state() }.am_map.scale_mtof);
}
pub unsafe fn AM_addMark() {
    unsafe { game_state() }.am_map.markpoints[unsafe { game_state() }.am_map.markpointnum as usize].x = (unsafe { game_state() }.am_map.m_x as i32 + unsafe { game_state() }.am_map.m_w as i32 / 2 as i32) as fixed_t;
    unsafe { game_state() }.am_map.markpoints[unsafe { game_state() }.am_map.markpointnum as usize].y = (unsafe { game_state() }.am_map.m_y as i32 + unsafe { game_state() }.am_map.m_h as i32 / 2 as i32) as fixed_t;
    unsafe { game_state() }.am_map.markpointnum = (unsafe { game_state() }.am_map.markpointnum + 1 as i32) % AM_NUMMARKPOINTS;
}
pub unsafe fn AM_findMinMaxBoundaries() {
    let mut i: i32 = 0;
    let mut a: fixed_t = 0;
    let mut b: fixed_t = 0;
    unsafe { game_state() }.am_map.min_y = INT_MAX as fixed_t;
    unsafe { game_state() }.am_map.min_x = unsafe { game_state() }.am_map.min_y;
    unsafe { game_state() }.am_map.max_y = -INT_MAX as fixed_t;
    unsafe { game_state() }.am_map.max_x = unsafe { game_state() }.am_map.max_y;
    i = 0 as i32;
    while i < numvertexes {
        if (*vertexes.offset(i as isize)).x < unsafe { game_state() }.am_map.min_x {
            unsafe { game_state() }.am_map.min_x = (*vertexes.offset(i as isize)).x;
        } else if (*vertexes.offset(i as isize)).x > unsafe { game_state() }.am_map.max_x {
            unsafe { game_state() }.am_map.max_x = (*vertexes.offset(i as isize)).x;
        }
        if (*vertexes.offset(i as isize)).y < unsafe { game_state() }.am_map.min_y {
            unsafe { game_state() }.am_map.min_y = (*vertexes.offset(i as isize)).y;
        } else if (*vertexes.offset(i as isize)).y > unsafe { game_state() }.am_map.max_y {
            unsafe { game_state() }.am_map.max_y = (*vertexes.offset(i as isize)).y;
        }
        i += 1;
    }
    unsafe { game_state() }.am_map.max_w = unsafe { game_state() }.am_map.max_x - unsafe { game_state() }.am_map.min_x;
    unsafe { game_state() }.am_map.max_h = unsafe { game_state() }.am_map.max_y - unsafe { game_state() }.am_map.min_y;
    unsafe { game_state() }.am_map.min_w = (2 as i32 * 16 as i32 * FRACUNIT) as fixed_t;
    unsafe { game_state() }.am_map.min_h = (2 as i32 * 16 as i32 * FRACUNIT) as fixed_t;
    a = FixedDiv((unsafe { game_state() }.am_map.f_w as fixed_t) << FRACBITS, unsafe { game_state() }.am_map.max_w);
    b = FixedDiv((unsafe { game_state() }.am_map.f_h as fixed_t) << FRACBITS, unsafe { game_state() }.am_map.max_h);
    unsafe { game_state() }.am_map.min_scale_mtof = if a < b { a } else { b };
    unsafe { game_state() }.am_map.max_scale_mtof = FixedDiv(
        (unsafe { game_state() }.am_map.f_h as fixed_t) << FRACBITS,
        2 as fixed_t * 16 as fixed_t * FRACUNIT,
    );
}
pub unsafe fn AM_changeWindowLoc() {
    if unsafe { game_state() }.am_map.m_paninc.x != 0 || unsafe { game_state() }.am_map.m_paninc.y != 0 {
        unsafe { game_state() }.am_map.followplayer = 0 as i32;
        unsafe { game_state() }.am_map.f_oldloc.x = INT_MAX as fixed_t;
    }
    unsafe { game_state() }.am_map.m_x += unsafe { game_state() }.am_map.m_paninc.x;
    unsafe { game_state() }.am_map.m_y += unsafe { game_state() }.am_map.m_paninc.y;
    if unsafe { game_state() }.am_map.m_x as i32 + unsafe { game_state() }.am_map.m_w as i32 / 2 as i32 > unsafe { game_state() }.am_map.max_x {
        unsafe { game_state() }.am_map.m_x = (unsafe { game_state() }.am_map.max_x as i32 - unsafe { game_state() }.am_map.m_w as i32 / 2 as i32) as fixed_t;
    } else if (unsafe { game_state() }.am_map.m_x as i32 + unsafe { game_state() }.am_map.m_w as i32 / 2 as i32) < unsafe { game_state() }.am_map.min_x {
        unsafe { game_state() }.am_map.m_x = (unsafe { game_state() }.am_map.min_x as i32 - unsafe { game_state() }.am_map.m_w as i32 / 2 as i32) as fixed_t;
    }
    if unsafe { game_state() }.am_map.m_y as i32 + unsafe { game_state() }.am_map.m_h as i32 / 2 as i32 > unsafe { game_state() }.am_map.max_y {
        unsafe { game_state() }.am_map.m_y = (unsafe { game_state() }.am_map.max_y as i32 - unsafe { game_state() }.am_map.m_h as i32 / 2 as i32) as fixed_t;
    } else if (unsafe { game_state() }.am_map.m_y as i32 + unsafe { game_state() }.am_map.m_h as i32 / 2 as i32) < unsafe { game_state() }.am_map.min_y {
        unsafe { game_state() }.am_map.m_y = (unsafe { game_state() }.am_map.min_y as i32 - unsafe { game_state() }.am_map.m_h as i32 / 2 as i32) as fixed_t;
    }
    unsafe { game_state() }.am_map.m_x2 = unsafe { game_state() }.am_map.m_x + unsafe { game_state() }.am_map.m_w;
    unsafe { game_state() }.am_map.m_y2 = unsafe { game_state() }.am_map.m_y + unsafe { game_state() }.am_map.m_h;
}
pub unsafe fn AM_initVariables() {
    let mut pnum: i32 = 0;
    const st_notify: event_t = event_t {
        type_0: ev_keyup,
        data1: AM_MSGENTERED,
        data2: 0 as i32,
        data3: 0 as i32,
        data4: 0,
    };
    unsafe { game_state() }.am_map.automapactive = true;
    unsafe { game_state() }.am_map.fb = I_VideoBuffer;
    unsafe { game_state() }.am_map.f_oldloc.x = INT_MAX as fixed_t;
    unsafe { game_state() }.am_map.amclock = 0 as i32;
    unsafe { game_state() }.am_map.lightlev = 0 as i32;
    unsafe { game_state() }.am_map.m_paninc.y = 0 as i32 as fixed_t;
    unsafe { game_state() }.am_map.m_paninc.x = unsafe { game_state() }.am_map.m_paninc.y;
    unsafe { game_state() }.am_map.ftom_zoommul = FRACUNIT as fixed_t;
    unsafe { game_state() }.am_map.mtof_zoommul = FRACUNIT as fixed_t;
    unsafe { game_state() }.am_map.m_w = FixedMul((unsafe { game_state() }.am_map.f_w as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
    unsafe { game_state() }.am_map.m_h = FixedMul((unsafe { game_state() }.am_map.f_h as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
    if unsafe { game_state() }.g_game.playeringame
        [unsafe { game_state() }.g_game.consoleplayer as usize]
        != 0
    {
        unsafe { game_state() }.am_map.plr = (&raw mut unsafe { game_state() }.g_game.players
            as *mut player_t)
            .offset(unsafe { game_state() }.g_game.consoleplayer as isize)
            as *mut player_t;
    } else {
        unsafe { game_state() }.am_map.plr =
            (&raw mut unsafe { game_state() }.g_game.players as *mut player_t)
                .offset(0 as i32 as isize) as *mut player_t;
        pnum = 0 as i32;
        while pnum < MAXPLAYERS {
            if unsafe { game_state() }.g_game.playeringame[pnum as usize] != 0 {
                unsafe { game_state() }.am_map.plr =
                    (&raw mut unsafe { game_state() }.g_game.players as *mut player_t)
                        .offset(pnum as isize) as *mut player_t;
                break;
            } else {
                pnum += 1;
            }
        }
    }
    unsafe { game_state() }.am_map.m_x =
        ((*(*unsafe { game_state() }.am_map.plr).mo).x as i32 - unsafe { game_state() }.am_map.m_w as i32 / 2 as i32) as fixed_t;
    unsafe { game_state() }.am_map.m_y =
        ((*(*unsafe { game_state() }.am_map.plr).mo).y as i32 - unsafe { game_state() }.am_map.m_h as i32 / 2 as i32) as fixed_t;
    AM_changeWindowLoc();
    unsafe { game_state() }.am_map.old_m_x = unsafe { game_state() }.am_map.m_x;
    unsafe { game_state() }.am_map.old_m_y = unsafe { game_state() }.am_map.m_y;
    unsafe { game_state() }.am_map.old_m_w = unsafe { game_state() }.am_map.m_w;
    unsafe { game_state() }.am_map.old_m_h = unsafe { game_state() }.am_map.m_h;
    ST_Responder(&st_notify);
}
pub unsafe fn AM_loadPics() {
    let mut i: i32 = 0;
    let mut namebuf: [::core::ffi::c_char; 9] = [0; 9];
    i = 0 as i32;
    while i < 10 as i32 {
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"AMMNUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        unsafe { game_state() }.am_map.marknums[i as usize] = W_CacheLumpName(
            &wad_name8_to_string(&raw mut namebuf as *mut ::core::ffi::c_char),
            PU_STATIC as i32,
        ) as *mut patch_t;
        i += 1;
    }
}
pub unsafe fn AM_unloadPics() {
    let mut i: i32 = 0;
    let mut namebuf: [::core::ffi::c_char; 9] = [0; 9];
    i = 0 as i32;
    while i < 10 as i32 {
        snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            9 as size_t,
            b"AMMNUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        W_ReleaseLumpName(&wad_name8_to_string(
            &raw mut namebuf as *mut ::core::ffi::c_char,
        ));
        i += 1;
    }
}
pub unsafe fn AM_clearMarks() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < AM_NUMMARKPOINTS {
        unsafe { game_state() }.am_map.markpoints[i as usize].x = -(1 as i32) as fixed_t;
        i += 1;
    }
    unsafe { game_state() }.am_map.markpointnum = 0 as i32;
}
pub unsafe fn AM_LevelInit() {
    unsafe { game_state() }.am_map.leveljuststarted = 0 as i32;
    unsafe { game_state() }.am_map.f_y = 0 as i32;
    unsafe { game_state() }.am_map.f_x = unsafe { game_state() }.am_map.f_y;
    unsafe { game_state() }.am_map.f_w = finit_width;
    unsafe { game_state() }.am_map.f_h = finit_height;
    AM_clearMarks();
    AM_findMinMaxBoundaries();
    unsafe { game_state() }.am_map.scale_mtof = FixedDiv(unsafe { game_state() }.am_map.min_scale_mtof, (0.7f64 * FRACUNIT as f64) as fixed_t);
    if unsafe { game_state() }.am_map.scale_mtof > unsafe { game_state() }.am_map.max_scale_mtof {
        unsafe { game_state() }.am_map.scale_mtof = unsafe { game_state() }.am_map.min_scale_mtof;
    }
    unsafe { game_state() }.am_map.scale_ftom = FixedDiv(FRACUNIT, unsafe { game_state() }.am_map.scale_mtof);
}
pub unsafe fn AM_Stop() {
    const st_notify: event_t = event_t {
        type_0: ev_keydown,
        data1: ev_keyup as i32,
        data2: AM_MSGEXITED,
        data3: 0 as i32,
        data4: 0,
    };
    AM_unloadPics();
    unsafe { game_state() }.am_map.automapactive = false;
    ST_Responder(&st_notify);
    unsafe { game_state() }.am_map.stopped = true;
}
pub unsafe fn AM_Start() {
    if !unsafe { game_state() }.am_map.stopped {
        AM_Stop();
    }
    unsafe { game_state() }.am_map.stopped = false;
    if unsafe { game_state() }.am_map.am_start_lastlevel != unsafe { game_state() }.g_game.gamemap
        || unsafe { game_state() }.am_map.am_start_lastepisode
            != unsafe { game_state() }.g_game.gameepisode
    {
        AM_LevelInit();
        unsafe { game_state() }.am_map.am_start_lastlevel =
            unsafe { game_state() }.g_game.gamemap;
        unsafe { game_state() }.am_map.am_start_lastepisode =
            unsafe { game_state() }.g_game.gameepisode;
    }
    AM_initVariables();
    AM_loadPics();
}
pub unsafe fn AM_minOutWindowScale() {
    unsafe { game_state() }.am_map.scale_mtof = unsafe { game_state() }.am_map.min_scale_mtof;
    unsafe { game_state() }.am_map.scale_ftom = FixedDiv(FRACUNIT, unsafe { game_state() }.am_map.scale_mtof);
    AM_activateNewScale();
}
pub unsafe fn AM_maxOutWindowScale() {
    unsafe { game_state() }.am_map.scale_mtof = unsafe { game_state() }.am_map.max_scale_mtof;
    unsafe { game_state() }.am_map.scale_ftom = FixedDiv(FRACUNIT, unsafe { game_state() }.am_map.scale_mtof);
    AM_activateNewScale();
}
pub unsafe fn AM_Responder(mut ev: &event_t) -> bool {
    let mut rc: i32 = 0;
    let mut key: i32 = 0;
    rc = false_0;
    if !unsafe { game_state() }.am_map.automapactive {
        if (*ev).type_0 as u32 == ev_keydown as i32 as u32
            && (*ev).data1 == unsafe { game_state() }.m_controls.key_map_toggle
        {
            AM_Start();
            unsafe { game_state() }.g_game.viewactive = false;
            rc = true_0;
        }
    } else if (*ev).type_0 as u32 == ev_keydown as i32 as u32 {
        rc = true_0;
        key = (*ev).data1;
        if key == unsafe { game_state() }.m_controls.key_map_east {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.x = FixedMul((4 as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_west {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.x = -FixedMul((4 as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_north {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.y = FixedMul((4 as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_south {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.y = -FixedMul((4 as fixed_t) << 16 as i32, unsafe { game_state() }.am_map.scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_zoomout {
            unsafe { game_state() }.am_map.mtof_zoommul = M_ZOOMOUT as fixed_t;
            unsafe { game_state() }.am_map.ftom_zoommul = M_ZOOMIN as fixed_t;
        } else if key == unsafe { game_state() }.m_controls.key_map_zoomin {
            unsafe { game_state() }.am_map.mtof_zoommul = M_ZOOMIN as fixed_t;
            unsafe { game_state() }.am_map.ftom_zoommul = M_ZOOMOUT as fixed_t;
        } else if key == unsafe { game_state() }.m_controls.key_map_toggle {
            unsafe { game_state() }.am_map.am_responder_bigstate = 0 as i32;
            unsafe { game_state() }.g_game.viewactive = true;
            AM_Stop();
        } else if key == unsafe { game_state() }.m_controls.key_map_maxzoom {
            unsafe { game_state() }.am_map.am_responder_bigstate =
                (unsafe { game_state() }.am_map.am_responder_bigstate == 0) as i32;
            if unsafe { game_state() }.am_map.am_responder_bigstate != 0 {
                AM_saveScaleAndLoc();
                AM_minOutWindowScale();
            } else {
                AM_restoreScaleAndLoc();
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_follow {
            unsafe { game_state() }.am_map.followplayer = (unsafe { game_state() }.am_map.followplayer == 0) as i32;
            unsafe { game_state() }.am_map.f_oldloc.x = INT_MAX as fixed_t;
            if unsafe { game_state() }.am_map.followplayer != 0 {
                (*unsafe { game_state() }.am_map.plr).message = b"Follow Mode ON\0" as *const u8
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                (*unsafe { game_state() }.am_map.plr).message = b"Follow Mode OFF\0" as *const u8
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_grid {
            unsafe { game_state() }.am_map.grid = (unsafe { game_state() }.am_map.grid == 0) as i32;
            if unsafe { game_state() }.am_map.grid != 0 {
                (*unsafe { game_state() }.am_map.plr).message = b"Grid ON\0" as *const u8
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                (*unsafe { game_state() }.am_map.plr).message = b"Grid OFF\0" as *const u8
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_mark {
            M_snprintf(
                &raw mut unsafe { game_state() }.am_map.am_responder_buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                b"%s %d\0" as *const u8 as *const ::core::ffi::c_char,
                b"Marked Spot\0" as *const u8 as *const ::core::ffi::c_char,
                unsafe { game_state() }.am_map.markpointnum,
            );
            (*unsafe { game_state() }.am_map.plr).message =
                &raw mut unsafe { game_state() }.am_map.am_responder_buffer as *mut ::core::ffi::c_char;
            AM_addMark();
        } else if key == unsafe { game_state() }.m_controls.key_map_clearmark {
            AM_clearMarks();
            (*unsafe { game_state() }.am_map.plr).message = b"All Marks Cleared\0" as *const u8
                as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else {
            rc = false_0;
        }
        if unsafe { game_state() }.g_game.deathmatch == 0
            && cht_CheckCheat(&raw mut unsafe { game_state() }.am_map.cheat_amap, (*ev).data2 as ::core::ffi::c_char) != 0
        {
            rc = false_0;
            unsafe { game_state() }.am_map.cheating = (unsafe { game_state() }.am_map.cheating + 1 as i32) % 3 as i32;
        }
    } else if (*ev).type_0 as u32 == ev_keyup as i32 as u32 {
        rc = false_0;
        key = (*ev).data1;
        if key == unsafe { game_state() }.m_controls.key_map_east {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.x = 0 as i32 as fixed_t;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_west {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.x = 0 as i32 as fixed_t;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_north {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.y = 0 as i32 as fixed_t;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_south {
            if unsafe { game_state() }.am_map.followplayer == 0 {
                unsafe { game_state() }.am_map.m_paninc.y = 0 as i32 as fixed_t;
            }
        } else if key == unsafe { game_state() }.m_controls.key_map_zoomout
            || key == unsafe { game_state() }.m_controls.key_map_zoomin
        {
            unsafe { game_state() }.am_map.mtof_zoommul = FRACUNIT as fixed_t;
            unsafe { game_state() }.am_map.ftom_zoommul = FRACUNIT as fixed_t;
        }
    }
    return rc != 0;
}
pub unsafe fn AM_changeWindowScale() {
    unsafe { game_state() }.am_map.scale_mtof = FixedMul(unsafe { game_state() }.am_map.scale_mtof, unsafe { game_state() }.am_map.mtof_zoommul);
    unsafe { game_state() }.am_map.scale_ftom = FixedDiv(FRACUNIT, unsafe { game_state() }.am_map.scale_mtof);
    if unsafe { game_state() }.am_map.scale_mtof < unsafe { game_state() }.am_map.min_scale_mtof {
        AM_minOutWindowScale();
    } else if unsafe { game_state() }.am_map.scale_mtof > unsafe { game_state() }.am_map.max_scale_mtof {
        AM_maxOutWindowScale();
    } else {
        AM_activateNewScale();
    };
}
pub unsafe fn AM_doFollowPlayer() {
    if unsafe { game_state() }.am_map.f_oldloc.x != (*(*unsafe { game_state() }.am_map.plr).mo).x
        || unsafe { game_state() }.am_map.f_oldloc.y != (*(*unsafe { game_state() }.am_map.plr).mo).y
    {
        unsafe { game_state() }.am_map.m_x = (FixedMul(
            (FixedMul((*(*unsafe { game_state() }.am_map.plr).mo).x, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)
                << 16 as i32,
            unsafe { game_state() }.am_map.scale_ftom,
        ) as i32
            - unsafe { game_state() }.am_map.m_w as i32 / 2 as i32) as fixed_t;
        unsafe { game_state() }.am_map.m_y = (FixedMul(
            (FixedMul((*(*unsafe { game_state() }.am_map.plr).mo).y, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)
                << 16 as i32,
            unsafe { game_state() }.am_map.scale_ftom,
        ) as i32
            - unsafe { game_state() }.am_map.m_h as i32 / 2 as i32) as fixed_t;
        unsafe { game_state() }.am_map.m_x2 = unsafe { game_state() }.am_map.m_x + unsafe { game_state() }.am_map.m_w;
        unsafe { game_state() }.am_map.m_y2 = unsafe { game_state() }.am_map.m_y + unsafe { game_state() }.am_map.m_h;
        unsafe { game_state() }.am_map.f_oldloc.x = (*(*unsafe { game_state() }.am_map.plr).mo).x;
        unsafe { game_state() }.am_map.f_oldloc.y = (*(*unsafe { game_state() }.am_map.plr).mo).y;
    }
}
pub unsafe fn AM_updateLightLev() {
    const litelevels: [i32; 8] = [
        0 as i32, 4 as i32, 7 as i32, 10 as i32, 12 as i32, 14 as i32, 15 as i32, 15 as i32,
    ];
    if unsafe { game_state() }.am_map.amclock > unsafe { game_state() }.am_map.am_updatelightlev_nexttic {
        let fresh1 = unsafe { game_state() }.am_map.am_updatelightlev_litelevelscnt;
        unsafe { game_state() }.am_map.am_updatelightlev_litelevelscnt =
            unsafe { game_state() }.am_map.am_updatelightlev_litelevelscnt + 1;
        unsafe { game_state() }.am_map.lightlev = litelevels[fresh1 as usize];
        if unsafe { game_state() }.am_map.am_updatelightlev_litelevelscnt as usize
            == (::core::mem::size_of::<[i32; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<i32>() as usize)
        {
            unsafe { game_state() }.am_map.am_updatelightlev_litelevelscnt = 0 as i32;
        }
        unsafe { game_state() }.am_map.am_updatelightlev_nexttic =
            unsafe { game_state() }.am_map.amclock + 6 as i32 - unsafe { game_state() }.am_map.amclock % 6 as i32;
    }
}
pub unsafe fn AM_Ticker() {
    if !unsafe { game_state() }.am_map.automapactive {
        return;
    }
    unsafe { game_state() }.am_map.amclock += 1;
    if unsafe { game_state() }.am_map.followplayer != 0 {
        AM_doFollowPlayer();
    }
    if unsafe { game_state() }.am_map.ftom_zoommul != FRACUNIT {
        AM_changeWindowScale();
    }
    if unsafe { game_state() }.am_map.m_paninc.x != 0 || unsafe { game_state() }.am_map.m_paninc.y != 0 {
        AM_changeWindowLoc();
    }
}
pub unsafe fn AM_clearFB(mut color: i32) {
    memset(unsafe { game_state() }.am_map.fb as *mut ::core::ffi::c_void, color, (unsafe { game_state() }.am_map.f_w * unsafe { game_state() }.am_map.f_h) as size_t);
}
pub unsafe fn AM_clipMline(mut ml: *mut mline_t, mut fl: *mut fline_t) -> bool {
    let mut outcode1: i32 = 0 as i32;
    let mut outcode2: i32 = 0 as i32;
    let mut outside: i32 = 0;
    let mut tmp: fpoint_t = fpoint_t { x: 0, y: 0 };
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    if (*ml).a.y > unsafe { game_state() }.am_map.m_y2 {
        outcode1 = TOP as i32;
    } else if (*ml).a.y < unsafe { game_state() }.am_map.m_y {
        outcode1 = BOTTOM as i32;
    }
    if (*ml).b.y > unsafe { game_state() }.am_map.m_y2 {
        outcode2 = TOP as i32;
    } else if (*ml).b.y < unsafe { game_state() }.am_map.m_y {
        outcode2 = BOTTOM as i32;
    }
    if outcode1 & outcode2 != 0 {
        return false;
    }
    if (*ml).a.x < unsafe { game_state() }.am_map.m_x {
        outcode1 |= LEFT as i32;
    } else if (*ml).a.x > unsafe { game_state() }.am_map.m_x2 {
        outcode1 |= RIGHT as i32;
    }
    if (*ml).b.x < unsafe { game_state() }.am_map.m_x {
        outcode2 |= LEFT as i32;
    } else if (*ml).b.x > unsafe { game_state() }.am_map.m_x2 {
        outcode2 |= RIGHT as i32;
    }
    if outcode1 & outcode2 != 0 {
        return false;
    }
    (*fl).a.x = (unsafe { game_state() }.am_map.f_x as fixed_t + (FixedMul((*ml).a.x - unsafe { game_state() }.am_map.m_x, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)) as i32;
    (*fl).a.y = (unsafe { game_state() }.am_map.f_y as fixed_t
        + (unsafe { game_state() }.am_map.f_h as fixed_t - (FixedMul((*ml).a.y - unsafe { game_state() }.am_map.m_y, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)))
        as i32;
    (*fl).b.x = (unsafe { game_state() }.am_map.f_x as fixed_t + (FixedMul((*ml).b.x - unsafe { game_state() }.am_map.m_x, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)) as i32;
    (*fl).b.y = (unsafe { game_state() }.am_map.f_y as fixed_t
        + (unsafe { game_state() }.am_map.f_h as fixed_t - (FixedMul((*ml).b.y - unsafe { game_state() }.am_map.m_y, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)))
        as i32;
    outcode1 = 0 as i32;
    if (*fl).a.y < 0 as i32 {
        outcode1 |= TOP as i32;
    } else if (*fl).a.y >= unsafe { game_state() }.am_map.f_h {
        outcode1 |= BOTTOM as i32;
    }
    if (*fl).a.x < 0 as i32 {
        outcode1 |= LEFT as i32;
    } else if (*fl).a.x >= unsafe { game_state() }.am_map.f_w {
        outcode1 |= RIGHT as i32;
    }
    outcode2 = 0 as i32;
    if (*fl).b.y < 0 as i32 {
        outcode2 |= TOP as i32;
    } else if (*fl).b.y >= unsafe { game_state() }.am_map.f_h {
        outcode2 |= BOTTOM as i32;
    }
    if (*fl).b.x < 0 as i32 {
        outcode2 |= LEFT as i32;
    } else if (*fl).b.x >= unsafe { game_state() }.am_map.f_w {
        outcode2 |= RIGHT as i32;
    }
    if outcode1 & outcode2 != 0 {
        return false;
    }
    while outcode1 | outcode2 != 0 {
        if outcode1 != 0 {
            outside = outcode1;
        } else {
            outside = outcode2;
        }
        if outside & TOP as i32 != 0 {
            dy = (*fl).a.y - (*fl).b.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.x = (*fl).a.x + dx * (*fl).a.y / dy;
            tmp.y = 0 as i32;
        } else if outside & BOTTOM as i32 != 0 {
            dy = (*fl).a.y - (*fl).b.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.x = (*fl).a.x + dx * ((*fl).a.y - unsafe { game_state() }.am_map.f_h) / dy;
            tmp.y = unsafe { game_state() }.am_map.f_h - 1 as i32;
        } else if outside & RIGHT as i32 != 0 {
            dy = (*fl).b.y - (*fl).a.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + dy * (unsafe { game_state() }.am_map.f_w - 1 as i32 - (*fl).a.x) / dx;
            tmp.x = unsafe { game_state() }.am_map.f_w - 1 as i32;
        } else if outside & LEFT as i32 != 0 {
            dy = (*fl).b.y - (*fl).a.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + dy * -(*fl).a.x / dx;
            tmp.x = 0 as i32;
        } else {
            tmp.x = 0 as i32;
            tmp.y = 0 as i32;
        }
        if outside == outcode1 {
            (*fl).a = tmp;
            outcode1 = 0 as i32;
            if (*fl).a.y < 0 as i32 {
                outcode1 |= TOP as i32;
            } else if (*fl).a.y >= unsafe { game_state() }.am_map.f_h {
                outcode1 |= BOTTOM as i32;
            }
            if (*fl).a.x < 0 as i32 {
                outcode1 |= LEFT as i32;
            } else if (*fl).a.x >= unsafe { game_state() }.am_map.f_w {
                outcode1 |= RIGHT as i32;
            }
        } else {
            (*fl).b = tmp;
            outcode2 = 0 as i32;
            if (*fl).b.y < 0 as i32 {
                outcode2 |= TOP as i32;
            } else if (*fl).b.y >= unsafe { game_state() }.am_map.f_h {
                outcode2 |= BOTTOM as i32;
            }
            if (*fl).b.x < 0 as i32 {
                outcode2 |= LEFT as i32;
            } else if (*fl).b.x >= unsafe { game_state() }.am_map.f_w {
                outcode2 |= RIGHT as i32;
            }
        }
        if outcode1 & outcode2 != 0 {
            return false;
        }
    }
    return true;
}
pub unsafe fn AM_drawFline(mut fl: *mut fline_t, mut color: i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut ax: i32 = 0;
    let mut ay: i32 = 0;
    let mut d: i32 = 0;
    if (*fl).a.x < 0 as i32
        || (*fl).a.x >= unsafe { game_state() }.am_map.f_w
        || (*fl).a.y < 0 as i32
        || (*fl).a.y >= unsafe { game_state() }.am_map.f_h
        || (*fl).b.x < 0 as i32
        || (*fl).b.x >= unsafe { game_state() }.am_map.f_w
        || (*fl).b.y < 0 as i32
        || (*fl).b.y >= unsafe { game_state() }.am_map.f_h
    {
        let fresh0 = unsafe { game_state() }.am_map.am_drawfline_fuck;
        unsafe { game_state() }.am_map.am_drawfline_fuck =
            unsafe { game_state() }.am_map.am_drawfline_fuck + 1;
        fprintf(
            stderr,
            b"fuck %d \r\0" as *const u8 as *const ::core::ffi::c_char,
            fresh0,
        );
        return;
    }
    dx = (*fl).b.x - (*fl).a.x;
    ax = 2 as i32 * (if dx < 0 as i32 { -dx } else { dx });
    sx = if dx < 0 as i32 { -(1 as i32) } else { 1 as i32 };
    dy = (*fl).b.y - (*fl).a.y;
    ay = 2 as i32 * (if dy < 0 as i32 { -dy } else { dy });
    sy = if dy < 0 as i32 { -(1 as i32) } else { 1 as i32 };
    x = (*fl).a.x;
    y = (*fl).a.y;
    if ax > ay {
        d = ay - ax / 2 as i32;
        loop {
            *unsafe { game_state() }.am_map.fb.offset((y * unsafe { game_state() }.am_map.f_w + x) as isize) = color as byte;
            if x == (*fl).b.x {
                return;
            }
            if d >= 0 as i32 {
                y += sy;
                d -= ax;
            }
            x += sx;
            d += ay;
        }
    } else {
        d = ax - ay / 2 as i32;
        loop {
            *unsafe { game_state() }.am_map.fb.offset((y * unsafe { game_state() }.am_map.f_w + x) as isize) = color as byte;
            if y == (*fl).b.y {
                return;
            }
            if d >= 0 as i32 {
                x += sx;
                d -= ay;
            }
            y += sy;
            d += ax;
        }
    };
}
pub unsafe fn AM_drawMline(mut ml: *mut mline_t, mut color: i32) {
    let mut fl: fline_t = fline_t {
        a: fpoint_t { x: 0, y: 0 },
        b: fpoint_t { x: 0, y: 0 },
    };
    if AM_clipMline(ml, &raw mut fl) {
        AM_drawFline(&raw mut fl, color);
    }
}
pub unsafe fn AM_drawGrid(mut color: i32) {
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut start: fixed_t = 0;
    let mut end: fixed_t = 0;
    let mut ml: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };
    start = unsafe { game_state() }.am_map.m_x;
    if (start as i32 - bmaporgx as i32) % (MAPBLOCKUNITS << FRACBITS) != 0 {
        start += (MAPBLOCKUNITS << FRACBITS)
            - (start as i32 - bmaporgx as i32) % (MAPBLOCKUNITS << FRACBITS);
    }
    end = unsafe { game_state() }.am_map.m_x + unsafe { game_state() }.am_map.m_w;
    ml.a.y = unsafe { game_state() }.am_map.m_y;
    ml.b.y = unsafe { game_state() }.am_map.m_y + unsafe { game_state() }.am_map.m_h;
    x = start;
    while x < end {
        ml.a.x = x;
        ml.b.x = x;
        AM_drawMline(&raw mut ml, color);
        x += MAPBLOCKUNITS << FRACBITS;
    }
    start = unsafe { game_state() }.am_map.m_y;
    if (start as i32 - bmaporgy as i32) % (MAPBLOCKUNITS << FRACBITS) != 0 {
        start += (MAPBLOCKUNITS << FRACBITS)
            - (start as i32 - bmaporgy as i32) % (MAPBLOCKUNITS << FRACBITS);
    }
    end = unsafe { game_state() }.am_map.m_y + unsafe { game_state() }.am_map.m_h;
    ml.a.x = unsafe { game_state() }.am_map.m_x;
    ml.b.x = unsafe { game_state() }.am_map.m_x + unsafe { game_state() }.am_map.m_w;
    y = start;
    while y < end {
        ml.a.y = y;
        ml.b.y = y;
        AM_drawMline(&raw mut ml, color);
        y += MAPBLOCKUNITS << FRACBITS;
    }
}
pub unsafe fn AM_drawWalls() {
    let mut i: i32 = 0;
    let mut l: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };
    i = 0 as i32;
    while i < numlines {
        l.a.x = (*(*lines.offset(i as isize)).v1).x;
        l.a.y = (*(*lines.offset(i as isize)).v1).y;
        l.b.x = (*(*lines.offset(i as isize)).v2).x;
        l.b.y = (*(*lines.offset(i as isize)).v2).y;
        if unsafe { game_state() }.am_map.cheating != 0 || (*lines.offset(i as isize)).flags as i32 & ML_MAPPED != 0 {
            if !((*lines.offset(i as isize)).flags as i32 & LINE_NEVERSEE != 0 && unsafe { game_state() }.am_map.cheating == 0) {
                if (*lines.offset(i as isize)).backsector.is_null() {
                    AM_drawMline(&raw mut l, WALLCOLORS + unsafe { game_state() }.am_map.lightlev);
                } else if (*lines.offset(i as isize)).special as i32 == 39 as i32 {
                    AM_drawMline(&raw mut l, WALLCOLORS + WALLRANGE / 2 as i32);
                } else if (*lines.offset(i as isize)).flags as i32 & ML_SECRET != 0 {
                    if unsafe { game_state() }.am_map.cheating != 0 {
                        AM_drawMline(&raw mut l, SECRETWALLCOLORS + unsafe { game_state() }.am_map.lightlev);
                    } else {
                        AM_drawMline(&raw mut l, WALLCOLORS + unsafe { game_state() }.am_map.lightlev);
                    }
                } else if (*(*lines.offset(i as isize)).backsector).floorheight
                    != (*(*lines.offset(i as isize)).frontsector).floorheight
                {
                    AM_drawMline(&raw mut l, FDWALLCOLORS + unsafe { game_state() }.am_map.lightlev);
                } else if (*(*lines.offset(i as isize)).backsector).ceilingheight
                    != (*(*lines.offset(i as isize)).frontsector).ceilingheight
                {
                    AM_drawMline(&raw mut l, CDWALLCOLORS + unsafe { game_state() }.am_map.lightlev);
                } else if unsafe { game_state() }.am_map.cheating != 0 {
                    AM_drawMline(&raw mut l, TSWALLCOLORS + unsafe { game_state() }.am_map.lightlev);
                }
            }
        } else if (*unsafe { game_state() }.am_map.plr).powers[pw_allmap as i32 as usize] != 0 {
            if (*lines.offset(i as isize)).flags as i32 & LINE_NEVERSEE == 0 {
                AM_drawMline(&raw mut l, GRAYS + 3 as i32);
            }
        }
        i += 1;
    }
}
pub unsafe fn AM_rotate(mut x: *mut fixed_t, mut y: *mut fixed_t, mut a: angle_t) {
    let mut tmpx: fixed_t = 0;
    tmpx = FixedMul(*x, finecosine[(a >> ANGLETOFINESHIFT) as isize])
        - FixedMul(*y, finesine[(a >> ANGLETOFINESHIFT) as usize]);
    *y = FixedMul(*x, finesine[(a >> ANGLETOFINESHIFT) as usize])
        + FixedMul(*y, finecosine[(a >> ANGLETOFINESHIFT) as isize]);
    *x = tmpx;
}
pub unsafe fn AM_drawLineCharacter(
    mut lineguy: *mut mline_t,
    mut lineguylines: i32,
    mut scale: fixed_t,
    mut angle: angle_t,
    mut color: i32,
    mut x: fixed_t,
    mut y: fixed_t,
) {
    let mut i: i32 = 0;
    let mut l: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };
    i = 0 as i32;
    while i < lineguylines {
        l.a.x = (*lineguy.offset(i as isize)).a.x;
        l.a.y = (*lineguy.offset(i as isize)).a.y;
        if scale != 0 {
            l.a.x = FixedMul(scale, l.a.x);
            l.a.y = FixedMul(scale, l.a.y);
        }
        if angle != 0 {
            AM_rotate(&raw mut l.a.x, &raw mut l.a.y, angle);
        }
        l.a.x += x;
        l.a.y += y;
        l.b.x = (*lineguy.offset(i as isize)).b.x;
        l.b.y = (*lineguy.offset(i as isize)).b.y;
        if scale != 0 {
            l.b.x = FixedMul(scale, l.b.x);
            l.b.y = FixedMul(scale, l.b.y);
        }
        if angle != 0 {
            AM_rotate(&raw mut l.b.x, &raw mut l.b.y, angle);
        }
        l.b.x += x;
        l.b.y += y;
        AM_drawMline(&raw mut l, color);
        i += 1;
    }
}
pub unsafe fn AM_drawPlayers() {
    let mut i: i32 = 0;
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    const their_colors: [i32; 4] = [GREENS, GRAYS, BROWNS, REDS];
    let mut their_color: i32 = -(1 as i32);
    let mut color: i32 = 0;
    if !unsafe { game_state() }.g_game.netgame {
        if unsafe { game_state() }.am_map.cheating != 0 {
            AM_drawLineCharacter(
                &raw const cheat_player_arrow as *mut mline_t,
                (::core::mem::size_of::<[mline_t; 16]>() as usize)
                    .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                    as i32,
                0 as fixed_t,
                (*(*unsafe { game_state() }.am_map.plr).mo).angle,
                WHITE,
                (*(*unsafe { game_state() }.am_map.plr).mo).x,
                (*(*unsafe { game_state() }.am_map.plr).mo).y,
            );
        } else {
            AM_drawLineCharacter(
                &raw const player_arrow as *mut mline_t,
                (::core::mem::size_of::<[mline_t; 7]>() as usize)
                    .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                    as i32,
                0 as fixed_t,
                (*(*unsafe { game_state() }.am_map.plr).mo).angle,
                WHITE,
                (*(*unsafe { game_state() }.am_map.plr).mo).x,
                (*(*unsafe { game_state() }.am_map.plr).mo).y,
            );
        }
        return;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        their_color += 1;
        p = (&raw mut unsafe { game_state() }.g_game.players as *mut player_t).offset(i as isize)
            as *mut player_t;
        if !(unsafe { game_state() }.g_game.deathmatch != 0
            && !unsafe { game_state() }.g_game.singledemo
            && p != unsafe { game_state() }.am_map.plr)
        {
            if !(unsafe { game_state() }.g_game.playeringame[i as usize] == 0) {
                if (*p).powers[pw_invisibility as i32 as usize] != 0 {
                    color = 246 as i32;
                } else {
                    color = their_colors[their_color as usize];
                }
                AM_drawLineCharacter(
                    &raw const player_arrow as *mut mline_t,
                    (::core::mem::size_of::<[mline_t; 7]>() as usize)
                        .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                        as i32,
                    0 as fixed_t,
                    (*(*p).mo).angle,
                    color,
                    (*(*p).mo).x,
                    (*(*p).mo).y,
                );
            }
        }
        i += 1;
    }
}
pub unsafe fn AM_drawThings(mut colors: i32, mut colorrange: i32) {
    let mut i: i32 = 0;
    let mut t: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    i = 0 as i32;
    while i < numsectors {
        t = (*sectors.offset(i as isize)).thinglist;
        while !t.is_null() {
            AM_drawLineCharacter(
                &raw const thintriangle_guy as *mut mline_t,
                (::core::mem::size_of::<[mline_t; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                    as i32,
                (16 as fixed_t) << FRACBITS,
                (*t).angle,
                colors + unsafe { game_state() }.am_map.lightlev,
                (*t).x,
                (*t).y,
            );
            t = (*t).snext as *mut mobj_t;
        }
        i += 1;
    }
}
pub unsafe fn AM_drawMarks() {
    let mut i: i32 = 0;
    let mut fx: i32 = 0;
    let mut fy: i32 = 0;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    i = 0 as i32;
    while i < AM_NUMMARKPOINTS {
        if unsafe { game_state() }.am_map.markpoints[i as usize].x != -(1 as i32) {
            w = 5 as i32;
            h = 6 as i32;
            fx = (unsafe { game_state() }.am_map.f_x as fixed_t
                + (FixedMul(unsafe { game_state() }.am_map.markpoints[i as usize].x - unsafe { game_state() }.am_map.m_x, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32))
                as i32;
            fy = (unsafe { game_state() }.am_map.f_y as fixed_t
                + (unsafe { game_state() }.am_map.f_h as fixed_t
                    - (FixedMul(unsafe { game_state() }.am_map.markpoints[i as usize].y - unsafe { game_state() }.am_map.m_y, unsafe { game_state() }.am_map.scale_mtof) >> 16 as i32)))
                as i32;
            if fx >= unsafe { game_state() }.am_map.f_x && fx <= unsafe { game_state() }.am_map.f_w - w && fy >= unsafe { game_state() }.am_map.f_y && fy <= unsafe { game_state() }.am_map.f_h - h {
                V_DrawPatch(
                    unsafe { &mut game_state().v_video },
                    fx,
                    fy,
                    unsafe { game_state() }.am_map.marknums[i as usize],
                );
            }
        }
        i += 1;
    }
}
pub unsafe fn AM_drawCrosshair(mut color: i32) {
    *unsafe { game_state() }.am_map.fb.offset((unsafe { game_state() }.am_map.f_w * (unsafe { game_state() }.am_map.f_h + 1 as i32) / 2 as i32) as isize) = color as byte;
}
pub unsafe fn AM_Drawer() {
    if !unsafe { game_state() }.am_map.automapactive {
        return;
    }
    AM_clearFB(BACKGROUND);
    if unsafe { game_state() }.am_map.grid != 0 {
        AM_drawGrid(GRIDCOLORS);
    }
    AM_drawWalls();
    AM_drawPlayers();
    if unsafe { game_state() }.am_map.cheating == 2 as i32 {
        AM_drawThings(THINGCOLORS, THINGRANGE);
    }
    AM_drawCrosshair(XHAIRCOLORS);
    AM_drawMarks();
    V_MarkRect(unsafe { &mut game_state().v_video }, unsafe { game_state() }.am_map.f_x, unsafe { game_state() }.am_map.f_y, unsafe { game_state() }.am_map.f_w, unsafe { game_state() }.am_map.f_h);
}
unsafe extern "C" fn run_static_initializers() {
    unsafe { game_state() }.am_map.cheat_amap = cheatseq_t {
        sequence: ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(
            *b"iddt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        sequence_len: (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
            .wrapping_sub(1 as size_t),
        parameter_chars: 0 as i32,
        chars_read: 0 as size_t,
        param_chars_read: 0 as i32,
        parameter_buf: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\0\0\0\0\0"),
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
