pub type gameaction_t = u32;
pub const ga_screenshot: gameaction_t = 9;
pub const ga_worlddone: gameaction_t = 8;
pub const ga_victory: gameaction_t = 7;
pub const ga_completed: gameaction_t = 6;
pub const ga_playdemo: gameaction_t = 5;
pub const ga_savegame: gameaction_t = 4;
pub const ga_loadgame: gameaction_t = 3;
pub const ga_newgame: gameaction_t = 2;
pub const ga_loadlevel: gameaction_t = 1;
pub const ga_nothing: gameaction_t = 0;
pub type gamestate_t = u32;
pub const GS_DEMOSCREEN: gamestate_t = 3;
pub const GS_FINALE: gamestate_t = 2;
pub const GS_INTERMISSION: gamestate_t = 1;
pub const GS_LEVEL: gamestate_t = 0;
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
pub const MAXEVENTS: i32 = 64;
static mut events: [event_t; 64] = [event_t {
    type_0: ev_keydown,
    data1: 0,
    data2: 0,
    data3: 0,
    data4: 0,
}; 64];
static mut eventhead: i32 = 0;
static mut eventtail: i32 = 0;
pub unsafe fn D_PostEvent(mut ev: *mut event_t) {
    events[eventhead as usize] = *ev;
    eventhead = (eventhead + 1 as i32) % MAXEVENTS;
}
pub unsafe fn D_PopEvent() -> *mut event_t {
    let mut result: *mut event_t = ::core::ptr::null_mut::<event_t>();
    if eventtail == eventhead {
        return ::core::ptr::null_mut::<event_t>();
    }
    result = (&raw mut events as *mut event_t).offset(eventtail as isize)
        as *mut event_t;
    eventtail = (eventtail + 1 as i32) % MAXEVENTS;
    return result;
}
