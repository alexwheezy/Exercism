section .text

global reverse
reverse:
    ; if string is empty, return
    cmp    byte [rdi], 0
    je     .exit
    lea    rbx, [rdi]
    mov    r12, 0
    xor    rax, rax

.pushloop:
    mov    al, [rbx + r12]
    cmp    byte al, 0
    je     .main
    push   rax
    inc    r12
    jmp   .pushloop

.main:
    mov    rcx, r12
    xor    r12, r12

.poploop:
    pop    rax
    mov    [rdi + r12], al
    inc    r12
    loop   .poploop

.exit:
    leave
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
