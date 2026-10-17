/* Powering a core down for idle and bringing it back (PSCI, DEN0022,
 * CPU_SUSPEND with a powerdown power_state; recalled, unverified: the core
 * loses its registers and caches, and comes back at the entry point given,
 * at EL1 or EL2, with the MMU off, interrupts masked and x0 the context id).
 *
 * `aarch64_cpu_suspend` saves this CPU's EL1 state into its context
 * (suspend.h) and makes the call. If the call returns, the core never went
 * down, and the PSCI answer is returned. If it went down, firmware starts
 * it at `_aarch64_resume_start`, which turns the MMU on as a secondary's
 * first start does, puts the state back, and returns SUSPEND_RESUMED from
 * the same call, on the same stack, to the same caller. To Rust it is one
 * call that returns once.
 *
 * Each register saved is put back, under the same FLAGS bit, at the same
 * offset; nothing else is. Why each one:
 *   x18-x30, SP     the caller's frame, which AAPCS64 says survives a call
 *   SCTLR_EL1       MMU and caches, alignment, UCI/UCT, SPAN, EnIA, BT0/1,
 *                   DSSBS, ATA: every control boot and security::* set
 *   TCR, MAIR, TTBR0/1  translation: this CPU's own, a process's root and
 *                   ASID in TTBR0 included, not the boot CPU's
 *   VBAR_EL1        per CPU, it is the EPD1 switch (security/epd1)
 *   TPIDR_EL1       the per-CPU block, and so the PAC keys (C-35)
 *   TPIDR_EL0, TPIDRRO_EL0  the current thread's, and the trampoline's
 *   SP_EL0          the IRQ stack (boot/stack/register.rs)
 *   CPACR_EL1       FPEN and ZEN: the lazy FP state's trap, as it was
 *   CNTKCTL_EL1     what EL0 may read of the counter
 *   CNTP_*, CNTV_*  this CPU's deadlines, compare before control
 *   MDSCR_EL1, OS lock  debug state as it was, whatever firmware left
 *   PAN             PSTATE.PAN, set in the kernel (security/pan.rs)
 *   ZCR_EL1         with SVE, EL1's vector length
 *   GCR_EL1         with MTE2, the tags IRG may pick (security/mte)
 *   ICC_*_EL1       with GICv3, the CPU interface, which is in the core
 *   FPCR, FPSR, Q0-Q31  when FP/SIMD was live: lazy FP leaves a thread's
 *                   registers in place across an idle
 * PSTATE.SSBS is set from the restored SCTLR_EL1.DSSBS, as every exception
 * entry does (security/spectre/ssbs.rs); it has no register to read on a
 * core with only the immediate form.
 *
 * Not saved, because nothing relies on them across a wait for an
 * interrupt: ELR_EL1, SPSR_EL1, ESR_EL1, FAR_EL1, which any interrupt taken
 * in the idle overwrites; and the keys, which come back from the per-CPU
 * block (percpu.h), so the context, kernel memory or not, never holds one. */

#include "percpu.h"
#include "el2.h"
#include "mmu_on.h"
#include "suspend.h"

