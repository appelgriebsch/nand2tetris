// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// calculates the multiply of R0 and R1 by iteratively sum up
// get the MIN value of R0, R1 first for the no. of iterations
// init the SUM value with the other entry, and sum it up for n iterations

// PSEUDO Code ------------
//
// R2 = 0
//
// if R0 > R1 then
// 		val = R0
//    n = R1
// else
// 		val = R1
// 		n = R0
// 
// for (i=0; i<n; i++)
// 		R2 += val
// -----------------------

 	// initialize R2 to 0
 	@R2
 	M=0

 	// initialize loop counter to 0
 	@i
 	M=0

	// if R0 > R1...
 	@R0
	D=M
	@R1
	D=D-M
	// ... jump to label R0_GT_R1
	@R0_GT_R1
	D;JGT
	// ... otherwise jump to label R1_GT_R0
	@R1_GT_R0
	0;JMP

(R0_GT_R1)
	// set variables val and n to respective values...
	@R0
	D=M
	// ... val = R0
	@val
	M=D
	@R1
	D=M
	// ... n = R1
	@n
	M=D
	@LOOP
	0;JMP

(R1_GT_R0)
	// set variables val and n to respective values...
	@R1
	D=M
	// ... val = R1
	@val
	M=D
	@R0
	D=M
	// ... n = R0
	@n
	M=D
	@LOOP
	0;JMP

(LOOP)
	// if (i - n) = 0 ...
	@i
	D=M
	@n
	D=D-M
	// ... jump to label END
	@END
	D;JEQ

	// loop for n times and add val to sum
	@R2
	D=M
	@val
	D=D+M
	@R2
	M=D

	// increase loop counter i by 1 and continue
	@i
	D=M+1
	M=D
	@LOOP
	0;JMP

(END)
	@END
	0;JMP