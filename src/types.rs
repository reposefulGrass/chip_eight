
pub type Address = u16;
pub type I = u16;
pub type Register = u8;


pub fn convert_to_register (byte: u16) -> Register {
    if byte > 0xF {
        panic!("Invalid Register");
    }

    byte as Register
}
