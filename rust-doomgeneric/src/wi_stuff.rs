use crate::src::hu_lib::patch_t;
use crate::src::d_event::event_t;
use crate::src::d_player::{player_t};
use crate::src::p_mobj::{actionf_t};
use crate::src::w_wad::{
    wad_name8_to_string, W_CacheLumpName, W_CheckNumForName, W_ReleaseLumpName,
};
use crate::src::g_game::G_WorldDone;
use crate::src::m_random::M_Random;
use crate::src::s_sound::S_ChangeMusic;
use crate::src::m_misc::M_StringCopy;
use crate::src::g_game::deathmatch;
use crate::src::g_game::playeringame;
use crate::src::g_game::netgame;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::s_sound::S_StartSound;
use crate::src::v_video::V_DrawPatch;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
use crate::src::sounds::{sfx_barexp, sfx_pistol, sfx_pldeth, sfx_sgcock, sfx_slop};
use crate::src::sounds::{mus_dm2int, mus_inter};
use crate::src::d_ticcmd::{BT_ATTACK, BT_USE};
use libc::{printf, snprintf};

pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type boolean = u32;
pub type byte = uint8_t;
pub type evtype_t = u32;
pub const ev_quit: evtype_t = 4;
pub const ev_joystick: evtype_t = 3;
pub const ev_mouse: evtype_t = 2;
pub const ev_keyup: evtype_t = 1;
pub const ev_keydown: evtype_t = 0;
pub type GameMode_t = u32;
pub const indetermined: GameMode_t = 4;
pub const retail: GameMode_t = 3;
pub const commercial: GameMode_t = 2;
pub const registered: GameMode_t = 1;
pub const shareware: GameMode_t = 0;
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
pub type actionf_v = Option<unsafe extern "C" fn() -> ()>;
pub type actionf_p1 = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type actionf_p2 = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
>;
pub type think_t = actionf_t;
pub type statenum_t = u32;
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
pub struct wbplayerstruct_t {
    pub in_0: bool,
    pub skills: i32,
    pub sitems: i32,
    pub ssecret: i32,
    pub stime: i32,
    pub frags: [i32; 4],
    pub score: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wbstartstruct_t {
    pub epsd: i32,
    pub didsecret: bool,
    pub last: i32,
    pub next: i32,
    pub maxkills: i32,
    pub maxitems: i32,
    pub maxsecret: i32,
    pub maxfrags: i32,
    pub partime: i32,
    pub pnum: i32,
    pub plyr: [wbplayerstruct_t; 4],
}
pub type stateenum_t = i32;
pub const ShowNextLoc: stateenum_t = 1;
pub const StatCount: stateenum_t = 0;
pub const NoState: stateenum_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct anim_t {
    pub type_0: animenum_t,
    pub period: i32,
    pub nanims: i32,
    pub loc: point_t,
    pub data1: i32,
    pub data2: i32,
    pub p: [*mut patch_t; 3],
    pub nexttic: i32,
    pub lastdrawn: i32,
    pub ctr: i32,
    pub state: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_t {
    pub x: i32,
    pub y: i32,
}
pub type animenum_t = u32;
pub const ANIM_LEVEL: animenum_t = 2;
pub const ANIM_RANDOM: animenum_t = 1;
pub const ANIM_ALWAYS: animenum_t = 0;
pub type load_callback_t = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_char, *mut *mut patch_t) -> (),
>;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TICRATE: i32 = 35 as i32;
pub const MAXPLAYERS: i32 = 4 as i32;
pub const SCREENWIDTH: i32 = 320 as i32;
pub const SCREENHEIGHT: i32 = 200 as i32;
pub const NUMMAPS: i32 = 9 as i32;
pub const WI_TITLEY: i32 = 2 as i32;
pub const WI_SPACINGY: i32 = 33 as i32;
pub const SP_STATSX: i32 = 50 as i32;
pub const SP_STATSY: i32 = 50 as i32;
pub const SP_TIMEX: i32 = 16 as i32;
pub const SP_TIMEY: i32 = SCREENHEIGHT - 32 as i32;
pub const NG_STATSY: i32 = 50 as i32;
pub const NG_SPACINGX: i32 = 64 as i32;
pub const DM_MATRIXX: i32 = 42 as i32;
pub const DM_MATRIXY: i32 = 68 as i32;
pub const DM_SPACINGX: i32 = 40 as i32;
pub const DM_TOTALSX: i32 = 269 as i32;
pub const DM_KILLERSX: i32 = 10 as i32;
pub const DM_KILLERSY: i32 = 100 as i32;
pub const DM_VICTIMSX: i32 = 5 as i32;
pub const DM_VICTIMSY: i32 = 50 as i32;
static mut lnodes: [[point_t; 9]; 4] = [
    [
        point_t {
            x: 185 as i32,
            y: 164 as i32,
        },
        point_t {
            x: 148 as i32,
            y: 143 as i32,
        },
        point_t {
            x: 69 as i32,
            y: 122 as i32,
        },
        point_t {
            x: 209 as i32,
            y: 102 as i32,
        },
        point_t {
            x: 116 as i32,
            y: 89 as i32,
        },
        point_t {
            x: 166 as i32,
            y: 55 as i32,
        },
        point_t {
            x: 71 as i32,
            y: 56 as i32,
        },
        point_t {
            x: 135 as i32,
            y: 29 as i32,
        },
        point_t {
            x: 71 as i32,
            y: 24 as i32,
        },
    ],
    [
        point_t {
            x: 254 as i32,
            y: 25 as i32,
        },
        point_t {
            x: 97 as i32,
            y: 50 as i32,
        },
        point_t {
            x: 188 as i32,
            y: 64 as i32,
        },
        point_t {
            x: 128 as i32,
            y: 78 as i32,
        },
        point_t {
            x: 214 as i32,
            y: 92 as i32,
        },
        point_t {
            x: 133 as i32,
            y: 130 as i32,
        },
        point_t {
            x: 208 as i32,
            y: 136 as i32,
        },
        point_t {
            x: 148 as i32,
            y: 140 as i32,
        },
        point_t {
            x: 235 as i32,
            y: 158 as i32,
        },
    ],
    [
        point_t {
            x: 156 as i32,
            y: 168 as i32,
        },
        point_t {
            x: 48 as i32,
            y: 154 as i32,
        },
        point_t {
            x: 174 as i32,
            y: 95 as i32,
        },
        point_t {
            x: 265 as i32,
            y: 75 as i32,
        },
        point_t {
            x: 130 as i32,
            y: 48 as i32,
        },
        point_t {
            x: 279 as i32,
            y: 23 as i32,
        },
        point_t {
            x: 198 as i32,
            y: 48 as i32,
        },
        point_t {
            x: 140 as i32,
            y: 25 as i32,
        },
        point_t {
            x: 281 as i32,
            y: 136 as i32,
        },
    ],
    [point_t { x: 0, y: 0 }; 9],
];
static mut epsd0animinfo: [anim_t; 10] = [
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 224 as i32,
            y: 104 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 184 as i32,
            y: 160 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 112 as i32,
            y: 136 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 72 as i32,
            y: 112 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 88 as i32,
            y: 96 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 64 as i32,
            y: 48 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 192 as i32,
            y: 40 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 136 as i32,
            y: 16 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 80 as i32,
            y: 16 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 64 as i32,
            y: 24 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
];
static mut epsd1animinfo: [anim_t; 9] = [
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 1 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 2 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 3 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 4 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 5 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 6 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 7 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 192 as i32,
            y: 144 as i32,
        },
        data1: 8 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_LEVEL,
        period: 35 as i32 / 3 as i32,
        nanims: 1 as i32,
        loc: point_t {
            x: 128 as i32,
            y: 136 as i32,
        },
        data1: 8 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
];
static mut epsd2animinfo: [anim_t; 6] = [
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 104 as i32,
            y: 168 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 40 as i32,
            y: 136 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 160 as i32,
            y: 96 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 104 as i32,
            y: 80 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 3 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 120 as i32,
            y: 32 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
    anim_t {
        type_0: ANIM_ALWAYS,
        period: 35 as i32 / 4 as i32,
        nanims: 3 as i32,
        loc: point_t {
            x: 40 as i32,
            y: 0 as i32,
        },
        data1: 0 as i32,
        data2: 0 as i32,
        p: [
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
            ::core::ptr::null::<patch_t>() as *mut patch_t,
        ],
        nexttic: 0 as i32,
        lastdrawn: 0 as i32,
        ctr: 0 as i32,
        state: 0 as i32,
    },
];
static mut NUMANIMS: [i32; 4] = [0; 4];
static mut anims: [*mut anim_t; 4] = unsafe {
    [
        &raw const epsd0animinfo as *mut anim_t,
        &raw const epsd1animinfo as *mut anim_t,
        &raw const epsd2animinfo as *mut anim_t,
        ::core::ptr::null::<anim_t>() as *mut anim_t,
    ]
};
pub const SHOWNEXTLOCDELAY: i32 = 4 as i32;
static mut acceleratestage: i32 = 0;
static mut me: i32 = 0;
static mut state: stateenum_t = StatCount;
static mut wbs: *mut wbstartstruct_t = ::core::ptr::null::<wbstartstruct_t>()
    as *mut wbstartstruct_t;
