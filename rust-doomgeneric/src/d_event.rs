pub type evtype_t = u32;
pub const ev_quit: evtype_t = 4;
pub const ev_joystick: evtype_t = 3;
pub const ev_mouse: evtype_t = 2;
pub const ev_keyup: evtype_t = 1;
pub const ev_keydown: evtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_t {
    pub type_0: evtype_t,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub data4: i32,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXEVENTS: i32 = 64 as i32;
static mut events: [event_t; 64] = [event_t {
    type_0: ev_keydown,
    data1: 0,
    data2: 0,
    data3: 0,
    data4: 0,
}; 64];
static mut eventhead: i32 = 0;
static mut eventtail: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn D_PostEvent(mut ev: *mut event_t) {
    events[eventhead as usize] = *ev;
    eventhead = (eventhead + 1 as i32) % MAXEVENTS;
}
#[no_mangle]
pub unsafe extern "C" fn D_PopEvent() -> *mut event_t {
    let mut result: *mut event_t = ::core::ptr::null_mut::<event_t>();
    if eventtail == eventhead {
        return ::core::ptr::null_mut::<event_t>();
    }
    result = (&raw mut events as *mut event_t).offset(eventtail as isize)
        as *mut event_t;
    eventtail = (eventtail + 1 as i32) % MAXEVENTS;
    return result;
}
