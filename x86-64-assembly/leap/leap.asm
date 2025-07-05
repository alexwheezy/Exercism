section .text
global leap_year
leap_year:
    xor    rdx, rdx
    lea    rax, [rdi]
    mov    rbx, 4
    div    rbx
    cmp    rdx, 0
    jnz    .false

.next:
    lea    rax, [rdi]
    mov    rbx, 100
    div    rbx
    cmp    rdx, 0
    jnz    .true
    lea    rax, [rdi]
    mov    rbx, 400
    div    rbx
    cmp    rdx, 0
    jz     .true

.false:
    xor    rax, rax
    ret

.true:
    mov    rax, 1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