.macro SAVE reg, off
    mrs     x9, \reg
    str     x9, [x0, #\off]
.endm

.macro LOAD reg, off
    ldr     x9, [x0, #\off]
    msr     \reg, x9
.endm

/* The kernel is built without FP/SIMD (-mgeneral-regs-only, softfloat), so
 * a task's vector registers survive every kernel entry. This file is one of
 * the few that move them on purpose. */
.arch_extension fp
.arch_extension simd

/* x0 = the context, x1 = power_state, x2 = the entry point's physical
 * address, x3 = the context id (the context's physical address), x4 = 1 to
 * call by SMC, 0 by HVC. Called with IRQs masked. */
.section .text.aarch64_cpu_suspend, "ax", %progbits
.balign 16
.globl aarch64_cpu_suspend
.type  aarch64_cpu_suspend, %function

aarch64_cpu_suspend:
    hint    #34                         /* BTI c */
    stp     x18, x19, [x0, #CTX_X18]
    stp     x20, x21, [x0, #CTX_X20]
    stp     x22, x23, [x0, #CTX_X22]
    stp     x24, x25, [x0, #CTX_X24]
    stp     x26, x27, [x0, #CTX_X26]
    stp     x28, x29, [x0, #CTX_X28]
    str     x30, [x0, #CTX_X30]
    mov     x9, sp
    str     x9, [x0, #CTX_SP]

    /* FP/SIMD was live if this CPU could use it: FPEN = 0b11. */
    ldr     x10, [x0, #CTX_FLAGS]
    bic     x10, x10, #(1 << FLAG_FP)
    mrs     x9, cpacr_el1
    ubfx    x9, x9, #20, #2
    cmp     x9, #3
    b.ne    1f
    orr     x10, x10, #(1 << FLAG_FP)
1:  str     x10, [x0, #CTX_FLAGS]

    SAVE    sctlr_el1, CTX_SCTLR
    SAVE    tcr_el1, CTX_TCR
    SAVE    mair_el1, CTX_MAIR
    SAVE    ttbr0_el1, CTX_TTBR0
    SAVE    ttbr1_el1, CTX_TTBR1
    SAVE    vbar_el1, CTX_VBAR
    SAVE    tpidr_el1, CTX_TPIDR_EL1
    SAVE    tpidr_el0, CTX_TPIDR_EL0
    SAVE    tpidrro_el0, CTX_TPIDRRO_EL0
    SAVE    sp_el0, CTX_SP_EL0
    SAVE    cpacr_el1, CTX_CPACR
    SAVE    cntkctl_el1, CTX_CNTKCTL
    SAVE    cntp_cval_el0, CTX_CNTP_CVAL
    SAVE    cntp_ctl_el0, CTX_CNTP_CTL
    SAVE    cntv_cval_el0, CTX_CNTV_CVAL
    SAVE    cntv_ctl_el0, CTX_CNTV_CTL
    SAVE    mdscr_el1, CTX_MDSCR
    SAVE    oslsr_el1, CTX_OSLSR
    tbz     x10, #FLAG_PAN, 2f
    SAVE    S3_0_C4_C2_3, CTX_PAN       /* PAN */
2:  tbz     x10, #FLAG_SVE, 3f
    SAVE    S3_0_C1_C2_0, CTX_ZCR       /* ZCR_EL1 */
3:  tbz     x10, #FLAG_MTE2, 4f
    SAVE    S3_0_C1_C6_6, CTX_GCR       /* GCR_EL1 */
4:  tbz     x10, #FLAG_GICV3, 5f
    SAVE    S3_0_C12_C12_5, CTX_ICC_SRE /* ICC_SRE_EL1 */
    SAVE    S3_0_C4_C6_0, CTX_ICC_PMR   /* ICC_PMR_EL1 */
    SAVE    S3_0_C12_C12_3, CTX_ICC_BPR1 /* ICC_BPR1_EL1 */
    SAVE    S3_0_C12_C12_4, CTX_ICC_CTLR /* ICC_CTLR_EL1 */
    SAVE    S3_0_C12_C12_7, CTX_ICC_IGRPEN1 /* ICC_IGRPEN1_EL1 */
5:  tbz     x10, #FLAG_FP, 6f
    SAVE    fpcr, CTX_FPCR
    SAVE    fpsr, CTX_FPSR
    add     x9, x0, #CTX_Q
    stp     q0, q1, [x9, #0]
    stp     q2, q3, [x9, #32]
    stp     q4, q5, [x9, #64]
    stp     q6, q7, [x9, #96]
    stp     q8, q9, [x9, #128]
    stp     q10, q11, [x9, #160]
    stp     q12, q13, [x9, #192]
    stp     q14, q15, [x9, #224]
    stp     q16, q17, [x9, #256]
    stp     q18, q19, [x9, #288]
    stp     q20, q21, [x9, #320]
    stp     q22, q23, [x9, #352]
    stp     q24, q25, [x9, #384]
    stp     q26, q27, [x9, #416]
    stp     q28, q29, [x9, #448]
    stp     q30, q31, [x9, #480]
6:
    /* The context to the point of coherency: the resume reads it before
     * this core's caches are anyone's, if they come back at all. */
    mrs     x9, ctr_el0
    ubfx    x9, x9, #16, #4             /* DminLine, log2 words */
    mov     x11, #4
    lsl     x11, x11, x9
    sub     x12, x11, #1
    bic     x9, x0, x12
    add     x12, x0, #CTX_SIZE
7:  dc      cvac, x9
    add     x9, x9, x11
    cmp     x9, x12
    b.lo    7b
    dsb     sy

    /* CPU_SUSPEND (SMC64). x19 is saved above, so it may hold the context
     * across the call, which preserves x18-x30 and SP (SMCCC). */
    mov     x19, x0
    movz    w0, #0x0001
    movk    w0, #0xc400, lsl #16
    cbz     x4, 8f
    smc     #0
    b       9f
8:  hvc     #0
9:
    /* The core stayed up: PSCI's answer, the caller's registers as saved. */
    mov     x9, x0
    mov     x0, x19
    ldp     x18, x19, [x0, #CTX_X18]
    ldp     x20, x21, [x0, #CTX_X20]
    ldp     x22, x23, [x0, #CTX_X22]
    ldp     x24, x25, [x0, #CTX_X24]
    ldp     x26, x27, [x0, #CTX_X26]
    ldp     x28, x29, [x0, #CTX_X28]
    ldr     x30, [x0, #CTX_X30]
    mov     x0, x9
    ret

.size aarch64_cpu_suspend, . - aarch64_cpu_suspend


/* Where firmware brings a powered-down core back: physical, MMU off, x0 the
 * context's physical address. The same way in as a secondary's first start
 * (start.S), then to the kernel's own address for the rest. */
.section .text._aarch64_resume_start, "ax", %progbits
.balign 16
.globl _aarch64_resume_start
.type  _aarch64_resume_start, %function

_aarch64_resume_start:
    drop_from_el2
    msr     daifset, #0xf
    mov     x1, #0x0838
    movk    x1, #0x30C5, lsl #16
    msr     sctlr_el1, x1
    isb
    MMU_ON_FROM_CONTEXT
    add     x0, x0, x7                  /* the context where the kernel runs */
    adrp    x16, aarch64_resume_high
    add     x16, x16, :lo12:aarch64_resume_high
    add     x16, x16, x7
    br      x16

.size _aarch64_resume_start, . - _aarch64_resume_start
/* Where the stub ends: the boot check (idle_states/resume_map.rs) asks
 * that every byte from _aarch64_resume_start to here runs one to one. */
.globl _aarch64_resume_start_end
_aarch64_resume_start_end:


.section .text.aarch64_resume_high, "ax", %progbits
.balign 16
.globl aarch64_resume_high
.type  aarch64_resume_high, %function

aarch64_resume_high:
    hint    #34                         /* BTI c */
    msr     spsel, #1

    /* A context this CPU armed, or nothing is put back. */
    ARMED_INTO x10
    ldr     x9, [x0, #CTX_MAGIC]
    cmp     x9, x10
    b.ne    .L_resume_lost
    AFFINITY_INTO x10, x11
    ldr     x9, [x0, #CTX_MPIDR]
    cmp     x9, x10
    b.ne    .L_resume_lost
    ldr     x10, [x0, #CTX_FLAGS]

    /* This CPU's own translation, over the boot CPU's the stub used. */
    LOAD    mair_el1, CTX_MAIR
    LOAD    tcr_el1, CTX_TCR
    LOAD    ttbr1_el1, CTX_TTBR1
    LOAD    ttbr0_el1, CTX_TTBR0
    isb
    tlbi    vmalle1
    dsb     nsh
    isb

    /* The per-CPU block, then the kernel's key from it where this CPU
     * switches keys, before SCTLR_EL1 turns EnIA back on and before any
     * return a signed frame authenticates. */
    LOAD    tpidr_el1, CTX_TPIDR_EL1
    mrs     x9, tpidr_el1
    cbz     x9, 10f
    ldr     x11, [x9, #PERCPU_PAC_ON]
    cbz     x11, 10f
    ldp     x11, x12, [x9, #PERCPU_PAC_KERNEL]
    msr     S3_0_C2_C1_0, x11           /* APIAKeyLo_EL1 */
    msr     S3_0_C2_C1_1, x12           /* APIAKeyHi_EL1 */
10: LOAD    sctlr_el1, CTX_SCTLR
    isb
    LOAD    vbar_el1, CTX_VBAR
    LOAD    tpidr_el0, CTX_TPIDR_EL0
    LOAD    tpidrro_el0, CTX_TPIDRRO_EL0
    LOAD    sp_el0, CTX_SP_EL0
    LOAD    cpacr_el1, CTX_CPACR
    isb
    LOAD    cntkctl_el1, CTX_CNTKCTL
    LOAD    cntp_cval_el0, CTX_CNTP_CVAL
    LOAD    cntp_ctl_el0, CTX_CNTP_CTL
    LOAD    cntv_cval_el0, CTX_CNTV_CVAL
    LOAD    cntv_ctl_el0, CTX_CNTV_CTL
    LOAD    mdscr_el1, CTX_MDSCR
    ldr     x9, [x0, #CTX_OSLSR]        /* OSLSR_EL1.OSLK, bit 1, */
    ubfx    x9, x9, #1, #1              /* back through OSLAR_EL1.OSLK */
    msr     oslar_el1, x9
    tbz     x10, #FLAG_PAN, 11f
    LOAD    S3_0_C4_C2_3, CTX_PAN       /* PAN */
11: tbz     x10, #FLAG_SVE, 12f
    LOAD    S3_0_C1_C2_0, CTX_ZCR       /* ZCR_EL1 */
12: tbz     x10, #FLAG_MTE2, 13f
    LOAD    S3_0_C1_C6_6, CTX_GCR       /* GCR_EL1 */
13: tbz     x10, #FLAG_GICV3, 14f
    LOAD    S3_0_C12_C12_5, CTX_ICC_SRE /* ICC_SRE_EL1 */
    isb
    LOAD    S3_0_C4_C6_0, CTX_ICC_PMR   /* ICC_PMR_EL1 */
    LOAD    S3_0_C12_C12_3, CTX_ICC_BPR1 /* ICC_BPR1_EL1 */
    LOAD    S3_0_C12_C12_4, CTX_ICC_CTLR /* ICC_CTLR_EL1 */
    LOAD    S3_0_C12_C12_7, CTX_ICC_IGRPEN1 /* ICC_IGRPEN1_EL1 */
14: tbz     x10, #FLAG_FP, 15f
    LOAD    fpcr, CTX_FPCR
    LOAD    fpsr, CTX_FPSR
    add     x9, x0, #CTX_Q
    ldp     q0, q1, [x9, #0]
    ldp     q2, q3, [x9, #32]
    ldp     q4, q5, [x9, #64]
    ldp     q6, q7, [x9, #96]
    ldp     q8, q9, [x9, #128]
    ldp     q10, q11, [x9, #160]
    ldp     q12, q13, [x9, #192]
    ldp     q14, q15, [x9, #224]
    ldp     q16, q17, [x9, #256]
    ldp     q18, q19, [x9, #288]
    ldp     q20, q21, [x9, #320]
    ldp     q22, q23, [x9, #352]
    ldp     q24, q25, [x9, #384]
    ldp     q26, q27, [x9, #416]
    ldp     q28, q29, [x9, #448]
    ldp     q30, q31, [x9, #480]
15: tbz     x10, #FLAG_SSBS, 17f
    ldr     x9, [x0, #CTX_SCTLR]
    tbnz    x9, #44, 16f                /* DSSBS */
    .inst   0xd503403f                  /* msr ssbs, #0 */
    b       17f
16: .inst   0xd503413f                  /* msr ssbs, #1 */
17: isb

    /* Disarmed, so a second entry finds nothing to put back, then the
     * caller's frame, and the call returns. */
    str     xzr, [x0, #CTX_MAGIC]
    ldp     x18, x19, [x0, #CTX_X18]
    ldp     x20, x21, [x0, #CTX_X20]
    ldp     x22, x23, [x0, #CTX_X22]
    ldp     x24, x25, [x0, #CTX_X24]
    ldp     x26, x27, [x0, #CTX_X26]
    ldp     x28, x29, [x0, #CTX_X28]
    ldr     x30, [x0, #CTX_X30]
    ldr     x9, [x0, #CTX_SP]
    mov     sp, x9
    mov     x0, #SUSPEND_RESUMED
    ret

/* A context this CPU did not arm: there is no frame to return to. Say so
 * where every other CPU's idle path looks (AARCH64_RESUME_LOST), so none
 * asks for a powerdown state again, and park. */
.L_resume_lost:
    adrp    x9, AARCH64_RESUME_LOST
    add     x9, x9, :lo12:AARCH64_RESUME_LOST
    mov     x10, #1
    str     x10, [x9]
    dsb     sy
18: wfi
    b       18b

.size aarch64_resume_high, . - aarch64_resume_high
.globl aarch64_resume_high_end
aarch64_resume_high_end:
