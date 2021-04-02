// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// start an event loop looking for input from keyboard
// set a flag denoting clearing or setting of pixels on display
// iterate over screen rows AND columns, setting the flag on each pixel
// 
// R0 = no. of rows in display
// R1 = no. of cols per row
// R2 = 0 (clear) | -1 (set) pixel

// PSEUDO Code --------------
// for(;;)
//		key = KBD
//		if (key == 0)
//				R2 = 0
// 		else
//			  R2 = -1
//
//		for(row=0; row < R0; ++i)
//			for(col=0; col < R1; ++j)
//					SCREEN[row+col]=R2

	// store no. of rows in R0
	@256
	D=A
	@R0
	M=D

	// store no. of cols per row in R1
	@32
	D=A
	@R1
	M=D

// start event loop
(EVENT_LOOP)
	// get input from keyboard (if any)
	@KBD
	D=M
	// if input == 0 (no key pressed) jump to label CLEAR_SCREEN
	@CLEAR_SCREEN
	D;JEQ
	// ... otherwise jump to label FILL_SCREEN
	@FILL_SCREEN
	0;JMP

(CLEAR_SCREEN)
	// set R2 to 0 (to flag the clear screen operation)
	@R2
	M=0
	// init row counter to 0
	@row
	M=0
	// init the row offset address to 0
	@offset
	M=0
	// ... and jump to label UPDATE_SCREEN
	@UPDATE_SCREEN
	0;JMP

(FILL_SCREEN)
	// set R2 to -1 (to flag the fill screen operation)
	@R2
	M=-1
	// init row counter to 0
	@row
	M=0
	// init the row offset address to 0
	@offset
	M=0
	// ... and jump to label UPDATE_SCREEN
	@UPDATE_SCREEN
	0;JMP

// iterate over every row via @row variable
(UPDATE_SCREEN)
	// check row counter variable against R0
	@R0
	D=M
	@row
	D=D-M
	// if row > R0 jump back to EVENT_LOOP
	@EVENT_LOOP
	D;JEQ

	// init column counter to 0 on start of each row
	@col
	M=0

// update a row on screen based on @offset variable
(UPDATE_ROW)
	// check the colum counter variable against R1
	@R1
	D=M
	@col
	D=D-M
	// if col > R1 jump to NEXT_ROW
	@NEXT_ROW
	D;JEQ
	
	// calculate the address of the memory address
	// based on the SCREEN base address
	@SCREEN
	D=A
	// and the current row offset (row * 32)
	@offset
	D=D+M
	// and the current column in the row
	@col
	D=D+M
	// store the address
	@address
	M=D
	// get the value to put in RAM (clear or fill) from R2
	@R2
	D=M
	// set the value of R2 to the memory address calculated before
	@address
	A=M
	M=D

	// increase the column counter by 1
	@col
	D=M+1
	M=D
	// and jump back to beginning of UPDATE_ROW
	@UPDATE_ROW
	0;JMP

// calculates the @row and @offset variable for the next row
// and jumps back to the beginning of UPDATE_SCREEN
(NEXT_ROW)
	@row
	D=M+1
	M=D
	@32
	D=A
	@offset
	D=D+M
	M=D
	@UPDATE_SCREEN
	0;JMP
