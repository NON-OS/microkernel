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
/* Loader -> kernel, the arm64 boot protocol. Caller (Rust, AAPCS64):
 *   nonos_arm_enter_kernel(entry: u64, dtb: u64, handoff: u64) -> !
 * Boot services have exited and the Rust caller has cleaned the image,
 * the device tree and the handoff to the point of coherency by address.
 * Here, with no memory touched after the data cache goes off:
 *   1. every exception masked;
 *   2. the data cache off at the current EL (SCTLR_ELx.C, bit 2);
 *   3. every cache level cleaned and invalidated by set and way, so
 *      nothing the loader wrote stays only in a cache;
 *   4. the MMU and the instruction cache off (bits 0 and 12), then the
 *      instruction cache invalidated;
 *   5. x0 = dtb, x1 = handoff, x2 = x3 = 0, and a branch to entry.
 * UEFI maps memory one to one, so this code keeps running at the same
 * address once the MMU is off. One way: nothing returns. */

/* SCTLR_EL1 or SCTLR_EL2, by CurrentEL, less the bits in \mask */
.macro sctlr_clear mask
    mov     x11, #(\mask)
    mrs     x9, CurrentEL
    cmp     x9, #(2 << 2)
    b.eq    80f
    mrs     x10, sctlr_el1
    bic     x10, x10, x11
    msr     sctlr_el1, x10
    b       81f
80: mrs     x10, sctlr_el2
    bic     x10, x10, x11
    msr     sctlr_el2, x10
81: isb
.endm

.global nonos_arm_enter_kernel
nonos_arm_enter_kernel:
    msr     daifset, #0xf
    mov     x19, x0
    mov     x20, x1
    mov     x21, x2
    sctlr_clear 0x4                     /* C */
    clean_all_by_set_way
    sctlr_clear 0x1001                  /* I, M */
    ic      iallu
    dsb     nsh
    isb
    mov     x0, x20
    mov     x1, x21
    mov     x2, xzr
    mov     x3, xzr
    br      x19
