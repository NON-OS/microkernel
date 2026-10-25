// NONOS Operating System (AGPL-3.0-or-later)
// A static PIE with pointers of every kind the kernel's data holds: to code,
// to read-only data, into the middle of an object, to itself, past the end of
// an object, and one plain number that must not move.
    .text
    .globl _start
_start:
    b       .
func:
    ret

    .section .rodata
msg:
    .ascii  "relocated"

    .data
    .balign 8
ptrs:
    .quad   func
    .quad   msg
    .quad   msg + 3
    .quad   ptrs
    .quad   tail + 8
    .quad   0x1234
tail:
    .quad   0
