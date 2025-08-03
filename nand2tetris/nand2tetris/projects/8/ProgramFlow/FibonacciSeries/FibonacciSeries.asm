//	push argument 1         // sets THAT, the base address of the
@1
D=A
@ARG
D=D+M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
//	pop pointer 1           // that segment, to argument[1]
@SP
M=M-1
@SP
A=M
D=M
@THAT

M=D
//	push constant 0         // sets the series' first and second
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
//	pop that 0              // elements to 0 and 1, respectively       
@0
D=A
@THAT
M=M+D
@SP
M=M-1
@SP
A=M
D=M
@THAT
A=M
M=D
@0
D=A
@THAT
M=M-D
//	push constant 1   
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
//	pop that 1              
@1
D=A
@THAT
M=M+D
@SP
M=M-1
@SP
A=M
D=M
@THAT
A=M
M=D
@1
D=A
@THAT
M=M-D
//	push argument 0         // sets n, the number of remaining elements
@0
D=A
@ARG
D=D+M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
//	push constant 2         // to be computed, to argument[0] minus 2,
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
//	sub                     // since 2 elements were already computed.
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1
//	pop argument 0          
@0
D=A
@ARG
M=M+D
@SP
M=M-1
@SP
A=M
D=M
@ARG
A=M
M=D
@0
D=A
@ARG
M=M-D
//label LOOP
($LOOP)
//	push argument 0
@0
D=A
@ARG
D=D+M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
//	if-goto COMPUTE_ELEMENT // if n > 0, goto COMPUTE_ELEMENT
@SP
M=M-1
@SP
A=M
D=M

@$COMPUTE_ELEMENT
D;JNE
//	goto END                // otherwise, goto END
@$END
0;JMP
//label COMPUTE_ELEMENT
($COMPUTE_ELEMENT)
//	push that 0
@0
D=A
@THAT
D=D+M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
//	push that 1
@1
D=A
@THAT
D=D+M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
//	add
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1
//	pop that 2
@2
D=A
@THAT
M=M+D
@SP
M=M-1
@SP
A=M
D=M
@THAT
A=M
M=D
@2
D=A
@THAT
M=M-D
//	push pointer 1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
//	push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
//	add
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1
//	pop pointer 1 
@SP
M=M-1
@SP
A=M
D=M
@THAT

M=D
//	push argument 0
@0
D=A
@ARG
D=D+M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
//	push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
//	sub
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1
//	pop argument 0          
@0
D=A
@ARG
M=M+D
@SP
M=M-1
@SP
A=M
D=M
@ARG
A=M
M=D
@0
D=A
@ARG
M=M-D
//	goto LOOP
@$LOOP
0;JMP
//label END
($END)
(END)
@END
0;JMP
