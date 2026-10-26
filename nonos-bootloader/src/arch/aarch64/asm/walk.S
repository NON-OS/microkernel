/* NØNOS Operating System
 * Copyright (C) 2026 NØNOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

/* Clean and invalidate every data or unified cache from level 1 up to the
 * level of coherency, by set and way (Arm ARM, DC CISW and CCSIDR_EL1).
 * Registers only: no load or store, so it runs with the data cache off.
 * Uses x0 and x3..x17. The operand for one line is
 *   way << clz32(ways - 1) | set << log2(line bytes) | level << 1
 * and CCSIDR_EL1 has two layouts, the wide one when ID_AA64MMFR2_EL1.CCIDX
 * (bits 23:20) is not zero. A DSB ends each level, so what it wrote back has
 * reached the next level before that one is cleaned. */
.macro clean_all_by_set_way
    read_clidr x0
    ubfx    x3, x0, #24, #3             /* LoC */
    cbz     x3, 97f
    read_mmfr2 x4
    ubfx    x4, x4, #20, #4             /* CCIDX */
    mov     x5, #0                      /* level, counting from 0 */
90: add     x6, x5, x5, lsl #1
    lsr     x7, x0, x6
    and     x7, x7, #7                  /* Ctype of this level */
    cmp     x7, #2
    b.lo    96f                         /* none, or instructions only */
    lsl     x8, x5, #1
    read_ccsidr x8, x9
    and     x10, x9, #7
    add     x10, x10, #4                /* log2 of the line in bytes */
    cbz     x4, 91f
    ubfx    x11, x9, #3, #21            /* ways - 1, wide layout */
    ubfx    x12, x9, #32, #24           /* sets - 1 */
    b       92f
91: ubfx    x11, x9, #3, #10            /* ways - 1 */
    ubfx    x12, x9, #13, #15           /* sets - 1 */
92: clz     w13, w11                    /* one way: 32, and way 0 stays 0 */
    mov     x14, x11
94: mov     x15, x12
95: lsl     x16, x14, x13
    lsl     x17, x15, x10
    orr     x16, x16, x17
    orr     x16, x16, x8
    op_cisw x16
    subs    x15, x15, #1
    b.hs    95b
    subs    x14, x14, #1
    b.hs    94b
    dsb     sy                          /* this level done before the next */
96: add     x5, x5, #1
    cmp     x5, x3
    b.lo    90b
97: dsb     sy
.endm
