use ::libc;
use crate::src::i_system::I_Error;
use crate::src::p_pspr::P_SetupPsprites;
use crate::src::p_map::ceilingline;
use crate::src::p_map::P_SlideMove;
use crate::src::st_stuff::ST_Start;
use crate::src::hu_stuff::HU_Start;
use crate::src::s_sound::S_StopSound;
use crate::src::g_game::respawnmonsters;
use crate::src::g_game::G_PlayerReborn;
use crate::src::p_map::attackrange;
use crate::src::g_game::totalkills;
use crate::src::g_game::totalitems;
use crate::src::p_map::P_TryMove;
use crate::src::p_map::linetarget;
use crate::src::p_setup::deathmatchstarts;
use crate::src::p_setup::deathmatch_p;
use crate::src::d_main::nomonsters;
use crate::src::p_map::P_CheckPosition;
use crate::src::p_map::P_AimLineAttack;
use crate::src::p_maputl::P_AproxDistance;
use crate::src::p_maputl::P_UnsetThingPosition;
use crate::src::p_setup::playerstarts;
use crate::src::p_maputl::P_SetThingPosition;
use crate::src::r_main::R_PointInSubsector;
use crate::src::g_game::gameskill;
use crate::src::info::mobjinfo;
use crate::src::p_tick::P_RemoveThinker;
use crate::src::r_sky::skyflatnum;
use crate::src::info::states;
use crate::src::p_tick::P_AddThinker;
use crate::src::r_main::R_PointToAngle2;
use crate::src::g_game::deathmatch;
use crate::src::g_game::playeringame;
use crate::src::m_random::P_Random;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::p_tick::leveltime;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::g_game::players;
use crate::src::s_sound::S_StartSound;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVEL;
use crate::src::sounds::{sfx_itmbk, sfx_oof, sfx_telept};
use crate::src::d_player::CF_NOMOMENTUM;
use crate::src::d_mode::exe_ultimate;
use crate::src::d_mode::{sk_baby, sk_nightmare};
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::stdint_types::size_t;
use crate::src::p_spec::{ceiling_t, floormove_t, plat_t};
use crate::src::p_doors::vldoor_t;
use crate::src::p_lights::{fireflicker_t, lightflash_t, strobe_t, glow_t};
use libc::{memcpy, memset};
use crate::src::info::{S_BLOOD2, S_BLOOD3, S_NULL, S_PLAY, S_PLAY_RUN1, S_PUFF3};

