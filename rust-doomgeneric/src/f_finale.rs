use crate::src::r_data::column_t;
use crate::src::r_defs::{spritedef_t, spriteframe_t};
use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::p_mobj::{state_t, mobjinfo_t};
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName};
use crate::src::d_main::wipegamestate;
use crate::src::g_game::gameaction;
use crate::src::hu_stuff::hu_font;
use crate::src::r_data::firstspritelump;
use crate::src::r_things::sprites;
use crate::src::s_sound::S_StartMusic;
use crate::src::g_game::gamestate;
use crate::src::g_game::viewactive;
use crate::src::s_sound::S_ChangeMusic;
use crate::src::v_video::V_MarkRect;
use crate::src::i_video::I_VideoBuffer;
use crate::src::info::mobjinfo;
use crate::src::g_game::gameepisode;
use crate::src::doomstat::gamemission;
use crate::src::g_game::gamemap;
use crate::src::info::states;
use crate::src::am_map::automapactive;
use crate::src::doomstat::gameversion;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::v_video::V_DrawPatchFlipped;
use crate::src::v_video::V_DrawPatch;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::z_zone::{PU_CACHE, PU_LEVEL};
use crate::src::sounds::{sfx_claw, sfx_dshtgn, sfx_firsht, sfx_pistol, sfx_plasma, sfx_rlaunc, sfx_sgtatk, sfx_shotgn, sfx_skeatk, sfx_skepch, sfx_skeswg, sfx_sklatk, sfx_vilatk};
use crate::src::sounds::{mus_bunny, mus_evil, mus_read_m, mus_victor};
use crate::src::p_mobj::{MT_BABY, MT_BRUISER, MT_CHAINGUY, MT_CYBORG, MT_FATSO, MT_HEAD, MT_KNIGHT, MT_PAIN, MT_PLAYER, MT_POSSESSED, MT_SERGEANT, MT_SHOTGUY, MT_SKULL, MT_SPIDER, MT_TROOP, MT_UNDEAD, MT_VILE, mobjtype_t};
use crate::src::d_mode::{commercial, retail};
use crate::src::d_mode::exe_chex;
use crate::src::d_mode::{GameMission_t, doom, doom2, pack_chex, pack_hacx, pack_plut, pack_tnt};
use crate::src::d_event::ev_keydown;
use crate::src::d_event::{GS_FINALE, gamestate_t};
use crate::src::d_event::{ga_nothing, ga_worlddone};
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use libc::memcpy;
use libc::toupper;
use libc::snprintf;
use crate::src::info::{S_NULL, S_PLAY_ATK1};
use crate::src::doomdef::NULL;
use crate::src::doomdef::true_0;
use crate::src::doomdef::MAXPLAYERS;
use crate::src::doomdef::SCREENWIDTH;
use crate::src::doomdef::SCREENHEIGHT;
use crate::src::r_things::FF_FRAMEMASK;
use crate::src::hu_stuff::HU_FONTSTART;
use crate::src::hu_stuff::HU_FONTSIZE;
pub type finalestage_t = u32;
pub const F_STAGE_CAST: finalestage_t = 2;
pub const F_STAGE_ARTSCREEN: finalestage_t = 1;
pub const F_STAGE_TEXT: finalestage_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textscreen_t {
    pub mission: GameMission_t,
    pub episode: i32,
    pub level: i32,
    pub background: &'static str,
    pub text: &'static str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct castinfo_t {
    pub name: Option<&'static str>,
    pub type_0: mobjtype_t,
}
pub const E1TEXT: &str = "Once you beat the big badasses and\nclean out the moon base you're supposed\nto win, aren't you? Aren't you? Where's\nyour fat reward and ticket home? What\nthe hell is this? It's not supposed to\nend this way!\n\nIt stinks like rotten meat, but looks\nlike the lost Deimos base.  Looks like\nyou're stuck on The Shores of Hell.\nThe only way out is through.\n\nTo continue the DOOM experience, play\nThe Shores of Hell and its amazing\nsequel, Inferno!\n";
pub const E2TEXT: &str = "You've done it! The hideous cyber-\ndemon lord that ruled the lost Deimos\nmoon base has been slain and you\nare triumphant! But ... where are\nyou? You clamber to the edge of the\nmoon and look down to see the awful\ntruth.\n\nDeimos floats above Hell itself!\nYou've never heard of anyone escaping\nfrom Hell, but you'll make the bastards\nsorry they ever heard of you! Quickly,\nyou rappel down to  the surface of\nHell.\n\nNow, it's on to the final chapter of\nDOOM! -- Inferno.";
pub const E3TEXT: &str = "The loathsome spiderdemon that\nmasterminded the invasion of the moon\nbases and caused so much death has had\nits ass kicked for all time.\n\nA hidden doorway opens and you enter.\nYou've proven too tough for Hell to\ncontain, and now Hell at last plays\nfair -- for you emerge from the door\nto see the green fields of Earth!\nHome at last.\n\nYou wonder what's been happening on\nEarth while you were battling evil\nunleashed. It's good that no Hell-\nspawn could have come through that\ndoor with you ...";
pub const E4TEXT: &str = "the spider mastermind must have sent forth\nits legions of hellspawn before your\nfinal confrontation with that terrible\nbeast from hell.  but you stepped forward\nand brought forth eternal damnation and\nsuffering upon the horde as a true hero\nwould in the face of something so evil.\n\nbesides, someone was gonna pay for what\nhappened to daisy, your pet rabbit.\n\nbut now, you see spread before you more\npotential pain and gibbitude as a nation\nof demons run amok among our cities.\n\nnext stop, hell on earth!";
pub const C1TEXT: &str = "YOU HAVE ENTERED DEEPLY INTO THE INFESTED\nSTARPORT. BUT SOMETHING IS WRONG. THE\nMONSTERS HAVE BROUGHT THEIR OWN REALITY\nWITH THEM, AND THE STARPORT'S TECHNOLOGY\nIS BEING SUBVERTED BY THEIR PRESENCE.\n\nAHEAD, YOU SEE AN OUTPOST OF HELL, A\nFORTIFIED ZONE. IF YOU CAN GET PAST IT,\nYOU CAN PENETRATE INTO THE HAUNTED HEART\nOF THE STARBASE AND FIND THE CONTROLLING\nSWITCH WHICH HOLDS EARTH'S POPULATION\nHOSTAGE.";
pub const C2TEXT: &str = "YOU HAVE WON! YOUR VICTORY HAS ENABLED\nHUMANKIND TO EVACUATE EARTH AND ESCAPE\nTHE NIGHTMARE.  NOW YOU ARE THE ONLY\nHUMAN LEFT ON THE FACE OF THE PLANET.\nCANNIBAL MUTATIONS, CARNIVOROUS ALIENS,\nAND EVIL SPIRITS ARE YOUR ONLY NEIGHBORS.\nYOU SIT BACK AND WAIT FOR DEATH, CONTENT\nTHAT YOU HAVE SAVED YOUR SPECIES.\n\nBUT THEN, EARTH CONTROL BEAMS DOWN A\nMESSAGE FROM SPACE: \"SENSORS HAVE LOCATED\nTHE SOURCE OF THE ALIEN INVASION. IF YOU\nGO THERE, YOU MAY BE ABLE TO BLOCK THEIR\nENTRY.  THE ALIEN BASE IS IN THE HEART OF\nYOUR OWN HOME CITY, NOT FAR FROM THE\nSTARPORT.\" SLOWLY AND PAINFULLY YOU GET\nUP AND RETURN TO THE FRAY.";
pub const C3TEXT: &str = "YOU ARE AT THE CORRUPT HEART OF THE CITY,\nSURROUNDED BY THE CORPSES OF YOUR ENEMIES.\nYOU SEE NO WAY TO DESTROY THE CREATURES'\nENTRYWAY ON THIS SIDE, SO YOU CLENCH YOUR\nTEETH AND PLUNGE THROUGH IT.\n\nTHERE MUST BE A WAY TO CLOSE IT ON THE\nOTHER SIDE. WHAT DO YOU CARE IF YOU'VE\nGOT TO GO THROUGH HELL TO GET TO IT?";
pub const C4TEXT: &str = "THE HORRENDOUS VISAGE OF THE BIGGEST\nDEMON YOU'VE EVER SEEN CRUMBLES BEFORE\nYOU, AFTER YOU PUMP YOUR ROCKETS INTO\nHIS EXPOSED BRAIN. THE MONSTER SHRIVELS\nUP AND DIES, ITS THRASHING LIMBS\nDEVASTATING UNTOLD MILES OF HELL'S\nSURFACE.\n\nYOU'VE DONE IT. THE INVASION IS OVER.\nEARTH IS SAVED. HELL IS A WRECK. YOU\nWONDER WHERE BAD FOLKS WILL GO WHEN THEY\nDIE, NOW. WIPING THE SWEAT FROM YOUR\nFOREHEAD YOU BEGIN THE LONG TREK BACK\nHOME. REBUILDING EARTH OUGHT TO BE A\nLOT MORE FUN THAN RUINING IT WAS.\n";
pub const C5TEXT: &str = "CONGRATULATIONS, YOU'VE FOUND THE SECRET\nLEVEL! LOOKS LIKE IT'S BEEN BUILT BY\nHUMANS, RATHER THAN DEMONS. YOU WONDER\nWHO THE INMATES OF THIS CORNER OF HELL\nWILL BE.";
pub const C6TEXT: &str = "CONGRATULATIONS, YOU'VE FOUND THE\nSUPER SECRET LEVEL!  YOU'D BETTER\nBLAZE THROUGH THIS ONE!\n";
pub const P1TEXT: &str = "You gloat over the steaming carcass of the\nGuardian.  With its death, you've wrested\nthe Accelerator from the stinking claws\nof Hell.  You relax and glance around the\nroom.  Damn!  There was supposed to be at\nleast one working prototype, but you can't\nsee it. The demons must have taken it.\n\nYou must find the prototype, or all your\nstruggles will have been wasted. Keep\nmoving, keep fighting, keep killing.\nOh yes, keep living, too.";
pub const P2TEXT: &str = "Even the deadly Arch-Vile labyrinth could\nnot stop you, and you've gotten to the\nprototype Accelerator which is soon\nefficiently and permanently deactivated.\n\nYou're good at that kind of thing.";
pub const P3TEXT: &str = "You've bashed and battered your way into\nthe heart of the devil-hive.  Time for a\nSearch-and-Destroy mission, aimed at the\nGatekeeper, whose foul offspring is\ncascading to Earth.  Yeah, he's bad. But\nyou know who's worse!\n\nGrinning evilly, you check your gear, and\nget ready to give the bastard a little Hell\nof your own making!";
pub const P4TEXT: &str = "The Gatekeeper's evil face is splattered\nall over the place.  As its tattered corpse\ncollapses, an inverted Gate forms and\nsucks down the shards of the last\nprototype Accelerator, not to mention the\nfew remaining demons.  You're done. Hell\nhas gone back to pounding bad dead folks \ninstead of good live ones.  Remember to\ntell your grandkids to put a rocket\nlauncher in your coffin. If you go to Hell\nwhen you die, you'll need it for some\nfinal cleaning-up ...";
pub const P5TEXT: &str = "You've found the second-hardest level we\ngot. Hope you have a saved game a level or\ntwo previous.  If not, be prepared to die\naplenty. For master marines only.";
pub const P6TEXT: &str = "Betcha wondered just what WAS the hardest\nlevel we had ready for ya?  Now you know.\nNo one gets out alive.";
pub const T1TEXT: &str = "You've fought your way out of the infested\nexperimental labs.   It seems that UAC has\nonce again gulped it down.  With their\nhigh turnover, it must be hard for poor\nold UAC to buy corporate health insurance\nnowadays..\n\nAhead lies the military complex, now\nswarming with diseased horrors hot to get\ntheir teeth into you. With luck, the\ncomplex still has some warlike ordnance\nlaying around.";
pub const T2TEXT: &str = "You hear the grinding of heavy machinery\nahead.  You sure hope they're not stamping\nout new hellspawn, but you're ready to\nream out a whole herd if you have to.\nThey might be planning a blood feast, but\nyou feel about as mean as two thousand\nmaniacs packed into one mad killer.\n\nYou don't plan to go down easy.";
pub const T3TEXT: &str = "The vista opening ahead looks real damn\nfamiliar. Smells familiar, too -- like\nfried excrement. You didn't like this\nplace before, and you sure as hell ain't\nplanning to like it now. The more you\nbrood on it, the madder you get.\nHefting your gun, an evil grin trickles\nonto your face. Time to take some names.";
pub const T4TEXT: &str = "Suddenly, all is silent, from one horizon\nto the other. The agonizing echo of Hell\nfades away, the nightmare sky turns to\nblue, the heaps of monster corpses start \nto evaporate along with the evil stench \nthat filled the air. Jeeze, maybe you've\ndone it. Have you really won?\n\nSomething rumbles in the distance.\nA blue light begins to glow inside the\nruined skull of the demon-spitter.";
pub const T5TEXT: &str = "What now? Looks totally different. Kind\nof like King Tut's condo. Well,\nwhatever's here can't be any worse\nthan usual. Can it?  Or maybe it's best\nto let sleeping gods lie..";
pub const T6TEXT: &str = "Time for a vacation. You've burst the\nbowels of hell and by golly you're ready\nfor a break. You mutter to yourself,\nMaybe someone else can kick Hell's ass\nnext time around. Ahead lies a quiet town,\nwith peaceful flowing water, quaint\nbuildings, and presumably no Hellspawn.\n\nAs you step off the transport, you hear\nthe stomp of a cyberdemon's iron shoe.";
pub const CC_ZOMBIE: &str = "ZOMBIEMAN";
pub const CC_SHOTGUN: &str = "SHOTGUN GUY";
pub const CC_HEAVY: &str = "HEAVY WEAPON DUDE";
pub const CC_IMP: &str = "IMP";
pub const CC_DEMON: &str = "DEMON";
pub const CC_LOST: &str = "LOST SOUL";
pub const CC_CACO: &str = "CACODEMON";
pub const CC_HELL: &str = "HELL KNIGHT";
pub const CC_BARON: &str = "BARON OF HELL";
pub const CC_ARACH: &str = "ARACHNOTRON";
pub const CC_PAIN: &str = "PAIN ELEMENTAL";
pub const CC_REVEN: &str = "REVENANT";
pub const CC_MANCU: &str = "MANCUBUS";
pub const CC_ARCH: &str = "ARCH-VILE";
pub const CC_SPIDER: &str = "THE SPIDER MASTERMIND";
pub const CC_CYBER: &str = "THE CYBERDEMON";
pub const CC_HERO: &str = "OUR HERO";
#[no_mangle]
pub static mut finalestage: finalestage_t = F_STAGE_TEXT;
#[no_mangle]
pub static mut finalecount: u32 = 0;
pub const TEXTSPEED: i32 = 3;
pub const TEXTWAIT: i32 = 250;
static mut textscreens: [textscreen_t; 22] = [
    textscreen_t {
        mission: doom,
        episode: 1 as i32,
        level: 8 as i32,
        background: "FLOOR4_8",
        text: E1TEXT,
    },
    textscreen_t {
        mission: doom,
        episode: 2 as i32,
        level: 8 as i32,
        background: "SFLR6_1",
        text: E2TEXT,
    },
    textscreen_t {
        mission: doom,
        episode: 3 as i32,
        level: 8 as i32,
        background: "MFLR8_4",
        text: E3TEXT,
    },
    textscreen_t {
        mission: doom,
        episode: 4 as i32,
        level: 8 as i32,
        background: "MFLR8_3",
        text: E4TEXT,
    },
    textscreen_t {
        mission: doom2,
        episode: 1 as i32,
        level: 6 as i32,
        background: "SLIME16",
        text: C1TEXT,
    },
    textscreen_t {
        mission: doom2,
        episode: 1 as i32,
        level: 11 as i32,
        background: "RROCK14",
        text: C2TEXT,
    },
    textscreen_t {
        mission: doom2,
        episode: 1 as i32,
        level: 20 as i32,
        background: "RROCK07",
        text: C3TEXT,
    },
    textscreen_t {
        mission: doom2,
        episode: 1 as i32,
        level: 30 as i32,
        background: "RROCK17",
        text: C4TEXT,
    },
    textscreen_t {
        mission: doom2,
        episode: 1 as i32,
        level: 15 as i32,
        background: "RROCK13",
        text: C5TEXT,
    },
    textscreen_t {
        mission: doom2,
        episode: 1 as i32,
        level: 31 as i32,
        background: "RROCK19",
        text: C6TEXT,
    },
    textscreen_t {
        mission: pack_tnt,
        episode: 1 as i32,
        level: 6 as i32,
        background: "SLIME16",
        text: T1TEXT,
    },
    textscreen_t {
        mission: pack_tnt,
        episode: 1 as i32,
        level: 11 as i32,
        background: "RROCK14",
        text: T2TEXT,
    },
    textscreen_t {
        mission: pack_tnt,
        episode: 1 as i32,
        level: 20 as i32,
        background: "RROCK07",
        text: T3TEXT,
    },
    textscreen_t {
        mission: pack_tnt,
        episode: 1 as i32,
        level: 30 as i32,
        background: "RROCK17",
        text: T4TEXT,
    },
    textscreen_t {
        mission: pack_tnt,
        episode: 1 as i32,
        level: 15 as i32,
        background: "RROCK13",
        text: T5TEXT,
    },
    textscreen_t {
        mission: pack_tnt,
        episode: 1 as i32,
        level: 31 as i32,
        background: "RROCK19",
        text: T6TEXT,
    },
    textscreen_t {
        mission: pack_plut,
        episode: 1 as i32,
        level: 6 as i32,
        background: "SLIME16",
        text: P1TEXT,
    },
    textscreen_t {
        mission: pack_plut,
        episode: 1 as i32,
        level: 11 as i32,
        background: "RROCK14",
        text: P2TEXT,
    },
    textscreen_t {
        mission: pack_plut,
        episode: 1 as i32,
        level: 20 as i32,
        background: "RROCK07",
        text: P3TEXT,
    },
    textscreen_t {
        mission: pack_plut,
        episode: 1 as i32,
        level: 30 as i32,
        background: "RROCK17",
        text: P4TEXT,
    },
    textscreen_t {
        mission: pack_plut,
        episode: 1 as i32,
        level: 15 as i32,
        background: "RROCK13",
        text: P5TEXT,
    },
    textscreen_t {
        mission: pack_plut,
        episode: 1 as i32,
        level: 31 as i32,
        background: "RROCK19",
        text: P6TEXT,
    },
];
#[no_mangle]
pub static mut finaletext: &str = "";
#[no_mangle]
pub static mut finaleflat: &str = "";
pub unsafe fn F_StartFinale() {
    let mut i: size_t = 0;
    gameaction = ga_nothing;
    gamestate = GS_FINALE;
    viewactive = false;
    automapactive = false;
    if (if gamemission as u32
        == pack_chex as i32 as u32
    {
        doom as i32 as u32
    } else {
        (if gamemission as u32
            == pack_hacx as i32 as u32
        {
            doom2 as i32 as u32
        } else {
            gamemission as u32
        })
    }) == doom as i32 as u32
    {
        S_ChangeMusic(mus_victor as i32, true_0);
    } else {
        S_ChangeMusic(mus_read_m as i32, true_0);
    }
    i = 0 as size_t;
    while i
        < (::core::mem::size_of::<[textscreen_t; 22]>() as usize)
            .wrapping_div(::core::mem::size_of::<textscreen_t>() as usize)
    {
        let mut screen: *mut textscreen_t = (&raw mut textscreens as *mut textscreen_t)
            .offset(i as isize) as *mut textscreen_t;
        if gameversion as u32
            == exe_chex as i32 as u32
            && (*screen).mission as u32
                == doom as i32 as u32
        {
            (*screen).level = 5 as i32;
        }
        if (if gamemission as u32
            == pack_chex as i32 as u32
        {
            doom as i32 as u32
        } else {
            (if gamemission as u32
                == pack_hacx as i32 as u32
            {
                doom2 as i32 as u32
            } else {
                gamemission as u32
            })
        }) == (*screen).mission as u32
            && ((if gamemission as u32
                == pack_chex as i32 as u32
            {
                doom as i32 as u32
            } else {
                (if gamemission as u32
                    == pack_hacx as i32 as u32
                {
                    doom2 as i32 as u32
                } else {
                    gamemission as u32
                })
            }) != doom as i32 as u32
                || gameepisode == (*screen).episode) && gamemap == (*screen).level
        {
            finaletext = (*screen).text;
            finaleflat = (*screen).background;
        }
        i = i.wrapping_add(1);
    }
    finaletext = finaletext;
    finaleflat = finaleflat;
    finalestage = F_STAGE_TEXT;
    finalecount = 0 as u32;
}
pub unsafe fn F_Responder(mut event: *mut event_t) -> bool {
    if finalestage as u32
        == F_STAGE_CAST as i32 as u32
    {
        return F_CastResponder(event);
    }
    return false;
}
pub unsafe fn F_Ticker() {
    let mut i: size_t = 0;
    if gamemode as u32
        == commercial as i32 as u32
        && finalecount > 50 as u32
    {
        i = 0 as size_t;
        while i < MAXPLAYERS as size_t {
            if players[i as usize].cmd.buttons != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i < MAXPLAYERS as size_t {
            if gamemap == 30 as i32 {
                F_StartCast();
            } else {
                gameaction = ga_worlddone;
            }
        }
    }
    finalecount = finalecount.wrapping_add(1);
    if finalestage as u32
        == F_STAGE_CAST as i32 as u32
    {
        F_CastTicker();
        return;
    }
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if finalestage as u32
        == F_STAGE_TEXT as i32 as u32
        && finalecount as size_t
            > (finaletext.len() as size_t)
                .wrapping_mul(TEXTSPEED as size_t)
                .wrapping_add(TEXTWAIT as size_t)
    {
        finalecount = 0 as u32;
        finalestage = F_STAGE_ARTSCREEN;
        wipegamestate = 4294967295 as gamestate_t;
        if gameepisode == 3 as i32 {
            S_StartMusic(mus_bunny as i32);
        }
    }
}
pub unsafe fn F_TextWrite() {
    let mut src: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    let mut count: i32 = 0;
    let mut c: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    src = W_CacheLumpName(finaleflat, PU_CACHE as i32) as *mut byte;
    dest = I_VideoBuffer;
    y = 0 as i32;
    while y < SCREENHEIGHT {
        x = 0 as i32;
        while x < SCREENWIDTH / 64 as i32 {
            memcpy(
                dest as *mut ::core::ffi::c_void,
                src
                    .offset(
                        ((y & 63 as i32) << 6 as i32)
                            as isize,
                    ) as *const ::core::ffi::c_void,
                64 as size_t,
            );
            dest = dest.offset(64 as i32 as isize);
            x += 1;
        }
        if SCREENWIDTH & 63 as i32 != 0 {
            memcpy(
                dest as *mut ::core::ffi::c_void,
                src
                    .offset(
                        ((y & 63 as i32) << 6 as i32)
                            as isize,
                    ) as *const ::core::ffi::c_void,
                (SCREENWIDTH & 63 as i32) as size_t,
            );
            dest = dest.offset((SCREENWIDTH & 63 as i32) as isize);
        }
        y += 1;
    }
    V_MarkRect(
        0 as i32,
        0 as i32,
        SCREENWIDTH,
        SCREENHEIGHT,
    );
    cx = 10 as i32;
    cy = 10 as i32;
    let mut chars = finaletext.bytes();
    count = (finalecount as i32 - 10 as i32) / TEXTSPEED;
    if count < 0 as i32 {
        count = 0 as i32;
    }
    while count != 0 {
        c = match chars.next() {
            Some(b) => b as i32,
            None => break,
        };
        if c == '\n' as i32 {
            cx = 10 as i32;
            cy += 11 as i32;
        } else {
            c = toupper(c) - HU_FONTSTART;
            if c < 0 as i32 || c > HU_FONTSIZE {
                cx += 4 as i32;
            } else {
                w = (*hu_font[c as usize]).width as i32;
                if cx + w > SCREENWIDTH {
                    break;
                }
                V_DrawPatch(cx, cy, hu_font[c as usize]);
                cx += w;
            }
        }
        count -= 1;
    }
}
#[no_mangle]
pub static mut castorder: [castinfo_t; 18] = [
    castinfo_t {
        name: Some(CC_ZOMBIE),
        type_0: MT_POSSESSED,
    },
    castinfo_t {
        name: Some(CC_SHOTGUN),
        type_0: MT_SHOTGUY,
    },
    castinfo_t {
        name: Some(CC_HEAVY),
        type_0: MT_CHAINGUY,
    },
    castinfo_t {
        name: Some(CC_IMP),
        type_0: MT_TROOP,
    },
    castinfo_t {
        name: Some(CC_DEMON),
        type_0: MT_SERGEANT,
    },
    castinfo_t {
        name: Some(CC_LOST),
        type_0: MT_SKULL,
    },
    castinfo_t {
        name: Some(CC_CACO),
        type_0: MT_HEAD,
    },
    castinfo_t {
        name: Some(CC_HELL),
        type_0: MT_KNIGHT,
    },
    castinfo_t {
        name: Some(CC_BARON),
        type_0: MT_BRUISER,
    },
    castinfo_t {
        name: Some(CC_ARACH),
        type_0: MT_BABY,
    },
    castinfo_t {
        name: Some(CC_PAIN),
        type_0: MT_PAIN,
    },
    castinfo_t {
        name: Some(CC_REVEN),
        type_0: MT_UNDEAD,
    },
    castinfo_t {
        name: Some(CC_MANCU),
        type_0: MT_FATSO,
    },
    castinfo_t {
        name: Some(CC_ARCH),
        type_0: MT_VILE,
    },
    castinfo_t {
        name: Some(CC_SPIDER),
        type_0: MT_SPIDER,
    },
    castinfo_t {
        name: Some(CC_CYBER),
        type_0: MT_CYBORG,
    },
    castinfo_t {
        name: Some(CC_HERO),
        type_0: MT_PLAYER,
    },
    castinfo_t {
        name: None,
        type_0: MT_PLAYER,
    },
];
#[no_mangle]
pub static mut castnum: i32 = 0;
#[no_mangle]
pub static mut casttics: i32 = 0;
#[no_mangle]
pub static mut caststate: *mut state_t = ::core::ptr::null::<state_t>() as *mut state_t;
#[no_mangle]
pub static mut castdeath: bool = false;
#[no_mangle]
pub static mut castframes: i32 = 0;
#[no_mangle]
pub static mut castonmelee: i32 = 0;
#[no_mangle]
pub static mut castattacking: bool = false;
pub unsafe fn F_StartCast() {
    wipegamestate = 4294967295 as gamestate_t;
    castnum = 0 as i32;
    caststate = (&raw mut states as *mut state_t)
        .offset(
            (*(&raw mut mobjinfo as *mut mobjinfo_t)
                .offset(
                    (*(&raw mut castorder as *mut castinfo_t).offset(castnum as isize))
                        .type_0 as isize,
                ))
                .seestate as isize,
        ) as *mut state_t;
    casttics = (*caststate).tics;
    castdeath = false;
    finalestage = F_STAGE_CAST;
    castframes = 0 as i32;
    castonmelee = 0 as i32;
    castattacking = false;
    S_ChangeMusic(mus_evil as i32, true_0);
}
pub unsafe fn F_CastTicker() {
    let mut current_block: u64;
    let mut st: i32 = 0;
    let mut sfx: i32 = 0;
    casttics -= 1;
    if casttics > 0 as i32 {
        return;
    }
    if (*caststate).tics == -(1 as i32)
        || (*caststate).nextstate as u32
            == S_NULL as i32 as u32
    {
        castnum += 1;
        castdeath = false;
        if castorder[castnum as usize].name.is_none() {
            castnum = 0 as i32;
        }
        if mobjinfo[castorder[castnum as usize].type_0 as usize].seesound != 0 {
            S_StartSound(
                NULL,
                mobjinfo[castorder[castnum as usize].type_0 as usize].seesound,
            );
        }
        caststate = (&raw mut states as *mut state_t)
            .offset(
                (*(&raw mut mobjinfo as *mut mobjinfo_t)
                    .offset(
                        (*(&raw mut castorder as *mut castinfo_t)
                            .offset(castnum as isize))
                            .type_0 as isize,
                    ))
                    .seestate as isize,
            ) as *mut state_t;
        castframes = 0 as i32;
        current_block = 1356832168064818221;
    } else if caststate
        == (&raw mut states as *mut state_t)
            .offset(S_PLAY_ATK1 as i32 as isize) as *mut state_t
    {
        current_block = 13354568087807251156;
    } else {
        st = (*caststate).nextstate as i32;
        caststate = (&raw mut states as *mut state_t).offset(st as isize)
            as *mut state_t;
        castframes += 1;
        match st {
            154 => {
                sfx = sfx_dshtgn as i32;
            }
            185 => {
                sfx = sfx_pistol as i32;
            }
            218 => {
                sfx = sfx_shotgn as i32;
            }
            256 => {
                sfx = sfx_vilatk as i32;
            }
            336 => {
                sfx = sfx_skeswg as i32;
            }
            338 => {
                sfx = sfx_skepch as i32;
            }
            340 => {
                sfx = sfx_skeatk as i32;
            }
            383 | 380 | 377 => {
                sfx = sfx_firsht as i32;
            }
            417 | 418 | 419 => {
                sfx = sfx_shotgn as i32;
            }
            454 => {
                sfx = sfx_claw as i32;
            }
            486 => {
                sfx = sfx_sgtatk as i32;
            }
            538 | 567 | 505 => {
                sfx = sfx_firsht as i32;
            }
            590 => {
                sfx = sfx_sklatk as i32;
            }
            616 | 617 => {
                sfx = sfx_shotgn as i32;
            }
            648 => {
                sfx = sfx_plasma as i32;
            }
            685 | 687 | 689 => {
                sfx = sfx_rlaunc as i32;
            }
            710 => {
                sfx = sfx_sklatk as i32;
            }
            _ => {
                sfx = 0 as i32;
            }
        }
        if sfx != 0 {
            S_StartSound(NULL, sfx);
        }
        current_block = 1356832168064818221;
    }
    match current_block {
        1356832168064818221 => {
            if castframes == 12 as i32 {
                castattacking = true;
                if castonmelee != 0 {
                    caststate = (&raw mut states as *mut state_t)
                        .offset(
                            (*(&raw mut mobjinfo as *mut mobjinfo_t)
                                .offset(
                                    (*(&raw mut castorder as *mut castinfo_t)
                                        .offset(castnum as isize))
                                        .type_0 as isize,
                                ))
                                .meleestate as isize,
                        ) as *mut state_t;
                } else {
                    caststate = (&raw mut states as *mut state_t)
                        .offset(
                            (*(&raw mut mobjinfo as *mut mobjinfo_t)
                                .offset(
                                    (*(&raw mut castorder as *mut castinfo_t)
                                        .offset(castnum as isize))
                                        .type_0 as isize,
                                ))
                                .missilestate as isize,
                        ) as *mut state_t;
                }
                castonmelee ^= 1 as i32;
                if caststate
                    == (&raw mut states as *mut state_t)
                        .offset(S_NULL as i32 as isize) as *mut state_t
                {
                    if castonmelee != 0 {
                        caststate = (&raw mut states as *mut state_t)
                            .offset(
                                (*(&raw mut mobjinfo as *mut mobjinfo_t)
                                    .offset(
                                        (*(&raw mut castorder as *mut castinfo_t)
                                            .offset(castnum as isize))
                                            .type_0 as isize,
                                    ))
                                    .meleestate as isize,
                            ) as *mut state_t;
                    } else {
                        caststate = (&raw mut states as *mut state_t)
                            .offset(
                                (*(&raw mut mobjinfo as *mut mobjinfo_t)
                                    .offset(
                                        (*(&raw mut castorder as *mut castinfo_t)
                                            .offset(castnum as isize))
                                            .type_0 as isize,
                                    ))
                                    .missilestate as isize,
                            ) as *mut state_t;
                    }
                }
            }
            if castattacking {
                if castframes == 24 as i32
                    || caststate
                        == (&raw mut states as *mut state_t)
                            .offset(
                                (*(&raw mut mobjinfo as *mut mobjinfo_t)
                                    .offset(
                                        (*(&raw mut castorder as *mut castinfo_t)
                                            .offset(castnum as isize))
                                            .type_0 as isize,
                                    ))
                                    .seestate as isize,
                            ) as *mut state_t
                {
                    current_block = 13354568087807251156;
                } else {
                    current_block = 168769493162332264;
                }
            } else {
                current_block = 168769493162332264;
            }
        }
        _ => {}
    }
    match current_block {
        13354568087807251156 => {
            castattacking = false;
            castframes = 0 as i32;
            caststate = (&raw mut states as *mut state_t)
                .offset(
                    (*(&raw mut mobjinfo as *mut mobjinfo_t)
                        .offset(
                            (*(&raw mut castorder as *mut castinfo_t)
                                .offset(castnum as isize))
                                .type_0 as isize,
                        ))
                        .seestate as isize,
                ) as *mut state_t;
        }
        _ => {}
    }
    casttics = (*caststate).tics;
    if casttics == -(1 as i32) {
        casttics = 15 as i32;
    }
}
pub unsafe fn F_CastResponder(mut ev: *mut event_t) -> bool {
    if (*ev).type_0 as u32
        != ev_keydown as i32 as u32
    {
        return false;
    }
    if castdeath {
        return true;
    }
    castdeath = true;
    caststate = (&raw mut states as *mut state_t)
        .offset(
            (*(&raw mut mobjinfo as *mut mobjinfo_t)
                .offset(
                    (*(&raw mut castorder as *mut castinfo_t).offset(castnum as isize))
                        .type_0 as isize,
                ))
                .deathstate as isize,
        ) as *mut state_t;
    casttics = (*caststate).tics;
    castframes = 0 as i32;
    castattacking = false;
    if mobjinfo[castorder[castnum as usize].type_0 as usize].deathsound != 0 {
        S_StartSound(
            NULL,
            mobjinfo[castorder[castnum as usize].type_0 as usize].deathsound,
        );
    }
    return true;
}
pub unsafe fn F_CastPrint(text: &str) {
    let mut c: i32 = 0;
    let mut cx: i32 = 0;
    let mut w: i32 = 0;
    let mut width: i32 = 0;
    for b in text.bytes() {
        c = toupper(b as i32) - HU_FONTSTART;
        if c < 0 as i32 || c > HU_FONTSIZE {
            width += 4 as i32;
        } else {
            w = (*hu_font[c as usize]).width as i32;
            width += w;
        }
    }
    cx = 160 as i32 - width / 2 as i32;
    for b in text.bytes() {
        c = toupper(b as i32) - HU_FONTSTART;
        if c < 0 as i32 || c > HU_FONTSIZE {
            cx += 4 as i32;
        } else {
            w = (*hu_font[c as usize]).width as i32;
            V_DrawPatch(cx, 180 as i32, hu_font[c as usize]);
            cx += w;
        }
    }
}
pub unsafe fn F_CastDrawer() {
    let mut sprdef: *mut spritedef_t = ::core::ptr::null_mut::<spritedef_t>();
    let mut sprframe: *mut spriteframe_t = ::core::ptr::null_mut::<spriteframe_t>();
    let mut lump: i32 = 0;
    let mut flip: bool = false;
    let mut patch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    V_DrawPatch(
        0 as i32,
        0 as i32,
        W_CacheLumpName("BOSSBACK",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    F_CastPrint(castorder[castnum as usize].name.unwrap());
    sprdef = sprites.offset((*caststate).sprite as isize) as *mut spritedef_t;
    sprframe = (*sprdef)
        .spriteframes
        .offset(((*caststate).frame & FF_FRAMEMASK) as isize) as *mut spriteframe_t;
    lump = (*sprframe).lump[0 as i32 as usize] as i32;
    flip = (*sprframe).flip[0 as i32 as usize] != 0;
    patch = W_CacheLumpNum(lump + firstspritelump, PU_CACHE as i32)
        as *mut patch_t;
    if flip {
        V_DrawPatchFlipped(160 as i32, 170 as i32, patch);
    } else {
        V_DrawPatch(160 as i32, 170 as i32, patch);
    };
}
pub unsafe fn F_DrawPatchCol(
    mut x: i32,
    mut patch: *mut patch_t,
    mut col: i32,
) {
    let mut column: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut dest: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut desttop: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut count: i32 = 0;
    column = (patch as *mut byte)
        .offset(
            *(&raw const (*patch).columnofs as *const i32)
                .offset(col as isize) as isize,
        ) as *mut column_t;
    desttop = I_VideoBuffer.offset(x as isize);
    while (*column).topdelta as i32 != 0xff as i32 {
        source = (column as *mut byte).offset(3 as i32 as isize);
        dest = desttop
            .offset(((*column).topdelta as i32 * SCREENWIDTH) as isize);
        count = (*column).length as i32;
        loop {
            let fresh3 = count;
            count = count - 1;
            if !(fresh3 != 0) {
                break;
            }
            let fresh4 = source;
            source = source.offset(1);
            *dest = *fresh4;
            dest = dest.offset(SCREENWIDTH as isize);
        }
        column = (column as *mut byte)
            .offset((*column).length as i32 as isize)
            .offset(4 as i32 as isize) as *mut column_t;
    }
}
pub unsafe fn F_BunnyScroll() {
    let mut scrolled: i32 = 0;
    let mut x: i32 = 0;
    let mut p1: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    let mut p2: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    let mut name: [::core::ffi::c_char; 10] = [0; 10];
    let mut stage: i32 = 0;
    static mut laststage: i32 = 0;
    p1 = W_CacheLumpName("PFUB2",
        PU_LEVEL as i32,
    ) as *mut patch_t;
    p2 = W_CacheLumpName("PFUB1",
        PU_LEVEL as i32,
    ) as *mut patch_t;
    V_MarkRect(
        0 as i32,
        0 as i32,
        SCREENWIDTH,
        SCREENHEIGHT,
    );
    scrolled = 320 as i32
        - (finalecount as i32 - 230 as i32)
            / 2 as i32;
    if scrolled > 320 as i32 {
        scrolled = 320 as i32;
    }
    if scrolled < 0 as i32 {
        scrolled = 0 as i32;
    }
    x = 0 as i32;
    while x < SCREENWIDTH {
        if x + scrolled < 320 as i32 {
            F_DrawPatchCol(x, p1, x + scrolled);
        } else {
            F_DrawPatchCol(x, p2, x + scrolled - 320 as i32);
        }
        x += 1;
    }
    if finalecount < 1130 as u32 {
        return;
    }
    if finalecount < 1180 as u32 {
        V_DrawPatch(
            (SCREENWIDTH - 13 as i32 * 8 as i32)
                / 2 as i32,
            (SCREENHEIGHT - 8 as i32 * 8 as i32)
                / 2 as i32,
            W_CacheLumpName("END0",
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
        laststage = 0 as i32;
        return;
    }
    stage = finalecount
        .wrapping_sub(1180 as u32)
        .wrapping_div(5 as u32) as i32;
    if stage > 6 as i32 {
        stage = 6 as i32;
    }
    if stage > laststage {
        S_StartSound(NULL, sfx_pistol as i32);
        laststage = stage;
    }
    snprintf(
        &raw mut name as *mut ::core::ffi::c_char,
        10 as size_t,
        b"END%i\0" as *const u8 as *const ::core::ffi::c_char,
        stage,
    );
    V_DrawPatch(
        (SCREENWIDTH - 13 as i32 * 8 as i32)
            / 2 as i32,
        (SCREENHEIGHT - 8 as i32 * 8 as i32)
            / 2 as i32,
        W_CacheLumpName(
            &wad_name8_to_string(&raw mut name as *mut ::core::ffi::c_char),
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
unsafe fn F_ArtScreenDrawer() {
    let mut lumpname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if gameepisode == 3 as i32 {
        F_BunnyScroll();
    } else {
        match gameepisode {
            1 => {
                if gamemode as u32
                    == retail as i32 as u32
                {
                    lumpname = b"CREDIT\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else {
                    lumpname = b"HELP2\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
            }
            2 => {
                lumpname = b"VICTORY2\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            4 => {
                lumpname = b"ENDPIC\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            _ => return,
        }
        lumpname = lumpname;
        V_DrawPatch(
            0 as i32,
            0 as i32,
            W_CacheLumpName(
                &wad_name8_to_string(lumpname),
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
    };
}
pub unsafe fn F_Drawer() {
    match finalestage as u32 {
        2 => {
            F_CastDrawer();
        }
        0 => {
            F_TextWrite();
        }
        1 => {
            F_ArtScreenDrawer();
        }
        _ => {}
    };
}
