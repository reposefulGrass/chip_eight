
#![allow(non_camel_case_types)]

use crate::types::*;


#[derive(Debug, PartialEq)]
enum Instruction {
    /* 00E0 */ CLS,
    /* 00EE */ RET,
    /* 0nnn */ SYS(Address),
    /* 1nnn */ JP_A(Address),
    /* 2nnn */ CALL(Address),
    /* 3xkk */ SE_B(Register, u8),
    /* 4xkk */ SNE(Register, u8),
    /* 5xy0 */ SE_R(Register, Register),
    /* 6xkk */ LD_B(Register, u8),
    /* 7xkk */ ADD_B(Register, u8),
    /* 8xy0 */ LD_R(Register, Register),
    /* 8xy1 */ OR(Register, Register),
    /* 8xy2 */ AND(Register, Register),
    /* 8xy3 */ XOR(Register, Register),
    /* 8xy4 */ ADD_R(Register, Register),
    /* 8xy5 */ SUB(Register, Register),
    /* 8xy6 */ SHR(Register),
    /* 8xy7 */ SUBN(Register, Register),
    /* 8xyE */ SHL(Register),
    /* 9xy0 */ SNE(Register, Register),
    /* Annn */ LD_A(I, Address),
    /* Bnnn */ JP_R(Register, Address),
    /* Cxkk */ RND(Register, u8),
    /* Dxyn */ DRW(Register, Register, u8),
    /* Ex9E */ SKP(Register),
    /* ExA1 */ SKNP(Register),
    /* Fx07 */ ST_DT(Register),
    /* Fx0A */ LD_K(Register),
    /* Fx15 */ LD_DT(Register),
    /* Fx18 */ LD_ST(Register),
    /* Fx1E */ ADD_I(I, Register),
    /* Fx29 */ LD_F(Register),
    /* Fx33 */ ST_BCD(Register),
    /* Fx55 */ ST_REG(I, Register),
    /* Fx65 */ LD_REG(Register, I),
}


