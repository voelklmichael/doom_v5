use crate::src::d_loop::NetUpdate;
use crate::src::d_player::player_t;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::game_state::game_state;
use crate::src::game_state::GameState;
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::src::m_fixed::fixed_t;
use crate::src::m_fixed::FixedDiv;
use crate::src::m_fixed::FixedMul;
use crate::src::m_fixed::FRACBITS;
use crate::src::m_fixed::FRACUNIT;
use crate::src::p_mobj::subsector_t;
use crate::src::p_setup::SubsectorId;
use crate::src::r_bsp::R_ClearClipSegs;
use crate::src::r_bsp::R_ClearDrawSegs;
use crate::src::r_bsp::R_RenderBSPNode;
use crate::src::r_bsp::NF_SUBSECTOR;
use crate::src::r_data::R_InitData;
use crate::src::r_defs::lighttable_t;
use crate::src::r_defs::{node_t, seg_t};
use crate::src::r_draw::R_InitBuffer;
use crate::src::r_draw::R_InitTranslationTables;
use crate::src::r_draw::{R_DrawColumn, R_DrawColumnLow, R_DrawFuzzColumn, R_DrawFuzzColumnLow, R_DrawSpan, R_DrawSpanLow, R_DrawTranslatedColumn, R_DrawTranslatedColumnLow};
use crate::src::r_plane::R_ClearPlanes;
use crate::src::r_plane::R_DrawPlanes;
use crate::src::r_sky::R_InitSkyMap;
use crate::src::r_things::R_ClearSprites;
use crate::src::r_things::R_DrawMasked;
use crate::src::tables::angle_t;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::tables::finetangent;
use crate::src::tables::tantoangle;
use crate::src::tables::SlopeDiv;
use crate::src::tables::ANG180;
use crate::src::tables::ANG270;
use crate::src::tables::ANG90;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::FINEANGLES;
use libc::printf;

pub struct RMainState {
    pub viewangleoffset: i32,
    pub validcount: i32,
    pub fixedcolormap: *mut lighttable_t,
    pub centerx: i32,
    pub centery: i32,
    pub centerxfrac: fixed_t,
    pub centeryfrac: fixed_t,
    pub projection: fixed_t,
    pub framecount: i32,
    pub sscount: i32,
    pub linecount: i32,
    pub loopcount: i32,
    pub viewx: fixed_t,
    pub viewy: fixed_t,
    pub viewz: fixed_t,
    pub viewangle: angle_t,
    pub viewcos: fixed_t,
    pub viewsin: fixed_t,
    pub viewplayer: *mut player_t,
    pub detailshift: i32,
    pub clipangle: angle_t,
    pub viewangletox: [i32; 4096],
    pub xtoviewangle: [angle_t; 321],
    pub scalelight: [[*mut lighttable_t; 48]; 16],
    pub scalelightfixed: [*mut lighttable_t; 48],
    pub zlight: [[*mut lighttable_t; 128]; 16],
    pub extralight: i32,
    pub colfunc: Option<unsafe fn() -> ()>,
    pub basecolfunc: Option<unsafe fn() -> ()>,
    pub fuzzcolfunc: Option<unsafe fn() -> ()>,
    pub transcolfunc: Option<unsafe fn() -> ()>,
    pub spanfunc: Option<unsafe fn() -> ()>,
    pub setsizeneeded: bool,
    pub setblocks: i32,
    pub setdetail: i32,
}

impl RMainState {
    pub const fn new() -> Self {
        RMainState {
            viewangleoffset: 0,
            validcount: 1,
            fixedcolormap: 
        ::core::ptr::null::<lighttable_t>() as *mut lighttable_t,
            centerx: 0,
            centery: 0,
            centerxfrac: 0,
            centeryfrac: 0,
            projection: 0,
            framecount: 0,
            sscount: 0,
            linecount: 0,
            loopcount: 0,
            viewx: 0,
            viewy: 0,
            viewz: 0,
            viewangle: 0,
            viewcos: 0,
            viewsin: 0,
            viewplayer: ::core::ptr::null::<player_t>() as *mut player_t,
            detailshift: 0,
            clipangle: 0,
            viewangletox: [0; 4096],
            xtoviewangle: [0; 321],
            scalelight: [[::core::ptr::null::<lighttable_t>() as *mut lighttable_t; 48]; 16],
            scalelightfixed: [::core::ptr::null::<lighttable_t>() as *mut lighttable_t; 48],
            zlight: [[::core::ptr::null::<lighttable_t>() as *mut lighttable_t; 128]; 16],
            extralight: 0,
            colfunc: None,
            basecolfunc: None,
            fuzzcolfunc: None,
            transcolfunc: None,
            spanfunc: None,
            setsizeneeded: false,
            setblocks: 0,
            setdetail: 0,
        }
    }
}

