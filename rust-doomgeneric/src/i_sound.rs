use crate::src::m_argv::M_CheckParm;
use crate::src::m_config::M_BindVariable;
use crate::src::i_video::screensaver_mode;

use crate::src::sounds::sfxinfo_t;
use crate::src::doomdef::boolean;
use crate::src::doomdef::NULL;
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
    pub GetSfxLumpNum: Option<
        unsafe extern "C" fn(*mut sfxinfo_t) -> i32,
    >,
    pub Update: Option<unsafe extern "C" fn() -> ()>,
    pub UpdateSoundParams: Option<
        unsafe extern "C" fn(
            i32,
            i32,
            i32,
        ) -> (),
    >,
    pub StartSound: Option<
        unsafe extern "C" fn(
            *mut sfxinfo_t,
            i32,
            i32,
            i32,
        ) -> i32,
    >,
    pub StopSound: Option<unsafe extern "C" fn(i32) -> ()>,
    pub SoundIsPlaying: Option<unsafe extern "C" fn(i32) -> boolean>,
    pub CacheSounds: Option<
        unsafe extern "C" fn(*mut sfxinfo_t, i32) -> (),
    >,
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
    pub RegisterSong: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            i32,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub UnRegisterSong: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub PlaySong: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, boolean) -> ()>,
    pub StopSong: Option<unsafe extern "C" fn() -> ()>,
    pub MusicIsPlaying: Option<unsafe extern "C" fn() -> boolean>,
    pub Poll: Option<unsafe extern "C" fn() -> ()>,
}
#[no_mangle]
pub static mut snd_samplerate: i32 = 44100 as i32;
#[no_mangle]
pub static mut snd_cachesize: i32 = 64 as i32
    * 1024 as i32 * 1024 as i32;
#[no_mangle]
pub static mut snd_maxslicetime_ms: i32 = 28 as i32;
#[no_mangle]
pub static mut snd_musiccmd: *mut ::core::ffi::c_char = b"\0" as *const u8
    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut sound_module: *mut sound_module_t = ::core::ptr::null::<sound_module_t>()
    as *mut sound_module_t;
static mut music_module: *mut music_module_t = ::core::ptr::null::<music_module_t>()
    as *mut music_module_t;
