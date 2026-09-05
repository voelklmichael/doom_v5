pub type boolean = ::core::ffi::c_uint;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut net_client_connected: boolean = false_0 as boolean;
#[no_mangle]
pub static mut drone: boolean = false_0 as boolean;
#[no_mangle]
pub unsafe extern "C" fn I_InitTimidityConfig() {}
