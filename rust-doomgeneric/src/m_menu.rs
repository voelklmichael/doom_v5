use crate::src::i_system::FILE;
use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::i_system::I_Error;
use crate::src::dstrings::{doom1_endmsg, doom2_endmsg};
use crate::src::w_wad::{wad_name8_to_string, W_CacheLumpName};
use crate::src::i_timer::I_WaitVBL;
use crate::src::d_main::D_StartTitle;
use crate::src::i_input::vanilla_keyboard_mapping;
use crate::src::i_video::usegamma;
use crate::src::r_main::R_SetViewSize;
use crate::src::g_game::G_SaveGame;
use crate::src::g_game::G_ScreenShot;
use crate::src::m_controls::key_menu_activate;
use crate::src::m_controls::key_menu_up;
use crate::src::m_controls::key_menu_down;
use crate::src::m_controls::key_menu_left;
use crate::src::m_controls::key_menu_right;
use crate::src::m_controls::key_menu_back;
use crate::src::m_controls::key_menu_forward;
use crate::src::m_controls::key_menu_confirm;
use crate::src::m_controls::key_menu_abort;
use crate::src::m_controls::key_menu_help;
use crate::src::m_controls::key_menu_save;
use crate::src::m_controls::key_menu_load;
use crate::src::m_controls::key_menu_volume;
use crate::src::m_controls::key_menu_detail;
use crate::src::m_controls::key_menu_qsave;
use crate::src::m_controls::key_menu_endgame;
use crate::src::m_controls::key_menu_messages;
use crate::src::m_controls::key_menu_qload;
use crate::src::m_controls::key_menu_quit;
use crate::src::m_controls::key_menu_gamma;
use crate::src::m_controls::key_menu_incscreen;
use crate::src::m_controls::key_menu_decscreen;
use crate::src::m_controls::key_menu_screenshot;
use crate::src::m_controls::joybmenu;
use crate::src::s_sound::S_SetMusicVolume;
use crate::src::s_sound::S_SetSfxVolume;
use crate::src::d_main::devparm;
use crate::src::hu_stuff::message_dontfuckwithme;
use crate::src::hu_stuff::chat_on;
use crate::src::g_game::G_LoadGame;
use crate::src::g_game::G_DeferedInitNew;
use crate::src::g_game::usergame;
use crate::src::g_game::testcontrols;
use crate::src::hu_stuff::hu_font;
use crate::src::i_system::I_Quit;
use crate::src::s_sound::sfxVolume;
use crate::src::s_sound::musicVolume;
use crate::src::g_game::gamestate;
use crate::src::i_video::I_SetPalette;
use crate::src::p_saveg::P_SaveGameFile;
use crate::src::v_video::V_DrawPatchDirect;
use crate::src::d_loop::gametic;
use crate::src::g_game::demoplayback;
use crate::src::doomstat::gamemission;
use crate::src::am_map::automapactive;
use crate::src::m_misc::M_StringCopy;
use crate::src::doomstat::gameversion;
use crate::src::g_game::netgame;
use crate::src::g_game::consoleplayer;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::i_timer::I_GetTime;
use crate::src::z_zone::PU_CACHE;
use crate::src::sounds::{sfx_boscub, sfx_bspact, sfx_dmpain, sfx_getpow, sfx_kntdth, sfx_oof, sfx_pistol, sfx_pldeth, sfx_popain, sfx_posit1, sfx_posit3, sfx_pstop, sfx_sgtatk, sfx_skeswg, sfx_slop, sfx_stnmov, sfx_swtchn, sfx_swtchx, sfx_telept, sfx_vilact};
use libc::toupper;
use libc::snprintf;
use crate::src::m_misc::__ctype_toupper_loc;
use crate::src::i_system::{fclose, fopen, fprintf, fread, stderr};
use crate::src::p_mobj::mobjtype_t;
use crate::src::p_mobj::statenum_t;
use crate::src::d_mode::{commercial, registered, retail, shareware};
use crate::src::d_mode::{exe_chex, exe_doom_1_9, exe_ultimate};
use crate::src::d_mode::{doom, doom2, pack_chex, pack_hacx};
use crate::src::d_mode::skill_t;
use crate::src::d_event::{ev_joystick, ev_keydown, ev_mouse, ev_quit};
use crate::src::d_event::GS_LEVEL;

pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub type weapontype_t = u32;
pub const wp_nochange: weapontype_t = 10;
pub const NUMWEAPONS: weapontype_t = 9;
pub const wp_supershotgun: weapontype_t = 8;
pub const wp_chainsaw: weapontype_t = 7;
pub const wp_bfg: weapontype_t = 6;
pub const wp_plasma: weapontype_t = 5;
pub const wp_missile: weapontype_t = 4;
pub const wp_chaingun: weapontype_t = 3;
pub const wp_shotgun: weapontype_t = 2;
pub const wp_pistol: weapontype_t = 1;
pub const wp_fist: weapontype_t = 0;
pub type fixed_t = i32;
pub type angle_t = u32;
pub const NUMSTATES: statenum_t = 967;
pub const S_TECH2LAMP4: statenum_t = 966;
pub const S_TECH2LAMP3: statenum_t = 965;
pub const S_TECH2LAMP2: statenum_t = 964;
pub const S_TECH2LAMP: statenum_t = 963;
pub const S_TECHLAMP4: statenum_t = 962;
pub const S_TECHLAMP3: statenum_t = 961;
pub const S_TECHLAMP2: statenum_t = 960;
pub const S_TECHLAMP: statenum_t = 959;
pub const S_BRAINSTEM: statenum_t = 958;
pub const S_SMALLPOOL: statenum_t = 957;
pub const S_COLONGIBS: statenum_t = 956;
pub const S_HANGTNOBRAIN: statenum_t = 955;
pub const S_HANGTLOOKUP: statenum_t = 954;
pub const S_HANGTSKULL: statenum_t = 953;
pub const S_HANGTLOOKDN: statenum_t = 952;
pub const S_HANGBNOBRAIN: statenum_t = 951;
pub const S_HANGNOGUTS: statenum_t = 950;
pub const S_RTORCHSHRT4: statenum_t = 949;
pub const S_RTORCHSHRT3: statenum_t = 948;
pub const S_RTORCHSHRT2: statenum_t = 947;
pub const S_RTORCHSHRT: statenum_t = 946;
pub const S_GTORCHSHRT4: statenum_t = 945;
pub const S_GTORCHSHRT3: statenum_t = 944;
pub const S_GTORCHSHRT2: statenum_t = 943;
pub const S_GTORCHSHRT: statenum_t = 942;
pub const S_BTORCHSHRT4: statenum_t = 941;
pub const S_BTORCHSHRT3: statenum_t = 940;
pub const S_BTORCHSHRT2: statenum_t = 939;
pub const S_BTORCHSHRT: statenum_t = 938;
pub const S_REDTORCH4: statenum_t = 937;
pub const S_REDTORCH3: statenum_t = 936;
pub const S_REDTORCH2: statenum_t = 935;
pub const S_REDTORCH: statenum_t = 934;
pub const S_GREENTORCH4: statenum_t = 933;
pub const S_GREENTORCH3: statenum_t = 932;
pub const S_GREENTORCH2: statenum_t = 931;
pub const S_GREENTORCH: statenum_t = 930;
pub const S_BLUETORCH4: statenum_t = 929;
pub const S_BLUETORCH3: statenum_t = 928;
pub const S_BLUETORCH2: statenum_t = 927;
pub const S_BLUETORCH: statenum_t = 926;
pub const S_HEARTCOL2: statenum_t = 925;
pub const S_HEARTCOL: statenum_t = 924;
pub const S_FLOATSKULL3: statenum_t = 923;
pub const S_FLOATSKULL2: statenum_t = 922;
pub const S_FLOATSKULL: statenum_t = 921;
pub const S_EVILEYE4: statenum_t = 920;
pub const S_EVILEYE3: statenum_t = 919;
pub const S_EVILEYE2: statenum_t = 918;
pub const S_EVILEYE: statenum_t = 917;
pub const S_TECHPILLAR: statenum_t = 916;
pub const S_BIGTREE: statenum_t = 915;
pub const S_TORCHTREE: statenum_t = 914;
pub const S_SKULLCOL: statenum_t = 913;
pub const S_CANDELABRA: statenum_t = 912;
pub const S_CANDLESTIK: statenum_t = 911;
pub const S_SHRTREDCOL: statenum_t = 910;
pub const S_TALLREDCOL: statenum_t = 909;
pub const S_SHRTGRNCOL: statenum_t = 908;
pub const S_TALLGRNCOL: statenum_t = 907;
pub const S_STALAGTITE: statenum_t = 906;
pub const S_MEAT5: statenum_t = 905;
pub const S_MEAT4: statenum_t = 904;
pub const S_MEAT3: statenum_t = 903;
pub const S_MEAT2: statenum_t = 902;
pub const S_LIVESTICK2: statenum_t = 901;
pub const S_LIVESTICK: statenum_t = 900;
pub const S_DEADSTICK: statenum_t = 899;
pub const S_HEADCANDLES2: statenum_t = 898;
pub const S_HEADCANDLES: statenum_t = 897;
pub const S_HEADONASTICK: statenum_t = 896;
pub const S_GIBS: statenum_t = 895;
pub const S_HEADSONSTICK: statenum_t = 894;
pub const S_DEADBOTTOM: statenum_t = 893;
pub const S_DEADTORSO: statenum_t = 892;
pub const S_BLOODYTWITCH4: statenum_t = 891;
pub const S_BLOODYTWITCH3: statenum_t = 890;
pub const S_BLOODYTWITCH2: statenum_t = 889;
pub const S_BLOODYTWITCH: statenum_t = 888;
pub const S_STALAG: statenum_t = 887;
pub const S_COLU: statenum_t = 886;
pub const S_SHOT2: statenum_t = 885;
pub const S_SHOT: statenum_t = 884;
pub const S_PLAS: statenum_t = 883;
pub const S_LAUN: statenum_t = 882;
pub const S_CSAW: statenum_t = 881;
pub const S_MGUN: statenum_t = 880;
pub const S_BFUG: statenum_t = 879;
pub const S_BPAK: statenum_t = 878;
pub const S_SBOX: statenum_t = 877;
pub const S_SHEL: statenum_t = 876;
pub const S_CELP: statenum_t = 875;
pub const S_CELL: statenum_t = 874;
pub const S_BROK: statenum_t = 873;
pub const S_ROCK: statenum_t = 872;
pub const S_AMMO: statenum_t = 871;
pub const S_CLIP: statenum_t = 870;
pub const S_PVIS2: statenum_t = 869;
pub const S_PVIS: statenum_t = 868;
pub const S_PMAP6: statenum_t = 867;
pub const S_PMAP5: statenum_t = 866;
pub const S_PMAP4: statenum_t = 865;
pub const S_PMAP3: statenum_t = 864;
pub const S_PMAP2: statenum_t = 863;
pub const S_PMAP: statenum_t = 862;
pub const S_SUIT: statenum_t = 861;
pub const S_MEGA4: statenum_t = 860;
pub const S_MEGA3: statenum_t = 859;
pub const S_MEGA2: statenum_t = 858;
pub const S_MEGA: statenum_t = 857;
pub const S_PINS4: statenum_t = 856;
pub const S_PINS3: statenum_t = 855;
pub const S_PINS2: statenum_t = 854;
pub const S_PINS: statenum_t = 853;
pub const S_PSTR: statenum_t = 852;
pub const S_PINV4: statenum_t = 851;
pub const S_PINV3: statenum_t = 850;
pub const S_PINV2: statenum_t = 849;
pub const S_PINV: statenum_t = 848;
pub const S_SOUL6: statenum_t = 847;
pub const S_SOUL5: statenum_t = 846;
pub const S_SOUL4: statenum_t = 845;
pub const S_SOUL3: statenum_t = 844;
pub const S_SOUL2: statenum_t = 843;
pub const S_SOUL: statenum_t = 842;
pub const S_MEDI: statenum_t = 841;
pub const S_STIM: statenum_t = 840;
pub const S_YSKULL2: statenum_t = 839;
pub const S_YSKULL: statenum_t = 838;
pub const S_RSKULL2: statenum_t = 837;
pub const S_RSKULL: statenum_t = 836;
pub const S_BSKULL2: statenum_t = 835;
pub const S_BSKULL: statenum_t = 834;
pub const S_YKEY2: statenum_t = 833;
pub const S_YKEY: statenum_t = 832;
pub const S_RKEY2: statenum_t = 831;
pub const S_RKEY: statenum_t = 830;
pub const S_BKEY2: statenum_t = 829;
pub const S_BKEY: statenum_t = 828;
pub const S_BON2E: statenum_t = 827;
pub const S_BON2D: statenum_t = 826;
pub const S_BON2C: statenum_t = 825;
pub const S_BON2B: statenum_t = 824;
pub const S_BON2A: statenum_t = 823;
pub const S_BON2: statenum_t = 822;
pub const S_BON1E: statenum_t = 821;
pub const S_BON1D: statenum_t = 820;
pub const S_BON1C: statenum_t = 819;
pub const S_BON1B: statenum_t = 818;
pub const S_BON1A: statenum_t = 817;
pub const S_BON1: statenum_t = 816;
pub const S_BBAR3: statenum_t = 815;
pub const S_BBAR2: statenum_t = 814;
pub const S_BBAR1: statenum_t = 813;
pub const S_BEXP5: statenum_t = 812;
pub const S_BEXP4: statenum_t = 811;
pub const S_BEXP3: statenum_t = 810;
pub const S_BEXP2: statenum_t = 809;
pub const S_BEXP: statenum_t = 808;
pub const S_BAR2: statenum_t = 807;
pub const S_BAR1: statenum_t = 806;
pub const S_ARM2A: statenum_t = 805;
pub const S_ARM2: statenum_t = 804;
pub const S_ARM1A: statenum_t = 803;
pub const S_ARM1: statenum_t = 802;
pub const S_BRAINEXPLODE3: statenum_t = 801;
pub const S_BRAINEXPLODE2: statenum_t = 800;
pub const S_BRAINEXPLODE1: statenum_t = 799;
pub const S_SPAWNFIRE8: statenum_t = 798;
pub const S_SPAWNFIRE7: statenum_t = 797;
pub const S_SPAWNFIRE6: statenum_t = 796;
pub const S_SPAWNFIRE5: statenum_t = 795;
pub const S_SPAWNFIRE4: statenum_t = 794;
pub const S_SPAWNFIRE3: statenum_t = 793;
pub const S_SPAWNFIRE2: statenum_t = 792;
pub const S_SPAWNFIRE1: statenum_t = 791;
pub const S_SPAWN4: statenum_t = 790;
pub const S_SPAWN3: statenum_t = 789;
pub const S_SPAWN2: statenum_t = 788;
pub const S_SPAWN1: statenum_t = 787;
pub const S_BRAINEYE1: statenum_t = 786;
pub const S_BRAINEYESEE: statenum_t = 785;
pub const S_BRAINEYE: statenum_t = 784;
pub const S_BRAIN_DIE4: statenum_t = 783;
pub const S_BRAIN_DIE3: statenum_t = 782;
pub const S_BRAIN_DIE2: statenum_t = 781;
pub const S_BRAIN_DIE1: statenum_t = 780;
pub const S_BRAIN_PAIN: statenum_t = 779;
pub const S_BRAIN: statenum_t = 778;
pub const S_KEENPAIN2: statenum_t = 777;
pub const S_KEENPAIN: statenum_t = 776;
pub const S_COMMKEEN12: statenum_t = 775;
pub const S_COMMKEEN11: statenum_t = 774;
pub const S_COMMKEEN10: statenum_t = 773;
pub const S_COMMKEEN9: statenum_t = 772;
pub const S_COMMKEEN8: statenum_t = 771;
pub const S_COMMKEEN7: statenum_t = 770;
pub const S_COMMKEEN6: statenum_t = 769;
pub const S_COMMKEEN5: statenum_t = 768;
pub const S_COMMKEEN4: statenum_t = 767;
pub const S_COMMKEEN3: statenum_t = 766;
pub const S_COMMKEEN2: statenum_t = 765;
pub const S_COMMKEEN: statenum_t = 764;
pub const S_KEENSTND: statenum_t = 763;
pub const S_SSWV_RAISE5: statenum_t = 762;
pub const S_SSWV_RAISE4: statenum_t = 761;
pub const S_SSWV_RAISE3: statenum_t = 760;
pub const S_SSWV_RAISE2: statenum_t = 759;
pub const S_SSWV_RAISE1: statenum_t = 758;
pub const S_SSWV_XDIE9: statenum_t = 757;
pub const S_SSWV_XDIE8: statenum_t = 756;
pub const S_SSWV_XDIE7: statenum_t = 755;
pub const S_SSWV_XDIE6: statenum_t = 754;
pub const S_SSWV_XDIE5: statenum_t = 753;
pub const S_SSWV_XDIE4: statenum_t = 752;
pub const S_SSWV_XDIE3: statenum_t = 751;
pub const S_SSWV_XDIE2: statenum_t = 750;
pub const S_SSWV_XDIE1: statenum_t = 749;
pub const S_SSWV_DIE5: statenum_t = 748;
pub const S_SSWV_DIE4: statenum_t = 747;
pub const S_SSWV_DIE3: statenum_t = 746;
pub const S_SSWV_DIE2: statenum_t = 745;
pub const S_SSWV_DIE1: statenum_t = 744;
pub const S_SSWV_PAIN2: statenum_t = 743;
pub const S_SSWV_PAIN: statenum_t = 742;
pub const S_SSWV_ATK6: statenum_t = 741;
pub const S_SSWV_ATK5: statenum_t = 740;
pub const S_SSWV_ATK4: statenum_t = 739;
pub const S_SSWV_ATK3: statenum_t = 738;
pub const S_SSWV_ATK2: statenum_t = 737;
pub const S_SSWV_ATK1: statenum_t = 736;
pub const S_SSWV_RUN8: statenum_t = 735;
pub const S_SSWV_RUN7: statenum_t = 734;
pub const S_SSWV_RUN6: statenum_t = 733;
pub const S_SSWV_RUN5: statenum_t = 732;
pub const S_SSWV_RUN4: statenum_t = 731;
pub const S_SSWV_RUN3: statenum_t = 730;
pub const S_SSWV_RUN2: statenum_t = 729;
pub const S_SSWV_RUN1: statenum_t = 728;
pub const S_SSWV_STND2: statenum_t = 727;
pub const S_SSWV_STND: statenum_t = 726;
pub const S_PAIN_RAISE6: statenum_t = 725;
pub const S_PAIN_RAISE5: statenum_t = 724;
pub const S_PAIN_RAISE4: statenum_t = 723;
pub const S_PAIN_RAISE3: statenum_t = 722;
pub const S_PAIN_RAISE2: statenum_t = 721;
pub const S_PAIN_RAISE1: statenum_t = 720;
pub const S_PAIN_DIE6: statenum_t = 719;
pub const S_PAIN_DIE5: statenum_t = 718;
pub const S_PAIN_DIE4: statenum_t = 717;
pub const S_PAIN_DIE3: statenum_t = 716;
pub const S_PAIN_DIE2: statenum_t = 715;
pub const S_PAIN_DIE1: statenum_t = 714;
pub const S_PAIN_PAIN2: statenum_t = 713;
pub const S_PAIN_PAIN: statenum_t = 712;
pub const S_PAIN_ATK4: statenum_t = 711;
pub const S_PAIN_ATK3: statenum_t = 710;
pub const S_PAIN_ATK2: statenum_t = 709;
pub const S_PAIN_ATK1: statenum_t = 708;
pub const S_PAIN_RUN6: statenum_t = 707;
pub const S_PAIN_RUN5: statenum_t = 706;
pub const S_PAIN_RUN4: statenum_t = 705;
pub const S_PAIN_RUN3: statenum_t = 704;
pub const S_PAIN_RUN2: statenum_t = 703;
pub const S_PAIN_RUN1: statenum_t = 702;
pub const S_PAIN_STND: statenum_t = 701;
pub const S_CYBER_DIE10: statenum_t = 700;
pub const S_CYBER_DIE9: statenum_t = 699;
pub const S_CYBER_DIE8: statenum_t = 698;
pub const S_CYBER_DIE7: statenum_t = 697;
pub const S_CYBER_DIE6: statenum_t = 696;
pub const S_CYBER_DIE5: statenum_t = 695;
pub const S_CYBER_DIE4: statenum_t = 694;
pub const S_CYBER_DIE3: statenum_t = 693;
pub const S_CYBER_DIE2: statenum_t = 692;
pub const S_CYBER_DIE1: statenum_t = 691;
pub const S_CYBER_PAIN: statenum_t = 690;
pub const S_CYBER_ATK6: statenum_t = 689;
pub const S_CYBER_ATK5: statenum_t = 688;
pub const S_CYBER_ATK4: statenum_t = 687;
pub const S_CYBER_ATK3: statenum_t = 686;
pub const S_CYBER_ATK2: statenum_t = 685;
pub const S_CYBER_ATK1: statenum_t = 684;
pub const S_CYBER_RUN8: statenum_t = 683;
pub const S_CYBER_RUN7: statenum_t = 682;
pub const S_CYBER_RUN6: statenum_t = 681;
pub const S_CYBER_RUN5: statenum_t = 680;
pub const S_CYBER_RUN4: statenum_t = 679;
pub const S_CYBER_RUN3: statenum_t = 678;
pub const S_CYBER_RUN2: statenum_t = 677;
pub const S_CYBER_RUN1: statenum_t = 676;
pub const S_CYBER_STND2: statenum_t = 675;
pub const S_CYBER_STND: statenum_t = 674;
pub const S_ARACH_PLEX5: statenum_t = 673;
pub const S_ARACH_PLEX4: statenum_t = 672;
pub const S_ARACH_PLEX3: statenum_t = 671;
pub const S_ARACH_PLEX2: statenum_t = 670;
pub const S_ARACH_PLEX: statenum_t = 669;
pub const S_ARACH_PLAZ2: statenum_t = 668;
pub const S_ARACH_PLAZ: statenum_t = 667;
pub const S_BSPI_RAISE7: statenum_t = 666;
pub const S_BSPI_RAISE6: statenum_t = 665;
pub const S_BSPI_RAISE5: statenum_t = 664;
pub const S_BSPI_RAISE4: statenum_t = 663;
pub const S_BSPI_RAISE3: statenum_t = 662;
pub const S_BSPI_RAISE2: statenum_t = 661;
pub const S_BSPI_RAISE1: statenum_t = 660;
pub const S_BSPI_DIE7: statenum_t = 659;
pub const S_BSPI_DIE6: statenum_t = 658;
pub const S_BSPI_DIE5: statenum_t = 657;
pub const S_BSPI_DIE4: statenum_t = 656;
pub const S_BSPI_DIE3: statenum_t = 655;
pub const S_BSPI_DIE2: statenum_t = 654;
pub const S_BSPI_DIE1: statenum_t = 653;
pub const S_BSPI_PAIN2: statenum_t = 652;
pub const S_BSPI_PAIN: statenum_t = 651;
pub const S_BSPI_ATK4: statenum_t = 650;
pub const S_BSPI_ATK3: statenum_t = 649;
pub const S_BSPI_ATK2: statenum_t = 648;
pub const S_BSPI_ATK1: statenum_t = 647;
pub const S_BSPI_RUN12: statenum_t = 646;
pub const S_BSPI_RUN11: statenum_t = 645;
pub const S_BSPI_RUN10: statenum_t = 644;
pub const S_BSPI_RUN9: statenum_t = 643;
pub const S_BSPI_RUN8: statenum_t = 642;
pub const S_BSPI_RUN7: statenum_t = 641;
pub const S_BSPI_RUN6: statenum_t = 640;
pub const S_BSPI_RUN5: statenum_t = 639;
pub const S_BSPI_RUN4: statenum_t = 638;
pub const S_BSPI_RUN3: statenum_t = 637;
pub const S_BSPI_RUN2: statenum_t = 636;
pub const S_BSPI_RUN1: statenum_t = 635;
pub const S_BSPI_SIGHT: statenum_t = 634;
pub const S_BSPI_STND2: statenum_t = 633;
pub const S_BSPI_STND: statenum_t = 632;
pub const S_SPID_DIE11: statenum_t = 631;
pub const S_SPID_DIE10: statenum_t = 630;
pub const S_SPID_DIE9: statenum_t = 629;
pub const S_SPID_DIE8: statenum_t = 628;
pub const S_SPID_DIE7: statenum_t = 627;
pub const S_SPID_DIE6: statenum_t = 626;
pub const S_SPID_DIE5: statenum_t = 625;
pub const S_SPID_DIE4: statenum_t = 624;
pub const S_SPID_DIE3: statenum_t = 623;
pub const S_SPID_DIE2: statenum_t = 622;
pub const S_SPID_DIE1: statenum_t = 621;
pub const S_SPID_PAIN2: statenum_t = 620;
pub const S_SPID_PAIN: statenum_t = 619;
pub const S_SPID_ATK4: statenum_t = 618;
pub const S_SPID_ATK3: statenum_t = 617;
pub const S_SPID_ATK2: statenum_t = 616;
pub const S_SPID_ATK1: statenum_t = 615;
pub const S_SPID_RUN12: statenum_t = 614;
pub const S_SPID_RUN11: statenum_t = 613;
pub const S_SPID_RUN10: statenum_t = 612;
pub const S_SPID_RUN9: statenum_t = 611;
pub const S_SPID_RUN8: statenum_t = 610;
pub const S_SPID_RUN7: statenum_t = 609;
pub const S_SPID_RUN6: statenum_t = 608;
pub const S_SPID_RUN5: statenum_t = 607;
pub const S_SPID_RUN4: statenum_t = 606;
pub const S_SPID_RUN3: statenum_t = 605;
pub const S_SPID_RUN2: statenum_t = 604;
pub const S_SPID_RUN1: statenum_t = 603;
pub const S_SPID_STND2: statenum_t = 602;
pub const S_SPID_STND: statenum_t = 601;
pub const S_SKULL_DIE6: statenum_t = 600;
pub const S_SKULL_DIE5: statenum_t = 599;
pub const S_SKULL_DIE4: statenum_t = 598;
pub const S_SKULL_DIE3: statenum_t = 597;
pub const S_SKULL_DIE2: statenum_t = 596;
pub const S_SKULL_DIE1: statenum_t = 595;
pub const S_SKULL_PAIN2: statenum_t = 594;
pub const S_SKULL_PAIN: statenum_t = 593;
pub const S_SKULL_ATK4: statenum_t = 592;
pub const S_SKULL_ATK3: statenum_t = 591;
pub const S_SKULL_ATK2: statenum_t = 590;
pub const S_SKULL_ATK1: statenum_t = 589;
pub const S_SKULL_RUN2: statenum_t = 588;
pub const S_SKULL_RUN1: statenum_t = 587;
pub const S_SKULL_STND2: statenum_t = 586;
pub const S_SKULL_STND: statenum_t = 585;
pub const S_BOS2_RAISE7: statenum_t = 584;
pub const S_BOS2_RAISE6: statenum_t = 583;
pub const S_BOS2_RAISE5: statenum_t = 582;
pub const S_BOS2_RAISE4: statenum_t = 581;
pub const S_BOS2_RAISE3: statenum_t = 580;
pub const S_BOS2_RAISE2: statenum_t = 579;
pub const S_BOS2_RAISE1: statenum_t = 578;
pub const S_BOS2_DIE7: statenum_t = 577;
pub const S_BOS2_DIE6: statenum_t = 576;
pub const S_BOS2_DIE5: statenum_t = 575;
pub const S_BOS2_DIE4: statenum_t = 574;
pub const S_BOS2_DIE3: statenum_t = 573;
pub const S_BOS2_DIE2: statenum_t = 572;
pub const S_BOS2_DIE1: statenum_t = 571;
pub const S_BOS2_PAIN2: statenum_t = 570;
pub const S_BOS2_PAIN: statenum_t = 569;
pub const S_BOS2_ATK3: statenum_t = 568;
pub const S_BOS2_ATK2: statenum_t = 567;
pub const S_BOS2_ATK1: statenum_t = 566;
pub const S_BOS2_RUN8: statenum_t = 565;
pub const S_BOS2_RUN7: statenum_t = 564;
pub const S_BOS2_RUN6: statenum_t = 563;
pub const S_BOS2_RUN5: statenum_t = 562;
pub const S_BOS2_RUN4: statenum_t = 561;
pub const S_BOS2_RUN3: statenum_t = 560;
pub const S_BOS2_RUN2: statenum_t = 559;
pub const S_BOS2_RUN1: statenum_t = 558;
pub const S_BOS2_STND2: statenum_t = 557;
pub const S_BOS2_STND: statenum_t = 556;
pub const S_BOSS_RAISE7: statenum_t = 555;
pub const S_BOSS_RAISE6: statenum_t = 554;
pub const S_BOSS_RAISE5: statenum_t = 553;
pub const S_BOSS_RAISE4: statenum_t = 552;
pub const S_BOSS_RAISE3: statenum_t = 551;
pub const S_BOSS_RAISE2: statenum_t = 550;
pub const S_BOSS_RAISE1: statenum_t = 549;
pub const S_BOSS_DIE7: statenum_t = 548;
pub const S_BOSS_DIE6: statenum_t = 547;
pub const S_BOSS_DIE5: statenum_t = 546;
pub const S_BOSS_DIE4: statenum_t = 545;
pub const S_BOSS_DIE3: statenum_t = 544;
pub const S_BOSS_DIE2: statenum_t = 543;
pub const S_BOSS_DIE1: statenum_t = 542;
pub const S_BOSS_PAIN2: statenum_t = 541;
pub const S_BOSS_PAIN: statenum_t = 540;
pub const S_BOSS_ATK3: statenum_t = 539;
pub const S_BOSS_ATK2: statenum_t = 538;
pub const S_BOSS_ATK1: statenum_t = 537;
pub const S_BOSS_RUN8: statenum_t = 536;
pub const S_BOSS_RUN7: statenum_t = 535;
pub const S_BOSS_RUN6: statenum_t = 534;
pub const S_BOSS_RUN5: statenum_t = 533;
pub const S_BOSS_RUN4: statenum_t = 532;
pub const S_BOSS_RUN3: statenum_t = 531;
pub const S_BOSS_RUN2: statenum_t = 530;
pub const S_BOSS_RUN1: statenum_t = 529;
pub const S_BOSS_STND2: statenum_t = 528;
pub const S_BOSS_STND: statenum_t = 527;
pub const S_BRBALLX3: statenum_t = 526;
pub const S_BRBALLX2: statenum_t = 525;
pub const S_BRBALLX1: statenum_t = 524;
pub const S_BRBALL2: statenum_t = 523;
pub const S_BRBALL1: statenum_t = 522;
pub const S_HEAD_RAISE6: statenum_t = 521;
pub const S_HEAD_RAISE5: statenum_t = 520;
pub const S_HEAD_RAISE4: statenum_t = 519;
pub const S_HEAD_RAISE3: statenum_t = 518;
pub const S_HEAD_RAISE2: statenum_t = 517;
pub const S_HEAD_RAISE1: statenum_t = 516;
pub const S_HEAD_DIE6: statenum_t = 515;
pub const S_HEAD_DIE5: statenum_t = 514;
pub const S_HEAD_DIE4: statenum_t = 513;
pub const S_HEAD_DIE3: statenum_t = 512;
pub const S_HEAD_DIE2: statenum_t = 511;
pub const S_HEAD_DIE1: statenum_t = 510;
pub const S_HEAD_PAIN3: statenum_t = 509;
pub const S_HEAD_PAIN2: statenum_t = 508;
pub const S_HEAD_PAIN: statenum_t = 507;
pub const S_HEAD_ATK3: statenum_t = 506;
pub const S_HEAD_ATK2: statenum_t = 505;
pub const S_HEAD_ATK1: statenum_t = 504;
pub const S_HEAD_RUN1: statenum_t = 503;
pub const S_HEAD_STND: statenum_t = 502;
pub const S_SARG_RAISE6: statenum_t = 501;
pub const S_SARG_RAISE5: statenum_t = 500;
pub const S_SARG_RAISE4: statenum_t = 499;
pub const S_SARG_RAISE3: statenum_t = 498;
pub const S_SARG_RAISE2: statenum_t = 497;
pub const S_SARG_RAISE1: statenum_t = 496;
pub const S_SARG_DIE6: statenum_t = 495;
pub const S_SARG_DIE5: statenum_t = 494;
pub const S_SARG_DIE4: statenum_t = 493;
pub const S_SARG_DIE3: statenum_t = 492;
pub const S_SARG_DIE2: statenum_t = 491;
pub const S_SARG_DIE1: statenum_t = 490;
pub const S_SARG_PAIN2: statenum_t = 489;
pub const S_SARG_PAIN: statenum_t = 488;
pub const S_SARG_ATK3: statenum_t = 487;
pub const S_SARG_ATK2: statenum_t = 486;
pub const S_SARG_ATK1: statenum_t = 485;
pub const S_SARG_RUN8: statenum_t = 484;
pub const S_SARG_RUN7: statenum_t = 483;
pub const S_SARG_RUN6: statenum_t = 482;
pub const S_SARG_RUN5: statenum_t = 481;
pub const S_SARG_RUN4: statenum_t = 480;
pub const S_SARG_RUN3: statenum_t = 479;
pub const S_SARG_RUN2: statenum_t = 478;
pub const S_SARG_RUN1: statenum_t = 477;
pub const S_SARG_STND2: statenum_t = 476;
pub const S_SARG_STND: statenum_t = 475;
pub const S_TROO_RAISE5: statenum_t = 474;
pub const S_TROO_RAISE4: statenum_t = 473;
pub const S_TROO_RAISE3: statenum_t = 472;
pub const S_TROO_RAISE2: statenum_t = 471;
pub const S_TROO_RAISE1: statenum_t = 470;
pub const S_TROO_XDIE8: statenum_t = 469;
pub const S_TROO_XDIE7: statenum_t = 468;
pub const S_TROO_XDIE6: statenum_t = 467;
pub const S_TROO_XDIE5: statenum_t = 466;
pub const S_TROO_XDIE4: statenum_t = 465;
pub const S_TROO_XDIE3: statenum_t = 464;
pub const S_TROO_XDIE2: statenum_t = 463;
pub const S_TROO_XDIE1: statenum_t = 462;
pub const S_TROO_DIE5: statenum_t = 461;
pub const S_TROO_DIE4: statenum_t = 460;
pub const S_TROO_DIE3: statenum_t = 459;
pub const S_TROO_DIE2: statenum_t = 458;
pub const S_TROO_DIE1: statenum_t = 457;
pub const S_TROO_PAIN2: statenum_t = 456;
pub const S_TROO_PAIN: statenum_t = 455;
pub const S_TROO_ATK3: statenum_t = 454;
pub const S_TROO_ATK2: statenum_t = 453;
pub const S_TROO_ATK1: statenum_t = 452;
pub const S_TROO_RUN8: statenum_t = 451;
pub const S_TROO_RUN7: statenum_t = 450;
pub const S_TROO_RUN6: statenum_t = 449;
pub const S_TROO_RUN5: statenum_t = 448;
pub const S_TROO_RUN4: statenum_t = 447;
pub const S_TROO_RUN3: statenum_t = 446;
pub const S_TROO_RUN2: statenum_t = 445;
pub const S_TROO_RUN1: statenum_t = 444;
pub const S_TROO_STND2: statenum_t = 443;
pub const S_TROO_STND: statenum_t = 442;
pub const S_CPOS_RAISE7: statenum_t = 441;
pub const S_CPOS_RAISE6: statenum_t = 440;
pub const S_CPOS_RAISE5: statenum_t = 439;
pub const S_CPOS_RAISE4: statenum_t = 438;
pub const S_CPOS_RAISE3: statenum_t = 437;
pub const S_CPOS_RAISE2: statenum_t = 436;
pub const S_CPOS_RAISE1: statenum_t = 435;
pub const S_CPOS_XDIE6: statenum_t = 434;
pub const S_CPOS_XDIE5: statenum_t = 433;
pub const S_CPOS_XDIE4: statenum_t = 432;
pub const S_CPOS_XDIE3: statenum_t = 431;
pub const S_CPOS_XDIE2: statenum_t = 430;
pub const S_CPOS_XDIE1: statenum_t = 429;
pub const S_CPOS_DIE7: statenum_t = 428;
pub const S_CPOS_DIE6: statenum_t = 427;
pub const S_CPOS_DIE5: statenum_t = 426;
pub const S_CPOS_DIE4: statenum_t = 425;
pub const S_CPOS_DIE3: statenum_t = 424;
pub const S_CPOS_DIE2: statenum_t = 423;
pub const S_CPOS_DIE1: statenum_t = 422;
pub const S_CPOS_PAIN2: statenum_t = 421;
pub const S_CPOS_PAIN: statenum_t = 420;
pub const S_CPOS_ATK4: statenum_t = 419;
pub const S_CPOS_ATK3: statenum_t = 418;
pub const S_CPOS_ATK2: statenum_t = 417;
pub const S_CPOS_ATK1: statenum_t = 416;
pub const S_CPOS_RUN8: statenum_t = 415;
pub const S_CPOS_RUN7: statenum_t = 414;
pub const S_CPOS_RUN6: statenum_t = 413;
pub const S_CPOS_RUN5: statenum_t = 412;
pub const S_CPOS_RUN4: statenum_t = 411;
pub const S_CPOS_RUN3: statenum_t = 410;
pub const S_CPOS_RUN2: statenum_t = 409;
pub const S_CPOS_RUN1: statenum_t = 408;
pub const S_CPOS_STND2: statenum_t = 407;
pub const S_CPOS_STND: statenum_t = 406;
pub const S_FATT_RAISE8: statenum_t = 405;
pub const S_FATT_RAISE7: statenum_t = 404;
pub const S_FATT_RAISE6: statenum_t = 403;
pub const S_FATT_RAISE5: statenum_t = 402;
pub const S_FATT_RAISE4: statenum_t = 401;
pub const S_FATT_RAISE3: statenum_t = 400;
pub const S_FATT_RAISE2: statenum_t = 399;
pub const S_FATT_RAISE1: statenum_t = 398;
pub const S_FATT_DIE10: statenum_t = 397;
pub const S_FATT_DIE9: statenum_t = 396;
pub const S_FATT_DIE8: statenum_t = 395;
pub const S_FATT_DIE7: statenum_t = 394;
pub const S_FATT_DIE6: statenum_t = 393;
pub const S_FATT_DIE5: statenum_t = 392;
pub const S_FATT_DIE4: statenum_t = 391;
pub const S_FATT_DIE3: statenum_t = 390;
pub const S_FATT_DIE2: statenum_t = 389;
pub const S_FATT_DIE1: statenum_t = 388;
pub const S_FATT_PAIN2: statenum_t = 387;
pub const S_FATT_PAIN: statenum_t = 386;
pub const S_FATT_ATK10: statenum_t = 385;
pub const S_FATT_ATK9: statenum_t = 384;
pub const S_FATT_ATK8: statenum_t = 383;
pub const S_FATT_ATK7: statenum_t = 382;
pub const S_FATT_ATK6: statenum_t = 381;
pub const S_FATT_ATK5: statenum_t = 380;
pub const S_FATT_ATK4: statenum_t = 379;
pub const S_FATT_ATK3: statenum_t = 378;
pub const S_FATT_ATK2: statenum_t = 377;
pub const S_FATT_ATK1: statenum_t = 376;
pub const S_FATT_RUN12: statenum_t = 375;
pub const S_FATT_RUN11: statenum_t = 374;
pub const S_FATT_RUN10: statenum_t = 373;
pub const S_FATT_RUN9: statenum_t = 372;
pub const S_FATT_RUN8: statenum_t = 371;
pub const S_FATT_RUN7: statenum_t = 370;
pub const S_FATT_RUN6: statenum_t = 369;
pub const S_FATT_RUN5: statenum_t = 368;
pub const S_FATT_RUN4: statenum_t = 367;
pub const S_FATT_RUN3: statenum_t = 366;
pub const S_FATT_RUN2: statenum_t = 365;
pub const S_FATT_RUN1: statenum_t = 364;
pub const S_FATT_STND2: statenum_t = 363;
pub const S_FATT_STND: statenum_t = 362;
pub const S_FATSHOTX3: statenum_t = 361;
pub const S_FATSHOTX2: statenum_t = 360;
pub const S_FATSHOTX1: statenum_t = 359;
pub const S_FATSHOT2: statenum_t = 358;
pub const S_FATSHOT1: statenum_t = 357;
pub const S_SKEL_RAISE6: statenum_t = 356;
pub const S_SKEL_RAISE5: statenum_t = 355;
pub const S_SKEL_RAISE4: statenum_t = 354;
pub const S_SKEL_RAISE3: statenum_t = 353;
pub const S_SKEL_RAISE2: statenum_t = 352;
pub const S_SKEL_RAISE1: statenum_t = 351;
pub const S_SKEL_DIE6: statenum_t = 350;
pub const S_SKEL_DIE5: statenum_t = 349;
pub const S_SKEL_DIE4: statenum_t = 348;
pub const S_SKEL_DIE3: statenum_t = 347;
pub const S_SKEL_DIE2: statenum_t = 346;
pub const S_SKEL_DIE1: statenum_t = 345;
pub const S_SKEL_PAIN2: statenum_t = 344;
pub const S_SKEL_PAIN: statenum_t = 343;
pub const S_SKEL_MISS4: statenum_t = 342;
pub const S_SKEL_MISS3: statenum_t = 341;
pub const S_SKEL_MISS2: statenum_t = 340;
pub const S_SKEL_MISS1: statenum_t = 339;
pub const S_SKEL_FIST4: statenum_t = 338;
pub const S_SKEL_FIST3: statenum_t = 337;
pub const S_SKEL_FIST2: statenum_t = 336;
pub const S_SKEL_FIST1: statenum_t = 335;
pub const S_SKEL_RUN12: statenum_t = 334;
pub const S_SKEL_RUN11: statenum_t = 333;
pub const S_SKEL_RUN10: statenum_t = 332;
pub const S_SKEL_RUN9: statenum_t = 331;
pub const S_SKEL_RUN8: statenum_t = 330;
pub const S_SKEL_RUN7: statenum_t = 329;
pub const S_SKEL_RUN6: statenum_t = 328;
pub const S_SKEL_RUN5: statenum_t = 327;
pub const S_SKEL_RUN4: statenum_t = 326;
pub const S_SKEL_RUN3: statenum_t = 325;
pub const S_SKEL_RUN2: statenum_t = 324;
pub const S_SKEL_RUN1: statenum_t = 323;
pub const S_SKEL_STND2: statenum_t = 322;
pub const S_SKEL_STND: statenum_t = 321;
pub const S_TRACEEXP3: statenum_t = 320;
pub const S_TRACEEXP2: statenum_t = 319;
pub const S_TRACEEXP1: statenum_t = 318;
pub const S_TRACER2: statenum_t = 317;
pub const S_TRACER: statenum_t = 316;
pub const S_SMOKE5: statenum_t = 315;
pub const S_SMOKE4: statenum_t = 314;
pub const S_SMOKE3: statenum_t = 313;
pub const S_SMOKE2: statenum_t = 312;
pub const S_SMOKE1: statenum_t = 311;
pub const S_FIRE30: statenum_t = 310;
pub const S_FIRE29: statenum_t = 309;
pub const S_FIRE28: statenum_t = 308;
pub const S_FIRE27: statenum_t = 307;
pub const S_FIRE26: statenum_t = 306;
pub const S_FIRE25: statenum_t = 305;
pub const S_FIRE24: statenum_t = 304;
pub const S_FIRE23: statenum_t = 303;
pub const S_FIRE22: statenum_t = 302;
pub const S_FIRE21: statenum_t = 301;
pub const S_FIRE20: statenum_t = 300;
pub const S_FIRE19: statenum_t = 299;
pub const S_FIRE18: statenum_t = 298;
pub const S_FIRE17: statenum_t = 297;
pub const S_FIRE16: statenum_t = 296;
pub const S_FIRE15: statenum_t = 295;
pub const S_FIRE14: statenum_t = 294;
pub const S_FIRE13: statenum_t = 293;
pub const S_FIRE12: statenum_t = 292;
pub const S_FIRE11: statenum_t = 291;
pub const S_FIRE10: statenum_t = 290;
pub const S_FIRE9: statenum_t = 289;
pub const S_FIRE8: statenum_t = 288;
pub const S_FIRE7: statenum_t = 287;
pub const S_FIRE6: statenum_t = 286;
pub const S_FIRE5: statenum_t = 285;
pub const S_FIRE4: statenum_t = 284;
pub const S_FIRE3: statenum_t = 283;
pub const S_FIRE2: statenum_t = 282;
pub const S_FIRE1: statenum_t = 281;
pub const S_VILE_DIE10: statenum_t = 280;
pub const S_VILE_DIE9: statenum_t = 279;
pub const S_VILE_DIE8: statenum_t = 278;
pub const S_VILE_DIE7: statenum_t = 277;
pub const S_VILE_DIE6: statenum_t = 276;
pub const S_VILE_DIE5: statenum_t = 275;
pub const S_VILE_DIE4: statenum_t = 274;
pub const S_VILE_DIE3: statenum_t = 273;
pub const S_VILE_DIE2: statenum_t = 272;
pub const S_VILE_DIE1: statenum_t = 271;
pub const S_VILE_PAIN2: statenum_t = 270;
pub const S_VILE_PAIN: statenum_t = 269;
pub const S_VILE_HEAL3: statenum_t = 268;
pub const S_VILE_HEAL2: statenum_t = 267;
pub const S_VILE_HEAL1: statenum_t = 266;
pub const S_VILE_ATK11: statenum_t = 265;
pub const S_VILE_ATK10: statenum_t = 264;
pub const S_VILE_ATK9: statenum_t = 263;
pub const S_VILE_ATK8: statenum_t = 262;
pub const S_VILE_ATK7: statenum_t = 261;
pub const S_VILE_ATK6: statenum_t = 260;
pub const S_VILE_ATK5: statenum_t = 259;
pub const S_VILE_ATK4: statenum_t = 258;
pub const S_VILE_ATK3: statenum_t = 257;
pub const S_VILE_ATK2: statenum_t = 256;
pub const S_VILE_ATK1: statenum_t = 255;
pub const S_VILE_RUN12: statenum_t = 254;
pub const S_VILE_RUN11: statenum_t = 253;
pub const S_VILE_RUN10: statenum_t = 252;
pub const S_VILE_RUN9: statenum_t = 251;
pub const S_VILE_RUN8: statenum_t = 250;
pub const S_VILE_RUN7: statenum_t = 249;
pub const S_VILE_RUN6: statenum_t = 248;
pub const S_VILE_RUN5: statenum_t = 247;
pub const S_VILE_RUN4: statenum_t = 246;
pub const S_VILE_RUN3: statenum_t = 245;
pub const S_VILE_RUN2: statenum_t = 244;
pub const S_VILE_RUN1: statenum_t = 243;
pub const S_VILE_STND2: statenum_t = 242;
pub const S_VILE_STND: statenum_t = 241;
pub const S_SPOS_RAISE5: statenum_t = 240;
pub const S_SPOS_RAISE4: statenum_t = 239;
pub const S_SPOS_RAISE3: statenum_t = 238;
pub const S_SPOS_RAISE2: statenum_t = 237;
pub const S_SPOS_RAISE1: statenum_t = 236;
pub const S_SPOS_XDIE9: statenum_t = 235;
pub const S_SPOS_XDIE8: statenum_t = 234;
pub const S_SPOS_XDIE7: statenum_t = 233;
pub const S_SPOS_XDIE6: statenum_t = 232;
pub const S_SPOS_XDIE5: statenum_t = 231;
pub const S_SPOS_XDIE4: statenum_t = 230;
pub const S_SPOS_XDIE3: statenum_t = 229;
pub const S_SPOS_XDIE2: statenum_t = 228;
pub const S_SPOS_XDIE1: statenum_t = 227;
pub const S_SPOS_DIE5: statenum_t = 226;
pub const S_SPOS_DIE4: statenum_t = 225;
pub const S_SPOS_DIE3: statenum_t = 224;
pub const S_SPOS_DIE2: statenum_t = 223;
pub const S_SPOS_DIE1: statenum_t = 222;
pub const S_SPOS_PAIN2: statenum_t = 221;
pub const S_SPOS_PAIN: statenum_t = 220;
pub const S_SPOS_ATK3: statenum_t = 219;
pub const S_SPOS_ATK2: statenum_t = 218;
pub const S_SPOS_ATK1: statenum_t = 217;
pub const S_SPOS_RUN8: statenum_t = 216;
pub const S_SPOS_RUN7: statenum_t = 215;
pub const S_SPOS_RUN6: statenum_t = 214;
pub const S_SPOS_RUN5: statenum_t = 213;
pub const S_SPOS_RUN4: statenum_t = 212;
pub const S_SPOS_RUN3: statenum_t = 211;
pub const S_SPOS_RUN2: statenum_t = 210;
pub const S_SPOS_RUN1: statenum_t = 209;
pub const S_SPOS_STND2: statenum_t = 208;
pub const S_SPOS_STND: statenum_t = 207;
pub const S_POSS_RAISE4: statenum_t = 206;
pub const S_POSS_RAISE3: statenum_t = 205;
pub const S_POSS_RAISE2: statenum_t = 204;
pub const S_POSS_RAISE1: statenum_t = 203;
pub const S_POSS_XDIE9: statenum_t = 202;
pub const S_POSS_XDIE8: statenum_t = 201;
pub const S_POSS_XDIE7: statenum_t = 200;
pub const S_POSS_XDIE6: statenum_t = 199;
pub const S_POSS_XDIE5: statenum_t = 198;
pub const S_POSS_XDIE4: statenum_t = 197;
pub const S_POSS_XDIE3: statenum_t = 196;
pub const S_POSS_XDIE2: statenum_t = 195;
pub const S_POSS_XDIE1: statenum_t = 194;
pub const S_POSS_DIE5: statenum_t = 193;
pub const S_POSS_DIE4: statenum_t = 192;
pub const S_POSS_DIE3: statenum_t = 191;
pub const S_POSS_DIE2: statenum_t = 190;
pub const S_POSS_DIE1: statenum_t = 189;
pub const S_POSS_PAIN2: statenum_t = 188;
pub const S_POSS_PAIN: statenum_t = 187;
pub const S_POSS_ATK3: statenum_t = 186;
pub const S_POSS_ATK2: statenum_t = 185;
pub const S_POSS_ATK1: statenum_t = 184;
pub const S_POSS_RUN8: statenum_t = 183;
pub const S_POSS_RUN7: statenum_t = 182;
pub const S_POSS_RUN6: statenum_t = 181;
pub const S_POSS_RUN5: statenum_t = 180;
pub const S_POSS_RUN4: statenum_t = 179;
pub const S_POSS_RUN3: statenum_t = 178;
pub const S_POSS_RUN2: statenum_t = 177;
pub const S_POSS_RUN1: statenum_t = 176;
pub const S_POSS_STND2: statenum_t = 175;
pub const S_POSS_STND: statenum_t = 174;
pub const S_PLAY_XDIE9: statenum_t = 173;
pub const S_PLAY_XDIE8: statenum_t = 172;
pub const S_PLAY_XDIE7: statenum_t = 171;
pub const S_PLAY_XDIE6: statenum_t = 170;
pub const S_PLAY_XDIE5: statenum_t = 169;
pub const S_PLAY_XDIE4: statenum_t = 168;
pub const S_PLAY_XDIE3: statenum_t = 167;
pub const S_PLAY_XDIE2: statenum_t = 166;
pub const S_PLAY_XDIE1: statenum_t = 165;
pub const S_PLAY_DIE7: statenum_t = 164;
pub const S_PLAY_DIE6: statenum_t = 163;
pub const S_PLAY_DIE5: statenum_t = 162;
pub const S_PLAY_DIE4: statenum_t = 161;
pub const S_PLAY_DIE3: statenum_t = 160;
pub const S_PLAY_DIE2: statenum_t = 159;
pub const S_PLAY_DIE1: statenum_t = 158;
pub const S_PLAY_PAIN2: statenum_t = 157;
pub const S_PLAY_PAIN: statenum_t = 156;
pub const S_PLAY_ATK2: statenum_t = 155;
pub const S_PLAY_ATK1: statenum_t = 154;
pub const S_PLAY_RUN4: statenum_t = 153;
pub const S_PLAY_RUN3: statenum_t = 152;
pub const S_PLAY_RUN2: statenum_t = 151;
pub const S_PLAY_RUN1: statenum_t = 150;
pub const S_PLAY: statenum_t = 149;
pub const S_IFOG5: statenum_t = 148;
pub const S_IFOG4: statenum_t = 147;
pub const S_IFOG3: statenum_t = 146;
pub const S_IFOG2: statenum_t = 145;
pub const S_IFOG02: statenum_t = 144;
pub const S_IFOG01: statenum_t = 143;
pub const S_IFOG: statenum_t = 142;
pub const S_TFOG10: statenum_t = 141;
pub const S_TFOG9: statenum_t = 140;
pub const S_TFOG8: statenum_t = 139;
pub const S_TFOG7: statenum_t = 138;
pub const S_TFOG6: statenum_t = 137;
pub const S_TFOG5: statenum_t = 136;
pub const S_TFOG4: statenum_t = 135;
pub const S_TFOG3: statenum_t = 134;
pub const S_TFOG2: statenum_t = 133;
pub const S_TFOG02: statenum_t = 132;
pub const S_TFOG01: statenum_t = 131;
pub const S_TFOG: statenum_t = 130;
pub const S_EXPLODE3: statenum_t = 129;
pub const S_EXPLODE2: statenum_t = 128;
pub const S_EXPLODE1: statenum_t = 127;
pub const S_BFGEXP4: statenum_t = 126;
pub const S_BFGEXP3: statenum_t = 125;
pub const S_BFGEXP2: statenum_t = 124;
pub const S_BFGEXP: statenum_t = 123;
pub const S_BFGLAND6: statenum_t = 122;
pub const S_BFGLAND5: statenum_t = 121;
pub const S_BFGLAND4: statenum_t = 120;
pub const S_BFGLAND3: statenum_t = 119;
pub const S_BFGLAND2: statenum_t = 118;
pub const S_BFGLAND: statenum_t = 117;
pub const S_BFGSHOT2: statenum_t = 116;
pub const S_BFGSHOT: statenum_t = 115;
pub const S_ROCKET: statenum_t = 114;
pub const S_PLASEXP5: statenum_t = 113;
pub const S_PLASEXP4: statenum_t = 112;
pub const S_PLASEXP3: statenum_t = 111;
pub const S_PLASEXP2: statenum_t = 110;
pub const S_PLASEXP: statenum_t = 109;
pub const S_PLASBALL2: statenum_t = 108;
pub const S_PLASBALL: statenum_t = 107;
pub const S_RBALLX3: statenum_t = 106;
pub const S_RBALLX2: statenum_t = 105;
pub const S_RBALLX1: statenum_t = 104;
pub const S_RBALL2: statenum_t = 103;
pub const S_RBALL1: statenum_t = 102;
pub const S_TBALLX3: statenum_t = 101;
pub const S_TBALLX2: statenum_t = 100;
pub const S_TBALLX1: statenum_t = 99;
pub const S_TBALL2: statenum_t = 98;
pub const S_TBALL1: statenum_t = 97;
pub const S_PUFF4: statenum_t = 96;
pub const S_PUFF3: statenum_t = 95;
pub const S_PUFF2: statenum_t = 94;
pub const S_PUFF1: statenum_t = 93;
pub const S_BLOOD3: statenum_t = 92;
pub const S_BLOOD2: statenum_t = 91;
pub const S_BLOOD1: statenum_t = 90;
pub const S_BFGFLASH2: statenum_t = 89;
pub const S_BFGFLASH1: statenum_t = 88;
pub const S_BFG4: statenum_t = 87;
pub const S_BFG3: statenum_t = 86;
pub const S_BFG2: statenum_t = 85;
pub const S_BFG1: statenum_t = 84;
pub const S_BFGUP: statenum_t = 83;
pub const S_BFGDOWN: statenum_t = 82;
pub const S_BFG: statenum_t = 81;
pub const S_PLASMAFLASH2: statenum_t = 80;
pub const S_PLASMAFLASH1: statenum_t = 79;
pub const S_PLASMA2: statenum_t = 78;
pub const S_PLASMA1: statenum_t = 77;
pub const S_PLASMAUP: statenum_t = 76;
pub const S_PLASMADOWN: statenum_t = 75;
pub const S_PLASMA: statenum_t = 74;
pub const S_SAW3: statenum_t = 73;
pub const S_SAW2: statenum_t = 72;
pub const S_SAW1: statenum_t = 71;
pub const S_SAWUP: statenum_t = 70;
pub const S_SAWDOWN: statenum_t = 69;
pub const S_SAWB: statenum_t = 68;
pub const S_SAW: statenum_t = 67;
pub const S_MISSILEFLASH4: statenum_t = 66;
pub const S_MISSILEFLASH3: statenum_t = 65;
pub const S_MISSILEFLASH2: statenum_t = 64;
pub const S_MISSILEFLASH1: statenum_t = 63;
pub const S_MISSILE3: statenum_t = 62;
pub const S_MISSILE2: statenum_t = 61;
pub const S_MISSILE1: statenum_t = 60;
pub const S_MISSILEUP: statenum_t = 59;
pub const S_MISSILEDOWN: statenum_t = 58;
pub const S_MISSILE: statenum_t = 57;
pub const S_CHAINFLASH2: statenum_t = 56;
pub const S_CHAINFLASH1: statenum_t = 55;
pub const S_CHAIN3: statenum_t = 54;
pub const S_CHAIN2: statenum_t = 53;
pub const S_CHAIN1: statenum_t = 52;
pub const S_CHAINUP: statenum_t = 51;
pub const S_CHAINDOWN: statenum_t = 50;
pub const S_CHAIN: statenum_t = 49;
pub const S_DSGUNFLASH2: statenum_t = 48;
pub const S_DSGUNFLASH1: statenum_t = 47;
pub const S_DSNR2: statenum_t = 46;
pub const S_DSNR1: statenum_t = 45;
pub const S_DSGUN10: statenum_t = 44;
pub const S_DSGUN9: statenum_t = 43;
pub const S_DSGUN8: statenum_t = 42;
pub const S_DSGUN7: statenum_t = 41;
pub const S_DSGUN6: statenum_t = 40;
pub const S_DSGUN5: statenum_t = 39;
pub const S_DSGUN4: statenum_t = 38;
pub const S_DSGUN3: statenum_t = 37;
pub const S_DSGUN2: statenum_t = 36;
pub const S_DSGUN1: statenum_t = 35;
pub const S_DSGUNUP: statenum_t = 34;
pub const S_DSGUNDOWN: statenum_t = 33;
pub const S_DSGUN: statenum_t = 32;
pub const S_SGUNFLASH2: statenum_t = 31;
pub const S_SGUNFLASH1: statenum_t = 30;
pub const S_SGUN9: statenum_t = 29;
pub const S_SGUN8: statenum_t = 28;
pub const S_SGUN7: statenum_t = 27;
pub const S_SGUN6: statenum_t = 26;
pub const S_SGUN5: statenum_t = 25;
pub const S_SGUN4: statenum_t = 24;
pub const S_SGUN3: statenum_t = 23;
pub const S_SGUN2: statenum_t = 22;
pub const S_SGUN1: statenum_t = 21;
pub const S_SGUNUP: statenum_t = 20;
pub const S_SGUNDOWN: statenum_t = 19;
pub const S_SGUN: statenum_t = 18;
pub const S_PISTOLFLASH: statenum_t = 17;
pub const S_PISTOL4: statenum_t = 16;
pub const S_PISTOL3: statenum_t = 15;
pub const S_PISTOL2: statenum_t = 14;
pub const S_PISTOL1: statenum_t = 13;
pub const S_PISTOLUP: statenum_t = 12;
pub const S_PISTOLDOWN: statenum_t = 11;
pub const S_PISTOL: statenum_t = 10;
pub const S_PUNCH5: statenum_t = 9;
pub const S_PUNCH4: statenum_t = 8;
pub const S_PUNCH3: statenum_t = 7;
pub const S_PUNCH2: statenum_t = 6;
pub const S_PUNCH1: statenum_t = 5;
pub const S_PUNCHUP: statenum_t = 4;
pub const S_PUNCHDOWN: statenum_t = 3;
pub const S_PUNCH: statenum_t = 2;
pub const S_LIGHTDONE: statenum_t = 1;
pub const S_NULL: statenum_t = 0;
pub const NUMMOBJTYPES: mobjtype_t = 137;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuitem_t {
    pub status: i16,
    pub name: [::core::ffi::c_char; 10],
    pub routine: Option<unsafe extern "C" fn(i32) -> ()>,
    pub alphaKey: ::core::ffi::c_char,
}
pub type menu_t = menu_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_s {
    pub numitems: i16,
    pub prevMenu: *mut menu_s,
    pub menuitems: *mut menuitem_t,
    pub routine: Option<unsafe extern "C" fn() -> ()>,
    pub x: i16,
    pub y: i16,
    pub lastOn: i16,
}
pub const read2_end: C2RustUnnamed_6 = 1;
pub const read1_end: C2RustUnnamed_5 = 1;
pub const load_end: C2RustUnnamed_8 = 6;
pub const scrnsize: C2RustUnnamed_4 = 3;
pub const mousesens: C2RustUnnamed_4 = 5;
pub const messages: C2RustUnnamed_4 = 1;
pub const detail: C2RustUnnamed_4 = 2;
pub const music_vol: C2RustUnnamed_7 = 2;
pub const sfx_vol: C2RustUnnamed_7 = 0;
pub const sound_end: C2RustUnnamed_7 = 4;
pub const opt_end: C2RustUnnamed_4 = 8;
pub const ep1: C2RustUnnamed_2 = 0;
pub const hurtme: C2RustUnnamed_3 = 2;
pub const nightmare: C2RustUnnamed_3 = 4;
pub const newg_end: C2RustUnnamed_3 = 5;
pub const ep_end: C2RustUnnamed_2 = 4;
pub const main_end: C2RustUnnamed_1 = 6;
pub const quitdoom: C2RustUnnamed_1 = 5;
pub const readthis: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_1 = u32;
pub const savegame: C2RustUnnamed_1 = 3;
pub const loadgame: C2RustUnnamed_1 = 2;
pub const options: C2RustUnnamed_1 = 1;
pub const newgame: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = u32;
pub const ep4: C2RustUnnamed_2 = 3;
pub const ep3: C2RustUnnamed_2 = 2;
pub const ep2: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = u32;
pub const violence: C2RustUnnamed_3 = 3;
pub const toorough: C2RustUnnamed_3 = 1;
pub const killthings: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = u32;
pub const soundvol: C2RustUnnamed_4 = 7;
pub const option_empty2: C2RustUnnamed_4 = 6;
pub const option_empty1: C2RustUnnamed_4 = 4;
pub const endgame: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = u32;
pub const rdthsempty1: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = u32;
pub const rdthsempty2: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = u32;
pub const sfx_empty2: C2RustUnnamed_7 = 3;
pub const sfx_empty1: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_8 = u32;
pub const load6: C2RustUnnamed_8 = 5;
pub const load5: C2RustUnnamed_8 = 4;
pub const load4: C2RustUnnamed_8 = 3;
pub const load3: C2RustUnnamed_8 = 2;
pub const load2: C2RustUnnamed_8 = 1;
pub const load1: C2RustUnnamed_8 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const KEY_ESCAPE: i32 = 27 as i32;
pub const KEY_ENTER: i32 = 13;
pub const KEY_BACKSPACE: i32 = 127;
pub const KEY_PAUSE: i32 = 0xff as i32;
pub const KEY_CAPSLOCK: i32 = 0x80 as i32
    + 0x3a as i32;
