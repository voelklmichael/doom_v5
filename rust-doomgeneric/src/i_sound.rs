use crate::src::i_video::screensaver_mode;
use crate::src::m_argv::M_CheckParm;
use crate::src::m_config::M_BindVariable;

use crate::src::doomdef::boolean;
use crate::src::doomdef::NULL;
use crate::src::sounds::sfxinfo_t;
pub type snddevice_t = u32;
pub const SNDDEVICE_CD: snddevice_t = 10;
pub const SNDDEVICE_AWE32: snddevice_t = 9;
pub const SNDDEVICE_GENMIDI: snddevice_t = 8;
pub const SNDDEVICE_SOUNDCANVAS: snddevice_t = 7;
pub const SNDDEVICE_WAVEBLASTER: snddevice_t = 6;
pub const SNDDEVICE_GUS: snddevice_t = 5;
pub const SNDDEVICE_PAS: snddevice_t = 4;
pub const SNDDEVICE_SB: snddevice_t = 3;
pub const SNDDEVICE_ADLIB: snddevice_t = 2;
pub const SNDDEVICE_PCSPEAKER: snddevice_t = 1;
pub const SNDDEVICE_NONE: snddevice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sound_module_t {
    pub sound_devices: *mut snddevice_t,
    pub num_sound_devices: i32,
    pub Init: Option<unsafe extern "C" fn(boolean) -> boolean>,
    pub Shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub GetSfxLumpNum: Option<unsafe extern "C" fn(*mut sfxinfo_t) -> i32>,
    pub Update: Option<unsafe extern "C" fn() -> ()>,
    pub UpdateSoundParams: Option<unsafe extern "C" fn(i32, i32, i32) -> ()>,
    pub StartSound: Option<unsafe extern "C" fn(*mut sfxinfo_t, i32, i32, i32) -> i32>,
    pub StopSound: Option<unsafe extern "C" fn(i32) -> ()>,
    pub SoundIsPlaying: Option<unsafe extern "C" fn(i32) -> boolean>,
    pub CacheSounds: Option<unsafe extern "C" fn(*mut sfxinfo_t, i32) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct music_module_t {
    pub sound_devices: *mut snddevice_t,
    pub num_sound_devices: i32,
    pub Init: Option<unsafe extern "C" fn() -> boolean>,
    pub Shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub SetMusicVolume: Option<unsafe extern "C" fn(i32) -> ()>,
    pub PauseMusic: Option<unsafe extern "C" fn() -> ()>,
    pub ResumeMusic: Option<unsafe extern "C" fn() -> ()>,
    pub RegisterSong:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, i32) -> *mut ::core::ffi::c_void>,
    pub UnRegisterSong: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub PlaySong: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, boolean) -> ()>,
    pub StopSong: Option<unsafe extern "C" fn() -> ()>,
    pub MusicIsPlaying: Option<unsafe extern "C" fn() -> boolean>,
    pub Poll: Option<unsafe extern "C" fn() -> ()>,
}
static mut sound_modules: [*mut sound_module_t; 1] =
    [::core::ptr::null::<sound_module_t>() as *mut sound_module_t];

pub struct ISoundState {
    pub snd_samplerate: i32,
    pub snd_cachesize: i32,
    pub snd_maxslicetime_ms: i32,
    pub snd_musiccmd: *mut ::core::ffi::c_char,
    sound_module: *mut sound_module_t,
    music_module: *mut music_module_t,
    pub snd_musicdevice: i32,
    pub snd_sfxdevice: i32,
    snd_sbport: i32,
    snd_sbirq: i32,
    snd_sbdma: i32,
    snd_mport: i32,
}

impl ISoundState {
    pub const fn new() -> Self {
        ISoundState {
            snd_samplerate: 44100,
            snd_cachesize: 64 * 1024 * 1024,
            snd_maxslicetime_ms: 28,
            snd_musiccmd: b"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            sound_module: ::core::ptr::null::<sound_module_t>() as *mut sound_module_t,
            music_module: ::core::ptr::null::<music_module_t>() as *mut music_module_t,
            snd_musicdevice: SNDDEVICE_SB as i32,
            snd_sfxdevice: SNDDEVICE_SB as i32,
            snd_sbport: 0,
            snd_sbirq: 0,
            snd_sbdma: 0,
            snd_mport: 0,
        }
    }
}
unsafe fn SndDeviceInList(
    mut device: snddevice_t,
    mut list: *mut snddevice_t,
    mut len: i32,
) -> bool {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < len {
        if device as u32 == *list.offset(i as isize) as u32 {
            return true;
        }
        i += 1;
    }
    return false;
}
unsafe fn InitSfxModule(state: &mut ISoundState, mut use_sfx_prefix: bool) {
    let mut i: i32 = 0;
    state.sound_module = ::core::ptr::null_mut::<sound_module_t>();
    i = 0 as i32;
    while !sound_modules[i as usize].is_null() {
        if SndDeviceInList(
            state.snd_sfxdevice as snddevice_t,
            (*sound_modules[i as usize]).sound_devices,
            (*sound_modules[i as usize]).num_sound_devices,
        ) {
            if (*sound_modules[i as usize])
                .Init
                .expect("non-null function pointer")(use_sfx_prefix as i32 as boolean)
                != 0
            {
                state.sound_module = sound_modules[i as usize];
                return;
            }
        }
        i += 1;
    }
}
pub unsafe fn I_InitSound(state: &mut ISoundState, mut use_sfx_prefix: bool) {
    let mut nosound: bool = false;
    let mut nosfx: bool = false;
    nosound = M_CheckParm("-nosound") > 0 as i32;
    nosfx = M_CheckParm("-nosfx") > 0 as i32;
    if !nosound && !screensaver_mode {
        if !nosfx {
            InitSfxModule(state, use_sfx_prefix);
        }
    }
}
pub unsafe fn I_ShutdownSound(state: &mut ISoundState) {
    if !state.sound_module.is_null() {
        (*state.sound_module)
            .Shutdown
            .expect("non-null function pointer")();
    }
    if !state.music_module.is_null() {
        (*state.music_module)
            .Shutdown
            .expect("non-null function pointer")();
    }
}
pub unsafe fn I_GetSfxLumpNum(state: &mut ISoundState, mut sfxinfo: *mut sfxinfo_t) -> i32 {
    if !state.sound_module.is_null() {
        return (*state.sound_module)
            .GetSfxLumpNum
            .expect("non-null function pointer")(sfxinfo);
    } else {
        return 0 as i32;
    };
}
pub unsafe fn I_UpdateSound(state: &mut ISoundState) {
    if !state.sound_module.is_null() {
        (*state.sound_module)
            .Update
            .expect("non-null function pointer")();
    }
    if !state.music_module.is_null() && (*state.music_module).Poll.is_some() {
        (*state.music_module)
            .Poll
            .expect("non-null function pointer")();
    }
}
unsafe fn CheckVolumeSeparation(mut vol: *mut i32, mut sep: *mut i32) {
    if *sep < 0 as i32 {
        *sep = 0 as i32;
    } else if *sep > 254 as i32 {
        *sep = 254 as i32;
    }
    if *vol < 0 as i32 {
        *vol = 0 as i32;
    } else if *vol > 127 as i32 {
        *vol = 127 as i32;
    }
}
pub unsafe fn I_UpdateSoundParams(
    state: &mut ISoundState,
    mut channel: i32,
    mut vol: i32,
    mut sep: i32,
) {
    if !state.sound_module.is_null() {
        CheckVolumeSeparation(&raw mut vol, &raw mut sep);
        (*state.sound_module)
            .UpdateSoundParams
            .expect("non-null function pointer")(channel, vol, sep);
    }
}
pub unsafe fn I_StartSound(
    state: &mut ISoundState,
    mut sfxinfo: *mut sfxinfo_t,
    mut channel: i32,
    mut vol: i32,
    mut sep: i32,
) -> i32 {
    if !state.sound_module.is_null() {
        CheckVolumeSeparation(&raw mut vol, &raw mut sep);
        return (*state.sound_module)
            .StartSound
            .expect("non-null function pointer")(sfxinfo, channel, vol, sep);
    } else {
        return 0 as i32;
    };
}
pub unsafe fn I_StopSound(state: &mut ISoundState, mut channel: i32) {
    if !state.sound_module.is_null() {
        (*state.sound_module)
            .StopSound
            .expect("non-null function pointer")(channel);
    }
}
pub unsafe fn I_SoundIsPlaying(state: &mut ISoundState, mut channel: i32) -> bool {
    if !state.sound_module.is_null() {
        return (*state.sound_module)
            .SoundIsPlaying
            .expect("non-null function pointer")(channel)
            != 0;
    } else {
        return false;
    };
}
pub unsafe fn I_PrecacheSounds(
    state: &mut ISoundState,
    mut sounds: *mut sfxinfo_t,
    mut num_sounds: i32,
) {
    if !state.sound_module.is_null() && (*state.sound_module).CacheSounds.is_some() {
        (*state.sound_module)
            .CacheSounds
            .expect("non-null function pointer")(sounds, num_sounds);
    }
}
pub unsafe fn I_InitMusic(state: &mut ISoundState) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .Init
            .expect("non-null function pointer")();
    }
}
pub unsafe fn I_SetMusicVolume(state: &mut ISoundState, mut volume: i32) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .SetMusicVolume
            .expect("non-null function pointer")(volume);
    }
}
pub unsafe fn I_PauseSong(state: &mut ISoundState) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .PauseMusic
            .expect("non-null function pointer")();
    }
}
pub unsafe fn I_ResumeSong(state: &mut ISoundState) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .ResumeMusic
            .expect("non-null function pointer")();
    }
}
pub unsafe fn I_RegisterSong(
    state: &mut ISoundState,
    mut data: *mut ::core::ffi::c_void,
    mut len: i32,
) -> *mut ::core::ffi::c_void {
    if !state.music_module.is_null() {
        return (*state.music_module)
            .RegisterSong
            .expect("non-null function pointer")(data, len);
    } else {
        return NULL;
    };
}
pub unsafe fn I_UnRegisterSong(state: &mut ISoundState, mut handle: *mut ::core::ffi::c_void) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .UnRegisterSong
            .expect("non-null function pointer")(handle);
    }
}
pub unsafe fn I_PlaySong(
    state: &mut ISoundState,
    mut handle: *mut ::core::ffi::c_void,
    mut looping: bool,
) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .PlaySong
            .expect("non-null function pointer")(handle, looping as i32 as boolean);
    }
}
pub unsafe fn I_StopSong(state: &mut ISoundState) {
    if !state.music_module.is_null() {
        (*state.music_module)
            .StopSong
            .expect("non-null function pointer")();
    }
}
pub unsafe fn I_MusicIsPlaying(state: &mut ISoundState) -> bool {
    if !state.music_module.is_null() {
        return (*state.music_module)
            .MusicIsPlaying
            .expect("non-null function pointer")()
            != 0;
    } else {
        return false;
    };
}
pub unsafe fn I_BindSoundVariables(state: &mut ISoundState) {
    extern "C" {
        static mut use_libsamplerate: i32;
    }
    extern "C" {
        static mut libsamplerate_scale: f32;
    }
    M_BindVariable(
        "snd_musicdevice",
        &raw mut state.snd_musicdevice as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_sfxdevice",
        &raw mut state.snd_sfxdevice as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_sbport",
        &raw mut state.snd_sbport as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_sbirq",
        &raw mut state.snd_sbirq as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_sbdma",
        &raw mut state.snd_sbdma as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_mport",
        &raw mut state.snd_mport as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_maxslicetime_ms",
        &raw mut state.snd_maxslicetime_ms as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_musiccmd",
        &raw mut state.snd_musiccmd as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_samplerate",
        &raw mut state.snd_samplerate as *mut ::core::ffi::c_void,
    );
    M_BindVariable(
        "snd_cachesize",
        &raw mut state.snd_cachesize as *mut ::core::ffi::c_void,
    );
}
