section .text
global steps
steps:
    test rdi, rdi
    jle err

    mov rsi, 3
    mov rbx, rdi
    mov rax, rbx
    mov rcx, 0 ;step

looping:
    cmp rbx, 1
    jg body
    mov rax, rcx
    leave
    ret

body:
    inc rcx
    and rbx, 1
    test rbx, rbx
    jle divided
    mul rsi
    inc rax
    mov rbx, rax
    jmp looping

divided:
    sar rax, 1
    mov rbx, rax
    jmp looping

err:
    mov rax, -1
    leave
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
