use crate::src::d_items::weaponinfo;
use crate::src::p_mobj::state_t;
use crate::src::d_player::{player_t, PST_DEAD};
use crate::src::p_mobj::{mobj_s, mobj_t};
use crate::src::i_system::I_Error;
use crate::src::i_system::I_Tactile;
use crate::src::p_pspr::P_DropWeapon;
use crate::src::am_map::AM_Stop;
use crate::src::g_game::gameskill;
use crate::src::p_mobj::P_SetMobjState;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::p_mobj::P_SpawnMobj;
use crate::src::info::states;
use crate::src::am_map::automapactive;
use crate::src::r_main::R_PointToAngle2;
use crate::src::g_game::deathmatch;
use crate::src::m_random::P_Random;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::tables::finecosine;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::p_mobj::{MF_CORPSE, MF_COUNTITEM, MF_COUNTKILL, MF_DROPOFF, MF_DROPPED, MF_FLOAT, MF_JUSTHIT, MF_NOCLIP, MF_NOGRAVITY, MF_SHADOW, MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID};
use crate::src::sounds::{sfx_getpow, sfx_itemup, sfx_wpnup};
use crate::src::d_player::{pw_allmap, pw_infrared, pw_invisibility, pw_invulnerability, pw_ironfeet, pw_strength};
use crate::src::d_player::CF_GODMODE;
use crate::src::p_mobj::{MT_CHAINGUN, MT_CLIP, MT_PLAYER, MT_SHOTGUN, MT_SKULL, MT_VILE, mobjtype_t};
use crate::src::p_mobj::statenum_t;
use crate::src::d_mode::commercial;
use crate::src::d_mode::exe_chex;
use crate::src::d_mode::{sk_baby, sk_nightmare};
use crate::src::d_player::{weapontype_t, wp_bfg, wp_chaingun, wp_chainsaw, wp_fist, wp_missile, wp_pistol, wp_plasma, wp_shotgun, wp_supershotgun};
use crate::src::d_player::{NUMAMMO, am_cell, am_clip, am_misl, am_noammo, am_shell, ammotype_t};
use crate::src::m_fixed::fixed_t;
use crate::src::info::{S_NULL};
use crate::src::doomdef::NULL;
use crate::src::m_fixed::FRACUNIT;
use crate::src::tables::ANGLETOFINESHIFT;
use crate::src::tables::ANG180;
use crate::src::p_mobj::ONFLOORZ;


