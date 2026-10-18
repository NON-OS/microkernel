/* A CPU's suspend context: what `aarch64_cpu_suspend` saves before asking
 * PSCI for an idle state that powers the core down, and what
 * `_aarch64_resume_start` puts back when firmware brings the core up at it
 * (suspend.S). One per CPU, a static in the kernel image (idle_states/
 * context.rs); layout.rs asserts the Rust side and the proofs hold the two
 * equal. No key is ever stored here: the pointer-authentication keys come
 * back from the per-CPU block (percpu.h).
 *
 * MAGIC and MPIDR are written by Rust before the call and checked by the
 * resume before anything is restored; FLAGS says which optional parts this
 * CPU has, and the save adds whether FP/SIMD was live. */

#define CTX_MAGIC 0
#define CTX_MPIDR 8
#define CTX_FLAGS 16
#define CTX_X18 24
#define CTX_X19 32
#define CTX_X20 40
#define CTX_X21 48
#define CTX_X22 56
#define CTX_X23 64
#define CTX_X24 72
#define CTX_X25 80
#define CTX_X26 88
#define CTX_X27 96
#define CTX_X28 104
#define CTX_X29 112
#define CTX_X30 120
#define CTX_SP 128
#define CTX_SCTLR 136
#define CTX_TCR 144
#define CTX_MAIR 152
#define CTX_TTBR0 160
#define CTX_TTBR1 168
#define CTX_VBAR 176
#define CTX_TPIDR_EL1 184
#define CTX_TPIDR_EL0 192
#define CTX_TPIDRRO_EL0 200
#define CTX_SP_EL0 208
#define CTX_CPACR 216
#define CTX_CNTKCTL 224
#define CTX_CNTP_CTL 232
#define CTX_CNTP_CVAL 240
#define CTX_CNTV_CTL 248
#define CTX_CNTV_CVAL 256
#define CTX_MDSCR 264
#define CTX_OSLSR 272
#define CTX_PAN 280
#define CTX_ZCR 288
#define CTX_GCR 296
#define CTX_ICC_SRE 304
#define CTX_ICC_PMR 312
#define CTX_ICC_BPR1 320
#define CTX_ICC_CTLR 328
#define CTX_ICC_IGRPEN1 336
#define CTX_FPCR 344
#define CTX_FPSR 352
/* Q0-Q31, 16-aligned, after one pad word. */
#define CTX_Q 368
#define CTX_SIZE 880

/* "NONOSRSM": an armed context, built 16 bits at a time (ARMED_INTO). */
#define CTX_ARMED 0x4e4f4e4f5352534d
.macro ARMED_INTO reg
    movz    \reg, #0x534d
    movk    \reg, #0x5352, lsl #16
    movk    \reg, #0x4e4f, lsl #32
    movk    \reg, #0x4e4f, lsl #48
.endm

/* MPIDR_EL1's affinity, Aff3 and Aff2-Aff0, as Rust stores it in MPIDR:
 * bits 39:32 and 23:0. */
.macro AFFINITY_INTO reg, tmp
    mrs     \tmp, mpidr_el1
    and     \reg, \tmp, #0xffffff
    ubfx    \tmp, \tmp, #32, #8
    orr     \reg, \reg, \tmp, lsl #32
.endm

/* FLAGS bits. FP is the save's own; the rest come from Rust, each from the
 * ID register that says the registers exist. */
#define FLAG_FP 0
#define FLAG_GICV3 1
#define FLAG_PAN 2
#define FLAG_SVE 3
#define FLAG_MTE2 4
#define FLAG_SSBS 5

/* What the call returns when the core powered down and came back. PSCI's
 * own answers are 0 and negative. */
#define SUSPEND_RESUMED 1
