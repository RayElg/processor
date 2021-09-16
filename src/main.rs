mod programs;
mod instruction_consts;

use instruction_consts::*;

use std::ops::BitAnd;
use std::ops::BitOr;
use std::io::{self, Write};


//Size of RAM
const MEM_LEN: usize = 8192;


fn main() {
    
    let mut registers: [i32; 16] = [0; 16];
    let mut mem: [u8; MEM_LEN] = [0; MEM_LEN];
    let mut flag: u8 = 0b0000000_0;
    let mut sp: u16 = 4096;
    //Flag:
    //XXXXXXX|Z
    //X: Reserved

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let program = programs::FIB2;
    let program_print = programs::PRINT32;

    for i in 0..45{
        mem[i] = program[i];
    }

    for i in 240..294{ //Copy Print32 program to memory
        mem[i] = program_print[i-240];
    }

    let mut loc: usize = 0;

    while loc < MEM_LEN{
        //println!("Loc: {}", loc);
        //println!("Reg 14: {}", registers[14]);
        match mem[loc] {
            EXIT => break,
            MOV => {
                mov(&mut registers, &mem[loc + 1]);
                loc += 2;
            },
            INC..=FLIP => { //1 reg math
                match mem[loc]{
                    INC => {inc(mem[loc + 1] as usize, &mut registers, 1, &mut flag);}, //Could also just be one liners
                    DEC => {inc(mem[loc + 1] as usize, &mut registers, -1, &mut flag);},
                    FLIP => {flip(mem[loc + 1] as usize, &mut registers, &mut flag);},
                    _ => {},
                }
                loc += 2;
            }, 
            ADD..=MOD => { //2 reg math
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
            }, 
            LD_32 => { //Load constants
                let dest: usize = mem[loc+1] as usize;
                let bytes = &mem[loc+2..loc+6];
                let the_int = bytes_to_i32(&bytes);

                ld_32(dest, &mut registers, the_int);
                loc += 6;
            }, 
            LD_BYTE => {
                let byte: u8 = mem[loc + 2];
                let dest: usize = mem[loc + 1] as usize;

                ld_byte(dest, &mut registers, byte);
                
                loc += 3;
            },
            READ_32_R..=WRITE_BYTE_C => { //Mem/Register transfer

                let reg: usize; //Register to read from or write to
                let mem_loc: usize; //Location in memory
                let inc_size: usize; //How much to increment loc afterwards


                match mem[loc]{
                    READ_32_R..=WRITE_BYTE_R => {
                        reg = mem[loc+1] as usize;
                        mem_loc = (mem[loc+3] as usize) + ((mem[loc+2] as usize) << 8);
                        inc_size = 2;
                    },
                    _ => {
                        reg = ((mem[loc+1] & 0b1111_0000) >> 4) as usize;
                        mem_loc = registers[(mem[loc+1] & 0b0000_1111) as usize] as usize;
                        inc_size = 4;
                    },
                }
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

                loc += inc_size;


            }, 
            PUSH | POP | PUSHA | POPA=> { //Mem/Stack manipulation
                
                match mem[loc] {
                    PUSH | POP => {
                        let register: usize = mem[loc + 1] as usize;

                        if mem[loc] == PUSH{
                            push_reg(register, &registers, &mut mem, &mut sp);
                        }else{
                            pop_reg(register, &mut registers, &mem, &mut sp);
                        }
                        loc += 2;
                    },
                    _ => {

                        if mem[loc] == PUSHA{
                            //println!("Pushing all...");
                            //println!("Pushed: {:?}", registers);
                            for register in 0..16{
                                push_reg(register, &registers, &mut mem, &mut sp);
                            }
                        }else{
                            //println!("Popping all...");
                            //println!("Before: {:?}", registers);
                            for register in (0..16).rev(){
                                pop_reg(register, &mut registers, &mem, &mut sp);
                            }
                            //println!("After: {:?}", registers);
                        }
                        loc += 1;

                    },
                }

            }, 
            JNZ..=JZ_R => { //Control flow

                let dest: usize;
                match mem[loc]{
                    JNZ_R..=JZ_R => {dest = registers[mem[loc+1] as usize] as usize;}
                    _ => {dest = mem[loc+1] as usize;}
                }

                match mem[loc]{
                    JZ | JZ_R => {jz(dest, &flag, &mut loc);}
                    _ => {jnz(dest, &flag, &mut loc);}
                }

            }, 
            PRNTC_LOC => { //Printing
                let mem_loc = registers[mem[loc+1] as usize] as usize;
                match handle.write(&mem[mem_loc..mem_loc+1]){
                    Ok(_) => {},
                    _ => {panic!("Error writing to stdout")},
                };    
                loc += 2;
            },
            PRNT_STACK => {
                let mut the_byte = (pop(&mem, &mut sp) & 0xFF) as u8;
                while the_byte != b'\0'{
                    match handle.write(std::slice::from_ref(&the_byte)){
                        Ok(_) => {},
                        _ => {panic!("Error writing to stdout")},
                    };  
                    the_byte = (pop(&mem, &mut sp) & 0xFF) as u8;
                }
                loc += 1;
            } 
            PAD => {loc = loc + 1},
            _ => println!("Unknown instruction {} at {}", mem[loc], loc),
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

fn mov(registers: &mut [i32], byte: &u8){
    let source: usize = ((byte & 0xF0) >> 4) as usize;
    let dest: usize = (byte & 0x0F) as usize;
    registers[dest] = registers[source];
}

fn push_reg(register: usize, registers: &[i32], mem: &mut [u8], sp: &mut u16){
    mem[*sp as usize] = ((registers[register] >> 24) & 0x00_00_00_FF) as u8;
    mem[(*sp + 1) as usize] = ((registers[register] >> 16) & 0x00_00_00_FF) as u8;
    mem[(*sp + 2) as usize] = ((registers[register] >> 8) & 0x00_00_00_FF) as u8;
    mem[(*sp + 3) as usize] = ((registers[register]) & 0x00_00_00_FF) as u8;
    *sp += 4;
}

fn pop_reg(register: usize, registers: &mut [i32], mem: &[u8], sp: &mut u16){
    *sp -= 4;
    registers[register] = bytes_to_i32(&mem[(*sp) as usize..(*sp+4) as usize]);
}

fn pop(mem: &[u8], sp: &mut u16)->i32{
    *sp -= 4;
    bytes_to_i32(&mem[(*sp) as usize..(*sp+4) as usize])
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