pub static mut snd_musicdevice: i32 = SNDDEVICE_SB as i32;
#[no_mangle]
pub static mut snd_sfxdevice: i32 = SNDDEVICE_SB as i32;
static mut snd_sbport: i32 = 0 as i32;
static mut snd_sbirq: i32 = 0 as i32;
static mut snd_sbdma: i32 = 0 as i32;
static mut snd_mport: i32 = 0 as i32;
static mut sound_modules: [*mut sound_module_t; 1] = [
    ::core::ptr::null::<sound_module_t>() as *mut sound_module_t,
];
unsafe fn SndDeviceInList(
    mut device: snddevice_t,
    mut list: *mut snddevice_t,
    mut len: i32,
) -> bool {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < len {
        if device as u32
            == *list.offset(i as isize) as u32
        {
            return true;
        }
        i += 1;
    }
    return false;
}
unsafe fn InitSfxModule(mut use_sfx_prefix: bool) {
    let mut i: i32 = 0;
    sound_module = ::core::ptr::null_mut::<sound_module_t>();
    i = 0 as i32;
    while !sound_modules[i as usize].is_null() {
        if SndDeviceInList(
            snd_sfxdevice as snddevice_t,
            (*sound_modules[i as usize]).sound_devices,
            (*sound_modules[i as usize]).num_sound_devices,
        )
        {
            if (*sound_modules[i as usize])
                .Init
                .expect("non-null function pointer")(use_sfx_prefix as i32 as boolean) != 0
            {
                sound_module = sound_modules[i as usize];
                return;
            }
        }
        i += 1;
    }
}
pub unsafe fn I_InitSound(mut use_sfx_prefix: bool) {
    let mut nosound: bool = false;
    let mut nosfx: bool = false;
    let mut nomusic: boolean = 0;
    nosound = M_CheckParm("-nosound") > 0 as i32;
    nosfx = M_CheckParm("-nosfx") > 0 as i32;
    nomusic = (M_CheckParm("-nomusic") > 0 as i32) as i32
        as boolean;
    if !nosound && !screensaver_mode {
        nomusic == 0
            && (snd_musicdevice == SNDDEVICE_GENMIDI as i32
                || snd_musicdevice == SNDDEVICE_GUS as i32);
        if !nosfx {
            InitSfxModule(use_sfx_prefix);
        }
    }
}
pub unsafe fn I_ShutdownSound() {
    if !sound_module.is_null() {
        (*sound_module).Shutdown.expect("non-null function pointer")();
    }
    if !music_module.is_null() {
        (*music_module).Shutdown.expect("non-null function pointer")();
    }
}
pub unsafe fn I_GetSfxLumpNum(
    mut sfxinfo: *mut sfxinfo_t,
) -> i32 {
    if !sound_module.is_null() {
        return (*sound_module).GetSfxLumpNum.expect("non-null function pointer")(sfxinfo)
    } else {
        return 0 as i32
    };
}
pub unsafe fn I_UpdateSound() {
    if !sound_module.is_null() {
        (*sound_module).Update.expect("non-null function pointer")();
    }
    if !music_module.is_null() && (*music_module).Poll.is_some() {
        (*music_module).Poll.expect("non-null function pointer")();
    }
}
unsafe fn CheckVolumeSeparation(
    mut vol: *mut i32,
    mut sep: *mut i32,
) {
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
    mut channel: i32,
    mut vol: i32,
    mut sep: i32,
) {
    if !sound_module.is_null() {
        CheckVolumeSeparation(&raw mut vol, &raw mut sep);
        (*sound_module)
            .UpdateSoundParams
            .expect("non-null function pointer")(channel, vol, sep);
    }
}
pub unsafe fn I_StartSound(
    mut sfxinfo: *mut sfxinfo_t,
    mut channel: i32,
    mut vol: i32,
    mut sep: i32,
) -> i32 {
    if !sound_module.is_null() {
        CheckVolumeSeparation(&raw mut vol, &raw mut sep);
        return (*sound_module)
            .StartSound
            .expect("non-null function pointer")(sfxinfo, channel, vol, sep);
    } else {
        return 0 as i32
    };
}
pub unsafe fn I_StopSound(mut channel: i32) {
    if !sound_module.is_null() {
        (*sound_module).StopSound.expect("non-null function pointer")(channel);
    }
}
pub unsafe fn I_SoundIsPlaying(mut channel: i32) -> bool {
    if !sound_module.is_null() {
        return (*sound_module)
            .SoundIsPlaying
            .expect("non-null function pointer")(channel) != 0
    } else {
        return false
    };
}
pub unsafe fn I_PrecacheSounds(
    mut sounds: *mut sfxinfo_t,
    mut num_sounds: i32,
) {
    if !sound_module.is_null() && (*sound_module).CacheSounds.is_some() {
        (*sound_module)
            .CacheSounds
            .expect("non-null function pointer")(sounds, num_sounds);
    }
}
pub unsafe fn I_InitMusic() {
    if !music_module.is_null() {
        (*music_module).Init.expect("non-null function pointer")();
    }
}
pub unsafe fn I_SetMusicVolume(mut volume: i32) {
    if !music_module.is_null() {
        (*music_module).SetMusicVolume.expect("non-null function pointer")(volume);
    }
}
pub unsafe fn I_PauseSong() {
    if !music_module.is_null() {
        (*music_module).PauseMusic.expect("non-null function pointer")();
    }
}
pub unsafe fn I_ResumeSong() {
    if !music_module.is_null() {
        (*music_module).ResumeMusic.expect("non-null function pointer")();
    }
}
pub unsafe fn I_RegisterSong(
    mut data: *mut ::core::ffi::c_void,
    mut len: i32,
) -> *mut ::core::ffi::c_void {
    if !music_module.is_null() {
        return (*music_module)
            .RegisterSong
            .expect("non-null function pointer")(data, len)
    } else {
        return NULL
    };
}
pub unsafe fn I_UnRegisterSong(mut handle: *mut ::core::ffi::c_void) {
    if !music_module.is_null() {
        (*music_module).UnRegisterSong.expect("non-null function pointer")(handle);
    }
}
pub unsafe fn I_PlaySong(
    mut handle: *mut ::core::ffi::c_void,
    mut looping: bool,
) {
    if !music_module.is_null() {
        (*music_module).PlaySong.expect("non-null function pointer")(handle, looping as i32 as boolean);
    }
}
pub unsafe fn I_StopSong() {
    if !music_module.is_null() {
        (*music_module).StopSong.expect("non-null function pointer")();
    }
}
pub unsafe fn I_MusicIsPlaying() -> bool {
    if !music_module.is_null() {
        return (*music_module).MusicIsPlaying.expect("non-null function pointer")() != 0
    } else {
        return false
    };
}
pub unsafe fn I_BindSoundVariables() {
    extern "C" {
        static mut use_libsamplerate: i32;
    }
    extern "C" {
        static mut libsamplerate_scale: f32;
    }
    M_BindVariable("snd_musicdevice",
        &raw mut snd_musicdevice as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_sfxdevice",
        &raw mut snd_sfxdevice as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_sbport",
        &raw mut snd_sbport as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_sbirq",
        &raw mut snd_sbirq as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_sbdma",
        &raw mut snd_sbdma as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_mport",
        &raw mut snd_mport as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_maxslicetime_ms",
        &raw mut snd_maxslicetime_ms as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_musiccmd",
        &raw mut snd_musiccmd as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_samplerate",
        &raw mut snd_samplerate as *mut ::core::ffi::c_void,
    );
    M_BindVariable("snd_cachesize",
        &raw mut snd_cachesize as *mut ::core::ffi::c_void,
    );
}
