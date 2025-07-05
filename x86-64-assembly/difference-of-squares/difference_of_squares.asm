section .text

global square_of_sum
square_of_sum:
    mov    rax, rdi
    inc    rax
    mul    rdi
    shr    rax, 1
    mul    rax
    ret

global sum_of_squares
sum_of_squares:
    mov    rax, rdi
    inc    rax
    mul    rdi
    mov    rbx, rax
    mov    rax, rdi
    shl    rax, 1
    inc    rax
    mul    rbx
    mov    rbx, 0xaaaaaaab
    imul   rax, rbx
    shr    rax, 34
    ret

global difference_of_squares
difference_of_squares:
    call   sum_of_squares
    mov    rbx, rax
    call   square_of_sum
    sub    rax, rbx
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