static mut plrs: *mut wbplayerstruct_t = ::core::ptr::null::<wbplayerstruct_t>()
    as *mut wbplayerstruct_t;
static mut cnt: i32 = 0;
static mut bcnt: i32 = 0;
static mut firstrefresh: i32 = 0;
static mut cnt_kills: [i32; 4] = [0; 4];
static mut cnt_items: [i32; 4] = [0; 4];
static mut cnt_secret: [i32; 4] = [0; 4];
static mut cnt_time: i32 = 0;
static mut cnt_par: i32 = 0;
static mut cnt_pause: i32 = 0;
static mut NUMCMAPS: i32 = 0;
static mut yah: [*mut patch_t; 3] = [
    ::core::ptr::null::<patch_t>() as *mut patch_t,
    ::core::ptr::null::<patch_t>() as *mut patch_t,
    ::core::ptr::null::<patch_t>() as *mut patch_t,
];
static mut splat: [*mut patch_t; 2] = [
    ::core::ptr::null::<patch_t>() as *mut patch_t,
    ::core::ptr::null::<patch_t>() as *mut patch_t,
];
static mut percent: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut colon: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut num: [*mut patch_t; 10] = [::core::ptr::null::<patch_t>()
    as *mut patch_t; 10];
static mut wiminus: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut finished: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut entering: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut sp_secret: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut kills: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut secret: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut items: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut frags: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut timepatch: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut par: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut sucks: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut killers: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut victims: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut total: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut star: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut bstar: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
static mut p: [*mut patch_t; 4] = [::core::ptr::null::<patch_t>() as *mut patch_t; 4];
static mut bp: [*mut patch_t; 4] = [::core::ptr::null::<patch_t>() as *mut patch_t; 4];
static mut lnames: *mut *mut patch_t = ::core::ptr::null::<*mut patch_t>()
    as *mut *mut patch_t;
