section .rodata
    ZERO_LENGTH equ 0
    SIDES       equ 3
    SIZE_OF     equ 8
    
section .text

init:
    push   rbp
    mov    rbp, rsp
    mov    rcx, SIDES

.check_input:
    mov    rax, [rbp + SIZE_OF * 4 * SIDES - 1]
    cmp    rax, ZERO_LENGTH
    je     error
    loop   .check_input

    movsd  xmm0, [rbp + SIZE_OF * 4] ; a
    movsd  xmm1, [rbp + SIZE_OF * 5] ; b
    movsd  xmm2, [rbp + SIZE_OF * 6] ; c

    movq   xmm3, xmm0
    addsd  xmm3, xmm1                ; a + b <= c
    comisd xmm3, xmm2
    jbe    error

    movq   xmm3, xmm1
    addsd  xmm3, xmm2                ; b + c <= a
    comisd xmm3, xmm0
    jbe    error

    movq   xmm3, xmm0
    addsd  xmm3, xmm2                ; a + c <= b
    comisd xmm3, xmm1
    jbe    error

    mov    rax, 1
    leave
    ret

error:
    mov    rax, 0
    leave
    ret

global is_equilateral
is_equilateral:
    push   rbp
    mov    rbp, rsp

    call   init
    cmp    rax, 1
    jne    error

    comisd xmm0, xmm1
    jne    error
    
    comisd xmm0, xmm2
    jne    error

    comisd xmm1, xmm2
    jne    error

    leave
    ret

global is_isosceles
is_isosceles:
    push   rbp
    mov    rbp, rsp

    call   init
    cmp    rax, 1
    jne    error

    comisd xmm0, xmm1
    je     .done

    comisd xmm0, xmm2
    je     .done

    comisd xmm1, xmm2
    jne    error

.done:
    leave
    ret

global is_scalene
is_scalene:
    push   rbp
    mov    rbp, rsp

    call   init
    cmp    rax, 1
    jne    error

    comisd xmm0, xmm1
    je     error
    
    comisd xmm0, xmm2
    je     error

    comisd xmm1, xmm2
    je     error

    leave
    ret
    
%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
