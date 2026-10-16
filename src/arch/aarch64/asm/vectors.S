/* EL1 vector table. 16 entries x 0x80, 2KB-aligned. VBAR_EL1 install
 * point lives in exceptions::install. Frame layout matches
 * exceptions::frame::ExceptionFrame (304 bytes, repr(C)). */

.section .text.vectors, "ax", %progbits

/* 31 GPRs + sp + elr + spsr + esr + far = 36*8 = 288, then EL0's IA key. */
.equ FRAME_SIZE, 304
.equ FRAME_PAC_IA, 288

.macro SAVE_GPRS
    sub     sp, sp, #FRAME_SIZE
    stp     x0,  x1,  [sp, #0]
    stp     x2,  x3,  [sp, #16]
    stp     x4,  x5,  [sp, #32]
    stp     x6,  x7,  [sp, #48]
    stp     x8,  x9,  [sp, #64]
    stp     x10, x11, [sp, #80]
    stp     x12, x13, [sp, #96]
    stp     x14, x15, [sp, #112]
    stp     x16, x17, [sp, #128]
    stp     x18, x19, [sp, #144]
    stp     x20, x21, [sp, #160]
    stp     x22, x23, [sp, #176]
    stp     x24, x25, [sp, #192]
    stp     x26, x27, [sp, #208]
    stp     x28, x29, [sp, #224]
    str     x30,      [sp, #240]
.endm

.macro SAVE_SYSREGS_KERNEL
    /* Pre-exception kernel sp = sp + FRAME_SIZE. */
    add     x0, sp, #FRAME_SIZE
    str     x0,       [sp, #248]
    mrs     x0, elr_el1
    mrs     x1, spsr_el1
    stp     x0, x1,   [sp, #256]
    mrs     x0, esr_el1
    mrs     x1, far_el1
    stp     x0, x1,   [sp, #272]
.endm

.macro SAVE_SYSREGS_USER
    mrs     x0, sp_el0
    str     x0,       [sp, #248]
    mrs     x0, elr_el1
    mrs     x1, spsr_el1
    stp     x0, x1,   [sp, #256]
    mrs     x0, esr_el1
    mrs     x1, far_el1
    stp     x0, x1,   [sp, #272]
.endm

/* Speculation work on entry from EL0, as security::spectre decided for the
 * CPUs (entry_word.rs): bit 3 CLRBHB, bits 15:8 the history loop's branch
 * count, bits 1:0 a firmware call (1 ARCH_WORKAROUND_1, 2 _3) by SMC, or HVC
 * where bit 2 is set. It runs before the handler's first indirect branch,
 * which is what EL0's branch history must not steer; the frame holds every
 * register already, so x0-x3 and x9-x10 are free. CLRBHB is HINT #22, a NOP
 * where not implemented. The loop, `k` taken branches then DSB and ISB, and
 * the function ids are from Arm's Spectre-BHB guidance and SMCCC (DEN0028). */
.macro SPECTRE_FROM_EL0
    adrp    x9, NONOS_SPECTRE_EL0_ENTRY
    ldr     w9, [x9, :lo12:NONOS_SPECTRE_EL0_ENTRY]
    cbz     w9, 9f
    tbz     w9, #3, 1f
    hint    #22                         /* CLRBHB */
    isb
1:  ubfx    w10, w9, #8, #8
    cbz     w10, 3f
2:  b       . + 4
    subs    w10, w10, #1
    b.ne    2b
    dsb     nsh
    isb
3:  and     w10, w9, #3
    cbz     w10, 9f
    mov     w0, #0x8000                 /* SMCCC_ARCH_WORKAROUND_1 */
    cmp     w10, #2
    b.ne    4f
    mov     w0, #0x3fff                 /* SMCCC_ARCH_WORKAROUND_3 */
4:  movk    w0, #0x8000, lsl #16
    tbnz    w9, #2, 5f
    smc     #0
    b       9f
5:  hvc     #0
9:
.endm

#include "epd1.h"

/* The IA key across the EL0 boundary (security::pac): EL0's is kept in the
 * frame and the kernel's put in, and EL0's put back before the eret. Only
 * where TPIDR_EL1 names a key slot, which is only where pointer
 * authentication is on: elsewhere the key registers may not exist. x9-x11
 * are saved in the frame already and reloaded from it after. */
.macro PAC_FROM_EL0
    mrs     x9, tpidr_el1
    cbz     x9, 1f
    mrs     x10, S3_0_C2_C1_0           /* APIAKeyLo_EL1 */
    mrs     x11, S3_0_C2_C1_1           /* APIAKeyHi_EL1 */
    stp     x10, x11, [sp, #FRAME_PAC_IA]
    ldp     x10, x11, [x9, #16]         /* the slot's kernel key */
    msr     S3_0_C2_C1_0, x10
    msr     S3_0_C2_C1_1, x11
    isb
1:
.endm

/* No isb: the eret that follows synchronizes the write. */
.macro PAC_TO_EL0
    mrs     x9, tpidr_el1
    cbz     x9, 1f
    ldp     x10, x11, [sp, #FRAME_PAC_IA]
    msr     S3_0_C2_C1_0, x10
    msr     S3_0_C2_C1_1, x11
1:
.endm

.macro RESTORE_GPRS
    ldp     x0,  x1,  [sp, #0]
    ldp     x2,  x3,  [sp, #16]
    ldp     x4,  x5,  [sp, #32]
    ldp     x6,  x7,  [sp, #48]
    ldp     x8,  x9,  [sp, #64]
    ldp     x10, x11, [sp, #80]
    ldp     x12, x13, [sp, #96]
    ldp     x14, x15, [sp, #112]
    ldp     x16, x17, [sp, #128]
    ldp     x18, x19, [sp, #144]
    ldp     x20, x21, [sp, #160]
    ldp     x22, x23, [sp, #176]
    ldp     x24, x25, [sp, #192]
    ldp     x26, x27, [sp, #208]
    ldp     x28, x29, [sp, #224]
    ldr     x30,      [sp, #240]
    add     sp, sp, #FRAME_SIZE
.endm

.macro RESTORE_SYSREGS_KERNEL
    ldp     x0, x1,   [sp, #256]
    msr     elr_el1, x0
    msr     spsr_el1, x1
.endm

.macro RESTORE_SYSREGS_USER
    ldr     x0,       [sp, #248]
    msr     sp_el0, x0
    ldp     x0, x1,   [sp, #256]
    msr     elr_el1, x0
    msr     spsr_el1, x1
.endm

/* Vector table. Each slot is exactly 0x80 bytes; we use one branch and
 * leave the assembler to pad. */
.balign 0x800
.globl __aarch64_vectors_el1
.type  __aarch64_vectors_el1, %function
__aarch64_vectors_el1:

    /* Current EL with SP_EL0 — kernel uses SPx; SP0 is a bug. */
    .balign 0x80
    b   __aarch64_invalid_sp0_sync
    .balign 0x80
    b   __aarch64_invalid_sp0_irq
    .balign 0x80
    b   __aarch64_invalid_sp0_fiq
    .balign 0x80
    b   __aarch64_invalid_sp0_serror

    /* Current EL with SP_ELx — kernel mode. */
    .balign 0x80
    b   __aarch64_sync_current
    .balign 0x80
    b   __aarch64_irq_current
    .balign 0x80
    b   __aarch64_fiq_current
    .balign 0x80
    b   __aarch64_serror_current

    /* Lower EL using AArch64 — user mode (CPL=3 equivalent). */
    .balign 0x80
    b   __aarch64_sync_lower
    .balign 0x80
    b   __aarch64_irq_lower
    .balign 0x80
    b   __aarch64_fiq_lower
    .balign 0x80
    b   __aarch64_serror_lower

    /* Lower EL using AArch32 — unsupported. */
    .balign 0x80
    b   __aarch64_invalid_aarch32_sync
    .balign 0x80
    b   __aarch64_invalid_aarch32_irq
    .balign 0x80
    b   __aarch64_invalid_aarch32_fiq
    .balign 0x80
    b   __aarch64_invalid_aarch32_serror

.size __aarch64_vectors_el1, . - __aarch64_vectors_el1

/* Where the EPD1 trampoline's slots land, in the same order as the table
 * above: x30 back from TPIDRRO_EL0, which is cleared, then the same entry
 * the table's slot takes. Reached by BR from the trampoline, hence `bti j`. */
.macro TRAMP_TARGET handler
    .balign 0x80
    hint    #36                         /* BTI j */
    mrs     x30, tpidrro_el0
    msr     tpidrro_el0, xzr
    b       \handler
.endm

.balign 0x800
.globl __aarch64_tramp_targets
__aarch64_tramp_targets:
    TRAMP_TARGET __aarch64_invalid_sp0_sync
    TRAMP_TARGET __aarch64_invalid_sp0_irq
    TRAMP_TARGET __aarch64_invalid_sp0_fiq
    TRAMP_TARGET __aarch64_invalid_sp0_serror
    TRAMP_TARGET __aarch64_sync_current
    TRAMP_TARGET __aarch64_irq_current
    TRAMP_TARGET __aarch64_fiq_current
    TRAMP_TARGET __aarch64_serror_current
    TRAMP_TARGET __aarch64_sync_lower
    TRAMP_TARGET __aarch64_irq_lower
    TRAMP_TARGET __aarch64_fiq_lower
    TRAMP_TARGET __aarch64_serror_lower
    TRAMP_TARGET __aarch64_invalid_aarch32_sync
    TRAMP_TARGET __aarch64_invalid_aarch32_irq
    TRAMP_TARGET __aarch64_invalid_aarch32_fiq
    TRAMP_TARGET __aarch64_invalid_aarch32_serror

/* Entry stubs. Each saves the frame, calls the Rust handler with x0 =
 * frame pointer, then restores and erets. A handler that diverges
 * (halt) never reaches the eret path. */

.balign 16
__aarch64_sync_current:
    SAVE_GPRS
    SAVE_SYSREGS_KERNEL
    mov     x0, sp
    bl      aarch64_exc_sync_current
    RESTORE_SYSREGS_KERNEL
    RESTORE_GPRS
    eret

.balign 16
__aarch64_irq_current:
    SAVE_GPRS
    SAVE_SYSREGS_KERNEL
    mov     x0, sp
    bl      aarch64_exc_irq_current
    RESTORE_SYSREGS_KERNEL
    RESTORE_GPRS
    eret

.balign 16
__aarch64_fiq_current:
    SAVE_GPRS
    SAVE_SYSREGS_KERNEL
    mov     x0, sp
    bl      aarch64_exc_fiq_current
    RESTORE_SYSREGS_KERNEL
    RESTORE_GPRS
    eret

.balign 16
__aarch64_serror_current:
    SAVE_GPRS
    SAVE_SYSREGS_KERNEL
    mov     x0, sp
    bl      aarch64_exc_serror_current
    RESTORE_SYSREGS_KERNEL
    RESTORE_GPRS
    eret

.balign 16
__aarch64_sync_lower:
    SAVE_GPRS
    SAVE_SYSREGS_USER
    SPECTRE_FROM_EL0
    PAC_FROM_EL0
    mov     x0, sp
    bl      aarch64_exc_sync_lower
    RESTORE_SYSREGS_USER
    PAC_TO_EL0
    RESTORE_GPRS
    EXIT_TO_EL0

.balign 16
__aarch64_irq_lower:
    SAVE_GPRS
    SAVE_SYSREGS_USER
    SPECTRE_FROM_EL0
    PAC_FROM_EL0
    mov     x0, sp
    bl      aarch64_exc_irq_lower
    RESTORE_SYSREGS_USER
    PAC_TO_EL0
    RESTORE_GPRS
    EXIT_TO_EL0

.balign 16
__aarch64_fiq_lower:
    SAVE_GPRS
    SAVE_SYSREGS_USER
    SPECTRE_FROM_EL0
    PAC_FROM_EL0
    mov     x0, sp
    bl      aarch64_exc_fiq_lower
    RESTORE_SYSREGS_USER
    PAC_TO_EL0
    RESTORE_GPRS
    EXIT_TO_EL0

.balign 16
__aarch64_serror_lower:
    SAVE_GPRS
    SAVE_SYSREGS_USER
    SPECTRE_FROM_EL0
    PAC_FROM_EL0
    mov     x0, sp
    bl      aarch64_exc_serror_lower
    RESTORE_SYSREGS_USER
    PAC_TO_EL0
    RESTORE_GPRS
    EXIT_TO_EL0

/* The two groups the kernel never uses. Each slot gets its own entry so
 * the handler knows which kind of exception it was: ESR_EL1 and FAR_EL1
 * belong to it only for a synchronous exception or an SError, and an
 * interrupt leaves whatever an earlier exception wrote there. x1 carries
 * the kind (0 sync, 1 IRQ, 2 FIQ, 3 SError) and is set after the frame is
 * saved, so the frame holds the interrupted x1.
 *
 * The SP_EL0 group is taken from EL1 running on SP_EL0, so the stack
 * pointer the interrupted code had is SP_EL0, not the SP_EL1 this entry
 * runs on. The user save records exactly that, as it does for AArch32 EL0. */
.macro INVALID_ENTRY name, handler, kind
.balign 16
\name:
    SAVE_GPRS
    SAVE_SYSREGS_USER
    mov     x0, sp
    mov     x1, #\kind
    bl      \handler
1:  wfi
    b       1b
.endm

INVALID_ENTRY __aarch64_invalid_sp0_sync,       aarch64_exc_invalid_sp0,     0
INVALID_ENTRY __aarch64_invalid_sp0_irq,        aarch64_exc_invalid_sp0,     1
INVALID_ENTRY __aarch64_invalid_sp0_fiq,        aarch64_exc_invalid_sp0,     2
INVALID_ENTRY __aarch64_invalid_sp0_serror,     aarch64_exc_invalid_sp0,     3
INVALID_ENTRY __aarch64_invalid_aarch32_sync,   aarch64_exc_invalid_aarch32, 0
INVALID_ENTRY __aarch64_invalid_aarch32_irq,    aarch64_exc_invalid_aarch32, 1
INVALID_ENTRY __aarch64_invalid_aarch32_fiq,    aarch64_exc_invalid_aarch32, 2
INVALID_ENTRY __aarch64_invalid_aarch32_serror, aarch64_exc_invalid_aarch32, 3
