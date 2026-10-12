/* Entry for a CPU firmware parked on a spin table.
 *
 * Such a CPU polls its cpu-release-addr and branches to the address the
 * kernel writes there, with no argument (Devicetree Specification v0.4,
 * 5.4.2: the spin-table enable method). PSCI's CPU_ON carries the stack in
 * x0; here the CPU looks its stack up by its own MPIDR affinity in
 * AARCH64_SPIN_STACKS (boot/multicore/spin_table.rs), which the boot CPU
 * wrote to the point of coherency before the release: this CPU runs with
 * its MMU and data cache off and reads memory, not anyone's cache.
 *
 * The table is 64 pairs of (affinity, stack top); a zero stack is an empty
 * slot. A CPU with no slot was released by mistake and waits for ever,
 * which is better than running on a stack another CPU owns. */

.section .text._aarch64_spin_table_start, "ax", %progbits
.balign 16
.globl _aarch64_spin_table_start
.type  _aarch64_spin_table_start, %function

_aarch64_spin_table_start:
    mrs     x1, mpidr_el1
    and     x2, x1, #0xffffff           /* Aff2, Aff1, Aff0 */
    ubfx    x1, x1, #32, #8             /* Aff3 */
    orr     x1, x2, x1, lsl #32         /* the device tree's cpu/reg shape */
    adrp    x2, AARCH64_SPIN_STACKS
    add     x2, x2, :lo12:AARCH64_SPIN_STACKS
    mov     x3, #64
1:  ldp     x4, x0, [x2], #16
    cmp     x4, x1
    b.ne    2f
    cbz     x0, 2f
    b       _aarch64_secondary_start    /* b reaches further than cbnz */
2:  subs    x3, x3, #1
    b.ne    1b
3:  wfe
    b       3b

.size _aarch64_spin_table_start, . - _aarch64_spin_table_start
