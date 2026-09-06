pub const FRACBITS: i32 = 16 as i32;
pub const FRACUNIT: i32 = (1 as i32) << FRACBITS;
pub static mut skyflatnum: i32 = 0;
pub static mut skytexture: i32 = 0;
pub static mut skytexturemid: i32 = 0;
pub unsafe fn R_InitSkyMap() {
    skytexturemid = 100 as i32 * FRACUNIT;
}
