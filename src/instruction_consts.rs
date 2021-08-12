
//INSTRUCTION CONSTANTS
//0: Exit
pub const EXIT: u8 = 0;

//1: Move
pub const MOV: u8 = 1; //Unimplemented

//2-9: 1 register math
pub const INC: u8 = 2; // 0x01 0x0000_[REGISTER]
pub const DEC: u8 = 3; // 0x02 0x0000_[REGISTER]
pub const FLIP: u8 = 4; //Unimplemented

//10 - 29: 2 register math
pub const ADD: u8 = 10;
pub const SUB: u8 = 11;
pub const MULT: u8 = 12;
pub const DIV: u8 = 13;
pub const AND: u8 = 14;
pub const OR: u8 = 15;
pub const ROTATE_LEFT: u8 = 16; //Unimplemented
pub const ROTATE_RIGHT: u8 = 17; //Unimplemented
pub const MOD: u8 = 18;

//30, 31: Load pub constants
pub const LD_32: u8 = 30;
pub const LD_BYTE: u8 = 31;

//40-49: mem/register transfer (Unimplemented)

pub const READ_32_R: u8 = 40; //Read/write locations at registers
pub const READ_BYTE_R: u8 = 41;
pub const WRITE_32_R: u8 = 42;
pub const WRITE_BYTE_R: u8 = 43;

pub const READ_32_C: u8 = 45; //Read/write locations at pub constants
pub const READ_BYTE_C: u8 = 46;
pub const WRITE_32_C: u8 = 47;
pub const WRITE_BYTE_C: u8 = 48;

//50-59: Mem manipulation (Unimplemented)

//80-99: Control flow
pub const JNZ: u8 = 80;
pub const JZ: u8 = 81;

//100-119

//120-129: Printing
pub const PRNTC_LOC: u8 = 120;

//255: PAD (continue)
pub const PAD: u8 = 0xFF;