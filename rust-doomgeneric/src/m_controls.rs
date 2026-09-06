use crate::src::m_config::M_BindVariable;
use crate::src::m_misc::M_snprintf;
use crate::src::stdint_types::size_t;

pub const KEY_RIGHTARROW: i32 = 0xae;
pub const KEY_LEFTARROW: i32 = 0xac;
pub const KEY_UPARROW: i32 = 0xad;
pub const KEY_DOWNARROW: i32 = 0xaf;
pub const KEY_STRAFE_L: i32 = 0xa0;
pub const KEY_STRAFE_R: i32 = 0xa1;
pub const KEY_USE: i32 = 0xa2;
pub const KEY_FIRE: i32 = 0xa3;
pub const KEY_ESCAPE: i32 = 27;
pub const KEY_ENTER: i32 = 13;
pub const KEY_TAB: i32 = 9;
pub const KEY_F1: i32 = 0x80
    + 0x3b as i32;
pub const KEY_F2: i32 = 0x80
    + 0x3c as i32;
pub const KEY_F3: i32 = 0x80
    + 0x3d as i32;
pub const KEY_F4: i32 = 0x80
    + 0x3e as i32;
pub const KEY_F5: i32 = 0x80
    + 0x3f as i32;
pub const KEY_F6: i32 = 0x80
    + 0x40 as i32;
pub const KEY_F7: i32 = 0x80
    + 0x41 as i32;
pub const KEY_F8: i32 = 0x80
    + 0x42 as i32;
pub const KEY_F9: i32 = 0x80
    + 0x43 as i32;
pub const KEY_F10: i32 = 0x80
    + 0x44 as i32;
pub const KEY_F11: i32 = 0x80
    + 0x57 as i32;
pub const KEY_F12: i32 = 0x80
    + 0x58 as i32;
pub const KEY_BACKSPACE: i32 = 0x7f;
pub const KEY_PAUSE: i32 = 0xff;
pub const KEY_EQUALS: i32 = 0x3d;
pub const KEY_MINUS: i32 = 0x2d;
pub const KEY_RSHIFT: i32 = 0x80
    + 0x36 as i32;
pub const KEY_RALT: i32 = 0x80
    + 0x38 as i32;
pub const KEY_HOME: i32 = 0x80
    + 0x47 as i32;
pub const KEY_END: i32 = 0x80
    + 0x4f as i32;
pub const KEY_PGUP: i32 = 0x80
    + 0x49 as i32;
pub const KEY_PGDN: i32 = 0x80
    + 0x51 as i32;
pub const KEY_INS: i32 = 0x80
    + 0x52 as i32;
pub const KEY_DEL: i32 = 0x80
    + 0x53 as i32;
