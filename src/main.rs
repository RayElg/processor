mod programs;
mod instruction_consts;

use instruction_consts::*;

use std::ops::BitAnd;
use std::ops::BitOr;
use std::io::{self, Write};


//Size of RAM
const MEM_LEN: usize = 4096;


fn main() {
    
    let mut registers: [i32; 16] = [0; 16];
    let mut mem: [u8; MEM_LEN] = [0; MEM_LEN];
    let mut flag: u8 = 0b0000000_0;
    //Flag:
    //XXXXXXX|Z
    //X: Reserved

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let program = programs::HELLO2;

    for i in 0..32{ //Copy Hello2 program to memory
        mem[i] = program[i];
    }

    let mut loc: usize = 0;

    //Mainline: iterate over memory until hitting 0 (exit) or passing out of memory
    //Execute instruction at loc
    while loc < MEM_LEN {
        //println!("Loc: {}", loc);
        if mem[loc] == EXIT { //EXIT
            break;

        }else if (mem[loc] >= 10) & (mem[loc] < 30){ //2-register math

            if loc + 2 < MEM_LEN{ //Pass i32 and following bytes function to two_register_math
                match mem[loc] {
                    ADD => {two_register_math(mem[loc+1], mem[loc+2], i32::wrapping_add, &mut registers, &mut flag);}, //All of these could be one-liners instead of function calls
                    SUB => {two_register_math(mem[loc+1], mem[loc+2], i32::wrapping_sub, &mut registers, &mut flag);}, //However, in the interest of possibly passing a custom function to two_register_math in the future, we will keep it like this 
                    AND => {two_register_math(mem[loc+1], mem[loc+2], i32::bitand, &mut registers, &mut flag);},
                    OR => {two_register_math(mem[loc+1], mem[loc+2], i32::bitor, &mut registers, &mut flag);},
                    MULT => {two_register_math(mem[loc+1], mem[loc+2], i32::wrapping_mul, &mut registers, &mut flag);}, 
                    DIV => {two_register_math(mem[loc+1], mem[loc+2], i32::wrapping_div, &mut registers, &mut flag);},
                    ROTATE_LEFT => {},
                    ROTATE_RIGHT => {},
                    MOD => {two_register_math(mem[loc+1], mem[loc+2], i32::rem_euclid, &mut registers, &mut flag);},
                    _ => {},
                };
                loc += 3;
            }else{
                panic!("2-register math instruction has args outside of mem");
            }

        }else if (mem[loc] > 1) & (mem[loc] < 10) { //1-register math
            if loc + 1 < MEM_LEN {
                
                match mem[loc]{
                    INC => {inc(mem[loc + 1] as usize, &mut registers, 1, &mut flag);}, //Could also just be one liners
                    DEC => {inc(mem[loc + 1] as usize, &mut registers, -1, &mut flag);},
                    FLIP => {flip(mem[loc + 1] as usize, &mut registers, &mut flag);},
                    _ => {},
                }
                loc += 2;

            }else{
                panic!("1-register math instruction has arg outside of mem");
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

        }else if (mem[loc] > 40) & (mem[loc] < 50){ //mem/register transfer
            //Instruction is form register, constant (constant is 2 bytes)
            let reg: usize; //Register to read from or write to
            let mem_loc: usize; //Location in memory
            if mem[loc] >= 45{
                if loc + 3 < MEM_LEN {
                    reg = mem[loc+1] as usize;
                    mem_loc = (mem[loc+3] as usize) + ((mem[loc+2] as usize) << 8);
                }else{
                    panic!("mem/register transfer has args outside of mem");
                }
            }else{ //Instruction is form register_register (2 parts of 1 byte)
                if loc + 1 < MEM_LEN{
                    reg = ((mem[loc+1] & 0b1111_0000) >> 4) as usize;
                    mem_loc = registers[(mem[loc+1] & 0b0000_1111) as usize] as usize;
                }else{
                    panic!("mem/register transfer has arg outside of mem");
                }
            }

            if (mem_loc >= MEM_LEN) | ((mem_loc + 3 >= MEM_LEN) & (mem[loc] >= 45)) {
                panic!("mem/register transfer references location out of range");
            }else{

                match mem[loc]{
                    READ_BYTE_C | READ_BYTE_R => {registers[reg] = mem[mem_loc] as i32;},
                    WRITE_BYTE_C | WRITE_BYTE_R => {mem[mem_loc] = (registers[reg] & 0b11111111) as u8;},
                    READ_32_C | READ_32_R => {registers[reg] = bytes_to_i32(&mem[mem_loc..mem_loc+4]);},
                    WRITE_32_C | WRITE_32_R => {
                        let mut contents: i32 = registers[reg];
                        for i in (0..4).rev(){ //Repeatedly write last byte of register and shift register right (so each byte of the register will be last byte at one point)
                            mem[mem_loc+i] = (contents & 0x00_00_00_FF) as u8;
                            contents = contents >> 8;
                        }
                    },
                    _ => {},
                }

                match mem[loc] { //Match on instruction size
                    READ_BYTE_R | WRITE_BYTE_R | READ_32_R | WRITE_32_R => {loc += 2;},
                    READ_32_C | READ_BYTE_C | WRITE_32_C | WRITE_BYTE_C => {loc += 4;},
                    _ => {},
                }

            }
        }else if mem[loc] == JNZ{ //JNZ
            //println!("JNZ");
            if loc + 1 < MEM_LEN {
                let dest: usize = mem[loc+1] as usize;

                jnz(dest, &flag, &mut loc);

            }else{
                panic!("JNZ instruction has arg outside of mem");
            }
        }else if mem[loc] == JZ{ //JZ
            //println!("JZ");
            if loc + 1 < MEM_LEN {
                let dest: usize = mem[loc+1] as usize;

                jz(dest, &flag, &mut loc);

            }else{
                panic!("JZ instruction has arg outside of mem");
            }
        }else if mem[loc] == JNZ_R{ //JNZ_R
            if loc + 1 < MEM_LEN {
                let dest: usize = registers[mem[loc+1] as usize] as usize;

                jnz(dest, &flag, &mut loc);

            }else{
                panic!("JNZ_R instruction has arg outside of mem");
            }
        }else if mem[loc] == JZ_R{ //JZ_R
            if loc + 1 < MEM_LEN {
                let dest: usize = registers[mem[loc+1] as usize] as usize;

                jz(dest, &flag, &mut loc);

            }else{
                panic!("JZ_R instruction has arg outside of mem");
            }
        }else if mem[loc] == PRNTC_LOC {
            if loc + 1 < MEM_LEN{
                let mem_loc = registers[mem[loc+1] as usize] as usize;
                match handle.write(&mem[mem_loc..mem_loc+1]){
                    Ok(_) => {},
                    _ => {panic!("Error writing to stdout")},
                };    
                loc += 2;
            }else{
                panic!("PRNTC_LOC instruction has arg outside of mem");
            }
        }else if mem[loc] == PAD{ //PAD
            loc = loc + 1;
        }else{
            println!("Unknown instruction: {} at loc {}", mem[loc], loc);
            break;
        }

    }

    //Uncomment to reveal fibonacci output
    //Report content of register 1 (nth fibonacci number)
    // println!("Register 1: {}", registers[1]);

    //Show 32bit integers at mem locations
    // for i in (100..100 + (20*4)).step_by(4){
    //      println!("Number contents at {}: {}", i, bytes_to_i32(&mem[i..i+4]));
    // };
}

fn inc(register: usize, registers: &mut [i32], by: i32, flag: &mut u8){ 
    registers[register] += by;
    if registers[register as usize] == 0 {
        *flag = *flag | 0b0000000_1; //zero flag = 1
    }else{
        *flag = *flag & 0b1111111_0; //Zero flag = 0
    }
}

fn flip(register: usize, registers: &mut [i32], flag: &mut u8){
    registers[register] = ! registers[register];
    if registers[register as usize] == 0 {
        *flag = *flag | 0b0000000_1; //zero flag = 1
    }else{
        *flag = *flag & 0b1111111_0; //Zero flag = 0
    }
}

fn two_register_math(byte: u8, dest: u8, operator: fn(i32, i32) -> i32, registers: &mut [i32], flag: &mut u8){
    let reg1: usize = ((byte & 0b1111_0000) >> 4) as usize; //Extract r1, r2 from first byte
    let reg2: usize = (byte & 0b0000_1111) as usize;
    registers[dest as usize] = operator(registers[reg1], registers[reg2]); //Store computed value in destination indicated by second byte
    if registers[dest as usize] == 0 {
        *flag = *flag | 0b0000000_1; //zero flag = 1
    }else{
        *flag = *flag & 0b1111111_0; //Zero flag = 0
    }
}

fn ld_32(register: usize, registers: &mut [i32], val: i32){  //Loads 32-bit integer represented by 4 bytes into register
    registers[register] = val;
}

fn ld_byte(register: usize, registers: &mut [i32], val: u8){  //Loads 8-bit integer into register
    registers[register] = val as i32;
}

//JNZ, JZ: Conditionally change loc based on zero flag
fn jnz(dest: usize, flag: &u8, loc: &mut usize){
    if *flag & 0b0000000_1 == 0{
        *loc = dest;
    }else{
        *loc = *loc + 2;
    }
}

fn jz(dest: usize, flag: &u8, loc: &mut usize){
    if *flag & 0b0000000_1 != 0{
        *loc = dest;
    }else{
        *loc = *loc + 2;
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