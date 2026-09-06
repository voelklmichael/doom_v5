extern "C" {
    fn abs(__x: i32) -> i32;
}
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type fixed_t = i32;
pub const INT_MAX: i32 = __INT_MAX__;
pub const INT_MIN: i32 = -__INT_MAX__ - 1 as i32;
pub const FRACBITS: i32 = 16 as i32;
pub unsafe fn FixedMul(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return (a as int64_t * b as int64_t >> FRACBITS) as fixed_t;
}
pub unsafe fn FixedDiv(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    if abs(a as i32) >> 14 as i32
        >= abs(b as i32)
    {
        return if a ^ b < 0 as i32 { INT_MIN } else { INT_MAX }
    } else {
        let mut result: int64_t = 0;
        result = ((a as int64_t) << 16 as i32) / b as int64_t;
        return result as fixed_t;
    };
}
pub const __INT_MAX__: i32 = 2147483647 as i32;
