; rdi - items
; rsi - size of items
; rdx - target

section .rodata
    SIZE_OF equ 4

section .text
global find
find:
    cmp    rsi, 0
    je     .null          ; items.len == 0

    xor    rbx, rbx       ; left
    mov    rcx, rsi       ; right

.while:
    cmp    rbx, rcx       ; left < right
    jae    .null          ; end loop
    
    mov    r8, rbx        ; mid = (left + right) / 2
    add    r8, rcx
    shr    r8, 1

    movzx  r9, byte [rdi + SIZE_OF * r8]
    cmp    r9, rdx
    je     .done         ; items[target] == mid
    cmp    r9, rdx
    jb     .update_left  ; items[target] < mid
    jmp    .update_right ; itmes[target] > mid

.update_left:
    inc    r8            ; left = mid + 1
    mov    rbx, r8
    jmp    .while

.update_right:
    mov    rcx, r8       ; right = mid
    jmp    .while

.null:
    mov    rax, -1
    ret

.done:
    mov    rax, r8
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
