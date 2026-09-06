use crate::src::i_system::FILE;
use crate::src::i_system::I_Error;
use crate::src::i_system::I_ZoneBase;

extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> i32;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> i32;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type C2RustUnnamed = u32;
pub const PU_NUM_TAGS: C2RustUnnamed = 9;
pub const PU_CACHE: C2RustUnnamed = 8;
pub const PU_PURGELEVEL: C2RustUnnamed = 7;
pub const PU_LEVSPEC: C2RustUnnamed = 6;
pub const PU_LEVEL: C2RustUnnamed = 5;
pub const PU_FREE: C2RustUnnamed = 4;
pub const PU_MUSIC: C2RustUnnamed = 3;
pub const PU_SOUND: C2RustUnnamed = 2;
pub const PU_STATIC: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memzone_t {
    pub size: i32,
    pub blocklist: memblock_t,
    pub rover: *mut memblock_t,
}
pub type memblock_t = memblock_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memblock_s {
    pub size: i32,
    pub user: *mut *mut ::core::ffi::c_void,
    pub tag: i32,
    pub id: i32,
    pub next: *mut memblock_s,
    pub prev: *mut memblock_s,
}
pub type byte = uint8_t;
pub type uint8_t = __uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MEM_ALIGN: usize = ::core::mem::size_of::<*mut ::core::ffi::c_void>();
pub const ZONEID: i32 = 0x1d4a11 as i32;
#[no_mangle]
pub static mut mainzone: *mut memzone_t = ::core::ptr::null::<memzone_t>()
    as *mut memzone_t;