impl Instruction {
    fn parse (bytes: u16) -> Instruction {
        match bytes {
            0x00E0 => { 
                Instruction::CLS 
            },

            0x00EE => { 
                Instruction::RET 
            },

            0x0000..=0x0FFF => {
                Instruction::SYS(bytes & 0x0FFF as Address)
            },

            0x1000..=0x1FFF => { 
                Instruction::JP(bytes & 0x0FFF as Address)
            },

            0x2000..=0x2FFF => {
                Instruction::CALL(bytes & 0x0FFF as Address)
            },

            0x3000..=0x3FFF => {
                let register = convert_to_register((bytes & 0x0F00) >> 8);
                let byte = (bytes & 0x00FF) as u8;

                Instruction::SE(register, byte)
            },

            0x4000..=0x4FFF => {
                let register = convert_to_register((bytes & 0x0F00) >> 8);
                let byte = (bytes & 0x00FF) as u8;

                Instruction::SNE(register, byte)
            },

            0x5000..=0x5FFF => {
                let register_x = convert_to_register((bytes & 0x0F00) >> 8); 
                let register_y = convert_to_register((bytes & 0x00F0) >> 4); 

                if bytes & 0x000F != 0 {
                    panic!("Invalid Instruction: {:#X}", bytes);
                }

                Instruction::SE_R(register_x, register_y)
            },

            0x6000..=0x6FFF => {
                let register = convert_to_register((bytes & 0x0F00) >> 8);
                let byte = (bytes & 0x00FF) as u8;

                Instruction::LD_B(register, byte)
            },

            0x7000..=0x7FFF => {
                let register = convert_to_register((bytes & 0x0F00) >> 8);
                let byte = (bytes & 0x00FF) as u8;

                Instruction::ADD_B(register, byte)
            },

            0x8000..=0x8FFF => {
                match bytes & 0x000F {
                    0x0 => {
                        let register_x = convert_to_register((bytes & 0x0F00) >> 8);
                        let register_y = convert_to_register((bytes & 0x00F0) >> 4);

                        Instruction::LD_R(register_x, register_y)
                    },

                    0x1 => {
                        let register_x = convert_to_register((bytes & 0x0F00) >> 8);
                        let register_y = convert_to_register((bytes & 0x00F0) >> 4);

                        Instruction::OR(register_x, register_y)
                    },

                    0x2 => {
                        let register_x = convert_to_register((bytes & 0x0F00) >> 8);
                        let register_y = convert_to_register((bytes & 0x00F0) >> 4);

                        Instruction::AND(register_x, register_y)
                    },

                    0x3 => {
                        let register_x = convert_to_register((bytes & 0x0F00) >> 8);
                        let register_y = convert_to_register((bytes & 0x00F0) >> 4);

                        Instruction::XOR(register_x, register_y)
                    },

                    0x4 => {
                        let register_x = convert_to_register((bytes & 0x0F00) >> 8);
                        let register_y = convert_to_register((bytes & 0x00F0) >> 4);

                        Instruction::ADD_R(register_x, register_y)
                    },
                    
                    0x5 => {
                        let register_x = convert_to_register((bytes & 0x0F00) >> 8);
                        let register_y = convert_to_register((bytes & 0x00F0) >> 4);

                        Instruction::SUB(register_x, register_y)
                    },

                    _ => {
                        panic!("Could not parse instruction!");
                    }
                }
            }

            _ => {
                panic!("Invalid Instruction: {:#X}", bytes);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cls () {
        let instr = Instruction::parse(0x00E0 as u16);        
        assert!(instr == Instruction::CLS);
    }

    #[test]
    fn test_ret () {
        let instr = Instruction::parse(0x00EE as u16);
        assert!(instr == Instruction::RET);
    }

    #[test]
    fn test_sys () {
        let bytes = 0x0123 as u16;
        let instr = Instruction::parse(bytes);
        
        match instr {
            Instruction::SYS(address) => {
                assert!(address == 0x123);
            },

            _ => {
                panic!("Could not parse instruction."); 
            },
        }
    }

    #[test]
    fn test_jp () {
        let bytes = 0x1234 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::JP(address) => {
                assert!(address == 0x234);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_call () {
        let bytes = 0x2345 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::CALL(address) => {
                assert!(address == 0x345);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_se () {
        let bytes = 0x3456 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::SE(register, byte) => {
                assert!(register == 0x4);
                assert!(byte == 0x56);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_sne () {
        let bytes = 0x4567 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::SNE(register, byte) => {
                assert!(register == 0x5);
                assert!(byte == 0x67);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_se_r () {
        let bytes = 0x5670 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::SE_R(register_a, register_b) => {
                assert!(register_a == 0x6);
                assert!(register_b == 0x7);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_ld_b () {
        let bytes = 0x6789 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::LD_B(register, byte) => {
                assert!(register == 0x7);
                assert!(byte == 0x89);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_add_b () {
        let bytes = 0x7890 as u16;
        let instr = Instruction::parse(bytes);
    
        match instr {
            Instruction::ADD_B(register, byte) => {
                assert!(register == 0x8);
                assert!(byte == 0x90);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_ld_r () {
        let bytes = 0x8120 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::LD_R(register_x, register_y) => { 
                assert!(register_x == 0x1);
                assert!(register_y == 0x2);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_or () {
        let bytes = 0x8121 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::OR_R(register_x, register_y) => { 
                assert!(register_x == 0x1);
                assert!(register_y == 0x2);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_and () {
        let bytes = 0x8122 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::AND_R(register_x, register_y) => { 
                assert!(register_x == 0x1);
                assert!(register_y == 0x2);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_xor () {
        let bytes = 0x8123 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::XOR_R(register_x, register_y) => { 
                assert!(register_x == 0x1);
                assert!(register_y == 0x2);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_add () {
        let bytes = 0x8124 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::ADD_R(register_x, register_y) => { 
                assert!(register_x == 0x1);
                assert!(register_y == 0x2);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }

    #[test]
    fn test_sub () {
        let bytes = 0x8125 as u16;
        let instr = Instruction::parse(bytes);

        match instr {
            Instruction::SUB_R(register_x, register_y) => { 
                assert!(register_x == 0x1);
                assert!(register_y == 0x2);
            },

            _ => {
                panic!("Could not parse instruction.");
            },
        }
    }
}

