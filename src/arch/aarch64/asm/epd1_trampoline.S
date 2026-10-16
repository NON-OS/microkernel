/* The EPD1 trampoline, for CPUs Meltdown reaches and E0PD cannot shield
 * (security/epd1, C-24's `Meltdown::Exposed`).
 *
 * While EL0 runs on such a CPU, TCR_EL1.EPD1 is set, so nothing walks
 * TTBR1_EL1's tables, and the exit emptied this CPU's TLB, so no kernel
 * translation is left to hit: speculation from EL0 has no way to a kernel
 * address. This page is mapped at EPD1_TRAMPOLINE in TTBR0, in every user
 * root and in the kernel's, EL1 only; the page after it holds where the
 * kernel's real entries are, and nothing else. Such a CPU keeps VBAR_EL1
 * here for good, so its exceptions from EL1 come through here too, with
 * EPD1 already clear: the only stretch with it set is the exit below, which
 * runs with every exception masked.
 *
 * Every entry finds all registers live, so x30 is put in TPIDRRO_EL0,
 * which nothing else in NONOS uses and which is zero while EL0 runs, and
 * taken back in the target (asm/vectors.S, __aarch64_tramp_targets). An
 * entry from EL0 clears EPD1 before anything else, and so before any kernel
 * data is touched. The branch to the target is BR through x30, which the
 * targets' `bti j` admits (Arm ARM, DDI 0487: TCR_EL1.EPD1; PSTATE.BTYPE;
 * VBAR_EL1 offsets 0x400-0x7FF for a lower EL using AArch64). */

#include "epd1.h"

.macro TRAMP_ENTRY slot, from_el0
    .balign 0x80
    msr     tpidrro_el0, x30
    .if \from_el0
    mrs     x30, tcr_el1
    bic     x30, x30, #TCR_EPD1
    msr     tcr_el1, x30
    isb
    .endif
    ldr     x30, __epd1_trampoline + 0x1000     /* the targets, high */
    add     x30, x30, #(\slot * 0x80)
    br      x30
.endm

.section .text.epd1_trampoline, "ax", %progbits
.balign 0x1000
.globl __epd1_trampoline
.type  __epd1_trampoline, %function
__epd1_trampoline:
    TRAMP_ENTRY 0, 0
    TRAMP_ENTRY 1, 0
    TRAMP_ENTRY 2, 0
    TRAMP_ENTRY 3, 0
    TRAMP_ENTRY 4, 0
    TRAMP_ENTRY 5, 0
    TRAMP_ENTRY 6, 0
    TRAMP_ENTRY 7, 0
    TRAMP_ENTRY 8, 1
    TRAMP_ENTRY 9, 1
    TRAMP_ENTRY 10, 1
    TRAMP_ENTRY 11, 1
    TRAMP_ENTRY 12, 1
    TRAMP_ENTRY 13, 1
    TRAMP_ENTRY 14, 1
    TRAMP_ENTRY 15, 1

/* The way out to EL0, at EPD1_EXIT_OFFSET, which EXIT_TO_EL0 branches to
 * with the user's x30 in TPIDRRO_EL0. EPD1 goes on and this CPU's TLB is
 * emptied of every translation, the kernel's global ones with the rest;
 * what is fetched after that is this page, through TTBR0. The barrier after
 * the eret keeps anything past it from running ahead. */
    .balign EPD1_EXIT_OFFSET
    hint    #36                         /* BTI j */
    mrs     x30, tcr_el1
    orr     x30, x30, #TCR_EPD1
    msr     tcr_el1, x30
    isb
    tlbi    vmalle1
    dsb     nsh
    isb
    mrs     x30, tpidrro_el0
    msr     tpidrro_el0, xzr
    eret
    dsb     nsh
    isb

    .balign 0x1000
.size __epd1_trampoline, . - __epd1_trampoline
