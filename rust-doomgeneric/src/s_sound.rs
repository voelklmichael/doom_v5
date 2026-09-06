use crate::src::p_mobj::mobj_t;
use crate::src::i_system::I_Error;
use crate::src::w_wad::{wad_name8_to_string, W_GetNumForName};
use crate::src::i_sound::I_ShutdownSound;
use crate::src::i_sound::I_GetSfxLumpNum;
use crate::src::i_sound::I_UpdateSoundParams;
use crate::src::i_sound::I_StartSound;
use crate::src::i_sound::I_StopSound;
use crate::src::i_sound::I_SoundIsPlaying;
use crate::src::i_sound::I_PrecacheSounds;
use crate::src::i_sound::I_ShutdownMusic;
use crate::src::i_sound::I_SetMusicVolume;
use crate::src::i_sound::I_PauseSong;
use crate::src::i_sound::I_ResumeSong;
use crate::src::i_sound::I_RegisterSong;
use crate::src::i_sound::I_UnRegisterSong;
use crate::src::i_sound::I_PlaySong;
use crate::src::i_sound::I_StopSong;
use crate::src::i_sound::I_MusicIsPlaying;
use crate::src::i_sound::snd_musicdevice;
use crate::src::sounds::S_sfx;
use crate::src::sounds::S_music;
use crate::src::i_system::I_AtExit;
use crate::src::g_game::gameepisode;
use crate::src::g_game::gamemap;
use crate::src::r_main::R_PointToAngle2;
use crate::src::m_misc::M_snprintf;
use crate::src::g_game::consoleplayer;
use crate::src::tables::finesine;
use crate::src::m_fixed::FixedMul;
use crate::src::g_game::players;
use crate::src::doomstat::gamemode;
use crate::src::i_sound::I_UpdateSound;
use crate::src::w_wad::W_LumpLength;
use crate::src::w_wad::W_ReleaseLumpNum;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::z_zone::Z_Malloc;

use crate::src::sounds::{sfxinfo_t, musicinfo_t};
use crate::src::z_zone::PU_STATIC;
use crate::src::sounds::NUMSFX;
use crate::src::sounds::{NUMMUSIC, mus_None, mus_e1m1, mus_e1m5, mus_e1m9, mus_e2m4, mus_e2m5, mus_e2m6, mus_e2m7, mus_e3m2, mus_e3m3, mus_e3m4, mus_intro, mus_introa, mus_runnin};
use crate::src::d_mode::commercial;
use crate::src::tables::angle_t;
use crate::src::m_fixed::fixed_t;
use crate::src::stdint_types::size_t;
use crate::src::i_sound::{SNDDEVICE_ADLIB, SNDDEVICE_SB};
use crate::src::doomdef::NULL;
use crate::src::doomdef::true_0;
use crate::src::doomdef::false_0;
use crate::src::m_fixed::FRACUNIT;
use crate::src::tables::ANGLETOFINESHIFT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_t {
    pub sfxinfo: *mut sfxinfo_t,
    pub origin: *mut mobj_t,
    pub handle: i32,
}
pub const FRACBITS: i32 = 16 as i32;
pub const S_CLIPPING_DIST: i32 = 1200 as i32 * FRACUNIT;
pub const S_CLOSE_DIST: i32 = 200 as i32 * FRACUNIT;
pub const S_ATTENUATOR: i32 = S_CLIPPING_DIST - S_CLOSE_DIST >> FRACBITS;
pub const S_STEREO_SWING: i32 = 96 as i32 * FRACUNIT;
pub const NORM_SEP: i32 = 128 as i32;
static mut channels: *mut channel_t = ::core::ptr::null::<channel_t>() as *mut channel_t;
pub static mut sfxVolume: i32 = 8 as i32;
pub static mut musicVolume: i32 = 8 as i32;
static mut snd_SfxVolume: i32 = 0;
static mut mus_paused: bool = false;
static mut mus_playing: *mut musicinfo_t = ::core::ptr::null::<musicinfo_t>()
    as *mut musicinfo_t;
