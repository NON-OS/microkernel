/* The boot CPU's move from where it was loaded, translated one to one
 * through TTBR0_EL1, to the image's high alias, which TTBR1_EL1 translates
 * and a user's TTBR0 never can (boot/high.rs).
 *
 *   x0 = the BootInfo, as the caller reaches it
 *   x1 = the alias's distance, KERNEL_SPACE_START
 *
 * The MMU is on and both mappings are live, so the rewritten pointers are
 * good from the moment they are written, whichever address runs: nothing
 * reads one that cannot be reached. The rewrite is refused whole or done
 * whole (boot/relocate/rebase.rs). Refused, this returns its code and the
 * kernel carries on where it is. Done, it never returns: the stack and the
 * BootInfo on it move by the same distance, the frame chain stops here,
 * and aarch64_kernel_high runs from the alias.
 *
 * The branch is BR through x16, the one register form `bti c` admits
 * (Arm ARM, DDI 0487, FEAT_BTI, PSTATE.BTYPE). */

.section .text.aarch64_go_high, "ax", %progbits
.balign 16
.globl aarch64_go_high
.type  aarch64_go_high, %function

aarch64_go_high:
    hint    #34                         /* BTI c */
    stp     x29, x30, [sp, #-32]!
    mov     x29, sp
    stp     x19, x20, [sp, #16]
    mov     x19, x0
    mov     x20, x1

    adrp    x0, __rela_start
    add     x0, x0, :lo12:__rela_start
    adrp    x1, __rela_end
    add     x1, x1, :lo12:__rela_end
    movz    x2, #:abs_g3:__kernel_link_base
    movk    x2, #:abs_g2_nc:__kernel_link_base
    movk    x2, #:abs_g1_nc:__kernel_link_base
    movk    x2, #:abs_g0_nc:__kernel_link_base
    adrp    x3, __kernel_image_start
    add     x3, x3, :lo12:__kernel_image_start
    movz    x4, #:abs_g3:__kernel_image_size
    movk    x4, #:abs_g2_nc:__kernel_image_size
    movk    x4, #:abs_g1_nc:__kernel_image_size
    movk    x4, #:abs_g0_nc:__kernel_image_size
    add     x5, x3, x20                 /* where the image runs next */
    adrp    x6, __kernel_rw_start
    add     x6, x6, :lo12:__kernel_rw_start
    sub     x6, x6, x3                  /* its writable part, from the start */
    bl      aarch64_rebase
    cbnz    x0, 1f

    /* The pointers are written; make them visible before the jump. */
    dsb     ish
    isb
    add     x0, x19, x20
    add     sp, sp, x20
    mov     x29, xzr
    mov     x30, xzr
    adrp    x16, aarch64_kernel_high
    add     x16, x16, :lo12:aarch64_kernel_high
    add     x16, x16, x20
    br      x16

1:  ldp     x19, x20, [sp, #16]
    ldp     x29, x30, [sp], #32
    ret

.size aarch64_go_high, . - aarch64_go_high
