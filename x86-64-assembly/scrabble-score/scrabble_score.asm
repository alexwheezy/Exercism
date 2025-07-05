default rel

section .data
              ;  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q   R  S  T  U  V  W  X  Y  Z
    alphabet: db 1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10

SENTINEL       equ 0
UPPER_MASK     equ 0x5f
BEGIN_ALPHABET equ 0x41

section .text
global score
score:
    xor    rax, rax
    xor    r8, r8
    xor    rcx, rcx
    xor    rsi, rsi

    lea    rbx, [alphabet]

.loop:
    movzx  rax, byte [rdi + rcx]
    cmp    rax, SENTINEL
    je     .done
    and    rax, UPPER_MASK
    sub    rax, BEGIN_ALPHABET
    movzx  rsi, byte [rbx + rax]
    add    r8, rsi
    inc    rcx
    jmp    .loop

.done:
    mov    rax, r8
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
