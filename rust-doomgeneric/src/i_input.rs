use crate::src::d_event::event_t;
use crate::src::d_event::D_PostEvent;
use crate::src::d_event::{ev_keydown, ev_keyup};
use crate::src::game_state::game_state;
use crate::src::m_controls::KEY_RSHIFT;

extern "C" {
    fn DG_GetKey(pressed: *mut i32, key: *mut u8) -> i32;
}
pub struct IInputState {
    pub vanilla_keyboard_mapping: i32,
    shiftdown: i32,
}

impl IInputState {
    pub const fn new() -> Self {
        IInputState {
            vanilla_keyboard_mapping: 1,
            shiftdown: 0,
        }
    }
}

static shiftxform: [::core::ffi::c_char; 128] = [
    0 as i32 as ::core::ffi::c_char,
    1 as i32 as ::core::ffi::c_char,
    2 as i32 as ::core::ffi::c_char,
    3 as i32 as ::core::ffi::c_char,
    4 as i32 as ::core::ffi::c_char,
    5 as i32 as ::core::ffi::c_char,
    6 as i32 as ::core::ffi::c_char,
    7 as i32 as ::core::ffi::c_char,
    8 as i32 as ::core::ffi::c_char,
    9 as i32 as ::core::ffi::c_char,
    10 as i32 as ::core::ffi::c_char,
    11 as i32 as ::core::ffi::c_char,
    12 as i32 as ::core::ffi::c_char,
    13 as i32 as ::core::ffi::c_char,
    14 as i32 as ::core::ffi::c_char,
    15 as i32 as ::core::ffi::c_char,
    16 as i32 as ::core::ffi::c_char,
    17 as i32 as ::core::ffi::c_char,
    18 as i32 as ::core::ffi::c_char,
    19 as i32 as ::core::ffi::c_char,
    20 as i32 as ::core::ffi::c_char,
    21 as i32 as ::core::ffi::c_char,
    22 as i32 as ::core::ffi::c_char,
    23 as i32 as ::core::ffi::c_char,
    24 as i32 as ::core::ffi::c_char,
    25 as i32 as ::core::ffi::c_char,
    26 as i32 as ::core::ffi::c_char,
    27 as i32 as ::core::ffi::c_char,
    28 as i32 as ::core::ffi::c_char,
    29 as i32 as ::core::ffi::c_char,
    30 as i32 as ::core::ffi::c_char,
    31 as i32 as ::core::ffi::c_char,
    ' ' as i32 as ::core::ffi::c_char,
    '!' as i32 as ::core::ffi::c_char,
    '"' as i32 as ::core::ffi::c_char,
    '#' as i32 as ::core::ffi::c_char,
    '$' as i32 as ::core::ffi::c_char,
    '%' as i32 as ::core::ffi::c_char,
    '&' as i32 as ::core::ffi::c_char,
    '"' as i32 as ::core::ffi::c_char,
    '(' as i32 as ::core::ffi::c_char,
    ')' as i32 as ::core::ffi::c_char,
    '*' as i32 as ::core::ffi::c_char,
    '+' as i32 as ::core::ffi::c_char,
    '<' as i32 as ::core::ffi::c_char,
    '_' as i32 as ::core::ffi::c_char,
    '>' as i32 as ::core::ffi::c_char,
    '?' as i32 as ::core::ffi::c_char,
    ')' as i32 as ::core::ffi::c_char,
    '!' as i32 as ::core::ffi::c_char,
    '@' as i32 as ::core::ffi::c_char,
    '#' as i32 as ::core::ffi::c_char,
    '$' as i32 as ::core::ffi::c_char,
    '%' as i32 as ::core::ffi::c_char,
    '^' as i32 as ::core::ffi::c_char,
    '&' as i32 as ::core::ffi::c_char,
    '*' as i32 as ::core::ffi::c_char,
    '(' as i32 as ::core::ffi::c_char,
    ':' as i32 as ::core::ffi::c_char,
    ':' as i32 as ::core::ffi::c_char,
    '<' as i32 as ::core::ffi::c_char,
    '+' as i32 as ::core::ffi::c_char,
    '>' as i32 as ::core::ffi::c_char,
    '?' as i32 as ::core::ffi::c_char,
    '@' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'J' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'Q' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'Z' as i32 as ::core::ffi::c_char,
    '[' as i32 as ::core::ffi::c_char,
    '!' as i32 as ::core::ffi::c_char,
    ']' as i32 as ::core::ffi::c_char,
    '"' as i32 as ::core::ffi::c_char,
    '_' as i32 as ::core::ffi::c_char,
    '\'' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'J' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'Q' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'Z' as i32 as ::core::ffi::c_char,
    '{' as i32 as ::core::ffi::c_char,
    '|' as i32 as ::core::ffi::c_char,
    '}' as i32 as ::core::ffi::c_char,
    '~' as i32 as ::core::ffi::c_char,
    127 as i32 as ::core::ffi::c_char,
];
unsafe fn TranslateKey(mut key: u8) -> u8 {
    return key;
}
unsafe fn GetTypedChar(state: &mut IInputState, mut key: u8) -> u8 {
    key = TranslateKey(key);
    if state.shiftdown > 0 as i32 {
        if key as i32 >= 0 as i32
            && (key as usize)
                < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        {
            key = shiftxform[key as usize] as u8;
        } else {
            key = 0 as u8;
        }
    }
    return key;
}
unsafe fn UpdateShiftStatus(state: &mut IInputState, mut pressed: i32, mut key: u8) {
    let mut change: i32 = 0;
    if pressed != 0 {
        change = 1 as i32;
    } else {
        change = -(1 as i32);
    }
    if key as i32 == KEY_RSHIFT {
        state.shiftdown += change;
    }
}
pub unsafe fn I_GetEvent(state: &mut IInputState) {
    let mut event: event_t = event_t {
        type_0: ev_keydown,
        data1: 0,
        data2: 0,
        data3: 0,
        data4: 0,
    };
    let mut pressed: i32 = 0;
    let mut key: u8 = 0;
    while DG_GetKey(&raw mut pressed, &raw mut key) != 0 {
        UpdateShiftStatus(state, pressed, key);
        if pressed != 0 {
            event.type_0 = ev_keydown;
            event.data1 = TranslateKey(key) as i32;
            event.data2 = GetTypedChar(state, key) as i32;
            if event.data1 != 0 as i32 {
                D_PostEvent(&mut game_state().d_event, &raw mut event);
            }
        } else {
            event.type_0 = ev_keyup;
            event.data1 = TranslateKey(key) as i32;
            event.data2 = 0 as i32;
            if event.data1 != 0 as i32 {
                D_PostEvent(&mut game_state().d_event, &raw mut event);
            }
            break;
        }
    }
}