pub static mut snd_channels: i32 = 8 as i32;
pub unsafe fn S_Init(
    mut sfxVolume_0: i32,
    mut musicVolume_0: i32,
) {
    let mut i: i32 = 0;
    I_PrecacheSounds(&raw mut S_sfx as *mut sfxinfo_t, NUMSFX as i32);
    S_SetSfxVolume(sfxVolume_0);
    S_SetMusicVolume(musicVolume_0);
    channels = Z_Malloc(
        (snd_channels as usize)
            .wrapping_mul(::core::mem::size_of::<channel_t>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut channel_t;
    i = 0 as i32;
    while i < snd_channels {
        let ref mut fresh0 = (*channels.offset(i as isize)).sfxinfo;
        *fresh0 = ::core::ptr::null_mut::<sfxinfo_t>();
        i += 1;
    }
    mus_paused = false;
    i = 1 as i32;
    while i < NUMSFX as i32 {
        let ref mut fresh1 = (*(&raw mut S_sfx as *mut sfxinfo_t).offset(i as isize))
            .usefulness;
        *fresh1 = -(1 as i32);
        (*(&raw mut S_sfx as *mut sfxinfo_t).offset(i as isize)).lumpnum = *fresh1;
        i += 1;
    }
    I_AtExit(Some(S_Shutdown as unsafe extern "C" fn() -> ()), true);
}
#[no_mangle]
pub unsafe extern "C" fn S_Shutdown() {
    I_ShutdownSound();
    I_ShutdownMusic();
}
unsafe extern "C" fn S_StopChannel(mut cnum: i32) {
    let mut i: i32 = 0;
    let mut c: *mut channel_t = ::core::ptr::null_mut::<channel_t>();
    c = channels.offset(cnum as isize) as *mut channel_t;
    if !(*c).sfxinfo.is_null() {
        if I_SoundIsPlaying((*c).handle) {
            I_StopSound((*c).handle);
        }
        i = 0 as i32;
        while i < snd_channels {
            if cnum != i && (*c).sfxinfo == (*channels.offset(i as isize)).sfxinfo {
                break;
            }
            i += 1;
        }
        (*(*c).sfxinfo).usefulness -= 1;
        (*c).sfxinfo = ::core::ptr::null_mut::<sfxinfo_t>();
    }
}
pub unsafe fn S_Start() {
    let mut cnum: i32 = 0;
    let mut mnum: i32 = 0;
    cnum = 0 as i32;
    while cnum < snd_channels {
        if !(*channels.offset(cnum as isize)).sfxinfo.is_null() {
            S_StopChannel(cnum);
        }
        cnum += 1;
    }
    mus_paused = false;
    if gamemode as u32
        == commercial as i32 as u32
    {
        mnum = mus_runnin as i32 + gamemap - 1 as i32;
    } else {
        let mut spmus: [i32; 9] = [
            mus_e3m4 as i32,
            mus_e3m2 as i32,
            mus_e3m3 as i32,
            mus_e1m5 as i32,
            mus_e2m7 as i32,
            mus_e2m4 as i32,
            mus_e2m6 as i32,
            mus_e2m5 as i32,
            mus_e1m9 as i32,
        ];
        if gameepisode < 4 as i32 {
            mnum = mus_e1m1 as i32
                + (gameepisode - 1 as i32) * 9 as i32
                + gamemap - 1 as i32;
        } else {
            mnum = spmus[(gamemap - 1 as i32) as usize];
        }
    }
    S_ChangeMusic(mnum, true_0);
}
pub unsafe fn S_StopSound(mut origin: *mut mobj_t) {
    let mut cnum: i32 = 0;
    cnum = 0 as i32;
    while cnum < snd_channels {
        if !(*channels.offset(cnum as isize)).sfxinfo.is_null()
            && (*channels.offset(cnum as isize)).origin == origin
        {
            S_StopChannel(cnum);
            break;
        } else {
            cnum += 1;
        }
    }
}
unsafe extern "C" fn S_GetChannel(
    mut origin: *mut mobj_t,
    mut sfxinfo: *mut sfxinfo_t,
) -> i32 {
    let mut cnum: i32 = 0;
    let mut c: *mut channel_t = ::core::ptr::null_mut::<channel_t>();
    cnum = 0 as i32;
    while cnum < snd_channels {
        if (*channels.offset(cnum as isize)).sfxinfo.is_null() {
            break;
        }
        if !origin.is_null() && (*channels.offset(cnum as isize)).origin == origin {
            S_StopChannel(cnum);
            break;
        } else {
            cnum += 1;
        }
    }
    if cnum == snd_channels {
        cnum = 0 as i32;
        while cnum < snd_channels {
            if (*(*channels.offset(cnum as isize)).sfxinfo).priority
                >= (*sfxinfo).priority
            {
                break;
            }
            cnum += 1;
        }
        if cnum == snd_channels {
            return -(1 as i32)
        } else {
            S_StopChannel(cnum);
        }
    }
    c = channels.offset(cnum as isize) as *mut channel_t;
    (*c).sfxinfo = sfxinfo;
    (*c).origin = origin;
    return cnum;
}
unsafe extern "C" fn S_AdjustSoundParams(
    mut listener: *mut mobj_t,
    mut source: *mut mobj_t,
    mut vol: *mut i32,
    mut sep: *mut i32,
) -> i32 {
    let mut approx_dist: fixed_t = 0;
    let mut adx: fixed_t = 0;
    let mut ady: fixed_t = 0;
    let mut angle: angle_t = 0;
    adx = ((*listener).x as i32 - (*source).x as i32).abs()
        as fixed_t;
    ady = ((*listener).y as i32 - (*source).y as i32).abs()
        as fixed_t;
    approx_dist = adx + ady
        - ((if adx < ady { adx } else { ady }) >> 1 as i32);
    if gamemap != 8 as i32 && approx_dist > S_CLIPPING_DIST {
        return 0 as i32;
    }
    angle = R_PointToAngle2((*listener).x, (*listener).y, (*source).x, (*source).y);
    if angle > (*listener).angle {
        angle = angle.wrapping_sub((*listener).angle);
    } else {
        angle = angle
            .wrapping_add((0xffffffff as angle_t).wrapping_sub((*listener).angle));
    }
    angle >>= ANGLETOFINESHIFT;
    *sep = (128 as fixed_t
        - (FixedMul(S_STEREO_SWING, finesine[angle as usize]) >> FRACBITS))
        as i32;
    if approx_dist < S_CLOSE_DIST {
        *vol = snd_SfxVolume;
    } else if gamemap == 8 as i32 {
        if approx_dist > S_CLIPPING_DIST {
            approx_dist = S_CLIPPING_DIST as fixed_t;
        }
        *vol = 15 as i32
            + (snd_SfxVolume - 15 as i32)
                * (S_CLIPPING_DIST - approx_dist as i32 >> FRACBITS)
                / S_ATTENUATOR;
    } else {
        *vol = snd_SfxVolume
            * (S_CLIPPING_DIST - approx_dist as i32 >> FRACBITS)
            / S_ATTENUATOR;
    }
    return (*vol > 0 as i32) as i32;
}
pub unsafe fn S_StartSound(
    mut origin_p: *mut ::core::ffi::c_void,
    mut sfx_id: i32,
) {
    let mut sfx: *mut sfxinfo_t = ::core::ptr::null_mut::<sfxinfo_t>();
    let mut origin: *mut mobj_t = ::core::ptr::null_mut::<mobj_t>();
    let mut rc: i32 = 0;
    let mut sep: i32 = 0;
    let mut cnum: i32 = 0;
    let mut volume: i32 = 0;
    origin = origin_p as *mut mobj_t;
    volume = snd_SfxVolume;
    if sfx_id < 1 as i32 || sfx_id > NUMSFX as i32 {
        I_Error(&format!("Bad sfx #: {}", sfx_id));
    }
    sfx = (&raw mut S_sfx as *mut sfxinfo_t).offset(sfx_id as isize) as *mut sfxinfo_t;
    if !(*sfx).link.is_null() {
        volume += (*sfx).volume;
        if volume < 1 as i32 {
            return;
        }
        if volume > snd_SfxVolume {
            volume = snd_SfxVolume;
        }
    }
    if !origin.is_null() && origin != players[consoleplayer as usize].mo {
        rc = S_AdjustSoundParams(
            players[consoleplayer as usize].mo,
            origin,
            &raw mut volume,
            &raw mut sep,
        );
        if (*origin).x == (*players[consoleplayer as usize].mo).x
            && (*origin).y == (*players[consoleplayer as usize].mo).y
        {
            sep = NORM_SEP;
        }
        if rc == 0 {
            return;
        }
    } else {
        sep = NORM_SEP;
    }
    S_StopSound(origin);
    cnum = S_GetChannel(origin, sfx);
    if cnum < 0 as i32 {
        return;
    }
    let fresh2 = (*sfx).usefulness;
    (*sfx).usefulness = (*sfx).usefulness + 1;
    if fresh2 < 0 as i32 {
        (*sfx).usefulness = 1 as i32;
    }
    if (*sfx).lumpnum < 0 as i32 {
        (*sfx).lumpnum = I_GetSfxLumpNum(sfx);
    }
    (*channels.offset(cnum as isize)).handle = I_StartSound(sfx, cnum, volume, sep);
}
pub unsafe fn S_PauseSound() {
    if !mus_playing.is_null() && !mus_paused {
        I_PauseSong();
        mus_paused = true;
    }
}
pub unsafe fn S_ResumeSound() {
    if !mus_playing.is_null() && mus_paused {
        I_ResumeSong();
        mus_paused = false;
    }
}
pub unsafe fn S_UpdateSounds(mut listener: *mut mobj_t) {
    let mut audible: i32 = 0;
    let mut cnum: i32 = 0;
    let mut volume: i32 = 0;
    let mut sep: i32 = 0;
    let mut sfx: *mut sfxinfo_t = ::core::ptr::null_mut::<sfxinfo_t>();
    let mut c: *mut channel_t = ::core::ptr::null_mut::<channel_t>();
    I_UpdateSound();
    let mut current_block_20: u64;
    cnum = 0 as i32;
    while cnum < snd_channels {
        c = channels.offset(cnum as isize) as *mut channel_t;
        sfx = (*c).sfxinfo;
        if !(*c).sfxinfo.is_null() {
            if I_SoundIsPlaying((*c).handle) {
                volume = snd_SfxVolume;
                sep = NORM_SEP;
                if !(*sfx).link.is_null() {
                    volume += (*sfx).volume;
                    if volume < 1 as i32 {
                        S_StopChannel(cnum);
                        current_block_20 = 10680521327981672866;
                    } else {
                        if volume > snd_SfxVolume {
                            volume = snd_SfxVolume;
                        }
                        current_block_20 = 17860125682698302841;
                    }
                } else {
                    current_block_20 = 17860125682698302841;
                }
                match current_block_20 {
                    10680521327981672866 => {}
                    _ => {
                        if !(*c).origin.is_null() && listener != (*c).origin {
                            audible = S_AdjustSoundParams(
                                listener,
                                (*c).origin,
                                &raw mut volume,
                                &raw mut sep,
                            );
                            if audible == 0 {
                                S_StopChannel(cnum);
                            } else {
                                I_UpdateSoundParams((*c).handle, volume, sep);
                            }
                        }
                    }
                }
            } else {
                S_StopChannel(cnum);
            }
        }
        cnum += 1;
    }
}
pub unsafe fn S_SetMusicVolume(mut volume: i32) {
    if volume < 0 as i32 || volume > 127 as i32 {
        I_Error(&format!("Attempt to set music volume at {}", volume));
    }
    I_SetMusicVolume(volume);
}
pub unsafe fn S_SetSfxVolume(mut volume: i32) {
    if volume < 0 as i32 || volume > 127 as i32 {
        I_Error(&format!("Attempt to set sfx volume at {}", volume));
    }
    snd_SfxVolume = volume;
}
pub unsafe fn S_StartMusic(mut m_id: i32) {
    S_ChangeMusic(m_id, false_0);
}
pub unsafe fn S_ChangeMusic(
    mut musicnum: i32,
    mut looping: i32,
) {
    let mut music: *mut musicinfo_t = ::core::ptr::null_mut::<musicinfo_t>();
    let mut namebuf: [::core::ffi::c_char; 9] = [0; 9];
    let mut handle: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if musicnum == mus_intro as i32
        && (snd_musicdevice == SNDDEVICE_ADLIB as i32
            || snd_musicdevice == SNDDEVICE_SB as i32)
    {
        musicnum = mus_introa as i32;
    }
    if musicnum <= mus_None as i32
        || musicnum >= NUMMUSIC as i32
    {
        I_Error(&format!("Bad music number {}", musicnum));
    } else {
        music = (&raw mut S_music as *mut musicinfo_t).offset(musicnum as isize)
            as *mut musicinfo_t;
    }
    if mus_playing == music {
        return;
    }
    S_StopMusic();
    if (*music).lumpnum == 0 {
        M_snprintf(
            &raw mut namebuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
            b"d_%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*music).name,
        );
        (*music).lumpnum = W_GetNumForName(
            &wad_name8_to_string(&raw mut namebuf as *mut ::core::ffi::c_char),
        );
    }
    (*music).data = W_CacheLumpNum((*music).lumpnum, PU_STATIC as i32);
    handle = I_RegisterSong(
        (*music).data,
        W_LumpLength((*music).lumpnum as u32),
    );
    (*music).handle = handle;
    I_PlaySong(handle, looping != 0);
    mus_playing = music;
}
#[no_mangle]
pub unsafe extern "C" fn S_MusicPlaying() -> bool {
    return I_MusicIsPlaying();
}
#[no_mangle]
pub unsafe extern "C" fn S_StopMusic() {
    if !mus_playing.is_null() {
        if mus_paused {
            I_ResumeSong();
        }
        I_StopSong();
        I_UnRegisterSong((*mus_playing).handle);
        W_ReleaseLumpNum((*mus_playing).lumpnum);
        (*mus_playing).data = NULL;
        mus_playing = ::core::ptr::null_mut::<musicinfo_t>();
    }
}