static mut background: *mut patch_t = ::core::ptr::null::<patch_t>() as *mut patch_t;
#[no_mangle]
pub unsafe extern "C" fn WI_slamBackground() {
    V_DrawPatch(0 as i32, 0 as i32, background);
}
#[no_mangle]
pub unsafe extern "C" fn WI_Responder(mut ev: *mut event_t) -> bool {
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawLF() {
    let mut y: i32 = WI_TITLEY;
    if gamemode as u32
        != commercial as i32 as u32
        || (*wbs).last < NUMCMAPS
    {
        V_DrawPatch(
            (SCREENWIDTH
                - (**lnames.offset((*wbs).last as isize)).width as i32)
                / 2 as i32,
            y,
            *lnames.offset((*wbs).last as isize),
        );
        y
            += 5 as i32
                * (**lnames.offset((*wbs).last as isize)).height as i32
                / 4 as i32;
        V_DrawPatch(
            (SCREENWIDTH - (*finished).width as i32)
                / 2 as i32,
            y,
            finished,
        );
    } else if !((*wbs).last == NUMCMAPS) {
        if (*wbs).last > NUMCMAPS {
            let mut tmp: patch_t = patch_t {
                width: SCREENWIDTH as i16,
                height: SCREENHEIGHT as i16,
                leftoffset: 1 as i16,
                topoffset: 1 as i16,
                columnofs: [
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                ],
            };
            V_DrawPatch(0 as i32, y, &raw mut tmp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawEL() {
    let mut y: i32 = WI_TITLEY;
    V_DrawPatch(
        (SCREENWIDTH - (*entering).width as i32)
            / 2 as i32,
        y,
        entering,
    );
    y
        += 5 as i32
            * (**lnames.offset((*wbs).next as isize)).height as i32
            / 4 as i32;
    V_DrawPatch(
        (SCREENWIDTH
            - (**lnames.offset((*wbs).next as isize)).width as i32)
            / 2 as i32,
        y,
        *lnames.offset((*wbs).next as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawOnLnode(
    mut n: i32,
    mut c: *mut *mut patch_t,
) {
    let mut i: i32 = 0;
    let mut left: i32 = 0;
    let mut top: i32 = 0;
    let mut right: i32 = 0;
    let mut bottom: i32 = 0;
    let mut fits: boolean = false_0 as boolean;
    i = 0 as i32;
    loop {
        left = lnodes[(*wbs).epsd as usize][n as usize].x
            - (**c.offset(i as isize)).leftoffset as i32;
        top = lnodes[(*wbs).epsd as usize][n as usize].y
            - (**c.offset(i as isize)).topoffset as i32;
        right = left + (**c.offset(i as isize)).width as i32;
        bottom = top + (**c.offset(i as isize)).height as i32;
        if left >= 0 as i32 && right < SCREENWIDTH
            && top >= 0 as i32 && bottom < SCREENHEIGHT
        {
            fits = true_0 as boolean;
        } else {
            i += 1;
        }
        if !(fits == 0 && i != 2 as i32
            && !(*c.offset(i as isize)).is_null())
        {
            break;
        }
    }
    if fits != 0 && i < 2 as i32 {
        V_DrawPatch(
            lnodes[(*wbs).epsd as usize][n as usize].x,
            lnodes[(*wbs).epsd as usize][n as usize].y,
            *c.offset(i as isize),
        );
    } else {
        printf(
            b"Could not place patch on level %d\0" as *const u8
                as *const ::core::ffi::c_char,
            n + 1 as i32,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn WI_initAnimatedBack() {
    let mut i: i32 = 0;
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if (*wbs).epsd > 2 as i32 {
        return;
    }
    i = 0 as i32;
    while i < NUMANIMS[(*wbs).epsd as usize] {
        a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
            .offset(i as isize) as *mut anim_t;
        (*a).ctr = -(1 as i32);
        if (*a).type_0 as u32
            == ANIM_ALWAYS as i32 as u32
        {
            (*a).nexttic = bcnt + 1 as i32 + M_Random() % (*a).period;
        } else if (*a).type_0 as u32
            == ANIM_RANDOM as i32 as u32
        {
            (*a).nexttic = bcnt + 1 as i32 + (*a).data2
                + M_Random() % (*a).data1;
        } else if (*a).type_0 as u32
            == ANIM_LEVEL as i32 as u32
        {
            (*a).nexttic = bcnt + 1 as i32;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateAnimatedBack() {
    let mut i: i32 = 0;
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if (*wbs).epsd > 2 as i32 {
        return;
    }
    i = 0 as i32;
    while i < NUMANIMS[(*wbs).epsd as usize] {
        a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
            .offset(i as isize) as *mut anim_t;
        if bcnt == (*a).nexttic {
            match (*a).type_0 as u32 {
                0 => {
                    (*a).ctr += 1;
                    if (*a).ctr >= (*a).nanims {
                        (*a).ctr = 0 as i32;
                    }
                    (*a).nexttic = bcnt + (*a).period;
                }
                1 => {
                    (*a).ctr += 1;
                    if (*a).ctr == (*a).nanims {
                        (*a).ctr = -(1 as i32);
                        (*a).nexttic = bcnt + (*a).data2 + M_Random() % (*a).data1;
                    } else {
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                2 => {
                    if !(state as i32 == StatCount as i32
                        && i == 7 as i32) && (*wbs).next == (*a).data1
                    {
                        (*a).ctr += 1;
                        if (*a).ctr == (*a).nanims {
                            (*a).ctr -= 1;
                        }
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawAnimatedBack() {
    let mut i: i32 = 0;
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        return;
    }
    if (*wbs).epsd > 2 as i32 {
        return;
    }
    i = 0 as i32;
    while i < NUMANIMS[(*wbs).epsd as usize] {
        a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
            .offset(i as isize) as *mut anim_t;
        if (*a).ctr >= 0 as i32 {
            V_DrawPatch((*a).loc.x, (*a).loc.y, (*a).p[(*a).ctr as usize]);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawNum(
    mut x: i32,
    mut y: i32,
    mut n: i32,
    mut digits: i32,
) -> i32 {
    let mut fontwidth: i32 = (*num[0 as i32 as usize])
        .width as i32;
    let mut neg: i32 = 0;
    let mut temp: i32 = 0;
    if digits < 0 as i32 {
        if n == 0 {
            digits = 1 as i32;
        } else {
            digits = 0 as i32;
            temp = n;
            while temp != 0 {
                temp /= 10 as i32;
                digits += 1;
            }
        }
    }
    neg = (n < 0 as i32) as i32;
    if neg != 0 {
        n = -n;
    }
    if n == 1994 as i32 {
        return 0 as i32;
    }
    loop {
        let fresh0 = digits;
        digits = digits - 1;
        if !(fresh0 != 0) {
            break;
        }
        x -= fontwidth;
        V_DrawPatch(x, y, num[(n % 10 as i32) as usize]);
        n /= 10 as i32;
    }
    if neg != 0 {
        x -= 8 as i32;
        V_DrawPatch(x, y, wiminus);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawPercent(
    mut x: i32,
    mut y: i32,
    mut p_0: i32,
) {
    if p_0 < 0 as i32 {
        return;
    }
    V_DrawPatch(x, y, percent);
    WI_drawNum(x, y, p_0, -(1 as i32));
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawTime(
    mut x: i32,
    mut y: i32,
    mut t: i32,
) {
    let mut div: i32 = 0;
    let mut n: i32 = 0;
    if t < 0 as i32 {
        return;
    }
    if t <= 61 as i32 * 59 as i32 {
        div = 1 as i32;
        loop {
            n = t / div % 60 as i32;
            x = WI_drawNum(x, y, n, 2 as i32)
                - (*colon).width as i32;
            div *= 60 as i32;
            if div == 60 as i32 || t / div != 0 {
                V_DrawPatch(x, y, colon);
            }
            if !(t / div != 0) {
                break;
            }
        }
    } else {
        V_DrawPatch(x - (*sucks).width as i32, y, sucks);
    };
}
pub unsafe fn WI_End() {
    #[export_name = "WI_unloadData"]
    pub unsafe extern "C" fn WI_unloadData_0() {
        WI_loadUnloadData(
            Some(
                WI_unloadCallback
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut *mut patch_t,
                    ) -> (),
            ),
        );
    }
    WI_unloadData_0();
}
#[no_mangle]
pub unsafe extern "C" fn WI_initNoState() {
    state = NoState;
    acceleratestage = 0 as i32;
    cnt = 10 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateNoState() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 {
        G_WorldDone();
    }
}
static mut snl_pointeron: bool = false;
#[no_mangle]
pub unsafe extern "C" fn WI_initShowNextLoc() {
    state = ShowNextLoc;
    acceleratestage = 0 as i32;
    cnt = SHOWNEXTLOCDELAY * TICRATE;
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateShowNextLoc() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 || acceleratestage != 0 {
        WI_initNoState();
    } else {
        snl_pointeron = (cnt & 31 as i32) < 20 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawShowNextLoc() {
    let mut i: i32 = 0;
    let mut last: i32 = 0;
    WI_slamBackground();
    WI_drawAnimatedBack();
    if gamemode as u32
        != commercial as i32 as u32
    {
        if (*wbs).epsd > 2 as i32 {
            WI_drawEL();
            return;
        }
        last = if (*wbs).last == 8 as i32 {
            (*wbs).next - 1 as i32
        } else {
            (*wbs).last
        };
        i = 0 as i32;
        while i <= last {
            WI_drawOnLnode(i, &raw mut splat as *mut *mut patch_t);
            i += 1;
        }
        if (*wbs).didsecret {
            WI_drawOnLnode(8 as i32, &raw mut splat as *mut *mut patch_t);
        }
        if snl_pointeron {
            WI_drawOnLnode((*wbs).next, &raw mut yah as *mut *mut patch_t);
        }
    }
    if gamemode as u32
        != commercial as i32 as u32
        || (*wbs).next != 30 as i32
    {
        WI_drawEL();
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawNoState() {
    snl_pointeron = true;
    WI_drawShowNextLoc();
}
#[no_mangle]
pub unsafe extern "C" fn WI_fragSum(
    mut playernum: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut frags_0: i32 = 0 as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 && i != playernum {
            frags_0 += (*plrs.offset(playernum as isize)).frags[i as usize];
        }
        i += 1;
    }
    frags_0 -= (*plrs.offset(playernum as isize)).frags[playernum as usize];
    return frags_0;
}
static mut dm_state: i32 = 0;
static mut dm_frags: [[i32; 4]; 4] = [[0; 4]; 4];
static mut dm_totals: [i32; 4] = [0; 4];
#[no_mangle]
pub unsafe extern "C" fn WI_initDeathmatchStats() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    state = StatCount;
    acceleratestage = 0 as i32;
    dm_state = 1 as i32;
    cnt_pause = TICRATE;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            j = 0 as i32;
            while j < MAXPLAYERS {
                if playeringame[j as usize] != 0 {
                    dm_frags[i as usize][j as usize] = 0 as i32;
                }
                j += 1;
            }
            dm_totals[i as usize] = 0 as i32;
        }
        i += 1;
    }
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateDeathmatchStats() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut stillticking: boolean = 0;
    WI_updateAnimatedBack();
    if acceleratestage != 0 && dm_state != 4 as i32 {
        acceleratestage = 0 as i32;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if playeringame[i as usize] != 0 {
                j = 0 as i32;
                while j < MAXPLAYERS {
                    if playeringame[j as usize] != 0 {
                        dm_frags[i as usize][j as usize] = (*plrs.offset(i as isize))
                            .frags[j as usize];
                    }
                    j += 1;
                }
                dm_totals[i as usize] = WI_fragSum(i);
            }
            i += 1;
        }
        S_StartSound(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            sfx_barexp as i32,
        );
        dm_state = 4 as i32;
    }
    if dm_state == 2 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false_0 as boolean;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if playeringame[i as usize] != 0 {
                j = 0 as i32;
                while j < MAXPLAYERS {
                    if playeringame[j as usize] != 0
                        && dm_frags[i as usize][j as usize]
                            != (*plrs.offset(i as isize)).frags[j as usize]
                    {
                        if (*plrs.offset(i as isize)).frags[j as usize]
                            < 0 as i32
                        {
                            dm_frags[i as usize][j as usize] -= 1;
                        } else {
                            dm_frags[i as usize][j as usize] += 1;
                        }
                        if dm_frags[i as usize][j as usize] > 99 as i32 {
                            dm_frags[i as usize][j as usize] = 99 as i32;
                        }
                        if dm_frags[i as usize][j as usize] < -(99 as i32)
                        {
                            dm_frags[i as usize][j as usize] = -(99
                                as i32);
                        }
                        stillticking = true_0 as boolean;
                    }
                    j += 1;
                }
                dm_totals[i as usize] = WI_fragSum(i);
                if dm_totals[i as usize] > 99 as i32 {
                    dm_totals[i as usize] = 99 as i32;
                }
                if dm_totals[i as usize] < -(99 as i32) {
                    dm_totals[i as usize] = -(99 as i32);
                }
            }
            i += 1;
        }
        if stillticking == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            dm_state += 1;
        }
    } else if dm_state == 4 as i32 {
        if acceleratestage != 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_slop as i32,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if dm_state & 1 as i32 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            dm_state += 1;
            cnt_pause = TICRATE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawDeathmatchStats() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();
    V_DrawPatch(
        DM_TOTALSX - (*total).width as i32 / 2 as i32,
        DM_MATRIXY - WI_SPACINGY + 10 as i32,
        total,
    );
    V_DrawPatch(DM_KILLERSX, DM_KILLERSY, killers);
    V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, victims);
    x = DM_MATRIXX + DM_SPACINGX;
    y = DM_MATRIXY;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            V_DrawPatch(
                x
                    - (*p[i as usize]).width as i32
                        / 2 as i32,
                DM_MATRIXY - WI_SPACINGY,
                p[i as usize],
            );
            V_DrawPatch(
                DM_MATRIXX
                    - (*p[i as usize]).width as i32
                        / 2 as i32,
                y,
                p[i as usize],
            );
            if i == me {
                V_DrawPatch(
                    x
                        - (*p[i as usize]).width as i32
                            / 2 as i32,
                    DM_MATRIXY - WI_SPACINGY,
                    bstar,
                );
                V_DrawPatch(
                    DM_MATRIXX
                        - (*p[i as usize]).width as i32
                            / 2 as i32,
                    y,
                    star,
                );
            }
        }
        x += DM_SPACINGX;
        y += WI_SPACINGY;
        i += 1;
    }
    y = DM_MATRIXY + 10 as i32;
    w = (*num[0 as i32 as usize]).width as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        x = DM_MATRIXX + DM_SPACINGX;
        if playeringame[i as usize] != 0 {
            j = 0 as i32;
            while j < MAXPLAYERS {
                if playeringame[j as usize] != 0 {
                    WI_drawNum(
                        x + w,
                        y,
                        dm_frags[i as usize][j as usize],
                        2 as i32,
                    );
                }
                x += DM_SPACINGX;
                j += 1;
            }
            WI_drawNum(
                DM_TOTALSX + w,
                y,
                dm_totals[i as usize],
                2 as i32,
            );
        }
        y += WI_SPACINGY;
        i += 1;
    }
}
static mut cnt_frags: [i32; 4] = [0; 4];
static mut dofrags: i32 = 0;
static mut ng_state: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn WI_initNetgameStats() {
    let mut i: i32 = 0;
    state = StatCount;
    acceleratestage = 0 as i32;
    ng_state = 1 as i32;
    cnt_pause = TICRATE;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            cnt_frags[i as usize] = 0 as i32;
            cnt_secret[i as usize] = cnt_frags[i as usize];
            cnt_items[i as usize] = cnt_secret[i as usize];
            cnt_kills[i as usize] = cnt_items[i as usize];
            dofrags += WI_fragSum(i);
        }
        i += 1;
    }
    dofrags = (dofrags != 0) as i32;
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateNetgameStats() {
    let mut i: i32 = 0;
    let mut fsum: i32 = 0;
    let mut stillticking: boolean = 0;
    WI_updateAnimatedBack();
    if acceleratestage != 0 && ng_state != 10 as i32 {
        acceleratestage = 0 as i32;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_kills[i as usize] = (*plrs.offset(i as isize)).skills
                    * 100 as i32 / (*wbs).maxkills;
                cnt_items[i as usize] = (*plrs.offset(i as isize)).sitems
                    * 100 as i32 / (*wbs).maxitems;
                cnt_secret[i as usize] = (*plrs.offset(i as isize)).ssecret
                    * 100 as i32 / (*wbs).maxsecret;
                if dofrags != 0 {
                    cnt_frags[i as usize] = WI_fragSum(i);
                }
            }
            i += 1;
        }
        S_StartSound(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            sfx_barexp as i32,
        );
        ng_state = 10 as i32;
    }
    if ng_state == 2 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false_0 as boolean;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_kills[i as usize] += 2 as i32;
                if cnt_kills[i as usize]
                    >= (*plrs.offset(i as isize)).skills * 100 as i32
                        / (*wbs).maxkills
                {
                    cnt_kills[i as usize] = (*plrs.offset(i as isize)).skills
                        * 100 as i32 / (*wbs).maxkills;
                } else {
                    stillticking = true_0 as boolean;
                }
            }
            i += 1;
        }
        if stillticking == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            ng_state += 1;
        }
    } else if ng_state == 4 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false_0 as boolean;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_items[i as usize] += 2 as i32;
                if cnt_items[i as usize]
                    >= (*plrs.offset(i as isize)).sitems * 100 as i32
                        / (*wbs).maxitems
                {
                    cnt_items[i as usize] = (*plrs.offset(i as isize)).sitems
                        * 100 as i32 / (*wbs).maxitems;
                } else {
                    stillticking = true_0 as boolean;
                }
            }
            i += 1;
        }
        if stillticking == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            ng_state += 1;
        }
    } else if ng_state == 6 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false_0 as boolean;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_secret[i as usize] += 2 as i32;
                if cnt_secret[i as usize]
                    >= (*plrs.offset(i as isize)).ssecret * 100 as i32
                        / (*wbs).maxsecret
                {
                    cnt_secret[i as usize] = (*plrs.offset(i as isize)).ssecret
                        * 100 as i32 / (*wbs).maxsecret;
                } else {
                    stillticking = true_0 as boolean;
                }
            }
            i += 1;
        }
        if stillticking == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            ng_state
                += 1 as i32
                    + 2 as i32 * (dofrags == 0) as i32;
        }
    } else if ng_state == 8 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        stillticking = false_0 as boolean;
        i = 0 as i32;
        while i < MAXPLAYERS {
            if !(playeringame[i as usize] == 0) {
                cnt_frags[i as usize] += 1 as i32;
                fsum = WI_fragSum(i);
                if cnt_frags[i as usize] >= fsum {
                    cnt_frags[i as usize] = fsum;
                } else {
                    stillticking = true_0 as boolean;
                }
            }
            i += 1;
        }
        if stillticking == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pldeth as i32,
            );
            ng_state += 1;
        }
    } else if ng_state == 10 as i32 {
        if acceleratestage != 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_sgcock as i32,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if ng_state & 1 as i32 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            ng_state += 1;
            cnt_pause = TICRATE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawNetgameStats() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut pwidth: i32 = (*percent).width as i32;
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();
    V_DrawPatch(
        32 as i32
            + (*star).width as i32 / 2 as i32
            + 32 as i32 * (dofrags == 0) as i32
            + NG_SPACINGX - (*kills).width as i32,
        NG_STATSY,
        kills,
    );
    V_DrawPatch(
        32 as i32
            + (*star).width as i32 / 2 as i32
            + 32 as i32 * (dofrags == 0) as i32
            + 2 as i32 * NG_SPACINGX
            - (*items).width as i32,
        NG_STATSY,
        items,
    );
    V_DrawPatch(
        32 as i32
            + (*star).width as i32 / 2 as i32
            + 32 as i32 * (dofrags == 0) as i32
            + 3 as i32 * NG_SPACINGX
            - (*secret).width as i32,
        NG_STATSY,
        secret,
    );
    if dofrags != 0 {
        V_DrawPatch(
            32 as i32
                + (*star).width as i32 / 2 as i32
                + 32 as i32 * (dofrags == 0) as i32
                + 4 as i32 * NG_SPACINGX
                - (*frags).width as i32,
            NG_STATSY,
            frags,
        );
    }
    y = NG_STATSY + (*kills).height as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            x = 32 as i32
                + (*star).width as i32 / 2 as i32
                + 32 as i32 * (dofrags == 0) as i32;
            V_DrawPatch(
                x - (*p[i as usize]).width as i32,
                y,
                p[i as usize],
            );
            if i == me {
                V_DrawPatch(x - (*p[i as usize]).width as i32, y, star);
            }
            x += NG_SPACINGX;
            WI_drawPercent(
                x - pwidth,
                y + 10 as i32,
                cnt_kills[i as usize],
            );
            x += NG_SPACINGX;
            WI_drawPercent(
                x - pwidth,
                y + 10 as i32,
                cnt_items[i as usize],
            );
            x += NG_SPACINGX;
            WI_drawPercent(
                x - pwidth,
                y + 10 as i32,
                cnt_secret[i as usize],
            );
            x += NG_SPACINGX;
            if dofrags != 0 {
                WI_drawNum(
                    x,
                    y + 10 as i32,
                    cnt_frags[i as usize],
                    -(1 as i32),
                );
            }
            y += WI_SPACINGY;
        }
        i += 1;
    }
}
static mut sp_state: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn WI_initStats() {
    state = StatCount;
    acceleratestage = 0 as i32;
    sp_state = 1 as i32;
    cnt_secret[0 as i32 as usize] = -(1 as i32);
    cnt_items[0 as i32 as usize] = cnt_secret[0 as i32
        as usize];
    cnt_kills[0 as i32 as usize] = cnt_items[0 as i32
        as usize];
    cnt_par = -(1 as i32);
    cnt_time = cnt_par;
    cnt_pause = TICRATE;
    WI_initAnimatedBack();
}
#[no_mangle]
pub unsafe extern "C" fn WI_updateStats() {
    WI_updateAnimatedBack();
    if acceleratestage != 0 && sp_state != 10 as i32 {
        acceleratestage = 0 as i32;
        cnt_kills[0 as i32 as usize] = (*plrs.offset(me as isize)).skills
            * 100 as i32 / (*wbs).maxkills;
        cnt_items[0 as i32 as usize] = (*plrs.offset(me as isize)).sitems
            * 100 as i32 / (*wbs).maxitems;
        cnt_secret[0 as i32 as usize] = (*plrs.offset(me as isize))
            .ssecret * 100 as i32 / (*wbs).maxsecret;
        cnt_time = (*plrs.offset(me as isize)).stime / TICRATE;
        cnt_par = (*wbs).partime / TICRATE;
        S_StartSound(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            sfx_barexp as i32,
        );
        sp_state = 10 as i32;
    }
    if sp_state == 2 as i32 {
        cnt_kills[0 as i32 as usize] += 2 as i32;
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        if cnt_kills[0 as i32 as usize]
            >= (*plrs.offset(me as isize)).skills * 100 as i32
                / (*wbs).maxkills
        {
            cnt_kills[0 as i32 as usize] = (*plrs.offset(me as isize))
                .skills * 100 as i32 / (*wbs).maxkills;
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            sp_state += 1;
        }
    } else if sp_state == 4 as i32 {
        cnt_items[0 as i32 as usize] += 2 as i32;
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        if cnt_items[0 as i32 as usize]
            >= (*plrs.offset(me as isize)).sitems * 100 as i32
                / (*wbs).maxitems
        {
            cnt_items[0 as i32 as usize] = (*plrs.offset(me as isize))
                .sitems * 100 as i32 / (*wbs).maxitems;
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            sp_state += 1;
        }
    } else if sp_state == 6 as i32 {
        cnt_secret[0 as i32 as usize] += 2 as i32;
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        if cnt_secret[0 as i32 as usize]
            >= (*plrs.offset(me as isize)).ssecret * 100 as i32
                / (*wbs).maxsecret
        {
            cnt_secret[0 as i32 as usize] = (*plrs.offset(me as isize))
                .ssecret * 100 as i32 / (*wbs).maxsecret;
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_barexp as i32,
            );
            sp_state += 1;
        }
    } else if sp_state == 8 as i32 {
        if bcnt & 3 as i32 == 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_pistol as i32,
            );
        }
        cnt_time += 3 as i32;
        if cnt_time >= (*plrs.offset(me as isize)).stime / TICRATE {
            cnt_time = (*plrs.offset(me as isize)).stime / TICRATE;
        }
        cnt_par += 3 as i32;
        if cnt_par >= (*wbs).partime / TICRATE {
            cnt_par = (*wbs).partime / TICRATE;
            if cnt_time >= (*plrs.offset(me as isize)).stime / TICRATE {
                S_StartSound(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    sfx_barexp as i32,
                );
                sp_state += 1;
            }
        }
    } else if sp_state == 10 as i32 {
        if acceleratestage != 0 {
            S_StartSound(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sfx_sgcock as i32,
            );
            if gamemode as u32
                == commercial as i32 as u32
            {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if sp_state & 1 as i32 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            sp_state += 1;
            cnt_pause = TICRATE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_drawStats() {
    let mut lh: i32 = 0;
    lh = 3 as i32
        * (*num[0 as i32 as usize]).height as i32
        / 2 as i32;
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();
    V_DrawPatch(SP_STATSX, SP_STATSY, kills);
    WI_drawPercent(
        SCREENWIDTH - SP_STATSX,
        SP_STATSY,
        cnt_kills[0 as i32 as usize],
    );
    V_DrawPatch(SP_STATSX, SP_STATSY + lh, items);
    WI_drawPercent(
        SCREENWIDTH - SP_STATSX,
        SP_STATSY + lh,
        cnt_items[0 as i32 as usize],
    );
    V_DrawPatch(SP_STATSX, SP_STATSY + 2 as i32 * lh, sp_secret);
    WI_drawPercent(
        SCREENWIDTH - SP_STATSX,
        SP_STATSY + 2 as i32 * lh,
        cnt_secret[0 as i32 as usize],
    );
    V_DrawPatch(SP_TIMEX, SP_TIMEY, timepatch);
    WI_drawTime(SCREENWIDTH / 2 as i32 - SP_TIMEX, SP_TIMEY, cnt_time);
    if (*wbs).epsd < 3 as i32 {
        V_DrawPatch(SCREENWIDTH / 2 as i32 + SP_TIMEX, SP_TIMEY, par);
        WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WI_checkForAccelerate() {
    let mut i: i32 = 0;
    let mut player: *mut player_t = ::core::ptr::null_mut::<player_t>();
    i = 0 as i32;
    player = &raw mut players as *mut player_t;
    while i < MAXPLAYERS {
        if playeringame[i as usize] != 0 {
            if (*player).cmd.buttons as i32
                & BT_ATTACK as i32 != 0
            {
                if (*player).attackdown == 0 {
                    acceleratestage = 1 as i32;
                }
                (*player).attackdown = true_0;
            } else {
                (*player).attackdown = false_0;
            }
            if (*player).cmd.buttons as i32 & BT_USE as i32
                != 0
            {
                if (*player).usedown == 0 {
                    acceleratestage = 1 as i32;
                }
                (*player).usedown = true_0;
            } else {
                (*player).usedown = false_0;
            }
        }
        i += 1;
        player = player.offset(1);
    }
}
pub unsafe fn WI_Ticker() {
    bcnt += 1;
    if bcnt == 1 as i32 {
        if gamemode as u32
            == commercial as i32 as u32
        {
            S_ChangeMusic(mus_dm2int as i32, true_0);
        } else {
            S_ChangeMusic(mus_inter as i32, true_0);
        }
    }
    WI_checkForAccelerate();
    match state as i32 {
        0 => {
            if deathmatch != 0 {
                WI_updateDeathmatchStats();
            } else if netgame {
                WI_updateNetgameStats();
            } else {
                WI_updateStats();
            }
        }
        1 => {
            WI_updateShowNextLoc();
        }
        -1 => {
            WI_updateNoState();
        }
        _ => {}
    };
}
unsafe extern "C" fn WI_loadUnloadData(mut callback: load_callback_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut name: [::core::ffi::c_char; 9] = [0; 9];
    let mut a: *mut anim_t = ::core::ptr::null_mut::<anim_t>();
    if gamemode as u32
        == commercial as i32 as u32
    {
        i = 0 as i32;
        while i < NUMCMAPS {
            snprintf(
                &raw mut name as *mut ::core::ffi::c_char,
                9 as size_t,
                b"CWILV%2.2d\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
            callback
                .expect(
                    "non-null function pointer",
                )(
                &raw mut name as *mut ::core::ffi::c_char,
                lnames.offset(i as isize) as *mut *mut patch_t,
            );
            i += 1;
        }
    } else {
        i = 0 as i32;
        while i < NUMMAPS {
            snprintf(
                &raw mut name as *mut ::core::ffi::c_char,
                9 as size_t,
                b"WILV%d%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*wbs).epsd,
                i,
            );
            callback
                .expect(
                    "non-null function pointer",
                )(
                &raw mut name as *mut ::core::ffi::c_char,
                lnames.offset(i as isize) as *mut *mut patch_t,
            );
            i += 1;
        }
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WIURH0\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (&raw mut yah as *mut *mut patch_t).offset(0 as i32 as isize)
                as *mut *mut patch_t,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WIURH1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (&raw mut yah as *mut *mut patch_t).offset(1 as i32 as isize)
                as *mut *mut patch_t,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WISPLAT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (&raw mut splat as *mut *mut patch_t)
                .offset(0 as i32 as isize) as *mut *mut patch_t,
        );
        if (*wbs).epsd < 3 as i32 {
            j = 0 as i32;
            while j < NUMANIMS[(*wbs).epsd as usize] {
                a = (*(&raw mut anims as *mut *mut anim_t).offset((*wbs).epsd as isize))
                    .offset(j as isize) as *mut anim_t;
                i = 0 as i32;
                while i < (*a).nanims {
                    if (*wbs).epsd != 1 as i32
                        || j != 8 as i32
                    {
                        snprintf(
                            &raw mut name as *mut ::core::ffi::c_char,
                            9 as size_t,
                            b"WIA%d%.2d%.2d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*wbs).epsd,
                            j,
                            i,
                        );
                        callback
                            .expect(
                                "non-null function pointer",
                            )(
                            &raw mut name as *mut ::core::ffi::c_char,
                            (&raw mut (*a).p as *mut *mut patch_t).offset(i as isize)
                                as *mut *mut patch_t,
                        );
                    } else {
                        (*a).p[i as usize] = (*anims[1 as i32 as usize]
                            .offset(4 as i32 as isize))
                            .p[i as usize];
                    }
                    i += 1;
                }
                j += 1;
            }
        }
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIMINUS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut wiminus,
    );
    i = 0 as i32;
    while i < 10 as i32 {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            9 as size_t,
            b"WINUM%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut name as *mut ::core::ffi::c_char,
            (&raw mut num as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        i += 1;
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIPCNT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut percent,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut finished,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIENTER\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut entering,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIOSTK\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut kills,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIOSTS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut secret,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WISCRT2\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut sp_secret,
    );
    if W_CheckNumForName("WIOBJ",
    ) >= 0 as i32
    {
        if netgame && deathmatch == 0 {
            callback
                .expect(
                    "non-null function pointer",
                )(
                b"WIOBJ\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut items,
            );
        } else {
            callback
                .expect(
                    "non-null function pointer",
                )(
                b"WIOSTI\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut items,
            );
        }
    } else {
        callback
            .expect(
                "non-null function pointer",
            )(
            b"WIOSTI\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut items,
        );
    }
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIFRGS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut frags,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WICOLON\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut colon,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WITIME\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut timepatch,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WISUCKS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut sucks,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIPAR\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut par,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIKILRS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut killers,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIVCTMS\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut victims,
    );
    callback
        .expect(
            "non-null function pointer",
        )(
        b"WIMSTT\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut total,
    );
    i = 0 as i32;
    while i < MAXPLAYERS {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            9 as size_t,
            b"STPB%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut name as *mut ::core::ffi::c_char,
            (&raw mut p as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            9 as size_t,
            b"WIBP%d\0" as *const u8 as *const ::core::ffi::c_char,
            i + 1 as i32,
        );
        callback
            .expect(
                "non-null function pointer",
            )(
            &raw mut name as *mut ::core::ffi::c_char,
            (&raw mut bp as *mut *mut patch_t).offset(i as isize) as *mut *mut patch_t,
        );
        i += 1;
    }
    if gamemode as u32
        == commercial as i32 as u32
    {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            b"INTERPIC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
        );
    } else if gamemode as u32
        == retail as i32 as u32
        && (*wbs).epsd == 3 as i32
    {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            b"INTERPIC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
        );
    } else {
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
            b"WIMAP%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*wbs).epsd,
        );
    }
    callback
        .expect(
            "non-null function pointer",
        )(&raw mut name as *mut ::core::ffi::c_char, &raw mut background);
}
unsafe extern "C" fn WI_loadCallback(
    mut name: *mut ::core::ffi::c_char,
    mut variable: *mut *mut patch_t,
) {
    *variable = W_CacheLumpName(
        &wad_name8_to_string(name),
        PU_STATIC as i32,
    ) as *mut patch_t;
}
#[no_mangle]
pub unsafe extern "C" fn WI_loadData() {
    if gamemode as u32
        == commercial as i32 as u32
    {
        NUMCMAPS = 32 as i32;
        lnames = Z_Malloc(
            (::core::mem::size_of::<*mut patch_t>() as usize)
                .wrapping_mul(NUMCMAPS as usize) as i32,
            PU_STATIC as i32,
            NULL,
        ) as *mut *mut patch_t;
    } else {
        lnames = Z_Malloc(
            (::core::mem::size_of::<*mut patch_t>() as usize)
                .wrapping_mul(NUMMAPS as usize) as i32,
            PU_STATIC as i32,
            NULL,
        ) as *mut *mut patch_t;
    }
    WI_loadUnloadData(
        Some(
            WI_loadCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut *mut patch_t,
                ) -> (),
        ),
    );
    star = W_CacheLumpName("STFST01",
        PU_STATIC as i32,
    ) as *mut patch_t;
    bstar = W_CacheLumpName("STFDEAD0",
        PU_STATIC as i32,
    ) as *mut patch_t;
}
unsafe extern "C" fn WI_unloadCallback(
    mut name: *mut ::core::ffi::c_char,
    mut variable: *mut *mut patch_t,
) {
    W_ReleaseLumpName(&wad_name8_to_string(name));
    *variable = ::core::ptr::null_mut::<patch_t>();
}
pub unsafe fn WI_Drawer() {
    match state as i32 {
        0 => {
            if deathmatch != 0 {
                WI_drawDeathmatchStats();
            } else if netgame {
                WI_drawNetgameStats();
            } else {
                WI_drawStats();
            }
        }
        1 => {
            WI_drawShowNextLoc();
        }
        -1 => {
            WI_drawNoState();
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn WI_initVariables(mut wbstartstruct: *mut wbstartstruct_t) {
    wbs = wbstartstruct;
    acceleratestage = 0 as i32;
    bcnt = 0 as i32;
    cnt = bcnt;
    firstrefresh = 1 as i32;
    me = (*wbs).pnum;
    plrs = &raw mut (*wbs).plyr as *mut wbplayerstruct_t;
    if (*wbs).maxkills == 0 {
        (*wbs).maxkills = 1 as i32;
    }
    if (*wbs).maxitems == 0 {
        (*wbs).maxitems = 1 as i32;
    }
    if (*wbs).maxsecret == 0 {
        (*wbs).maxsecret = 1 as i32;
    }
    if gamemode as u32
        != retail as i32 as u32
    {
        if (*wbs).epsd > 2 as i32 {
            (*wbs).epsd -= 3 as i32;
        }
    }
}
pub unsafe fn WI_Start(mut wbstartstruct: *mut wbstartstruct_t) {
    WI_initVariables(wbstartstruct);
    WI_loadData();
    if deathmatch != 0 {
        WI_initDeathmatchStats();
    } else if netgame {
        WI_initNetgameStats();
    } else {
        WI_initStats();
    };
}
unsafe extern "C" fn run_static_initializers() {
    NUMANIMS = [
        (::core::mem::size_of::<[anim_t; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<anim_t>() as usize)
            as i32,
        (::core::mem::size_of::<[anim_t; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<anim_t>() as usize)
            as i32,
        (::core::mem::size_of::<[anim_t; 6]>() as usize)
            .wrapping_div(::core::mem::size_of::<anim_t>() as usize)
            as i32,
        0,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
