/* Drop to EL1 if firmware, or PSCI for a secondary or a core coming back
 * from a powerdown idle state, entered at EL2. Every
 * EL2 control is per CPU and resets UNKNOWN, and EL2 has no vectors here,
 * so anything left trapping hangs the CPU the first time EL1 touches it.
 * Bits from the Arm ARM (DDI 0487), each gated on the ID register that
 * says the feature exists; a bit for an absent feature is RES0 or RES1 and
 * is written as such. Leaves x0 alone, for the secondary's context id.
 *   HCR_EL2       RW: EL1 is AArch64. APK and API with pointer
 *                 authentication, or EL1's key writes and PAC
 *                 instructions trap. ATA with MTE2, or GCR_EL1 does.
 *   CNTHCTL_EL2   bits 1:0, EL1 reaches the physical counter and timer
 *   CNTVOFF_EL2   virtual counter equals physical
 *   CPTR_EL2      RES1 bits only, 0x33ff, less TZ with SVE and TSM with
 *                 SME, which are RES1 only without them
 *   ZCR_EL2       LEN at its largest, so EL1's vector length is uncapped
 *   HSTR_EL2      no trapped system registers
 *   HFG*TR_EL2    with fine-grained traps, none set
 *   HCRX_EL2      with it present, every extra control off
 *   MDCR_EL2      no debug, PMU or trace traps; HPMN = PMCR_EL0.N so EL1
 *                 sees every event counter, and with SPE or TRBE their
 *                 buffers owned by EL1 (E2PB, E2TB = 0b11), not trapped
 *   ICC_SRE_EL2   on a GICv3, SRE plus Enable, or EL1 cannot turn on its
 *                 own system-register interface
 *   SCTLR_EL1     MMU and caches off, RES1 bits 11/20/22/23/28/29 */
.macro drop_from_el2
    mrs     x9, CurrentEL
    cmp     x9, #(2 << 2)
    b.ne    9f

    mov     x9, #(1 << 31)
    mrs     x10, id_aa64isar1_el1
    tst     x10, #0xff0                 /* APA, API */
    b.ne    1f
    mrs     x10, S3_0_C0_C6_2           /* ID_AA64ISAR2_EL1 */
    tst     x10, #0xf000                /* APA3 */
    b.eq    2f
1:  orr     x9, x9, #(3 << 40)          /* APK | API */
2:  mrs     x10, id_aa64pfr1_el1
    ubfx    x10, x10, #8, #4            /* MTE */
    cmp     x10, #2
    b.lo    3f
    orr     x9, x9, #(1 << 56)          /* ATA */
3:  msr     hcr_el2, x9

    mrs     x9, cnthctl_el2
    orr     x9, x9, #3
    msr     cnthctl_el2, x9
    msr     cntvoff_el2, xzr
    msr     hstr_el2, xzr

    mov     x9, #0x33ff
    mrs     x10, id_aa64pfr0_el1
    ubfx    x11, x10, #32, #4           /* SVE */
    cbz     x11, 4f
    bic     x9, x9, #(1 << 8)           /* TZ */
4:  mrs     x12, id_aa64pfr1_el1
    ubfx    x12, x12, #24, #4           /* SME */
    cbz     x12, 5f
    bic     x9, x9, #(1 << 12)          /* TSM */
5:  msr     cptr_el2, x9
    isb
    cbz     x11, 6f
    mov     x9, #0xf
    msr     S3_4_C1_C2_0, x9            /* ZCR_EL2 */

6:  mrs     x9, id_aa64mmfr0_el1
    ubfx    x9, x9, #56, #4             /* FGT */
    cbz     x9, 7f
    msr     S3_4_C1_C1_4, xzr           /* HFGRTR_EL2 */
    msr     S3_4_C1_C1_5, xzr           /* HFGWTR_EL2 */
    msr     S3_4_C1_C1_6, xzr           /* HFGITR_EL2 */
    msr     S3_4_C3_C1_4, xzr           /* HDFGRTR_EL2 */
    msr     S3_4_C3_C1_5, xzr           /* HDFGWTR_EL2 */
7:  mrs     x9, id_aa64mmfr1_el1
    ubfx    x9, x9, #40, #4             /* HCX */
    cbz     x9, 8f
    msr     S3_4_C1_C2_2, xzr           /* HCRX_EL2 */

8:  mrs     x9, id_aa64dfr0_el1
    mov     x12, xzr
    ubfx    x11, x9, #8, #4             /* PMUVer */
    cbz     x11, 12f
    cmp     x11, #0xf                   /* an IMPLEMENTATION DEFINED PMU */
    b.eq    12f
    mrs     x12, pmcr_el0
    ubfx    x12, x12, #11, #5           /* N, as HPMN: every counter EL1's */
12: ubfx    x11, x9, #32, #4            /* PMSVer */
    cbz     x11, 13f
    orr     x12, x12, #(3 << 12)        /* E2PB: EL1 owns SPE's buffer */
13: ubfx    x11, x9, #44, #4            /* TraceBuffer */
    cbz     x11, 14f
    orr     x12, x12, #(3 << 24)        /* E2TB: EL1 owns the trace buffer */
14: msr     mdcr_el2, x12

    ubfx    x9, x10, #24, #4            /* GIC system registers */
    cbz     x9, 10f
    mrs     x9, S3_4_C12_C9_5           /* ICC_SRE_EL2 */
    mov     x11, #0x9                   /* SRE | Enable */
    orr     x9, x9, x11
    msr     S3_4_C12_C9_5, x9
    isb

10: mov     x9, #0x0838
    movk    x9, #0x30C5, lsl #16
    msr     sctlr_el1, x9
    mov     x9, #0x3C5                  /* EL1h, DAIF masked */
    msr     spsr_el2, x9
    adr     x9, 9f
    msr     elr_el2, x9
    eret
9:
.endm
