default rel

section .data
    BUF_LEN        equ 19
    INPUT_LEN      equ 4
    SIZE_OF        equ 8
    SENTINEL       equ 0
    BEGIN_ALPHABET equ 0x41
    END_ALPHABET   equ 0x54

section .data
    buffer times BUF_LEN db 0

section .text

global nucleotide_counts
nucleotide_counts:
    xor    rcx, rcx
    lea    rbx, [buffer]

.loop:
    movzx  rax, byte [rdi + rcx]
    cmp    rax, END_ALPHABET
    ja     .err
    cmp    rax, SENTINEL
    je     .set
    sub    rax, BEGIN_ALPHABET
    inc    byte [rbx + rax]
    inc    rcx
    jmp    .loop

.set:
    movzx  rax, byte [rbx]           ; A
    mov    [rsi], rax
    movzx  rax, byte [rbx + 2]       ; C
    mov    [rsi + SIZE_OF * 1], rax
    movzx  rax, byte [rbx + 6]       ; G
    mov    [rsi + SIZE_OF * 2], rax
    movzx  rax, byte [rbx + 19]      ; T
    mov    [rsi + SIZE_OF * 3], rax
    jmp    .done

.err:
    ; invalid input: if a char was in out of range
    mov    rdi, rsi
    mov    al, 1
    neg    rax
    mov    rcx, INPUT_LEN
    rep    stosq

.done:
    ; clean the buffer
    mov    rdi, rbx
    mov    al, 0
    mov    rcx, BUF_LEN
    rep    stosb
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
