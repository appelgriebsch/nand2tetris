//push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

//eq
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
@EQ_TRUE_2
D;JEQ
D=0
@EQ_END_2
0;JMP
(EQ_TRUE_2)
D=-1
(EQ_END_2)
@SP
A=M
M=D
@SP
M=M+1

//push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

//eq
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
@EQ_TRUE_5
D;JEQ
D=0
@EQ_END_5
0;JMP
(EQ_TRUE_5)
D=-1
(EQ_END_5)
@SP
A=M
M=D
@SP
M=M+1

//push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

//eq
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
@EQ_TRUE_8
D;JEQ
D=0
@EQ_END_8
0;JMP
(EQ_TRUE_8)
D=-1
(EQ_END_8)
@SP
A=M
M=D
@SP
M=M+1

//push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

//lt
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
@LT_TRUE_11
D;JLT
D=0
@LT_END_11
0;JMP
(LT_TRUE_11)
D=-1
(LT_END_11)
@SP
A=M
M=D
@SP
M=M+1

//push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

//lt
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
@LT_TRUE_14
D;JLT
D=0
@LT_END_14
0;JMP
(LT_TRUE_14)
D=-1
(LT_END_14)
@SP
A=M
M=D
@SP
M=M+1

//push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

//lt
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
@LT_TRUE_17
D;JLT
D=0
@LT_END_17
0;JMP
(LT_TRUE_17)
D=-1
(LT_END_17)
@SP
A=M
M=D
@SP
M=M+1

//push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

//gt
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
@GT_TRUE_20
D;JGT
D=0
@GT_END_20
0;JMP
(GT_TRUE_20)
D=-1
(GT_END_20)
@SP
A=M
M=D
@SP
M=M+1

//push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

//gt
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
@GT_TRUE_23
D;JGT
D=0
@GT_END_23
0;JMP
(GT_TRUE_23)
D=-1
(GT_END_23)
@SP
A=M
M=D
@SP
M=M+1

//push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

//gt
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
@GT_TRUE_26
D;JGT
D=0
@GT_END_26
0;JMP
(GT_TRUE_26)
D=-1
(GT_END_26)
@SP
A=M
M=D
@SP
M=M+1

//push constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 53
@53
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

//push constant 112
@112
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

//neg
@SP
A=M
@SP
M=M-1
A=M
D=-M
@SP
A=M
M=D
@SP
M=M+1

//and
@SP
M=M-1
A=M
D=M
@SP
A=M
@SP
M=M-1
A=M
D=D&M
@SP
A=M
M=D
@SP
M=M+1

//push constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1

//or
@SP
M=M-1
A=M
D=M
@SP
A=M
@SP
M=M-1
A=M
D=D|M
@SP
A=M
M=D
@SP
M=M+1

//not
@SP
A=M
@SP
M=M-1
A=M
D=!M
@SP
A=M
M=D
@SP
M=M+1

