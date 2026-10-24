// NONOS Operating System (AGPL-3.0-or-later)
//! The order the boot path keeps, read from the kernel's source: x1 carried
//! from the first instruction to `kernel_entry`, the handoff copied while the
//! MMU is off, and refused unless whole and current.

use crate::source::{code, order, raw};

#[test]
fn x1_reaches_kernel_entry() {
    let start = raw("src/arch/aarch64/asm/start.S");
    order(
        &start,
        &["mov     x19, x0", "mov     x20, x1", "drop_from_el2", "bl      aarch64_relocate"],
    );
    order(&start, &["mov     x0, x19", "mov     x1, x20", "bl      kernel_entry"]);
    let entry = code("arch/aarch64/boot/entry.rs");
    order(
        &entry,
        &[
            "pub extern \"C\" fn kernel_entry(dtb_ptr: u64, handoff_phys: u64) -> ! {",
            "super::dtb_adapter::populate(dtb_ptr, &mut info);",
            "super::refuse(b\"device tree\", reason);",
            "if let Err(reason) = super::loader::take(handoff_phys, &info) {",
            "super::refuse(b\"loader handoff\", reason);",
            "super::init(&info);",
        ],
    );
}

#[test]
fn copied_only_when_whole_and_current() {
    let take = code("arch/aarch64/boot/loader/take.rs");
    order(
        &take,
        &[
            "if phys == 0 {",
            "return Ok(());",
            "if !placed(regions, phys, len) {",
            "return Err(",
            "mmu::pointer::ram::<BootHandoffV1>(phys)",
            "read_volatile(at)",
            "if handoff.magic != HANDOFF_MAGIC {",
            "return Err(",
            "if handoff.version != HANDOFF_VERSION || u64::from(handoff.size) != len {",
            "return Err(",
            "COPY.call_once(|| handoff);",
        ],
    );
}
