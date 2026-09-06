use crate::src::r_defs::{node_t, seg_t, side_t};
use crate::src::p_mobj::{thinker_s, mapthing_t, sector_t, line_s, ST_NEGATIVE, ST_POSITIVE, ST_VERTICAL, ST_HORIZONTAL, vertex_t, degenmobj_t, line_t, subsector_t, ThinkerFn};
use crate::src::p_mobj::{mobj_t};
use crate::src::m_argv::M_CheckParm;
use crate::src::w_wad::{wad_name8_to_string, W_GetNumForName};
use crate::src::z_zone::Z_FreeTags;
use crate::src::m_bbox::M_ClearBox;
use crate::src::g_game::G_DeathMatchSpawnPlayer;
use crate::src::i_system::I_GetMemoryValue;
use crate::src::info::sprnames;
use crate::src::r_data::R_PrecacheLevel;
use crate::src::p_spec::P_SpawnSpecials;
use crate::src::p_switch::P_InitSwitchList;
use crate::src::g_game::wminfo;
use crate::src::g_game::precache;
use crate::src::g_game::bodyqueslot;
use crate::src::g_game::totalsecret;
use crate::src::g_game::totalkills;
use crate::src::g_game::totalitems;
use crate::src::m_bbox::M_AddToBox;
use crate::src::p_tick::P_InitThinkers;
use crate::src::m_fixed::FixedDiv;
use crate::src::g_game::deathmatch;
use crate::src::g_game::playeringame;
use crate::src::g_game::consoleplayer;
use crate::src::p_tick::leveltime;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::p_mobj::P_SpawnMapThing;
use crate::src::p_spec::P_InitPicAnims;
use crate::src::r_things::R_InitSprites;
use crate::src::s_sound::S_Start;
use crate::src::w_wad::W_ReadLump;
use crate::src::r_data::R_FlatNumForName;
use crate::src::w_wad::W_LumpLength;
use crate::src::w_wad::W_ReleaseLumpNum;
use crate::src::r_data::R_TextureNumForName;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_LEVEL, PU_PURGELEVEL, PU_STATIC};
use crate::src::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use libc::memset;
use libc::snprintf;
use crate::src::i_system::{fprintf, stderr};
use crate::src::d_mode::commercial;
use crate::src::d_mode::skill_t;
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use crate::src::doomdef::NULL;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::m_fixed::FRACUNIT;
use crate::src::p_spec::ML_TWOSIDED;
use crate::src::p_maputl::MAPBLOCKSHIFT;
use crate::src::m_fixed::FRACBITS;
use crate::src::game_state::game_state;

