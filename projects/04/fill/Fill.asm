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

(LOOP)
    // キーボード入力
    @KBD
    D=M

    // 押下されている場合 ON へジャンプ
    @ON
    D;JGT

    // 押下されていない場合 R0 に 0(0b000000000000) をセット
    @R0
    M=0

    // FILL へジャンプ
    @FILL
    0;JMP

(ON)
    // R0 に -1(0b111111111111) をセット
    @R0
    M=-1

(FILL)
    // 描写開始位置 (スクリーン右下) を初期化し R1 へ
    @SCREEN
    D=A
    @8191    // 8192 = (512 / 16) * 256 - 1
    D=D+A
    @R1
    M=D

    (FILLLOOP)
        // 描画する値
        @R0
        D=M

        // R1 に描画
        @R1
        A=M  // Aレジスタに描画位置のアドレスをセット
        M=D

        // 描画位置のアドレスをデクリメント
        @R1
        MD=M-1

        // スクリーン左上に到達するまで FILLLOOP へジャンプ
        @SCREEN
        D=D-A
        @FILLLOOP
        D;JGE
    
@LOOP
0;JMP
