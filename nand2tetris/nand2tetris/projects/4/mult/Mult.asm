// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

//get addition const
@R1
D=M
@R4
M=D
//initialize to zero
@0
D=A
@R2 
M=D
@R1
M=D
(LOOP)
//check done mult
@R0
D=M
@END
D;JLE
// add constant
@R4
D=M
@R1
M=D+M
//decrement
@R0
M=M-1
//loop
@LOOP
0;JMP
(END)
//set output
@R1 
D=M
@R2
M=D
(ENDLOOP)
@ENDLOOP
0;JMP

