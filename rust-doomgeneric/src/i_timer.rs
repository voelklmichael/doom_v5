use crate::src::stdint_types::uint32_t;
use crate::src::doomdef::TICRATE;
extern "C" {
    fn DG_SleepMs(ms: uint32_t);
    fn DG_GetTicksMs() -> uint32_t;
}
static mut basetime: uint32_t = 0 as uint32_t;
pub unsafe fn I_GetTicks() -> i32 {
    return DG_GetTicksMs() as i32;
}
pub unsafe fn I_GetTime() -> i32 {
    let mut ticks: uint32_t = 0;
    ticks = I_GetTicks() as uint32_t;
    if basetime == 0 as uint32_t {
        basetime = ticks;
    }
    ticks = ticks.wrapping_sub(basetime);
    return ticks.wrapping_mul(TICRATE as uint32_t).wrapping_div(1000 as uint32_t)
        as i32;
}
pub unsafe fn I_GetTimeMS() -> i32 {
    let mut ticks: uint32_t = 0;
    ticks = I_GetTicks() as uint32_t;
    if basetime == 0 as uint32_t {
        basetime = ticks;
    }
    return ticks.wrapping_sub(basetime) as i32;
}
pub unsafe fn I_Sleep(mut ms: i32) {
    DG_SleepMs(ms as uint32_t);
}
