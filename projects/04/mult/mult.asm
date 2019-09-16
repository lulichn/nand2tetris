// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)


// 結果を格納する R2 を 0 で初期化
@R2
M=0

// R1 が 0 以下の場合は END へジャンプ
@R1
D=M
@END
D;JLE

(LOOP)  // R2 += R0
    // R2 += R0
    @R0
    D=M
    @R2
    M=M+D

    // R1 をデクリメント
    @R1
    MD=M-1

    // R1 > 0 の時, Loop にジャンプ
    @LOOP
    D;JGT

(END)
    // 無限ループは Hack プログラムを "終了させる"
    @END
    0;JMP