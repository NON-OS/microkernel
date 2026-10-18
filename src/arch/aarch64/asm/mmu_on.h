/* Turning a CPU's MMU on with the boot CPU's translation, from a stub that
 * firmware entered with translation and the data cache off: a secondary's
 * first start (start.S) and a core's return from a powerdown idle state
 * (suspend.S).
 *
 * With the MMU off this CPU's stores bypass the caches the others read, and
 * its atomics act on Device memory (mmu/secondary.rs). AARCH64_MMU_CONTEXT
 * was written to memory before any CPU could read it, so it reads right with
 * the cache off. Stale instruction lines go before the I-cache comes on.
 *
 * Uses x1-x7 and leaves x0 alone. Leaves in x7 the kernel's distance above
 * its load address: zero one to one, KERNEL_SPACE_START once it runs high. */
.macro MMU_ON_FROM_CONTEXT
    adrp    x1, AARCH64_MMU_CONTEXT
    add     x1, x1, :lo12:AARCH64_MMU_CONTEXT
    ldp     x2, x3, [x1]
    ldp     x4, x5, [x1, #16]
    ldr     x6, [x1, #32]
    ldr     x7, [x1, #40]
    msr     mair_el1, x2
    msr     tcr_el1, x3
    msr     ttbr0_el1, x4
    msr     ttbr1_el1, x5
    isb
    tlbi    vmalle1
    ic      iallu
    dsb     nsh
    isb
    mrs     x2, sctlr_el1
    orr     x2, x2, x6
    msr     sctlr_el1, x2
    isb
.endm
