use crate::src::r_defs::{node_t, seg_t};
use crate::src::p_mobj::subsector_t;
use crate::src::d_player::{player_t};
use crate::src::tables::SlopeDiv;
use crate::src::r_segs::walllights;
use crate::src::r_data::R_InitData;
use crate::src::r_segs::rw_distance;
use crate::src::r_segs::rw_normalangle;
use crate::src::r_bsp::R_ClearClipSegs;
use crate::src::r_bsp::R_ClearDrawSegs;
use crate::src::r_bsp::R_RenderBSPNode;
use crate::src::r_plane::yslope;
use crate::src::r_plane::distscale;
use crate::src::r_plane::R_InitPlanes;
use crate::src::r_plane::R_ClearPlanes;
use crate::src::r_things::pspritescale;
use crate::src::r_things::R_ClearSprites;
use crate::src::r_draw::R_InitBuffer;
use crate::src::r_draw::R_InitTranslationTables;
use crate::src::r_sky::R_InitSkyMap;
use crate::src::d_loop::NetUpdate;
use crate::src::m_menu::detailLevel;
use crate::src::m_menu::screenblocks;
use crate::src::p_setup::numnodes;
use crate::src::r_draw::scaledviewwidth;
use crate::src::r_things::pspriteiscale;
use crate::src::r_things::screenheightarray;
use crate::src::tables::tantoangle;
use crate::src::p_setup::subsectors;
use crate::src::p_setup::nodes;
use crate::src::tables::finetangent;
use crate::src::r_data::colormaps;
use crate::src::r_draw::viewwidth;
use crate::src::r_draw::viewheight;
use crate::src::m_fixed::FixedDiv;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::r_plane::R_DrawPlanes;
use crate::src::r_things::R_DrawMasked;
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::r_defs::lighttable_t;
use libc::printf;
use crate::src::r_draw::{
    R_DrawColumn, R_DrawColumnLow, R_DrawFuzzColumn, R_DrawFuzzColumnLow,
    R_DrawTranslatedColumn, R_DrawTranslatedColumnLow, R_DrawSpan, R_DrawSpanLow,
};
use crate::src::doomdef::SCREENWIDTH;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::m_fixed::FRACUNIT;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG180;
use crate::src::tables::ANG90;
use crate::src::tables::ANG270;
pub const FRACBITS: i32 = 16 as i32;
pub const FINEANGLES: i32 = 8192;
pub const SLOPEBITS: i32 = 11 as i32;
pub const DBITS: i32 = FRACBITS - SLOPEBITS;
pub const NF_SUBSECTOR: i32 = 0x8000 as i32;
pub const FIELDOFVIEW: i32 = 2048 as i32;
pub static mut viewangleoffset: i32 = 0;
pub static mut validcount: i32 = 1 as i32;
pub static mut fixedcolormap: *mut lighttable_t = ::core::ptr::null::<lighttable_t>()
    as *mut lighttable_t;
#[no_mangle]
pub static mut centerx: i32 = 0;
pub static mut centery: i32 = 0;
pub static mut centerxfrac: fixed_t = 0;
pub static mut centeryfrac: fixed_t = 0;
pub static mut projection: fixed_t = 0;
#[no_mangle]
pub static mut framecount: i32 = 0;
pub static mut sscount: i32 = 0;
#[no_mangle]
pub static mut linecount: i32 = 0;
#[no_mangle]
pub static mut loopcount: i32 = 0;
pub static mut viewx: fixed_t = 0;
pub static mut viewy: fixed_t = 0;
pub static mut viewz: fixed_t = 0;
pub static mut viewangle: angle_t = 0;
pub static mut viewcos: fixed_t = 0;
pub static mut viewsin: fixed_t = 0;
pub static mut viewplayer: *mut player_t = ::core::ptr::null::<player_t>()
    as *mut player_t;
