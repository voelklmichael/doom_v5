use crate::src::stdint_types::uint32_t;
use crate::src::doomdef::TICRATE;
extern "C" {
    fn DG_SleepMs(ms: uint32_t);
    fn DG_GetTicksMs() -> uint32_t;
}
pub struct ITimerState {
    basetime: uint32_t,
}

impl ITimerState {
    pub const fn new() -> Self {
        ITimerState { basetime: 0 }
    }
}

pub unsafe fn I_GetTicks() -> i32 {
    return DG_GetTicksMs() as i32;
}
pub fn I_GetTime(state: &mut ITimerState) -> i32 {
    let mut ticks: uint32_t = 0;
    ticks = unsafe { I_GetTicks() } as uint32_t;
    if state.basetime == 0 as uint32_t {
        state.basetime = ticks;
    }
    ticks = ticks.wrapping_sub(state.basetime);
    return ticks.wrapping_mul(TICRATE as uint32_t).wrapping_div(1000 as uint32_t)
        as i32;
}
pub fn I_GetTimeMS(state: &mut ITimerState) -> i32 {
    let mut ticks: uint32_t = 0;
    ticks = unsafe { I_GetTicks() } as uint32_t;
    if state.basetime == 0 as uint32_t {
        state.basetime = ticks;
    }
    return ticks.wrapping_sub(state.basetime) as i32;
}
pub unsafe fn I_Sleep(mut ms: i32) {
    DG_SleepMs(ms as uint32_t);
}