pub use crate::src::d_ticcmd::ticcmd_t;
pub type C2RustUnnamed_0 = u32;
pub const NUMCARDS: C2RustUnnamed_0 = 6;
pub const it_redskull: C2RustUnnamed_0 = 5;
pub const it_yellowskull: C2RustUnnamed_0 = 4;
pub const it_blueskull: C2RustUnnamed_0 = 3;
pub const it_redcard: C2RustUnnamed_0 = 2;
pub const it_yellowcard: C2RustUnnamed_0 = 1;
pub const it_bluecard: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
pub enum StateAction {
    None,
    Mobj(unsafe fn(*mut mobj_t)),
    Weapon(unsafe fn(*mut player_t, *mut pspdef_t)),
}
#[derive(Copy, Clone)]
pub enum ThinkerFn {
    Paused,
    Removed,
    Unresolved,
    Mobj(unsafe fn(*mut mobj_t)),
    Ceiling(unsafe fn(*mut ceiling_t)),
    Door(unsafe fn(*mut vldoor_t)),
    Floor(unsafe fn(*mut floormove_t)),
    Plat(unsafe fn(*mut plat_t)),
    FireFlicker(unsafe fn(*mut fireflicker_t)),
    LightFlash(unsafe fn(*mut lightflash_t)),
    Strobe(unsafe fn(*mut strobe_t)),
    Glow(unsafe fn(*mut glow_t)),
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thinker_s {
    pub prev: *mut thinker_s,
    pub next: *mut thinker_s,
    pub function: ThinkerFn,
}
pub type thinker_t = thinker_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mapthing_t {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub type_0: i16,
    pub options: i16,
}
pub type spritenum_t = u32;
pub const NUMSPRITES: spritenum_t = 138;
pub const SPR_TLP2: spritenum_t = 137;
pub const SPR_TLMP: spritenum_t = 136;
pub const SPR_BRS1: spritenum_t = 135;
pub const SPR_POB2: spritenum_t = 134;
pub const SPR_POB1: spritenum_t = 133;
pub const SPR_HDB6: spritenum_t = 132;
pub const SPR_HDB5: spritenum_t = 131;
pub const SPR_HDB4: spritenum_t = 130;
pub const SPR_HDB3: spritenum_t = 129;
pub const SPR_HDB2: spritenum_t = 128;
pub const SPR_HDB1: spritenum_t = 127;
pub const SPR_SMRT: spritenum_t = 126;
pub const SPR_SMGT: spritenum_t = 125;
pub const SPR_SMBT: spritenum_t = 124;
pub const SPR_TRED: spritenum_t = 123;
pub const SPR_TGRN: spritenum_t = 122;
pub const SPR_TBLU: spritenum_t = 121;
pub const SPR_COL5: spritenum_t = 120;
pub const SPR_FSKU: spritenum_t = 119;
pub const SPR_CEYE: spritenum_t = 118;
pub const SPR_ELEC: spritenum_t = 117;
pub const SPR_TRE2: spritenum_t = 116;
pub const SPR_TRE1: spritenum_t = 115;
pub const SPR_COL6: spritenum_t = 114;
pub const SPR_CBRA: spritenum_t = 113;
pub const SPR_CAND: spritenum_t = 112;
pub const SPR_COL4: spritenum_t = 111;
pub const SPR_COL3: spritenum_t = 110;
pub const SPR_COL2: spritenum_t = 109;
pub const SPR_COL1: spritenum_t = 108;
pub const SPR_SMIT: spritenum_t = 107;
pub const SPR_GOR5: spritenum_t = 106;
pub const SPR_GOR4: spritenum_t = 105;
pub const SPR_GOR3: spritenum_t = 104;
pub const SPR_GOR2: spritenum_t = 103;
pub const SPR_POL6: spritenum_t = 102;
pub const SPR_POL1: spritenum_t = 101;
pub const SPR_POL3: spritenum_t = 100;
pub const SPR_POL4: spritenum_t = 99;
pub const SPR_POL5: spritenum_t = 98;
pub const SPR_POL2: spritenum_t = 97;
pub const SPR_GOR1: spritenum_t = 96;
pub const SPR_SMT2: spritenum_t = 95;
pub const SPR_COLU: spritenum_t = 94;
pub const SPR_SGN2: spritenum_t = 93;
pub const SPR_SHOT: spritenum_t = 92;
pub const SPR_PLAS: spritenum_t = 91;
pub const SPR_LAUN: spritenum_t = 90;
pub const SPR_CSAW: spritenum_t = 89;
pub const SPR_MGUN: spritenum_t = 88;
pub const SPR_BFUG: spritenum_t = 87;
pub const SPR_BPAK: spritenum_t = 86;
pub const SPR_SBOX: spritenum_t = 85;
pub const SPR_SHEL: spritenum_t = 84;
pub const SPR_CELP: spritenum_t = 83;
pub const SPR_CELL: spritenum_t = 82;
pub const SPR_BROK: spritenum_t = 81;
pub const SPR_ROCK: spritenum_t = 80;
pub const SPR_AMMO: spritenum_t = 79;
pub const SPR_CLIP: spritenum_t = 78;
pub const SPR_PVIS: spritenum_t = 77;
pub const SPR_PMAP: spritenum_t = 76;
pub const SPR_SUIT: spritenum_t = 75;
pub const SPR_MEGA: spritenum_t = 74;
pub const SPR_PINS: spritenum_t = 73;
pub const SPR_PSTR: spritenum_t = 72;
pub const SPR_PINV: spritenum_t = 71;
pub const SPR_SOUL: spritenum_t = 70;
pub const SPR_MEDI: spritenum_t = 69;
pub const SPR_STIM: spritenum_t = 68;
pub const SPR_YSKU: spritenum_t = 67;
pub const SPR_RSKU: spritenum_t = 66;
pub const SPR_BSKU: spritenum_t = 65;
pub const SPR_YKEY: spritenum_t = 64;
pub const SPR_RKEY: spritenum_t = 63;
pub const SPR_BKEY: spritenum_t = 62;
pub const SPR_BON2: spritenum_t = 61;
pub const SPR_BON1: spritenum_t = 60;
pub const SPR_FCAN: spritenum_t = 59;
pub const SPR_BEXP: spritenum_t = 58;
pub const SPR_BAR1: spritenum_t = 57;
pub const SPR_ARM2: spritenum_t = 56;
pub const SPR_ARM1: spritenum_t = 55;
pub const SPR_BOSF: spritenum_t = 54;
pub const SPR_BBRN: spritenum_t = 53;
pub const SPR_KEEN: spritenum_t = 52;
pub const SPR_SSWV: spritenum_t = 51;
pub const SPR_PAIN: spritenum_t = 50;
pub const SPR_CYBR: spritenum_t = 49;
pub const SPR_APBX: spritenum_t = 48;
pub const SPR_APLS: spritenum_t = 47;
pub const SPR_BSPI: spritenum_t = 46;
pub const SPR_SPID: spritenum_t = 45;
pub const SPR_SKUL: spritenum_t = 44;
pub const SPR_BOS2: spritenum_t = 43;
pub const SPR_BOSS: spritenum_t = 42;
pub const SPR_BAL7: spritenum_t = 41;
pub const SPR_HEAD: spritenum_t = 40;
pub const SPR_SARG: spritenum_t = 39;
pub const SPR_CPOS: spritenum_t = 38;
pub const SPR_FATT: spritenum_t = 37;
pub const SPR_MANF: spritenum_t = 36;
pub const SPR_SKEL: spritenum_t = 35;
pub const SPR_FBXP: spritenum_t = 34;
pub const SPR_FATB: spritenum_t = 33;
pub const SPR_FIRE: spritenum_t = 32;
pub const SPR_VILE: spritenum_t = 31;
pub const SPR_SPOS: spritenum_t = 30;
pub const SPR_POSS: spritenum_t = 29;
pub const SPR_PLAY: spritenum_t = 28;
pub const SPR_IFOG: spritenum_t = 27;
pub const SPR_TFOG: spritenum_t = 26;
pub const SPR_BFE2: spritenum_t = 25;
pub const SPR_BFE1: spritenum_t = 24;
pub const SPR_BFS1: spritenum_t = 23;
pub const SPR_MISL: spritenum_t = 22;
pub const SPR_PLSE: spritenum_t = 21;
pub const SPR_PLSS: spritenum_t = 20;
pub const SPR_BAL2: spritenum_t = 19;
pub const SPR_BAL1: spritenum_t = 18;
pub const SPR_PUFF: spritenum_t = 17;
pub const SPR_BLUD: spritenum_t = 16;
pub const SPR_BFGF: spritenum_t = 15;
pub const SPR_BFGG: spritenum_t = 14;
pub const SPR_PLSF: spritenum_t = 13;
pub const SPR_PLSG: spritenum_t = 12;
pub const SPR_SAWG: spritenum_t = 11;
pub const SPR_MISF: spritenum_t = 10;
pub const SPR_MISG: spritenum_t = 9;
pub const SPR_CHGF: spritenum_t = 8;
pub const SPR_CHGG: spritenum_t = 7;
pub const SPR_SHT2: spritenum_t = 6;
pub const SPR_SHTF: spritenum_t = 5;
pub const SPR_PISF: spritenum_t = 4;
pub const SPR_PISG: spritenum_t = 3;
pub const SPR_PUNG: spritenum_t = 2;
pub const SPR_SHTG: spritenum_t = 1;
pub const SPR_TROO: spritenum_t = 0;
pub type statenum_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t {
    pub sprite: spritenum_t,
    pub frame: i32,
    pub tics: i32,
    pub action: StateAction,
    pub nextstate: statenum_t,
    pub misc1: i32,
    pub misc2: i32,
}
pub type mobjtype_t = u32;
pub const NUMMOBJTYPES: mobjtype_t = 137;
pub const MT_MISC86: mobjtype_t = 136;
pub const MT_MISC85: mobjtype_t = 135;
pub const MT_MISC84: mobjtype_t = 134;
pub const MT_MISC83: mobjtype_t = 133;
pub const MT_MISC82: mobjtype_t = 132;
pub const MT_MISC81: mobjtype_t = 131;
pub const MT_MISC80: mobjtype_t = 130;
pub const MT_MISC79: mobjtype_t = 129;
pub const MT_MISC78: mobjtype_t = 128;
pub const MT_MISC77: mobjtype_t = 127;
pub const MT_MISC76: mobjtype_t = 126;
pub const MT_MISC75: mobjtype_t = 125;
pub const MT_MISC74: mobjtype_t = 124;
pub const MT_MISC73: mobjtype_t = 123;
pub const MT_MISC72: mobjtype_t = 122;
pub const MT_MISC71: mobjtype_t = 121;
pub const MT_MISC70: mobjtype_t = 120;
pub const MT_MISC69: mobjtype_t = 119;
pub const MT_MISC68: mobjtype_t = 118;
pub const MT_MISC67: mobjtype_t = 117;
pub const MT_MISC66: mobjtype_t = 116;
pub const MT_MISC65: mobjtype_t = 115;
pub const MT_MISC64: mobjtype_t = 114;
pub const MT_MISC63: mobjtype_t = 113;
pub const MT_MISC62: mobjtype_t = 112;
pub const MT_MISC61: mobjtype_t = 111;
pub const MT_MISC60: mobjtype_t = 110;
pub const MT_MISC59: mobjtype_t = 109;
pub const MT_MISC58: mobjtype_t = 108;
pub const MT_MISC57: mobjtype_t = 107;
pub const MT_MISC56: mobjtype_t = 106;
pub const MT_MISC55: mobjtype_t = 105;
pub const MT_MISC54: mobjtype_t = 104;
pub const MT_MISC53: mobjtype_t = 103;
pub const MT_MISC52: mobjtype_t = 102;
pub const MT_MISC51: mobjtype_t = 101;
pub const MT_MISC50: mobjtype_t = 100;
pub const MT_MISC49: mobjtype_t = 99;
pub const MT_MISC48: mobjtype_t = 98;
pub const MT_MISC47: mobjtype_t = 97;
pub const MT_MISC46: mobjtype_t = 96;
pub const MT_MISC45: mobjtype_t = 95;
pub const MT_MISC44: mobjtype_t = 94;
pub const MT_MISC43: mobjtype_t = 93;
pub const MT_MISC42: mobjtype_t = 92;
pub const MT_MISC41: mobjtype_t = 91;
pub const MT_MISC40: mobjtype_t = 90;
pub const MT_MISC39: mobjtype_t = 89;
pub const MT_MISC38: mobjtype_t = 88;
pub const MT_MISC37: mobjtype_t = 87;
pub const MT_MISC36: mobjtype_t = 86;
pub const MT_MISC35: mobjtype_t = 85;
pub const MT_MISC34: mobjtype_t = 84;
pub const MT_MISC33: mobjtype_t = 83;
pub const MT_MISC32: mobjtype_t = 82;
pub const MT_MISC31: mobjtype_t = 81;
pub const MT_MISC30: mobjtype_t = 80;
pub const MT_MISC29: mobjtype_t = 79;
pub const MT_SUPERSHOTGUN: mobjtype_t = 78;
pub const MT_SHOTGUN: mobjtype_t = 77;
pub const MT_MISC28: mobjtype_t = 76;
pub const MT_MISC27: mobjtype_t = 75;
pub const MT_MISC26: mobjtype_t = 74;
pub const MT_CHAINGUN: mobjtype_t = 73;
pub const MT_MISC25: mobjtype_t = 72;
pub const MT_MISC24: mobjtype_t = 71;
pub const MT_MISC23: mobjtype_t = 70;
pub const MT_MISC22: mobjtype_t = 69;
pub const MT_MISC21: mobjtype_t = 68;
pub const MT_MISC20: mobjtype_t = 67;
pub const MT_MISC19: mobjtype_t = 66;
pub const MT_MISC18: mobjtype_t = 65;
pub const MT_MISC17: mobjtype_t = 64;
pub const MT_CLIP: mobjtype_t = 63;
pub const MT_MEGA: mobjtype_t = 62;
pub const MT_MISC16: mobjtype_t = 61;
pub const MT_MISC15: mobjtype_t = 60;
pub const MT_MISC14: mobjtype_t = 59;
pub const MT_INS: mobjtype_t = 58;
pub const MT_MISC13: mobjtype_t = 57;
pub const MT_INV: mobjtype_t = 56;
pub const MT_MISC12: mobjtype_t = 55;
pub const MT_MISC11: mobjtype_t = 54;
pub const MT_MISC10: mobjtype_t = 53;
pub const MT_MISC9: mobjtype_t = 52;
pub const MT_MISC8: mobjtype_t = 51;
pub const MT_MISC7: mobjtype_t = 50;
pub const MT_MISC6: mobjtype_t = 49;
pub const MT_MISC5: mobjtype_t = 48;
pub const MT_MISC4: mobjtype_t = 47;
pub const MT_MISC3: mobjtype_t = 46;
pub const MT_MISC2: mobjtype_t = 45;
pub const MT_MISC1: mobjtype_t = 44;
pub const MT_MISC0: mobjtype_t = 43;
pub const MT_EXTRABFG: mobjtype_t = 42;
pub const MT_TELEPORTMAN: mobjtype_t = 41;
pub const MT_IFOG: mobjtype_t = 40;
pub const MT_TFOG: mobjtype_t = 39;
pub const MT_BLOOD: mobjtype_t = 38;
pub const MT_PUFF: mobjtype_t = 37;
pub const MT_ARACHPLAZ: mobjtype_t = 36;
pub const MT_BFG: mobjtype_t = 35;
pub const MT_PLASMA: mobjtype_t = 34;
pub const MT_ROCKET: mobjtype_t = 33;
pub const MT_HEADSHOT: mobjtype_t = 32;
pub const MT_TROOPSHOT: mobjtype_t = 31;
pub const MT_BARREL: mobjtype_t = 30;
pub const MT_SPAWNFIRE: mobjtype_t = 29;
pub const MT_SPAWNSHOT: mobjtype_t = 28;
pub const MT_BOSSTARGET: mobjtype_t = 27;
pub const MT_BOSSSPIT: mobjtype_t = 26;
pub const MT_BOSSBRAIN: mobjtype_t = 25;
pub const MT_KEEN: mobjtype_t = 24;
pub const MT_WOLFSS: mobjtype_t = 23;
pub const MT_PAIN: mobjtype_t = 22;
pub const MT_CYBORG: mobjtype_t = 21;
pub const MT_BABY: mobjtype_t = 20;
pub const MT_SPIDER: mobjtype_t = 19;
pub const MT_SKULL: mobjtype_t = 18;
pub const MT_KNIGHT: mobjtype_t = 17;
pub const MT_BRUISERSHOT: mobjtype_t = 16;
pub const MT_BRUISER: mobjtype_t = 15;
pub const MT_HEAD: mobjtype_t = 14;
pub const MT_SHADOWS: mobjtype_t = 13;
pub const MT_SERGEANT: mobjtype_t = 12;
pub const MT_TROOP: mobjtype_t = 11;
pub const MT_CHAINGUY: mobjtype_t = 10;
pub const MT_FATSHOT: mobjtype_t = 9;
pub const MT_FATSO: mobjtype_t = 8;
pub const MT_SMOKE: mobjtype_t = 7;
pub const MT_TRACER: mobjtype_t = 6;
pub const MT_UNDEAD: mobjtype_t = 5;
pub const MT_FIRE: mobjtype_t = 4;
pub const MT_VILE: mobjtype_t = 3;
pub const MT_SHOTGUY: mobjtype_t = 2;
pub const MT_POSSESSED: mobjtype_t = 1;
pub const MT_PLAYER: mobjtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mobjinfo_t {
    pub doomednum: i32,
    pub spawnstate: i32,
    pub spawnhealth: i32,
    pub seestate: i32,
    pub seesound: i32,
    pub reactiontime: i32,
    pub attacksound: i32,
    pub painstate: i32,
    pub painchance: i32,
    pub painsound: i32,
    pub meleestate: i32,
    pub missilestate: i32,
    pub deathstate: i32,
    pub xdeathstate: i32,
    pub deathsound: i32,
    pub speed: i32,
    pub radius: i32,
    pub height: i32,
    pub mass: i32,
    pub damage: i32,
    pub activesound: i32,
    pub flags: i32,
    pub raisestate: i32,
}
pub type C2RustUnnamed_1 = u32;
pub const MF_TRANSSHIFT: C2RustUnnamed_1 = 26;
pub const MF_TRANSLATION: C2RustUnnamed_1 = 201326592;
pub const MF_NOTDMATCH: C2RustUnnamed_1 = 33554432;
pub const MF_SKULLFLY: C2RustUnnamed_1 = 16777216;
pub const MF_COUNTITEM: C2RustUnnamed_1 = 8388608;
pub const MF_COUNTKILL: C2RustUnnamed_1 = 4194304;
pub const MF_INFLOAT: C2RustUnnamed_1 = 2097152;
pub const MF_CORPSE: C2RustUnnamed_1 = 1048576;
pub const MF_NOBLOOD: C2RustUnnamed_1 = 524288;
pub const MF_SHADOW: C2RustUnnamed_1 = 262144;
pub const MF_DROPPED: C2RustUnnamed_1 = 131072;
pub const MF_MISSILE: C2RustUnnamed_1 = 65536;
pub const MF_TELEPORT: C2RustUnnamed_1 = 32768;
pub const MF_FLOAT: C2RustUnnamed_1 = 16384;
pub const MF_SLIDE: C2RustUnnamed_1 = 8192;
pub const MF_NOCLIP: C2RustUnnamed_1 = 4096;
pub const MF_PICKUP: C2RustUnnamed_1 = 2048;
pub const MF_DROPOFF: C2RustUnnamed_1 = 1024;
pub const MF_NOGRAVITY: C2RustUnnamed_1 = 512;
pub const MF_SPAWNCEILING: C2RustUnnamed_1 = 256;
pub const MF_JUSTATTACKED: C2RustUnnamed_1 = 128;
pub const MF_JUSTHIT: C2RustUnnamed_1 = 64;
pub const MF_AMBUSH: C2RustUnnamed_1 = 32;
pub const MF_NOBLOCKMAP: C2RustUnnamed_1 = 16;
pub const MF_NOSECTOR: C2RustUnnamed_1 = 8;
pub const MF_SHOOTABLE: C2RustUnnamed_1 = 4;
pub const MF_SOLID: C2RustUnnamed_1 = 2;
pub const MF_SPECIAL: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mobj_s {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub snext: *mut mobj_s,
    pub sprev: *mut mobj_s,
    pub angle: angle_t,
    pub sprite: spritenum_t,
    pub frame: i32,
    pub bnext: *mut mobj_s,
    pub bprev: *mut mobj_s,
    pub subsector: *mut subsector_s,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
    pub validcount: i32,
    pub type_0: mobjtype_t,
    pub info: *mut mobjinfo_t,
    pub tics: i32,
    pub state: *mut state_t,
    pub flags: i32,
    pub health: i32,
    pub movedir: i32,
    pub movecount: i32,
    pub target: *mut mobj_s,
    pub reactiontime: i32,
    pub threshold: i32,
    pub player: *mut player_s,
    pub lastlook: i32,
    pub spawnpoint: mapthing_t,
    pub tracer: *mut mobj_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspdef_t {
    pub state: *mut state_t,
    pub tics: i32,
    pub sx: fixed_t,
    pub sy: fixed_t,
}
pub type mobj_t = mobj_s;
pub use crate::src::d_player::{player_s, player_t, playerstate_t, PST_DEAD, PST_LIVE, PST_REBORN};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subsector_s {
    pub sector: *mut sector_t,
    pub numlines: i16,
    pub firstline: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sector_t {
    pub floorheight: fixed_t,
    pub ceilingheight: fixed_t,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    pub soundtraversed: i32,
    pub soundtarget: *mut mobj_t,
    pub blockbox: [i32; 4],
    pub soundorg: degenmobj_t,
    pub validcount: i32,
    pub thinglist: *mut mobj_t,
    pub specialdata: *mut ::core::ffi::c_void,
    pub linecount: i32,
    pub lines: *mut *mut line_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_s {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [fixed_t; 4],
    pub slopetype: slopetype_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: i32,
    pub specialdata: *mut ::core::ffi::c_void,
}
pub type slopetype_t = u32;
pub const ST_NEGATIVE: slopetype_t = 3;
pub const ST_POSITIVE: slopetype_t = 2;
pub const ST_VERTICAL: slopetype_t = 1;
pub const ST_HORIZONTAL: slopetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertex_t {
    pub x: fixed_t,
    pub y: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct degenmobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}
pub type line_t = line_s;
pub type subsector_t = subsector_s;
pub const INT_MAX: i32 = __INT_MAX__;
pub const INT_MIN: i32 = -__INT_MAX__ - 1 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TICRATE: i32 = 35 as i32;
pub const MAXPLAYERS: i32 = 4 as i32;
pub const MTF_AMBUSH: i32 = 8 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub const FRACUNIT: i32 = (1 as i32) << FRACBITS;
pub const ANGLETOFINESHIFT: i32 = 19 as i32;
pub const ANG45: i32 = 0x20000000 as i32;
pub const FLOATSPEED: i32 = FRACUNIT * 4 as i32;
pub const VIEWHEIGHT: i32 = 41 as i32 * FRACUNIT;
pub const GRAVITY: i32 = FRACUNIT;
pub const MAXMOVE: i32 = 30 as i32 * FRACUNIT;
pub const MELEERANGE: i32 = 64 as i32 * FRACUNIT;
pub const ONFLOORZ: i32 = INT_MIN;
pub const ONCEILINGZ: i32 = INT_MAX;
pub const ITEMQUESIZE: i32 = 128 as i32;
#[no_mangle]
pub static mut test: i32 = 0;
pub unsafe fn P_SetMobjState(
    mut mobj: *mut mobj_t,
    mut state: statenum_t,
) -> bool {
    let mut st: *mut state_t = ::core::ptr::null_mut::<state_t>();
    loop {
        if state as u32
            == S_NULL as i32 as u32
        {
            (*mobj).state = ::core::ptr::null_mut::<state_t>();
            P_RemoveMobj(mobj);
            return false;
        }
        st = (&raw mut states as *mut state_t).offset(state as isize) as *mut state_t;
        (*mobj).state = st;
        (*mobj).tics = (*st).tics;
        (*mobj).sprite = (*st).sprite;
        (*mobj).frame = (*st).frame;
        if let StateAction::Mobj(f) = (*st).action {
            f(mobj);
        }
        state = (*st).nextstate;
        if !((*mobj).tics == 0) {
            break;
        }
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn P_ExplodeMissile(mut mo: *mut mobj_t) {
    (*mo).momz = 0 as i32 as fixed_t;
    (*mo).momy = (*mo).momz;
    (*mo).momx = (*mo).momy;
    P_SetMobjState(mo, mobjinfo[(*mo).type_0 as usize].deathstate as statenum_t);
    (*mo).tics -= P_Random() & 3 as i32;
    if (*mo).tics < 1 as i32 {
        (*mo).tics = 1 as i32;
    }
    (*mo).flags &= !(MF_MISSILE as i32);
    if (*(*mo).info).deathsound != 0 {
        S_StartSound(mo as *mut ::core::ffi::c_void, (*(*mo).info).deathsound);
    }
}
pub const STOPSPEED: i32 = 0x1000 as i32;
pub const FRICTION: i32 = 0xe800 as i32;
#[no_mangle]
pub unsafe extern "C" fn P_XYMovement(mut mo: *mut mobj_t) {
    let mut ptryx: fixed_t = 0;
    let mut ptryy: fixed_t = 0;
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut xmove: fixed_t = 0;
    let mut ymove: fixed_t = 0;
    if (*mo).momx == 0 && (*mo).momy == 0 {
        if (*mo).flags & MF_SKULLFLY as i32 != 0 {
            (*mo).flags &= !(MF_SKULLFLY as i32);
            (*mo).momz = 0 as i32 as fixed_t;
            (*mo).momy = (*mo).momz;
            (*mo).momx = (*mo).momy;
            P_SetMobjState(mo, (*(*mo).info).spawnstate as statenum_t);
        }
        return;
    }
    player = (*mo).player as *mut player_t;
    if (*mo).momx > MAXMOVE {
        (*mo).momx = MAXMOVE as fixed_t;
    } else if (*mo).momx < -MAXMOVE {
        (*mo).momx = -MAXMOVE as fixed_t;
    }
    if (*mo).momy > MAXMOVE {
        (*mo).momy = MAXMOVE as fixed_t;
    } else if (*mo).momy < -MAXMOVE {
        (*mo).momy = -MAXMOVE as fixed_t;
    }
    xmove = (*mo).momx;
    ymove = (*mo).momy;
    loop {
        if xmove > MAXMOVE / 2 as i32
            || ymove > MAXMOVE / 2 as i32
        {
            ptryx = ((*mo).x as i32
                + xmove as i32 / 2 as i32) as fixed_t;
            ptryy = ((*mo).y as i32
                + ymove as i32 / 2 as i32) as fixed_t;
            xmove >>= 1 as i32;
            ymove >>= 1 as i32;
        } else {
            ptryx = (*mo).x + xmove;
            ptryy = (*mo).y + ymove;
            ymove = 0 as i32 as fixed_t;
            xmove = ymove;
        }
        if !P_TryMove(mo, ptryx, ptryy) {
            if !(*mo).player.is_null() {
                P_SlideMove(mo);
            } else if (*mo).flags & MF_MISSILE as i32 != 0 {
                if !ceilingline.is_null() && !(*ceilingline).backsector.is_null()
                    && (*(*ceilingline).backsector).ceilingpic as i32
                        == skyflatnum
                {
                    P_RemoveMobj(mo);
                    return;
                }
                P_ExplodeMissile(mo);
            } else {
                (*mo).momy = 0 as i32 as fixed_t;
                (*mo).momx = (*mo).momy;
            }
        }
        if !(xmove != 0 || ymove != 0) {
            break;
        }
    }
    if !player.is_null() && (*player).cheats & CF_NOMOMENTUM as i32 != 0 {
        (*mo).momy = 0 as i32 as fixed_t;
        (*mo).momx = (*mo).momy;
        return;
    }
    if (*mo).flags
        & (MF_MISSILE as i32 | MF_SKULLFLY as i32) != 0
    {
        return;
    }
    if (*mo).z > (*mo).floorz {
        return;
    }
    if (*mo).flags & MF_CORPSE as i32 != 0 {
        if (*mo).momx > FRACUNIT / 4 as i32
            || (*mo).momx < -FRACUNIT / 4 as i32
            || (*mo).momy > FRACUNIT / 4 as i32
            || (*mo).momy < -FRACUNIT / 4 as i32
        {
            if (*mo).floorz != (*(*(*mo).subsector).sector).floorheight {
                return;
            }
        }
    }
    if (*mo).momx > -STOPSPEED && (*mo).momx < STOPSPEED && (*mo).momy > -STOPSPEED
        && (*mo).momy < STOPSPEED
        && (player.is_null()
            || (*player).cmd.forwardmove as i32 == 0 as i32
                && (*player).cmd.sidemove as i32
                    == 0 as i32)
    {
        if !player.is_null()
            && (((*(*player).mo).state.offset_from(&raw mut states as *mut state_t)
                as i64
                - S_PLAY_RUN1 as i32 as i64)
                as u32) < 4 as u32
        {
            P_SetMobjState((*player).mo, S_PLAY);
        }
        (*mo).momx = 0 as i32 as fixed_t;
        (*mo).momy = 0 as i32 as fixed_t;
    } else {
        (*mo).momx = FixedMul((*mo).momx, FRICTION);
        (*mo).momy = FixedMul((*mo).momy, FRICTION);
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_ZMovement(mut mo: *mut mobj_t) {
    let mut dist: fixed_t = 0;
    let mut delta: fixed_t = 0;
    if !(*mo).player.is_null() && (*mo).z < (*mo).floorz {
        (*(*mo).player).viewheight -= (*mo).floorz - (*mo).z;
        (*(*mo).player).deltaviewheight = VIEWHEIGHT - (*(*mo).player).viewheight
            >> 3 as i32;
    }
    (*mo).z += (*mo).momz;
    if (*mo).flags & MF_FLOAT as i32 != 0 && !(*mo).target.is_null() {
        if (*mo).flags & MF_SKULLFLY as i32 == 0
            && (*mo).flags & MF_INFLOAT as i32 == 0
        {
            dist = P_AproxDistance(
                (*mo).x - (*(*mo).target).x,
                (*mo).y - (*(*mo).target).y,
            );
            delta = (*(*mo).target).z + ((*mo).height >> 1 as i32)
                - (*mo).z;
            if delta < 0 as i32
                && dist < -(delta as i32 * 3 as i32)
            {
                (*mo).z -= FLOATSPEED;
            } else if delta > 0 as i32
                && dist < delta as i32 * 3 as i32
            {
                (*mo).z += FLOATSPEED;
            }
        }
    }
    if (*mo).z <= (*mo).floorz {
        let mut correct_lost_soul_bounce: i32 = (gameversion
            as u32
            >= exe_ultimate as i32 as u32)
            as i32;
        if correct_lost_soul_bounce != 0
            && (*mo).flags & MF_SKULLFLY as i32 != 0
        {
            (*mo).momz = -(*mo).momz;
        }
        if (*mo).momz < 0 as i32 {
            if !(*mo).player.is_null() && (*mo).momz < -GRAVITY * 8 as i32
            {
                (*(*mo).player).deltaviewheight = (*mo).momz >> 3 as i32;
                S_StartSound(
                    mo as *mut ::core::ffi::c_void,
                    sfx_oof as i32,
                );
            }
            (*mo).momz = 0 as i32 as fixed_t;
        }
        (*mo).z = (*mo).floorz;
        if correct_lost_soul_bounce == 0
            && (*mo).flags & MF_SKULLFLY as i32 != 0
        {
            (*mo).momz = -(*mo).momz;
        }
        if (*mo).flags & MF_MISSILE as i32 != 0
            && (*mo).flags & MF_NOCLIP as i32 == 0
        {
            P_ExplodeMissile(mo);
            return;
        }
    } else if (*mo).flags & MF_NOGRAVITY as i32 == 0 {
        if (*mo).momz == 0 as i32 {
            (*mo).momz = (-GRAVITY * 2 as i32) as fixed_t;
        } else {
            (*mo).momz -= GRAVITY;
        }
    }
    if (*mo).z + (*mo).height > (*mo).ceilingz {
        if (*mo).momz > 0 as i32 {
            (*mo).momz = 0 as i32 as fixed_t;
        }
        (*mo).z = (*mo).ceilingz - (*mo).height;
        if (*mo).flags & MF_SKULLFLY as i32 != 0 {
            (*mo).momz = -(*mo).momz;
        }
        if (*mo).flags & MF_MISSILE as i32 != 0
            && (*mo).flags & MF_NOCLIP as i32 == 0
        {
            P_ExplodeMissile(mo);
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_NightmareRespawn(mut mobj: *mut mobj_t) {
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut mthing: *mut mapthing_t = ::core::ptr::null_mut::<mapthing_t>();
    x = (((*mobj).spawnpoint.x as i32) << FRACBITS) as fixed_t;
    y = (((*mobj).spawnpoint.y as i32) << FRACBITS) as fixed_t;
    if !P_CheckPosition(mobj, x, y) {
        return;
    }
    mo = P_SpawnMobj(
        (*mobj).x,
        (*mobj).y,
        (*(*(*mobj).subsector).sector).floorheight,
        MT_TFOG,
    );
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_telept as i32);
    ss = R_PointInSubsector(x, y);
    mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_TFOG);
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_telept as i32);
    mthing = &raw mut (*mobj).spawnpoint;
    if (*(*mobj).info).flags & MF_SPAWNCEILING as i32 != 0 {
        z = ONCEILINGZ as fixed_t;
    } else {
        z = ONFLOORZ as fixed_t;
    }
    mo = P_SpawnMobj(x, y, z, (*mobj).type_0);
    (*mo).spawnpoint = (*mobj).spawnpoint;
    (*mo).angle = (ANG45
        * ((*mthing).angle as i32 / 45 as i32)) as angle_t;
    if (*mthing).options as i32 & MTF_AMBUSH != 0 {
        (*mo).flags |= MF_AMBUSH as i32;
    }
    (*mo).reactiontime = 18 as i32;
    P_RemoveMobj(mobj);
}
pub unsafe fn P_MobjThinker(mut mobj: *mut mobj_t) {
    if (*mobj).momx != 0 || (*mobj).momy != 0
        || (*mobj).flags & MF_SKULLFLY as i32 != 0
    {
        P_XYMovement(mobj);
        if matches!((*mobj).thinker.function, ThinkerFn::Removed) {
            return;
        }
    }
    if (*mobj).z != (*mobj).floorz || (*mobj).momz != 0 {
        P_ZMovement(mobj);
        if matches!((*mobj).thinker.function, ThinkerFn::Removed) {
            return;
        }
    }
    if (*mobj).tics != -(1 as i32) {
        (*mobj).tics -= 1;
        if (*mobj).tics == 0 {
            if !P_SetMobjState(mobj, (*(*mobj).state).nextstate) {
                return;
            }
        }
    } else {
        if (*mobj).flags & MF_COUNTKILL as i32 == 0 {
            return;
        }
        if !respawnmonsters {
            return;
        }
        (*mobj).movecount += 1;
        if (*mobj).movecount < 12 as i32 * TICRATE {
            return;
        }
        if leveltime & 31 as i32 != 0 {
            return;
        }
        if P_Random() > 4 as i32 {
            return;
        }
        P_NightmareRespawn(mobj);
    };
}
pub unsafe fn P_SpawnMobj(
    mut x: fixed_t,
    mut y: fixed_t,
    mut z: fixed_t,
    mut type_0: mobjtype_t,
) -> *mut mobj_t {
    let mut mobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut st: *mut state_t = ::core::ptr::null_mut::<state_t>();
    let mut info: *mut mobjinfo_t = ::core::ptr::null_mut::<mobjinfo_t>();
    mobj = Z_Malloc(
        ::core::mem::size_of::<mobj_t>() as i32,
        PU_LEVEL as i32,
        NULL,
    ) as *mut mobj_t;
    memset(
        mobj as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<mobj_t>() as size_t,
    );
    info = (&raw mut mobjinfo as *mut mobjinfo_t).offset(type_0 as isize)
        as *mut mobjinfo_t;
    (*mobj).type_0 = type_0;
    (*mobj).info = info;
    (*mobj).x = x;
    (*mobj).y = y;
    (*mobj).radius = (*info).radius as fixed_t;
    (*mobj).height = (*info).height as fixed_t;
    (*mobj).flags = (*info).flags;
    (*mobj).health = (*info).spawnhealth;
    if gameskill as i32 != sk_nightmare as i32 {
        (*mobj).reactiontime = (*info).reactiontime;
    }
    (*mobj).lastlook = P_Random() % MAXPLAYERS;
    st = (&raw mut states as *mut state_t).offset((*info).spawnstate as isize)
        as *mut state_t;
    (*mobj).state = st;
    (*mobj).tics = (*st).tics;
    (*mobj).sprite = (*st).sprite;
    (*mobj).frame = (*st).frame;
    P_SetThingPosition(mobj);
    (*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
    (*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
    if z == ONFLOORZ {
        (*mobj).z = (*mobj).floorz;
    } else if z == ONCEILINGZ {
        (*mobj).z = ((*mobj).ceilingz as i32 - (*(*mobj).info).height)
            as fixed_t;
    } else {
        (*mobj).z = z;
    }
    (*mobj).thinker.function = ThinkerFn::Mobj(P_MobjThinker);
    P_AddThinker(&raw mut (*mobj).thinker);
    return mobj;
}
#[no_mangle]
pub static mut itemrespawnque: [mapthing_t; 128] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    type_0: 0,
    options: 0,
}; 128];
#[no_mangle]
pub static mut itemrespawntime: [i32; 128] = [0; 128];
pub static mut iquehead: i32 = 0;
pub static mut iquetail: i32 = 0;
pub unsafe fn P_RemoveMobj(mut mobj: *mut mobj_t) {
    if (*mobj).flags & MF_SPECIAL as i32 != 0
        && (*mobj).flags & MF_DROPPED as i32 == 0
        && (*mobj).type_0 as u32
            != MT_INV as i32 as u32
        && (*mobj).type_0 as u32
            != MT_INS as i32 as u32
    {
        itemrespawnque[iquehead as usize] = (*mobj).spawnpoint;
        itemrespawntime[iquehead as usize] = leveltime;
        iquehead = iquehead + 1 as i32
            & ITEMQUESIZE - 1 as i32;
        if iquehead == iquetail {
            iquetail = iquetail + 1 as i32
                & ITEMQUESIZE - 1 as i32;
        }
    }
    P_UnsetThingPosition(mobj);
    S_StopSound(mobj);
    P_RemoveThinker(mobj as *mut thinker_t);
}
pub unsafe fn P_RespawnSpecials() {
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut ss: *mut subsector_t = ::core::ptr::null_mut::<subsector_t>();
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut mthing: *mut mapthing_t = ::core::ptr::null_mut::<mapthing_t>();
    let mut i: i32 = 0;
    if deathmatch != 2 as i32 {
        return;
    }
    if iquehead == iquetail {
        return;
    }
    if leveltime - itemrespawntime[iquetail as usize]
        < 30 as i32 * TICRATE
    {
        return;
    }
    mthing = (&raw mut itemrespawnque as *mut mapthing_t).offset(iquetail as isize)
        as *mut mapthing_t;
    x = (((*mthing).x as i32) << FRACBITS) as fixed_t;
    y = (((*mthing).y as i32) << FRACBITS) as fixed_t;
    ss = R_PointInSubsector(x, y);
    mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_IFOG);
    S_StartSound(mo as *mut ::core::ffi::c_void, sfx_itmbk as i32);
    i = 0 as i32;
    while i < NUMMOBJTYPES as i32 {
        if (*mthing).type_0 as i32 == mobjinfo[i as usize].doomednum {
            break;
        }
        i += 1;
    }
    if mobjinfo[i as usize].flags & MF_SPAWNCEILING as i32 != 0 {
        z = ONCEILINGZ as fixed_t;
    } else {
        z = ONFLOORZ as fixed_t;
    }
    mo = P_SpawnMobj(x, y, z, i as mobjtype_t);
    (*mo).spawnpoint = *mthing;
    (*mo).angle = (ANG45
        * ((*mthing).angle as i32 / 45 as i32)) as angle_t;
    iquetail = iquetail + 1 as i32
        & ITEMQUESIZE - 1 as i32;
}
pub unsafe fn P_SpawnPlayer(mut mthing: *mut mapthing_t) {
    let mut p: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut mobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut i: i32 = 0;
    if (*mthing).type_0 as i32 == 0 as i32 {
        return;
    }
    if playeringame[((*mthing).type_0 as i32 - 1 as i32)
        as usize] == 0
    {
        return;
    }
    p = (&raw mut players as *mut player_t)
        .offset(
            ((*mthing).type_0 as i32 - 1 as i32) as isize,
        ) as *mut player_t;
    if (*p).playerstate as u32
        == PST_REBORN as i32 as u32
    {
        G_PlayerReborn((*mthing).type_0 as i32 - 1 as i32);
    }
    x = (((*mthing).x as i32) << FRACBITS) as fixed_t;
    y = (((*mthing).y as i32) << FRACBITS) as fixed_t;
    z = ONFLOORZ as fixed_t;
    mobj = P_SpawnMobj(x, y, z, MT_PLAYER);
    if (*mthing).type_0 as i32 > 1 as i32 {
        (*mobj).flags
            |= ((*mthing).type_0 as i32 - 1 as i32)
                << MF_TRANSSHIFT as i32;
    }
    (*mobj).angle = (ANG45
        * ((*mthing).angle as i32 / 45 as i32)) as angle_t;
    (*mobj).player = p as *mut player_s;
    (*mobj).health = (*p).health;
    (*p).mo = mobj;
    (*p).playerstate = PST_LIVE;
    (*p).refire = 0 as i32;
    (*p).message = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*p).damagecount = 0 as i32;
    (*p).bonuscount = 0 as i32;
    (*p).extralight = 0 as i32;
    (*p).fixedcolormap = 0 as i32;
    (*p).viewheight = VIEWHEIGHT as fixed_t;
    P_SetupPsprites(p);
    if deathmatch != 0 {
        i = 0 as i32;
        while i < NUMCARDS as i32 {
            (*p).cards[i as usize] = true;
            i += 1;
        }
    }
    if (*mthing).type_0 as i32 - 1 as i32 == consoleplayer
    {
        ST_Start();
        HU_Start();
    }
}
pub unsafe fn P_SpawnMapThing(mut mthing: *mut mapthing_t) {
    let mut i: i32 = 0;
    let mut bit: i32 = 0;
    let mut mobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    if (*mthing).type_0 as i32 == 11 as i32 {
        if deathmatch_p
            < (&raw mut deathmatchstarts as *mut mapthing_t)
                .offset(10 as i32 as isize) as *mut mapthing_t
        {
            memcpy(
                deathmatch_p as *mut ::core::ffi::c_void,
                mthing as *const ::core::ffi::c_void,
                ::core::mem::size_of::<mapthing_t>() as size_t,
            );
            deathmatch_p = deathmatch_p.offset(1);
        }
        return;
    }
    if (*mthing).type_0 as i32 <= 0 as i32 {
        return;
    }
    if (*mthing).type_0 as i32 <= 4 as i32 {
        playerstarts[((*mthing).type_0 as i32 - 1 as i32)
            as usize] = *mthing;
        if deathmatch == 0 {
            P_SpawnPlayer(mthing);
        }
        return;
    }
    if !netgame
        && (*mthing).options as i32 & 16 as i32 != 0
    {
        return;
    }
    if gameskill as i32 == sk_baby as i32 {
        bit = 1 as i32;
    } else if gameskill as i32 == sk_nightmare as i32 {
        bit = 4 as i32;
    } else {
        bit = (1 as i32)
            << gameskill as i32 - 1 as i32;
    }
    if (*mthing).options as i32 & bit == 0 {
        return;
    }
    i = 0 as i32;
    while i < NUMMOBJTYPES as i32 {
        if (*mthing).type_0 as i32 == mobjinfo[i as usize].doomednum {
            break;
        }
        i += 1;
    }
    if i == NUMMOBJTYPES as i32 {
        I_Error(&format!(
            "P_SpawnMapThing: Unknown type {} at ({}, {})",
            (*mthing).type_0 as i32,
            (*mthing).x as i32,
            (*mthing).y as i32,
        ));
    }
    if deathmatch != 0
        && mobjinfo[i as usize].flags & MF_NOTDMATCH as i32 != 0
    {
        return;
    }
    if nomonsters
        && (i == MT_SKULL as i32
            || mobjinfo[i as usize].flags & MF_COUNTKILL as i32 != 0)
    {
        return;
    }
    x = (((*mthing).x as i32) << FRACBITS) as fixed_t;
    y = (((*mthing).y as i32) << FRACBITS) as fixed_t;
    if mobjinfo[i as usize].flags & MF_SPAWNCEILING as i32 != 0 {
        z = ONCEILINGZ as fixed_t;
    } else {
        z = ONFLOORZ as fixed_t;
    }
    mobj = P_SpawnMobj(x, y, z, i as mobjtype_t);
    (*mobj).spawnpoint = *mthing;
    if (*mobj).tics > 0 as i32 {
        (*mobj).tics = 1 as i32 + P_Random() % (*mobj).tics;
    }
    if (*mobj).flags & MF_COUNTKILL as i32 != 0 {
        totalkills += 1;
    }
    if (*mobj).flags & MF_COUNTITEM as i32 != 0 {
        totalitems += 1;
    }
    (*mobj).angle = (ANG45
        * ((*mthing).angle as i32 / 45 as i32)) as angle_t;
    if (*mthing).options as i32 & MTF_AMBUSH != 0 {
        (*mobj).flags |= MF_AMBUSH as i32;
    }
}
pub unsafe fn P_SpawnPuff(mut x: fixed_t, mut y: fixed_t, mut z: fixed_t) {
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    z += P_Random() - P_Random() << 10 as i32;
    th = P_SpawnMobj(x, y, z, MT_PUFF);
    (*th).momz = FRACUNIT as fixed_t;
    (*th).tics -= P_Random() & 3 as i32;
    if (*th).tics < 1 as i32 {
        (*th).tics = 1 as i32;
    }
    if attackrange == MELEERANGE {
        P_SetMobjState(th, S_PUFF3);
    }
}
pub unsafe fn P_SpawnBlood(
    mut x: fixed_t,
    mut y: fixed_t,
    mut z: fixed_t,
    mut damage: i32,
) {
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    z += P_Random() - P_Random() << 10 as i32;
    th = P_SpawnMobj(x, y, z, MT_BLOOD);
    (*th).momz = (FRACUNIT * 2 as i32) as fixed_t;
    (*th).tics -= P_Random() & 3 as i32;
    if (*th).tics < 1 as i32 {
        (*th).tics = 1 as i32;
    }
    if damage <= 12 as i32 && damage >= 9 as i32 {
        P_SetMobjState(th, S_BLOOD2);
    } else if damage < 9 as i32 {
        P_SetMobjState(th, S_BLOOD3);
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckMissileSpawn(mut th: *mut mobj_t) {
    (*th).tics -= P_Random() & 3 as i32;
    if (*th).tics < 1 as i32 {
        (*th).tics = 1 as i32;
    }
    (*th).x += (*th).momx >> 1 as i32;
    (*th).y += (*th).momy >> 1 as i32;
    (*th).z += (*th).momz >> 1 as i32;
    if !P_TryMove(th, (*th).x, (*th).y) {
        P_ExplodeMissile(th);
    }
}
pub unsafe fn P_SubstNullMobj(mut mobj: *mut mobj_t) -> *mut mobj_t {
    if mobj.is_null() {
        static mut dummy_mobj: mobj_t = mobj_s {
            thinker: thinker_s {
                prev: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
                next: ::core::ptr::null::<thinker_s>() as *mut thinker_s,
                function: ThinkerFn::Paused,
            },
            x: 0,
            y: 0,
            z: 0,
            snext: ::core::ptr::null::<mobj_s>() as *mut mobj_s,
            sprev: ::core::ptr::null::<mobj_s>() as *mut mobj_s,
            angle: 0,
            sprite: SPR_TROO,
            frame: 0,
            bnext: ::core::ptr::null::<mobj_s>() as *mut mobj_s,
            bprev: ::core::ptr::null::<mobj_s>() as *mut mobj_s,
            subsector: ::core::ptr::null::<subsector_s>() as *mut subsector_s,
            floorz: 0,
            ceilingz: 0,
            radius: 0,
            height: 0,
            momx: 0,
            momy: 0,
            momz: 0,
            validcount: 0,
            type_0: MT_PLAYER,
            info: ::core::ptr::null::<mobjinfo_t>() as *mut mobjinfo_t,
            tics: 0,
            state: ::core::ptr::null::<state_t>() as *mut state_t,
            flags: 0,
            health: 0,
            movedir: 0,
            movecount: 0,
            target: ::core::ptr::null::<mobj_s>() as *mut mobj_s,
            reactiontime: 0,
            threshold: 0,
            player: ::core::ptr::null::<player_s>() as *mut player_s,
            lastlook: 0,
            spawnpoint: mapthing_t {
                x: 0,
                y: 0,
                angle: 0,
                type_0: 0,
                options: 0,
            },
            tracer: ::core::ptr::null::<mobj_s>() as *mut mobj_s,
        };
        dummy_mobj.x = 0 as i32 as fixed_t;
        dummy_mobj.y = 0 as i32 as fixed_t;
        dummy_mobj.z = 0 as i32 as fixed_t;
        dummy_mobj.flags = 0 as i32;
        mobj = &raw mut dummy_mobj;
    }
    return mobj;
}
pub unsafe fn P_SpawnMissile(
    mut source: *mut mobj_t,
    mut dest: *mut mobj_t,
    mut type_0: mobjtype_t,
) -> *mut mobj_t {
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: angle_t = 0;
    let mut dist: i32 = 0;
    th = P_SpawnMobj(
        (*source).x,
        (*source).y,
        (*source).z + 4 as fixed_t * 8 as fixed_t * FRACUNIT,
        type_0,
    );
    if (*(*th).info).seesound != 0 {
        S_StartSound(th as *mut ::core::ffi::c_void, (*(*th).info).seesound);
    }
    (*th).target = source as *mut mobj_s;
    an = R_PointToAngle2((*source).x, (*source).y, (*dest).x, (*dest).y);
    if (*dest).flags & MF_SHADOW as i32 != 0 {
        an = an
            .wrapping_add(
                (P_Random() - P_Random() << 20 as i32) as angle_t,
            );
    }
    (*th).angle = an;
    an >>= ANGLETOFINESHIFT;
    (*th).momx = FixedMul(
        (*(*th).info).speed as fixed_t,
        finecosine[an as isize],
    );
    (*th).momy = FixedMul((*(*th).info).speed as fixed_t, finesine[an as usize]);
    dist = P_AproxDistance((*dest).x - (*source).x, (*dest).y - (*source).y)
        as i32;
    dist = dist / (*(*th).info).speed;
    if dist < 1 as i32 {
        dist = 1 as i32;
    }
    (*th).momz = (((*dest).z as i32 - (*source).z as i32)
        / dist) as fixed_t;
    P_CheckMissileSpawn(th);
    return th;
}
pub unsafe fn P_SpawnPlayerMissile(
    mut source: *mut mobj_t,
    mut type_0: mobjtype_t,
) {
    let mut th: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut an: angle_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut slope: fixed_t = 0;
    an = (*source).angle;
    slope = P_AimLineAttack(source, an, 16 as fixed_t * 64 as fixed_t * FRACUNIT);
    if linetarget.is_null() {
        an = an
            .wrapping_add(
                ((1 as i32) << 26 as i32) as angle_t,
            );
        slope = P_AimLineAttack(source, an, 16 as fixed_t * 64 as fixed_t * FRACUNIT);
        if linetarget.is_null() {
            an = an
                .wrapping_sub(
                    ((2 as i32) << 26 as i32) as angle_t,
                );
            slope = P_AimLineAttack(
                source,
                an,
                16 as fixed_t * 64 as fixed_t * FRACUNIT,
            );
        }
        if linetarget.is_null() {
            an = (*source).angle;
            slope = 0 as i32 as fixed_t;
        }
    }
    x = (*source).x;
    y = (*source).y;
    z = ((*source).z as i32
        + 4 as i32 * 8 as i32 * FRACUNIT) as fixed_t;
    th = P_SpawnMobj(x, y, z, type_0);
    if (*(*th).info).seesound != 0 {
        S_StartSound(th as *mut ::core::ffi::c_void, (*(*th).info).seesound);
    }
    (*th).target = source as *mut mobj_s;
    (*th).angle = an;
    (*th).momx = FixedMul(
        (*(*th).info).speed as fixed_t,
        finecosine[(an >> ANGLETOFINESHIFT) as isize],
    );
    (*th).momy = FixedMul(
        (*(*th).info).speed as fixed_t,
        finesine[(an >> ANGLETOFINESHIFT) as usize],
    );
    (*th).momz = FixedMul((*(*th).info).speed as fixed_t, slope);
    P_CheckMissileSpawn(th);
}
pub const __INT_MAX__: i32 = 2147483647 as i32;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
