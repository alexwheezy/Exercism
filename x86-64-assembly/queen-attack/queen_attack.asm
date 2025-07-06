%macro check_input 1
    cmp    %1, 0
    jb     error

    cmp    %1, MAX_HEIGHT 
    jae    error
%endmacro

section .rodata
    MAX_WIDTH  equ 8
    MAX_HEIGHT equ MAX_WIDTH

section .text

; Parameters:
; rdi - row
; rsi - column

global can_create
can_create:
    push   rbp
    mov    rbp, rsp

    check_input rdi
    check_input rsi

    mov    rax, 1
    leave
    ret

; Parameters:
; rdi - white row
; rsi - white column
; rdx - black row
; rcx - black column

global can_attack
can_attack:
    push   rbp
    mov    rbp, rsp

    ; Store parameters function
    push   rdi
    push   rsi
    push   rdx
    push   rcx
       
    call   can_create
    cmp    rax, 0
    je     error

    mov    rdi, [rsp + 24]
    mov    rsi, [rsp + 16]

    call   can_create
    cmp    rax, 0
    je     error

    ; Restore parameters function
    mov    rdi, [rsp + 24]
    mov    rsi, [rsp + 16]
    mov    rdx, [rsp + 8]
    mov    rcx, [rsp]

    ; white_row == black_row
    cmp    rdi, rdx
    je     .done

    ; white_col == black_col
    cmp    rsi, rcx
    je     .done

    ; white_row - black_row == white_col - black_col
    sub    rdi, rdx
    mov    rax, rdi
    sub    rsi, rcx
    cmp    rax, rsi
    je     .done
    
    mov    rdi, [rsp + 24]
    mov    rsi, [rsp + 16]
    mov    rdx, [rsp + 8]
    mov    rcx, [rsp]

    ; white_row + white_col == black_row + black_col
    add    rdi, rsi
    mov    rax, rdi
    add    rdx, rcx
    cmp    rax, rdx
    je     .done
    jmp    error

.done:
    mov    rax, 1
    add    rsp, 32
    leave
    ret

error:
    xor rax, rax
    leave
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
