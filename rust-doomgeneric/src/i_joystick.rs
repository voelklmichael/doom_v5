use crate::src::m_config::M_BindVariable;
use crate::src::m_misc::M_snprintf;

pub type size_t = usize;
pub const NUM_VIRTUAL_BUTTONS: i32 = 10 as i32;
static mut usejoystick: i32 = 0 as i32;
static mut joystick_index: i32 = -(1 as i32);
static mut joystick_x_axis: i32 = 0 as i32;
static mut joystick_x_invert: i32 = 0 as i32;
static mut joystick_y_axis: i32 = 1 as i32;
static mut joystick_y_invert: i32 = 0 as i32;
static mut joystick_strafe_axis: i32 = -(1 as i32);
static mut joystick_strafe_invert: i32 = 0 as i32;
static mut joystick_physical_buttons: [i32; 10] = [
    0 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    4 as i32,
    5 as i32,
    6 as i32,
    7 as i32,
    8 as i32,
    9 as i32,
];
#[no_mangle]
pub unsafe extern "C" fn I_ShutdownJoystick() {}
pub unsafe fn I_InitJoystick() {}
#[no_mangle]
pub unsafe extern "C" fn I_UpdateJoystick() {}
pub unsafe fn I_BindJoystickVariables() {
    let mut i: i32 = 0;
    M_BindVariable("use_joystick",
        &raw mut usejoystick as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_index",
        &raw mut joystick_index as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_x_axis",
        &raw mut joystick_x_axis as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_y_axis",
        &raw mut joystick_y_axis as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_strafe_axis",
        &raw mut joystick_strafe_axis as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_x_invert",
        &raw mut joystick_x_invert as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_y_invert",
        &raw mut joystick_y_invert as *mut ::core::ffi::c_void,
    );
    M_BindVariable("joystick_strafe_invert",
        &raw mut joystick_strafe_invert as *mut ::core::ffi::c_void,
    );
    i = 0 as i32;
    while i < NUM_VIRTUAL_BUTTONS {
        let mut name: [::core::ffi::c_char; 32] = [0; 32];
        M_snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
            b"joystick_physical_button%i\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        M_BindVariable(
            ::std::ffi::CStr::from_ptr(&raw mut name as *mut ::core::ffi::c_char)
                .to_str()
                .unwrap(),
            (&raw mut joystick_physical_buttons as *mut i32)
                .offset(i as isize) as *mut i32
                as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
}
