use crate::src::d_loop::{net_connect_data_t, net_gamesettings_t, loop_interface_t};
use crate::src::d_player::{player_t};
use crate::src::d_ticcmd::{ticcmd_t};
use crate::src::m_argv::M_CheckParm;
use crate::src::w_wad::W_CheckNumForName;
use crate::src::d_main::D_DoAdvanceDemo;
use crate::src::g_game::G_Ticker;
use crate::src::d_loop::D_RegisterLoopCallbacks;
use crate::src::d_loop::D_InitNetGame;
use crate::src::d_loop::D_StartNetGame;
use crate::src::d_main::startskill;
use crate::src::d_main::startepisode;
use crate::src::d_main::startmap;
use crate::src::d_main::startloadgame;
use crate::src::d_main::autostart;
use crate::src::g_game::lowres_turn;
use crate::src::w_checksum::W_Checksum;
use crate::src::d_main::advancedemo;
use crate::src::d_main::respawnparm;
use crate::src::g_game::demorecording;
use crate::src::r_main::viewangleoffset;
use crate::src::d_main::nomonsters;
use crate::src::d_main::fastparm;
use crate::src::g_game::timelimit;
use crate::src::g_game::demoplayback;
use crate::src::doomstat::gamemission;
use crate::src::m_misc::M_StringCopy;
use crate::src::g_game::deathmatch;
use crate::src::g_game::playeringame;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::d_mode::skill_t;
use crate::src::doomdef::boolean;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use libc::printf;

extern "C" {
    fn G_CheckDemoStatus() -> boolean;
}
use crate::src::d_main::D_ProcessEvents;
use crate::src::m_menu::M_Ticker;
use crate::src::g_game::G_BuildTiccmd;
use crate::src::doomdef::true_0;
use crate::src::doomdef::false_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::tables::ANG90;
use crate::src::tables::ANG270;
use crate::src::game_state::game_state;
pub struct DNetState {
    pub netcmds: *mut ticcmd_t,
}

impl DNetState {
    pub const fn new() -> Self {
        DNetState {
            netcmds: ::core::ptr::null::<ticcmd_t>() as *mut ticcmd_t,
        }
    }
}

