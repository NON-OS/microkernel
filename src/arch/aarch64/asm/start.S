/* aarch64 kernel entry, as an ELF and as an arm64 Image.
 *   x0  = dtb_ptr (preserved in x19 across EL2 drop)
 *   x1  = the NONOS loader's handoff, physical, or zero for none
 *         (preserved in x20; the arm64 boot protocol has x1 zero)
 *   sp  = bootloader-provided
 *   may be at EL1 or EL2; we always continue at EL1.
 * Symbols: __stack_top, __bss_start, __bss_end from linker.ld.
 * Tail-calls kernel_entry(dtb_ptr, handoff_phys).
 *
 * The first 64 bytes are the Image header Linux's arm64 boot protocol
 * defines (Documentation/arch/arm64/booting.rst). A loader that sees the
 * magic places the image text_offset past a 2 MiB aligned RAM base and
 * hands over the device tree in x0; QEMU's -kernel does that for an Image
 * and passes nothing for an ELF. The kernel is linked at 0x4008_0000, so
 * text_offset is 0x8_0000; flags bit 3 says any 2 MiB aligned base will
 * do, which the relocation below makes true. */

#include "el2.h"
#include "mmu_on.h"

.section .text._start, "ax", %progbits
.balign 16
.globl _start
.type  _start, %function

_start:
    b       .L_entry                    /* code0: jump over the header */
    .long   0                           /* code1 */
    .quad   0x80000                     /* text_offset */
    .quad   __kernel_image_size         /* image_size, BSS and stack included */
    .quad   0xa                         /* flags: little endian, 4 KiB pages,
                                           any 2 MiB aligned base in DRAM */
    .quad   0                           /* res2 */
    .quad   0                           /* res3 */
    .quad   0                           /* res4 */
    .ascii  "ARM\x64"                   /* magic */
    .long   0                           /* res5: no PE/COFF header */

.L_entry:
    mov     x19, x0
    mov     x20, x1

    drop_from_el2

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

    /* Every pointer stored in data still holds nothing (boot/relocate):
     * apply .rela.dyn before anything reads one, Rust included. */
    adrp    x0, __rela_start
    add     x0, x0, :lo12:__rela_start
    adrp    x1, __rela_end
    add     x1, x1, :lo12:__rela_end
    movz    x2, #:abs_g3:__kernel_link_base
    movk    x2, #:abs_g2_nc:__kernel_link_base
    movk    x2, #:abs_g1_nc:__kernel_link_base
    movk    x2, #:abs_g0_nc:__kernel_link_base
    adrp    x3, __kernel_image_start
    add     x3, x3, :lo12:__kernel_image_start
    movz    x4, #:abs_g3:__kernel_image_size
    movk    x4, #:abs_g2_nc:__kernel_image_size
    movk    x4, #:abs_g1_nc:__kernel_image_size
    movk    x4, #:abs_g0_nc:__kernel_image_size
    bl      aarch64_relocate
    cbnz    x0, .L_unrelocatable

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

    /* The relocations and the BSS were written with the data cache off,
     * straight to memory. A line the loader left in the cache would
     * shadow them once it is on, so drop every line of that range
     * (Arm ARM, DDI 0487, DC IVAC and cache-disabled accesses). */
    mrs     x3, ctr_el0
    ubfx    x3, x3, #16, #4             /* DminLine, log2 words */
    mov     x4, #4
    lsl     x4, x4, x3
    sub     x5, x4, #1
    adrp    x1, __kernel_rw_start
    add     x1, x1, :lo12:__kernel_rw_start
    bic     x1, x1, x5
    adrp    x2, __bss_end
    add     x2, x2, :lo12:__bss_end
.L_inval:
    dc      ivac, x1
    add     x1, x1, x4
    cmp     x1, x2
    b.lo    .L_inval
    dsb     sy

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
    mov     x1, x20
    bl      kernel_entry

.L_hang:
    wfi
    b       .L_hang

.L_secondary:
    wfe
    b       .L_secondary

/* boot/relocate refused the image: loaded where its layout breaks, or a
 * relocation it does not apply. Nothing can report it yet. */
.L_unrelocatable:
    wfe
    b       .L_unrelocatable

.size _start, . - _start


/* AP entry. PSCI CPU_ON arranges for this hart to land here with
 *   x0 = context_id (= stack_top picked by the kernel)
 * at the EL PSCI chooses, which is EL2 on a CPU whose firmware entered
 * the boot CPU there. SCTLR_EL1 is reset to the
 * same safe value as on BSP entry and VBAR_EL1 is installed here;
 * per-CPU GIC / timer setup happens in Rust before IRQs are unmasked. */

.section .text._aarch64_secondary_start, "ax", %progbits
.balign 16
.globl _aarch64_secondary_start
.type  _aarch64_secondary_start, %function

_aarch64_secondary_start:
    /* Before the stack: at EL2 it would be SP_EL2, not the CPU's own. */
    drop_from_el2
    mov     sp, x0

    mov     x1, #0x0838
    movk    x1, #0x30C5, lsl #16
    msr     sctlr_el1, x1
    isb

    /* The boot CPU's translation and caching, before any Rust (mmu_on.h).
     * Leaves the kernel's distance above its load address in x7. */
    MMU_ON_FROM_CONTEXT

    /* Vectors and FP/SIMD before any Rust runs, as on the boot CPU.
     * x0 still carries the context id, so x1 does the work. Both the
     * vectors and the Rust entry are taken where the kernel runs: at its
     * high alias once it has moved there (boot/high.rs), x7 above here.
     * The stack in x0 is already that address. BR through x16 is the
     * register form `bti c` admits. */
    mov     x1, #(3 << 20)
    msr     cpacr_el1, x1
    adrp    x1, __aarch64_vectors_el1
    add     x1, x1, :lo12:__aarch64_vectors_el1
    add     x1, x1, x7
    msr     vbar_el1, x1
    isb

    mov     x29, xzr
    adrp    x16, aarch64_ap_entry
    add     x16, x16, :lo12:aarch64_ap_entry
    add     x16, x16, x7
    br      x16

.L_ap_hang:
    wfi
    b       .L_ap_hang

.size _aarch64_secondary_start, . - _aarch64_secondary_start
