default rel

section .rodata
    black    db "black", 0
    brown    db "brown", 0
    red      db "red", 0
    orange   db "orange", 0
    yellow   db "yellow", 0
    green    db "green", 0
    blue     db "blue", 0
    violet   db "violet", 0
    grey     db "grey", 0
    white    db "white", 0

section .data
    colors: dq black, brown, red, orange, yellow, green, blue, violet, grey, white

OFFSET equ 0x8
MULT   equ 0xa

section .text
global value
value:
    call   .by_color
    mov    r10, MULT
    mul    r10
    mov    rdi, rsi
    mov    r12, rax
    call   .by_color
    add    rax, r12
    ret

.by_color:
    push   rbp
    mov    rbp, rsp

    mov    qword [rbp - 16], rsi
    mov    qword [rbp - 8], rdi

    ; Counting the number of characters in the source
    xor    rax, rax
    xor    r9, r9
    mov    qword rcx, [rbp - 8]

.count:
    cmp    byte [rcx + r9], 0
    je     .main
    inc    r9b
    jne   .count

.main:
    test   r9b, r9b
    jz     .exit
    xor    r11, r11

.color_cmp:
    mov    rdi, [rbp - 8]
    mov    rbx, OFFSET
    mov    al, r11b
    mul    rbx
    lea    rdx, [colors]
    mov    rsi, [rdx + rax]
    mov    rcx, r9
    inc    r11b
    repe   cmpsb
    jne    .color_cmp

    dec    r11b
    xor    rax, rax
    mov    al, r11b

.exit:
    mov    rsi, [rbp - 16]
    mov    rdi, [rbp - 8]
    leave
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
