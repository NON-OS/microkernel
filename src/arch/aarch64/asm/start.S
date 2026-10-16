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
 * text_offset is 0x8_0000; flags bit 3 says any 2 MiB aligned base will
 * do, which the relocation below makes true. */

/* Drop to EL1 if firmware, or PSCI for a secondary, entered at EL2. Every
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

    /* The boot CPU's translation and caching, before any Rust: with the
     * MMU off this CPU's stores bypass the caches the others read, and
     * its atomics act on Device memory (mmu/secondary.rs). The block was
     * written to memory before CPU_ON, so it reads right with the cache
     * off. Stale instruction lines go before the I-cache comes on. */
    adrp    x1, AARCH64_MMU_CONTEXT
    add     x1, x1, :lo12:AARCH64_MMU_CONTEXT
    ldp     x2, x3, [x1]
    ldp     x4, x5, [x1, #16]
    ldr     x6, [x1, #32]
    ldr     x7, [x1, #40]               /* the kernel's distance above its load */
    msr     mair_el1, x2
    msr     tcr_el1, x3
    msr     ttbr0_el1, x4
    msr     ttbr1_el1, x5
    isb
    tlbi    vmalle1
    ic      iallu
    dsb     nsh
    isb
    mrs     x2, sctlr_el1
    orr     x2, x2, x6
    msr     sctlr_el1, x2
    isb

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