pub const KEY_NUMLOCK: i32 = 0x80 as i32
    + 0x45 as i32;
pub const KEY_SCRLCK: i32 = 0x80 as i32
    + 0x46 as i32;
pub const GAMMALVL0: &str = "Gamma correction OFF\0";
pub const GAMMALVL1: &str = "Gamma correction level 1\0";
pub const GAMMALVL2: &str = "Gamma correction level 2\0";
pub const GAMMALVL3: &str = "Gamma correction level 3\0";
pub const GAMMALVL4: &str = "Gamma correction level 4\0";
pub const EMPTYSTRING: &str = "empty slot\0";
pub const NUM_QUITMESSAGES: i32 = 8 as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const HU_FONTSTART: i32 = '!' as i32;
pub const HU_FONTEND: i32 = '_' as i32;
pub const HU_FONTSIZE: i32 = HU_FONTEND - HU_FONTSTART
    + 1 as i32;
pub const SAVESTRINGSIZE: i32 = 24 as i32;
pub static mut mouseSensitivity: i32 = 5 as i32;
pub static mut showMessages: i32 = 1 as i32;
pub static mut detailLevel: i32 = 0 as i32;
pub static mut screenblocks: i32 = 10 as i32;
#[no_mangle]
pub static mut screenSize: i32 = 0;
#[no_mangle]
pub static mut quickSaveSlot: i32 = 0;
#[no_mangle]
pub static mut messageToPrint: i32 = 0;
pub static mut messageString: String = String::new();
#[no_mangle]
pub static mut messx: i32 = 0;
#[no_mangle]
pub static mut messy: i32 = 0;
#[no_mangle]
pub static mut messageLastMenuActive: i32 = 0;
#[no_mangle]
pub static mut messageNeedsInput: bool = false;
#[no_mangle]
pub static mut messageRoutine: Option<unsafe extern "C" fn(i32) -> ()> = None;
pub static gammamsg: [&str; 5] = [GAMMALVL0, GAMMALVL1, GAMMALVL2, GAMMALVL3, GAMMALVL4];
#[no_mangle]
pub static mut saveStringEnter: i32 = 0;
#[no_mangle]
pub static mut saveSlot: i32 = 0;
#[no_mangle]
pub static mut saveCharIndex: i32 = 0;
#[no_mangle]
pub static mut saveOldString: String = String::new();
pub static mut inhelpscreens: bool = false;
pub static mut menuactive: bool = false;
pub const SKULLXOFF: i32 = -(32 as i32);
pub const LINEHEIGHT: i32 = 16 as i32;
pub static mut savegamestrings: [String; 10] = [
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
];
#[no_mangle]
pub static mut endstring: [::core::ffi::c_char; 160] = [0; 160];
#[no_mangle]
pub static mut itemOn: i16 = 0;
#[no_mangle]
pub static mut skullAnimCounter: i16 = 0;
#[no_mangle]
pub static mut whichSkull: i16 = 0;
#[no_mangle]
pub static skullName: [&str; 2] = ["M_SKULL1", "M_SKULL2"];
#[no_mangle]
pub static mut currentMenu: *mut menu_t = ::core::ptr::null::<menu_t>() as *mut menu_t;
#[no_mangle]
pub static mut main_e: C2RustUnnamed_1 = newgame;
#[no_mangle]
pub static mut MainMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_NGAME\0\0\0"),
            routine: Some(M_NewGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'n' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_OPTION\0\0"),
            routine: Some(M_Options as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'o' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_LOADG\0\0\0"),
            routine: Some(M_LoadGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'l' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SAVEG\0\0\0"),
            routine: Some(M_SaveGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_RDTHIS\0\0"),
            routine: Some(M_ReadThis as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'r' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_QUITG\0\0\0"),
            routine: Some(M_QuitDOOM as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'q' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut MainDef: menu_t = unsafe {
    menu_s {
        numitems: main_end as i32 as i16,
        prevMenu: ::core::ptr::null::<menu_s>() as *mut menu_s,
        menuitems: &raw const MainMenu as *mut menuitem_t,
        routine: Some(M_DrawMainMenu as unsafe extern "C" fn() -> ()),
        x: 97 as i16,
        y: 64 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut episodes_e: C2RustUnnamed_2 = ep1;
#[no_mangle]
pub static mut EpisodeMenu: [menuitem_t; 4] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI1\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'k' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI2\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 't' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI3\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'i' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_EPI4\0\0\0\0"),
            routine: Some(M_Episode as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 't' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut EpiDef: menu_t = unsafe {
    menu_s {
        numitems: ep_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const EpisodeMenu as *mut menuitem_t,
        routine: Some(M_DrawEpisode as unsafe extern "C" fn() -> ()),
        x: 48 as i16,
        y: 63 as i16,
        lastOn: ep1 as i32 as i16,
    }
};
#[no_mangle]
pub static mut newgame_e: C2RustUnnamed_3 = killthings;
#[no_mangle]
pub static mut NewGameMenu: [menuitem_t; 5] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_JKILL\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'i' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_ROUGH\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'h' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_HURT\0\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'h' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_ULTRA\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'u' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_NMARE\0\0\0"),
            routine: Some(
                M_ChooseSkill as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'n' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut NewDef: menu_t = unsafe {
    menu_s {
        numitems: newg_end as i32 as i16,
        prevMenu: &raw const EpiDef as *mut menu_s,
        menuitems: &raw const NewGameMenu as *mut menuitem_t,
        routine: Some(M_DrawNewGame as unsafe extern "C" fn() -> ()),
        x: 48 as i16,
        y: 63 as i16,
        lastOn: hurtme as i32 as i16,
    }
};
#[no_mangle]
pub static mut options_e: C2RustUnnamed_4 = endgame;
#[no_mangle]
pub static mut OptionsMenu: [menuitem_t; 8] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_ENDGAM\0\0"),
            routine: Some(M_EndGame as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'e' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_MESSG\0\0\0"),
            routine: Some(
                M_ChangeMessages as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'm' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_DETAIL\0\0"),
            routine: Some(
                M_ChangeDetail as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'g' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SCRNSZ\0\0"),
            routine: Some(
                M_SizeDisplay as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_MSENS\0\0\0"),
            routine: Some(
                M_ChangeSensitivity as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 'm' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SVOL\0\0\0\0"),
            routine: Some(M_Sound as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut OptionsDef: menu_t = unsafe {
    menu_s {
        numitems: opt_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const OptionsMenu as *mut menuitem_t,
        routine: Some(M_DrawOptions as unsafe extern "C" fn() -> ()),
        x: 60 as i16,
        y: 37 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut read_e: C2RustUnnamed_5 = rdthsempty1;
#[no_mangle]
pub static mut ReadMenu1: [menuitem_t; 1] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(M_ReadThis2 as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 0 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut ReadDef1: menu_t = unsafe {
    menu_s {
        numitems: read1_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const ReadMenu1 as *mut menuitem_t,
        routine: Some(M_DrawReadThis1 as unsafe extern "C" fn() -> ()),
        x: 280 as i16,
        y: 185 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut read_e2: C2RustUnnamed_6 = rdthsempty2;
#[no_mangle]
pub static mut ReadMenu2: [menuitem_t; 1] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_FinishReadThis as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: 0 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut ReadDef2: menu_t = unsafe {
    menu_s {
        numitems: read2_end as i32 as i16,
        prevMenu: &raw const ReadDef1 as *mut menu_s,
        menuitems: &raw const ReadMenu2 as *mut menuitem_t,
        routine: Some(M_DrawReadThis2 as unsafe extern "C" fn() -> ()),
        x: 330 as i16,
        y: 175 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut sound_e: C2RustUnnamed_7 = sfx_vol;
#[no_mangle]
pub static mut SoundMenu: [menuitem_t; 4] = unsafe {
    [
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_SFXVOL\0\0"),
            routine: Some(M_SfxVol as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 's' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 2 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"M_MUSVOL\0\0"),
            routine: Some(M_MusicVol as unsafe extern "C" fn(i32) -> ()),
            alphaKey: 'm' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: -(1 as i32) as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: None,
            alphaKey: '\0' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut SoundDef: menu_t = unsafe {
    menu_s {
        numitems: sound_end as i32 as i16,
        prevMenu: &raw const OptionsDef as *mut menu_s,
        menuitems: &raw const SoundMenu as *mut menuitem_t,
        routine: Some(M_DrawSound as unsafe extern "C" fn() -> ()),
        x: 80 as i16,
        y: 64 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut load_e: C2RustUnnamed_8 = load1;
#[no_mangle]
pub static mut LoadMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '1' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '2' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '3' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '4' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '5' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_LoadSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '6' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut LoadDef: menu_t = unsafe {
    menu_s {
        numitems: load_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const LoadMenu as *mut menuitem_t,
        routine: Some(M_DrawLoad as unsafe extern "C" fn() -> ()),
        x: 80 as i16,
        y: 54 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub static mut SaveMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '1' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '2' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '3' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '4' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '5' as i32 as ::core::ffi::c_char,
        },
        menuitem_t {
            status: 1 as i16,
            name: ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0\0\0\0\0\0\0\0\0\0"),
            routine: Some(
                M_SaveSelect as unsafe extern "C" fn(i32) -> (),
            ),
            alphaKey: '6' as i32 as ::core::ffi::c_char,
        },
    ]
};
#[no_mangle]
pub static mut SaveDef: menu_t = unsafe {
    menu_s {
        numitems: load_end as i32 as i16,
        prevMenu: &raw const MainDef as *mut menu_s,
        menuitems: &raw const SaveMenu as *mut menuitem_t,
        routine: Some(M_DrawSave as unsafe extern "C" fn() -> ()),
        x: 80 as i16,
        y: 54 as i16,
        lastOn: 0 as i16,
    }
};
#[no_mangle]
pub unsafe extern "C" fn M_ReadSaveStrings() {
    let mut handle: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut i: i32 = 0;
    let mut name: [::core::ffi::c_char; 256] = [0; 256];
    i = 0 as i32;
    while i < load_end as i32 {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            P_SaveGameFile(i),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        );
        handle = fopen(
            &raw mut name as *mut ::core::ffi::c_char,
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if handle.is_null() {
            savegamestrings[i as usize] = EMPTYSTRING.trim_end_matches('\0').to_string();
            LoadMenu[i as usize].status = 0 as i16;
        } else {
            let mut buf: [u8; 24] = [0; 24];
            fread(
                buf.as_mut_ptr() as *mut ::core::ffi::c_void,
                1 as size_t,
                SAVESTRINGSIZE as size_t,
                handle,
            );
            let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            savegamestrings[i as usize] = String::from_utf8_lossy(&buf[..len]).into_owned();
            fclose(handle);
            LoadMenu[i as usize].status = 1 as i16;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawLoad() {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        72 as i32,
        28 as i32,
        W_CacheLumpName("M_LOADG",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    i = 0 as i32;
    while i < load_end as i32 {
        M_DrawSaveLoadBorder(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
        );
        M_WriteText(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
            &savegamestrings[i as usize],
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSaveLoadBorder(
    mut x: i32,
    mut y: i32,
) {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        x - 8 as i32,
        y + 7 as i32,
        W_CacheLumpName("M_LSLEFT",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    i = 0 as i32;
    while i < 24 as i32 {
        V_DrawPatchDirect(
            x,
            y + 7 as i32,
            W_CacheLumpName("M_LSCNTR",
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
        x += 8 as i32;
        i += 1;
    }
    V_DrawPatchDirect(
        x,
        y + 7 as i32,
        W_CacheLumpName("M_LSRGHT",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_LoadSelect(mut choice: i32) {
    let mut name: [::core::ffi::c_char; 256] = [0; 256];
    M_StringCopy(
        &raw mut name as *mut ::core::ffi::c_char,
        P_SaveGameFile(choice),
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
    );
    G_LoadGame(&raw mut name as *mut ::core::ffi::c_char);
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_LoadGame(mut choice: i32) {
    if netgame {
        M_StartMessage(
            "you can't do load while in a net game!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    M_SetupNextMenu(&raw mut LoadDef);
    M_ReadSaveStrings();
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSave() {
    let mut i: i32 = 0;
    V_DrawPatchDirect(
        72 as i32,
        28 as i32,
        W_CacheLumpName("M_SAVEG",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    i = 0 as i32;
    while i < load_end as i32 {
        M_DrawSaveLoadBorder(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
        );
        M_WriteText(
            LoadDef.x as i32,
            LoadDef.y as i32 + LINEHEIGHT * i,
            &savegamestrings[i as usize],
        );
        i += 1;
    }
    if saveStringEnter != 0 {
        i = M_StringWidth(&savegamestrings[saveSlot as usize]);
        M_WriteText(
            LoadDef.x as i32 + i,
            LoadDef.y as i32 + LINEHEIGHT * saveSlot,
            "_",
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_DoSave(mut slot: i32) {
    let name_cstring = ::std::ffi::CString::new(savegamestrings[slot as usize].as_str())
        .unwrap();
    G_SaveGame(slot, name_cstring.as_ptr() as *mut ::core::ffi::c_char);
    M_ClearMenus();
    if quickSaveSlot == -(2 as i32) {
        quickSaveSlot = slot;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveSelect(mut choice: i32) {
    saveStringEnter = 1 as i32;
    saveSlot = choice;
    saveOldString = savegamestrings[choice as usize].clone();
    if savegamestrings[choice as usize] == EMPTYSTRING.trim_end_matches('\0') {
        savegamestrings[choice as usize].clear();
    }
    saveCharIndex = savegamestrings[choice as usize].len() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn M_SaveGame(mut choice: i32) {
    if !usergame {
        M_StartMessage(
            "you can't save if you aren't playing!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if gamestate as u32
        != GS_LEVEL as i32 as u32
    {
        return;
    }
    M_SetupNextMenu(&raw mut SaveDef);
    M_ReadSaveStrings();
}
#[no_mangle]
pub static mut tempstring: [::core::ffi::c_char; 80] = [0; 80];
#[no_mangle]
pub unsafe extern "C" fn M_QuickSaveResponse(mut key: i32) {
    if key == key_menu_confirm {
        M_DoSave(quickSaveSlot);
        S_StartSound(NULL, sfx_swtchx as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickSave() {
    if !usergame {
        S_StartSound(NULL, sfx_oof as i32);
        return;
    }
    if gamestate as u32
        != GS_LEVEL as i32 as u32
    {
        return;
    }
    if quickSaveSlot < 0 as i32 {
        M_StartControlPanel();
        M_ReadSaveStrings();
        M_SetupNextMenu(&raw mut SaveDef);
        quickSaveSlot = -(2 as i32);
        return;
    }
    let quicksave_name_cstring = ::std::ffi::CString::new(
        savegamestrings[quickSaveSlot as usize].as_str(),
    )
        .unwrap();
    snprintf(
        &raw mut tempstring as *mut ::core::ffi::c_char,
        80 as size_t,
        b"quicksave over your game named\n\n'%s'?\n\npress y or n.\0" as *const u8
            as *const ::core::ffi::c_char,
        quicksave_name_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(&raw mut tempstring as *mut ::core::ffi::c_char)
            .to_str()
            .unwrap(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_QuickSaveResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickLoadResponse(mut key: i32) {
    if key == key_menu_confirm {
        M_LoadSelect(quickSaveSlot);
        S_StartSound(NULL, sfx_swtchx as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_QuickLoad() {
    if netgame {
        M_StartMessage(
            "you can't quickload during a netgame!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if quickSaveSlot < 0 as i32 {
        M_StartMessage(
            "you haven't picked a quicksave slot yet!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    let quickload_name_cstring = ::std::ffi::CString::new(
        savegamestrings[quickSaveSlot as usize].as_str(),
    )
        .unwrap();
    snprintf(
        &raw mut tempstring as *mut ::core::ffi::c_char,
        80 as size_t,
        b"do you want to quickload the game named\n\n'%s'?\n\npress y or n.\0"
            as *const u8 as *const ::core::ffi::c_char,
        quickload_name_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(&raw mut tempstring as *mut ::core::ffi::c_char)
            .to_str()
            .unwrap(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_QuickLoadResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawReadThis1() {
    let mut lumpname: *mut ::core::ffi::c_char = b"CREDIT\0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut skullx: i32 = 330 as i32;
    let mut skully: i32 = 175 as i32;
    inhelpscreens = true;
    match gameversion as u32 {
        1 | 2 | 3 | 4 | 5 => {
            if gamemode as u32
                == commercial as i32 as u32
            {
                lumpname = b"HELP\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                skullx = 330 as i32;
                skully = 165 as i32;
            } else {
                lumpname = b"HELP2\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                skullx = 280 as i32;
                skully = 185 as i32;
            }
        }
        6 | 9 => {
            lumpname = b"HELP1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 | 8 => {
            lumpname = b"HELP\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            I_Error("Unhandled game version");
        }
    }
    lumpname = lumpname;
    V_DrawPatchDirect(
        0 as i32,
        0 as i32,
        W_CacheLumpName(
            &wad_name8_to_string(lumpname),
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    ReadDef1.x = skullx as i16;
    ReadDef1.y = skully as i16;
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawReadThis2() {
    inhelpscreens = true;
    V_DrawPatchDirect(
        0 as i32,
        0 as i32,
        W_CacheLumpName("HELP1",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSound() {
    V_DrawPatchDirect(
        60 as i32,
        38 as i32,
        W_CacheLumpName("M_SVOL",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    M_DrawThermo(
        SoundDef.x as i32,
        SoundDef.y as i32
            + LINEHEIGHT * (sfx_vol as i32 + 1 as i32),
        16 as i32,
        sfxVolume,
    );
    M_DrawThermo(
        SoundDef.x as i32,
        SoundDef.y as i32
            + LINEHEIGHT * (music_vol as i32 + 1 as i32),
        16 as i32,
        musicVolume,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_Sound(mut choice: i32) {
    M_SetupNextMenu(&raw mut SoundDef);
}
#[no_mangle]
pub unsafe extern "C" fn M_SfxVol(mut choice: i32) {
    match choice {
        0 => {
            if sfxVolume != 0 {
                sfxVolume -= 1;
            }
        }
        1 => {
            if sfxVolume < 15 as i32 {
                sfxVolume += 1;
            }
        }
        _ => {}
    }
    S_SetSfxVolume(sfxVolume * 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn M_MusicVol(mut choice: i32) {
    match choice {
        0 => {
            if musicVolume != 0 {
                musicVolume -= 1;
            }
        }
        1 => {
            if musicVolume < 15 as i32 {
                musicVolume += 1;
            }
        }
        _ => {}
    }
    S_SetMusicVolume(musicVolume * 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawMainMenu() {
    V_DrawPatchDirect(
        94 as i32,
        2 as i32,
        W_CacheLumpName("M_DOOM",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawNewGame() {
    V_DrawPatchDirect(
        96 as i32,
        14 as i32,
        W_CacheLumpName("M_NEWG",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        54 as i32,
        38 as i32,
        W_CacheLumpName("M_SKILL",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_NewGame(mut choice: i32) {
    if netgame && !demoplayback {
        M_StartMessage(
            "you can't start a new game\nwhile in a network game.\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    if gamemode as u32
        == commercial as i32 as u32
        || gameversion as u32
            == exe_chex as i32 as u32
    {
        M_SetupNextMenu(&raw mut NewDef);
    } else {
        M_SetupNextMenu(&raw mut EpiDef);
    };
}
#[no_mangle]
pub static mut epi: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn M_DrawEpisode() {
    V_DrawPatchDirect(
        54 as i32,
        38 as i32,
        W_CacheLumpName("M_EPISOD",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_VerifyNightmare(mut key: i32) {
    if key != key_menu_confirm {
        return;
    }
    G_DeferedInitNew(
        nightmare as i32 as skill_t,
        epi + 1 as i32,
        1 as i32,
    );
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_ChooseSkill(mut choice: i32) {
    if choice == nightmare as i32 {
        M_StartMessage(
            "are you sure? this skill level\nisn't even remotely fair.\n\npress y or n.",
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(i32) -> ()>,
                *mut ::core::ffi::c_void,
            >(Some(M_VerifyNightmare as unsafe extern "C" fn(i32) -> ())),
            true,
        );
        return;
    }
    G_DeferedInitNew(
        choice as skill_t,
        epi + 1 as i32,
        1 as i32,
    );
    M_ClearMenus();
}
#[no_mangle]
pub unsafe extern "C" fn M_Episode(mut choice: i32) {
    if gamemode as u32
        == shareware as i32 as u32 && choice != 0
    {
        M_StartMessage(
            "this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\npress a key.",
            NULL,
            false,
        );
        M_SetupNextMenu(&raw mut ReadDef1);
        return;
    }
    if gamemode as u32
        == registered as i32 as u32
        && choice > 2 as i32
    {
        fprintf(
            stderr,
            b"M_Episode: 4th episode requires UltimateDOOM\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        choice = 0 as i32;
    }
    epi = choice;
    M_SetupNextMenu(&raw mut NewDef);
}
static detailNames: [&str; 2] = ["M_GDHIGH", "M_GDLOW"];
static msgNames: [&str; 2] = ["M_MSGOFF", "M_MSGON"];
#[no_mangle]
pub unsafe extern "C" fn M_DrawOptions() {
    V_DrawPatchDirect(
        108 as i32,
        15 as i32,
        W_CacheLumpName("M_OPTTTL",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        OptionsDef.x as i32 + 175 as i32,
        OptionsDef.y as i32 + LINEHEIGHT * detail as i32,
        W_CacheLumpName(
            detailNames[detailLevel as usize],
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        OptionsDef.x as i32 + 120 as i32,
        OptionsDef.y as i32 + LINEHEIGHT * messages as i32,
        W_CacheLumpName(msgNames[showMessages as usize], PU_CACHE as i32)
            as *mut patch_t,
    );
    M_DrawThermo(
        OptionsDef.x as i32,
        OptionsDef.y as i32
            + LINEHEIGHT * (mousesens as i32 + 1 as i32),
        10 as i32,
        mouseSensitivity,
    );
    M_DrawThermo(
        OptionsDef.x as i32,
        OptionsDef.y as i32
            + LINEHEIGHT * (scrnsize as i32 + 1 as i32),
        9 as i32,
        screenSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_Options(mut choice: i32) {
    M_SetupNextMenu(&raw mut OptionsDef);
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeMessages(mut choice: i32) {
    choice = 0 as i32;
    showMessages = 1 as i32 - showMessages;
    if showMessages == 0 {
        players[consoleplayer as usize].message = b"Messages OFF\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        players[consoleplayer as usize].message = b"Messages ON\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    message_dontfuckwithme = true;
}
#[no_mangle]
pub unsafe extern "C" fn M_EndGameResponse(mut key: i32) {
    if key != key_menu_confirm {
        return;
    }
    (*currentMenu).lastOn = itemOn;
    M_ClearMenus();
    D_StartTitle();
}
#[no_mangle]
pub unsafe extern "C" fn M_EndGame(mut choice: i32) {
    choice = 0 as i32;
    if !usergame {
        S_StartSound(NULL, sfx_oof as i32);
        return;
    }
    if netgame {
        M_StartMessage(
            "you can't end a netgame!\n\npress a key.",
            NULL,
            false,
        );
        return;
    }
    M_StartMessage(
            "are you sure you want to end the game?\n\npress y or n.",
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_EndGameResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadThis(mut choice: i32) {
    choice = 0 as i32;
    M_SetupNextMenu(&raw mut ReadDef1);
}
#[no_mangle]
pub unsafe extern "C" fn M_ReadThis2(mut choice: i32) {
    if gameversion as u32
        <= exe_doom_1_9 as i32 as u32
        && gamemode as u32
            != commercial as i32 as u32
    {
        choice = 0 as i32;
        M_SetupNextMenu(&raw mut ReadDef2);
    } else {
        M_FinishReadThis(0 as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_FinishReadThis(mut choice: i32) {
    choice = 0 as i32;
    M_SetupNextMenu(&raw mut MainDef);
}
#[no_mangle]
pub static mut quitsounds: [i32; 8] = [
    sfx_pldeth as i32,
    sfx_dmpain as i32,
    sfx_popain as i32,
    sfx_slop as i32,
    sfx_telept as i32,
    sfx_posit1 as i32,
    sfx_posit3 as i32,
    sfx_sgtatk as i32,
];
#[no_mangle]
pub static mut quitsounds2: [i32; 8] = [
    sfx_vilact as i32,
    sfx_getpow as i32,
    sfx_boscub as i32,
    sfx_slop as i32,
    sfx_skeswg as i32,
    sfx_kntdth as i32,
    sfx_bspact as i32,
    sfx_sgtatk as i32,
];
#[no_mangle]
pub unsafe extern "C" fn M_QuitResponse(mut key: i32) {
    if key != key_menu_confirm {
        return;
    }
    if !netgame {
        if gamemode as u32
            == commercial as i32 as u32
        {
            S_StartSound(
                NULL,
                quitsounds2[(gametic >> 2 as i32
                    & 7 as i32) as usize],
            );
        } else {
            S_StartSound(
                NULL,
                quitsounds[(gametic >> 2 as i32 & 7 as i32)
                    as usize],
            );
        }
        I_WaitVBL(105 as i32);
    }
    I_Quit();
}
unsafe fn M_SelectEndMessage() -> &'static str {
    let endmsg: &'static [&'static str; 8] = if (if gamemission as u32
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
        &doom1_endmsg
    } else {
        &doom2_endmsg
    };
    endmsg[(gametic % NUM_QUITMESSAGES) as usize]
}
#[no_mangle]
pub unsafe extern "C" fn M_QuitDOOM(mut choice: i32) {
    let endmsg_cstring = ::std::ffi::CString::new(M_SelectEndMessage()).unwrap();
    snprintf(
        &raw mut endstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>() as size_t,
        b"%s\n\n(press y to quit to dos.)\0" as *const u8 as *const ::core::ffi::c_char,
        endmsg_cstring.as_ptr(),
    );
    M_StartMessage(
        ::std::ffi::CStr::from_ptr(&raw mut endstring as *mut ::core::ffi::c_char)
            .to_str()
            .unwrap(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> ()>,
            *mut ::core::ffi::c_void,
        >(Some(M_QuitResponse as unsafe extern "C" fn(i32) -> ())),
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeSensitivity(mut choice: i32) {
    match choice {
        0 => {
            if mouseSensitivity != 0 {
                mouseSensitivity -= 1;
            }
        }
        1 => {
            if mouseSensitivity < 9 as i32 {
                mouseSensitivity += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeDetail(mut choice: i32) {
    choice = 0 as i32;
    detailLevel = 1 as i32 - detailLevel;
    R_SetViewSize(screenblocks, detailLevel);
    if detailLevel == 0 {
        players[consoleplayer as usize].message = b"High detail\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        players[consoleplayer as usize].message = b"Low detail\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn M_SizeDisplay(mut choice: i32) {
    match choice {
        0 => {
            if screenSize > 0 as i32 {
                screenblocks -= 1;
                screenSize -= 1;
            }
        }
        1 => {
            if screenSize < 8 as i32 {
                screenblocks += 1;
                screenSize += 1;
            }
        }
        _ => {}
    }
    R_SetViewSize(screenblocks, detailLevel);
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawThermo(
    mut x: i32,
    mut y: i32,
    mut thermWidth: i32,
    mut thermDot: i32,
) {
    let mut xx: i32 = 0;
    let mut i: i32 = 0;
    xx = x;
    V_DrawPatchDirect(
        xx,
        y,
        W_CacheLumpName("M_THERML",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    xx += 8 as i32;
    i = 0 as i32;
    while i < thermWidth {
        V_DrawPatchDirect(
            xx,
            y,
            W_CacheLumpName("M_THERMM",
                PU_CACHE as i32,
            ) as *mut patch_t,
        );
        xx += 8 as i32;
        i += 1;
    }
    V_DrawPatchDirect(
        xx,
        y,
        W_CacheLumpName("M_THERMR",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
    V_DrawPatchDirect(
        x + 8 as i32 + thermDot * 8 as i32,
        y,
        W_CacheLumpName("M_THERMO",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawEmptyCell(
    mut menu: *mut menu_t,
    mut item: i32,
) {
    V_DrawPatchDirect(
        (*menu).x as i32 - 10 as i32,
        (*menu).y as i32 + item * LINEHEIGHT - 1 as i32,
        W_CacheLumpName("M_CELL1",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_DrawSelCell(
    mut menu: *mut menu_t,
    mut item: i32,
) {
    V_DrawPatchDirect(
        (*menu).x as i32 - 10 as i32,
        (*menu).y as i32 + item * LINEHEIGHT - 1 as i32,
        W_CacheLumpName("M_CELL2",
            PU_CACHE as i32,
        ) as *mut patch_t,
    );
}
pub unsafe fn M_StartMessage(
    string: &str,
    mut routine: *mut ::core::ffi::c_void,
    mut input: bool,
) {
    messageLastMenuActive = menuactive as i32;
    messageToPrint = 1 as i32;
    messageString = string.to_string();
    messageRoutine = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(i32) -> ()>,
    >(routine);
    messageNeedsInput = input;
    menuactive = true;
}
#[no_mangle]
pub unsafe extern "C" fn M_StopMessage() {
    menuactive = messageLastMenuActive != 0;
    messageToPrint = 0 as i32;
}
pub unsafe fn M_StringWidth(string: &str) -> i32 {
    let mut w: i32 = 0 as i32;
    let mut c: i32 = 0;
    for b in string.bytes() {
        c = toupper(b as i32) - HU_FONTSTART;
        if c < 0 as i32 || c >= HU_FONTSIZE {
            w += 4 as i32;
        } else {
            w += (*hu_font[c as usize]).width as i32;
        }
    }
    return w;
}
pub unsafe fn M_StringHeight(string: &str) -> i32 {
    let mut h: i32 = 0;
    let height: i32 = (*hu_font[0 as i32 as usize])
        .height as i32;
    h = height;
    for b in string.bytes() {
        if b == b'\n' {
            h += height;
        }
    }
    return h;
}
pub unsafe fn M_WriteText(x: i32, y: i32, string: &str) {
    let mut w: i32 = 0;
    let mut c: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    cx = x;
    cy = y;
    'outer: for b in string.bytes() {
        c = b as i32;
        if c == '\n' as i32 {
            cx = x;
            cy += 12 as i32;
        } else {
            c = toupper(c) - HU_FONTSTART;
            if c < 0 as i32 || c >= HU_FONTSIZE {
                cx += 4 as i32;
            } else {
                w = (*hu_font[c as usize]).width as i32;
                if cx + w > SCREENWIDTH {
                    break 'outer;
                }
                V_DrawPatchDirect(cx, cy, hu_font[c as usize]);
                cx += w;
            }
        }
    };
}
unsafe extern "C" fn IsNullKey(mut key: i32) -> bool {
    return key == KEY_PAUSE || key == KEY_CAPSLOCK || key == KEY_SCRLCK
        || key == KEY_NUMLOCK;
}
pub unsafe fn M_Responder(mut ev: *mut event_t) -> bool {
    let mut ch: i32 = 0;
    let mut key: i32 = 0;
    let mut i: i32 = 0;
    static mut joywait: i32 = 0 as i32;
    static mut mousewait: i32 = 0 as i32;
    static mut mousey: i32 = 0 as i32;
    static mut lasty: i32 = 0 as i32;
    static mut mousex: i32 = 0 as i32;
    static mut lastx: i32 = 0 as i32;
    if testcontrols {
        if (*ev).type_0 as u32
            == ev_quit as i32 as u32
            || (*ev).type_0 as u32
                == ev_keydown as i32 as u32
                && ((*ev).data1 == key_menu_activate || (*ev).data1 == key_menu_quit)
        {
            I_Quit();
            return true;
        }
        return false;
    }
    if (*ev).type_0 as u32
        == ev_quit as i32 as u32
    {
        if menuactive && messageToPrint != 0
            && messageRoutine
                == Some(M_QuitResponse as unsafe extern "C" fn(i32) -> ())
        {
            M_QuitResponse(key_menu_confirm);
        } else {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuitDOOM(0 as i32);
        }
        return true;
    }
    ch = 0 as i32;
    key = -(1 as i32);
    if (*ev).type_0 as u32
        == ev_joystick as i32 as u32
        && joywait < I_GetTime()
    {
        if (*ev).data3 < 0 as i32 {
            key = key_menu_up;
            joywait = I_GetTime() + 5 as i32;
        } else if (*ev).data3 > 0 as i32 {
            key = key_menu_down;
            joywait = I_GetTime() + 5 as i32;
        }
        if (*ev).data2 < 0 as i32 {
            key = key_menu_left;
            joywait = I_GetTime() + 2 as i32;
        } else if (*ev).data2 > 0 as i32 {
            key = key_menu_right;
            joywait = I_GetTime() + 2 as i32;
        }
        if (*ev).data1 & 1 as i32 != 0 {
            key = key_menu_forward;
            joywait = I_GetTime() + 5 as i32;
        }
        if (*ev).data1 & 2 as i32 != 0 {
            key = key_menu_back;
            joywait = I_GetTime() + 5 as i32;
        }
        if joybmenu >= 0 as i32
            && (*ev).data1 & (1 as i32) << joybmenu
                != 0 as i32
        {
            key = key_menu_activate;
            joywait = I_GetTime() + 5 as i32;
        }
    } else if (*ev).type_0 as u32
        == ev_mouse as i32 as u32
        && mousewait < I_GetTime()
    {
        mousey += (*ev).data3;
        if mousey < lasty - 30 as i32 {
            key = key_menu_down;
            mousewait = I_GetTime() + 5 as i32;
            lasty -= 30 as i32;
            mousey = lasty;
        } else if mousey > lasty + 30 as i32 {
            key = key_menu_up;
            mousewait = I_GetTime() + 5 as i32;
            lasty += 30 as i32;
            mousey = lasty;
        }
        mousex += (*ev).data2;
        if mousex < lastx - 30 as i32 {
            key = key_menu_left;
            mousewait = I_GetTime() + 5 as i32;
            lastx -= 30 as i32;
            mousex = lastx;
        } else if mousex > lastx + 30 as i32 {
            key = key_menu_right;
            mousewait = I_GetTime() + 5 as i32;
            lastx += 30 as i32;
            mousex = lastx;
        }
        if (*ev).data1 & 1 as i32 != 0 {
            key = key_menu_forward;
            mousewait = I_GetTime() + 15 as i32;
        }
        if (*ev).data1 & 2 as i32 != 0 {
            key = key_menu_back;
            mousewait = I_GetTime() + 15 as i32;
        }
    } else if (*ev).type_0 as u32
        == ev_keydown as i32 as u32
    {
        key = (*ev).data1;
        ch = (*ev).data2;
    }
    if key == -(1 as i32) {
        return false;
    }
    if saveStringEnter != 0 {
        match key {
            KEY_BACKSPACE => {
                if saveCharIndex > 0 as i32 {
                    saveCharIndex -= 1;
                    savegamestrings[saveSlot as usize].truncate(saveCharIndex as usize);
                }
            }
            KEY_ESCAPE => {
                saveStringEnter = 0 as i32;
                savegamestrings[saveSlot as usize] = saveOldString.clone();
            }
            KEY_ENTER => {
                saveStringEnter = 0 as i32;
                if !savegamestrings[saveSlot as usize].is_empty() {
                    M_DoSave(saveSlot);
                }
            }
            _ => {
                if vanilla_keyboard_mapping != 0 {
                    ch = key;
                }
                ch = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<i32>() as usize > 1 as usize
                    {
                        if 0 != 0 {
                            let mut __c: i32 = ch;
                            __res = (if __c < -(128 as i32)
                                || __c > 255 as i32
                            {
                                __c as __int32_t
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            }) as i32;
                        } else {
                            __res = toupper(ch);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(ch as isize)
                            as i32;
                    }
                    __res
                });
                if !(ch != ' ' as i32
                    && (ch - HU_FONTSTART < 0 as i32
                        || ch - HU_FONTSTART >= HU_FONTSIZE))
                {
                    if ch >= 32 as i32 && ch <= 127 as i32
                        && saveCharIndex < SAVESTRINGSIZE - 1 as i32
                        && M_StringWidth(&savegamestrings[saveSlot as usize])
                            < (SAVESTRINGSIZE - 2 as i32)
                                * 8 as i32
                    {
                        saveCharIndex += 1;
                        savegamestrings[saveSlot as usize].push(ch as u8 as char);
                    }
                }
            }
        }
        return true;
    }
    if messageToPrint != 0 {
        if messageNeedsInput {
            if key != ' ' as i32 && key != KEY_ESCAPE && key != key_menu_confirm
                && key != key_menu_abort
            {
                return false;
            }
        }
        menuactive = messageLastMenuActive != 0;
        messageToPrint = 0 as i32;
        if messageRoutine.is_some() {
            messageRoutine.expect("non-null function pointer")(key);
        }
        menuactive = false;
        S_StartSound(NULL, sfx_swtchx as i32);
        return true;
    }
    if devparm && key == key_menu_help
        || key != 0 as i32 && key == key_menu_screenshot
    {
        G_ScreenShot();
        return true;
    }
    if !menuactive {
        if key == key_menu_decscreen {
            if automapactive || chat_on {
                return false;
            }
            M_SizeDisplay(0 as i32);
            S_StartSound(NULL, sfx_stnmov as i32);
            return true;
        } else if key == key_menu_incscreen {
            if automapactive || chat_on {
                return false;
            }
            M_SizeDisplay(1 as i32);
            S_StartSound(NULL, sfx_stnmov as i32);
            return true;
        } else if key == key_menu_help {
            M_StartControlPanel();
            if gamemode as u32
                == retail as i32 as u32
            {
                currentMenu = &raw mut ReadDef2;
            } else {
                currentMenu = &raw mut ReadDef1;
            }
            itemOn = 0 as i16;
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_save {
            M_StartControlPanel();
            S_StartSound(NULL, sfx_swtchn as i32);
            M_SaveGame(0 as i32);
            return true;
        } else if key == key_menu_load {
            M_StartControlPanel();
            S_StartSound(NULL, sfx_swtchn as i32);
            M_LoadGame(0 as i32);
            return true;
        } else if key == key_menu_volume {
            M_StartControlPanel();
            currentMenu = &raw mut SoundDef;
            itemOn = sfx_vol as i32 as i16;
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_detail {
            M_ChangeDetail(0 as i32);
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_qsave {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuickSave();
            return true;
        } else if key == key_menu_endgame {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_EndGame(0 as i32);
            return true;
        } else if key == key_menu_messages {
            M_ChangeMessages(0 as i32);
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        } else if key == key_menu_qload {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuickLoad();
            return true;
        } else if key == key_menu_quit {
            S_StartSound(NULL, sfx_swtchn as i32);
            M_QuitDOOM(0 as i32);
            return true;
        } else if key == key_menu_gamma {
            usegamma += 1;
            if usegamma > 4 as i32 {
                usegamma = 0 as i32;
            }
            players[consoleplayer as usize].message = gammamsg[usegamma as usize]
                .as_ptr() as *mut ::core::ffi::c_char;
            I_SetPalette(
                W_CacheLumpName("PLAYPAL",
                    PU_CACHE as i32,
                ) as *mut byte,
            );
            return true;
        }
    }
    if !menuactive {
        if key == key_menu_activate {
            M_StartControlPanel();
            S_StartSound(NULL, sfx_swtchn as i32);
            return true;
        }
        return false;
    }
    if key == key_menu_down {
        loop {
            if itemOn as i32 + 1 as i32
                > (*currentMenu).numitems as i32 - 1 as i32
            {
                itemOn = 0 as i16;
            } else {
                itemOn += 1;
            }
            S_StartSound(NULL, sfx_pstop as i32);
            if !((*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == -(1 as i32))
            {
                break;
            }
        }
        return true;
    } else if key == key_menu_up {
        loop {
            if itemOn == 0 {
                itemOn = ((*currentMenu).numitems as i32
                    - 1 as i32) as i16;
            } else {
                itemOn -= 1;
            }
            S_StartSound(NULL, sfx_pstop as i32);
            if !((*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == -(1 as i32))
            {
                break;
            }
        }
        return true;
    } else if key == key_menu_left {
        if (*(*currentMenu).menuitems.offset(itemOn as isize)).routine.is_some()
            && (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == 2 as i32
        {
            S_StartSound(NULL, sfx_stnmov as i32);
            (*(*currentMenu).menuitems.offset(itemOn as isize))
                .routine
                .expect("non-null function pointer")(0 as i32);
        }
        return true;
    } else if key == key_menu_right {
        if (*(*currentMenu).menuitems.offset(itemOn as isize)).routine.is_some()
            && (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == 2 as i32
        {
            S_StartSound(NULL, sfx_stnmov as i32);
            (*(*currentMenu).menuitems.offset(itemOn as isize))
                .routine
                .expect("non-null function pointer")(1 as i32);
        }
        return true;
    } else if key == key_menu_forward {
        if (*(*currentMenu).menuitems.offset(itemOn as isize)).routine.is_some()
            && (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 != 0
        {
            (*currentMenu).lastOn = itemOn;
            if (*(*currentMenu).menuitems.offset(itemOn as isize)).status
                as i32 == 2 as i32
            {
                (*(*currentMenu).menuitems.offset(itemOn as isize))
                    .routine
                    .expect("non-null function pointer")(1 as i32);
                S_StartSound(NULL, sfx_stnmov as i32);
            } else {
                (*(*currentMenu).menuitems.offset(itemOn as isize))
                    .routine
                    .expect("non-null function pointer")(itemOn as i32);
                S_StartSound(NULL, sfx_pistol as i32);
            }
        }
        return true;
    } else if key == key_menu_activate {
        (*currentMenu).lastOn = itemOn;
        M_ClearMenus();
        S_StartSound(NULL, sfx_swtchx as i32);
        return true;
    } else if key == key_menu_back {
        (*currentMenu).lastOn = itemOn;
        if !(*currentMenu).prevMenu.is_null() {
            currentMenu = (*currentMenu).prevMenu as *mut menu_t;
            itemOn = (*currentMenu).lastOn;
            S_StartSound(NULL, sfx_swtchn as i32);
        }
        return true;
    } else if ch != 0 as i32 || IsNullKey(key) {
        i = itemOn as i32 + 1 as i32;
        while i < (*currentMenu).numitems as i32 {
            if (*(*currentMenu).menuitems.offset(i as isize)).alphaKey
                as i32 == ch
            {
                itemOn = i as i16;
                S_StartSound(NULL, sfx_pstop as i32);
                return true;
            }
            i += 1;
        }
        i = 0 as i32;
        while i <= itemOn as i32 {
            if (*(*currentMenu).menuitems.offset(i as isize)).alphaKey
                as i32 == ch
            {
                itemOn = i as i16;
                S_StartSound(NULL, sfx_pstop as i32);
                return true;
            }
            i += 1;
        }
    }
    return false;
}
pub unsafe fn M_StartControlPanel() {
    if menuactive {
        return;
    }
    menuactive = true;
    currentMenu = &raw mut MainDef;
    itemOn = (*currentMenu).lastOn;
}
pub unsafe fn M_Drawer() {
    static mut x: i16 = 0;
    static mut y: i16 = 0;
    let mut i: u32 = 0;
    let mut max: u32 = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    inhelpscreens = false;
    if messageToPrint != 0 {
        y = (SCREENHEIGHT / 2 as i32
            - M_StringHeight(&messageString) / 2 as i32) as i16;
        for line in messageString.split('\n') {
            let line = if line.len() > 79 { &line[..79] } else { line };
            x = (SCREENWIDTH / 2 as i32
                - M_StringWidth(line) / 2 as i32) as i16;
            M_WriteText(x as i32, y as i32, line);
            y = (y as i32
                + (*hu_font[0 as i32 as usize]).height
                    as i32) as i16;
        }
        return;
    }
    if !menuactive {
        return;
    }
    if (*currentMenu).routine.is_some() {
        ::core::mem::transmute::<
            _,
            fn(),
        >((*currentMenu).routine.expect("non-null function pointer"))();
    }
    x = (*currentMenu).x;
    y = (*currentMenu).y;
    max = (*currentMenu).numitems as u32;
    i = 0 as u32;
    while i < max {
        name = &raw mut (*(*currentMenu).menuitems.offset(i as isize)).name
            as *mut ::core::ffi::c_char;
        if *name.offset(0 as i32 as isize) != 0 {
            V_DrawPatchDirect(
                x as i32,
                y as i32,
                W_CacheLumpName(
                    &wad_name8_to_string(name),
                    PU_CACHE as i32,
                ) as *mut patch_t,
            );
        }
        y = (y as i32 + LINEHEIGHT) as i16;
        i = i.wrapping_add(1);
    }
    V_DrawPatchDirect(
        x as i32 + SKULLXOFF,
        (*currentMenu).y as i32 - 5 as i32
            + itemOn as i32 * LINEHEIGHT,
        W_CacheLumpName(skullName[whichSkull as usize], PU_CACHE as i32)
            as *mut patch_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn M_ClearMenus() {
    menuactive = false;
}
#[no_mangle]
pub unsafe extern "C" fn M_SetupNextMenu(mut menudef: *mut menu_t) {
    currentMenu = menudef;
    itemOn = (*currentMenu).lastOn;
}
#[no_mangle]
pub unsafe extern "C" fn M_Ticker() {
    skullAnimCounter -= 1;
    if skullAnimCounter as i32 <= 0 as i32 {
        whichSkull = (whichSkull as i32 ^ 1 as i32)
            as i16;
        skullAnimCounter = 8 as i16;
    }
}
pub unsafe fn M_Init() {
    currentMenu = &raw mut MainDef;
    menuactive = false;
    itemOn = (*currentMenu).lastOn;
    whichSkull = 0 as i16;
    skullAnimCounter = 10 as i16;
    screenSize = screenblocks - 3 as i32;
    messageToPrint = 0 as i32;
    messageString = String::new();
    messageLastMenuActive = menuactive as i32;
    quickSaveSlot = -(1 as i32);
    match gamemode as u32 {
        2 => {
            MainMenu[readthis as i32 as usize] = MainMenu[quitdoom
                as i32 as usize];
            MainDef.numitems -= 1;
            MainDef.y = (MainDef.y as i32 + 8 as i32)
                as i16;
            NewDef.prevMenu = &raw mut MainDef as *mut menu_s;
        }
        0 => {}
        1 | 3 | _ => {}
    }
    if (gameversion as u32)
        < exe_ultimate as i32 as u32
    {
        EpiDef.numitems -= 1;
    }
}
