*=$1000

lda #0
clc
loop
    adc #8
    cmp #24
    bne loop
ldx #20
