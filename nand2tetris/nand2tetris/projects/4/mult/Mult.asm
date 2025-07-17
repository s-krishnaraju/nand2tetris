// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

//// Replace this comment with your code.
@0
D=A
@R2 
M=D
@R1 
M=D
(LOOP)
// end if 0
@R0
D=M
@END
D;JEQ

@R1
M = M + M
// decrement
@R0
M=M-1
@LOOP
0;JMP

(END)
@END 
0;JMP

