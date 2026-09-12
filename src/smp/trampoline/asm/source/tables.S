/*
 * Every descriptor below carries the Accessed bit (bit 40) already set, which
 * is what the 0x..9B and 0x..93 access bytes mean rather than 0x..9A and
 * 0x..92. Loading a segment register makes the processor set that bit itself,
 * and that is a write into whichever page holds the GDT. This GDT lives inside
 * the trampoline page, which the BSP identity-maps read plus execute and not
 * write so the low mapping satisfies W^X. The write therefore faulted, and an
 * AP that faults has no IDT, so the #PF became a #DF and then a triple fault
 * that reset the machine before the BSP's startup timeout could fire.
 *
 * Presetting the bit is the fix that keeps W^X: the processor finds nothing to
 * update and issues no write. Mapping the page writable would work too and
 * would hand every AP a writable, executable low page during bring-up.
 */
.balign 8
nonos_ap_gdt32:
    .quad   0x0000000000000000
    .quad   0x00CF9B000000FFFF
    .quad   0x00CF93000000FFFF
nonos_ap_gdt32_end:

nonos_ap_gdt32_ptr:
    .word   nonos_ap_gdt32_end - nonos_ap_gdt32 - 1
    .long   TRAMP_BASE + OFF_GDT32

.balign 8
nonos_ap_gdt64:
    .quad   0x0000000000000000
    .quad   0x00AF9B000000FFFF
    .quad   0x00AF93000000FFFF
nonos_ap_gdt64_end:

nonos_ap_gdt64_ptr:
    .word   nonos_ap_gdt64_end - nonos_ap_gdt64 - 1
    .quad   TRAMP_BASE + OFF_GDT64

.balign 8
nonos_ap_trampoline_pml4:
    .quad   0
nonos_ap_trampoline_stack:
    .quad   0
nonos_ap_trampoline_entry:
    .quad   0
nonos_ap_trampoline_cpu_id:
    .long   0
nonos_ap_trampoline_ready:
    .long   0

.balign 16
nonos_ap_trampoline_end:
