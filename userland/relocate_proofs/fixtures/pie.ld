/* NONOS Operating System (AGPL-3.0-or-later)
 * The kernel's shape in miniature: linked at 0x4008_0000, writable data on
 * its own 2 MiB block, .rela.dyn kept in the loaded image. */
ENTRY(_start)
SECTIONS {
    . = 0x40080000;
    .text : { *(.text*) }
    .rodata : { *(.rodata*) }
    .rela.dyn : { *(.rela.dyn) }
    .dynsym : { *(.dynsym) }
    .dynstr : { *(.dynstr) }
    .hash : { *(.hash) }
    .gnu.hash : { *(.gnu.hash) }
    . = ALIGN(0x200000);
    .data : { *(.data*) }
    .dynamic : { *(.dynamic) }
    .got : { *(.got) }
    /DISCARD/ : { *(.comment) *(.note*) }
}
