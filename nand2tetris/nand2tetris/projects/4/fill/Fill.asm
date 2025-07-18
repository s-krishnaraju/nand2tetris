// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

//This program runs an infinite loop that listens to the
//keyboard. When a key is pressed (any key), the program blackens the
//screen by writing black in every pixel. When no key is pressed, the program
//clears the screen by writing white in every pixel. You may choose to
//blacken and clear the screen in any spatial pattern, as long as pressing a key
//continuously for long enough will result in a fully blackened screen, and not
//pressing any key for long enough will result in a cleared screen. This
//program has a test script (Fill.tst) but no compare file—it should be checked
//by visibly inspecting the simulated screen in the CPU emulator.

@256
D=A
@num_rows
M=D

@32
D=A
@num_cols
M=D

(LOOP)
//init
@SCREEN
D=A
@screen_ptr
M=D

@0
D=A
@row
M=D

//check keyboard
@KBD
D=M
@BLACK
D;JGT

// set white
@color
M=0
@DRAW
0;JMP

// set black
(BLACK)
@color
M=-1
@DRAW
0;JMP

(DRAW)
// exit if row==num_rows
@num_rows
D=M
@row
D=D-M
@LOOP
D;JEQ

//init
@0
D=A
@col
M=D

(DRAW_ROW)
// exit if cols==num_cols
@num_cols
D=M
@col
D=D-M
@END_DRAW_ROW
D;JEQ

@color
D=M
@screen_ptr
A=M
M=D

@col
M=M+1
@screen_ptr
M=M+1

@DRAW_ROW
0;JMP
(END_DRAW_ROW)
@row
M=M+1
@DRAW
0;JMP
