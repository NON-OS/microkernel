/* aarch64 first-entry to EL0.
 *
 * extern "C" fn aarch64_enter_user(ctx: *const UserEntry) -> !
 *
 * x0 = ctx. Layout (matches src/arch/aarch64/context/types.rs::UserEntry):
 *     0    entry      (ELR_EL1)
 *     8    user_sp    (SP_EL0)
 *    16    spsr       (SPSR_EL1)
 *    24    kernel_sp  (SP_EL1 to install before eret)
 *    32    args[0..8] (x0..x7 at eret)
 *
 * On entry IRQs are masked by the caller, and masked again here: an
 * exception between the ELR_EL1 write and the eret would overwrite it. The eret crosses to EL0t with
 * SPSR's DAIF (the caller writes SPIE-equivalent bits as it sees fit).
 * FP/SIMD is left in whatever state CPACR_EL1 already has — the lazy-
 * enable trap delivers EL0 FP traps to the fail-closed path until per-
 * PCB FP slot ownership lands. */

#include "epd1.h"
#include "percpu.h"

.section .text.enter_user, "ax", %progbits

.balign 16
.globl aarch64_enter_user
.type  aarch64_enter_user, %function
aarch64_enter_user:
    hint    #34                         /* BTI c: a landing pad (FEAT_BTI) */
    msr     daifset, #0xf
    /* System regs first, using x9 as scratch (caller-save). */
    ldr     x9,  [x0, #0]
    msr     elr_el1, x9
    ldr     x9,  [x0, #8]
    msr     sp_el0, x9
    ldr     x9,  [x0, #16]
    msr     spsr_el1, x9

    /* The process's IA key, which the switch put in this CPU's per-CPU
     * block (security::pac), before x10 and x11 are loaded; none where the
     * block says keys are not switched (asm/percpu.h). */
    mrs     x9, tpidr_el1
    cbz     x9, 1f
    ldr     x10, [x9, #PERCPU_PAC_ON]
    cbz     x10, 1f
    ldp     x10, x11, [x9, #PERCPU_PAC_USER]
    msr     S3_0_C2_C1_0, x10           /* APIAKeyLo_EL1 */
    msr     S3_0_C2_C1_1, x11           /* APIAKeyHi_EL1 */
1:
    /* Load args x2..x7 in pairs; x0 still holds ctx so do it last. */
    ldp     x2,  x3,  [x0, #(32 + 16)]
    ldp     x4,  x5,  [x0, #(32 + 32)]
    ldp     x6,  x7,  [x0, #(32 + 48)]

    /* Install per-task kernel sp into SP_EL1. After this `mov`, the
     * caller's stack is no longer reachable — the helper does not
     * return, so abandoning it is the contract. */
    ldr     x9,  [x0, #24]
    mov     sp,  x9

    /* Nothing of the kernel's goes to EL0: x8-x30 held kernel values,
     * pointers among them, and the process starts with them clear. */
    mov     x8,  xzr
    mov     x9,  xzr
    mov     x10, xzr
    mov     x11, xzr
    mov     x12, xzr
    mov     x13, xzr
    mov     x14, xzr
    mov     x15, xzr
    mov     x16, xzr
    mov     x17, xzr
    mov     x18, xzr
    mov     x19, xzr
    mov     x20, xzr
    mov     x21, xzr
    mov     x22, xzr
    mov     x23, xzr
    mov     x24, xzr
    mov     x25, xzr
    mov     x26, xzr
    mov     x27, xzr
    mov     x28, xzr
    mov     x29, xzr
    mov     x30, xzr

    /* x0,x1 last — x0 was the ctx pointer. */
    ldp     x0,  x1,  [x0, #(32 + 0)]

    isb
    EXIT_TO_EL0

.size aarch64_enter_user, . - aarch64_enter_user
