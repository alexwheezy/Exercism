default rel

section .data
    SENTINEL       equ 0
    UPPER_MASK     equ 0x5f
    BEGIN_ALPHABET equ 0x41
    ALPHABET_LEN   equ 25
    alphabet times ALPHABET_LEN db 0

section .text

global is_pangram
is_pangram:
    xor    rax, rax
    lea    rsi, [rdi]
    lea    rbx, [alphabet]
    mov    rcx, ALPHABET_LEN

.init:
    mov    byte [rbx + rcx], 0
    loop   .init

    xor    rcx, rcx

.chars:
    movzx  rax, byte [rsi + rcx]
    cmp    rax, SENTINEL
    je     .done
    and    rax, UPPER_MASK
    sub    rax, BEGIN_ALPHABET
    cmp    rax, ALPHABET_LEN
    ja     .continue
    inc    byte [rbx + rax]

.continue:
    inc    rcx
    jmp    .chars
        
.done:
    mov    rcx, ALPHABET_LEN
    xor    rax, rax

.count:
    cmp    rcx, 0
    je     .end
    movzx  rdx, byte [rbx + rcx]
    cmp    rdx, 0
    jne    .update
    dec    rcx
    jmp    .count

.update:
    inc    rax
    dec    rcx
    jmp    .count

.end:
    cmp    rax, ALPHABET_LEN
    jne    .false
    ret

.false:
    xor    rax, rax
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
