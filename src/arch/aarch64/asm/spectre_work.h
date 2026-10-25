/* The speculation work on entry from EL0, as security::spectre decided for
 * the CPUs (entry_word.rs), shared by the vectors (SPECTRE_FROM_EL0 in
 * vectors.S) and the EPD1 trampoline (epd1_trampoline.S), so the two can
 * never decide differently. `word` holds the entry word, nonzero: bit 3
 * CLRBHB, bits 15:8 the history loop's branch count, bits 1:0 a firmware
 * call (1 ARCH_WORKAROUND_1, 2 _3) by SMC, or HVC where bit 2 is set.
 * `tmp` is a second free register. Neither may be w0, which carries the
 * function id, and x0-x3 are the call's to change (SMCCC 1.1, DEN0028).
 * CLRBHB is HINT #22, a NOP where not implemented. The loop, `k` taken
 * branches then DSB and ISB, and the function ids are from Arm's
 * Spectre-BHB guidance and SMCCC. The work ends at label 9. */
.macro SPECTRE_WORK word, tmp
    tbz     \word, #3, 1f
    hint    #22                         /* CLRBHB */
    isb
1:  ubfx    \tmp, \word, #8, #8
    cbz     \tmp, 3f
2:  b       . + 4
    subs    \tmp, \tmp, #1
    b.ne    2b
    dsb     nsh
    isb
3:  and     \tmp, \word, #3
    cbz     \tmp, 9f
    mov     w0, #0x8000                 /* SMCCC_ARCH_WORKAROUND_1 */
    cmp     \tmp, #2
    b.ne    4f
    mov     w0, #0x3fff                 /* SMCCC_ARCH_WORKAROUND_3 */
4:  movk    w0, #0x8000, lsl #16
    tbnz    \word, #2, 5f
    smc     #0
    b       9f
5:  hvc     #0
9:
.endm
