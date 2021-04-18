//push constant 10
@10
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

//push constant 21
@21
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 22
@22
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop argument 2
@2
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
@2
D=A
@ARG
M=M-D

//pop argument 1
@1
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
@1
D=A
@ARG
M=M-D

//push constant 36
@36
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop this 6
@6
D=A
@THIS
M=M+D
@SP
M=M-1
A=M
D=M
@THIS
A=M
M=D
@6
D=A
@THIS
M=M-D

//push constant 42
@42
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 45
@45
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop that 5
@5
D=A
@THAT
M=M+D
@SP
M=M-1
A=M
D=M
@THAT
A=M
M=D
@5
D=A
@THAT
M=M-D

//pop that 2
@2
D=A
@THAT
M=M+D
@SP
M=M-1
A=M
D=M
@THAT
A=M
M=D
@2
D=A
@THAT
M=M-D

//push constant 510
@510
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop temp 6
@SP
M=M-1
A=M
D=M
@R11
M=D

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

//push that 5
@5
D=A
@THAT
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@5
D=A
@THAT
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

//push argument 1
@1
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
@1
D=A
@ARG
M=M-D

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

//push this 6
@6
D=A
@THIS
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@6
D=A
@THIS
M=M-D

//push this 6
@6
D=A
@THIS
M=M+D
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
@6
D=A
@THIS
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

//push temp 6
@R11
D=M
@SP
A=M
M=D
@SP
M=M+1

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