unsafe fn PlayerQuitGame(mut player: *mut player_t) {
    static mut exitmsg: [::core::ffi::c_char; 80] = [0; 80];
    let mut player_num: u32 = 0;
    player_num = player.offset_from(&raw mut players as *mut player_t)
        as i64 as u32;
    M_StringCopy(
        &raw mut exitmsg as *mut ::core::ffi::c_char,
        b"Player 1 left the game\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as size_t,
    );
    exitmsg[7 as i32 as usize] = (exitmsg[7 as i32
        as usize] as u32)
        .wrapping_add(player_num) as ::core::ffi::c_char as ::core::ffi::c_char;
    playeringame[player_num as usize] = false_0 as boolean;
    players[consoleplayer as usize].message = &raw mut exitmsg
        as *mut ::core::ffi::c_char;
    if demorecording {
        G_CheckDemoStatus();
    }
}
unsafe fn RunTic(mut cmds: *mut ticcmd_t, mut ingame: *mut boolean) {
    let mut i: u32 = 0;
    i = 0 as u32;
    while i < MAXPLAYERS as u32 {
        if !demoplayback && playeringame[i as usize] != 0
            && *ingame.offset(i as isize) == 0
        {
            PlayerQuitGame(
                (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t,
            );
        }
        i = i.wrapping_add(1);
    }
    let gs = game_state();
    gs.d_net.netcmds = cmds;
    if advancedemo {
        D_DoAdvanceDemo();
    }
    G_Ticker(&mut gs.m_random, &mut gs.d_net);
}
static mut doom_loop_interface: loop_interface_t = unsafe {
    loop_interface_t {
        ProcessEvents: Some(D_ProcessEvents as unsafe fn() -> ()),
        BuildTiccmd: Some(
            G_BuildTiccmd
                as unsafe fn(*mut ticcmd_t, i32) -> (),
        ),
        RunTic: Some(RunTic as unsafe fn(*mut ticcmd_t, *mut boolean) -> ()),
        RunMenu: Some(M_Ticker as unsafe fn() -> ()),
    }
};
unsafe fn LoadGameSettings(mut settings: *mut net_gamesettings_t) {
    let mut i: u32 = 0;
    deathmatch = (*settings).deathmatch;
    startepisode = (*settings).episode;
    startmap = (*settings).map;
    startskill = (*settings).skill as skill_t;
    startloadgame = (*settings).loadgame;
    lowres_turn = (*settings).lowres_turn != 0;
    nomonsters = (*settings).nomonsters != 0;
    fastparm = (*settings).fast_monsters != 0;
    respawnparm = (*settings).respawn_monsters != 0;
    timelimit = (*settings).timelimit;
    consoleplayer = (*settings).consoleplayer;
    if lowres_turn {
        printf(
            b"NOTE: Turning resolution is reduced; this is probably because there is a client recording a Vanilla demo.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 0 as u32;
    while i < MAXPLAYERS as u32 {
        playeringame[i as usize] = (i < (*settings).num_players as u32)
            as i32 as boolean;
        i = i.wrapping_add(1);
    }
}
unsafe fn SaveGameSettings(mut settings: *mut net_gamesettings_t) {
    (*settings).deathmatch = deathmatch;
    (*settings).episode = startepisode;
    (*settings).map = startmap;
    (*settings).skill = startskill as i32;
    (*settings).loadgame = startloadgame;
    (*settings).gameversion = gameversion as i32;
    (*settings).nomonsters = nomonsters as i32;
    (*settings).fast_monsters = fastparm as i32;
    (*settings).respawn_monsters = respawnparm as i32;
    (*settings).timelimit = timelimit;
    (*settings).lowres_turn = (M_CheckParm("-record") > 0 as i32
        && M_CheckParm("-longtics") == 0 as i32) as i32;
}
unsafe fn InitConnectData(mut connect_data: *mut net_connect_data_t) {
    (*connect_data).max_players = MAXPLAYERS;
    (*connect_data).drone = false_0;
    if M_CheckParm("-left") > 0 as i32 {
        viewangleoffset = ANG90;
        (*connect_data).drone = true_0;
    }
    if M_CheckParm("-right") > 0 as i32 {
        viewangleoffset = ANG270 as i32;
        (*connect_data).drone = true_0;
    }
    (*connect_data).gamemode = gamemode as i32;
    (*connect_data).gamemission = gamemission as i32;
    (*connect_data).lowres_turn = (M_CheckParm("-record") > 0 as i32
        && M_CheckParm("-longtics") == 0 as i32) as i32;
    W_Checksum(unsafe { &mut game_state().w_checksum }, &raw mut (*connect_data).wad_sha1sum as *mut byte);
    (*connect_data).is_freedoom = (W_CheckNumForName("FREEDOOM")
        >= 0 as i32) as i32;
}
pub unsafe fn D_ConnectNetGame() {
    let mut connect_data: net_connect_data_t = net_connect_data_t {
        gamemode: 0,
        gamemission: 0,
        lowres_turn: 0,
        drone: 0,
        max_players: 0,
        is_freedoom: 0,
        wad_sha1sum: [0; 20],
        deh_sha1sum: [0; 20],
        player_class: 0,
    };
    InitConnectData(&raw mut connect_data);
    netgame = D_InitNetGame(&raw mut connect_data);
    if M_CheckParm("-solo-net") > 0 as i32 {
        netgame = true;
    }
}
pub unsafe fn D_CheckNetGame() {
    let mut settings: net_gamesettings_t = net_gamesettings_t {
        ticdup: 0,
        extratics: 0,
        deathmatch: 0,
        episode: 0,
        nomonsters: 0,
        fast_monsters: 0,
        respawn_monsters: 0,
        map: 0,
        skill: 0,
        gameversion: 0,
        lowres_turn: 0,
        new_sync: 0,
        timelimit: 0,
        loadgame: 0,
        random: 0,
        num_players: 0,
        consoleplayer: 0,
        player_classes: [0; 8],
    };
    if netgame {
        autostart = true;
    }
    D_RegisterLoopCallbacks(&raw mut doom_loop_interface);
    SaveGameSettings(&raw mut settings);
    D_StartNetGame(&raw mut settings, None);
    LoadGameSettings(&raw mut settings);
    printf(
        b"startskill %i  deathmatch: %i  startmap: %i  startepisode: %i\n\0" as *const u8
            as *const ::core::ffi::c_char,
        startskill as i32,
        deathmatch,
        startmap,
        startepisode,
    );
    printf(
        b"player %i of %i (%i nodes)\n\0" as *const u8 as *const ::core::ffi::c_char,
        consoleplayer + 1 as i32,
        settings.num_players,
        settings.num_players,
    );
    if timelimit > 0 as i32 && deathmatch != 0 {
        if timelimit == 20 as i32
            && M_CheckParm("-avg") != 0
        {
            printf(
                b"Austin Virtual Gaming: Levels will end after 20 minutes\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            printf(
                b"Levels will end after %d minute\0" as *const u8
                    as *const ::core::ffi::c_char,
                timelimit,
            );
            if timelimit > 1 as i32 {
                printf(b"s\0" as *const u8 as *const ::core::ffi::c_char);
            }
            printf(b".\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
}
