// NONOS Operating System (AGPL-3.0-or-later)
//! Which memory type an aarch64 leaf names, read from the entry's AttrIndx
//! and SH bits as the walker reads them, against the MAIR slot table.

use crate::arch::aarch64::mmu::MemoryType;
use crate::descriptor::aarch64::leaf;
use crate::descriptor::flags::{DEVICE, NO_CACHE, PRESENT, USER, WRITABLE, WRITE_THROUGH};

const PA: u64 = 0x4100_0000;
const SH: u64 = 0b11 << 8;

fn attr(entry: u64) -> u64 {
    (entry >> 2) & 0b111
}

#[test]
fn registers_are_device_and_uncached_ram_is_normal_non_cacheable() {
    let base = PRESENT | WRITABLE | USER;
    let cases = [
        (0, MemoryType::NormalWB),
        (NO_CACHE, MemoryType::NormalNC),
        (WRITE_THROUGH, MemoryType::NormalNC),
        (NO_CACHE | WRITE_THROUGH, MemoryType::NormalNC),
        (DEVICE, MemoryType::DeviceNGnRnE),
        (NO_CACHE | WRITE_THROUGH | DEVICE, MemoryType::DeviceNGnRnE),
    ];
    for (asked, want) in cases {
        let entry = leaf(PA, base | asked);
        assert_eq!(attr(entry), want.attr_index(), "{asked:#x}");
    }
}

#[test]
fn only_normal_memory_carries_a_shareability() {
    assert_eq!(leaf(PA, PRESENT | NO_CACHE) & SH, SH, "Normal NC, inner shareable");
    assert_eq!(leaf(PA, PRESENT) & SH, SH);
    assert_eq!(leaf(PA, PRESENT | DEVICE) & SH, 0, "Device memory leaves SH to the architecture");
}

#[test]
fn device_does_not_reach_the_x86_entry_as_anything_but_an_ignored_bit() {
    // x86 takes NO_CACHE as PCD; bit 52 is one the processor ignores.
    let x = crate::descriptor::x86_64::leaf(PA, PRESENT | NO_CACHE | DEVICE);
    assert_eq!(x & 0xfff, PRESENT | NO_CACHE);
    assert_eq!(x & DEVICE, DEVICE);
    assert_eq!(DEVICE, 1 << 52);
}
