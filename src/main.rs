use std::ops::BitAnd;
use std::ops::BitOr;

//Size of RAM
const MEM_LEN: usize = 4096;

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
const READ_32: u8 = 40;
const READ_BYTE: u8 = 41;
const WRITE_32: u8 = 42;
const WRITE_BYTE: u8 = 43;

//50-59: Mem manipulation (Unimplemented)

//80-99: Control flow
const JNZ: u8 = 80;
const JZ: u8 = 81;

//255: PAD (continue)
const PAD: u8 = 0xFF;

fn main() {
    
    let mut registers: [i32; 16] = [0; 16];
    let mut mem: [u8; MEM_LEN] = [0; MEM_LEN];


    let program = [ //Calculates 12th fibonacci number
        LD_BYTE, 1, 0, //a=0
        LD_BYTE, 2, 1, //b=0
        LD_BYTE, 10, 0, //Z=0 (was already 0 regardless)
        LD_BYTE, 14, 12, //target=12
        LD_BYTE, 15, 0, //i=0 (was already 0 regardless)
        ADD, 0b0001_0010, 3, //c = a+b
        OR, 0b0001_1010, 2, //b = 1 || Z
        OR, 0b0011_1010, 1, //a = c || Z
        INC, 15, PAD, //i++
                      //If we wanted our binary file to be aligned in a hex editor we could use PAD to ensure each newline starts with an instruction
        SUB, 0b1110_1111, 7, //diff=target-i
        JNZ, 7, 15 //Jump to loc=7 if diff != 0
    ];

    for i in 0..33{ //Copy fibonacci program to memory
        mem[i] = program[i];
    }

    let mut loc: usize = 0;

    //Mainline: iterate over memory until hitting 0 (exit) or passing out of memory
    //Execute instruction at loc
    while loc < MEM_LEN {

        if mem[loc] == EXIT { //EXIT
            break;

        }else if (mem[loc] >= 10) & (mem[loc] < 30){ //2-register math

            if loc + 2 < MEM_LEN{ //Pass i32 and following bytes function to two_register_math
                match mem[loc] {
                    ADD => {two_register_math(mem[loc+1], mem[loc+2], i32::wrapping_add, &mut registers);},
                    SUB => {two_register_math(mem[loc+1], mem[loc+2], i32::wrapping_sub, &mut registers);},
                    AND => {two_register_math(mem[loc+1], mem[loc+2], i32::bitand, &mut registers);},
                    OR => {two_register_math(mem[loc+1], mem[loc+2], i32::bitor, &mut registers);},
                    MULT => {},
                    DIV => {},
                    ROTATE_LEFT => {},
                    ROTATE_RIGHT => {},
                    _ => {},
                };
                loc += 3;
            }else{
                panic!("2-register math instruction has args outside of mem");
            }

        }else if mem[loc] == INC { //INC
            //println!("INC");
            if loc + 1 < MEM_LEN {
                
                inc(mem[loc + 1] as usize, &mut registers, 1);
                loc += 2;

            }else{
                panic!("INC instruction has arg outside of mem");
            }
        }else if mem[loc] == DEC { //DEC
            //println!("DEC");
            if loc + 1 < MEM_LEN {
                
                inc(mem[loc + 1] as usize, &mut registers, -1);
                loc += 2;

            }else{
                panic!("DEC instruction has arg outside of mem");
            }
        }else if mem[loc] == LD_32 { //LD_32
            //println!("LD_32");
            if loc + 5 < MEM_LEN {
                let dest: usize = mem[loc+1] as usize;
                let bytes = &mem[loc+2..loc+6];
                let the_int = bytes_to_i32(&bytes);

                ld_32(dest, &mut registers, the_int);
                loc += 6;

            }else{
                panic!("LD_32 instruction has args outside of mem");
            }
        }else if mem[loc] == LD_BYTE { //LD_BYTE
            //println!("LD_BYTE");
            if loc + 2 < MEM_LEN {
                let byte: u8 = mem[loc + 2];
                let dest: usize = mem[loc + 1] as usize;

                ld_byte(dest, &mut registers, byte);
                
                loc += 3;

            }else{
                panic!("LD_BYTE instruction has args outside of mem");
            }

        }else if mem[loc] == JNZ{ //JNZ
            //println!("JNZ");
            if loc + 2 < MEM_LEN {
                let dest: usize = mem[loc+2] as usize;
                let register: usize = mem[loc+1] as usize;

                jnz(register, dest, &mut registers, &mut loc);

            }else{
                panic!("JNZ instruction has args outside of mem");
            }
        }else if mem[loc] == JZ{ //JZ
            //println!("JZ");
            if loc + 2 < MEM_LEN {
                let dest: usize = mem[loc+2] as usize;
                let register: usize = mem[loc+1] as usize;

                jz(register, dest, &mut registers, &mut loc);

            }else{
                panic!("JZ instruction has args outside of mem");
            }
        }else if mem[loc] == PAD{ //PAD
            loc = loc + 1;
        }else{
            println!("Unknown instruction: {} at loc {}", mem[loc], loc);
            break;
        }

    }

    //Report content of register 1 (12th fibonacci number)
    println!("Register 1: {}", registers[1]);
}


fn inc(register: usize, registers: &mut [i32], by: i32){ 
    registers[register] += by;
}

fn two_register_math(byte: u8, dest: u8, operator: fn(i32, i32) -> i32, registers: &mut [i32]){
    let reg1: usize = ((byte & 0b1111_0000) >> 4) as usize; //Extract r1, r2 from first byte
    let reg2: usize = (byte & 0b0000_1111) as usize;
    registers[dest as usize] = operator(registers[reg1], registers[reg2]); //Store computed value in destination indicated by second byte
}

fn ld_32(register: usize, registers: &mut [i32], val: i32){  //Loads 32-bit integer represented by 4 bytes into register
    registers[register] = val;
}

fn ld_byte(register: usize, registers: &mut [i32], val: u8){  //Loads 8-bit integer into register
    registers[register] = val as i32;
}

// fn write_32(register: usize, dest: usize, registers: &mut [i32]);

// fn write_byte();

// fn read_32();

// fn read_byte();

//JNZ, JZ: Compare register to 0, conditionally change loc
fn jnz(register: usize, dest: usize, registers: &mut [i32], loc: &mut usize){
    if registers[register] != 0{
        *loc = dest;
    }else{
        *loc = *loc + 3;
    }
}

fn jz(register: usize, dest: usize, registers: &mut [i32], loc: &mut usize){
    if registers[register] == 0{
        *loc = dest;
    }else{
        *loc = *loc + 3;
    }
}


//Helpers
fn bytes_to_i32(bytes: &[u8])->i32{
    let mut the_int: i32 = 0;
    the_int = the_int + (bytes[3] as i32);
    the_int = the_int + ((bytes[2] as i32) << 8);
    the_int = the_int + ((bytes[1] as i32) << 16);
    the_int = the_int + ((bytes[0] as i32) << 24);
    return the_int;
}