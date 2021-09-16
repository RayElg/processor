#![allow(dead_code)]
use crate::instruction_consts::*;


pub const FIB: [u8; 53] = [ //Calculates 19th fibonacci number
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
        JNZ, 18, //Jump to loc=18 if diff != 0
        WRITE_32_R, 0b0001_1001, PAD,
];


pub const FIB2: [u8; 45] = [ //Calculates 32nd fibonacci number
        LD_BYTE, 1, 0, //a=0
        LD_BYTE, 2, 1, //b=1
        LD_BYTE, 10, 0, //Z=0 (was already 0 regardless)
        LD_BYTE, 13, 32, //target=32
        LD_BYTE, 14, 0, //i=0 (was already 0 regardless)
        ADD, 0b0001_0010, 3, //c = a+b
        MOV, 0b0001_0010, //b = a
        MOV, 0b0011_0001, //a = c
        INC, 14, PAD, //i++
        PUSHA, PAD, PAD, //Push all registers to stack
        LD_BYTE, 15, 37, //Return location
        LD_BYTE, 10, 240, //Location of print program
        JNZ_R, 10, PAD, //Go to print
        POPA, PAD, PAD, //Pop all registers back on to stack
        SUB, 0b1101_1110, 7, //diff=target-i
        JNZ, 15, //Jump to loc=18 if diff != 0
];

pub const HELLO: [u8; 57] = [
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
        JNZ, 45
];

pub const HELLO2: [u8; 32] = [
    JNZ, 15, PAD, //Jump past characters
    b'h', b'e', b'l', //put 'helloworld!' in memory
    b'l', b'o', b'w',
    b'o', b'r', b'l',
    b'd', b'!', b'\n',
    LD_BYTE, 0, 3, //PTR=3
    LD_BYTE, 1, 15, //TARGET=15
    PRNTC_LOC, 0, PAD, //PRNT PTR
    INC, 0, PAD, //PTR++
    SUB, 0b0001_0000, 14, //DIFF = TARGET-PTR
    JNZ, 21 //Jump to PRNTC_LOC location if DIFF != 0
];


//Right now this program prints a 32-bit integer in reverse. When I implement stack I'll have this program
//Utilize the stack to hold the ASCII digits and possibly to hold args


//REGISTERS USED: 14 (10),13 (ptr),12 (MOD loc),11 (remainder), 9 (value to print, divided), 8 (byte to print), 7 (30)
//Args: 15 (return), 1 (to print), 10 (starting location of PRINT32)
pub const PRINT32: [u8; 54] = [ //Print (positive) 32-bit number in register 1, then jump to mem loc at 15
        LD_BYTE, 14, 10, //Stores number 10
        LD_BYTE, 13, 200, //Stores ptr = 200 <- now unused?
        LD_BYTE, 12, 30, //Location of MOD
        LD_BYTE, 7, 0x30, //Stores number 0x30
        LD_BYTE, 8, b'\0', //Load null character
        PUSH, 8, PAD, //Push character onto stack
        LD_BYTE, 8, b' ', //Load space
        PUSH, 8, PAD, //Push character onto stack
        ADD, 0b1100_1010, 12, //Actual location of MOD
        OR, 0b0001_0001, 9, //Copy 1 to 9
        MOD, 0b1001_1110, 8, //r9 % r14 -> r8
        DIV, 0b1001_1110, 9, //r9 / r14 -> r9
        ADD, 0b0111_1000, 8, //r8 + r7 -> r8 (adds 30)
        PUSH, 8, PAD, //Push character onto stack
        OR, 0b1001_1001, 9, //9 | 9 -> 9
        JNZ_R, 12, PAD,
        PRNT_STACK, PAD, PAD,
        JZ_R, 15, PAD
];
