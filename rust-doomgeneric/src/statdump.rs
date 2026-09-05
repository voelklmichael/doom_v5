use crate::src::wi_stuff::{wbplayerstruct_t, wbstartstruct_t};
use crate::src::m_argv::M_ParmExists;
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type boolean = ::core::ffi::c_uint;
pub const MAX_CAPTURES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut captured_stats: [wbstartstruct_t; 32] = [wbstartstruct_t {
    epsd: 0,
    didsecret: false,
    last: 0,
    next: 0,
    maxkills: 0,
    maxitems: 0,
    maxsecret: 0,
    maxfrags: 0,
    partime: 0,
    pnum: 0,
    plyr: [wbplayerstruct_t {
        in_0: false,
        skills: 0,
        sitems: 0,
        ssecret: 0,
        stime: 0,
        frags: [0; 4],
        score: 0,
    }; 4],
}; 32];
static mut num_captured_stats: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn StatCopy(mut stats: *mut wbstartstruct_t) {
    if M_ParmExists("-statdump") && num_captured_stats < MAX_CAPTURES {
        memcpy(
            (&raw mut captured_stats as *mut wbstartstruct_t)
                .offset(num_captured_stats as isize) as *mut wbstartstruct_t
                as *mut ::core::ffi::c_void,
            stats as *const ::core::ffi::c_void,
            ::core::mem::size_of::<wbstartstruct_t>() as size_t,
        );
        num_captured_stats += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn StatDump() {}
