/* NONOS Operating System (AGPL-3.0-or-later)
 *   walk_harness(fixture: *const u64, log: *mut u64, cap: u64,
 *                overflow: *mut u64) -> u64
 * Runs the loader's clean_all_by_set_way over the fixture and returns how
 * many operands it logged. Saves every callee-saved register it uses. */
.global walk_harness
walk_harness:
    stp     x19, x20, [sp, #-64]!
    stp     x21, x22, [sp, #16]
    stp     x23, x24, [sp, #32]
    str     x25, [sp, #48]
    mov     x19, x0
    mov     x20, x1
    mov     x21, x1
    add     x22, x1, x2, lsl #3
    mov     x24, #0
    mov     x25, x3
    clean_all_by_set_way
    str     x24, [x25]
    sub     x0, x20, x21
    lsr     x0, x0, #3
    ldr     x25, [sp, #48]
    ldp     x23, x24, [sp, #32]
    ldp     x21, x22, [sp, #16]
    ldp     x19, x20, [sp], #64
    ret