pub static mut key_right: i32 = KEY_RIGHTARROW;
pub static mut key_left: i32 = KEY_LEFTARROW;
pub static mut key_up: i32 = KEY_UPARROW;
pub static mut key_down: i32 = KEY_DOWNARROW;
pub static mut key_strafeleft: i32 = KEY_STRAFE_L;
pub static mut key_straferight: i32 = KEY_STRAFE_R;
pub static mut key_fire: i32 = KEY_FIRE;
pub static mut key_use: i32 = KEY_USE;
pub static mut key_strafe: i32 = KEY_RALT;
pub static mut key_speed: i32 = KEY_RSHIFT;
#[no_mangle]
pub static mut key_flyup: i32 = KEY_PGUP;
#[no_mangle]
pub static mut key_flydown: i32 = KEY_INS;
#[no_mangle]
pub static mut key_flycenter: i32 = KEY_HOME;
#[no_mangle]
pub static mut key_lookup: i32 = KEY_PGDN;
#[no_mangle]
pub static mut key_lookdown: i32 = KEY_DEL;
#[no_mangle]
pub static mut key_lookcenter: i32 = KEY_END;
#[no_mangle]
pub static mut key_invleft: i32 = '[' as i32;
#[no_mangle]
pub static mut key_invright: i32 = ']' as i32;
#[no_mangle]
pub static mut key_useartifact: i32 = KEY_ENTER;
#[no_mangle]
pub static mut key_jump: i32 = '/' as i32;
#[no_mangle]
pub static mut key_arti_all: i32 = KEY_BACKSPACE;
#[no_mangle]
pub static mut key_arti_health: i32 = '\\' as i32;
#[no_mangle]
pub static mut key_arti_poisonbag: i32 = '0' as i32;
#[no_mangle]
pub static mut key_arti_blastradius: i32 = '9' as i32;
#[no_mangle]
pub static mut key_arti_teleport: i32 = '8' as i32;
#[no_mangle]
pub static mut key_arti_teleportother: i32 = '7' as i32;
#[no_mangle]
pub static mut key_arti_egg: i32 = '6' as i32;
#[no_mangle]
pub static mut key_arti_invulnerability: i32 = '5' as i32;
#[no_mangle]
pub static mut key_usehealth: i32 = 'h' as i32;
#[no_mangle]
pub static mut key_invquery: i32 = 'q' as i32;
#[no_mangle]
pub static mut key_mission: i32 = 'w' as i32;
#[no_mangle]
pub static mut key_invpop: i32 = 'z' as i32;
#[no_mangle]
pub static mut key_invkey: i32 = 'k' as i32;
#[no_mangle]
pub static mut key_invhome: i32 = KEY_HOME;
#[no_mangle]
pub static mut key_invend: i32 = KEY_END;
#[no_mangle]
pub static mut key_invuse: i32 = KEY_ENTER;
#[no_mangle]
pub static mut key_invdrop: i32 = KEY_BACKSPACE;
pub static mut mousebfire: i32 = 0 as i32;
pub static mut mousebstrafe: i32 = 1 as i32;
pub static mut mousebforward: i32 = 2 as i32;
#[no_mangle]
pub static mut mousebjump: i32 = -(1 as i32);
pub static mut mousebstrafeleft: i32 = -(1 as i32);
pub static mut mousebstraferight: i32 = -(1 as i32);
pub static mut mousebbackward: i32 = -(1 as i32);
pub static mut mousebuse: i32 = -(1 as i32);
pub static mut mousebprevweapon: i32 = -(1 as i32);
pub static mut mousebnextweapon: i32 = -(1 as i32);
pub static mut key_message_refresh: i32 = KEY_ENTER;
pub static mut key_pause: i32 = KEY_PAUSE;
pub static mut key_demo_quit: i32 = 'q' as i32;
pub static mut key_spy: i32 = KEY_F12;
pub static mut key_multi_msg: i32 = 't' as i32;
pub static mut key_multi_msgplayer: [i32; 8] = [0; 8];
pub static mut key_weapon1: i32 = '1' as i32;
pub static mut key_weapon2: i32 = '2' as i32;
pub static mut key_weapon3: i32 = '3' as i32;
pub static mut key_weapon4: i32 = '4' as i32;
pub static mut key_weapon5: i32 = '5' as i32;
pub static mut key_weapon6: i32 = '6' as i32;
pub static mut key_weapon7: i32 = '7' as i32;
pub static mut key_weapon8: i32 = '8' as i32;
pub static mut key_prevweapon: i32 = 0 as i32;
pub static mut key_nextweapon: i32 = 0 as i32;
pub static mut key_map_north: i32 = KEY_UPARROW;
pub static mut key_map_south: i32 = KEY_DOWNARROW;
pub static mut key_map_east: i32 = KEY_RIGHTARROW;
pub static mut key_map_west: i32 = KEY_LEFTARROW;
pub static mut key_map_zoomin: i32 = '=' as i32;
pub static mut key_map_zoomout: i32 = '-' as i32;
pub static mut key_map_toggle: i32 = KEY_TAB;
pub static mut key_map_maxzoom: i32 = '0' as i32;
pub static mut key_map_follow: i32 = 'f' as i32;
pub static mut key_map_grid: i32 = 'g' as i32;
pub static mut key_map_mark: i32 = 'm' as i32;
pub static mut key_map_clearmark: i32 = 'c' as i32;
pub static mut key_menu_activate: i32 = KEY_ESCAPE;
pub static mut key_menu_up: i32 = KEY_UPARROW;
pub static mut key_menu_down: i32 = KEY_DOWNARROW;
pub static mut key_menu_left: i32 = KEY_LEFTARROW;
pub static mut key_menu_right: i32 = KEY_RIGHTARROW;
pub static mut key_menu_back: i32 = KEY_BACKSPACE;
pub static mut key_menu_forward: i32 = KEY_ENTER;
pub static mut key_menu_confirm: i32 = 'y' as i32;
pub static mut key_menu_abort: i32 = 'n' as i32;
pub static mut key_menu_help: i32 = KEY_F1;
pub static mut key_menu_save: i32 = KEY_F2;
pub static mut key_menu_load: i32 = KEY_F3;
pub static mut key_menu_volume: i32 = KEY_F4;
pub static mut key_menu_detail: i32 = KEY_F5;
pub static mut key_menu_qsave: i32 = KEY_F6;
pub static mut key_menu_endgame: i32 = KEY_F7;
pub static mut key_menu_messages: i32 = KEY_F8;
pub static mut key_menu_qload: i32 = KEY_F9;
pub static mut key_menu_quit: i32 = KEY_F10;
pub static mut key_menu_gamma: i32 = KEY_F11;
pub static mut key_menu_incscreen: i32 = KEY_EQUALS;
pub static mut key_menu_decscreen: i32 = KEY_MINUS;
pub static mut key_menu_screenshot: i32 = 0 as i32;
pub static mut joybfire: i32 = 0 as i32;
pub static mut joybstrafe: i32 = 1 as i32;
pub static mut joybuse: i32 = 3 as i32;
pub static mut joybspeed: i32 = 2 as i32;
pub static mut joybstrafeleft: i32 = -(1 as i32);
pub static mut joybstraferight: i32 = -(1 as i32);
#[no_mangle]
pub static mut joybjump: i32 = -(1 as i32);
pub static mut joybprevweapon: i32 = -(1 as i32);
pub static mut joybnextweapon: i32 = -(1 as i32);
pub static mut joybmenu: i32 = -(1 as i32);
pub static mut dclick_use: i32 = 1 as i32;
pub unsafe fn M_BindBaseControls() {
    M_BindVariable("key_right",
        &raw mut key_right as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_left",
        &raw mut key_left as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_up",
        &raw mut key_up as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_down",
        &raw mut key_down as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_strafeleft",
        &raw mut key_strafeleft as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_straferight",
        &raw mut key_straferight as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_fire",
        &raw mut key_fire as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_use",
        &raw mut key_use as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_strafe",
        &raw mut key_strafe as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_speed",
        &raw mut key_speed as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_fire",
        &raw mut mousebfire as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_strafe",
        &raw mut mousebstrafe as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_forward",
        &raw mut mousebforward as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_fire",
        &raw mut joybfire as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_strafe",
        &raw mut joybstrafe as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_use",
        &raw mut joybuse as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_speed",
        &raw mut joybspeed as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_menu_activate",
        &raw mut joybmenu as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_strafeleft",
        &raw mut joybstrafeleft as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_straferight",
        &raw mut joybstraferight as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_strafeleft",
        &raw mut mousebstrafeleft as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_straferight",
        &raw mut mousebstraferight as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_use",
        &raw mut mousebuse as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_backward",
        &raw mut mousebbackward as *mut ::core::ffi::c_void,
    );
    M_BindVariable("dclick_use",
        &raw mut dclick_use as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_pause",
        &raw mut key_pause as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_message_refresh",
        &raw mut key_message_refresh as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindHereticControls() {
    M_BindVariable("key_flyup",
        &raw mut key_flyup as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_flydown",
        &raw mut key_flydown as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_flycenter",
        &raw mut key_flycenter as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_lookup",
        &raw mut key_lookup as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_lookdown",
        &raw mut key_lookdown as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_lookcenter",
        &raw mut key_lookcenter as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invleft",
        &raw mut key_invleft as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invright",
        &raw mut key_invright as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_useartifact",
        &raw mut key_useartifact as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindHexenControls() {
    M_BindVariable("key_jump",
        &raw mut key_jump as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_jump",
        &raw mut mousebjump as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_jump",
        &raw mut joybjump as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_all",
        &raw mut key_arti_all as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_health",
        &raw mut key_arti_health as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_poisonbag",
        &raw mut key_arti_poisonbag as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_blastradius",
        &raw mut key_arti_blastradius as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_teleport",
        &raw mut key_arti_teleport as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_teleportother",
        &raw mut key_arti_teleportother as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_egg",
        &raw mut key_arti_egg as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_arti_invulnerability",
        &raw mut key_arti_invulnerability as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindStrifeControls() {
    key_message_refresh = '/' as i32;
    key_jump = 'a' as i32;
    key_lookup = KEY_PGUP;
    key_lookdown = KEY_PGDN;
    key_invleft = KEY_INS;
    key_invright = KEY_DEL;
    M_BindVariable("key_jump",
        &raw mut key_jump as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_lookUp",
        &raw mut key_lookup as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_lookDown",
        &raw mut key_lookdown as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invLeft",
        &raw mut key_invleft as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invRight",
        &raw mut key_invright as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_useHealth",
        &raw mut key_usehealth as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invquery",
        &raw mut key_invquery as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_mission",
        &raw mut key_mission as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invPop",
        &raw mut key_invpop as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invKey",
        &raw mut key_invkey as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invHome",
        &raw mut key_invhome as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invEnd",
        &raw mut key_invend as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invUse",
        &raw mut key_invuse as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_invDrop",
        &raw mut key_invdrop as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_jump",
        &raw mut mousebjump as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_jump",
        &raw mut joybjump as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindWeaponControls() {
    M_BindVariable("key_weapon1",
        &raw mut key_weapon1 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon2",
        &raw mut key_weapon2 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon3",
        &raw mut key_weapon3 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon4",
        &raw mut key_weapon4 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon5",
        &raw mut key_weapon5 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon6",
        &raw mut key_weapon6 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon7",
        &raw mut key_weapon7 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_weapon8",
        &raw mut key_weapon8 as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_prevweapon",
        &raw mut key_prevweapon as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_nextweapon",
        &raw mut key_nextweapon as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_prevweapon",
        &raw mut joybprevweapon as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joyb_nextweapon",
        &raw mut joybnextweapon as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_prevweapon",
        &raw mut mousebprevweapon as *mut ::core::ffi::c_void,
    );
    M_BindVariable("mouseb_nextweapon",
        &raw mut mousebnextweapon as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindMapControls() {
    M_BindVariable("key_map_north",
        &raw mut key_map_north as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_south",
        &raw mut key_map_south as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_east",
        &raw mut key_map_east as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_west",
        &raw mut key_map_west as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_zoomin",
        &raw mut key_map_zoomin as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_zoomout",
        &raw mut key_map_zoomout as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_toggle",
        &raw mut key_map_toggle as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_maxzoom",
        &raw mut key_map_maxzoom as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_follow",
        &raw mut key_map_follow as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_grid",
        &raw mut key_map_grid as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_mark",
        &raw mut key_map_mark as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_map_clearmark",
        &raw mut key_map_clearmark as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindMenuControls() {
    M_BindVariable("key_menu_activate",
        &raw mut key_menu_activate as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_up",
        &raw mut key_menu_up as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_down",
        &raw mut key_menu_down as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_left",
        &raw mut key_menu_left as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_right",
        &raw mut key_menu_right as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_back",
        &raw mut key_menu_back as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_forward",
        &raw mut key_menu_forward as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_confirm",
        &raw mut key_menu_confirm as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_abort",
        &raw mut key_menu_abort as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_help",
        &raw mut key_menu_help as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_save",
        &raw mut key_menu_save as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_load",
        &raw mut key_menu_load as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_volume",
        &raw mut key_menu_volume as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_detail",
        &raw mut key_menu_detail as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_qsave",
        &raw mut key_menu_qsave as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_endgame",
        &raw mut key_menu_endgame as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_messages",
        &raw mut key_menu_messages as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_qload",
        &raw mut key_menu_qload as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_quit",
        &raw mut key_menu_quit as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_gamma",
        &raw mut key_menu_gamma as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_incscreen",
        &raw mut key_menu_incscreen as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_decscreen",
        &raw mut key_menu_decscreen as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_menu_screenshot",
        &raw mut key_menu_screenshot as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_demo_quit",
        &raw mut key_demo_quit as *mut ::core::ffi::c_void,
    );
    M_BindVariable("key_spy",
        &raw mut key_spy as *mut ::core::ffi::c_void,
    );
}
pub unsafe fn M_BindChatControls(mut num_players: u32) {
    let mut name: [::core::ffi::c_char; 32] = [0; 32];
    let mut i: u32 = 0;
    M_BindVariable("key_multi_msg",
        &raw mut key_multi_msg as *mut ::core::ffi::c_void,
    );
    i = 0 as u32;
    while i < num_players {
        M_snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
            b"key_multi_msgplayer%i\0" as *const u8 as *const ::core::ffi::c_char,
            i.wrapping_add(1 as u32),
        );
        M_BindVariable(
            ::std::ffi::CStr::from_ptr(&raw mut name as *mut ::core::ffi::c_char)
                .to_str()
                .unwrap(),
            (&raw mut key_multi_msgplayer as *mut i32).offset(i as isize)
                as *mut i32 as *mut ::core::ffi::c_void,
        );
        i = i.wrapping_add(1);
    }
}
pub unsafe fn M_ApplyPlatformDefaults() {}
pub const KEY_CAPSLOCK: i32 = 0x80
    + 0x3a as i32;
pub const KEY_SCRLCK: i32 = 0x80
    + 0x46 as i32;
