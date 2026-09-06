pub type byte = u8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ticcmd_t {
    pub forwardmove: i8,
    pub sidemove: i8,
    pub angleturn: i16,
    pub chatchar: byte,
    pub buttons: byte,
    pub consistancy: byte,
    pub buttons2: byte,
    pub inventory: i32,
    pub lookfly: byte,
    pub arti: byte,
}
