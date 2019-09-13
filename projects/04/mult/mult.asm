// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)


// 結果を格納する R2 に 0 を代入
@R2
M=0

// R1 が 0 より大きい場合は LOOP へジャンプ
@R1
D=M
@LOOP
D;JGT

// ここに到達した(R1 が 0) の場合は END へジャンプ
@END
0;JMP

(LOOP)
    // R2 += R0

    // D = R2
    @R2
    D=M
    // D += R0
    @R0
    D=D+M
    // R2 = D
    @R2
    M=D

    // R1 をデクリメント
    // D = R1
    @R1
    D=M
    // D -= 1
    D=D-1
    // R1 = D
    M=D

    // R1 > 0 の時, Loop にジャンプ
    @LOOP
    D;JGT
(END)
    // 無限ループは Hack プログラムを "終了させる"
    @END
    0;JMP