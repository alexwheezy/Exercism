default rel

section .data
    year    dd 31557600.0                 ; Earth year in seconds

    MERCURY dd 0.2408467
    VENUS   dd 0.61519726
    EARTH   dd 1.0
    MARS    dd 1.8808158
    JUPITER dd 11.862615
    SATURN  dd 29.447498
    URANUS  dd 84.016846
    NEPTUNE dd 164.79132

    planet: dq MERCURY, VENUS, EARTH, MARS, JUPITER, SATURN, URANUS, NEPTUNE

SIZE_OF equ 4

section .text
global age
age:
    push     rbp
    mov      rbp, rsp

    mov      rcx, rdi                     ; variant a planet
    cvtsi2ss xmm1, rsi                    ; secs = (double)seconds
    
    mov      rax,  qword [planet]
    movq     xmm0, [rax + SIZE_OF * rcx]  ; orbital period in Earth years
    mulss    xmm0, [year]                 ; period * Earth year
    divss    xmm1, xmm0                   ; secs / (period * Earth year)
    movq     xmm0, xmm1
    
    leave
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
