//push constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop local 0
@0
D=A
@LCL
M=M+D
@SP
M=M-1
A=M
D=M
@LCL
A=M
M=D
@0
D=A
@LCL
M=M-D

//label LOOP_START
(LOOP_START)

//push argument 0
@0
D=A
@ARG
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@ARG
M=M-D

//push local 0
@0
D=A
@LCL
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@LCL
M=M-D

//add
@SP
M=M-1
A=M
D=M
@SP
A=M
@SP
M=M-1
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1

//pop local 0
@0
D=A
@LCL
M=M+D
@SP
M=M-1
A=M
D=M
@LCL
A=M
M=D
@0
D=A
@LCL
M=M-D

//push argument 0
@0
D=A
@ARG
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@ARG
M=M-D

//push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

//sub
@SP
M=M-1
A=M
D=M
@SP
A=M
@SP
M=M-1
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1

//pop argument 0
@0
D=A
@ARG
M=M+D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@0
D=A
@ARG
M=M-D

//push argument 0
@0
D=A
@ARG
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@ARG
M=M-D

//if-goto LOOP_START
@SP
M=M-1
A=M
D=M
@LOOP_START
D;JNE

//push local 0
@0
D=A
@LCL
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@LCL
M=M-D

