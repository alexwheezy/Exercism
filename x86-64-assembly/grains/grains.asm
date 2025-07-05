section .text

global square
square:
    xor    rax, rax
    lea    rcx, [rdi - 1]
    mov    rax, 1
    xor    rdx, rdx
    sal    rax, cl
    cmp    rcx, 64
    cmovnb rax, rdx
    ret

global total
total:
    mov  rax, 0xffffffffffffffff
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
