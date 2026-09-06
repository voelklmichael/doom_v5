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
const MAXEVENTS: i32 = 64;

pub struct DEventState {
    events: [event_t; MAXEVENTS as usize],
    eventhead: i32,
    eventtail: i32,
}

impl DEventState {
    pub const fn new() -> Self {
        DEventState {
            events: [event_t {
                type_0: ev_keydown,
                data1: 0,
                data2: 0,
                data3: 0,
                data4: 0,
            }; 64],
            eventhead: 0,
            eventtail: 0,
        }
    }
}

pub fn D_PostEvent(state: &mut DEventState, ev: event_t) {
    state.events[state.eventhead as usize] = ev;
    state.eventhead = (state.eventhead + 1 as i32) % MAXEVENTS;
}
pub fn D_PopEvent(state: &mut DEventState) -> Option<event_t> {
    if state.eventtail == state.eventhead {
        return None;
    }
    let event = state.events[state.eventtail as usize].clone();

    state.eventtail = (state.eventtail + 1 as i32) % MAXEVENTS;
    return Some(event);
}