pub type card_t = u32;
pub const NUMCARDS: card_t = 6;
pub const it_redskull: card_t = 5;
pub const it_yellowskull: card_t = 4;
pub const it_blueskull: card_t = 3;
pub const it_redcard: card_t = 2;
pub const it_yellowcard: card_t = 1;
pub const it_bluecard: card_t = 0;
pub type C2RustUnnamed_0 = u32;
pub const IRONTICS: C2RustUnnamed_0 = 2100;
pub const INFRATICS: C2RustUnnamed_0 = 4200;
pub const INVISTICS: C2RustUnnamed_0 = 2100;
pub const INVULNTICS: C2RustUnnamed_0 = 1050;
pub const DEH_DEFAULT_MAX_HEALTH: i32 = 200 as i32;
pub const DEH_DEFAULT_MAX_ARMOR: i32 = 200 as i32;
pub const DEH_DEFAULT_GREEN_ARMOR_CLASS: i32 = 1 as i32;
pub const DEH_DEFAULT_BLUE_ARMOR_CLASS: i32 = 2 as i32;
pub const DEH_DEFAULT_MAX_SOULSPHERE: i32 = 200 as i32;
pub const DEH_DEFAULT_SOULSPHERE_HEALTH: i32 = 100 as i32;
pub const DEH_DEFAULT_MEGASPHERE_HEALTH: i32 = 200 as i32;
pub const deh_max_health: i32 = DEH_DEFAULT_MAX_HEALTH;
pub const deh_max_armor: i32 = DEH_DEFAULT_MAX_ARMOR;
pub const deh_green_armor_class: i32 = DEH_DEFAULT_GREEN_ARMOR_CLASS;
pub const deh_blue_armor_class: i32 = DEH_DEFAULT_BLUE_ARMOR_CLASS;
pub const deh_max_soulsphere: i32 = DEH_DEFAULT_MAX_SOULSPHERE;
pub const deh_soulsphere_health: i32 = DEH_DEFAULT_SOULSPHERE_HEALTH;
pub const deh_megasphere_health: i32 = DEH_DEFAULT_MEGASPHERE_HEALTH;
pub const FRACBITS: i32 = 16 as i32;
pub const MAXHEALTH: i32 = 100 as i32;
pub const BASETHRESHOLD: i32 = 100 as i32;
pub const BONUSADD: i32 = 6 as i32;
pub static mut maxammo: [i32; 4] = [
    200 as i32,
    50 as i32,
    300 as i32,
    50 as i32,
];
#[no_mangle]
pub static mut clipammo: [i32; 4] = [
    10 as i32,
    4 as i32,
    20 as i32,
    1 as i32,
];
#[no_mangle]
pub unsafe extern "C" fn P_GiveAmmo(
    mut player: *mut player_t,
    mut ammo: ammotype_t,
    mut num: i32,
) -> bool {
    let mut oldammo: i32 = 0;
    if ammo as u32
        == am_noammo as i32 as u32
    {
        return false;
    }
    if ammo as u32 > NUMAMMO as i32 as u32
    {
        I_Error(&format!("P_GiveAmmo: bad type {}", ammo as u32));
    }
    if (*player).ammo[ammo as usize] == (*player).maxammo[ammo as usize] {
        return false;
    }
    if num != 0 {
        num *= clipammo[ammo as usize];
    } else {
        num = clipammo[ammo as usize] / 2 as i32;
    }
    if gameskill as i32 == sk_baby as i32
        || gameskill as i32 == sk_nightmare as i32
    {
        num <<= 1 as i32;
    }
    oldammo = (*player).ammo[ammo as usize];
    (*player).ammo[ammo as usize] += num;
    if (*player).ammo[ammo as usize] > (*player).maxammo[ammo as usize] {
        (*player).ammo[ammo as usize] = (*player).maxammo[ammo as usize];
    }
    if oldammo != 0 {
        return true;
    }
    match ammo as u32 {
        0 => {
            if (*player).readyweapon as u32
                == wp_fist as i32 as u32
            {
                if (*player).weaponowned[wp_chaingun as i32 as usize]
                {
                    (*player).pendingweapon = wp_chaingun;
                } else {
                    (*player).pendingweapon = wp_pistol;
                }
            }
        }
        1 => {
            if (*player).readyweapon as u32
                == wp_fist as i32 as u32
                || (*player).readyweapon as u32
                    == wp_pistol as i32 as u32
            {
                if (*player).weaponowned[wp_shotgun as i32 as usize]
                {
                    (*player).pendingweapon = wp_shotgun;
                }
            }
        }
        2 => {
            if (*player).readyweapon as u32
                == wp_fist as i32 as u32
                || (*player).readyweapon as u32
                    == wp_pistol as i32 as u32
            {
                if (*player).weaponowned[wp_plasma as i32 as usize] {
                    (*player).pendingweapon = wp_plasma;
                }
            }
        }
        3 => {
            if (*player).readyweapon as u32
                == wp_fist as i32 as u32
            {
                if (*player).weaponowned[wp_missile as i32 as usize]
                {
                    (*player).pendingweapon = wp_missile;
                }
            }
        }
        _ => {}
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn P_GiveWeapon(
    mut player: *mut player_t,
    mut weapon: weapontype_t,
    mut dropped: bool,
) -> bool {
    let mut gaveammo: bool = false;
    let mut gaveweapon: bool;
    if netgame && deathmatch != 2 as i32 && !dropped {
        if (*player).weaponowned[weapon as usize] {
            return false;
        }
        (*player).bonuscount += BONUSADD;
        (*player).weaponowned[weapon as usize] = true;
        if deathmatch != 0 {
            P_GiveAmmo(
                player,
                weaponinfo[weapon as usize].ammo,
                5 as i32,
            );
        } else {
            P_GiveAmmo(
                player,
                weaponinfo[weapon as usize].ammo,
                2 as i32,
            );
        }
        (*player).pendingweapon = weapon;
        if player
            == (&raw mut players as *mut player_t).offset(consoleplayer as isize)
                as *mut player_t
        {
            S_StartSound(NULL, sfx_wpnup as i32);
        }
        return false;
    }
    if weaponinfo[weapon as usize].ammo as u32
        != am_noammo as i32 as u32
    {
        if dropped {
            gaveammo = P_GiveAmmo(
                player,
                weaponinfo[weapon as usize].ammo,
                1 as i32,
            );
        } else {
            gaveammo = P_GiveAmmo(
                player,
                weaponinfo[weapon as usize].ammo,
                2 as i32,
            );
        }
    } else {
        gaveammo = false;
    }
    if (*player).weaponowned[weapon as usize] {
        gaveweapon = false;
    } else {
        gaveweapon = true;
        (*player).weaponowned[weapon as usize] = true;
        (*player).pendingweapon = weapon;
    }
    return gaveweapon || gaveammo;
}
#[no_mangle]
pub unsafe extern "C" fn P_GiveBody(
    mut player: *mut player_t,
    mut num: i32,
) -> bool {
    if (*player).health >= MAXHEALTH {
        return false;
    }
    (*player).health += num;
    if (*player).health > MAXHEALTH {
        (*player).health = MAXHEALTH;
    }
    (*(*player).mo).health = (*player).health;
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn P_GiveArmor(
    mut player: *mut player_t,
    mut armortype: i32,
) -> bool {
    let mut hits: i32 = 0;
    hits = armortype * 100 as i32;
    if (*player).armorpoints >= hits {
        return false;
    }
    (*player).armortype = armortype;
    (*player).armorpoints = hits;
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn P_GiveCard(mut player: *mut player_t, mut card: card_t) {
    if (*player).cards[card as usize] {
        return;
    }
    (*player).bonuscount = BONUSADD;
    (*player).cards[card as usize] = true;
}
pub unsafe fn P_GivePower(
    mut player: *mut player_t,
    mut power: i32,
) -> bool {
    if power == pw_invulnerability as i32 {
        (*player).powers[power as usize] = INVULNTICS as i32;
        return true;
    }
    if power == pw_invisibility as i32 {
        (*player).powers[power as usize] = INVISTICS as i32;
        (*(*player).mo).flags |= MF_SHADOW as i32;
        return true;
    }
    if power == pw_infrared as i32 {
        (*player).powers[power as usize] = INFRATICS as i32;
        return true;
    }
    if power == pw_ironfeet as i32 {
        (*player).powers[power as usize] = IRONTICS as i32;
        return true;
    }
    if power == pw_strength as i32 {
        P_GiveBody(player, 100 as i32);
        (*player).powers[power as usize] = 1 as i32;
        return true;
    }
    if (*player).powers[power as usize] != 0 {
        return false;
    }
    (*player).powers[power as usize] = 1 as i32;
    return true;
}
pub unsafe fn P_TouchSpecialThing(
    mut special: *mut mobj_t,
    mut toucher: *mut mobj_t,
) {
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut i: i32 = 0;
    let mut delta: fixed_t = 0;
    let mut sound: i32 = 0;
    delta = (*special).z - (*toucher).z;
    if delta > (*toucher).height || delta < -(8 as i32) * FRACUNIT {
        return;
    }
    sound = sfx_itemup as i32;
    player = (*toucher).player as *mut player_t;
    if (*toucher).health <= 0 as i32 {
        return;
    }
    match (*special).sprite as u32 {
        55 => {
            if !P_GiveArmor(player, deh_green_armor_class) {
                return;
            }
            (*player).message = b"Picked up the armor.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        56 => {
            if !P_GiveArmor(player, deh_blue_armor_class) {
                return;
            }
            (*player).message = b"Picked up the MegaArmor!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        60 => {
            (*player).health += 1;
            if (*player).health > deh_max_health {
                (*player).health = deh_max_health;
            }
            (*(*player).mo).health = (*player).health;
            (*player).message = b"Picked up a health bonus.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        61 => {
            (*player).armorpoints += 1;
            if (*player).armorpoints > deh_max_armor {
                (*player).armorpoints = deh_max_armor;
            }
            if (*player).armortype == 0 {
                (*player).armortype = 1 as i32;
            }
            (*player).message = b"Picked up an armor bonus.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        70 => {
            (*player).health += deh_soulsphere_health;
            if (*player).health > deh_max_soulsphere {
                (*player).health = deh_max_soulsphere;
            }
            (*(*player).mo).health = (*player).health;
            (*player).message = b"Supercharge!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        74 => {
            if gamemode as u32
                != commercial as i32 as u32
            {
                return;
            }
            (*player).health = deh_megasphere_health;
            (*(*player).mo).health = (*player).health;
            P_GiveArmor(player, 2 as i32);
            (*player).message = b"MegaSphere!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        62 => {
            if !(*player).cards[it_bluecard as i32 as usize] {
                (*player).message = b"Picked up a blue keycard.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            P_GiveCard(player, it_bluecard);
            if netgame {
                return;
            }
        }
        64 => {
            if !(*player).cards[it_yellowcard as i32 as usize] {
                (*player).message = b"Picked up a yellow keycard.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            P_GiveCard(player, it_yellowcard);
            if netgame {
                return;
            }
        }
        63 => {
            if !(*player).cards[it_redcard as i32 as usize] {
                (*player).message = b"Picked up a red keycard.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            P_GiveCard(player, it_redcard);
            if netgame {
                return;
            }
        }
        65 => {
            if !(*player).cards[it_blueskull as i32 as usize] {
                (*player).message = b"Picked up a blue skull key.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            P_GiveCard(player, it_blueskull);
            if netgame {
                return;
            }
        }
        67 => {
            if !(*player).cards[it_yellowskull as i32 as usize] {
                (*player).message = b"Picked up a yellow skull key.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            P_GiveCard(player, it_yellowskull);
            if netgame {
                return;
            }
        }
        66 => {
            if !(*player).cards[it_redskull as i32 as usize] {
                (*player).message = b"Picked up a red skull key.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            P_GiveCard(player, it_redskull);
            if netgame {
                return;
            }
        }
        68 => {
            if !P_GiveBody(player, 10 as i32) {
                return;
            }
            (*player).message = b"Picked up a stimpack.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        69 => {
            if !P_GiveBody(player, 25 as i32) {
                return;
            }
            if (*player).health < 25 as i32 {
                (*player).message = b"Picked up a medikit that you REALLY need!\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                (*player).message = b"Picked up a medikit.\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
        }
        71 => {
            if !P_GivePower(player, pw_invulnerability as i32) {
                return;
            }
            (*player).message = b"Invulnerability!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        72 => {
            if !P_GivePower(player, pw_strength as i32) {
                return;
            }
            (*player).message = b"Berserk!\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            if (*player).readyweapon as u32
                != wp_fist as i32 as u32
            {
                (*player).pendingweapon = wp_fist;
            }
            sound = sfx_getpow as i32;
        }
        73 => {
            if !P_GivePower(player, pw_invisibility as i32) {
                return;
            }
            (*player).message = b"Partial Invisibility\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        75 => {
            if !P_GivePower(player, pw_ironfeet as i32) {
                return;
            }
            (*player).message = b"Radiation Shielding Suit\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        76 => {
            if !P_GivePower(player, pw_allmap as i32) {
                return;
            }
            (*player).message = b"Computer Area Map\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        77 => {
            if !P_GivePower(player, pw_infrared as i32) {
                return;
            }
            (*player).message = b"Light Amplification Visor\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_getpow as i32;
        }
        78 => {
            if (*special).flags & MF_DROPPED as i32 != 0 {
                if !P_GiveAmmo(player, am_clip, 0 as i32) {
                    return;
                }
            } else if !P_GiveAmmo(player, am_clip, 1 as i32) {
                return
            }
            (*player).message = b"Picked up a clip.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        79 => {
            if !P_GiveAmmo(player, am_clip, 5 as i32) {
                return;
            }
            (*player).message = b"Picked up a box of bullets.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        80 => {
            if !P_GiveAmmo(player, am_misl, 1 as i32) {
                return;
            }
            (*player).message = b"Picked up a rocket.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        81 => {
            if !P_GiveAmmo(player, am_misl, 5 as i32) {
                return;
            }
            (*player).message = b"Picked up a box of rockets.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        82 => {
            if !P_GiveAmmo(player, am_cell, 1 as i32) {
                return;
            }
            (*player).message = b"Picked up an energy cell.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        83 => {
            if !P_GiveAmmo(player, am_cell, 5 as i32) {
                return;
            }
            (*player).message = b"Picked up an energy cell pack.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        84 => {
            if !P_GiveAmmo(player, am_shell, 1 as i32) {
                return;
            }
            (*player).message = b"Picked up 4 shotgun shells.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        85 => {
            if !P_GiveAmmo(player, am_shell, 5 as i32) {
                return;
            }
            (*player).message = b"Picked up a box of shotgun shells.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        86 => {
            if !(*player).backpack {
                i = 0 as i32;
                while i < NUMAMMO as i32 {
                    (*player).maxammo[i as usize] *= 2 as i32;
                    i += 1;
                }
                (*player).backpack = true;
            }
            i = 0 as i32;
            while i < NUMAMMO as i32 {
                P_GiveAmmo(player, i as ammotype_t, 1 as i32);
                i += 1;
            }
            (*player).message = b"Picked up a backpack full of ammo!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        87 => {
            if !P_GiveWeapon(player, wp_bfg, false) {
                return;
            }
            (*player).message = b"You got the BFG9000!  Oh, yes.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        88 => {
            if !P_GiveWeapon(
                player,
                wp_chaingun,
                (*special).flags & MF_DROPPED as i32 != 0,
            )
            {
                return;
            }
            (*player).message = b"You got the chaingun!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        89 => {
            if !P_GiveWeapon(player, wp_chainsaw, false) {
                return;
            }
            (*player).message = b"A chainsaw!  Find some meat!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        90 => {
            if !P_GiveWeapon(player, wp_missile, false) {
                return;
            }
            (*player).message = b"You got the rocket launcher!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        91 => {
            if !P_GiveWeapon(player, wp_plasma, false) {
                return;
            }
            (*player).message = b"You got the plasma gun!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        92 => {
            if !P_GiveWeapon(
                player,
                wp_shotgun,
                (*special).flags & MF_DROPPED as i32 != 0,
            )
            {
                return;
            }
            (*player).message = b"You got the shotgun!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        93 => {
            if !P_GiveWeapon(
                player,
                wp_supershotgun,
                (*special).flags & MF_DROPPED as i32 != 0,
            )
            {
                return;
            }
            (*player).message = b"You got the super shotgun!\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            sound = sfx_wpnup as i32;
        }
        _ => {
            I_Error("P_SpecialThing: Unknown gettable thing");
        }
    }
    if (*special).flags & MF_COUNTITEM as i32 != 0 {
        (*player).itemcount += 1;
    }
    P_RemoveMobj(special);
    (*player).bonuscount += BONUSADD;
    if player
        == (&raw mut players as *mut player_t).offset(consoleplayer as isize)
            as *mut player_t
    {
        S_StartSound(NULL, sound);
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_KillMobj(mut source: *mut mobj_t, mut target: *mut mobj_t) {
    let mut item: mobjtype_t = MT_PLAYER;
    let mut mo: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    (*target).flags
        &= !(MF_SHOOTABLE as i32 | MF_FLOAT as i32
            | MF_SKULLFLY as i32);
    if (*target).type_0 as u32
        != MT_SKULL as i32 as u32
    {
        (*target).flags &= !(MF_NOGRAVITY as i32);
    }
    (*target).flags
        |= MF_CORPSE as i32 | MF_DROPOFF as i32;
    (*target).height >>= 2 as i32;
    if !source.is_null() && !(*source).player.is_null() {
        if (*target).flags & MF_COUNTKILL as i32 != 0 {
            (*(*source).player).killcount += 1;
        }
        if !(*target).player.is_null() {
            (*(*source).player)
                .frags[(*target).player.offset_from(&raw mut players as *mut player_t)
                as i64 as usize] += 1;
        }
    } else if !netgame && (*target).flags & MF_COUNTKILL as i32 != 0 {
        players[0 as i32 as usize].killcount += 1;
    }
    if !(*target).player.is_null() {
        if source.is_null() {
            (*(*target).player)
                .frags[(*target).player.offset_from(&raw mut players as *mut player_t)
                as i64 as usize] += 1;
        }
        (*target).flags &= !(MF_SOLID as i32);
        (*(*target).player).playerstate = PST_DEAD;
        P_DropWeapon((*target).player as *mut player_t);
        if (*target).player
            == (&raw mut players as *mut player_t).offset(consoleplayer as isize)
                as *mut player_t && automapactive
        {
            AM_Stop();
        }
    }
    if (*target).health < -(*(*target).info).spawnhealth
        && (*(*target).info).xdeathstate != 0
    {
        P_SetMobjState(target, (*(*target).info).xdeathstate as statenum_t);
    } else {
        P_SetMobjState(target, (*(*target).info).deathstate as statenum_t);
    }
    (*target).tics -= P_Random() & 3 as i32;
    if (*target).tics < 1 as i32 {
        (*target).tics = 1 as i32;
    }
    if gameversion as u32
        == exe_chex as i32 as u32
    {
        return;
    }
    match (*target).type_0 as u32 {
        23 | 1 => {
            item = MT_CLIP;
        }
        2 => {
            item = MT_SHOTGUN;
        }
        10 => {
            item = MT_CHAINGUN;
        }
        _ => return,
    }
    mo = P_SpawnMobj((*target).x, (*target).y, ONFLOORZ, item);
    (*mo).flags |= MF_DROPPED as i32;
}
pub unsafe fn P_DamageMobj(
    mut target: *mut mobj_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: i32,
) {
    let mut ang: u32 = 0;
    let mut saved: i32 = 0;
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    let mut thrust: fixed_t = 0;
    let mut temp: i32 = 0;
    if (*target).flags & MF_SHOOTABLE as i32 == 0 {
        return;
    }
    if (*target).health <= 0 as i32 {
        return;
    }
    if (*target).flags & MF_SKULLFLY as i32 != 0 {
        (*target).momz = 0 as i32 as fixed_t;
        (*target).momy = (*target).momz;
        (*target).momx = (*target).momy;
    }
    player = (*target).player as *mut player_t;
    if !player.is_null()
        && gameskill as i32 == sk_baby as i32
    {
        damage >>= 1 as i32;
    }
    if !inflictor.is_null() && (*target).flags & MF_NOCLIP as i32 == 0
        && (source.is_null() || (*source).player.is_null()
            || (*(*source).player).readyweapon as u32
                != wp_chainsaw as i32 as u32)
    {
        ang = R_PointToAngle2((*inflictor).x, (*inflictor).y, (*target).x, (*target).y)
            as u32;
        thrust = (damage * (FRACUNIT >> 3 as i32)
            * 100 as i32 / (*(*target).info).mass) as fixed_t;
        if damage < 40 as i32 && damage > (*target).health
            && (*target).z - (*inflictor).z > 64 as i32 * FRACUNIT
            && P_Random() & 1 as i32 != 0
        {
            ang = ang.wrapping_add(ANG180);
            thrust *= 4 as i32;
        }
        ang >>= ANGLETOFINESHIFT;
        (*target).momx += FixedMul(thrust, finecosine[ang as isize]);
        (*target).momy += FixedMul(thrust, finesine[ang as usize]);
    }
    if !player.is_null() {
        if (*(*(*target).subsector).sector).special as i32
            == 11 as i32 && damage >= (*target).health
        {
            damage = (*target).health - 1 as i32;
        }
        if damage < 1000 as i32
            && ((*player).cheats & CF_GODMODE as i32 != 0
                || (*player).powers[pw_invulnerability as i32 as usize]
                    != 0)
        {
            return;
        }
        if (*player).armortype != 0 {
            if (*player).armortype == 1 as i32 {
                saved = damage / 3 as i32;
            } else {
                saved = damage / 2 as i32;
            }
            if (*player).armorpoints <= saved {
                saved = (*player).armorpoints;
                (*player).armortype = 0 as i32;
            }
            (*player).armorpoints -= saved;
            damage -= saved;
        }
        (*player).health -= damage;
        if (*player).health < 0 as i32 {
            (*player).health = 0 as i32;
        }
        (*player).attacker = source;
        (*player).damagecount += damage;
        if (*player).damagecount > 100 as i32 {
            (*player).damagecount = 100 as i32;
        }
        temp = if damage < 100 as i32 {
            damage
        } else {
            100 as i32
        };
        if player
            == (&raw mut players as *mut player_t).offset(consoleplayer as isize)
                as *mut player_t
        {
            I_Tactile(
                40 as i32,
                10 as i32,
                40 as i32 + temp * 2 as i32,
            );
        }
    }
    (*target).health -= damage;
    if (*target).health <= 0 as i32 {
        P_KillMobj(source, target);
        return;
    }
    if P_Random() < (*(*target).info).painchance
        && (*target).flags & MF_SKULLFLY as i32 == 0
    {
        (*target).flags |= MF_JUSTHIT as i32;
        P_SetMobjState(target, (*(*target).info).painstate as statenum_t);
    }
    (*target).reactiontime = 0 as i32;
    if ((*target).threshold == 0
        || (*target).type_0 as u32
            == MT_VILE as i32 as u32) && !source.is_null()
        && source != target
        && (*source).type_0 as u32
            != MT_VILE as i32 as u32
    {
        (*target).target = source as *mut mobj_s;
        (*target).threshold = BASETHRESHOLD;
        if (*target).state
            == (&raw mut states as *mut state_t)
                .offset((*(*target).info).spawnstate as isize) as *mut state_t
            && (*(*target).info).seestate != S_NULL as i32
        {
            P_SetMobjState(target, (*(*target).info).seestate as statenum_t);
        }
    }
}