pub static mut detailshift: i32 = 0;
pub static mut clipangle: angle_t = 0;
pub static mut viewangletox: [i32; 4096] = [0; 4096];
pub static mut xtoviewangle: [angle_t; 321] = [0; 321];
pub static mut scalelight: [[*mut lighttable_t; 48]; 16] = [[::core::ptr::null::<
    lighttable_t,
>() as *mut lighttable_t; 48]; 16];
#[no_mangle]
pub static mut scalelightfixed: [*mut lighttable_t; 48] = [::core::ptr::null::<
    lighttable_t,
>() as *mut lighttable_t; 48];
pub static mut zlight: [[*mut lighttable_t; 128]; 16] = [[::core::ptr::null::<
    lighttable_t,
>() as *mut lighttable_t; 128]; 16];
pub static mut extralight: i32 = 0;
pub static mut colfunc: Option<unsafe fn() -> ()> = None;
pub static mut basecolfunc: Option<unsafe fn() -> ()> = None;
pub static mut fuzzcolfunc: Option<unsafe fn() -> ()> = None;
pub static mut transcolfunc: Option<unsafe fn() -> ()> = None;
pub static mut spanfunc: Option<unsafe fn() -> ()> = None;
#[no_mangle]
pub unsafe extern "C" fn R_AddPointToBox(
    mut x: i32,
    mut y: i32,
    mut box_0: *mut fixed_t,
) {
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
pub unsafe fn R_PointOnSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut node: *mut node_t,
) -> i32 {
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
    if ((*node).dy ^ (*node).dx ^ dx ^ dy) as u32
        & 0x80000000 as u32 != 0
    {
        if ((*node).dy ^ dx) as u32 & 0x80000000 as u32
            != 0
        {
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
pub unsafe fn R_PointOnSegSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut line: *mut seg_t,
) -> i32 {
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
    if (ldy ^ ldx ^ dx ^ dy) as u32 & 0x80000000 as u32
        != 0
    {
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
    x -= viewx;
    y -= viewy;
    if x == 0 && y == 0 {
        return 0 as angle_t;
    }
    if x >= 0 as i32 {
        if y >= 0 as i32 {
            if x > y {
                return tantoangle[SlopeDiv(
                    y as u32,
                    x as u32,
                ) as usize]
            } else {
                return ((ANG90 - 1 as i32) as angle_t)
                    .wrapping_sub(
                        tantoangle[SlopeDiv(
                            x as u32,
                            y as u32,
                        ) as usize],
                    )
            }
        } else {
            y = -y;
            if x > y {
                return tantoangle[SlopeDiv(
                        y as u32,
                        x as u32,
                    ) as usize]
                    .wrapping_neg()
            } else {
                return ANG270
                    .wrapping_add(
                        tantoangle[SlopeDiv(
                            x as u32,
                            y as u32,
                        ) as usize],
                    )
            }
        }
    } else {
        x = -x;
        if y >= 0 as i32 {
            if x > y {
                return ANG180
                    .wrapping_sub(1 as angle_t)
                    .wrapping_sub(
                        tantoangle[SlopeDiv(
                            y as u32,
                            x as u32,
                        ) as usize],
                    )
            } else {
                return (ANG90 as angle_t)
                    .wrapping_add(
                        tantoangle[SlopeDiv(
                            x as u32,
                            y as u32,
                        ) as usize],
                    )
            }
        } else {
            y = -y;
            if x > y {
                return ANG180
                    .wrapping_add(
                        tantoangle[SlopeDiv(
                            y as u32,
                            x as u32,
                        ) as usize],
                    )
            } else {
                return ANG270
                    .wrapping_sub(1 as angle_t)
                    .wrapping_sub(
                        tantoangle[SlopeDiv(
                            x as u32,
                            y as u32,
                        ) as usize],
                    )
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
    viewx = x1;
    viewy = y1;
    return R_PointToAngle(x2, y2);
}
pub unsafe fn R_PointToDist(mut x: fixed_t, mut y: fixed_t) -> fixed_t {
    let mut angle: i32 = 0;
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut temp: fixed_t = 0;
    let mut dist: fixed_t = 0;
    let mut frac: fixed_t = 0;
    dx = (x as i32 - viewx as i32).abs() as fixed_t;
    dy = (y as i32 - viewy as i32).abs() as fixed_t;
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
#[no_mangle]
pub unsafe extern "C" fn R_InitPointToAngle() {}
pub unsafe fn R_ScaleFromGlobalAngle(mut visangle: angle_t) -> fixed_t {
    let mut scale: fixed_t = 0;
    let mut anglea: angle_t = 0;
    let mut angleb: angle_t = 0;
    let mut sinea: i32 = 0;
    let mut sineb: i32 = 0;
    let mut num: fixed_t = 0;
    let mut den: i32 = 0;
    anglea = (ANG90 as angle_t).wrapping_add(visangle.wrapping_sub(viewangle));
    angleb = (ANG90 as angle_t).wrapping_add(visangle.wrapping_sub(rw_normalangle));
    sinea = finesine[(anglea >> ANGLETOFINESHIFT) as usize] as i32;
    sineb = finesine[(angleb >> ANGLETOFINESHIFT) as usize] as i32;
    num = FixedMul(projection, sineb as fixed_t) << detailshift;
    den = FixedMul(rw_distance, sinea as fixed_t) as i32;
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
#[no_mangle]
pub unsafe extern "C" fn R_InitTables() {}
#[no_mangle]
pub unsafe extern "C" fn R_InitTextureMapping() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut t: i32 = 0;
    let mut focallength: fixed_t = 0;
    focallength = FixedDiv(
        centerxfrac,
        finetangent[(FINEANGLES / 4 as i32
            + FIELDOFVIEW / 2 as i32) as usize],
    );
    i = 0 as i32;
    while i < FINEANGLES / 2 as i32 {
        if finetangent[i as usize] > FRACUNIT * 2 as i32 {
            t = -(1 as i32);
        } else if finetangent[i as usize] < -FRACUNIT * 2 as i32 {
            t = viewwidth + 1 as i32;
        } else {
            t = FixedMul(finetangent[i as usize], focallength) as i32;
            t = centerxfrac as i32 - t + FRACUNIT
                - 1 as i32 >> FRACBITS;
            if t < -(1 as i32) {
                t = -(1 as i32);
            } else if t > viewwidth + 1 as i32 {
                t = viewwidth + 1 as i32;
            }
        }
        viewangletox[i as usize] = t;
        i += 1;
    }
    x = 0 as i32;
    while x <= viewwidth {
        i = 0 as i32;
        while viewangletox[i as usize] > x {
            i += 1;
        }
        xtoviewangle[x as usize] = ((i << ANGLETOFINESHIFT) - ANG90) as angle_t;
        x += 1;
    }
    i = 0 as i32;
    while i < FINEANGLES / 2 as i32 {
        t = FixedMul(finetangent[i as usize], focallength) as i32;
        t = centerx - t;
        if viewangletox[i as usize] == -(1 as i32) {
            viewangletox[i as usize] = 0 as i32;
        } else if viewangletox[i as usize] == viewwidth + 1 as i32 {
            viewangletox[i as usize] = viewwidth;
        }
        i += 1;
    }
    clipangle = xtoviewangle[0 as i32 as usize];
}
pub const DISTMAP: i32 = 2 as i32;
#[no_mangle]
pub unsafe extern "C" fn R_InitLightTables() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut level: i32 = 0;
    let mut startmap: i32 = 0;
    let mut scale: i32 = 0;
    i = 0 as i32;
    while i < LIGHTLEVELS {
        startmap = (LIGHTLEVELS - 1 as i32 - i) * 2 as i32
            * NUMCOLORMAPS / LIGHTLEVELS;
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
            zlight[i as usize][j as usize] = colormaps
                .offset((level * 256 as i32) as isize);
            j += 1;
        }
        i += 1;
    }
}
pub static mut setsizeneeded: bool = false;
#[no_mangle]
pub static mut setblocks: i32 = 0;
#[no_mangle]
pub static mut setdetail: i32 = 0;
pub unsafe fn R_SetViewSize(
    mut blocks: i32,
    mut detail: i32,
) {
    setsizeneeded = true;
    setblocks = blocks;
    setdetail = detail;
}
pub unsafe fn R_ExecuteSetViewSize() {
    let mut cosadj: fixed_t = 0;
    let mut dy: fixed_t = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut level: i32 = 0;
    let mut startmap: i32 = 0;
    setsizeneeded = false;
    if setblocks == 11 as i32 {
        scaledviewwidth = SCREENWIDTH;
        viewheight = SCREENHEIGHT;
    } else {
        scaledviewwidth = setblocks * 32 as i32;
        viewheight = setblocks * 168 as i32 / 10 as i32
            & !(7 as i32);
    }
    detailshift = setdetail;
    viewwidth = scaledviewwidth >> detailshift;
    centery = viewheight / 2 as i32;
    centerx = viewwidth / 2 as i32;
    centerxfrac = (centerx << FRACBITS) as fixed_t;
    centeryfrac = (centery << FRACBITS) as fixed_t;
    projection = centerxfrac;
    if detailshift == 0 {
        basecolfunc = Some(R_DrawColumn as unsafe fn() -> ());
        colfunc = basecolfunc;
        fuzzcolfunc = Some(R_DrawFuzzColumn as unsafe fn() -> ());
        transcolfunc = Some(R_DrawTranslatedColumn as unsafe fn() -> ());
        spanfunc = Some(R_DrawSpan as unsafe fn() -> ());
    } else {
        basecolfunc = Some(R_DrawColumnLow as unsafe fn() -> ());
        colfunc = basecolfunc;
        fuzzcolfunc = Some(R_DrawFuzzColumnLow as unsafe fn() -> ());
        transcolfunc = Some(R_DrawTranslatedColumnLow as unsafe fn() -> ());
        spanfunc = Some(R_DrawSpanLow as unsafe fn() -> ());
    }
    R_InitBuffer(scaledviewwidth, viewheight);
    R_InitTextureMapping();
    pspritescale = (FRACUNIT * viewwidth / SCREENWIDTH) as fixed_t;
    pspriteiscale = (FRACUNIT * SCREENWIDTH / viewwidth) as fixed_t;
    i = 0 as i32;
    while i < viewwidth {
        screenheightarray[i as usize] = viewheight as i16;
        i += 1;
    }
    i = 0 as i32;
    while i < viewheight {
        dy = (((i - viewheight / 2 as i32) << FRACBITS)
            + FRACUNIT / 2 as i32) as fixed_t;
        dy = (dy as i32).abs() as fixed_t;
        yslope[i as usize] = FixedDiv(
            ((viewwidth as fixed_t) << detailshift) / 2 as fixed_t * FRACUNIT,
            dy,
        );
        i += 1;
    }
    i = 0 as i32;
    while i < viewwidth {
        cosadj = (
            finecosine[(xtoviewangle[i as usize] >> ANGLETOFINESHIFT) as isize]
                as i32
        ).abs() as fixed_t;
        distscale[i as usize] = FixedDiv(FRACUNIT, cosadj);
        i += 1;
    }
    i = 0 as i32;
    while i < LIGHTLEVELS {
        startmap = (LIGHTLEVELS - 1 as i32 - i) * 2 as i32
            * NUMCOLORMAPS / LIGHTLEVELS;
        j = 0 as i32;
        while j < MAXLIGHTSCALE {
            level = startmap - j * SCREENWIDTH / (viewwidth << detailshift) / DISTMAP;
            if level < 0 as i32 {
                level = 0 as i32;
            }
            if level >= NUMCOLORMAPS {
                level = NUMCOLORMAPS - 1 as i32;
            }
            scalelight[i as usize][j as usize] = colormaps
                .offset((level * 256 as i32) as isize);
            j += 1;
        }
        i += 1;
    }
}
pub unsafe fn R_Init() {
    R_InitData();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitPointToAngle();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitTables();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_SetViewSize(screenblocks, detailLevel);
    R_InitPlanes();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitLightTables();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitSkyMap();
    R_InitTranslationTables();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    framecount = 0 as i32;
}
pub unsafe fn R_PointInSubsector(
    mut x: fixed_t,
    mut y: fixed_t,
) -> *mut subsector_t {
    let mut node: *mut node_t = ::core::ptr::null_mut::<node_t>();
    let mut side: i32 = 0;
    let mut nodenum: i32 = 0;
    if numnodes == 0 {
        return subsectors;
    }
    nodenum = numnodes - 1 as i32;
    while nodenum & NF_SUBSECTOR == 0 {
        node = nodes.offset(nodenum as isize) as *mut node_t;
        side = R_PointOnSide(x, y, node);
        nodenum = (*node).children[side as usize] as i32;
    }
    return subsectors.offset((nodenum & !NF_SUBSECTOR) as isize) as *mut subsector_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_SetupFrame(mut player: *mut player_t) {
    let mut i: i32 = 0;
    viewplayer = player;
    viewx = (*(*player).mo).x;
    viewy = (*(*player).mo).y;
    viewangle = (*(*player).mo).angle.wrapping_add(viewangleoffset as angle_t);
    extralight = (*player).extralight;
    viewz = (*player).viewz;
    viewsin = finesine[(viewangle >> ANGLETOFINESHIFT) as usize];
    viewcos = finecosine[(viewangle >> ANGLETOFINESHIFT) as isize];
    sscount = 0 as i32;
    if (*player).fixedcolormap != 0 {
        fixedcolormap = colormaps
            .offset(
                (((*player).fixedcolormap * 256 as i32) as usize)
                    .wrapping_mul(::core::mem::size_of::<lighttable_t>() as usize)
                    as isize,
            );
        walllights = &raw mut scalelightfixed as *mut *mut lighttable_t;
        i = 0 as i32;
        while i < MAXLIGHTSCALE {
            scalelightfixed[i as usize] = fixedcolormap;
            i += 1;
        }
    } else {
        fixedcolormap = ::core::ptr::null_mut::<lighttable_t>();
    }
    framecount += 1;
    validcount += 1;
}
pub unsafe fn R_RenderPlayerView(mut player: *mut player_t) {
    R_SetupFrame(player);
    R_ClearClipSegs();
    R_ClearDrawSegs();
    R_ClearPlanes();
    R_ClearSprites();
    NetUpdate();
    R_RenderBSPNode(numnodes - 1 as i32);
    NetUpdate();
    R_DrawPlanes();
    NetUpdate();
    R_DrawMasked();
    NetUpdate();
}
pub const LIGHTLEVELS: i32 = 16 as i32;
pub const MAXLIGHTSCALE: i32 = 48 as i32;
pub const LIGHTSCALESHIFT: i32 = 12 as i32;
pub const MAXLIGHTZ: i32 = 128 as i32;
pub const LIGHTZSHIFT: i32 = 20 as i32;
pub const NUMCOLORMAPS: i32 = 32 as i32;
