default rel
section .rodata
    prefix db "One for "
    prefix_len equ $ - prefix
    suffix db ", one for me.", 0
    default_str db "you"

OFFSET equ 8

section .text
global two_fer
two_fer:
    ;Counting the number of characters in the source
    test rdi, rdi
    lea rdx, [default_str]
    cmovz rdi, rdx

    xor rax, rax
    xor r9, r9
.count:
    mov al, byte [rdi + r9]
    inc r9
    cmp al, 0
    jne .count
    dec r9

    ;Adding the prefix reference to the buffer
    lea r10, [prefix]
    mov r11, [r10]
    mov [rsi], r11

    ;Adding a reference to the argument to the buffer
    mov rbx, rsi
    mov rsi, rdi
    lea rdi, [rbx + prefix_len]
    mov rcx, r9
    rep movsb
    mov rsi, rbx ;Restoring the buffer value

    ;And add a suffix after
    lea r10, [suffix]
    mov r11, [r10]
    lea r12, [rsi + prefix_len + r9]
    mov [r12], r11

    lea r10, [suffix + OFFSET]
    mov r11, [r10]
    lea r12, [rsi + prefix_len + r9 + OFFSET]
    mov [r12], r11
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
