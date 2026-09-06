use crate::src::d_event::event_t;
use crate::src::d_event::{ev_keydown, ev_keyup};
use crate::src::d_player::player_t;
use crate::src::d_player::{pw_allmap, pw_invisibility};
use crate::src::doomdef::false_0;
use crate::src::doomdef::true_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::deathmatch;
use crate::src::g_game::gameepisode;
use crate::src::g_game::gamemap;
use crate::src::g_game::netgame;
use crate::src::g_game::playeringame;
use crate::src::g_game::players;
use crate::src::g_game::singledemo;
use crate::src::g_game::viewactive;
use crate::src::hu_lib::patch_t;
use crate::src::i_system::{fprintf, stderr};
use crate::src::i_video::I_VideoBuffer;
use crate::src::m_cheat::cheatseq_t;
use crate::src::m_cheat::cht_CheckCheat;
use crate::src::m_controls::key_map_clearmark;
use crate::src::m_controls::key_map_east;
use crate::src::m_controls::key_map_follow;
use crate::src::m_controls::key_map_grid;
use crate::src::m_controls::key_map_mark;
use crate::src::m_controls::key_map_maxzoom;
use crate::src::m_controls::key_map_north;
use crate::src::m_controls::key_map_south;
use crate::src::m_controls::key_map_toggle;
use crate::src::m_controls::key_map_west;
use crate::src::m_controls::key_map_zoomin;
use crate::src::m_controls::key_map_zoomout;
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
pub static mut player_arrow: [mline_t; 7] = [
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
pub static mut cheat_player_arrow: [mline_t; 16] = [
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
pub static mut triangle_guy: [mline_t; 3] = [
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
pub static mut thintriangle_guy: [mline_t; 3] = [
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
static mut cheating: i32 = 0;
static mut grid: i32 = 0;
static mut leveljuststarted: i32 = 1;
pub static mut automapactive: bool = false;
static mut finit_width: i32 = SCREENWIDTH;
static mut finit_height: i32 = SCREENHEIGHT - 32;
static mut f_x: i32 = 0;
static mut f_y: i32 = 0;
static mut f_w: i32 = 0;
static mut f_h: i32 = 0;
static mut lightlev: i32 = 0;
static mut fb: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
static mut amclock: i32 = 0;
static mut m_paninc: mpoint_t = mpoint_t { x: 0, y: 0 };
static mut mtof_zoommul: fixed_t = 0;
static mut ftom_zoommul: fixed_t = 0;
static mut m_y: fixed_t = 0;
static mut m_x: fixed_t = 0;
static mut m_x2: fixed_t = 0;
static mut m_y2: fixed_t = 0;
static mut m_w: fixed_t = 0;
static mut m_h: fixed_t = 0;
static mut min_x: fixed_t = 0;
static mut min_y: fixed_t = 0;
static mut max_x: fixed_t = 0;
static mut max_y: fixed_t = 0;
static mut max_w: fixed_t = 0;
static mut max_h: fixed_t = 0;
static mut min_w: fixed_t = 0;
static mut min_h: fixed_t = 0;
static mut min_scale_mtof: fixed_t = 0;
static mut max_scale_mtof: fixed_t = 0;
static mut old_m_h: fixed_t = 0;
static mut old_m_w: fixed_t = 0;
static mut old_m_y: fixed_t = 0;
static mut old_m_x: fixed_t = 0;
static mut f_oldloc: mpoint_t = mpoint_t { x: 0, y: 0 };
static mut scale_mtof: fixed_t = INITSCALEMTOF as fixed_t;
static mut scale_ftom: fixed_t = 0;
static mut plr: *mut player_t = ::core::ptr::null::<player_t>() as *mut player_t;
static mut marknums: [*mut patch_t; 10] = [::core::ptr::null::<patch_t>() as *mut patch_t; 10];
static mut markpoints: [mpoint_t; 10] = [mpoint_t { x: 0, y: 0 }; 10];
static mut markpointnum: i32 = 0;
static mut followplayer: i32 = 1;
#[no_mangle]
pub static mut cheat_amap: cheatseq_t = cheatseq_t {
    sequence: [0; 25],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};
static mut stopped: bool = true;
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
    m_x += m_w as i32 / 2 as i32;
    m_y += m_h as i32 / 2 as i32;
    m_w = FixedMul((f_w as fixed_t) << 16 as i32, scale_ftom);
    m_h = FixedMul((f_h as fixed_t) << 16 as i32, scale_ftom);
    m_x -= m_w as i32 / 2 as i32;
    m_y -= m_h as i32 / 2 as i32;
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
}
pub unsafe fn AM_saveScaleAndLoc() {
    old_m_x = m_x;
    old_m_y = m_y;
    old_m_w = m_w;
    old_m_h = m_h;
}
pub unsafe fn AM_restoreScaleAndLoc() {
    m_w = old_m_w;
    m_h = old_m_h;
    if followplayer == 0 {
        m_x = old_m_x;
        m_y = old_m_y;
    } else {
        m_x = ((*(*plr).mo).x as i32 - m_w as i32 / 2 as i32) as fixed_t;
        m_y = ((*(*plr).mo).y as i32 - m_h as i32 / 2 as i32) as fixed_t;
    }
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
    scale_mtof = FixedDiv((f_w as fixed_t) << FRACBITS, m_w);
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
}
pub unsafe fn AM_addMark() {
    markpoints[markpointnum as usize].x = (m_x as i32 + m_w as i32 / 2 as i32) as fixed_t;
    markpoints[markpointnum as usize].y = (m_y as i32 + m_h as i32 / 2 as i32) as fixed_t;
    markpointnum = (markpointnum + 1 as i32) % AM_NUMMARKPOINTS;
}
pub unsafe fn AM_findMinMaxBoundaries() {
    let mut i: i32 = 0;
    let mut a: fixed_t = 0;
    let mut b: fixed_t = 0;
    min_y = INT_MAX as fixed_t;
    min_x = min_y;
    max_y = -INT_MAX as fixed_t;
    max_x = max_y;
    i = 0 as i32;
    while i < numvertexes {
        if (*vertexes.offset(i as isize)).x < min_x {
            min_x = (*vertexes.offset(i as isize)).x;
        } else if (*vertexes.offset(i as isize)).x > max_x {
            max_x = (*vertexes.offset(i as isize)).x;
        }
        if (*vertexes.offset(i as isize)).y < min_y {
            min_y = (*vertexes.offset(i as isize)).y;
        } else if (*vertexes.offset(i as isize)).y > max_y {
            max_y = (*vertexes.offset(i as isize)).y;
        }
        i += 1;
    }
    max_w = max_x - min_x;
    max_h = max_y - min_y;
    min_w = (2 as i32 * 16 as i32 * FRACUNIT) as fixed_t;
    min_h = (2 as i32 * 16 as i32 * FRACUNIT) as fixed_t;
    a = FixedDiv((f_w as fixed_t) << FRACBITS, max_w);
    b = FixedDiv((f_h as fixed_t) << FRACBITS, max_h);
    min_scale_mtof = if a < b { a } else { b };
    max_scale_mtof = FixedDiv(
        (f_h as fixed_t) << FRACBITS,
        2 as fixed_t * 16 as fixed_t * FRACUNIT,
    );
}
pub unsafe fn AM_changeWindowLoc() {
    if m_paninc.x != 0 || m_paninc.y != 0 {
        followplayer = 0 as i32;
        f_oldloc.x = INT_MAX as fixed_t;
    }
    m_x += m_paninc.x;
    m_y += m_paninc.y;
    if m_x as i32 + m_w as i32 / 2 as i32 > max_x {
        m_x = (max_x as i32 - m_w as i32 / 2 as i32) as fixed_t;
    } else if (m_x as i32 + m_w as i32 / 2 as i32) < min_x {
        m_x = (min_x as i32 - m_w as i32 / 2 as i32) as fixed_t;
    }
    if m_y as i32 + m_h as i32 / 2 as i32 > max_y {
        m_y = (max_y as i32 - m_h as i32 / 2 as i32) as fixed_t;
    } else if (m_y as i32 + m_h as i32 / 2 as i32) < min_y {
        m_y = (min_y as i32 - m_h as i32 / 2 as i32) as fixed_t;
    }
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
}
pub unsafe fn AM_initVariables() {
    let mut pnum: i32 = 0;
    static mut st_notify: event_t = event_t {
        type_0: ev_keyup,
        data1: AM_MSGENTERED,
        data2: 0 as i32,
        data3: 0 as i32,
        data4: 0,
    };
    automapactive = true;
    fb = I_VideoBuffer;
    f_oldloc.x = INT_MAX as fixed_t;
    amclock = 0 as i32;
    lightlev = 0 as i32;
    m_paninc.y = 0 as i32 as fixed_t;
    m_paninc.x = m_paninc.y;
    ftom_zoommul = FRACUNIT as fixed_t;
    mtof_zoommul = FRACUNIT as fixed_t;
    m_w = FixedMul((f_w as fixed_t) << 16 as i32, scale_ftom);
    m_h = FixedMul((f_h as fixed_t) << 16 as i32, scale_ftom);
    if playeringame[consoleplayer as usize] != 0 {
        plr = (&raw mut players as *mut player_t).offset(consoleplayer as isize) as *mut player_t;
    } else {
        plr = (&raw mut players as *mut player_t).offset(0 as i32 as isize) as *mut player_t;
        pnum = 0 as i32;
        while pnum < MAXPLAYERS {
            if playeringame[pnum as usize] != 0 {
                plr = (&raw mut players as *mut player_t).offset(pnum as isize) as *mut player_t;
                break;
            } else {
                pnum += 1;
            }
        }
    }
    m_x = ((*(*plr).mo).x as i32 - m_w as i32 / 2 as i32) as fixed_t;
    m_y = ((*(*plr).mo).y as i32 - m_h as i32 / 2 as i32) as fixed_t;
    AM_changeWindowLoc();
    old_m_x = m_x;
    old_m_y = m_y;
    old_m_w = m_w;
    old_m_h = m_h;
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
        marknums[i as usize] = W_CacheLumpName(
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
        markpoints[i as usize].x = -(1 as i32) as fixed_t;
        i += 1;
    }
    markpointnum = 0 as i32;
}
pub unsafe fn AM_LevelInit() {
    leveljuststarted = 0 as i32;
    f_y = 0 as i32;
    f_x = f_y;
    f_w = finit_width;
    f_h = finit_height;
    AM_clearMarks();
    AM_findMinMaxBoundaries();
    scale_mtof = FixedDiv(min_scale_mtof, (0.7f64 * FRACUNIT as f64) as fixed_t);
    if scale_mtof > max_scale_mtof {
        scale_mtof = min_scale_mtof;
    }
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
}
pub unsafe fn AM_Stop() {
    static mut st_notify: event_t = event_t {
        type_0: ev_keydown,
        data1: ev_keyup as i32,
        data2: AM_MSGEXITED,
        data3: 0 as i32,
        data4: 0,
    };
    AM_unloadPics();
    automapactive = false;
    ST_Responder(&st_notify);
    stopped = true;
}
pub unsafe fn AM_Start() {
    static mut lastlevel: i32 = -1;
    static mut lastepisode: i32 = -1;
    if !stopped {
        AM_Stop();
    }
    stopped = false;
    if lastlevel != gamemap || lastepisode != gameepisode {
        AM_LevelInit();
        lastlevel = gamemap;
        lastepisode = gameepisode;
    }
    AM_initVariables();
    AM_loadPics();
}
pub unsafe fn AM_minOutWindowScale() {
    scale_mtof = min_scale_mtof;
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    AM_activateNewScale();
}
pub unsafe fn AM_maxOutWindowScale() {
    scale_mtof = max_scale_mtof;
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    AM_activateNewScale();
}
pub unsafe fn AM_Responder(mut ev: &event_t) -> bool {
    let mut rc: i32 = 0;
    static mut bigstate: i32 = 0;
    static mut buffer: [::core::ffi::c_char; 20] = [0; 20];
    let mut key: i32 = 0;
    rc = false_0;
    if !automapactive {
        if (*ev).type_0 as u32 == ev_keydown as i32 as u32 && (*ev).data1 == key_map_toggle {
            AM_Start();
            viewactive = false;
            rc = true_0;
        }
    } else if (*ev).type_0 as u32 == ev_keydown as i32 as u32 {
        rc = true_0;
        key = (*ev).data1;
        if key == key_map_east {
            if followplayer == 0 {
                m_paninc.x = FixedMul((4 as fixed_t) << 16 as i32, scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == key_map_west {
            if followplayer == 0 {
                m_paninc.x = -FixedMul((4 as fixed_t) << 16 as i32, scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == key_map_north {
            if followplayer == 0 {
                m_paninc.y = FixedMul((4 as fixed_t) << 16 as i32, scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == key_map_south {
            if followplayer == 0 {
                m_paninc.y = -FixedMul((4 as fixed_t) << 16 as i32, scale_ftom);
            } else {
                rc = false_0;
            }
        } else if key == key_map_zoomout {
            mtof_zoommul = M_ZOOMOUT as fixed_t;
            ftom_zoommul = M_ZOOMIN as fixed_t;
        } else if key == key_map_zoomin {
            mtof_zoommul = M_ZOOMIN as fixed_t;
            ftom_zoommul = M_ZOOMOUT as fixed_t;
        } else if key == key_map_toggle {
            bigstate = 0 as i32;
            viewactive = true;
            AM_Stop();
        } else if key == key_map_maxzoom {
            bigstate = (bigstate == 0) as i32;
            if bigstate != 0 {
                AM_saveScaleAndLoc();
                AM_minOutWindowScale();
            } else {
                AM_restoreScaleAndLoc();
            }
        } else if key == key_map_follow {
            followplayer = (followplayer == 0) as i32;
            f_oldloc.x = INT_MAX as fixed_t;
            if followplayer != 0 {
                (*plr).message = b"Follow Mode ON\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                (*plr).message = b"Follow Mode OFF\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        } else if key == key_map_grid {
            grid = (grid == 0) as i32;
            if grid != 0 {
                (*plr).message = b"Grid ON\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                (*plr).message = b"Grid OFF\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        } else if key == key_map_mark {
            M_snprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                b"%s %d\0" as *const u8 as *const ::core::ffi::c_char,
                b"Marked Spot\0" as *const u8 as *const ::core::ffi::c_char,
                markpointnum,
            );
            (*plr).message = &raw mut buffer as *mut ::core::ffi::c_char;
            AM_addMark();
        } else if key == key_map_clearmark {
            AM_clearMarks();
            (*plr).message = b"All Marks Cleared\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else {
            rc = false_0;
        }
        if deathmatch == 0
            && cht_CheckCheat(&raw mut cheat_amap, (*ev).data2 as ::core::ffi::c_char) != 0
        {
            rc = false_0;
            cheating = (cheating + 1 as i32) % 3 as i32;
        }
    } else if (*ev).type_0 as u32 == ev_keyup as i32 as u32 {
        rc = false_0;
        key = (*ev).data1;
        if key == key_map_east {
            if followplayer == 0 {
                m_paninc.x = 0 as i32 as fixed_t;
            }
        } else if key == key_map_west {
            if followplayer == 0 {
                m_paninc.x = 0 as i32 as fixed_t;
            }
        } else if key == key_map_north {
            if followplayer == 0 {
                m_paninc.y = 0 as i32 as fixed_t;
            }
        } else if key == key_map_south {
            if followplayer == 0 {
                m_paninc.y = 0 as i32 as fixed_t;
            }
        } else if key == key_map_zoomout || key == key_map_zoomin {
            mtof_zoommul = FRACUNIT as fixed_t;
            ftom_zoommul = FRACUNIT as fixed_t;
        }
    }
    return rc != 0;
}
pub unsafe fn AM_changeWindowScale() {
    scale_mtof = FixedMul(scale_mtof, mtof_zoommul);
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    if scale_mtof < min_scale_mtof {
        AM_minOutWindowScale();
    } else if scale_mtof > max_scale_mtof {
        AM_maxOutWindowScale();
    } else {
        AM_activateNewScale();
    };
}
pub unsafe fn AM_doFollowPlayer() {
    if f_oldloc.x != (*(*plr).mo).x || f_oldloc.y != (*(*plr).mo).y {
        m_x = (FixedMul(
            (FixedMul((*(*plr).mo).x, scale_mtof) >> 16 as i32) << 16 as i32,
            scale_ftom,
        ) as i32
            - m_w as i32 / 2 as i32) as fixed_t;
        m_y = (FixedMul(
            (FixedMul((*(*plr).mo).y, scale_mtof) >> 16 as i32) << 16 as i32,
            scale_ftom,
        ) as i32
            - m_h as i32 / 2 as i32) as fixed_t;
        m_x2 = m_x + m_w;
        m_y2 = m_y + m_h;
        f_oldloc.x = (*(*plr).mo).x;
        f_oldloc.y = (*(*plr).mo).y;
    }
}
pub unsafe fn AM_updateLightLev() {
    static mut nexttic: i32 = 0;
    static mut litelevels: [i32; 8] = [
        0 as i32, 4 as i32, 7 as i32, 10 as i32, 12 as i32, 14 as i32, 15 as i32, 15 as i32,
    ];
    static mut litelevelscnt: i32 = 0;
    if amclock > nexttic {
        let fresh1 = litelevelscnt;
        litelevelscnt = litelevelscnt + 1;
        lightlev = litelevels[fresh1 as usize];
        if litelevelscnt as usize
            == (::core::mem::size_of::<[i32; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<i32>() as usize)
        {
            litelevelscnt = 0 as i32;
        }
        nexttic = amclock + 6 as i32 - amclock % 6 as i32;
    }
}
pub unsafe fn AM_Ticker() {
    if !automapactive {
        return;
    }
    amclock += 1;
    if followplayer != 0 {
        AM_doFollowPlayer();
    }
    if ftom_zoommul != FRACUNIT {
        AM_changeWindowScale();
    }
    if m_paninc.x != 0 || m_paninc.y != 0 {
        AM_changeWindowLoc();
    }
}
pub unsafe fn AM_clearFB(mut color: i32) {
    memset(fb as *mut ::core::ffi::c_void, color, (f_w * f_h) as size_t);
}
pub unsafe fn AM_clipMline(mut ml: *mut mline_t, mut fl: *mut fline_t) -> bool {
    let mut outcode1: i32 = 0 as i32;
    let mut outcode2: i32 = 0 as i32;
    let mut outside: i32 = 0;
    let mut tmp: fpoint_t = fpoint_t { x: 0, y: 0 };
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    if (*ml).a.y > m_y2 {
        outcode1 = TOP as i32;
    } else if (*ml).a.y < m_y {
        outcode1 = BOTTOM as i32;
    }
    if (*ml).b.y > m_y2 {
        outcode2 = TOP as i32;
    } else if (*ml).b.y < m_y {
        outcode2 = BOTTOM as i32;
    }
    if outcode1 & outcode2 != 0 {
        return false;
    }
    if (*ml).a.x < m_x {
        outcode1 |= LEFT as i32;
    } else if (*ml).a.x > m_x2 {
        outcode1 |= RIGHT as i32;
    }
    if (*ml).b.x < m_x {
        outcode2 |= LEFT as i32;
    } else if (*ml).b.x > m_x2 {
        outcode2 |= RIGHT as i32;
    }
    if outcode1 & outcode2 != 0 {
        return false;
    }
    (*fl).a.x = (f_x as fixed_t + (FixedMul((*ml).a.x - m_x, scale_mtof) >> 16 as i32)) as i32;
    (*fl).a.y = (f_y as fixed_t
        + (f_h as fixed_t - (FixedMul((*ml).a.y - m_y, scale_mtof) >> 16 as i32)))
        as i32;
    (*fl).b.x = (f_x as fixed_t + (FixedMul((*ml).b.x - m_x, scale_mtof) >> 16 as i32)) as i32;
    (*fl).b.y = (f_y as fixed_t
        + (f_h as fixed_t - (FixedMul((*ml).b.y - m_y, scale_mtof) >> 16 as i32)))
        as i32;
    outcode1 = 0 as i32;
    if (*fl).a.y < 0 as i32 {
        outcode1 |= TOP as i32;
    } else if (*fl).a.y >= f_h {
        outcode1 |= BOTTOM as i32;
    }
    if (*fl).a.x < 0 as i32 {
        outcode1 |= LEFT as i32;
    } else if (*fl).a.x >= f_w {
        outcode1 |= RIGHT as i32;
    }
    outcode2 = 0 as i32;
    if (*fl).b.y < 0 as i32 {
        outcode2 |= TOP as i32;
    } else if (*fl).b.y >= f_h {
        outcode2 |= BOTTOM as i32;
    }
    if (*fl).b.x < 0 as i32 {
        outcode2 |= LEFT as i32;
    } else if (*fl).b.x >= f_w {
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
            tmp.x = (*fl).a.x + dx * ((*fl).a.y - f_h) / dy;
            tmp.y = f_h - 1 as i32;
        } else if outside & RIGHT as i32 != 0 {
            dy = (*fl).b.y - (*fl).a.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + dy * (f_w - 1 as i32 - (*fl).a.x) / dx;
            tmp.x = f_w - 1 as i32;
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
            } else if (*fl).a.y >= f_h {
                outcode1 |= BOTTOM as i32;
            }
            if (*fl).a.x < 0 as i32 {
                outcode1 |= LEFT as i32;
            } else if (*fl).a.x >= f_w {
                outcode1 |= RIGHT as i32;
            }
        } else {
            (*fl).b = tmp;
            outcode2 = 0 as i32;
            if (*fl).b.y < 0 as i32 {
                outcode2 |= TOP as i32;
            } else if (*fl).b.y >= f_h {
                outcode2 |= BOTTOM as i32;
            }
            if (*fl).b.x < 0 as i32 {
                outcode2 |= LEFT as i32;
            } else if (*fl).b.x >= f_w {
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
    static mut fuck: i32 = 0;
    if (*fl).a.x < 0 as i32
        || (*fl).a.x >= f_w
        || (*fl).a.y < 0 as i32
        || (*fl).a.y >= f_h
        || (*fl).b.x < 0 as i32
        || (*fl).b.x >= f_w
        || (*fl).b.y < 0 as i32
        || (*fl).b.y >= f_h
    {
        let fresh0 = fuck;
        fuck = fuck + 1;
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
            *fb.offset((y * f_w + x) as isize) = color as byte;
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
            *fb.offset((y * f_w + x) as isize) = color as byte;
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
    static mut fl: fline_t = fline_t {
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
    start = m_x;
    if (start as i32 - bmaporgx as i32) % (MAPBLOCKUNITS << FRACBITS) != 0 {
        start += (MAPBLOCKUNITS << FRACBITS)
            - (start as i32 - bmaporgx as i32) % (MAPBLOCKUNITS << FRACBITS);
    }
    end = m_x + m_w;
    ml.a.y = m_y;
    ml.b.y = m_y + m_h;
    x = start;
    while x < end {
        ml.a.x = x;
        ml.b.x = x;
        AM_drawMline(&raw mut ml, color);
        x += MAPBLOCKUNITS << FRACBITS;
    }
    start = m_y;
    if (start as i32 - bmaporgy as i32) % (MAPBLOCKUNITS << FRACBITS) != 0 {
        start += (MAPBLOCKUNITS << FRACBITS)
            - (start as i32 - bmaporgy as i32) % (MAPBLOCKUNITS << FRACBITS);
    }
    end = m_y + m_h;
    ml.a.x = m_x;
    ml.b.x = m_x + m_w;
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
    static mut l: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };
    i = 0 as i32;
    while i < numlines {
        l.a.x = (*(*lines.offset(i as isize)).v1).x;
        l.a.y = (*(*lines.offset(i as isize)).v1).y;
        l.b.x = (*(*lines.offset(i as isize)).v2).x;
        l.b.y = (*(*lines.offset(i as isize)).v2).y;
        if cheating != 0 || (*lines.offset(i as isize)).flags as i32 & ML_MAPPED != 0 {
            if !((*lines.offset(i as isize)).flags as i32 & LINE_NEVERSEE != 0 && cheating == 0) {
                if (*lines.offset(i as isize)).backsector.is_null() {
                    AM_drawMline(&raw mut l, WALLCOLORS + lightlev);
                } else if (*lines.offset(i as isize)).special as i32 == 39 as i32 {
                    AM_drawMline(&raw mut l, WALLCOLORS + WALLRANGE / 2 as i32);
                } else if (*lines.offset(i as isize)).flags as i32 & ML_SECRET != 0 {
                    if cheating != 0 {
                        AM_drawMline(&raw mut l, SECRETWALLCOLORS + lightlev);
                    } else {
                        AM_drawMline(&raw mut l, WALLCOLORS + lightlev);
                    }
                } else if (*(*lines.offset(i as isize)).backsector).floorheight
                    != (*(*lines.offset(i as isize)).frontsector).floorheight
                {
                    AM_drawMline(&raw mut l, FDWALLCOLORS + lightlev);
                } else if (*(*lines.offset(i as isize)).backsector).ceilingheight
                    != (*(*lines.offset(i as isize)).frontsector).ceilingheight
                {
                    AM_drawMline(&raw mut l, CDWALLCOLORS + lightlev);
                } else if cheating != 0 {
                    AM_drawMline(&raw mut l, TSWALLCOLORS + lightlev);
                }
            }
        } else if (*plr).powers[pw_allmap as i32 as usize] != 0 {
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
    static mut their_colors: [i32; 4] = [GREENS, GRAYS, BROWNS, REDS];
    let mut their_color: i32 = -(1 as i32);
    let mut color: i32 = 0;
    if !netgame {
        if cheating != 0 {
            AM_drawLineCharacter(
                &raw mut cheat_player_arrow as *mut mline_t,
                (::core::mem::size_of::<[mline_t; 16]>() as usize)
                    .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                    as i32,
                0 as fixed_t,
                (*(*plr).mo).angle,
                WHITE,
                (*(*plr).mo).x,
                (*(*plr).mo).y,
            );
        } else {
            AM_drawLineCharacter(
                &raw mut player_arrow as *mut mline_t,
                (::core::mem::size_of::<[mline_t; 7]>() as usize)
                    .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                    as i32,
                0 as fixed_t,
                (*(*plr).mo).angle,
                WHITE,
                (*(*plr).mo).x,
                (*(*plr).mo).y,
            );
        }
        return;
    }
    i = 0 as i32;
    while i < MAXPLAYERS {
        their_color += 1;
        p = (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t;
        if !(deathmatch != 0 && !singledemo && p != plr) {
            if !(playeringame[i as usize] == 0) {
                if (*p).powers[pw_invisibility as i32 as usize] != 0 {
                    color = 246 as i32;
                } else {
                    color = their_colors[their_color as usize];
                }
                AM_drawLineCharacter(
                    &raw mut player_arrow as *mut mline_t,
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
                &raw mut thintriangle_guy as *mut mline_t,
                (::core::mem::size_of::<[mline_t; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<mline_t>() as usize)
                    as i32,
                (16 as fixed_t) << FRACBITS,
                (*t).angle,
                colors + lightlev,
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
        if markpoints[i as usize].x != -(1 as i32) {
            w = 5 as i32;
            h = 6 as i32;
            fx = (f_x as fixed_t
                + (FixedMul(markpoints[i as usize].x - m_x, scale_mtof) >> 16 as i32))
                as i32;
            fy = (f_y as fixed_t
                + (f_h as fixed_t
                    - (FixedMul(markpoints[i as usize].y - m_y, scale_mtof) >> 16 as i32)))
                as i32;
            if fx >= f_x && fx <= f_w - w && fy >= f_y && fy <= f_h - h {
                V_DrawPatch(fx, fy, marknums[i as usize]);
            }
        }
        i += 1;
    }
}
pub unsafe fn AM_drawCrosshair(mut color: i32) {
    *fb.offset((f_w * (f_h + 1 as i32) / 2 as i32) as isize) = color as byte;
}
pub unsafe fn AM_Drawer() {
    if !automapactive {
        return;
    }
    AM_clearFB(BACKGROUND);
    if grid != 0 {
        AM_drawGrid(GRIDCOLORS);
    }
    AM_drawWalls();
    AM_drawPlayers();
    if cheating == 2 as i32 {
        AM_drawThings(THINGCOLORS, THINGRANGE);
    }
    AM_drawCrosshair(XHAIRCOLORS);
    AM_drawMarks();
    V_MarkRect(f_x, f_y, f_w, f_h);
}
unsafe extern "C" fn run_static_initializers() {
    cheat_amap = cheatseq_t {
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
