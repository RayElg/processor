#![allow(dead_code)]
//INSTRUCTION CONSTANTS
//0: Exit
const EXIT: u8 = 0;

//1: Move
const MOV: u8 = 1; //Unimplemented

//2-9: 1 register math
const INC: u8 = 2; // 0x01 0x0000_[REGISTER]
const DEC: u8 = 3; // 0x02 0x0000_[REGISTER]
const FLIP: u8 = 4; //Unimplemented

//10 - 29: 2 register math
const ADD: u8 = 10;
const SUB: u8 = 11;
const MULT: u8 = 12;
const DIV: u8 = 13;
const AND: u8 = 14;
const OR: u8 = 15;
const ROTATE_LEFT: u8 = 16; //Unimplemented
const ROTATE_RIGHT: u8 = 17; //Unimplemented

//30, 31: Load constants
const LD_32: u8 = 30;
const LD_BYTE: u8 = 31;

//40-49: mem/register transfer (Unimplemented)

const READ_32_R: u8 = 40; //Read/write locations at registers
const READ_BYTE_R: u8 = 41;
const WRITE_32_R: u8 = 42;
const WRITE_BYTE_R: u8 = 43;

const READ_32_C: u8 = 45; //Read/write locations at constants
const READ_BYTE_C: u8 = 46;
const WRITE_32_C: u8 = 47;
const WRITE_BYTE_C: u8 = 48;

//50-59: Mem manipulation (Unimplemented)

//80-99: Control flow
const JNZ: u8 = 80;
const JZ: u8 = 81;

//100-119

//120-129: Printing
const PRNTC_LOC: u8 = 120;

//255: PAD (continue)
const PAD: u8 = 0xFF;


pub const FIB: [u8; 54] = [ //Calculates 12th fibonacci number
        LD_BYTE, 1, 0, //a=0
        LD_BYTE, 2, 1, //b=0
        LD_BYTE, 10, 0, //Z=0 (was already 0 regardless)
        LD_BYTE, 14, 19, //target=19
        LD_BYTE, 15, 0, //i=0 (was already 0 regardless)

        LD_BYTE, 9, 100, //PTR=100

        WRITE_32_R, 0b0001_1001, PAD, //Write a to PTR
        INC, 9, PAD, //Increment PTR by 4
        INC, 9, PAD,
        INC, 9, PAD,
        INC, 9, PAD,

        ADD, 0b0001_0010, 3, //c = a+b
        OR, 0b0001_1010, 2, //b = 1 || Z
        OR, 0b0011_1010, 1, //a = c || Z
        INC, 15, PAD, //i++
                      //If we wanted our binary file to be aligned in a hex editor we could use PAD to ensure each newline starts with an instruction
        SUB, 0b1110_1111, 7, //diff=target-i
        JNZ, 7, 18, //Jump to loc=18 if diff != 0
        WRITE_32_R, 0b0001_1001, PAD,
];

pub const HELLO: [u8; 58] = [
        LD_BYTE, 0, 100, //PTR=100
        LD_BYTE, 1, b'h', //Put letter in register 1
        WRITE_BYTE_R, 0b0001_0000, //Write register 1 to PTR
        INC, 0, PAD, //INC PTR
        LD_BYTE, 1, b'e', //Put letter in register 1
        WRITE_BYTE_R, 0b0001_0000, //Write register 1 to PTR
        INC, 0, PAD, //INC PTR
        LD_BYTE, 1, b'l', //Put letter in register 1
        WRITE_BYTE_R, 0b0001_0000, //Write register 1 to PTR
        INC, 0, PAD, //INC PTR
        LD_BYTE, 1, b'l', //Put letter in register 1
        WRITE_BYTE_R, 0b0001_0000, //Write register 1 to PTR
        INC, 0, PAD, //INC PTR
        LD_BYTE, 1, b'o', //Put letter in register 1
        WRITE_BYTE_R, 0b0001_0000, //Write register 1 to PTR
        INC, 0, PAD, //INC PTR
        LD_BYTE, 2, 100, //PTR2 = 100
        PRNTC_LOC, 0b00000010, PAD, //Print character at PTR2
        INC, 2, PAD, //INC PTR2
        SUB, 0b0010_0000, 10,
        JNZ, 10, 46
];

pub const HELLO2: [u8; 33] = [
    JZ, 0, 15, //Jump past characters
    b'h', b'e', b'l', //put 'helloworld!' in memory
    b'l', b'o', b'w',
    b'o', b'r', b'l',
    b'd', b'!', b'\n',
    LD_BYTE, 0, 3, //PTR=3
    LD_BYTE, 1, 14, //TARGET=14
    PRNTC_LOC, 0, PAD, //PRNT PTR
    INC, 0, PAD, //PTR++
    SUB, 0b0001_0000, 14, //DIFF = TARGET-PTR
    JNZ, 14, 21 //Jump to PRNTC_LOC location if DIFF != 0
];