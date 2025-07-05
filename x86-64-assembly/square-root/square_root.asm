default rel

section .data
    eps      dq 1e-9
    half     dq 0.5
    init     dq 1.0
align 16
    abs_mask dq 0x7fffffffffffffff, 0 

section .text

global square_root
square_root:
    push      rbp
    mov       rbp, rsp

    cvtsi2sd  xmm0, rdi        ; xmm0 = (double)radicand
    movsd     xmm1, [init]     ; x = 1.0
    movsd     xmm2, [eps]      ; eps = 1e-9
    movsd     xmm3, [half]     ; 0.5
    
.loop:
    movsd     xmm4, xmm1       ; xmm4 = x
    mulsd     xmm4, xmm4       ; xmm4 = x * x
    subsd     xmm4, xmm0       ; xmm4 = x * x - radicand
    
    andpd     xmm4, [abs_mask] ; abs(x * x - radicand)
    
    comisd    xmm4, xmm2
    jbe       .done
    
    movsd     xmm4, xmm0       ; xmm4 = radicand
    divsd     xmm4, xmm1       ; xmm4 = radicand / x
    addsd     xmm4, xmm1       ; xmm4 = x + radicand / x
    mulsd     xmm4, xmm3       ; xmm4 = (x + radicand / x) * 0.5
    movsd     xmm1, xmm4       ; x = new value
    jmp .loop

.done:
    cvttsd2si rax, xmm1        ; rax = (usize)x

    pop rbp
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
