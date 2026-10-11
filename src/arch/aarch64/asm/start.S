/* aarch64 kernel entry, as an ELF and as an arm64 Image.
 *   x0  = dtb_ptr (preserved in x19 across EL2 drop)
 *   sp  = bootloader-provided
 *   may be at EL1 or EL2; we always continue at EL1.
 * Symbols: __stack_top, __bss_start, __bss_end from linker.ld.
 * Tail-calls kernel_entry(dtb_ptr).
 *
 * The first 64 bytes are the Image header Linux's arm64 boot protocol
 * defines (Documentation/arch/arm64/booting.rst). A loader that sees the
 * magic places the image text_offset past a 2 MiB aligned RAM base and
 * hands over the device tree in x0; QEMU's -kernel does that for an Image
 * and passes nothing for an ELF. The kernel is linked at 0x4008_0000, so
 * text_offset is 0x8_0000 and the base has to be virt's 0x4000_0000. */

.section .text._start, "ax", %progbits
.balign 16
.globl _start
.type  _start, %function

_start:
    b       .L_entry                    /* code0: jump over the header */
    .long   0                           /* code1 */
    .quad   0x80000                     /* text_offset */
    .quad   __kernel_image_size         /* image_size, BSS and stack included */
    .quad   0x2                         /* flags: little endian, 4 KiB pages,
                                           placed near the base of DRAM */
    .quad   0                           /* res2 */
    .quad   0                           /* res3 */
    .quad   0                           /* res4 */
    .ascii  "ARM\x64"                   /* magic */
    .long   0                           /* res5: no PE/COFF header */

.L_entry:
    mov     x19, x0

    /* Drop to EL1 if firmware left us at EL2. SCTLR_EL1 is set to a
     * safe reset value either way. */
    mrs     x0, CurrentEL
    cmp     x0, #(2 << 2)
    b.ne    .L_at_el1

    mov     x0, #(1 << 31)
    msr     hcr_el2, x0

    /* An EL1 kernel needs EL2 to stop trapping what it uses. Firmware
     * that enters at EL2, as the Raspberry Pi's does, leaves this to the
     * kernel, and reset values are UNKNOWN:
     *   CNTHCTL_EL2 bits 1:0  EL1 reaches the physical counter and timer
     *   CNTVOFF_EL2           virtual counter equals physical
     *   CPTR_EL2 0x33ff       RES1 bits only: no FP/SIMD trap to EL2
     *   HSTR_EL2              no trapped system registers
     *   ICC_SRE_EL2           on a GICv3, SRE plus Enable, or EL1 cannot
     *                         turn on its own system-register interface */
    mrs     x0, cnthctl_el2
    orr     x0, x0, #3
    msr     cnthctl_el2, x0
    msr     cntvoff_el2, xzr
    mov     x0, #0x33ff
    msr     cptr_el2, x0
    msr     hstr_el2, xzr
    mrs     x0, id_aa64pfr0_el1
    ubfx    x0, x0, #24, #4             /* GIC: system registers present */
    cbz     x0, .L_el2_no_gicv3
    mrs     x0, S3_4_C12_C9_5           /* ICC_SRE_EL2 */
    mov     x1, #0x9                    /* SRE | Enable */
    orr     x0, x0, x1
    msr     S3_4_C12_C9_5, x0
    isb
.L_el2_no_gicv3:

    /* SCTLR_EL1 reset: MMU/I/D off, RES1 bits 11/20/22/23/28/29. */
    mov     x0, #0x0838
    movk    x0, #0x30C5, lsl #16
    msr     sctlr_el1, x0

    /* SPSR_EL2: return to EL1h with DAIF masked. */
    mov     x0, #0x3C5
    msr     spsr_el2, x0

    adr     x0, .L_at_el1
    msr     elr_el2, x0
    eret

.L_at_el1:
    mov     x0, #0x0838
    movk    x0, #0x30C5, lsl #16
    msr     sctlr_el1, x0
    isb

    /* BSP-only continuation. Secondaries wfe; they come up via PSCI. */
    mrs     x0, mpidr_el1
    and     x0, x0, #0xFF
    cbnz    x0, .L_secondary

    adrp    x0, __stack_top
    add     x0, x0, :lo12:__stack_top
    mov     sp, x0

    adrp    x1, __bss_start
    add     x1, x1, :lo12:__bss_start
    adrp    x2, __bss_end
    add     x2, x2, :lo12:__bss_end
.L_bss:
    cmp     x1, x2
    b.hs    .L_bss_done
    str     xzr, [x1], #8
    b       .L_bss
.L_bss_done:

    /* Vectors before any Rust runs, so an exception from here on is
     * reported by name instead of taken through whatever VBAR_EL1 held
     * at reset. FP/SIMD with them: the handlers are compiled code and
     * may use the vector registers, which trap while CPACR_EL1.FPEN is
     * clear. After the BSS clear, because the handlers read BSS. */
    mov     x0, #(3 << 20)
    msr     cpacr_el1, x0
    adrp    x0, __aarch64_vectors_el1
    add     x0, x0, :lo12:__aarch64_vectors_el1
    msr     vbar_el1, x0
    isb

    mov     x0, x19
    bl      kernel_entry

.L_hang:
    wfi
    b       .L_hang

.L_secondary:
    wfe
    b       .L_secondary

.size _start, . - _start


/* AP entry. PSCI CPU_ON arranges for this hart to land here with
 *   x0 = context_id (= stack_top picked by the kernel)
 * and the EL the boot CPU was running at. SCTLR_EL1 is reset to the
 * same safe value as on BSP entry and VBAR_EL1 is installed here;
 * per-CPU GIC / timer setup happens in Rust before IRQs are unmasked. */

.section .text._aarch64_secondary_start, "ax", %progbits
.balign 16
.globl _aarch64_secondary_start
.type  _aarch64_secondary_start, %function

_aarch64_secondary_start:
    mov     sp, x0

    mov     x1, #0x0838
    movk    x1, #0x30C5, lsl #16
    msr     sctlr_el1, x1
    isb

    /* Vectors and FP/SIMD before any Rust runs, as on the boot CPU.
     * x0 still carries the context id, so x1 does the work. */
    mov     x1, #(3 << 20)
    msr     cpacr_el1, x1
    adrp    x1, __aarch64_vectors_el1
    add     x1, x1, :lo12:__aarch64_vectors_el1
    msr     vbar_el1, x1
    isb

    bl      aarch64_ap_entry

.L_ap_hang:
    wfi
    b       .L_ap_hang

.size _aarch64_secondary_start, . - _aarch64_secondary_start
