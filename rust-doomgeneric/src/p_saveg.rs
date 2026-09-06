use crate::src::i_system::FILE;
use crate::src::r_defs::{side_t};
use crate::src::p_spec::{plat_t, ceiling_t, floormove_t};
use crate::src::p_lights::{lightflash_t, strobe_t, glow_t};
use crate::src::p_doors::{vldoor_t};
use crate::src::p_mobj::{thinker_s, thinker_t, mapthing_t, state_t, mobjinfo_t, subsector_s, sector_t, line_t, actionf_t};
use crate::src::d_player::{player_s, player_t, playerstate_t};
use crate::src::p_mobj::{mobj_s, mobj_t, pspdef_t};
use crate::src::d_ticcmd::{ticcmd_t};
use crate::src::i_system::I_Error;
use crate::src::p_ceilng::P_AddActiveCeiling;
use crate::src::d_main::savegamedir;
use crate::src::g_game::G_VanillaVersionCode;
use crate::src::p_ceilng::activeceilings;
use crate::src::p_tick::P_InitThinkers;
use crate::src::p_setup::numlines;
use crate::src::p_maputl::P_SetThingPosition;
use crate::src::p_tick::thinkercap;
use crate::src::g_game::gameskill;
use crate::src::info::mobjinfo;
use crate::src::p_mobj::P_RemoveMobj;
use crate::src::p_setup::lines;
use crate::src::g_game::gameepisode;
use crate::src::g_game::gamemap;
use crate::src::info::states;
use crate::src::p_setup::numsectors;
use crate::src::p_setup::sides;
use crate::src::p_tick::P_AddThinker;
use crate::src::g_game::playeringame;
use crate::src::m_misc::M_snprintf;
use crate::src::p_setup::sectors;
use crate::src::p_tick::leveltime;
use crate::src::g_game::players;
use crate::src::p_plats::P_AddActivePlat;
use crate::src::m_misc::M_StringJoin;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_LEVEL;
use crate::src::d_player::NUMPOWERS;
use crate::src::d_player::NUMPSPRITES;
use libc::memset;
use libc::{strcmp, strlen};
use libc::{malloc, snprintf};
use crate::src::i_system::{fprintf, fread, ftell, fwrite, stderr};
use crate::src::p_mobj::spritenum_t;
use crate::src::p_mobj::mobjtype_t;
use crate::src::p_mobj::{actionf_p1, statenum_t};
use crate::src::d_mode::skill_t;
use crate::src::d_player::{NUMWEAPONS, weapontype_t};

