/* NONOS Operating System (AGPL-3.0-or-later)
 * Stand-ins for the loader's system-register macros (asm/sysregs.S), so its
 * walk runs at EL0. x19 points at a fixture: CLIDR_EL1 at 0, ID_AA64MMFR2_EL1
 * at 8, then CCSIDR_EL1 for levels 1..7 from 16. x20 is the next free slot of
 * the operand log, x22 its end; an operand past the end bumps x24 instead. */
.macro read_clidr reg
    ldr     \reg, [x19]
.endm
.macro read_mmfr2 reg
    ldr     \reg, [x19, #8]
.endm
.macro read_ccsidr sel, out
    lsl     x23, \sel, #2               /* sel = level << 1: 8 bytes a level */
    add     x23, x23, x19
    ldr     \out, [x23, #16]
.endm
.macro op_cisw reg
    cmp     x20, x22
    b.hs    70f
    str     \reg, [x20], #8
    b       71f
70: add     x24, x24, #1
71:
.endm
