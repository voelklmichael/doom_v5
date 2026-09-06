pub struct ICdMusState {
    cd_Error: i32,
}

impl ICdMusState {
    pub const fn new() -> Self {
        ICdMusState { cd_Error: 0 }
    }
}

pub unsafe fn I_CDMusInit() -> i32 {
    return 0 as i32;
}
pub unsafe fn I_CDMusPlay(
    mut track: i32,
) -> i32 {
    return 0 as i32;
}
pub unsafe fn I_CDMusStop() -> i32 {
    return 0 as i32;
}
pub unsafe fn I_CDMusResume() -> i32 {
    return 0 as i32;
}
pub fn I_CDMusSetVolume(
    state: &mut ICdMusState,
    mut volume: i32,
) -> i32 {
    state.cd_Error = 0 as i32;
    return 0 as i32;
}
pub unsafe fn I_CDMusFirstTrack() -> i32 {
    return 0 as i32;
}
pub unsafe fn I_CDMusLastTrack() -> i32 {
    return 0 as i32;
}
pub unsafe fn I_CDMusTrackLength(
    mut track_num: i32,
) -> i32 {
    return 0 as i32;
}
