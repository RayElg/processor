# Processor

CPU Emulator for a custom assembly language.

16 register, 32-bit CPU that executes instructions from memory byte-array

Building & executing this repo will compile the emulator with example program Hello2 in memory.

## Program examples

These programs would be placed into mem location 0 to be executed

### Fibonacci
Calculates 12th fibonacci number, places it in register 1  
Fibonacci numbers 0 through 11 are written to memory as 32 bit signed integers starting at memory location 100  
```
        LD_BYTE, 1, 0, //a=0
        LD_BYTE, 2, 1, //b=0
        LD_BYTE, 10, 0, //Z=0 (was already 0 regardless)
        LD_BYTE, 14, 12, //target=12
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
```

### Hello2
Writes "helloworld!\n" to stdout

```
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
```

## Instructions:

### Math:

Register contents are taken as signed 32 bit integers

INC (0x02), DEC (0x03), FLIP (0x04):
|XXXXXXXX|0000|AAAA|
|---|---|---|
|Opcode|BLANK|Register|

ADD, SUB wraps on overflow

ADD (0x0A), SUB (0x0B), AND (0x0E), OR (0x0F):
|XXXXXXXX|AAAA|BBBB|0000|DDDD|
|---|---|---|---|---|
|Opcode|Register 1|Register 2|BLANK|Destination Register|

### Load constants:

These load constants defined in instructions into a register

LD_32 (0x1E):
|XXXXXXXX|0000|AAAA|NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN|
|---|---|---|---|
|Opcode|Blank|Register|32-bit integer|

LD_BYTE (0x1F):
|XXXXXXXX|0000|AAAA|NNNNNNNN|
|---|---|---|---|
|Opcode|Blank|Register|8-bit integer|


### Read/Write to mem:

These 4 following instructions read or write to memory given two registers  

READ_32_R (0x28), READ_BYTE_R (0x29), WRITE_32_R (0x2A), WRITE_BYTE_R (0x2B):
|XXXXXXXX|AAAA|BBBB|
|---|---|---|
|Opcode|Register|Memory location register|

READ_32_C (0x2D), WRITE_32_C (0x2F):
|XXXXXXXX|0000|AAAA|LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL|
|---|---|---|---|
|Opcode|Blank|Register|Memory location|

READ_BYTE_C (0x2E), WRITE_BYTE_C (0x30):
|XXXXXXXX|0000|AAAA|LLLLLLLL|
|---|---|---|---|
|Opcode|Blank|Register|Memory location|

### Control flow:

Jump if not zero or jump if zero

JNZ (0x50), JZ (0x51):
|XXXXXXXX|0000|AAAA|LLLLLLLL|
|---|---|---|---|
|Opcode|Blank|Register|Memory location|

### Printing

PRINTC_LOC prints the byte contained at the memory location defined by the given register.

PRINTC_LOC (0x78):
|XXXXXXXX|0000|BBBB|
|---|---|---|
|Opcode|Blank|Memory location register|

### Misc

Pad simply increments program counter. Useful to keep programs aligned

PAD (0xFF):
|11111111|
|---|
|Opcode|

## TODO:
* Implement MOV (currently done using OR)
* Implement MULT, DIV, and rotate left/right
* CALL instruction or JMP instructions that take registers as args(and possibly a stack to go with it)
* Add MOD (modulus)
* Possibly add math instructions using constants rather than register contents
* ~~Move constants to dedicated file if elegant way to do so exists~~
* Possibly add instructions to assist with printing chararrays, 32-bit integers
* Clean up mainline (possibly add debugging mode)
* CPU Flags
* Read & execute binary files (rather than copying const arrays into mem array)
* **Write an assembler**
* Create a cool way to poke at memory/registers