#[no_mangle]
pub unsafe extern "C" fn Z_ClearZone(mut zone: *mut memzone_t) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    block = (zone as *mut byte)
        .offset(::core::mem::size_of::<memzone_t>() as usize as isize)
        as *mut memblock_t;
    (*zone).blocklist.prev = block as *mut memblock_s;
    (*zone).blocklist.next = (*zone).blocklist.prev;
    (*zone).blocklist.user = zone as *mut ::core::ffi::c_void
        as *mut *mut ::core::ffi::c_void;
    (*zone).blocklist.tag = PU_STATIC as i32;
    (*zone).rover = block;
    (*block).next = &raw mut (*zone).blocklist as *mut memblock_s;
    (*block).prev = (*block).next;
    (*block).tag = PU_FREE as i32;
    (*block).size = ((*zone).size as usize)
        .wrapping_sub(::core::mem::size_of::<memzone_t>() as usize)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn Z_Init() {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut size: i32 = 0;
    mainzone = I_ZoneBase(&raw mut size) as *mut memzone_t;
    (*mainzone).size = size;
    block = (mainzone as *mut byte)
        .offset(::core::mem::size_of::<memzone_t>() as usize as isize)
        as *mut memblock_t;
    (*mainzone).blocklist.prev = block as *mut memblock_s;
    (*mainzone).blocklist.next = (*mainzone).blocklist.prev;
    (*mainzone).blocklist.user = mainzone as *mut ::core::ffi::c_void
        as *mut *mut ::core::ffi::c_void;
    (*mainzone).blocklist.tag = PU_STATIC as i32;
    (*mainzone).rover = block;
    (*block).next = &raw mut (*mainzone).blocklist as *mut memblock_s;
    (*block).prev = (*block).next;
    (*block).tag = PU_FREE as i32;
    (*block).size = ((*mainzone).size as usize)
        .wrapping_sub(::core::mem::size_of::<memzone_t>() as usize)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn Z_Free(mut ptr: *mut ::core::ffi::c_void) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut other: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    block = (ptr as *mut byte)
        .offset(-(::core::mem::size_of::<memblock_t>() as usize as isize))
        as *mut memblock_t;
    if (*block).id != ZONEID {
        I_Error("Z_Free: freed a pointer without ZONEID");
    }
    if (*block).tag != PU_FREE as i32 && !(*block).user.is_null() {
        *(*block).user = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*block).tag = PU_FREE as i32;
    (*block).user = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    (*block).id = 0 as i32;
    other = (*block).prev as *mut memblock_t;
    if (*other).tag == PU_FREE as i32 {
        (*other).size += (*block).size;
        (*other).next = (*block).next;
        (*(*other).next).prev = other as *mut memblock_s;
        if block == (*mainzone).rover {
            (*mainzone).rover = other;
        }
        block = other;
    }
    other = (*block).next as *mut memblock_t;
    if (*other).tag == PU_FREE as i32 {
        (*block).size += (*other).size;
        (*block).next = (*other).next;
        (*(*block).next).prev = block as *mut memblock_s;
        if other == (*mainzone).rover {
            (*mainzone).rover = block;
        }
    }
}
pub const MINFRAGMENT: i32 = 64 as i32;
#[no_mangle]
pub unsafe extern "C" fn Z_Malloc(
    mut size: i32,
    mut tag: i32,
    mut user: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut extra: i32 = 0;
    let mut start: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut rover: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut newblock: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut base: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    size = ((size as usize).wrapping_add(MEM_ALIGN).wrapping_sub(1 as usize)
        & !MEM_ALIGN.wrapping_sub(1 as usize)) as i32;
    size = (size as u64)
        .wrapping_add(
            ::core::mem::size_of::<memblock_t>() as usize as u64,
        ) as i32 as i32;
    base = (*mainzone).rover;
    if (*(*base).prev).tag == PU_FREE as i32 {
        base = (*base).prev as *mut memblock_t;
    }
    rover = base;
    start = (*base).prev as *mut memblock_t;
    loop {
        if rover == start {
            I_Error(&format!("Z_Malloc: failed on allocation of {} bytes", size));
        }
        if (*rover).tag != PU_FREE as i32 {
            if (*rover).tag < PU_PURGELEVEL as i32 {
                rover = (*rover).next as *mut memblock_t;
                base = rover;
            } else {
                base = (*base).prev as *mut memblock_t;
                Z_Free(
                    (rover as *mut byte)
                        .offset(::core::mem::size_of::<memblock_t>() as usize as isize)
                        as *mut ::core::ffi::c_void,
                );
                base = (*base).next as *mut memblock_t;
                rover = (*base).next as *mut memblock_t;
            }
        } else {
            rover = (*rover).next as *mut memblock_t;
        }
        if !((*base).tag != PU_FREE as i32 || (*base).size < size) {
            break;
        }
    }
    extra = (*base).size - size;
    if extra > MINFRAGMENT {
        newblock = (base as *mut byte).offset(size as isize) as *mut memblock_t;
        (*newblock).size = extra;
        (*newblock).tag = PU_FREE as i32;
        (*newblock).user = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
        (*newblock).prev = base as *mut memblock_s;
        (*newblock).next = (*base).next;
        (*(*newblock).next).prev = newblock as *mut memblock_s;
        (*base).next = newblock as *mut memblock_s;
        (*base).size = size;
    }
    if user.is_null() && tag >= PU_PURGELEVEL as i32 {
        I_Error("Z_Malloc: an owner is required for purgable blocks");
    }
    (*base).user = user as *mut *mut ::core::ffi::c_void;
    (*base).tag = tag;
    result = (base as *mut byte)
        .offset(::core::mem::size_of::<memblock_t>() as usize as isize)
        as *mut ::core::ffi::c_void;
    if !(*base).user.is_null() {
        *(*base).user = result;
    }
    (*mainzone).rover = (*base).next as *mut memblock_t;
    (*base).id = ZONEID;
    return result;
}
pub unsafe fn Z_FreeTags(
    mut lowtag: i32,
    mut hightag: i32,
) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut next: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    block = (*mainzone).blocklist.next as *mut memblock_t;
    while block != &raw mut (*mainzone).blocklist {
        next = (*block).next as *mut memblock_t;
        if !((*block).tag == PU_FREE as i32) {
            if (*block).tag >= lowtag && (*block).tag <= hightag {
                Z_Free(
                    (block as *mut byte)
                        .offset(::core::mem::size_of::<memblock_t>() as usize as isize)
                        as *mut ::core::ffi::c_void,
                );
            }
        }
        block = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Z_DumpHeap(
    mut lowtag: i32,
    mut hightag: i32,
) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    printf(
        b"zone size: %i  location: %p\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*mainzone).size,
        mainzone,
    );
    printf(
        b"tag range: %i to %i\n\0" as *const u8 as *const ::core::ffi::c_char,
        lowtag,
        hightag,
    );
    block = (*mainzone).blocklist.next as *mut memblock_t;
    loop {
        if (*block).tag >= lowtag && (*block).tag <= hightag {
            printf(
                b"block:%p    size:%7i    user:%p    tag:%3i\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                block,
                (*block).size,
                (*block).user,
                (*block).tag,
            );
        }
        if (*block).next == &raw mut (*mainzone).blocklist {
            break;
        }
        if (block as *mut byte).offset((*block).size as isize)
            != (*block).next as *mut byte
        {
            printf(
                b"ERROR: block size does not touch the next block\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*(*block).next).prev != block {
            printf(
                b"ERROR: next block doesn't have proper back link\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*block).tag == PU_FREE as i32
            && (*(*block).next).tag == PU_FREE as i32
        {
            printf(
                b"ERROR: two consecutive free blocks\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        block = (*block).next as *mut memblock_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Z_FileDumpHeap(mut f: *mut FILE) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    fprintf(
        f,
        b"zone size: %i  location: %p\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*mainzone).size,
        mainzone,
    );
    block = (*mainzone).blocklist.next as *mut memblock_t;
    loop {
        fprintf(
            f,
            b"block:%p    size:%7i    user:%p    tag:%3i\n\0" as *const u8
                as *const ::core::ffi::c_char,
            block,
            (*block).size,
            (*block).user,
            (*block).tag,
        );
        if (*block).next == &raw mut (*mainzone).blocklist {
            break;
        }
        if (block as *mut byte).offset((*block).size as isize)
            != (*block).next as *mut byte
        {
            fprintf(
                f,
                b"ERROR: block size does not touch the next block\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*(*block).next).prev != block {
            fprintf(
                f,
                b"ERROR: next block doesn't have proper back link\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*block).tag == PU_FREE as i32
            && (*(*block).next).tag == PU_FREE as i32
        {
            fprintf(
                f,
                b"ERROR: two consecutive free blocks\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        block = (*block).next as *mut memblock_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Z_CheckHeap() {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    block = (*mainzone).blocklist.next as *mut memblock_t;
    while !((*block).next == &raw mut (*mainzone).blocklist) {
        if (block as *mut byte).offset((*block).size as isize)
            != (*block).next as *mut byte
        {
            I_Error("Z_CheckHeap: block size does not touch the next block\n");
        }
        if (*(*block).next).prev != block {
            I_Error("Z_CheckHeap: next block doesn't have proper back link\n");
        }
        if (*block).tag == PU_FREE as i32
            && (*(*block).next).tag == PU_FREE as i32
        {
            I_Error("Z_CheckHeap: two consecutive free blocks\n");
        }
        block = (*block).next as *mut memblock_t;
    }
}
pub unsafe fn Z_ChangeTag2(
    mut ptr: *mut ::core::ffi::c_void,
    mut tag: i32,
    mut file: *mut ::core::ffi::c_char,
    mut line: i32,
) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    block = (ptr as *mut byte)
        .offset(-(::core::mem::size_of::<memblock_t>() as usize as isize))
        as *mut memblock_t;
    if (*block).id != ZONEID {
        I_Error(&format!(
            "{}:{}: Z_ChangeTag: block without a ZONEID!",
            ::std::ffi::CStr::from_ptr(file).to_str().unwrap(),
            line,
        ));
    }
    if tag >= PU_PURGELEVEL as i32 && (*block).user.is_null() {
        I_Error(&format!(
            "{}:{}: Z_ChangeTag: an owner is required for purgable blocks",
            ::std::ffi::CStr::from_ptr(file).to_str().unwrap(),
            line,
        ));
    }
    (*block).tag = tag;
}
#[no_mangle]
pub unsafe extern "C" fn Z_ChangeUser(
    mut ptr: *mut ::core::ffi::c_void,
    mut user: *mut *mut ::core::ffi::c_void,
) {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    block = (ptr as *mut byte)
        .offset(-(::core::mem::size_of::<memblock_t>() as usize as isize))
        as *mut memblock_t;
    if (*block).id != ZONEID {
        I_Error("Z_ChangeUser: Tried to change user for invalid block!");
    }
    (*block).user = user;
    *user = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn Z_FreeMemory() -> i32 {
    let mut block: *mut memblock_t = ::core::ptr::null_mut::<memblock_t>();
    let mut free: i32 = 0;
    free = 0 as i32;
    block = (*mainzone).blocklist.next as *mut memblock_t;
    while block != &raw mut (*mainzone).blocklist {
        if (*block).tag == PU_FREE as i32
            || (*block).tag >= PU_PURGELEVEL as i32
        {
            free += (*block).size;
        }
        block = (*block).next as *mut memblock_t;
    }
    return free;
}
#[no_mangle]
pub unsafe extern "C" fn Z_ZoneSize() -> u32 {
    return (*mainzone).size as u32;
}
