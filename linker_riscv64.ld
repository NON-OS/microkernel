ENTRY(_start)
OUTPUT_ARCH(riscv)
OUTPUT_FORMAT(elf64-littleriscv)

/* riscv64 kernel image, linked at 0x80200000: QEMU virt's RAM base plus the
 * 2 MiB the SBI firmware keeps. The section bounds below are what the boot
 * page tables map, each with its own rights. */

PHDRS {
    text   PT_LOAD FLAGS(5);  /* r-x */
    rodata PT_LOAD FLAGS(4);  /* r-- */
    data   PT_LOAD FLAGS(6);  /* rw- */
}

SECTIONS {
    . = 0x80200000;
    __kernel_image_start = .;

    .text ALIGN(0x1000) : {
        __kernel_text_start = .;
        *(.text._start)
        *(.text .text.*)
        *(.ltext .ltext.*)
        __kernel_text_end = .;
    } :text

    .rodata ALIGN(0x1000) : {
        __kernel_rodata_start = .;
        *(.rodata .rodata.*)
        *(.lrodata .lrodata.*)
        /* RISC-V small-data anchor used by gp-relative addressing. */
        . = ALIGN(0x10);
        __global_pointer$ = . + 0x800;
        *(.sdata2 .sdata2.*)
        __kernel_rodata_end = .;
    } :rodata

    .data ALIGN(0x1000) : {
        __kernel_data_start = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        __kernel_data_end = .;
    } :data

    .nonos.manifest ALIGN(0x1000) : {
        __nonos_manifest_start = .;
        KEEP(*(.nonos.manifest))
        __nonos_manifest_end = .;
    } :data

    .nonos.sig ALIGN(8) : {
        __nonos_signature_start = .;
        KEEP(*(.nonos.sig))
        __nonos_signature_end = .;
    } :data

    .bss ALIGN(0x1000) : {
        __bss_start = .;
        __kernel_bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
        __kernel_bss_end = .;
    } :data

    . = ALIGN(0x1000);
    __stack_bottom = .;
    . += 64K;
    __stack_top = .;

    __kernel_image_end = .;
    _end = .;

    /DISCARD/ : {
        *(.eh_frame*) *(.comment) *(.note*) *(.debug*) *(.plt*)
        *(.dynamic) *(.dynsym) *(.dynstr) *(.hash) *(.gnu.hash)
        *(.rela.*) *(.got) *(.got.plt) *(.interp) *(.riscv.attributes)
    }
}
