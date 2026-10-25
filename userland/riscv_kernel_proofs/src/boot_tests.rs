// NONOS Operating System (AGPL-3.0-or-later)
//! The riscv64 boot path, read from the kernel's source: what it keeps out of
//! the allocator, and how it maps the image.

use crate::source::{code, order};

#[test]
fn every_reserved_range_is_kept_from_the_tree() {
    let m = code("arch/riscv64/boot/dtb_adapter/memory.rs");
    order(
        &m,
        &[
            "MemoryType::Available",
            "info.reserved.add(info.dtb_base, info.dtb_size)",
            "fdt.reservations()",
            "fdt.reserved_memory(&mut firmware)",
        ],
    );
    let init = code("arch/riscv64/boot/memory.rs");
    order(
        &init,
        &[
            "phys::init(",
            "for &(a, b) in info.reserved.ranges()",
            "phys::reserve(a, b)",
            "phys init OK",
        ],
    );
}

#[test]
fn a_tree_that_cannot_be_read_stops_the_boot() {
    order(
        &code("arch/riscv64/boot/entry.rs"),
        &[
            "let parsed =",
            "super::init(&info);",
            "if !parsed {",
            "could not be read; stopping",
            "halt_loop()",
            "init_bootstrap();",
        ],
    );
}

#[test]
fn the_image_is_mapped_by_section() {
    order(
        &code("arch/riscv64/mmu/image_map.rs"),
        &[
            "pages(text, text_end, PageAttributes::kernel_code());",
            "pages(ro, ro_end, PageAttributes::kernel_rodata());",
            "pages(ro_end, end, PageAttributes::kernel_data());",
        ],
    );
    let boot = code("arch/riscv64/mmu/boot_map.rs");
    order(&boot, &["map_region(region);", "map_image_and_devices(boot_info)"]);
}
