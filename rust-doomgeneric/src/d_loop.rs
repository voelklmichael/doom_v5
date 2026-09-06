use crate::src::i_system::I_Error;
use crate::src::i_timer::I_GetTimeMS;
use crate::src::i_video::I_StartTic;
use crate::src::dummy::net_client_connected;
use crate::src::dummy::drone;
use crate::src::i_timer::I_Sleep;
use crate::src::i_system::I_AtExit;

extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: i32,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
    fn I_GetTime() -> i32;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub type C2RustUnnamed = u32;
pub const BTS_SAVESHIFT: C2RustUnnamed = 2;
pub const BTS_SAVEMASK: C2RustUnnamed = 28;
pub const BTS_SAVEGAME: C2RustUnnamed = 2;
pub const BTS_PAUSE: C2RustUnnamed = 1;
pub const BT_WEAPONSHIFT: C2RustUnnamed = 3;
pub const BT_WEAPONMASK: C2RustUnnamed = 56;
pub const BT_CHANGE: C2RustUnnamed = 4;
pub const BT_SPECIALMASK: C2RustUnnamed = 3;
pub const BT_SPECIAL: C2RustUnnamed = 128;
pub const BT_USE: C2RustUnnamed = 2;
pub const BT_ATTACK: C2RustUnnamed = 1;
pub use crate::src::d_ticcmd::ticcmd_t;
pub type sha1_digest_t = [byte; 20];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_connect_data_t {
    pub gamemode: i32,
    pub gamemission: i32,
    pub lowres_turn: i32,
    pub drone: i32,
    pub max_players: i32,
    pub is_freedoom: i32,
    pub wad_sha1sum: sha1_digest_t,
    pub deh_sha1sum: sha1_digest_t,
    pub player_class: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_gamesettings_t {
    pub ticdup: i32,
    pub extratics: i32,
    pub deathmatch: i32,
    pub episode: i32,
    pub nomonsters: i32,
    pub fast_monsters: i32,
    pub respawn_monsters: i32,
    pub map: i32,
    pub skill: i32,
    pub gameversion: i32,
    pub lowres_turn: i32,
    pub new_sync: i32,
    pub timelimit: i32,
    pub loadgame: i32,
    pub random: i32,
    pub num_players: i32,
    pub consoleplayer: i32,
    pub player_classes: [i32; 8],
}
pub type netgame_startup_callback_t = Option<
    unsafe extern "C" fn(i32, i32) -> boolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_interface_t {
    pub ProcessEvents: Option<unsafe extern "C" fn() -> ()>,
    pub BuildTiccmd: Option<
        unsafe extern "C" fn(*mut ticcmd_t, i32) -> (),
    >,
    pub RunTic: Option<unsafe extern "C" fn(*mut ticcmd_t, *mut boolean) -> ()>,
    pub RunMenu: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ticcmd_set_t {
    pub cmds: [ticcmd_t; 8],
    pub ingame: [boolean; 8],
}
pub type fixed_t = i32;
pub type atexit_func_t = Option<unsafe extern "C" fn() -> ()>;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NET_MAXPLAYERS: i32 = 8 as i32;
pub const BACKUPTICS: i32 = 128 as i32;
pub const TICRATE: i32 = 35 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub const FRACUNIT: i32 = (1 as i32) << FRACBITS;
static mut ticdata: [ticcmd_set_t; 128] = [ticcmd_set_t {
    cmds: [ticcmd_t {
        forwardmove: 0,
        sidemove: 0,
        angleturn: 0,
        chatchar: 0,
        buttons: 0,
        consistancy: 0,
        buttons2: 0,
        inventory: 0,
        lookfly: 0,
        arti: 0,
    }; 8],
    ingame: [0; 8],
}; 128];
static mut maketic: i32 = 0;
static mut recvtic: i32 = 0;
pub static mut gametic: i32 = 0;
pub static mut singletics: bool = false;
static mut localplayer: i32 = 0;
static mut skiptics: i32 = 0 as i32;
pub static mut ticdup: i32 = 0;
#[no_mangle]
pub static mut offsetms: fixed_t = 0;
static mut new_sync: bool = true;
static mut loop_interface: *mut loop_interface_t = ::core::ptr::null::<
    loop_interface_t,
>() as *mut loop_interface_t;
static mut local_playeringame: [boolean; 8] = [0; 8];
static mut player_class: i32 = 0;
unsafe extern "C" fn GetAdjustedTime() -> i32 {
    let mut time_ms: i32 = 0;
    time_ms = I_GetTimeMS();
    if new_sync {
        time_ms += offsetms as i32 / FRACUNIT;
    }
    return time_ms * TICRATE / 1000 as i32;
}
unsafe extern "C" fn BuildNewTic() -> boolean {
    let mut gameticdiv: i32 = 0;
    let mut cmd: ticcmd_t = ticcmd_t {
        forwardmove: 0,
        sidemove: 0,
        angleturn: 0,
        chatchar: 0,
        buttons: 0,
        consistancy: 0,
        buttons2: 0,
        inventory: 0,
        lookfly: 0,
        arti: 0,
    };
    gameticdiv = gametic / ticdup;
    I_StartTic();
    ::core::mem::transmute::<
        _,
        fn(),
    >((*loop_interface).ProcessEvents.expect("non-null function pointer"))();
    ::core::mem::transmute::<
        _,
        fn(),
    >((*loop_interface).RunMenu.expect("non-null function pointer"))();
    if drone {
        return false_0 as boolean;
    }
    if new_sync {
        if !net_client_connected && maketic - gameticdiv > 2 as i32 {
            return false_0 as boolean;
        }
        if maketic - gameticdiv > 8 as i32 {
            return false_0 as boolean;
        }
    } else if maketic - gameticdiv >= 5 as i32 {
        return false_0 as boolean
    }
    memset(
        &raw mut cmd as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<ticcmd_t>() as size_t,
    );
    (*loop_interface)
        .BuildTiccmd
        .expect("non-null function pointer")(&raw mut cmd, maketic);
    ticdata[(maketic % BACKUPTICS) as usize].cmds[localplayer as usize] = cmd;
    ticdata[(maketic % BACKUPTICS) as usize].ingame[localplayer as usize] = true_0
        as boolean;
    maketic += 1;
    return true_0 as boolean;
}
#[no_mangle]
pub static mut lasttime: i32 = 0;
pub unsafe fn NetUpdate() {
    let mut nowtime: i32 = 0;
    let mut newtics: i32 = 0;
    let mut i: i32 = 0;
    if singletics {
        return;
    }
    nowtime = GetAdjustedTime() / ticdup;
    newtics = nowtime - lasttime;
    lasttime = nowtime;
    if skiptics <= newtics {
        newtics -= skiptics;
        skiptics = 0 as i32;
    } else {
        skiptics -= newtics;
        newtics = 0 as i32;
    }
    i = 0 as i32;
    while i < newtics {
        if BuildNewTic() == 0 {
            break;
        }
        i += 1;
    }
}
unsafe extern "C" fn D_Disconnected() {
    if drone {
        I_Error("Disconnected from server in drone mode.");
    }
    printf(b"Disconnected from server.\n\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn D_ReceiveTic(
    mut ticcmds: *mut ticcmd_t,
    mut players_mask: *mut boolean,
) {
    let mut i: i32 = 0;
    if ticcmds.is_null() && players_mask.is_null() {
        D_Disconnected();
        return;
    }
    i = 0 as i32;
    while i < NET_MAXPLAYERS {
        if !(!drone && i == localplayer) {
            ticdata[(recvtic % BACKUPTICS) as usize].cmds[i as usize] = *ticcmds
                .offset(i as isize);
            ticdata[(recvtic % BACKUPTICS) as usize].ingame[i as usize] = *players_mask
                .offset(i as isize);
        }
        i += 1;
    }
    recvtic += 1;
}
pub unsafe fn D_StartGameLoop() {
    lasttime = GetAdjustedTime() / ticdup;
}
pub unsafe fn D_StartNetGame(
    mut settings: *mut net_gamesettings_t,
    mut callback: netgame_startup_callback_t,
) {
    (*settings).consoleplayer = 0 as i32;
    (*settings).num_players = 1 as i32;
    (*settings).player_classes[0 as i32 as usize] = player_class;
    (*settings).new_sync = 0 as i32;
    (*settings).extratics = 1 as i32;
    (*settings).ticdup = 1 as i32;
    ticdup = (*settings).ticdup;
    new_sync = (*settings).new_sync != 0;
}
pub unsafe fn D_InitNetGame(
    mut connect_data: *mut net_connect_data_t,
) -> boolean {
    let mut result: boolean = false_0 as boolean;
    I_AtExit(Some(D_QuitNetGame as unsafe extern "C" fn() -> ()), true_0 as boolean);
    player_class = (*connect_data).player_class;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn D_QuitNetGame() {}
unsafe extern "C" fn GetLowTic() -> i32 {
    let mut lowtic: i32 = 0;
    lowtic = maketic;
    return lowtic;
}
static mut frameon: i32 = 0;
static mut frameskip: [i32; 4] = [0; 4];
static mut oldnettics: i32 = 0;
unsafe extern "C" fn OldNetSync() {
    let mut i: u32 = 0;
    let mut keyplayer: i32 = -(1 as i32);
    frameon += 1;
    i = 0 as u32;
    while i < NET_MAXPLAYERS as u32 {
        if local_playeringame[i as usize] != 0 {
            keyplayer = i as i32;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if keyplayer < 0 as i32 {
        return;
    }
    if !(localplayer == keyplayer) {
        if maketic <= recvtic {
            lasttime -= 1;
        }
        frameskip[(frameon & 3 as i32) as usize] = (oldnettics > recvtic)
            as i32;
        oldnettics = maketic;
        if frameskip[0 as i32 as usize] != 0
            && frameskip[1 as i32 as usize] != 0
            && frameskip[2 as i32 as usize] != 0
            && frameskip[3 as i32 as usize] != 0
        {
            skiptics = 1 as i32;
        }
    }
}
unsafe extern "C" fn PlayersInGame() -> boolean {
    let mut result: boolean = false_0 as boolean;
    let mut i: u32 = 0;
    if net_client_connected {
        i = 0 as u32;
        while i < NET_MAXPLAYERS as u32 {
            result = (result != 0 || local_playeringame[i as usize] != 0)
                as i32 as boolean;
            i = i.wrapping_add(1);
        }
    }
    if !drone {
        result = true_0 as boolean;
    }
    return result;
}
unsafe extern "C" fn TicdupSquash(mut set: *mut ticcmd_set_t) {
    let mut cmd: *mut ticcmd_t = ::core::ptr::null_mut::<ticcmd_t>();
    let mut i: u32 = 0;
    i = 0 as u32;
    while i < NET_MAXPLAYERS as u32 {
        cmd = (&raw mut (*set).cmds as *mut ticcmd_t).offset(i as isize)
            as *mut ticcmd_t;
        (*cmd).chatchar = 0 as byte;
        if (*cmd).buttons as i32 & BT_SPECIAL as i32 != 0 {
            (*cmd).buttons = 0 as byte;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn SinglePlayerClear(mut set: *mut ticcmd_set_t) {
    let mut i: u32 = 0;
    i = 0 as u32;
    while i < NET_MAXPLAYERS as u32 {
        if i != localplayer as u32 {
            (*set).ingame[i as usize] = false_0 as boolean;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TryRunTics() {
    let mut i: i32 = 0;
    let mut lowtic: i32 = 0;
    let mut entertic: i32 = 0;
    static mut oldentertics: i32 = 0;
    let mut realtics: i32 = 0;
    let mut availabletics: i32 = 0;
    let mut counts: i32 = 0;
    entertic = I_GetTime() / ticdup;
    realtics = entertic - oldentertics;
    oldentertics = entertic;
    if singletics {
        BuildNewTic();
    } else {
        NetUpdate();
    }
    lowtic = GetLowTic();
    availabletics = lowtic - gametic / ticdup;
    if new_sync {
        counts = availabletics;
    } else {
        if realtics < availabletics - 1 as i32 {
            counts = realtics + 1 as i32;
        } else if realtics < availabletics {
            counts = realtics;
        } else {
            counts = availabletics;
        }
        if counts < 1 as i32 {
            counts = 1 as i32;
        }
        if net_client_connected {
            OldNetSync();
        }
    }
    if counts < 1 as i32 {
        counts = 1 as i32;
    }
    while PlayersInGame() == 0 || lowtic < gametic / ticdup + counts {
        NetUpdate();
        lowtic = GetLowTic();
        if lowtic < gametic / ticdup {
            I_Error("TryRunTics: lowtic < gametic");
        }
        if I_GetTime() / ticdup - entertic > 0 as i32 {
            return;
        }
        I_Sleep(1 as i32);
    }
    loop {
        let fresh0 = counts;
        counts = counts - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut set: *mut ticcmd_set_t = ::core::ptr::null_mut::<ticcmd_set_t>();
        if PlayersInGame() == 0 {
            return;
        }
        set = (&raw mut ticdata as *mut ticcmd_set_t)
            .offset((gametic / ticdup % BACKUPTICS) as isize) as *mut ticcmd_set_t;
        if !net_client_connected {
            SinglePlayerClear(set);
        }
        i = 0 as i32;
        while i < ticdup {
            if gametic / ticdup > lowtic {
                I_Error("gametic>lowtic");
            }
            memcpy(
                &raw mut local_playeringame as *mut boolean as *mut ::core::ffi::c_void,
                &raw mut (*set).ingame as *mut boolean as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[boolean; 8]>() as size_t,
            );
            (*loop_interface)
                .RunTic
                .expect(
                    "non-null function pointer",
                )(
                &raw mut (*set).cmds as *mut ticcmd_t,
                &raw mut (*set).ingame as *mut boolean,
            );
            gametic += 1;
            TicdupSquash(set);
            i += 1;
        }
        NetUpdate();
    };
}
pub unsafe fn D_RegisterLoopCallbacks(mut i: *mut loop_interface_t) {
    loop_interface = i;
}
