pub type boolean = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const DIR_SEPARATOR: char = '/';
pub static mut myargv: Vec<::std::ffi::CString> = Vec::new();
pub unsafe fn M_CheckParmWithArgs(
    check: &str,
    mut num_args: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < myargv.len() as ::core::ffi::c_int - num_args {
        if myargv[i as usize]
            .to_str()
            .map_or(false, |arg| arg.eq_ignore_ascii_case(check))
        {
            return i;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe fn M_ParmExists(check: &str) -> boolean {
    return (M_CheckParm(check) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as boolean;
}
pub unsafe fn M_CheckParm(check: &str) -> ::core::ffi::c_int {
    return M_CheckParmWithArgs(check, 0 as ::core::ffi::c_int);
}
unsafe fn LoadResponseFile(mut argv_index: ::core::ffi::c_int) {}
pub unsafe fn M_FindResponseFile() {
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < myargv.len() as ::core::ffi::c_int {
        if myargv[i as usize].as_bytes().first() == Some(&b'@') {
            LoadResponseFile(i);
        }
        i += 1;
    }
}
pub unsafe fn M_GetExecutableName() -> &'static str {
    let arg0 = myargv[0].to_str().unwrap();
    match arg0.rfind(DIR_SEPARATOR) {
        Some(pos) => &arg0[pos + 1..],
        None => arg0,
    }
}
