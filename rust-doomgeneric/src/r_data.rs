use crate::src::r_defs::spriteframe_t;
use crate::src::hu_lib::patch_t;
use crate::src::p_mobj::thinker_t;
use crate::src::p_mobj::{mobj_t};
use crate::src::i_system::I_Error;
use crate::src::w_wad::{
    wad_name8_to_string, W_CacheLumpName, W_CheckNumForName, W_GetNumForName,
    W_ReleaseLumpName,
};
use crate::src::i_system::I_ConsoleStdout;
use crate::src::w_wad::W_LumpNameHash;
use crate::src::r_things::numsprites;
use crate::src::p_setup::numsides;
use crate::src::r_things::sprites;
use crate::src::z_zone::Z_ChangeTag2;
use crate::src::r_sky::skytexture;
use crate::src::p_tick::thinkercap;
use crate::src::w_wad::lumpinfo;
use crate::src::g_game::demoplayback;
use crate::src::p_setup::numsectors;
use crate::src::p_setup::sides;
use crate::src::m_misc::M_StringCopy;
use crate::src::p_setup::sectors;
use crate::src::w_wad::W_LumpLength;
use crate::src::w_wad::W_CacheLumpNum;
use crate::src::z_zone::Z_Free;
use crate::src::z_zone::Z_Malloc;
use crate::src::z_zone::{PU_CACHE, PU_STATIC};
use crate::src::p_mobj::ThinkerFn;
use crate::src::m_fixed::fixed_t;
use crate::src::r_defs::lighttable_t;
use crate::src::stdint_types::byte;
use crate::src::stdint_types::size_t;
use libc::{memcpy, memset};
use libc::strncasecmp;
use libc::printf;
use crate::src::doomdef::NULL;
use crate::src::m_fixed::FRACBITS;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct post_t {
    pub topdelta: byte,
    pub length: byte,
}
pub type column_t = post_t;
pub type texture_t = texture_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texture_s {
    pub name: [::core::ffi::c_char; 8],
    pub width: i16,
    pub height: i16,
    pub index: i32,
    pub next: *mut texture_t,
    pub patchcount: i16,
    pub patches: [texpatch_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texpatch_t {
    pub originx: i16,
    pub originy: i16,
    pub patch: i32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mappatch_t {
    pub originx: i16,
    pub originy: i16,
    pub patch: i16,
    pub stepdir: i16,
    pub colormap: i16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct maptexture_t {
    pub name: [::core::ffi::c_char; 8],
    pub masked: i32,
    pub width: i16,
    pub height: i16,
    pub obsolete: i32,
    pub patchcount: i16,
    pub patches: [mappatch_t; 1],
}
pub static mut firstflat: i32 = 0;
#[no_mangle]
pub static mut lastflat: i32 = 0;
pub static mut numflats: i32 = 0;
#[no_mangle]
pub static mut firstpatch: i32 = 0;
#[no_mangle]
pub static mut lastpatch: i32 = 0;
#[no_mangle]
pub static mut numpatches: i32 = 0;
pub static mut firstspritelump: i32 = 0;
pub static mut lastspritelump: i32 = 0;
#[no_mangle]
pub static mut numspritelumps: i32 = 0;
#[no_mangle]
pub static mut numtextures: i32 = 0;
#[no_mangle]
pub static mut textures: *mut *mut texture_t = ::core::ptr::null::<*mut texture_t>()
    as *mut *mut texture_t;
#[no_mangle]
pub static mut textures_hashtable: *mut *mut texture_t = ::core::ptr::null::<
    *mut texture_t,
>() as *mut *mut texture_t;
#[no_mangle]
pub static mut texturewidthmask: *mut i32 = ::core::ptr::null::<
    i32,
>() as *mut i32;
pub static mut textureheight: *mut fixed_t = ::core::ptr::null::<fixed_t>()
    as *mut fixed_t;
#[no_mangle]
pub static mut texturecompositesize: *mut i32 = ::core::ptr::null::<
    i32,
>() as *mut i32;
#[no_mangle]
pub static mut texturecolumnlump: *mut *mut i16 = ::core::ptr::null::<
    *mut i16,
>() as *mut *mut i16;
#[no_mangle]
pub static mut texturecolumnofs: *mut *mut u16 = ::core::ptr::null::<
    *mut u16,
>() as *mut *mut u16;
#[no_mangle]
pub static mut texturecomposite: *mut *mut byte = ::core::ptr::null::<*mut byte>()
    as *mut *mut byte;
pub static mut flattranslation: *mut i32 = ::core::ptr::null::<
    i32,
>() as *mut i32;
pub static mut texturetranslation: *mut i32 = ::core::ptr::null::<
    i32,
>() as *mut i32;
pub static mut spritewidth: *mut fixed_t = ::core::ptr::null::<fixed_t>()
    as *mut fixed_t;
pub static mut spriteoffset: *mut fixed_t = ::core::ptr::null::<fixed_t>()
    as *mut fixed_t;
pub static mut spritetopoffset: *mut fixed_t = ::core::ptr::null::<fixed_t>()
    as *mut fixed_t;
pub static mut colormaps: *mut lighttable_t = ::core::ptr::null::<lighttable_t>()
    as *mut lighttable_t;
#[no_mangle]
pub unsafe extern "C" fn R_DrawColumnInCache(
    mut patch: *mut column_t,
    mut cache: *mut byte,
    mut originy: i32,
    mut cacheheight: i32,
) {
    let mut count: i32 = 0;
    let mut position: i32 = 0;
    let mut source: *mut byte = ::core::ptr::null_mut::<byte>();
    while (*patch).topdelta as i32 != 0xff as i32 {
        source = (patch as *mut byte).offset(3 as i32 as isize);
        count = (*patch).length as i32;
        position = originy + (*patch).topdelta as i32;
        if position < 0 as i32 {
            count += position;
            position = 0 as i32;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        if count > 0 as i32 {
            memcpy(
                cache.offset(position as isize) as *mut ::core::ffi::c_void,
                source as *const ::core::ffi::c_void,
                count as size_t,
            );
        }
        patch = (patch as *mut byte)
            .offset((*patch).length as i32 as isize)
            .offset(4 as i32 as isize) as *mut column_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateComposite(mut texnum: i32) {
    let mut block: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut texture: *mut texture_t = ::core::ptr::null_mut::<texture_t>();
    let mut patch: *mut texpatch_t = ::core::ptr::null_mut::<texpatch_t>();
    let mut realpatch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    let mut x: i32 = 0;
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut i: i32 = 0;
    let mut patchcol: *mut column_t = ::core::ptr::null_mut::<column_t>();
    let mut collump: *mut i16 = ::core::ptr::null_mut::<
        i16,
    >();
    let mut colofs: *mut u16 = ::core::ptr::null_mut::<
        u16,
    >();
    texture = *textures.offset(texnum as isize);
    block = Z_Malloc(
        *texturecompositesize.offset(texnum as isize),
        PU_STATIC as i32,
        texturecomposite.offset(texnum as isize) as *mut *mut byte
            as *mut ::core::ffi::c_void,
    ) as *mut byte;
    collump = *texturecolumnlump.offset(texnum as isize);
    colofs = *texturecolumnofs.offset(texnum as isize);
    patch = &raw mut (*texture).patches as *mut texpatch_t;
    i = 0 as i32;
    patch = &raw mut (*texture).patches as *mut texpatch_t;
    while i < (*texture).patchcount as i32 {
        realpatch = W_CacheLumpNum((*patch).patch, PU_CACHE as i32)
            as *mut patch_t;
        x1 = (*patch).originx as i32;
        x2 = x1 + (*realpatch).width as i32;
        if x1 < 0 as i32 {
            x = 0 as i32;
        } else {
            x = x1;
        }
        if x2 > (*texture).width as i32 {
            x2 = (*texture).width as i32;
        }
        while x < x2 {
            if !(*collump.offset(x as isize) as i32
                >= 0 as i32)
            {
                patchcol = (realpatch as *mut byte)
                    .offset(
                        *(&raw const (*realpatch).columnofs as *const i32)
                            .offset((x - x1) as isize) as isize,
                    ) as *mut column_t;
                R_DrawColumnInCache(
                    patchcol,
                    block
                        .offset(
                            *colofs.offset(x as isize) as i32 as isize,
                        ),
                    (*patch).originy as i32,
                    (*texture).height as i32,
                );
            }
            x += 1;
        }
        i += 1;
        patch = patch.offset(1);
    }
    Z_ChangeTag2(
        block as *mut ::core::ffi::c_void,
        PU_CACHE as i32,
        b"r_data.c\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        286 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateLookup(mut texnum: i32) {
    let mut texture: *mut texture_t = ::core::ptr::null_mut::<texture_t>();
    let mut patchcount: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut patch: *mut texpatch_t = ::core::ptr::null_mut::<texpatch_t>();
    let mut realpatch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    let mut x: i32 = 0;
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut i: i32 = 0;
    let mut collump: *mut i16 = ::core::ptr::null_mut::<
        i16,
    >();
    let mut colofs: *mut u16 = ::core::ptr::null_mut::<
        u16,
    >();
    texture = *textures.offset(texnum as isize);
    let ref mut fresh4 = *texturecomposite.offset(texnum as isize);
    *fresh4 = ::core::ptr::null_mut::<byte>();
    *texturecompositesize.offset(texnum as isize) = 0 as i32;
    collump = *texturecolumnlump.offset(texnum as isize);
    colofs = *texturecolumnofs.offset(texnum as isize);
    patchcount = Z_Malloc(
        (*texture).width as i32,
        PU_STATIC as i32,
        &raw mut patchcount as *mut ::core::ffi::c_void,
    ) as *mut byte;
    memset(
        patchcount as *mut ::core::ffi::c_void,
        0 as i32,
        (*texture).width as size_t,
    );
    patch = &raw mut (*texture).patches as *mut texpatch_t;
    i = 0 as i32;
    patch = &raw mut (*texture).patches as *mut texpatch_t;
    while i < (*texture).patchcount as i32 {
        realpatch = W_CacheLumpNum((*patch).patch, PU_CACHE as i32)
            as *mut patch_t;
        x1 = (*patch).originx as i32;
        x2 = x1 + (*realpatch).width as i32;
        if x1 < 0 as i32 {
            x = 0 as i32;
        } else {
            x = x1;
        }
        if x2 > (*texture).width as i32 {
            x2 = (*texture).width as i32;
        }
        while x < x2 {
            let ref mut fresh5 = *patchcount.offset(x as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            *collump.offset(x as isize) = (*patch).patch as i16;
            *colofs.offset(x as isize) = (*(&raw const (*realpatch).columnofs
                as *const i32)
                .offset((x - x1) as isize)
                + 3 as i32) as u16;
            x += 1;
        }
        i += 1;
        patch = patch.offset(1);
    }
    x = 0 as i32;
    while x < (*texture).width as i32 {
        if *patchcount.offset(x as isize) == 0 {
            printf(
                b"R_GenerateLookup: column without a patch (%s)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut (*texture).name as *mut ::core::ffi::c_char,
            );
            return;
        }
        if *patchcount.offset(x as isize) as i32 > 1 as i32
        {
            *collump.offset(x as isize) = -(1 as i32)
                as i16;
            *colofs.offset(x as isize) = *texturecompositesize.offset(texnum as isize)
                as u16;
            if *texturecompositesize.offset(texnum as isize)
                > 0x10000 as i32 - (*texture).height as i32
            {
                I_Error(&format!("R_GenerateLookup: texture {} is >64k", texnum));
            }
            *texturecompositesize.offset(texnum as isize)
                += (*texture).height as i32;
        }
        x += 1;
    }
    Z_Free(patchcount as *mut ::core::ffi::c_void);
}
pub unsafe fn R_GetColumn(
    mut tex: i32,
    mut col: i32,
) -> *mut byte {
    let mut lump: i32 = 0;
    let mut ofs: i32 = 0;
    col &= *texturewidthmask.offset(tex as isize);
    lump = *(*texturecolumnlump.offset(tex as isize)).offset(col as isize)
        as i32;
    ofs = *(*texturecolumnofs.offset(tex as isize)).offset(col as isize)
        as i32;
    if lump > 0 as i32 {
        return (W_CacheLumpNum(lump, PU_CACHE as i32) as *mut byte)
            .offset(ofs as isize);
    }
    if (*texturecomposite.offset(tex as isize)).is_null() {
        R_GenerateComposite(tex);
    }
    return (*texturecomposite.offset(tex as isize)).offset(ofs as isize);
}
unsafe extern "C" fn GenerateTextureHashTable() {
    let mut rover: *mut *mut texture_t = ::core::ptr::null_mut::<*mut texture_t>();
    let mut i: i32 = 0;
    let mut key: i32 = 0;
    textures_hashtable = Z_Malloc(
        (::core::mem::size_of::<*mut texture_t>() as usize)
            .wrapping_mul(numtextures as usize) as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut texture_t;
    memset(
        textures_hashtable as *mut ::core::ffi::c_void,
        0 as i32,
        (::core::mem::size_of::<*mut texture_t>() as size_t)
            .wrapping_mul(numtextures as size_t),
    );
    i = 0 as i32;
    while i < numtextures {
        (**textures.offset(i as isize)).index = i;
        key = W_LumpNameHash(
                &raw mut (**textures.offset(i as isize)).name as *mut ::core::ffi::c_char,
            )
            .wrapping_rem(numtextures as u32) as i32;
        rover = textures_hashtable.offset(key as isize) as *mut *mut texture_t;
        while !(*rover).is_null() {
            rover = &raw mut (**rover).next;
        }
        let ref mut fresh3 = (**textures.offset(i as isize)).next;
        *fresh3 = ::core::ptr::null_mut::<texture_t>();
        *rover = *textures.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitTextures() {
    let mut mtexture: *mut maptexture_t = ::core::ptr::null_mut::<maptexture_t>();
    let mut texture: *mut texture_t = ::core::ptr::null_mut::<texture_t>();
    let mut mpatch: *mut mappatch_t = ::core::ptr::null_mut::<mappatch_t>();
    let mut patch: *mut texpatch_t = ::core::ptr::null_mut::<texpatch_t>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut maptex: *mut i32 = ::core::ptr::null_mut::<
        i32,
    >();
    let mut maptex2: *mut i32 = ::core::ptr::null_mut::<
        i32,
    >();
    let mut maptex1: *mut i32 = ::core::ptr::null_mut::<
        i32,
    >();
    let mut name: [::core::ffi::c_char; 9] = [0; 9];
    let mut names: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut name_p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut patchlookup: *mut i32 = ::core::ptr::null_mut::<
        i32,
    >();
    let mut totalwidth: i32 = 0;
    let mut nummappatches: i32 = 0;
    let mut offset: i32 = 0;
    let mut maxoff: i32 = 0;
    let mut maxoff2: i32 = 0;
    let mut numtextures1: i32 = 0;
    let mut numtextures2: i32 = 0;
    let mut directory: *mut i32 = ::core::ptr::null_mut::<
        i32,
    >();
    let mut temp1: i32 = 0;
    let mut temp2: i32 = 0;
    let mut temp3: i32 = 0;
    name[8 as i32 as usize] = 0 as ::core::ffi::c_char;
    names = W_CacheLumpName("PNAMES",
        PU_STATIC as i32,
    ) as *mut ::core::ffi::c_char;
    nummappatches = *(names as *mut i32);
    name_p = names.offset(4 as i32 as isize);
    patchlookup = Z_Malloc(
        (nummappatches as usize)
            .wrapping_mul(::core::mem::size_of::<i32>() as usize)
            as i32,
        PU_STATIC as i32,
        NULL,
    ) as *mut i32;
    i = 0 as i32;
    while i < nummappatches {
        M_StringCopy(
            &raw mut name as *mut ::core::ffi::c_char,
            name_p.offset((i * 8 as i32) as isize),
            ::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t,
        );
        *patchlookup.offset(i as isize) = W_CheckNumForName(
            &wad_name8_to_string(&raw const name as *const ::core::ffi::c_char),
        );
        i += 1;
    }
    W_ReleaseLumpName("PNAMES");
    maptex1 = W_CacheLumpName("TEXTURE1",
        PU_STATIC as i32,
    ) as *mut i32;
    maptex = maptex1;
    numtextures1 = *maptex;
    maxoff = W_LumpLength(
        W_GetNumForName("TEXTURE1",
        ) as u32,
    );
    directory = maptex.offset(1 as i32 as isize);
    if W_CheckNumForName("TEXTURE2",
    ) != -(1 as i32)
    {
        maptex2 = W_CacheLumpName("TEXTURE2",
            PU_STATIC as i32,
        ) as *mut i32;
        numtextures2 = *maptex2;
        maxoff2 = W_LumpLength(
            W_GetNumForName("TEXTURE2",
            ) as u32,
        );
    } else {
        maptex2 = ::core::ptr::null_mut::<i32>();
        numtextures2 = 0 as i32;
        maxoff2 = 0 as i32;
    }
    numtextures = numtextures1 + numtextures2;
    textures = Z_Malloc(
        (numtextures as usize)
            .wrapping_mul(::core::mem::size_of::<*mut texture_t>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut texture_t;
    texturecolumnlump = Z_Malloc(
        (numtextures as usize)
            .wrapping_mul(::core::mem::size_of::<*mut i16>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut i16;
    texturecolumnofs = Z_Malloc(
        (numtextures as usize)
            .wrapping_mul(::core::mem::size_of::<*mut u16>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut u16;
    texturecomposite = Z_Malloc(
        (numtextures as usize).wrapping_mul(::core::mem::size_of::<*mut byte>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut *mut byte;
    texturecompositesize = Z_Malloc(
        (numtextures as usize)
            .wrapping_mul(::core::mem::size_of::<i32>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i32;
    texturewidthmask = Z_Malloc(
        (numtextures as usize)
            .wrapping_mul(::core::mem::size_of::<i32>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i32;
    textureheight = Z_Malloc(
        (numtextures as usize).wrapping_mul(::core::mem::size_of::<fixed_t>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut fixed_t;
    totalwidth = 0 as i32;
    temp1 = W_GetNumForName("S_START",
    );
    temp2 = W_GetNumForName("S_END",
    ) - 1 as i32;
    temp3 = (temp2 - temp1 + 63 as i32) / 64 as i32
        + (numtextures + 63 as i32) / 64 as i32;
    if I_ConsoleStdout() {
        printf(b"[\0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as i32;
        while i < temp3 + 9 as i32 {
            printf(b" \0" as *const u8 as *const ::core::ffi::c_char);
            i += 1;
        }
        printf(b"]\0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as i32;
        while i < temp3 + 10 as i32 {
            printf(b"\x08\0" as *const u8 as *const ::core::ffi::c_char);
            i += 1;
        }
    }
    i = 0 as i32;
    while i < numtextures {
        if i & 63 as i32 == 0 {
            printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if i == numtextures1 {
            maptex = maptex2;
            maxoff = maxoff2;
            directory = maptex.offset(1 as i32 as isize);
        }
        offset = *directory;
        if offset > maxoff {
            I_Error("R_InitTextures: bad texture directory");
        }
        mtexture = (maptex as *mut byte).offset(offset as isize) as *mut maptexture_t;
        let ref mut fresh0 = *textures.offset(i as isize);
        *fresh0 = Z_Malloc(
            (::core::mem::size_of::<texture_t>() as usize)
                .wrapping_add(
                    (::core::mem::size_of::<texpatch_t>() as usize)
                        .wrapping_mul(
                            ((*mtexture).patchcount as i32
                                - 1 as i32) as usize,
                        ),
                ) as i32,
            PU_STATIC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut texture_t;
        texture = *fresh0;
        (*texture).width = (*mtexture).width;
        (*texture).height = (*mtexture).height;
        (*texture).patchcount = (*mtexture).patchcount;
        memcpy(
            &raw mut (*texture).name as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            &raw mut (*mtexture).name as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t,
        );
        mpatch = (&raw mut (*mtexture).patches as *mut mappatch_t)
            .offset(0 as i32 as isize) as *mut mappatch_t;
        patch = (&raw mut (*texture).patches as *mut texpatch_t)
            .offset(0 as i32 as isize) as *mut texpatch_t;
        j = 0 as i32;
        while j < (*texture).patchcount as i32 {
            (*patch).originx = (*mpatch).originx;
            (*patch).originy = (*mpatch).originy;
            (*patch).patch = *patchlookup.offset((*mpatch).patch as isize);
            if (*patch).patch == -(1 as i32) {
                I_Error(&format!(
                    "R_InitTextures: Missing patch in texture {}",
                    wad_name8_to_string(&raw mut (*texture).name as *mut ::core::ffi::c_char),
                ));
            }
            j += 1;
            mpatch = mpatch.offset(1);
            patch = patch.offset(1);
        }
        let ref mut fresh1 = *texturecolumnlump.offset(i as isize);
        *fresh1 = Z_Malloc(
            ((*texture).width as usize)
                .wrapping_mul(::core::mem::size_of::<i16>() as usize)
                as i32,
            PU_STATIC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut i16;
        let ref mut fresh2 = *texturecolumnofs.offset(i as isize);
        *fresh2 = Z_Malloc(
            ((*texture).width as usize)
                .wrapping_mul(::core::mem::size_of::<u16>() as usize)
                as i32,
            PU_STATIC as i32,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut u16;
        j = 1 as i32;
        while j * 2 as i32 <= (*texture).width as i32 {
            j <<= 1 as i32;
        }
        *texturewidthmask.offset(i as isize) = j - 1 as i32;
        *textureheight.offset(i as isize) = (((*texture).height as i32)
            << FRACBITS) as fixed_t;
        totalwidth += (*texture).width as i32;
        i += 1;
        directory = directory.offset(1);
    }
    Z_Free(patchlookup as *mut ::core::ffi::c_void);
    W_ReleaseLumpName("TEXTURE1");
    if !maptex2.is_null() {
        W_ReleaseLumpName("TEXTURE2");
    }
    i = 0 as i32;
    while i < numtextures {
        R_GenerateLookup(i);
        i += 1;
    }
    texturetranslation = Z_Malloc(
        ((numtextures + 1 as i32) as usize)
            .wrapping_mul(::core::mem::size_of::<i32>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i32;
    i = 0 as i32;
    while i < numtextures {
        *texturetranslation.offset(i as isize) = i;
        i += 1;
    }
    GenerateTextureHashTable();
}
#[no_mangle]
pub unsafe extern "C" fn R_InitFlats() {
    let mut i: i32 = 0;
    firstflat = W_GetNumForName("F_START",
    ) + 1 as i32;
    lastflat = W_GetNumForName("F_END",
    ) - 1 as i32;
    numflats = lastflat - firstflat + 1 as i32;
    flattranslation = Z_Malloc(
        ((numflats + 1 as i32) as usize)
            .wrapping_mul(::core::mem::size_of::<i32>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut i32;
    i = 0 as i32;
    while i < numflats {
        *flattranslation.offset(i as isize) = i;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitSpriteLumps() {
    let mut i: i32 = 0;
    let mut patch: *mut patch_t = ::core::ptr::null_mut::<patch_t>();
    firstspritelump = W_GetNumForName("S_START",
    ) + 1 as i32;
    lastspritelump = W_GetNumForName("S_END",
    ) - 1 as i32;
    numspritelumps = lastspritelump - firstspritelump + 1 as i32;
    spritewidth = Z_Malloc(
        (numspritelumps as usize)
            .wrapping_mul(::core::mem::size_of::<fixed_t>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut fixed_t;
    spriteoffset = Z_Malloc(
        (numspritelumps as usize)
            .wrapping_mul(::core::mem::size_of::<fixed_t>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut fixed_t;
    spritetopoffset = Z_Malloc(
        (numspritelumps as usize)
            .wrapping_mul(::core::mem::size_of::<fixed_t>() as usize)
            as i32,
        PU_STATIC as i32,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut fixed_t;
    i = 0 as i32;
    while i < numspritelumps {
        if i & 63 as i32 == 0 {
            printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
        }
        patch = W_CacheLumpNum(firstspritelump + i, PU_CACHE as i32)
            as *mut patch_t;
        *spritewidth.offset(i as isize) = (((*patch).width as i32)
            << FRACBITS) as fixed_t;
        *spriteoffset.offset(i as isize) = (((*patch).leftoffset as i32)
            << FRACBITS) as fixed_t;
        *spritetopoffset.offset(i as isize) = (((*patch).topoffset as i32)
            << FRACBITS) as fixed_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitColormaps() {
    let mut lump: i32 = 0;
    lump = W_GetNumForName("COLORMAP",
    );
    colormaps = W_CacheLumpNum(lump, PU_STATIC as i32)
        as *mut lighttable_t;
}
pub unsafe fn R_InitData() {
    R_InitTextures();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitFlats();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitSpriteLumps();
    printf(b".\0" as *const u8 as *const ::core::ffi::c_char);
    R_InitColormaps();
}
pub unsafe fn R_FlatNumForName(
    mut name: *mut ::core::ffi::c_char,
) -> i32 {
    let mut i: i32 = 0;
    let mut namet: [::core::ffi::c_char; 9] = [0; 9];
    i = W_CheckNumForName(&wad_name8_to_string(name));
    if i == -(1 as i32) {
        namet[8 as i32 as usize] = 0 as ::core::ffi::c_char;
        memcpy(
            &raw mut namet as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            8 as size_t,
        );
        I_Error(&format!(
            "R_FlatNumForName: {} not found",
            wad_name8_to_string(&raw mut namet as *mut ::core::ffi::c_char),
        ));
    }
    return i - firstflat;
}
pub unsafe fn R_CheckTextureNumForName(
    mut name: *mut ::core::ffi::c_char,
) -> i32 {
    let mut texture: *mut texture_t = ::core::ptr::null_mut::<texture_t>();
    let mut key: i32 = 0;
    if *name.offset(0 as i32 as isize) as i32 == '-' as i32
    {
        return 0 as i32;
    }
    key = W_LumpNameHash(name).wrapping_rem(numtextures as u32)
        as i32;
    texture = *textures_hashtable.offset(key as isize);
    while !texture.is_null() {
        if strncasecmp(
            &raw mut (*texture).name as *mut ::core::ffi::c_char,
            name,
            8 as size_t,
        ) == 0
        {
            return (*texture).index;
        }
        texture = (*texture).next;
    }
    return -(1 as i32);
}
pub unsafe fn R_TextureNumForName(
    mut name: *mut ::core::ffi::c_char,
) -> i32 {
    let mut i: i32 = 0;
    i = R_CheckTextureNumForName(name);
    if i == -(1 as i32) {
        I_Error(&format!("R_TextureNumForName: {} not found", wad_name8_to_string(name)));
    }
    return i;
}
#[no_mangle]
pub static mut flatmemory: i32 = 0;
#[no_mangle]
pub static mut texturememory: i32 = 0;
#[no_mangle]
pub static mut spritememory: i32 = 0;
pub unsafe fn R_PrecacheLevel() {
    let mut flatpresent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut texturepresent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut spritepresent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut lump: i32 = 0;
    let mut texture: *mut texture_t = ::core::ptr::null_mut::<texture_t>();
    let mut th: *mut thinker_t = ::core::ptr::null_mut::<thinker_t>();
    let mut sf: *mut spriteframe_t = ::core::ptr::null_mut::<spriteframe_t>();
    if demoplayback {
        return;
    }
    flatpresent = Z_Malloc(numflats, PU_STATIC as i32, NULL)
        as *mut ::core::ffi::c_char;
    memset(
        flatpresent as *mut ::core::ffi::c_void,
        0 as i32,
        numflats as size_t,
    );
    i = 0 as i32;
    while i < numsectors {
        *flatpresent.offset((*sectors.offset(i as isize)).floorpic as isize) = 1
            as ::core::ffi::c_char;
        *flatpresent.offset((*sectors.offset(i as isize)).ceilingpic as isize) = 1
            as ::core::ffi::c_char;
        i += 1;
    }
    flatmemory = 0 as i32;
    i = 0 as i32;
    while i < numflats {
        if *flatpresent.offset(i as isize) != 0 {
            lump = firstflat + i;
            flatmemory += (*lumpinfo.offset(lump as isize)).size;
            W_CacheLumpNum(lump, PU_CACHE as i32);
        }
        i += 1;
    }
    Z_Free(flatpresent as *mut ::core::ffi::c_void);
    texturepresent = Z_Malloc(numtextures, PU_STATIC as i32, NULL)
        as *mut ::core::ffi::c_char;
    memset(
        texturepresent as *mut ::core::ffi::c_void,
        0 as i32,
        numtextures as size_t,
    );
    i = 0 as i32;
    while i < numsides {
        *texturepresent.offset((*sides.offset(i as isize)).toptexture as isize) = 1
            as ::core::ffi::c_char;
        *texturepresent.offset((*sides.offset(i as isize)).midtexture as isize) = 1
            as ::core::ffi::c_char;
        *texturepresent.offset((*sides.offset(i as isize)).bottomtexture as isize) = 1
            as ::core::ffi::c_char;
        i += 1;
    }
    *texturepresent.offset(skytexture as isize) = 1 as ::core::ffi::c_char;
    texturememory = 0 as i32;
    i = 0 as i32;
    while i < numtextures {
        if !(*texturepresent.offset(i as isize) == 0) {
            texture = *textures.offset(i as isize);
            j = 0 as i32;
            while j < (*texture).patchcount as i32 {
                lump = (*(&raw mut (*texture).patches as *mut texpatch_t)
                    .offset(j as isize))
                    .patch;
                texturememory += (*lumpinfo.offset(lump as isize)).size;
                W_CacheLumpNum(lump, PU_CACHE as i32);
                j += 1;
            }
        }
        i += 1;
    }
    Z_Free(texturepresent as *mut ::core::ffi::c_void);
    spritepresent = Z_Malloc(numsprites, PU_STATIC as i32, NULL)
        as *mut ::core::ffi::c_char;
    memset(
        spritepresent as *mut ::core::ffi::c_void,
        0 as i32,
        numsprites as size_t,
    );
    th = thinkercap.next as *mut thinker_t;
    while th != &raw mut thinkercap {
        if matches!((*th).function, ThinkerFn::Mobj(_))
        {
            *spritepresent.offset((*(th as *mut mobj_t)).sprite as isize) = 1
                as ::core::ffi::c_char;
        }
        th = (*th).next as *mut thinker_t;
    }
    spritememory = 0 as i32;
    i = 0 as i32;
    while i < numsprites {
        if !(*spritepresent.offset(i as isize) == 0) {
            j = 0 as i32;
            while j < (*sprites.offset(i as isize)).numframes {
                sf = (*sprites.offset(i as isize)).spriteframes.offset(j as isize)
                    as *mut spriteframe_t;
                k = 0 as i32;
                while k < 8 as i32 {
                    lump = firstspritelump
                        + (*sf).lump[k as usize] as i32;
                    spritememory += (*lumpinfo.offset(lump as isize)).size;
                    W_CacheLumpNum(lump, PU_CACHE as i32);
                    k += 1;
                }
                j += 1;
            }
        }
        i += 1;
    }
    Z_Free(spritepresent as *mut ::core::ffi::c_void);
}