pub const SLOPEBITS: i32 = 11;
pub const DBITS: i32 = FRACBITS - SLOPEBITS;
pub const FIELDOFVIEW: i32 = 2048;
pub unsafe fn R_AddPointToBox(mut x: i32, mut y: i32, mut box_0: *mut fixed_t) {
    if x < *box_0.offset(BOXLEFT as i32 as isize) {
        *box_0.offset(BOXLEFT as i32 as isize) = x as fixed_t;
    }
    if x > *box_0.offset(BOXRIGHT as i32 as isize) {
        *box_0.offset(BOXRIGHT as i32 as isize) = x as fixed_t;
    }
    if y < *box_0.offset(BOXBOTTOM as i32 as isize) {
        *box_0.offset(BOXBOTTOM as i32 as isize) = y as fixed_t;
    }
    if y > *box_0.offset(BOXTOP as i32 as isize) {
        *box_0.offset(BOXTOP as i32 as isize) = y as fixed_t;
    }
}
pub unsafe fn R_PointOnSide(mut x: fixed_t, mut y: fixed_t, mut node: *mut node_t) -> i32 {
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut left: fixed_t = 0;
    let mut right: fixed_t = 0;
    if (*node).dx == 0 {
        if x <= (*node).x {
            return ((*node).dy > 0 as i32) as i32;
        }
        return ((*node).dy < 0 as i32) as i32;
    }
    if (*node).dy == 0 {
        if y <= (*node).y {
            return ((*node).dx < 0 as i32) as i32;
        }
        return ((*node).dx > 0 as i32) as i32;
    }
    dx = x - (*node).x;
    dy = y - (*node).y;
    if ((*node).dy ^ (*node).dx ^ dx ^ dy) as u32 & 0x80000000 as u32 != 0 {
        if ((*node).dy ^ dx) as u32 & 0x80000000 as u32 != 0 {
            return 1 as i32;
        }
        return 0 as i32;
    }
    left = FixedMul((*node).dy >> FRACBITS, dx);
    right = FixedMul(dy, (*node).dx >> FRACBITS);
    if right < left {
        return 0 as i32;
    }
    return 1 as i32;
}
pub unsafe fn R_PointOnSegSide(mut x: fixed_t, mut y: fixed_t, mut line: *mut seg_t) -> i32 {
    let mut lx: fixed_t = 0;
    let mut ly: fixed_t = 0;
    let mut ldx: fixed_t = 0;
    let mut ldy: fixed_t = 0;
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut left: fixed_t = 0;
    let mut right: fixed_t = 0;
    lx = (*(*line).v1).x;
    ly = (*(*line).v1).y;
    ldx = (*(*line).v2).x - lx;
    ldy = (*(*line).v2).y - ly;
    if ldx == 0 {
        if x <= lx {
            return (ldy > 0 as i32) as i32;
        }
        return (ldy < 0 as i32) as i32;
    }
    if ldy == 0 {
        if y <= ly {
            return (ldx < 0 as i32) as i32;
        }
        return (ldx > 0 as i32) as i32;
    }
    dx = x - lx;
    dy = y - ly;
    if (ldy ^ ldx ^ dx ^ dy) as u32 & 0x80000000 as u32 != 0 {
        if (ldy ^ dx) as u32 & 0x80000000 as u32 != 0 {
            return 1 as i32;
        }
        return 0 as i32;
    }
    left = FixedMul(ldy >> FRACBITS, dx);
    right = FixedMul(dy, ldx >> FRACBITS);
    if right < left {
        return 0 as i32;
    }
    return 1 as i32;
}
pub unsafe fn R_PointToAngle(mut x: fixed_t, mut y: fixed_t) -> angle_t {
    x -= unsafe { game_state() }.r_main.viewx;
    y -= unsafe { game_state() }.r_main.viewy;
    if x == 0 && y == 0 {
        return 0 as angle_t;
    }
    if x >= 0 as i32 {
        if y >= 0 as i32 {
            if x > y {
                return tantoangle[SlopeDiv(y as u32, x as u32) as usize];
            } else {
                return ((ANG90 - 1 as i32) as angle_t)
                    .wrapping_sub(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        } else {
            y = -y;
            if x > y {
                return tantoangle[SlopeDiv(y as u32, x as u32) as usize].wrapping_neg();
            } else {
                return ANG270.wrapping_add(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        }
    } else {
        x = -x;
        if y >= 0 as i32 {
            if x > y {
                return ANG180
                    .wrapping_sub(1 as angle_t)
                    .wrapping_sub(tantoangle[SlopeDiv(y as u32, x as u32) as usize]);
            } else {
                return (ANG90 as angle_t)
                    .wrapping_add(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        } else {
            y = -y;
            if x > y {
                return ANG180.wrapping_add(tantoangle[SlopeDiv(y as u32, x as u32) as usize]);
            } else {
                return ANG270
                    .wrapping_sub(1 as angle_t)
                    .wrapping_sub(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        }
    };
}
pub unsafe fn R_PointToAngle2(
    mut x1: fixed_t,
    mut y1: fixed_t,
    mut x2: fixed_t,
    mut y2: fixed_t,
) -> angle_t {
    unsafe { game_state() }.r_main.viewx = x1;
    unsafe { game_state() }.r_main.viewy = y1;
    return R_PointToAngle(x2, y2);
}
pub unsafe fn R_PointToDist(mut x: fixed_t, mut y: fixed_t) -> fixed_t {
    let mut angle: i32 = 0;
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut temp: fixed_t = 0;
    let mut dist: fixed_t = 0;
    let mut frac: fixed_t = 0;
    dx = (x as i32 - unsafe { game_state() }.r_main.viewx as i32).abs() as fixed_t;
    dy = (y as i32 - unsafe { game_state() }.r_main.viewy as i32).abs() as fixed_t;
    if dy > dx {
        temp = dx;
        dx = dy;
        dy = temp;
    }
    if dx != 0 as i32 {
        frac = FixedDiv(dy, dx);
    } else {
        frac = 0 as i32 as fixed_t;
    }
    angle = (tantoangle[(frac >> DBITS) as usize].wrapping_add(ANG90 as angle_t)
        >> ANGLETOFINESHIFT) as i32;
    dist = FixedDiv(dx, finesine[angle as usize]);
    return dist;
}
pub unsafe fn R_ScaleFromGlobalAngle(mut visangle: angle_t) -> fixed_t {
    let mut scale: fixed_t = 0;
    let mut anglea: angle_t = 0;
    let mut angleb: angle_t = 0;
    let mut sinea: i32 = 0;
    let mut sineb: i32 = 0;
    let mut num: fixed_t = 0;
    let mut den: i32 = 0;
    anglea = (ANG90 as angle_t).wrapping_add(visangle.wrapping_sub(unsafe { game_state() }.r_main.viewangle));
    angleb = (ANG90 as angle_t).wrapping_add(visangle.wrapping_sub(unsafe { game_state() }.r_segs.rw_normalangle));
    sinea = finesine[(anglea >> ANGLETOFINESHIFT) as usize] as i32;
    sineb = finesine[(angleb >> ANGLETOFINESHIFT) as usize] as i32;
    num = FixedMul(unsafe { game_state() }.r_main.projection, sineb as fixed_t) << unsafe { game_state() }.r_main.detailshift;
    den = FixedMul(unsafe { game_state() }.r_segs.rw_distance, sinea as fixed_t) as i32;
    if den > num >> 16 as i32 {
        scale = FixedDiv(num, den as fixed_t);
        if scale > 64 as i32 * FRACUNIT {
            scale = (64 as i32 * FRACUNIT) as fixed_t;
        } else if scale < 256 as i32 {
            scale = 256 as i32 as fixed_t;
        }
    } else {
        scale = (64 as i32 * FRACUNIT) as fixed_t;
    }
    return scale;
}
pub unsafe fn R_InitTextureMapping() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut t: i32 = 0;
    let mut focallength: fixed_t = 0;
    focallength = FixedDiv(
        unsafe { game_state() }.r_main.centerxfrac,
        finetangent[(FINEANGLES / 4 as i32 + FIELDOFVIEW / 2 as i32) as usize],
    );
    i = 0 as i32;
    while i < FINEANGLES / 2 as i32 {
        if finetangent[i as usize] > FRACUNIT * 2 as i32 {
            t = -(1 as i32);
        } else if finetangent[i as usize] < -FRACUNIT * 2 as i32 {
            t = unsafe { game_state() }.r_draw.viewwidth + 1 as i32;
        } else {
            t = FixedMul(finetangent[i as usize], focallength) as i32;
            t = unsafe { game_state() }.r_main.centerxfrac as i32 - t + FRACUNIT - 1 as i32 >> FRACBITS;
            if t < -(1 as i32) {
                t = -(1 as i32);
            } else if t > unsafe { game_state() }.r_draw.viewwidth + 1 as i32 {
                t = unsafe { game_state() }.r_draw.viewwidth + 1 as i32;
            }
        }
        unsafe { game_state() }.r_main.viewangletox[i as usize] = t;
        i += 1;
    }
    x = 0 as i32;
    while x <= unsafe { game_state() }.r_draw.viewwidth {
        i = 0 as i32;
        while unsafe { game_state() }.r_main.viewangletox[i as usize] > x {
            i += 1;
        }
        unsafe { game_state() }.r_main.xtoviewangle[x as usize] = ((i << ANGLETOFINESHIFT) - ANG90) as angle_t;
        x += 1;
    }
    i = 0 as i32;
    while i < FINEANGLES / 2 as i32 {
        t = FixedMul(finetangent[i as usize], focallength) as i32;
        t = unsafe { game_state() }.r_main.centerx - t;
        if unsafe { game_state() }.r_main.viewangletox[i as usize] == -(1 as i32) {
            unsafe { game_state() }.r_main.viewangletox[i as usize] = 0 as i32;
        } else if unsafe { game_state() }.r_main.viewangletox[i as usize] == unsafe { game_state() }.r_draw.viewwidth + 1 as i32 {
            unsafe { game_state() }.r_main.viewangletox[i as usize] = unsafe { game_state() }.r_draw.viewwidth;
        }
        i += 1;
    }
    unsafe { game_state() }.r_main.clipangle = unsafe { game_state() }.r_main.xtoviewangle[0 as i32 as usize];
}
pub const DISTMAP: i32 = 2;
pub unsafe fn R_InitLightTables() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut level: i32 = 0;
    let mut startmap: i32 = 0;
    let mut scale: i32 = 0;
    i = 0 as i32;
    while i < LIGHTLEVELS {
        startmap = (LIGHTLEVELS - 1 as i32 - i) * 2 as i32 * NUMCOLORMAPS / LIGHTLEVELS;
        j = 0 as i32;
        while j < MAXLIGHTZ {
            scale = FixedDiv(
                SCREENWIDTH / 2 as fixed_t * FRACUNIT,
                (j as fixed_t + 1 as fixed_t) << LIGHTZSHIFT,
            ) as i32;
            scale >>= LIGHTSCALESHIFT;
            level = startmap - scale / DISTMAP;
            if level < 0 as i32 {
                level = 0 as i32;
            }
            if level >= NUMCOLORMAPS {
                level = NUMCOLORMAPS - 1 as i32;
            }
            unsafe { game_state() }.r_main.zlight[i as usize][j as usize] = unsafe { game_state() }.r_data.colormaps.offset((level * 256 as i32) as isize);
            j += 1;
        }
        i += 1;
    }
}
pub unsafe fn R_SetViewSize(mut blocks: i32, mut detail: i32) {
    unsafe { game_state() }.r_main.setsizeneeded = true;
    unsafe { game_state() }.r_main.setblocks = blocks;
    unsafe { game_state() }.r_main.setdetail = detail;
}
pub unsafe fn R_ExecuteSetViewSize(state: &mut GameState) {
    let mut cosadj: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut level: i32 = 0;
    let mut startmap: i32 = 0;
    state.r_main.setsizeneeded = false;
    if state.r_main.setblocks == 11 as i32 {
        state.r_draw.scaledviewwidth = SCREENWIDTH;
        state.r_draw.viewheight = SCREENHEIGHT;
    } else {
        state.r_draw.scaledviewwidth = state.r_main.setblocks * 32 as i32;
        state.r_draw.viewheight = state.r_main.setblocks * 168 as i32 / 10 as i32 & !(7 as i32);
    }
    state.r_main.detailshift = state.r_main.setdetail;
    state.r_draw.viewwidth = state.r_draw.scaledviewwidth >> state.r_main.detailshift;
    state.r_main.centery = state.r_draw.viewheight / 2 as i32;
    state.r_main.centerx = state.r_draw.viewwidth / 2 as i32;
    state.r_main.centerxfrac = (state.r_main.centerx << FRACBITS) as fixed_t;
    state.r_main.centeryfrac = (state.r_main.centery << FRACBITS) as fixed_t;
    state.r_main.projection = state.r_main.centerxfrac;
    if state.r_main.detailshift == 0 {
        state.r_main.basecolfunc = Some(R_DrawColumn as unsafe fn() -> ());
        state.r_main.colfunc = state.r_main.basecolfunc;
        state.r_main.fuzzcolfunc = Some(R_DrawFuzzColumn as unsafe fn() -> ());
        state.r_main.transcolfunc = Some(R_DrawTranslatedColumn as unsafe fn() -> ());
        state.r_main.spanfunc = Some(R_DrawSpan as unsafe fn() -> ());
    } else {
        state.r_main.basecolfunc = Some(R_DrawColumnLow as unsafe fn() -> ());
        state.r_main.colfunc = state.r_main.basecolfunc;
        state.r_main.fuzzcolfunc = Some(R_DrawFuzzColumnLow as unsafe fn() -> ());
        state.r_main.transcolfunc = Some(R_DrawTranslatedColumnLow as unsafe fn() -> ());
        state.r_main.spanfunc = Some(R_DrawSpanLow as unsafe fn() -> ());
    }
    let scaledviewwidth = state.r_draw.scaledviewwidth;
    let viewheight = state.r_draw.viewheight;
    R_InitBuffer(state, scaledviewwidth, viewheight);
    R_InitTextureMapping();
    state.r_things.pspritescale = (FRACUNIT * state.r_draw.viewwidth / SCREENWIDTH) as fixed_t;
    state.r_things.pspriteiscale =
        (FRACUNIT * SCREENWIDTH / state.r_draw.viewwidth) as fixed_t;
    i = 0 as i32;
    while i < state.r_draw.viewwidth {
        state.r_things.screenheightarray[i as usize] = state.r_draw.viewheight as i16;
        i += 1;
    }
    i = 0 as i32;
    while i < state.r_draw.viewheight {
        dy = (((i - state.r_draw.viewheight / 2 as i32) << FRACBITS) + FRACUNIT / 2 as i32) as fixed_t;
        dy = (dy as i32).abs() as fixed_t;
        state.r_plane.yslope[i as usize] = FixedDiv(
            ((state.r_draw.viewwidth as fixed_t) << state.r_main.detailshift) / 2 as fixed_t * FRACUNIT,
            dy,
        );
        i += 1;
    }
    i = 0 as i32;
    while i < state.r_draw.viewwidth {
        cosadj = (finecosine[(state.r_main.xtoviewangle[i as usize] >> ANGLETOFINESHIFT) as isize] as i32).abs()
            as fixed_t;
        state.r_plane.distscale[i as usize] = FixedDiv(FRACUNIT, cosadj);
        i += 1;
    }
    i = 0 as i32;
    while i < LIGHTLEVELS {
        startmap = (LIGHTLEVELS - 1 as i32 - i) * 2 as i32 * NUMCOLORMAPS / LIGHTLEVELS;
        j = 0 as i32;
        while j < MAXLIGHTSCALE {
            level = startmap - j * SCREENWIDTH / (state.r_draw.viewwidth << state.r_main.detailshift) / DISTMAP;
            if level < 0 as i32 {
                level = 0 as i32;
            }
            if level >= NUMCOLORMAPS {
                level = NUMCOLORMAPS - 1 as i32;
            }
            state.r_main.scalelight[i as usize][j as usize] = state.r_data.colormaps.offset((level * 256 as i32) as isize);
            j += 1;
        }
        i += 1;
    }
}
pub unsafe fn R_Init() {
    R_InitData(unsafe { game_state() });
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_SetViewSize(
        unsafe { game_state() }.m_menu.screenblocks,
        unsafe { game_state() }.m_menu.detailLevel,
    );
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitLightTables();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitSkyMap();
    R_InitTranslationTables();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    unsafe { game_state() }.r_main.framecount = 0 as i32;
}
pub unsafe fn R_PointInSubsector(mut x: fixed_t, mut y: fixed_t) -> *mut subsector_t {
    let mut node: *mut node_t = ::core::ptr::null_mut::<node_t>();
    let mut side: i32 = 0;
    let mut nodenum: i32 = 0;
    if unsafe { game_state() }.p_setup.numnodes == 0 {
        return unsafe { game_state() }.p_setup.subsector_mut(SubsectorId(0));
    }
    nodenum = unsafe { game_state() }.p_setup.numnodes - 1 as i32;
    while nodenum & NF_SUBSECTOR == 0 {
        node = unsafe { game_state() }.p_setup.nodes.offset(nodenum as isize) as *mut node_t;
        side = R_PointOnSide(x, y, node);
        nodenum = (*node).children[side as usize] as i32;
    }
    return unsafe { game_state() }
        .p_setup
        .subsector_mut(SubsectorId((nodenum & !NF_SUBSECTOR) as u32));
}
pub unsafe fn R_SetupFrame(mut player: *mut player_t) {
    let mut i: i32 = 0;
    unsafe { game_state() }.r_main.viewplayer = player;
    unsafe { game_state() }.r_main.viewx = (*(*player).mo).x;
    unsafe { game_state() }.r_main.viewy = (*(*player).mo).y;
    unsafe { game_state() }.r_main.viewangle = (*(*player).mo)
        .angle
        .wrapping_add(unsafe { game_state() }.r_main.viewangleoffset as angle_t);
    unsafe { game_state() }.r_main.extralight = (*player).extralight;
    unsafe { game_state() }.r_main.viewz = (*player).viewz;
    unsafe { game_state() }.r_main.viewsin = finesine[(unsafe { game_state() }.r_main.viewangle >> ANGLETOFINESHIFT) as usize];
    unsafe { game_state() }.r_main.viewcos = finecosine[(unsafe { game_state() }.r_main.viewangle >> ANGLETOFINESHIFT) as isize];
    unsafe { game_state() }.r_main.sscount = 0 as i32;
    if (*player).fixedcolormap != 0 {
        unsafe { game_state() }.r_main.fixedcolormap = unsafe { game_state() }.r_data.colormaps.offset(
            (((*player).fixedcolormap * 256 as i32) as usize)
                .wrapping_mul(::core::mem::size_of::<lighttable_t>() as usize) as isize,
        );
        unsafe { game_state() }.r_segs.walllights = &raw mut unsafe { game_state() }.r_main.scalelightfixed as *mut *mut lighttable_t;
        i = 0 as i32;
        while i < MAXLIGHTSCALE {
            unsafe { game_state() }.r_main.scalelightfixed[i as usize] = unsafe { game_state() }.r_main.fixedcolormap;
            i += 1;
        }
    } else {
        unsafe { game_state() }.r_main.fixedcolormap = ::core::ptr::null_mut::<lighttable_t>();
    }
    unsafe { game_state() }.r_main.framecount += 1;
    unsafe { game_state() }.r_main.validcount += 1;
}
pub unsafe fn R_RenderPlayerView(state: &mut GameState, mut player: *mut player_t) {
    R_SetupFrame(player);
    R_ClearClipSegs();
    R_ClearDrawSegs();
    R_ClearPlanes();
    R_ClearSprites();
    NetUpdate(state);
    R_RenderBSPNode(state.p_setup.numnodes - 1 as i32);
    NetUpdate(state);
    R_DrawPlanes();
    NetUpdate(state);
    R_DrawMasked();
    NetUpdate(state);
}
pub const LIGHTLEVELS: i32 = 16;
pub const MAXLIGHTSCALE: i32 = 48;
pub const LIGHTSCALESHIFT: i32 = 12;
pub const MAXLIGHTZ: i32 = 128;
pub const LIGHTZSHIFT: i32 = 20;
pub const NUMCOLORMAPS: i32 = 32;
pub const LIGHTSEGSHIFT: i32 = 4;
