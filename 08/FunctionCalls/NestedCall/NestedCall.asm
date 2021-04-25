@256
D=A
@SP
M=D
@0
D=A
@LCL
M=D
@ARG
M=D
@THIS
M=D
@THAT
M=D
@Bootstrap.Sys.init$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
D=D-1
D=D-1
D=D-1
D=D-1
D=D-1
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.init
0;JMP
(Bootstrap.Sys.init$ret.0)

//function Sys.init (0)
(Sys.init)

//push constant 4000
@4000
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D

//push constant 5000
@5000
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop pointer 1
@SP
M=M-1
A=M
D=M
@THAT
M=D

//call Sys.main (0)
@Sys.Sys.main$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
D=D-1
D=D-1
D=D-1
D=D-1
D=D-1
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.main
0;JMP
(Sys.Sys.main$ret.0)

//pop temp 1
@SP
M=M-1
A=M
D=M
@R6
M=D

//label LOOP
(Sys.init$LOOP)

//goto LOOP
@Sys.init$LOOP
0;JMP

//function Sys.main (5)
(Sys.main)
@0
D=A
@LCL
M=M+D
@0
D=A
@LCL
A=M
M=D
@0
D=A
@LCL
M=M-D
@SP
M=M+1
@1
D=A
@LCL
M=M+D
@0
D=A
@LCL
A=M
M=D
@1
D=A
@LCL
M=M-D
@SP
M=M+1
@2
D=A
@LCL
M=M+D
@0
D=A
@LCL
A=M
M=D
@2
D=A
@LCL
M=M-D
@SP
M=M+1
@3
D=A
@LCL
M=M+D
@0
D=A
@LCL
A=M
M=D
@3
D=A
@LCL
M=M-D
@SP
M=M+1
@4
D=A
@LCL
M=M+D
@0
D=A
@LCL
A=M
M=D
@4
D=A
@LCL
M=M-D
@SP
M=M+1

//push constant 4001
@4001
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D

//push constant 5001
@5001
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop pointer 1
@SP
M=M-1
A=M
D=M
@THAT
M=D

//push constant 200
@200
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop local 1
@1
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
@1
D=A
@LCL
M=M-D

//push constant 40
@40
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop local 2
@2
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
@2
D=A
@LCL
M=M-D

//push constant 6
@6
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop local 3
@3
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
@3
D=A
@LCL
M=M-D

//push constant 123
@123
D=A
@SP
A=M
M=D
@SP
M=M+1

//call Sys.add12 (1)
@Sys.Sys.add12$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
D=D-1
D=D-1
D=D-1
D=D-1
D=D-1
D=D-1
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.add12
0;JMP
(Sys.Sys.add12$ret.0)

//pop temp 0
@SP
M=M-1
A=M
D=M
@R5
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

//push local 1
@1
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
@1
D=A
@LCL
M=M-D

//push local 2
@2
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
@2
D=A
@LCL
M=M-D

//push local 3
@3
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
@3
D=A
@LCL
M=M-D

//push local 4
@4
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
@4
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

//return
@LCL
D=M
@R13
M=D
@LCL
D=M
D=D-1
D=D-1
D=D-1
D=D-1
D=D-1
A=D
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
D=D+1
@SP
M=D
@R13
M=M-1
A=M
D=M
@THAT
M=D
@R13
M=M-1
A=M
D=M
@THIS
M=D
@R13
M=M-1
A=M
D=M
@ARG
M=D
@R13
M=M-1
A=M
D=M
@LCL
M=D
@R14
D=M
A=D
0;JMP

//function Sys.add12 (0)
(Sys.add12)

//push constant 4002
@4002
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D

//push constant 5002
@5002
D=A
@SP
A=M
M=D
@SP
M=M+1

//pop pointer 1
@SP
M=M-1
A=M
D=M
@THAT
M=D

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

//push constant 12
@12
D=A
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

//return
@LCL
D=M
@R13
M=D
@LCL
D=M
D=D-1
D=D-1
D=D-1
D=D-1
D=D-1
A=D
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
D=D+1
@SP
M=D
@R13
M=M-1
A=M
D=M
@THAT
M=D
@R13
M=M-1
A=M
D=M
@THIS
M=D
@R13
M=M-1
A=M
D=M
@ARG
M=D
@R13
M=M-1
A=M
D=M
@LCL
M=D
@R14
D=M
A=D
0;JMP

