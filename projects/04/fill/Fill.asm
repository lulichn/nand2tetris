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

    // そうでない場合 0 (000000000000) で埋める
    @value
    M=0

    // FILL へジャンプ
    @FILL
    0;JMP

(ON)
    // -1 (111111111111) で埋める
    @value
    M=-1

(FILL)
    // カウンタ (描写位置) を初期化
    @counter
    M=0

    (FILLLOOP)
        @counter
        D=M
        
        // アドレスを temp に確保
        @SCREEN
        A=A+D
        D=A
        @temp
        M=D

        // 描画する値
        @value
        D=M

        // temp に描画
        @temp
        A=M
        M=D

        // カウンタをインクリメント
        @counter
        D=M+1
        M=D

        // スクリーンの最後まで
        // 8192 = (512 / 16) * 256, 512:横, 256:縦
        @8192
        D=A-D

        @FILLLOOP
        D;JGT
    
@LOOP
0;JMP