pub type C2RustUnnamed_1 = u32;
pub const ML_BLOCKMAP: C2RustUnnamed_1 = 10;
pub const ML_REJECT: C2RustUnnamed_1 = 9;
pub const ML_SECTORS: C2RustUnnamed_1 = 8;
pub const ML_NODES: C2RustUnnamed_1 = 7;
pub const ML_SSECTORS: C2RustUnnamed_1 = 6;
pub const ML_SEGS: C2RustUnnamed_1 = 5;
pub const ML_VERTEXES: C2RustUnnamed_1 = 4;
pub const ML_SIDEDEFS: C2RustUnnamed_1 = 3;
pub const ML_LINEDEFS: C2RustUnnamed_1 = 2;
pub const ML_THINGS: C2RustUnnamed_1 = 1;
pub const ML_LABEL: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapvertex_t {
    pub x: i16,
    pub y: i16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapsidedef_t {
    pub textureoffset: i16,
    pub rowoffset: i16,
    pub toptexture: [::core::ffi::c_char; 8],
    pub bottomtexture: [::core::ffi::c_char; 8],
    pub midtexture: [::core::ffi::c_char; 8],
    pub sector: i16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct maplinedef_t {
    pub v1: i16,
    pub v2: i16,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapsector_t {
    pub floorheight: i16,
    pub ceilingheight: i16,
    pub floorpic: [::core::ffi::c_char; 8],
    pub ceilingpic: [::core::ffi::c_char; 8],
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapsubsector_t {
    pub numsegs: i16,
    pub firstseg: i16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapseg_t {
    pub v1: i16,
    pub v2: i16,
    pub angle: i16,
    pub linedef: i16,
    pub side: i16,
    pub offset: i16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapnode_t {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox: [[i16; 4]; 2],
    pub children: [u16; 2],
}
pub static mut numvertexes: i32 = 0;
pub static mut vertexes: *mut vertex_t = ::core::ptr::null::<vertex_t>()
    as *mut vertex_t;
#[no_mangle]
pub static mut numsegs: i32 = 0;
pub static mut segs: *mut seg_t = ::core::ptr::null::<seg_t>() as *mut seg_t;
pub static mut numsectors: i32 = 0;
pub static mut sectors: *mut sector_t = ::core::ptr::null::<sector_t>() as *mut sector_t;
pub static mut numsubsectors: i32 = 0;
pub static mut subsectors: *mut subsector_t = ::core::ptr::null::<subsector_t>()
    as *mut subsector_t;
pub static mut numnodes: i32 = 0;
pub static mut nodes: *mut node_t = ::core::ptr::null::<node_t>() as *mut node_t;
pub static mut numlines: i32 = 0;
pub static mut lines: *mut line_t = ::core::ptr::null::<line_t>() as *mut line_t;
pub static mut numsides: i32 = 0;
pub static mut sides: *mut side_t = ::core::ptr::null::<side_t>() as *mut side_t;
static mut totallines: i32 = 0;
pub static mut bmapwidth: i32 = 0;
pub static mut bmapheight: i32 = 0;
pub static mut blockmap: *mut i16 = ::core::ptr::null::<
    i16,
>() as *mut i16;
pub static mut blockmaplump: *mut i16 = ::core::ptr::null::<
    i16,
>() as *mut i16;
pub static mut bmaporgx: fixed_t = 0;
pub static mut bmaporgy: fixed_t = 0;
pub static mut blocklinks: *mut *mut mobj_t = ::core::ptr::null::<*mut mobj_t>()
    as *mut *mut mobj_t;
pub static mut rejectmatrix: *mut byte = ::core::ptr::null::<byte>() as *mut byte;
pub static mut deathmatchstarts: [mapthing_t; 10] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    type_0: 0,
    options: 0,
}; 10];
pub static mut deathmatch_p: *mut mapthing_t = ::core::ptr::null::<mapthing_t>()
    as *mut mapthing_t;
pub static mut playerstarts: [mapthing_t; 4] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    type_0: 0,
    options: 0,
}; 4];
pub unsafe fn P_LoadVertexes(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut ml: *mut mapvertex_t = ::core::ptr::null_mut::<mapvertex_t>();
    let mut li: *mut vertex_t = ::core::ptr::null_mut::<vertex_t>();
    numvertexes = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapvertex_t>() as usize)
        as i32;
    vertexes = Z_Malloc(
        (numvertexes as usize).wrapping_mul(::core::mem::size_of::<vertex_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut vertex_t;
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    ml = data as *mut mapvertex_t;
    li = vertexes;
    i = 0 as i32;
    while i < numvertexes {
        (*li).x = (((*ml).x as i32) << FRACBITS) as fixed_t;
        (*li).y = (((*ml).y as i32) << FRACBITS) as fixed_t;
        i += 1;
        li = li.offset(1);
        ml = ml.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn GetSectorAtNullAddress() -> *mut sector_t {
    static mut null_sector_is_initialized: bool = false;
    static mut null_sector: sector_t = sector_t {
        floorheight: 0,
        ceilingheight: 0,
        floorpic: 0,
        ceilingpic: 0,
        lightlevel: 0,
        special: 0,
        tag: 0,
        soundtraversed: 0,
        soundtarget: ::core::ptr::null::<mobj_t>() as *mut mobj_t,
        blockbox: [0; 4],
        soundorg: degenmobj_t {
            thinker: thinker_s {
                prev: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
                next: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
                function: ThinkerFn::Paused,
            },
            x: 0,
            y: 0,
            z: 0,
        },
        validcount: 0,
        thinglist: ::core::ptr::null::<mobj_t>() as *mut mobj_t,
        specialdata: ::core::ptr::null::<::core::ffi::c_void>()
            as *mut ::core::ffi::c_void,
        linecount: 0,
        lines: ::core::ptr::null::<*mut line_s>() as *mut *mut line_s,
    };
    if !null_sector_is_initialized {
        memset(
            &raw mut null_sector as *mut ::core::ffi::c_void,
            0 as i32,
            ::core::mem::size_of::<sector_t>() as size_t,
        );
        I_GetMemoryValue(
            0 as u32,
            &raw mut null_sector.floorheight as *mut ::core::ffi::c_void,
            4 as i32,
        );
        I_GetMemoryValue(
            4 as u32,
            &raw mut null_sector.ceilingheight as *mut ::core::ffi::c_void,
            4 as i32,
        );
        null_sector_is_initialized = true;
    }
    return &raw mut null_sector;
}
pub unsafe fn P_LoadSegs(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut ml: *mut mapseg_t = ::core::ptr::null_mut::<mapseg_t>();
    let mut li: *mut seg_t = ::core::ptr::null_mut::<seg_t>();
    let mut ldef: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut linedef: i32 = 0;
    let mut side: i32 = 0;
    let mut sidenum: i32 = 0;
    numsegs = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapseg_t>() as usize) as i32;
    segs = Z_Malloc(
        (numsegs as usize).wrapping_mul(::core::mem::size_of::<seg_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut seg_t;
    memset(
        segs as *mut ::core::ffi::c_void,
        0 as i32,
        (numsegs as size_t).wrapping_mul(::core::mem::size_of::<seg_t>() as size_t),
    );
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    ml = data as *mut mapseg_t;
    li = segs;
    i = 0 as i32;
    while i < numsegs {
        (*li).v1 = vertexes.offset((*ml).v1 as isize) as *mut vertex_t;
        (*li).v2 = vertexes.offset((*ml).v2 as isize) as *mut vertex_t;
        (*li).angle = (((*ml).angle as i32) << 16 as i32)
            as angle_t;
        (*li).offset = (((*ml).offset as i32) << 16 as i32)
            as fixed_t;
        linedef = (*ml).linedef as i32;
        ldef = lines.offset(linedef as isize) as *mut line_t;
        (*li).linedef = ldef;
        side = (*ml).side as i32;
        (*li).sidedef = sides
            .offset(
                *(&raw mut (*ldef).sidenum as *mut i16)
                    .offset(side as isize) as isize,
            ) as *mut side_t;
        (*li).frontsector = (*sides.offset((*ldef).sidenum[side as usize] as isize))
            .sector;
        if (*ldef).flags as i32 & ML_TWOSIDED != 0 {
            sidenum = (*ldef).sidenum[(side ^ 1 as i32) as usize]
                as i32;
            if sidenum < 0 as i32 || sidenum >= numsides {
                (*li).backsector = GetSectorAtNullAddress();
            } else {
                (*li).backsector = (*sides.offset(sidenum as isize)).sector;
            }
        } else {
            (*li).backsector = ::core::ptr::null_mut::<sector_t>();
        }
        i += 1;
        li = li.offset(1);
        ml = ml.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadSubsectors(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut ms: *mut mapsubsector_t = ::core::ptr::null_mut::<mapsubsector_t>();
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    numsubsectors = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapsubsector_t>() as usize)
        as i32;
    subsectors = Z_Malloc(
        (numsubsectors as usize)
            .wrapping_mul(::core::mem::size_of::<subsector_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut subsector_t;
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    ms = data as *mut mapsubsector_t;
    memset(
        subsectors as *mut ::core::ffi::c_void,
        0 as i32,
        (numsubsectors as size_t)
            .wrapping_mul(::core::mem::size_of::<subsector_t>() as size_t),
    );
    ss = subsectors;
    i = 0 as i32;
    while i < numsubsectors {
        (*ss).numlines = (*ms).numsegs;
        (*ss).firstline = (*ms).firstseg;
        i += 1;
        ss = ss.offset(1);
        ms = ms.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadSectors(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut ms: *mut mapsector_t = ::core::ptr::null_mut::<mapsector_t>();
    let mut ss: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    numsectors = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapsector_t>() as usize)
        as i32;
    sectors = Z_Malloc(
        (numsectors as usize).wrapping_mul(::core::mem::size_of::<sector_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut sector_t;
    memset(
        sectors as *mut ::core::ffi::c_void,
        0 as i32,
        (numsectors as size_t).wrapping_mul(::core::mem::size_of::<sector_t>() as size_t),
    );
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    ms = data as *mut mapsector_t;
    ss = sectors;
    i = 0 as i32;
    while i < numsectors {
        (*ss).floorheight = (((*ms).floorheight as i32) << FRACBITS)
            as fixed_t;
        (*ss).ceilingheight = (((*ms).ceilingheight as i32) << FRACBITS)
            as fixed_t;
        (*ss).floorpic = R_FlatNumForName(
            &raw mut (*ms).floorpic as *mut ::core::ffi::c_char,
        ) as i16;
        (*ss).ceilingpic = R_FlatNumForName(
            &raw mut (*ms).ceilingpic as *mut ::core::ffi::c_char,
        ) as i16;
        (*ss).lightlevel = (*ms).lightlevel;
        (*ss).special = (*ms).special;
        (*ss).tag = (*ms).tag;
        (*ss).thinglist = ::core::ptr::null_mut::<mobj_t>();
        i += 1;
        ss = ss.offset(1);
        ms = ms.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadNodes(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut mn: *mut mapnode_t = ::core::ptr::null_mut::<mapnode_t>();
    let mut no: *mut node_t = ::core::ptr::null_mut::<node_t>();
    numnodes = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapnode_t>() as usize)
        as i32;
    nodes = Z_Malloc(
        (numnodes as usize).wrapping_mul(::core::mem::size_of::<node_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut node_t;
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    mn = data as *mut mapnode_t;
    no = nodes;
    i = 0 as i32;
    while i < numnodes {
        (*no).x = (((*mn).x as i32) << FRACBITS) as fixed_t;
        (*no).y = (((*mn).y as i32) << FRACBITS) as fixed_t;
        (*no).dx = (((*mn).dx as i32) << FRACBITS) as fixed_t;
        (*no).dy = (((*mn).dy as i32) << FRACBITS) as fixed_t;
        j = 0 as i32;
        while j < 2 as i32 {
            (*no).children[j as usize] = (*mn).children[j as usize]
                as i16 as u16;
            k = 0 as i32;
            while k < 4 as i32 {
                (*no).bbox[j as usize][k as usize] = (((*mn).bbox[j as usize][k as usize]
                    as i32) << FRACBITS) as fixed_t;
                k += 1;
            }
            j += 1;
        }
        i += 1;
        no = no.offset(1);
        mn = mn.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadThings(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut mt: *mut mapthing_t = ::core::ptr::null_mut::<mapthing_t>();
    let mut spawnthing: mapthing_t = mapthing_t {
        x: 0,
        y: 0,
        angle: 0,
        type_0: 0,
        options: 0,
    };
    let mut numthings: i32 = 0;
    let mut spawn: bool = false;
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    numthings = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapthing_t>() as usize)
        as i32;
    mt = data as *mut mapthing_t;
    i = 0 as i32;
    while i < numthings {
        spawn = true;
        if gamemode as u32
            != commercial as i32 as u32
        {
            let mut current_block_5: u64;
            match (*mt).type_0 as i32 {
                64 => {
                    current_block_5 = 10716006297776741838;
                }
                88 => {
                    current_block_5 = 10716006297776741838;
                }
                89 => {
                    current_block_5 = 4903439290872339201;
                }
                69 => {
                    current_block_5 = 4274804826831059371;
                }
                67 => {
                    current_block_5 = 12519425194970330903;
                }
                71 => {
                    current_block_5 = 14837270904287063365;
                }
                65 => {
                    current_block_5 = 10735630984003381802;
                }
                66 => {
                    current_block_5 = 14624214236247155710;
                }
                68 | 84 => {
                    current_block_5 = 16749256938512238719;
                }
                _ => {
                    current_block_5 = 2979737022853876585;
                }
            }
            match current_block_5 {
                10716006297776741838 => {
                    current_block_5 = 4903439290872339201;
                }
                _ => {}
            }
            match current_block_5 {
                4903439290872339201 => {
                    current_block_5 = 4274804826831059371;
                }
                _ => {}
            }
            match current_block_5 {
                4274804826831059371 => {
                    current_block_5 = 12519425194970330903;
                }
                _ => {}
            }
            match current_block_5 {
                12519425194970330903 => {
                    current_block_5 = 14837270904287063365;
                }
                _ => {}
            }
            match current_block_5 {
                14837270904287063365 => {
                    current_block_5 = 10735630984003381802;
                }
                _ => {}
            }
            match current_block_5 {
                10735630984003381802 => {
                    current_block_5 = 14624214236247155710;
                }
                _ => {}
            }
            match current_block_5 {
                14624214236247155710 => {
                    current_block_5 = 16749256938512238719;
                }
                _ => {}
            }
            match current_block_5 {
                16749256938512238719 => {
                    spawn = false;
                }
                _ => {}
            }
        }
        if !spawn {
            break;
        }
        spawnthing.x = (*mt).x;
        spawnthing.y = (*mt).y;
        spawnthing.angle = (*mt).angle;
        spawnthing.type_0 = (*mt).type_0;
        spawnthing.options = (*mt).options;
        P_SpawnMapThing(&raw mut spawnthing);
        i += 1;
        mt = mt.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadLineDefs(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut mld: *mut maplinedef_t = ::core::ptr::null_mut::<maplinedef_t>();
    let mut ld: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut v1: *mut vertex_t = ::core::ptr::null_mut::<vertex_t>();
    let mut v2: *mut vertex_t = ::core::ptr::null_mut::<vertex_t>();
    numlines = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<maplinedef_t>() as usize)
        as i32;
    lines = Z_Malloc(
        (numlines as usize).wrapping_mul(::core::mem::size_of::<line_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut line_t;
    memset(
        lines as *mut ::core::ffi::c_void,
        0 as i32,
        (numlines as size_t).wrapping_mul(::core::mem::size_of::<line_t>() as size_t),
    );
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    mld = data as *mut maplinedef_t;
    ld = lines;
    i = 0 as i32;
    while i < numlines {
        (*ld).flags = (*mld).flags;
        (*ld).special = (*mld).special;
        (*ld).tag = (*mld).tag;
        (*ld).v1 = vertexes.offset((*mld).v1 as isize) as *mut vertex_t;
        v1 = (*ld).v1;
        (*ld).v2 = vertexes.offset((*mld).v2 as isize) as *mut vertex_t;
        v2 = (*ld).v2;
        (*ld).dx = (*v2).x - (*v1).x;
        (*ld).dy = (*v2).y - (*v1).y;
        if (*ld).dx == 0 {
            (*ld).slopetype = ST_VERTICAL;
        } else if (*ld).dy == 0 {
            (*ld).slopetype = ST_HORIZONTAL;
        } else if FixedDiv((*ld).dy, (*ld).dx) > 0 as i32 {
            (*ld).slopetype = ST_POSITIVE;
        } else {
            (*ld).slopetype = ST_NEGATIVE;
        }
        if (*v1).x < (*v2).x {
            (*ld).bbox[BOXLEFT as i32 as usize] = (*v1).x;
            (*ld).bbox[BOXRIGHT as i32 as usize] = (*v2).x;
        } else {
            (*ld).bbox[BOXLEFT as i32 as usize] = (*v2).x;
            (*ld).bbox[BOXRIGHT as i32 as usize] = (*v1).x;
        }
        if (*v1).y < (*v2).y {
            (*ld).bbox[BOXBOTTOM as i32 as usize] = (*v1).y;
            (*ld).bbox[BOXTOP as i32 as usize] = (*v2).y;
        } else {
            (*ld).bbox[BOXBOTTOM as i32 as usize] = (*v2).y;
            (*ld).bbox[BOXTOP as i32 as usize] = (*v1).y;
        }
        (*ld).sidenum[0 as i32 as usize] = (*mld)
            .sidenum[0 as i32 as usize];
        (*ld).sidenum[1 as i32 as usize] = (*mld)
            .sidenum[1 as i32 as usize];
        if (*ld).sidenum[0 as i32 as usize] as i32
            != -(1 as i32)
        {
            (*ld).frontsector = (*sides
                .offset((*ld).sidenum[0 as i32 as usize] as isize))
                .sector;
        } else {
            (*ld).frontsector = ::core::ptr::null_mut::<sector_t>();
        }
        if (*ld).sidenum[1 as i32 as usize] as i32
            != -(1 as i32)
        {
            (*ld).backsector = (*sides
                .offset((*ld).sidenum[1 as i32 as usize] as isize))
                .sector;
        } else {
            (*ld).backsector = ::core::ptr::null_mut::<sector_t>();
        }
        i += 1;
        mld = mld.offset(1);
        ld = ld.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadSideDefs(mut lump: i32) {
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: i32 = 0;
    let mut msd: *mut mapsidedef_t = ::core::ptr::null_mut::<mapsidedef_t>();
    let mut sd: *mut side_t = ::core::ptr::null_mut::<side_t>();
    numsides = (W_LumpLength(lump as u32) as usize)
        .wrapping_div(::core::mem::size_of::<mapsidedef_t>() as usize)
        as i32;
    sides = Z_Malloc(
        (numsides as usize).wrapping_mul(::core::mem::size_of::<side_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut side_t;
    memset(
        sides as *mut ::core::ffi::c_void,
        0 as i32,
        (numsides as size_t).wrapping_mul(::core::mem::size_of::<side_t>() as size_t),
    );
    data = W_CacheLumpNum(lump, PU_STATIC as i32) as *mut byte;
    msd = data as *mut mapsidedef_t;
    sd = sides;
    i = 0 as i32;
    while i < numsides {
        (*sd).textureoffset = (((*msd).textureoffset as i32) << FRACBITS)
            as fixed_t;
        (*sd).rowoffset = (((*msd).rowoffset as i32) << FRACBITS)
            as fixed_t;
        (*sd).toptexture = R_TextureNumForName(
            &raw mut (*msd).toptexture as *mut ::core::ffi::c_char,
        ) as i16;
        (*sd).bottomtexture = R_TextureNumForName(
            &raw mut (*msd).bottomtexture as *mut ::core::ffi::c_char,
        ) as i16;
        (*sd).midtexture = R_TextureNumForName(
            &raw mut (*msd).midtexture as *mut ::core::ffi::c_char,
        ) as i16;
        (*sd).sector = sectors.offset((*msd).sector as isize) as *mut sector_t;
        i += 1;
        msd = msd.offset(1);
        sd = sd.offset(1);
    }
    W_ReleaseLumpNum(lump);
}
pub unsafe fn P_LoadBlockMap(mut lump: i32) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut lumplen: i32 = 0;
    lumplen = W_LumpLength(lump as u32);
    count = lumplen / 2 as i32;
    blockmaplump = Z_Malloc(lumplen, PU_LEVEL as i32, NULL)
        as *mut i16;
    W_ReadLump(lump as u32, blockmaplump as *mut ::core::ffi::c_void);
    blockmap = blockmaplump.offset(4 as i32 as isize);
    i = 0 as i32;
    while i < count {
        *blockmaplump.offset(i as isize) = *blockmaplump.offset(i as isize);
        i += 1;
    }
    bmaporgx = ((*blockmaplump.offset(0 as i32 as isize)
        as i32) << FRACBITS) as fixed_t;
    bmaporgy = ((*blockmaplump.offset(1 as i32 as isize)
        as i32) << FRACBITS) as fixed_t;
    bmapwidth = *blockmaplump.offset(2 as i32 as isize)
        as i32;
    bmapheight = *blockmaplump.offset(3 as i32 as isize)
        as i32;
    count = (::core::mem::size_of::<*mut mobj_t>() as usize)
        .wrapping_mul(bmapwidth as usize)
        .wrapping_mul(bmapheight as usize) as i32;
    blocklinks = Z_Malloc(
        count,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut mobj_t;
    memset(
        blocklinks as *mut ::core::ffi::c_void,
        0 as i32,
        count as size_t,
    );
}
pub unsafe fn P_GroupLines() {
    let mut linebuffer: *mut *mut line_t = ::core::ptr::null_mut::<*mut line_t>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut sector: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    let mut seg: *mut seg_t = ::core::ptr::null_mut::<seg_t>();
    let mut bbox: [fixed_t; 4] = [0; 4];
    let mut block: i32 = 0;
    ss = subsectors;
    i = 0 as i32;
    while i < numsubsectors {
        seg = segs.offset((*ss).firstline as isize) as *mut seg_t;
        (*ss).sector = (*(*seg).sidedef).sector;
        i += 1;
        ss = ss.offset(1);
    }
    li = lines;
    totallines = 0 as i32;
    i = 0 as i32;
    while i < numlines {
        totallines += 1;
        (*(*li).frontsector).linecount += 1;
        if !(*li).backsector.is_null() && (*li).backsector != (*li).frontsector {
            (*(*li).backsector).linecount += 1;
            totallines += 1;
        }
        i += 1;
        li = li.offset(1);
    }
    linebuffer = Z_Malloc(
        (totallines as usize)
            .wrapping_mul(::core::mem::size_of::<*mut line_t>() as usize)
            as i32,
        PU_LEVEL as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut line_t;
    i = 0 as i32;
    while i < numsectors {
        let ref mut fresh0 = (*sectors.offset(i as isize)).lines;
        *fresh0 = linebuffer as *mut *mut line_s;
        linebuffer = linebuffer.offset((*sectors.offset(i as isize)).linecount as isize);
        (*sectors.offset(i as isize)).linecount = 0 as i32;
        i += 1;
    }
    i = 0 as i32;
    while i < numlines {
        li = lines.offset(i as isize) as *mut line_t;
        if !(*li).frontsector.is_null() {
            sector = (*li).frontsector;
            let ref mut fresh1 = *(*sector).lines.offset((*sector).linecount as isize);
            *fresh1 = li as *mut line_s;
            (*sector).linecount += 1;
        }
        if !(*li).backsector.is_null() && (*li).frontsector != (*li).backsector {
            sector = (*li).backsector;
            let ref mut fresh2 = *(*sector).lines.offset((*sector).linecount as isize);
            *fresh2 = li as *mut line_s;
            (*sector).linecount += 1;
        }
        i += 1;
    }
    sector = sectors;
    i = 0 as i32;
    while i < numsectors {
        M_ClearBox(&raw mut bbox as *mut fixed_t);
        j = 0 as i32;
        while j < (*sector).linecount {
            li = *(*sector).lines.offset(j as isize) as *mut line_t;
            M_AddToBox(&raw mut bbox as *mut fixed_t, (*(*li).v1).x, (*(*li).v1).y);
            M_AddToBox(&raw mut bbox as *mut fixed_t, (*(*li).v2).x, (*(*li).v2).y);
            j += 1;
        }
        (*sector).soundorg.x = ((bbox[BOXRIGHT as i32 as usize]
            + bbox[BOXLEFT as i32 as usize]) / 2 as i32)
            as fixed_t;
        (*sector).soundorg.y = ((bbox[BOXTOP as i32 as usize]
            + bbox[BOXBOTTOM as i32 as usize]) / 2 as i32)
            as fixed_t;
        block = bbox[BOXTOP as i32 as usize]
            - bmaporgy as i32 + 32 as i32 * FRACUNIT
            >> MAPBLOCKSHIFT;
        block = if block >= bmapheight {
            bmapheight - 1 as i32
        } else {
            block
        };
        (*sector).blockbox[BOXTOP as i32 as usize] = block;
        block = bbox[BOXBOTTOM as i32 as usize]
            - bmaporgy as i32 - 32 as i32 * FRACUNIT
            >> MAPBLOCKSHIFT;
        block = if block < 0 as i32 {
            0 as i32
        } else {
            block
        };
        (*sector).blockbox[BOXBOTTOM as i32 as usize] = block;
        block = bbox[BOXRIGHT as i32 as usize]
            - bmaporgx as i32 + 32 as i32 * FRACUNIT
            >> MAPBLOCKSHIFT;
        block = if block >= bmapwidth {
            bmapwidth - 1 as i32
        } else {
            block
        };
        (*sector).blockbox[BOXRIGHT as i32 as usize] = block;
        block = bbox[BOXLEFT as i32 as usize]
            - bmaporgx as i32 - 32 as i32 * FRACUNIT
            >> MAPBLOCKSHIFT;
        block = if block < 0 as i32 {
            0 as i32
        } else {
            block
        };
        (*sector).blockbox[BOXLEFT as i32 as usize] = block;
        i += 1;
        sector = sector.offset(1);
    }
}
unsafe fn PadRejectArray(mut array: *mut byte, mut len: u32) {
    let mut i: u32 = 0;
    let mut byte_num: u32 = 0;
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut padvalue: u32 = 0;
    let mut rejectpad: [u32; 4] = [
        ((totallines * 4 as i32 + 3 as i32
            & !(3 as i32)) + 24 as i32)
            as u32,
        0 as i32 as u32,
        50 as i32 as u32,
        0x1d4a11 as i32 as u32,
    ];
    dest = array;
    i = 0 as u32;
    while i < len
        && (i as usize) < ::core::mem::size_of::<[u32; 4]>() as usize
    {
        byte_num = i.wrapping_rem(4 as u32);
        *dest = (rejectpad[i.wrapping_div(4 as u32) as usize]
            >> byte_num.wrapping_mul(8 as u32)
            & 0xff as u32) as byte;
        dest = dest.offset(1);
        i = i.wrapping_add(1);
    }
    if len as usize > ::core::mem::size_of::<[u32; 4]>() as usize {
        fprintf(
            stderr,
            b"PadRejectArray: REJECT lump too short to pad! (%i > %i)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            len,
            ::core::mem::size_of::<[u32; 4]>() as i32,
        );
        if M_CheckParm("-reject_pad_with_ff") != 0 {
            padvalue = 0xff as u32;
        } else {
            padvalue = 0xf00 as u32;
        }
        memset(
            array
                .offset(
                    ::core::mem::size_of::<[u32; 4]>() as usize as isize,
                ) as *mut ::core::ffi::c_void,
            padvalue as i32,
            (len as size_t)
                .wrapping_sub(
                    ::core::mem::size_of::<[u32; 4]>() as size_t,
                ),
        );
    }
}
unsafe fn P_LoadReject(mut lumpnum: i32) {
    let mut minlength: i32 = 0;
    let mut lumplen: i32 = 0;
    minlength = (numsectors * numsectors + 7 as i32)
        / 8 as i32;
    lumplen = W_LumpLength(lumpnum as u32);
    if lumplen >= minlength {
        rejectmatrix = W_CacheLumpNum(lumpnum, PU_LEVEL as i32)
            as *mut byte;
    } else {
        rejectmatrix = Z_Malloc(
            minlength,
            PU_LEVEL as i32,
            &raw mut rejectmatrix as *mut ::core::ffi::c_void,
        ) as *mut byte;
        W_ReadLump(
            lumpnum as u32,
            rejectmatrix as *mut ::core::ffi::c_void,
        );
        PadRejectArray(
            rejectmatrix.offset(lumplen as isize),
            (minlength - lumplen) as u32,
        );
    };
}
pub unsafe fn P_SetupLevel(
    mut episode: i32,
    mut map: i32,
    mut playermask: i32,
    mut skill: skill_t,
) {
    let mut i: i32 = 0;
    let mut lumpname: [::core::ffi::c_char; 9] = [0; 9];
    let mut lumpnum: i32 = 0;
    wminfo.maxfrags = 0 as i32;
    totalsecret = wminfo.maxfrags;
    totalitems = totalsecret;
    totalkills = totalitems;
    wminfo.partime = 180 as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        players[i as usize].itemcount = 0 as i32;
        players[i as usize].secretcount = players[i as usize].itemcount;
        players[i as usize].killcount = players[i as usize].secretcount;
        i += 1;
    }
    players[consoleplayer as usize].viewz = 1 as i32 as fixed_t;
    S_Start(unsafe { &mut game_state().sounds });
    Z_FreeTags(
        PU_LEVEL as i32,
        PU_PURGELEVEL as i32 - 1 as i32,
    );
    P_InitThinkers();
    if gamemode as u32
        == commercial as i32 as u32
    {
        if map < 10 as i32 {
            snprintf(
                &raw mut lumpname as *mut ::core::ffi::c_char,
                9 as size_t,
                b"map0%i\0" as *const u8 as *const ::core::ffi::c_char,
                map,
            );
        } else {
            snprintf(
                &raw mut lumpname as *mut ::core::ffi::c_char,
                9 as size_t,
                b"map%i\0" as *const u8 as *const ::core::ffi::c_char,
                map,
            );
        }
    } else {
        lumpname[0 as i32 as usize] = 'E' as i32 as ::core::ffi::c_char;
        lumpname[1 as i32 as usize] = ('0' as i32 + episode)
            as ::core::ffi::c_char;
        lumpname[2 as i32 as usize] = 'M' as i32 as ::core::ffi::c_char;
        lumpname[3 as i32 as usize] = ('0' as i32 + map)
            as ::core::ffi::c_char;
        lumpname[4 as i32 as usize] = 0 as ::core::ffi::c_char;
    }
    lumpnum = W_GetNumForName(
        &wad_name8_to_string(&raw mut lumpname as *mut ::core::ffi::c_char),
    );
    leveltime = 0 as i32;
    P_LoadBlockMap(lumpnum + ML_BLOCKMAP as i32);
    P_LoadVertexes(lumpnum + ML_VERTEXES as i32);
    P_LoadSectors(lumpnum + ML_SECTORS as i32);
    P_LoadSideDefs(lumpnum + ML_SIDEDEFS as i32);
    P_LoadLineDefs(lumpnum + ML_LINEDEFS as i32);
    P_LoadSubsectors(lumpnum + ML_SSECTORS as i32);
    P_LoadNodes(lumpnum + ML_NODES as i32);
    P_LoadSegs(lumpnum + ML_SEGS as i32);
    P_GroupLines();
    P_LoadReject(lumpnum + ML_REJECT as i32);
    bodyqueslot = 0 as i32;
    deathmatch_p = &raw mut deathmatchstarts as *mut mapthing_t;
    P_LoadThings(lumpnum + ML_THINGS as i32);
    if deathmatch != 0 {
        i = 0 as i32;
        while i < MAXPLAYERS {
            if playeringame[i as usize] != 0 {
                players[i as usize].mo = ::core::ptr::null_mut::<mobj_t>();
                G_DeathMatchSpawnPlayer(i);
            }
            i += 1;
        }
    }
    let gs = unsafe { game_state() };
    gs.p_mobj.iquetail = 0 as i32;
    gs.p_mobj.iquehead = gs.p_mobj.iquetail;
    P_SpawnSpecials(&mut gs.p_switch, &mut gs.p_plats, &mut gs.p_ceilng);
    if precache {
        R_PrecacheLevel();
    }
}
pub unsafe fn P_Init() {
    P_InitSwitchList(unsafe { &mut game_state().p_switch });
    P_InitPicAnims();
    R_InitSprites(&raw mut sprnames as *mut *mut ::core::ffi::c_char);
}
