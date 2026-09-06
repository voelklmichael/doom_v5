use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::PU_STATIC;
extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MEMFILE {
    pub buf: *mut u8,
    pub buflen: size_t,
    pub alloced: size_t,
    pub position: u32,
    pub mode: memfile_mode_t,
}
pub type memfile_mode_t = u32;
pub const MODE_WRITE: memfile_mode_t = 1;
pub const MODE_READ: memfile_mode_t = 0;
pub type MEMFILE = _MEMFILE;
pub type mem_rel_t = u32;
pub const MEM_SEEK_END: mem_rel_t = 2;
pub const MEM_SEEK_CUR: mem_rel_t = 1;
pub const MEM_SEEK_SET: mem_rel_t = 0;
#[no_mangle]
pub unsafe extern "C" fn mem_fopen_read(
    mut buf: *mut ::core::ffi::c_void,
    mut buflen: size_t,
) -> *mut MEMFILE {
    let mut file: *mut MEMFILE = ::core::ptr::null_mut::<MEMFILE>();
    file = Z_Malloc(
        ::core::mem::size_of::<MEMFILE>() as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut MEMFILE;
    (*file).buf = buf as *mut u8;
    (*file).buflen = buflen;
    (*file).position = 0 as u32;
    (*file).mode = MODE_READ;
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn mem_fread(
    mut buf: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut MEMFILE,
) -> size_t {
    let mut items: size_t = 0;
    if (*stream).mode as u32
        != MODE_READ as i32 as u32
    {
        printf(b"not a read stream\n\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as i32) as size_t;
    }
    items = nmemb;
    if items.wrapping_mul(size)
        > (*stream).buflen.wrapping_sub((*stream).position as size_t)
    {
        items = (*stream)
            .buflen
            .wrapping_sub((*stream).position as size_t)
            .wrapping_div(size);
    }
    memcpy(
        buf,
        (*stream).buf.offset((*stream).position as isize) as *const ::core::ffi::c_void,
        items.wrapping_mul(size),
    );
    (*stream).position = ((*stream).position as size_t)
        .wrapping_add(items.wrapping_mul(size)) as u32
        as u32;
    return items;
}
#[no_mangle]
pub unsafe extern "C" fn mem_fopen_write() -> *mut MEMFILE {
    let mut file: *mut MEMFILE = ::core::ptr::null_mut::<MEMFILE>();
    file = Z_Malloc(
        ::core::mem::size_of::<MEMFILE>() as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut MEMFILE;
    (*file).alloced = 1024 as size_t;
    (*file).buf = Z_Malloc(
        (*file).alloced as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut u8;
    (*file).buflen = 0 as size_t;
    (*file).position = 0 as u32;
    (*file).mode = MODE_WRITE;
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn mem_fwrite(
    mut ptr: *const ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut MEMFILE,
) -> size_t {
    let mut bytes: size_t = 0;
    if (*stream).mode as u32
        != MODE_WRITE as i32 as u32
    {
        return -(1 as i32) as size_t;
    }
    bytes = size.wrapping_mul(nmemb);
    while bytes > (*stream).alloced.wrapping_sub((*stream).position as size_t) {
        let mut newbuf: *mut u8 = ::core::ptr::null_mut::<
            u8,
        >();
        newbuf = Z_Malloc(
            (*stream).alloced.wrapping_mul(2 as size_t) as i32,
            PU_STATIC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut u8;
        memcpy(
            newbuf as *mut ::core::ffi::c_void,
            (*stream).buf as *const ::core::ffi::c_void,
            (*stream).alloced,
        );
        Z_Free((*stream).buf as *mut ::core::ffi::c_void);
        (*stream).buf = newbuf;
        (*stream).alloced = (*stream).alloced.wrapping_mul(2 as size_t);
    }
    memcpy(
        (*stream).buf.offset((*stream).position as isize) as *mut ::core::ffi::c_void,
        ptr,
        bytes,
    );
    (*stream).position = ((*stream).position as size_t).wrapping_add(bytes)
        as u32 as u32;
    if (*stream).position as size_t > (*stream).buflen {
        (*stream).buflen = (*stream).position as size_t;
    }
    return nmemb;
}
#[no_mangle]
pub unsafe extern "C" fn mem_get_buf(
    mut stream: *mut MEMFILE,
    mut buf: *mut *mut ::core::ffi::c_void,
    mut buflen: *mut size_t,
) {
    *buf = (*stream).buf as *mut ::core::ffi::c_void;
    *buflen = (*stream).buflen;
}
#[no_mangle]
pub unsafe extern "C" fn mem_fclose(mut stream: *mut MEMFILE) {
    if (*stream).mode as u32
        == MODE_WRITE as i32 as u32
    {
        Z_Free((*stream).buf as *mut ::core::ffi::c_void);
    }
    Z_Free(stream as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mem_ftell(mut stream: *mut MEMFILE) -> i64 {
    return (*stream).position as i64;
}
#[no_mangle]
pub unsafe extern "C" fn mem_fseek(
    mut stream: *mut MEMFILE,
    mut position: i64,
    mut whence: mem_rel_t,
) -> i32 {
    let mut newpos: u32 = 0;
    match whence as u32 {
        0 => {
            newpos = position as i32 as u32;
        }
        1 => {
            newpos = ((*stream).position as i64 + position)
                as i32 as u32;
        }
        2 => {
            newpos = (*stream).buflen.wrapping_add(position as size_t)
                as i32 as u32;
        }
        _ => return -(1 as i32),
    }
    if (newpos as size_t) < (*stream).buflen {
        (*stream).position = newpos;
        return 0 as i32;
    } else {
        printf(
            b"Error seeking to %i\n\0" as *const u8 as *const ::core::ffi::c_char,
            newpos,
        );
        return -(1 as i32);
    };
}
