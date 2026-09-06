pub const DIR_SEPARATOR: char = '/';
pub static mut myargv: Vec<::std::ffi::CString> = Vec::new();
pub unsafe fn M_CheckParmWithArgs(
    check: &str,
    mut num_args: i32,
) -> i32 {
    let mut i: i32 = 1 as i32;
    while i < myargv.len() as i32 - num_args {
        if myargv[i as usize]
            .to_str()
            .map_or(false, |arg| arg.eq_ignore_ascii_case(check))
        {
            return i;
        }
        i += 1;
    }
    return 0 as i32;
}
pub unsafe fn M_ParmExists(check: &str) -> bool {
    return M_CheckParm(check) != 0 as i32;
}
pub unsafe fn M_CheckParm(check: &str) -> i32 {
    return M_CheckParmWithArgs(check, 0 as i32);
}
pub unsafe fn M_FindResponseFile() {
    let mut i: i32 = 1 as i32;
    while i < myargv.len() as i32 {
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
