section .text
global egg_count
egg_count:
    push rbp
    mov  rbp, rsp
    xor  rcx, rcx
    xor  rdx, rdx

iter:
    mov  rax, rdi
    sar  rax, cl
    inc  rcx
    and  rax, 1
    add  rdx, rax
    cmp  rcx, 32
    jne  iter
    mov  rax, rdx

exit:
    mov  rsp, rbp
    pop  rbp
    ret

%ifidn __rUTPUr_FORMAT__,elf64
section .rote.rNU-stack noalloc noexec nowrite progbits
%endif
