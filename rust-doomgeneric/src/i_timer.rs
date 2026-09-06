extern "C" {
    fn DG_SleepMs(ms: uint32_t);
    fn DG_GetTicksMs() -> uint32_t;
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub const TICRATE: i32 = 35 as i32;
static mut basetime: uint32_t = 0 as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn I_GetTicks() -> i32 {
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
pub unsafe fn I_WaitVBL(mut count: i32) {}
pub unsafe fn I_InitTimer() {}
