use crate::src::p_mobj::{fixed_t, angle_t, vertex_t, sector_t, line_t, byte};

pub type boolean = ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct side_t {
    pub textureoffset: fixed_t,
    pub rowoffset: fixed_t,
    pub toptexture: ::core::ffi::c_short,
    pub bottomtexture: ::core::ffi::c_short,
    pub midtexture: ::core::ffi::c_short,
    pub sector: *mut sector_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub offset: fixed_t,
    pub angle: angle_t,
    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub bbox: [[fixed_t; 4]; 2],
    pub children: [::core::ffi::c_ushort; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawseg_s {
    pub curline: *mut seg_t,
    pub x1: ::core::ffi::c_int,
    pub x2: ::core::ffi::c_int,
    pub scale1: fixed_t,
    pub scale2: fixed_t,
    pub scalestep: fixed_t,
    pub silhouette: ::core::ffi::c_int,
    pub bsilheight: fixed_t,
    pub tsilheight: fixed_t,
    pub sprtopclip: *mut ::core::ffi::c_short,
    pub sprbottomclip: *mut ::core::ffi::c_short,
    pub maskedtexturecol: *mut ::core::ffi::c_short,
}
pub type drawseg_t = drawseg_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct visplane_t {
    pub height: fixed_t,
    pub picnum: ::core::ffi::c_int,
    pub lightlevel: ::core::ffi::c_int,
    pub minx: ::core::ffi::c_int,
    pub maxx: ::core::ffi::c_int,
    pub pad1: byte,
    pub top: [byte; 320],
    pub pad2: byte,
    pub pad3: byte,
    pub bottom: [byte; 320],
    pub pad4: byte,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct spriteframe_t {
    pub rotate: boolean,
    pub lump: [::core::ffi::c_short; 8],
    pub flip: [byte; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct spritedef_t {
    pub numframes: ::core::ffi::c_int,
    pub spriteframes: *mut spriteframe_t,
}
