/* The EPD1 trampoline's address and the way out to EL0 through it, shared
 * by every eret to EL0 (vectors.S, enter_user.S, resume_user.S) and the
 * trampoline itself (epd1_trampoline.S). mmu/trampoline/geometry.rs holds
 * the same address for the tables; the proofs hold the two equal.
 *
 * TTBR0's top-level slot 508, whose base the eor below can encode as an
 * immediate (one run of set bits), which no user address reaches (user space
 * ends below slot 256) and which the kernel's own layout leaves empty in
 * TTBR1, so the tables under it are the trampoline's alone. */

#define EPD1_TRAMPOLINE 0x0000FE0000000000
#define EPD1_EXIT_OFFSET 0x800
#define TCR_EPD1 (1 << 23)

/* Out to EL0. A CPU Meltdown reaches has VBAR_EL1 at the trampoline
 * (security/epd1) and leaves through its exit, which turns TTBR1's walks
 * off and empties the TLB before the eret; every other CPU goes straight
 * out after reading its own VBAR, so the choice is per CPU and costs the
 * rest no load. x30 rides in TPIDRRO_EL0, zero again by the eret either
 * way: every other register already holds EL0's value, and NZCV comes back
 * from SPSR_EL1. */
.macro EXIT_TO_EL0
    msr     tpidrro_el0, x30
    mrs     x30, vbar_el1
    eor     x30, x30, #EPD1_TRAMPOLINE
    cbnz    x30, 8f
    mrs     x30, vbar_el1
    add     x30, x30, #EPD1_EXIT_OFFSET
    br      x30
8:  mrs     x30, tpidrro_el0
    msr     tpidrro_el0, xzr
    eret
.endm
