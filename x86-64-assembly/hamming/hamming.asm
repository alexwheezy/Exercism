section .text
global distance
distance:
    lea    r10, [rsi] ; second arg
    call   .length
    mov    r8, rax
    lea    r10, [rdi] ; first arg
    call   .length
    mov    r9, rax
    cmp    r8, r9
    jne    .err
    xor    rax, rax
    xor    r11, r11

.diff:
    cmp    byte [rsi], 0
    je     .diff_end
    mov    r11b, byte [rsi]
    cmp    r11b, byte [rdi]
    jne    .diff_inc
    jmp    .diff_continue

.diff_inc:
    inc    rax

.diff_continue:
    inc    rsi
    inc    rdi
    jmp    .diff

.diff_end:
    ret

.length:
    push   rbp
    mov    rbp, rsp
    xor    rax, rax

.length_iterate:
    cmp    byte [r10], 0
    je     .length_end
    inc    rax
    inc    r10
    jmp    .length_iterate

.length_end:
    leave
    ret

.err:
    mov rax, -1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
