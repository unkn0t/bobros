global long_mode_entry
extern _start

section .text
bits 64
long_mode_entry:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; jump to rust _start (src/lib.rs)
    jmp _start
