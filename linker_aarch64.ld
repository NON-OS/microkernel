ENTRY(_start)
OUTPUT_ARCH(aarch64)
OUTPUT_FORMAT(elf64-littleaarch64)

/* aarch64 kernel image, a static position-independent executable. Linked
 * at QEMU virt's RAM + 512 KiB, and runs wherever a loader puts it at a
 * 2 MiB aligned base + 512 KiB: the entry stub applies .rela.dyn, every
 * entry R_AARCH64_RELATIVE, before anything reads a pointer from data
 * (boot/relocate). It runs identity-mapped where it was loaded until
 * mmu::init_mmu installs the high-half mapping. */

PHDRS {
    text   PT_LOAD FLAGS(5);  /* r-x */
    rodata PT_LOAD FLAGS(4);  /* r-- */
    data   PT_LOAD FLAGS(6);  /* rw- */
    /* The static PIE's dynamic table, inside the data segment: what a tool
       or loader finds the relocations through (ELF gABI, PT_DYNAMIC). */
    dynamic PT_DYNAMIC;
}

SECTIONS {
    . = 0x40080000;
    __kernel_image_start = .;
    /* The address this image is linked at, as a constant rather than a
       position: the entry stub subtracts it from where it finds itself. */
    __kernel_link_base = ABSOLUTE(__kernel_image_start);

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
        __kernel_rodata_end = .;
    } :rodata

    .rela.dyn ALIGN(8) : {
        __rela_start = .;
        *(.rela.dyn .rela.*)
        __rela_end = .;
    } :rodata
    .dynsym : { *(.dynsym) } :rodata
    .dynstr : { *(.dynstr) } :rodata
    .hash : { *(.hash) } :rodata
    .gnu.hash : { *(.gnu.hash) } :rodata

    /* Writable data starts on a 2MB boundary so the boot identity map, which
       uses 2MB blocks, can keep text read only and executable without also
       making .data and .bss read only. */
    . = ALIGN(0x200000);
    __kernel_rw_start = .;

    .data ALIGN(0x1000) : {
        __kernel_data_start = .;
        *(.data .data.*)
        __kernel_data_end = .;
    } :data
    .dynamic : { *(.dynamic) } :data :dynamic
    .got : { *(.got) *(.got.plt) } :data

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
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
        __kernel_bss_end = .;
    } :data

    /* 2MB so the boot identity map, which uses 2MB blocks, can mark the text
       below executable without making the stack executable with it. */
    . = ALIGN(0x200000);
    __stack_bottom = .;
    . += 64K;
    __stack_top = .;

    __kernel_image_end = .;
    _end = .;
    /* The Image header's image_size: everything a loader must leave free. */
    __kernel_image_size = __kernel_image_end - __kernel_image_start;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.comment)
        *(.note*)
        *(.debug*)
        *(.plt*)
        *(.interp)
    }
}