extern "C" {
    fn P_MobjThinker(mobj: *mut mobj_t);
    fn T_LightFlash(flash: *mut lightflash_t);
    fn T_StrobeFlash(flash: *mut strobe_t);
    fn T_Glow(g: *mut glow_t);
    fn T_PlatRaise(plat: *mut plat_t);
    fn T_VerticalDoor(door: *mut vldoor_t);
    fn T_MoveCeiling(ceiling: *mut ceiling_t);
    fn T_MoveFloor(floor: *mut floormove_t);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type intptr_t = isize;
pub type boolean = u32;
pub type byte = uint8_t;
pub type fixed_t = i32;
pub type angle_t = u32;
pub type C2RustUnnamed_0 = u32;
pub const NUMCARDS: C2RustUnnamed_0 = 6;
pub const it_redskull: C2RustUnnamed_0 = 5;
pub const it_yellowskull: C2RustUnnamed_0 = 4;
pub const it_blueskull: C2RustUnnamed_0 = 3;
pub const it_redcard: C2RustUnnamed_0 = 2;
pub const it_yellowcard: C2RustUnnamed_0 = 1;
pub const it_bluecard: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = u32;
pub const am_noammo: C2RustUnnamed_1 = 5;
pub const NUMAMMO: C2RustUnnamed_1 = 4;
pub const am_misl: C2RustUnnamed_1 = 3;
pub const am_cell: C2RustUnnamed_1 = 2;
pub const am_shell: C2RustUnnamed_1 = 1;
pub const am_clip: C2RustUnnamed_1 = 0;
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
pub type plat_e = u32;
pub const in_stasis: plat_e = 3;
pub const waiting: plat_e = 2;
pub const down: plat_e = 1;
pub const up: plat_e = 0;
pub type plattype_e = u32;
pub const blazeDWUS: plattype_e = 4;
pub const raiseToNearestAndChange: plattype_e = 3;
pub const raiseAndChange: plattype_e = 2;
pub const downWaitUpStay: plattype_e = 1;
pub const perpetualRaise: plattype_e = 0;
pub type vldoor_e = u32;
pub const vld_blazeClose: vldoor_e = 7;
pub const vld_blazeOpen: vldoor_e = 6;
pub const vld_blazeRaise: vldoor_e = 5;
pub const vld_raiseIn5Mins: vldoor_e = 4;
pub const vld_open: vldoor_e = 3;
pub const vld_close: vldoor_e = 2;
pub const vld_close30ThenOpen: vldoor_e = 1;
pub const vld_normal: vldoor_e = 0;
pub type ceiling_e = u32;
pub const silentCrushAndRaise: ceiling_e = 5;
pub const fastCrushAndRaise: ceiling_e = 4;
pub const crushAndRaise: ceiling_e = 3;
pub const lowerAndCrush: ceiling_e = 2;
pub const raiseToHighest: ceiling_e = 1;
pub const lowerToFloor: ceiling_e = 0;
pub type floor_e = u32;
pub const raiseFloor512: floor_e = 12;
pub const donutRaise: floor_e = 11;
pub const raiseFloorTurbo: floor_e = 10;
pub const raiseFloorCrush: floor_e = 9;
pub const raiseFloor24AndChange: floor_e = 8;
pub const raiseFloor24: floor_e = 7;
pub const lowerAndChange: floor_e = 6;
pub const raiseToTexture: floor_e = 5;
pub const raiseFloorToNearest: floor_e = 4;
pub const raiseFloor: floor_e = 3;
pub const turboLower: floor_e = 2;
pub const lowerFloorToLowest: floor_e = 1;
pub const lowerFloor: floor_e = 0;
pub const tc_end: C2RustUnnamed_4 = 0;
pub const tc_mobj: C2RustUnnamed_4 = 1;
pub const tc_endspecials: C2RustUnnamed_5 = 7;
pub const tc_glow: C2RustUnnamed_5 = 6;
pub const tc_strobe: C2RustUnnamed_5 = 5;
pub const tc_flash: C2RustUnnamed_5 = 4;
pub const tc_plat: C2RustUnnamed_5 = 3;
pub const tc_floor: C2RustUnnamed_5 = 2;
pub const tc_door: C2RustUnnamed_5 = 1;
pub const tc_ceiling: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_4 = u32;
pub type C2RustUnnamed_5 = u32;
pub const true_0: i32 = 1 as i32;
pub const false_0: i32 = 0 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXPLAYERS: i32 = 4 as i32;
pub const MAXCEILINGS: i32 = 30 as i32;
pub const SAVESTRINGSIZE: i32 = 24 as i32;
pub const SAVEGAME_EOF: i32 = 0x1d as i32;
pub const VERSIONSIZE: i32 = 16 as i32;
pub static mut save_stream: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut savegamelength: i32 = 0;
pub static mut savegame_error: bool = false;
pub unsafe fn P_TempSaveGameFile() -> *mut ::core::ffi::c_char {
    static mut filename: *mut ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >() as *mut ::core::ffi::c_char;
    if filename.is_null() {
        filename = M_StringJoin(
            savegamedir,
            b"temp.dsg\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
    }
    return filename;
}
pub unsafe fn P_SaveGameFile(
    mut slot: i32,
) -> *mut ::core::ffi::c_char {
    static mut filename: *mut ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >() as *mut ::core::ffi::c_char;
    static mut filename_size: size_t = 0 as size_t;
    let mut basename: [::core::ffi::c_char; 32] = [0; 32];
    if filename.is_null() {
        filename_size = strlen(savegamedir).wrapping_add(32 as size_t);
        filename = malloc(filename_size) as *mut ::core::ffi::c_char;
    }
    snprintf(
        &raw mut basename as *mut ::core::ffi::c_char,
        32 as size_t,
        b"doomsav%d.dsg\0" as *const u8 as *const ::core::ffi::c_char,
        slot,
    );
    M_snprintf(
        filename,
        filename_size,
        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        savegamedir,
        &raw mut basename as *mut ::core::ffi::c_char,
    );
    return filename;
}
unsafe extern "C" fn saveg_read8() -> byte {
    let mut result: byte = 0;
    if fread(
        &raw mut result as *mut ::core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        save_stream,
    ) < 1 as u64
    {
        if !savegame_error {
            fprintf(
                stderr,
                b"saveg_read8: Unexpected end of file while reading save game\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            savegame_error = true;
        }
    }
    return result;
}
unsafe extern "C" fn saveg_write8(mut value: byte) {
    if fwrite(
        &raw mut value as *const ::core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        save_stream,
    ) < 1 as u64
    {
        if !savegame_error {
            fprintf(
                stderr,
                b"saveg_write8: Error while writing save game\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            savegame_error = true;
        }
    }
}
unsafe extern "C" fn saveg_read16() -> i16 {
    let mut result: i32 = 0;
    result = saveg_read8() as i32;
    result |= (saveg_read8() as i32) << 8 as i32;
    return result as i16;
}
unsafe extern "C" fn saveg_write16(mut value: i16) {
    saveg_write8((value as i32 & 0xff as i32) as byte);
    saveg_write8(
        (value as i32 >> 8 as i32
            & 0xff as i32) as byte,
    );
}
unsafe extern "C" fn saveg_read32() -> i32 {
    let mut result: i32 = 0;
    result = saveg_read8() as i32;
    result |= (saveg_read8() as i32) << 8 as i32;
    result |= (saveg_read8() as i32) << 16 as i32;
    result |= (saveg_read8() as i32) << 24 as i32;
    return result;
}
unsafe extern "C" fn saveg_write32(mut value: i32) {
    saveg_write8((value & 0xff as i32) as byte);
    saveg_write8(
        (value >> 8 as i32 & 0xff as i32) as byte,
    );
    saveg_write8(
        (value >> 16 as i32 & 0xff as i32) as byte,
    );
    saveg_write8(
        (value >> 24 as i32 & 0xff as i32) as byte,
    );
}
unsafe extern "C" fn saveg_read_pad() {
    let mut pos: u64 = 0;
    let mut padding: i32 = 0;
    let mut i: i32 = 0;
    pos = ftell(save_stream) as u64;
    padding = ((4 as u64).wrapping_sub(pos & 3 as u64)
        & 3 as u64) as i32;
    i = 0 as i32;
    while i < padding {
        saveg_read8();
        i += 1;
    }
}
unsafe extern "C" fn saveg_write_pad() {
    let mut pos: u64 = 0;
    let mut padding: i32 = 0;
    let mut i: i32 = 0;
    pos = ftell(save_stream) as u64;
    padding = ((4 as u64).wrapping_sub(pos & 3 as u64)
        & 3 as u64) as i32;
    i = 0 as i32;
    while i < padding {
        saveg_write8(0 as byte);
        i += 1;
    }
}
unsafe extern "C" fn saveg_readp() -> *mut ::core::ffi::c_void {
    return saveg_read32() as intptr_t as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn saveg_writep(mut p: *mut ::core::ffi::c_void) {
    saveg_write32(p as intptr_t as i32);
}
unsafe extern "C" fn saveg_read_mapthing_t(mut str: *mut mapthing_t) {
    (*str).x = saveg_read16();
    (*str).y = saveg_read16();
    (*str).angle = saveg_read16();
    (*str).type_0 = saveg_read16();
    (*str).options = saveg_read16();
}
unsafe extern "C" fn saveg_write_mapthing_t(mut str: *mut mapthing_t) {
    saveg_write16((*str).x);
    saveg_write16((*str).y);
    saveg_write16((*str).angle);
    saveg_write16((*str).type_0);
    saveg_write16((*str).options);
}
unsafe extern "C" fn saveg_read_actionf_t(mut str: *mut actionf_t) {
    (*str).acp1 = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        actionf_p1,
    >(saveg_readp());
}
unsafe extern "C" fn saveg_write_actionf_t(mut str: *mut actionf_t) {
    saveg_writep(
        ::core::mem::transmute::<actionf_p1, *mut ::core::ffi::c_void>((*str).acp1),
    );
}
unsafe extern "C" fn saveg_read_thinker_t(mut str: *mut thinker_t) {
    (*str).prev = saveg_readp() as *mut thinker_s;
    (*str).next = saveg_readp() as *mut thinker_s;
    saveg_read_actionf_t(&raw mut (*str).function);
}
unsafe extern "C" fn saveg_write_thinker_t(mut str: *mut thinker_t) {
    saveg_writep((*str).prev as *mut ::core::ffi::c_void);
    saveg_writep((*str).next as *mut ::core::ffi::c_void);
    saveg_write_actionf_t(&raw mut (*str).function);
}
unsafe extern "C" fn saveg_read_mobj_t(mut str: *mut mobj_t) {
    let mut pl: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).x = saveg_read32() as fixed_t;
    (*str).y = saveg_read32() as fixed_t;
    (*str).z = saveg_read32() as fixed_t;
    (*str).snext = saveg_readp() as *mut mobj_s;
    (*str).sprev = saveg_readp() as *mut mobj_s;
    (*str).angle = saveg_read32() as angle_t;
    (*str).sprite = saveg_read32() as spritenum_t;
    (*str).frame = saveg_read32();
    (*str).bnext = saveg_readp() as *mut mobj_s;
    (*str).bprev = saveg_readp() as *mut mobj_s;
    (*str).subsector = saveg_readp() as *mut subsector_s;
    (*str).floorz = saveg_read32() as fixed_t;
    (*str).ceilingz = saveg_read32() as fixed_t;
    (*str).radius = saveg_read32() as fixed_t;
    (*str).height = saveg_read32() as fixed_t;
    (*str).momx = saveg_read32() as fixed_t;
    (*str).momy = saveg_read32() as fixed_t;
    (*str).momz = saveg_read32() as fixed_t;
    (*str).validcount = saveg_read32();
    (*str).type_0 = saveg_read32() as mobjtype_t;
    (*str).info = saveg_readp() as *mut mobjinfo_t;
    (*str).tics = saveg_read32();
    (*str).state = (&raw mut states as *mut state_t)
        .offset(
            (saveg_read32 as unsafe extern "C" fn() -> i32)() as isize,
        ) as *mut state_t;
    (*str).flags = saveg_read32();
    (*str).health = saveg_read32();
    (*str).movedir = saveg_read32();
    (*str).movecount = saveg_read32();
    (*str).target = saveg_readp() as *mut mobj_s;
    (*str).reactiontime = saveg_read32();
    (*str).threshold = saveg_read32();
    pl = saveg_read32();
    if pl > 0 as i32 {
        (*str).player = (&raw mut players as *mut player_t)
            .offset((pl - 1 as i32) as isize) as *mut player_t
            as *mut player_s;
        (*(*str).player).mo = str;
    } else {
        (*str).player = ::core::ptr::null_mut::<player_s>();
    }
    (*str).lastlook = saveg_read32();
    saveg_read_mapthing_t(&raw mut (*str).spawnpoint);
    (*str).tracer = saveg_readp() as *mut mobj_s;
}
unsafe extern "C" fn saveg_write_mobj_t(mut str: *mut mobj_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).x as i32);
    saveg_write32((*str).y as i32);
    saveg_write32((*str).z as i32);
    saveg_writep((*str).snext as *mut ::core::ffi::c_void);
    saveg_writep((*str).sprev as *mut ::core::ffi::c_void);
    saveg_write32((*str).angle as i32);
    saveg_write32((*str).sprite as i32);
    saveg_write32((*str).frame);
    saveg_writep((*str).bnext as *mut ::core::ffi::c_void);
    saveg_writep((*str).bprev as *mut ::core::ffi::c_void);
    saveg_writep((*str).subsector as *mut ::core::ffi::c_void);
    saveg_write32((*str).floorz as i32);
    saveg_write32((*str).ceilingz as i32);
    saveg_write32((*str).radius as i32);
    saveg_write32((*str).height as i32);
    saveg_write32((*str).momx as i32);
    saveg_write32((*str).momy as i32);
    saveg_write32((*str).momz as i32);
    saveg_write32((*str).validcount);
    saveg_write32((*str).type_0 as i32);
    saveg_writep((*str).info as *mut ::core::ffi::c_void);
    saveg_write32((*str).tics);
    saveg_write32(
        (*str).state.offset_from(&raw mut states as *mut state_t) as i64
            as i32,
    );
    saveg_write32((*str).flags);
    saveg_write32((*str).health);
    saveg_write32((*str).movedir);
    saveg_write32((*str).movecount);
    saveg_writep((*str).target as *mut ::core::ffi::c_void);
    saveg_write32((*str).reactiontime);
    saveg_write32((*str).threshold);
    if !(*str).player.is_null() {
        saveg_write32(
            ((*str).player.offset_from(&raw mut players as *mut player_t)
                as i64 + 1 as i64) as i32,
        );
    } else {
        saveg_write32(0 as i32);
    }
    saveg_write32((*str).lastlook);
    saveg_write_mapthing_t(&raw mut (*str).spawnpoint);
    saveg_writep((*str).tracer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn saveg_read_ticcmd_t(mut str: *mut ticcmd_t) {
    (*str).forwardmove = saveg_read8() as i8;
    (*str).sidemove = saveg_read8() as i8;
    (*str).angleturn = saveg_read16();
    (*str).consistancy = saveg_read16() as byte;
    (*str).chatchar = saveg_read8();
    (*str).buttons = saveg_read8();
}
unsafe extern "C" fn saveg_write_ticcmd_t(mut str: *mut ticcmd_t) {
    saveg_write8((*str).forwardmove as byte);
    saveg_write8((*str).sidemove as byte);
    saveg_write16((*str).angleturn);
    saveg_write16((*str).consistancy as i16);
    saveg_write8((*str).chatchar);
    saveg_write8((*str).buttons);
}
unsafe extern "C" fn saveg_read_pspdef_t(mut str: *mut pspdef_t) {
    let mut state: i32 = 0;
    state = saveg_read32();
    if state > 0 as i32 {
        (*str).state = (&raw mut states as *mut state_t).offset(state as isize)
            as *mut state_t;
    } else {
        (*str).state = ::core::ptr::null_mut::<state_t>();
    }
    (*str).tics = saveg_read32();
    (*str).sx = saveg_read32() as fixed_t;
    (*str).sy = saveg_read32() as fixed_t;
}
unsafe extern "C" fn saveg_write_pspdef_t(mut str: *mut pspdef_t) {
    if !(*str).state.is_null() {
        saveg_write32(
            (*str).state.offset_from(&raw mut states as *mut state_t)
                as i64 as i32,
        );
    } else {
        saveg_write32(0 as i32);
    }
    saveg_write32((*str).tics);
    saveg_write32((*str).sx as i32);
    saveg_write32((*str).sy as i32);
}
unsafe extern "C" fn saveg_read_player_t(mut str: *mut player_t) {
    let mut i: i32 = 0;
    (*str).mo = saveg_readp() as *mut mobj_t;
    (*str).playerstate = saveg_read32() as playerstate_t;
    saveg_read_ticcmd_t(&raw mut (*str).cmd);
    (*str).viewz = saveg_read32() as fixed_t;
    (*str).viewheight = saveg_read32() as fixed_t;
    (*str).deltaviewheight = saveg_read32() as fixed_t;
    (*str).bob = saveg_read32() as fixed_t;
    (*str).health = saveg_read32();
    (*str).armorpoints = saveg_read32();
    (*str).armortype = saveg_read32();
    i = 0 as i32;
    while i < NUMPOWERS as i32 {
        (*str).powers[i as usize] = saveg_read32();
        i += 1;
    }
    i = 0 as i32;
    while i < NUMCARDS as i32 {
        (*str).cards[i as usize] = saveg_read32() != 0;
        i += 1;
    }
    (*str).backpack = saveg_read32() != 0;
    i = 0 as i32;
    while i < MAXPLAYERS {
        (*str).frags[i as usize] = saveg_read32();
        i += 1;
    }
    (*str).readyweapon = saveg_read32() as weapontype_t;
    (*str).pendingweapon = saveg_read32() as weapontype_t;
    i = 0 as i32;
    while i < NUMWEAPONS as i32 {
        (*str).weaponowned[i as usize] = saveg_read32() != 0;
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        (*str).ammo[i as usize] = saveg_read32();
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        (*str).maxammo[i as usize] = saveg_read32();
        i += 1;
    }
    (*str).attackdown = saveg_read32();
    (*str).usedown = saveg_read32();
    (*str).cheats = saveg_read32();
    (*str).refire = saveg_read32();
    (*str).killcount = saveg_read32();
    (*str).itemcount = saveg_read32();
    (*str).secretcount = saveg_read32();
    (*str).message = saveg_readp() as *mut ::core::ffi::c_char;
    (*str).damagecount = saveg_read32();
    (*str).bonuscount = saveg_read32();
    (*str).attacker = saveg_readp() as *mut mobj_t;
    (*str).extralight = saveg_read32();
    (*str).fixedcolormap = saveg_read32();
    (*str).colormap = saveg_read32();
    i = 0 as i32;
    while i < NUMPSPRITES as i32 {
        saveg_read_pspdef_t(
            (&raw mut (*str).psprites as *mut pspdef_t).offset(i as isize)
                as *mut pspdef_t,
        );
        i += 1;
    }
    (*str).didsecret = saveg_read32() != 0;
}
unsafe extern "C" fn saveg_write_player_t(mut str: *mut player_t) {
    let mut i: i32 = 0;
    saveg_writep((*str).mo as *mut ::core::ffi::c_void);
    saveg_write32((*str).playerstate as i32);
    saveg_write_ticcmd_t(&raw mut (*str).cmd);
    saveg_write32((*str).viewz as i32);
    saveg_write32((*str).viewheight as i32);
    saveg_write32((*str).deltaviewheight as i32);
    saveg_write32((*str).bob as i32);
    saveg_write32((*str).health);
    saveg_write32((*str).armorpoints);
    saveg_write32((*str).armortype);
    i = 0 as i32;
    while i < NUMPOWERS as i32 {
        saveg_write32((*str).powers[i as usize]);
        i += 1;
    }
    i = 0 as i32;
    while i < NUMCARDS as i32 {
        saveg_write32((*str).cards[i as usize] as i32);
        i += 1;
    }
    saveg_write32((*str).backpack as i32);
    i = 0 as i32;
    while i < MAXPLAYERS {
        saveg_write32((*str).frags[i as usize]);
        i += 1;
    }
    saveg_write32((*str).readyweapon as i32);
    saveg_write32((*str).pendingweapon as i32);
    i = 0 as i32;
    while i < NUMWEAPONS as i32 {
        saveg_write32((*str).weaponowned[i as usize] as i32);
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        saveg_write32((*str).ammo[i as usize]);
        i += 1;
    }
    i = 0 as i32;
    while i < NUMAMMO as i32 {
        saveg_write32((*str).maxammo[i as usize]);
        i += 1;
    }
    saveg_write32((*str).attackdown);
    saveg_write32((*str).usedown);
    saveg_write32((*str).cheats);
    saveg_write32((*str).refire);
    saveg_write32((*str).killcount);
    saveg_write32((*str).itemcount);
    saveg_write32((*str).secretcount);
    saveg_writep((*str).message as *mut ::core::ffi::c_void);
    saveg_write32((*str).damagecount);
    saveg_write32((*str).bonuscount);
    saveg_writep((*str).attacker as *mut ::core::ffi::c_void);
    saveg_write32((*str).extralight);
    saveg_write32((*str).fixedcolormap);
    saveg_write32((*str).colormap);
    i = 0 as i32;
    while i < NUMPSPRITES as i32 {
        saveg_write_pspdef_t(
            (&raw mut (*str).psprites as *mut pspdef_t).offset(i as isize)
                as *mut pspdef_t,
        );
        i += 1;
    }
    saveg_write32((*str).didsecret as i32);
}
unsafe extern "C" fn saveg_read_ceiling_t(mut str: *mut ceiling_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).type_0 = saveg_read32() as ceiling_e;
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).bottomheight = saveg_read32() as fixed_t;
    (*str).topheight = saveg_read32() as fixed_t;
    (*str).speed = saveg_read32() as fixed_t;
    (*str).crush = saveg_read32() != 0;
    (*str).direction = saveg_read32();
    (*str).tag = saveg_read32();
    (*str).olddirection = saveg_read32();
}
unsafe extern "C" fn saveg_write_ceiling_t(mut str: *mut ceiling_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).type_0 as i32);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).bottomheight as i32);
    saveg_write32((*str).topheight as i32);
    saveg_write32((*str).speed as i32);
    saveg_write32((*str).crush as i32);
    saveg_write32((*str).direction);
    saveg_write32((*str).tag);
    saveg_write32((*str).olddirection);
}
unsafe extern "C" fn saveg_read_vldoor_t(mut str: *mut vldoor_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).type_0 = saveg_read32() as vldoor_e;
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).topheight = saveg_read32() as fixed_t;
    (*str).speed = saveg_read32() as fixed_t;
    (*str).direction = saveg_read32();
    (*str).topwait = saveg_read32();
    (*str).topcountdown = saveg_read32();
}
unsafe extern "C" fn saveg_write_vldoor_t(mut str: *mut vldoor_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).type_0 as i32);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).topheight as i32);
    saveg_write32((*str).speed as i32);
    saveg_write32((*str).direction);
    saveg_write32((*str).topwait);
    saveg_write32((*str).topcountdown);
}
unsafe extern "C" fn saveg_read_floormove_t(mut str: *mut floormove_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    (*str).type_0 = saveg_read32() as floor_e;
    (*str).crush = saveg_read32() != 0;
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).direction = saveg_read32();
    (*str).newspecial = saveg_read32();
    (*str).texture = saveg_read16();
    (*str).floordestheight = saveg_read32() as fixed_t;
    (*str).speed = saveg_read32() as fixed_t;
}
unsafe extern "C" fn saveg_write_floormove_t(mut str: *mut floormove_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32((*str).type_0 as i32);
    saveg_write32((*str).crush as i32);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).direction);
    saveg_write32((*str).newspecial);
    saveg_write16((*str).texture);
    saveg_write32((*str).floordestheight as i32);
    saveg_write32((*str).speed as i32);
}
unsafe extern "C" fn saveg_read_plat_t(mut str: *mut plat_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).speed = saveg_read32() as fixed_t;
    (*str).low = saveg_read32() as fixed_t;
    (*str).high = saveg_read32() as fixed_t;
    (*str).wait = saveg_read32();
    (*str).count = saveg_read32();
    (*str).status = saveg_read32() as plat_e;
    (*str).oldstatus = saveg_read32() as plat_e;
    (*str).crush = saveg_read32() != 0;
    (*str).tag = saveg_read32();
    (*str).type_0 = saveg_read32() as plattype_e;
}
unsafe extern "C" fn saveg_write_plat_t(mut str: *mut plat_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).speed as i32);
    saveg_write32((*str).low as i32);
    saveg_write32((*str).high as i32);
    saveg_write32((*str).wait);
    saveg_write32((*str).count);
    saveg_write32((*str).status as i32);
    saveg_write32((*str).oldstatus as i32);
    saveg_write32((*str).crush as i32);
    saveg_write32((*str).tag);
    saveg_write32((*str).type_0 as i32);
}
unsafe extern "C" fn saveg_read_lightflash_t(mut str: *mut lightflash_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).count = saveg_read32();
    (*str).maxlight = saveg_read32();
    (*str).minlight = saveg_read32();
    (*str).maxtime = saveg_read32();
    (*str).mintime = saveg_read32();
}
unsafe extern "C" fn saveg_write_lightflash_t(mut str: *mut lightflash_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).count);
    saveg_write32((*str).maxlight);
    saveg_write32((*str).minlight);
    saveg_write32((*str).maxtime);
    saveg_write32((*str).mintime);
}
unsafe extern "C" fn saveg_read_strobe_t(mut str: *mut strobe_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).count = saveg_read32();
    (*str).minlight = saveg_read32();
    (*str).maxlight = saveg_read32();
    (*str).darktime = saveg_read32();
    (*str).brighttime = saveg_read32();
}
unsafe extern "C" fn saveg_write_strobe_t(mut str: *mut strobe_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).count);
    saveg_write32((*str).minlight);
    saveg_write32((*str).maxlight);
    saveg_write32((*str).darktime);
    saveg_write32((*str).brighttime);
}
unsafe extern "C" fn saveg_read_glow_t(mut str: *mut glow_t) {
    let mut sector: i32 = 0;
    saveg_read_thinker_t(&raw mut (*str).thinker);
    sector = saveg_read32();
    (*str).sector = sectors.offset(sector as isize) as *mut sector_t;
    (*str).minlight = saveg_read32();
    (*str).maxlight = saveg_read32();
    (*str).direction = saveg_read32();
}
unsafe extern "C" fn saveg_write_glow_t(mut str: *mut glow_t) {
    saveg_write_thinker_t(&raw mut (*str).thinker);
    saveg_write32(
        (*str).sector.offset_from(sectors) as i64 as i32,
    );
    saveg_write32((*str).minlight);
    saveg_write32((*str).maxlight);
    saveg_write32((*str).direction);
}
pub unsafe fn P_WriteSaveGameHeader(
    mut description: *mut ::core::ffi::c_char,
) {
    let mut name: [::core::ffi::c_char; 16] = [0; 16];
    let mut i: i32 = 0;
    i = 0 as i32;
    while *description.offset(i as isize) as i32 != '\0' as i32 {
        saveg_write8(*description.offset(i as isize) as byte);
        i += 1;
    }
    while i < SAVESTRINGSIZE {
        saveg_write8(0 as byte);
        i += 1;
    }
    memset(
        &raw mut name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
    );
    M_snprintf(
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        b"version %i\0" as *const u8 as *const ::core::ffi::c_char,
        G_VanillaVersionCode(),
    );
    i = 0 as i32;
    while i < VERSIONSIZE {
        saveg_write8(name[i as usize] as byte);
        i += 1;
    }
    saveg_write8(gameskill as byte);
    saveg_write8(gameepisode as byte);
    saveg_write8(gamemap as byte);
    i = 0 as i32;
    while i < MAXPLAYERS {
        saveg_write8(playeringame[i as usize] as byte);
        i += 1;
    }
    saveg_write8(
        (leveltime >> 16 as i32 & 0xff as i32) as byte,
    );
    saveg_write8(
        (leveltime >> 8 as i32 & 0xff as i32) as byte,
    );
    saveg_write8((leveltime & 0xff as i32) as byte);
}
pub unsafe fn P_ReadSaveGameHeader() -> bool {
    let mut i: i32 = 0;
    let mut a: byte = 0;
    let mut b: byte = 0;
    let mut c: byte = 0;
    let mut vcheck: [::core::ffi::c_char; 16] = [0; 16];
    let mut read_vcheck: [::core::ffi::c_char; 16] = [0; 16];
    i = 0 as i32;
    while i < SAVESTRINGSIZE {
        saveg_read8();
        i += 1;
    }
    i = 0 as i32;
    while i < VERSIONSIZE {
        read_vcheck[i as usize] = saveg_read8() as ::core::ffi::c_char;
        i += 1;
    }
    memset(
        &raw mut vcheck as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as i32,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
    );
    M_snprintf(
        &raw mut vcheck as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        b"version %i\0" as *const u8 as *const ::core::ffi::c_char,
        G_VanillaVersionCode(),
    );
    if strcmp(
        &raw mut read_vcheck as *mut ::core::ffi::c_char,
        &raw mut vcheck as *mut ::core::ffi::c_char,
    ) != 0 as i32
    {
        return false;
    }
    gameskill = saveg_read8() as skill_t;
    gameepisode = saveg_read8() as i32;
    gamemap = saveg_read8() as i32;
    i = 0 as i32;
    while i < MAXPLAYERS {
        playeringame[i as usize] = saveg_read8() as boolean;
        i += 1;
    }
    a = saveg_read8();
    b = saveg_read8();
    c = saveg_read8();
    leveltime = ((a as i32) << 16 as i32)
        + ((b as i32) << 8 as i32)
        + c as i32;
    return true;
}
pub unsafe fn P_ReadSaveGameEOF() -> bool {
    let mut value: i32 = 0;
    value = saveg_read8() as i32;
    return value == SAVEGAME_EOF;
}
pub unsafe fn P_WriteSaveGameEOF() {
    saveg_write8(SAVEGAME_EOF as byte);
}
pub unsafe fn P_ArchivePlayers() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            saveg_write_pad();
            saveg_write_player_t(
                (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t,
            );
        }
        i += 1;
    }
}
pub unsafe fn P_UnArchivePlayers() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < MAXPLAYERS {
        if !(playeringame[i as usize] == 0) {
            saveg_read_pad();
            saveg_read_player_t(
                (&raw mut players as *mut player_t).offset(i as isize) as *mut player_t,
            );
            players[i as usize].mo = ::core::ptr::null_mut::<mobj_t>();
            players[i as usize].message = ::core::ptr::null_mut::<::core::ffi::c_char>();
            players[i as usize].attacker = ::core::ptr::null_mut::<mobj_t>();
        }
        i += 1;
    }
}
pub unsafe fn P_ArchiveWorld() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut si: *mut side_t = ::core::ptr::null_mut::<side_t>();
    i = 0 as i32;
    sec = sectors;
    while i < numsectors {
        saveg_write16(((*sec).floorheight >> FRACBITS) as i16);
        saveg_write16(((*sec).ceilingheight >> FRACBITS) as i16);
        saveg_write16((*sec).floorpic);
        saveg_write16((*sec).ceilingpic);
        saveg_write16((*sec).lightlevel);
        saveg_write16((*sec).special);
        saveg_write16((*sec).tag);
        i += 1;
        sec = sec.offset(1);
    }
    i = 0 as i32;
    li = lines;
    while i < numlines {
        saveg_write16((*li).flags);
        saveg_write16((*li).special);
        saveg_write16((*li).tag);
        j = 0 as i32;
        while j < 2 as i32 {
            if !((*li).sidenum[j as usize] as i32
                == -(1 as i32))
            {
                si = sides
                    .offset(
                        *(&raw mut (*li).sidenum as *mut i16)
                            .offset(j as isize) as isize,
                    ) as *mut side_t;
                saveg_write16(((*si).textureoffset >> FRACBITS) as i16);
                saveg_write16(((*si).rowoffset >> FRACBITS) as i16);
                saveg_write16((*si).toptexture);
                saveg_write16((*si).bottomtexture);
                saveg_write16((*si).midtexture);
            }
            j += 1;
        }
        i += 1;
        li = li.offset(1);
    }
}
pub unsafe fn P_UnArchiveWorld() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sec: *mut sector_t = ::core::ptr::null_mut::<sector_t>();
    let mut li: *mut line_t = ::core::ptr::null_mut::<line_t>();
    let mut si: *mut side_t = ::core::ptr::null_mut::<side_t>();
    i = 0 as i32;
    sec = sectors;
    while i < numsectors {
        (*sec).floorheight = ((saveg_read16() as i32) << FRACBITS)
            as fixed_t;
        (*sec).ceilingheight = ((saveg_read16() as i32) << FRACBITS)
            as fixed_t;
        (*sec).floorpic = saveg_read16();
        (*sec).ceilingpic = saveg_read16();
        (*sec).lightlevel = saveg_read16();
        (*sec).special = saveg_read16();
        (*sec).tag = saveg_read16();
        (*sec).specialdata = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*sec).soundtarget = ::core::ptr::null_mut::<mobj_t>();
        i += 1;
        sec = sec.offset(1);
    }
    i = 0 as i32;
    li = lines;
    while i < numlines {
        (*li).flags = saveg_read16();
        (*li).special = saveg_read16();
        (*li).tag = saveg_read16();
        j = 0 as i32;
        while j < 2 as i32 {
            if !((*li).sidenum[j as usize] as i32
                == -(1 as i32))
            {
                si = sides
                    .offset(
                        *(&raw mut (*li).sidenum as *mut i16)
                            .offset(j as isize) as isize,
                    ) as *mut side_t;
                (*si).textureoffset = ((saveg_read16() as i32)
                    << FRACBITS) as fixed_t;
                (*si).rowoffset = ((saveg_read16() as i32) << FRACBITS)
                    as fixed_t;
                (*si).toptexture = saveg_read16();
                (*si).bottomtexture = saveg_read16();
                (*si).midtexture = saveg_read16();
            }
            j += 1;
        }
        i += 1;
        li = li.offset(1);
    }
}
pub unsafe fn P_ArchiveThinkers() {
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                actionf_p1,
            >(Some(P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> ()))
        {
            saveg_write8(tc_mobj as i32 as byte);
            saveg_write_pad();
            saveg_write_mobj_t(th as *mut mobj_t);
        }
        th = (*th).next as *mut thinker_t;
    }
    saveg_write8(tc_end as i32 as byte);
}
pub unsafe fn P_UnArchiveThinkers() {
    let mut tclass: byte = 0;
    let mut currentthinker: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut next: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut mobj: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    currentthinker = thinkercap.next as *mut thinker_t;
    while currentthinker != &raw mut thinkercap {
        next = (*currentthinker).next as *mut thinker_t;
        if (*currentthinker).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                actionf_p1,
            >(Some(P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> ()))
        {
            P_RemoveMobj(currentthinker as *mut mobj_t);
        } else {
            Z_Free(currentthinker as *mut ::core::ffi::c_void);
        }
        currentthinker = next;
    }
    P_InitThinkers();
    loop {
        tclass = saveg_read8();
        match tclass as i32 {
            0 => return,
            1 => {
                saveg_read_pad();
                mobj = Z_Malloc(
                    ::core::mem::size_of::<mobj_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut mobj_t;
                saveg_read_mobj_t(mobj);
                (*mobj).target = ::core::ptr::null_mut::<mobj_s>();
                (*mobj).tracer = ::core::ptr::null_mut::<mobj_s>();
                P_SetThingPosition(mobj);
                (*mobj).info = (&raw mut mobjinfo as *mut mobjinfo_t)
                    .offset((*mobj).type_0 as isize) as *mut mobjinfo_t;
                (*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
                (*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
                (*mobj).thinker.function.acp1 = ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                    actionf_p1,
                >(Some(P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> ()));
                P_AddThinker(&raw mut (*mobj).thinker);
            }
            _ => {
                I_Error(&format!(
                    "Unknown tclass {} in savegame",
                    tclass as i32,
                ));
            }
        }
    };
}
#[no_mangle]
pub static mut specials_e: C2RustUnnamed_5 = tc_ceiling;
pub unsafe fn P_ArchiveSpecials() {
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut i: i32 = 0;
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        if (*th).function.acv.is_none() {
            i = 0 as i32;
            while i < MAXCEILINGS {
                if activeceilings[i as usize] == th as *mut ceiling_t {
                    break;
                }
                i += 1;
            }
            if i < MAXCEILINGS {
                saveg_write8(tc_ceiling as i32 as byte);
                saveg_write_pad();
                saveg_write_ceiling_t(th as *mut ceiling_t);
            }
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ceiling_t) -> ()>,
                actionf_p1,
            >(Some(T_MoveCeiling as unsafe extern "C" fn(*mut ceiling_t) -> ()))
        {
            saveg_write8(tc_ceiling as i32 as byte);
            saveg_write_pad();
            saveg_write_ceiling_t(th as *mut ceiling_t);
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut vldoor_t) -> ()>,
                actionf_p1,
            >(Some(T_VerticalDoor as unsafe extern "C" fn(*mut vldoor_t) -> ()))
        {
            saveg_write8(tc_door as i32 as byte);
            saveg_write_pad();
            saveg_write_vldoor_t(th as *mut vldoor_t);
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut floormove_t) -> ()>,
                actionf_p1,
            >(Some(T_MoveFloor as unsafe extern "C" fn(*mut floormove_t) -> ()))
        {
            saveg_write8(tc_floor as i32 as byte);
            saveg_write_pad();
            saveg_write_floormove_t(th as *mut floormove_t);
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut plat_t) -> ()>,
                actionf_p1,
            >(Some(T_PlatRaise as unsafe extern "C" fn(*mut plat_t) -> ()))
        {
            saveg_write8(tc_plat as i32 as byte);
            saveg_write_pad();
            saveg_write_plat_t(th as *mut plat_t);
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut lightflash_t) -> ()>,
                actionf_p1,
            >(Some(T_LightFlash as unsafe extern "C" fn(*mut lightflash_t) -> ()))
        {
            saveg_write8(tc_flash as i32 as byte);
            saveg_write_pad();
            saveg_write_lightflash_t(th as *mut lightflash_t);
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut strobe_t) -> ()>,
                actionf_p1,
            >(Some(T_StrobeFlash as unsafe extern "C" fn(*mut strobe_t) -> ()))
        {
            saveg_write8(tc_strobe as i32 as byte);
            saveg_write_pad();
            saveg_write_strobe_t(th as *mut strobe_t);
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut glow_t) -> ()>,
                actionf_p1,
            >(Some(T_Glow as unsafe extern "C" fn(*mut glow_t) -> ()))
        {
            saveg_write8(tc_glow as i32 as byte);
            saveg_write_pad();
            saveg_write_glow_t(th as *mut glow_t);
        }
        th = (*th).next as *mut thinker_t;
    }
    saveg_write8(tc_endspecials as i32 as byte);
}
pub unsafe fn P_UnArchiveSpecials() {
    let mut tclass: byte = 0;
    let mut ceiling: *mut ceiling_t = ::core::ptr::null_mut::<ceiling_t>();
    let mut door: *mut vldoor_t = ::core::ptr::null_mut::<vldoor_t>();
    let mut floor: *mut floormove_t = ::core::ptr::null_mut::<floormove_t>();
    let mut plat: *mut plat_t = ::core::ptr::null_mut::<plat_t>();
    let mut flash: *mut lightflash_t = ::core::ptr::null_mut::<lightflash_t>();
    let mut strobe: *mut strobe_t = ::core::ptr::null_mut::<strobe_t>();
    let mut glow: *mut glow_t = ::core::ptr::null_mut::<glow_t>();
    loop {
        tclass = saveg_read8();
        match tclass as i32 {
            7 => return,
            0 => {
                saveg_read_pad();
                ceiling = Z_Malloc(
                    ::core::mem::size_of::<ceiling_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut ceiling_t;
                saveg_read_ceiling_t(ceiling);
                (*(*ceiling).sector).specialdata = ceiling as *mut ::core::ffi::c_void;
                if (*ceiling).thinker.function.acp1.is_some() {
                    (*ceiling).thinker.function.acp1 = ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut ceiling_t) -> ()>,
                        actionf_p1,
                    >(Some(T_MoveCeiling as unsafe extern "C" fn(*mut ceiling_t) -> ()));
                }
                P_AddThinker(&raw mut (*ceiling).thinker);
                P_AddActiveCeiling(ceiling);
            }
            1 => {
                saveg_read_pad();
                door = Z_Malloc(
                    ::core::mem::size_of::<vldoor_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut vldoor_t;
                saveg_read_vldoor_t(door);
                (*(*door).sector).specialdata = door as *mut ::core::ffi::c_void;
                (*door).thinker.function.acp1 = ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut vldoor_t) -> ()>,
                    actionf_p1,
                >(Some(T_VerticalDoor as unsafe extern "C" fn(*mut vldoor_t) -> ()));
                P_AddThinker(&raw mut (*door).thinker);
            }
            2 => {
                saveg_read_pad();
                floor = Z_Malloc(
                    ::core::mem::size_of::<floormove_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut floormove_t;
                saveg_read_floormove_t(floor);
                (*(*floor).sector).specialdata = floor as *mut ::core::ffi::c_void;
                (*floor).thinker.function.acp1 = ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut floormove_t) -> ()>,
                    actionf_p1,
                >(Some(T_MoveFloor as unsafe extern "C" fn(*mut floormove_t) -> ()));
                P_AddThinker(&raw mut (*floor).thinker);
            }
            3 => {
                saveg_read_pad();
                plat = Z_Malloc(
                    ::core::mem::size_of::<plat_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut plat_t;
                saveg_read_plat_t(plat);
                (*(*plat).sector).specialdata = plat as *mut ::core::ffi::c_void;
                if (*plat).thinker.function.acp1.is_some() {
                    (*plat).thinker.function.acp1 = ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut plat_t) -> ()>,
                        actionf_p1,
                    >(Some(T_PlatRaise as unsafe extern "C" fn(*mut plat_t) -> ()));
                }
                P_AddThinker(&raw mut (*plat).thinker);
                P_AddActivePlat(plat);
            }
            4 => {
                saveg_read_pad();
                flash = Z_Malloc(
                    ::core::mem::size_of::<lightflash_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut lightflash_t;
                saveg_read_lightflash_t(flash);
                (*flash).thinker.function.acp1 = ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut lightflash_t) -> ()>,
                    actionf_p1,
                >(Some(T_LightFlash as unsafe extern "C" fn(*mut lightflash_t) -> ()));
                P_AddThinker(&raw mut (*flash).thinker);
            }
            5 => {
                saveg_read_pad();
                strobe = Z_Malloc(
                    ::core::mem::size_of::<strobe_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut strobe_t;
                saveg_read_strobe_t(strobe);
                (*strobe).thinker.function.acp1 = ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut strobe_t) -> ()>,
                    actionf_p1,
                >(Some(T_StrobeFlash as unsafe extern "C" fn(*mut strobe_t) -> ()));
                P_AddThinker(&raw mut (*strobe).thinker);
            }
            6 => {
                saveg_read_pad();
                glow = Z_Malloc(
                    ::core::mem::size_of::<glow_t>() as i32,
                    PU_LEVEL as i32,
                    NULL,
                ) as *mut glow_t;
                saveg_read_glow_t(glow);
                (*glow).thinker.function.acp1 = ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut glow_t) -> ()>,
                    actionf_p1,
                >(Some(T_Glow as unsafe extern "C" fn(*mut glow_t) -> ()));
                P_AddThinker(&raw mut (*glow).thinker);
            }
            _ => {
                I_Error(&format!(
                    "P_UnarchiveSpecials:Unknown tclass {} in savegame",
                    tclass as i32,
                ));
            }
        }
    };
}
