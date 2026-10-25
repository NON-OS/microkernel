// NONOS Operating System (AGPL-3.0-or-later)
//! The image's bounds are physical even once the kernel runs from TTBR1.
//! Memory setup runs after the move, where a linker symbol's address is its
//! high-half alias; RAM from the device tree is physical. Compared as they
//! came, the image avoided nothing: the allocator started at the bottom of
//! RAM and the DMA pool, made uncached, held the kernel's own page tables
//! (a write fault in `uncached::cut` on QEMU virt at 885f21662b).

use crate::dma_place::place;

const FLOOR: u64 = 0x0100_0000;
const WANT: u64 = 8 << 20;
/// QEMU virt with 512 MiB, and the image as that boot linked it.
const RAM: (u64, u64) = (0x4000_0000, 0x6000_0000);
const IMAGE: (u64, u64) = (0x4008_0000, 0x4581_0000);
/// The direct map's offset, `KERNEL_SPACE_START`.
const HIGH: u64 = 0xFFFF_8000_0000_0000;

fn meets(at: u64, len: u64, (a, b): (u64, u64)) -> bool {
    a < at + len && at < b
}

#[test]
fn a_high_half_image_range_avoids_nothing_and_the_pool_lands_on_the_kernel() {
    let high = (IMAGE.0 + HIGH, IMAGE.1 + HIGH);
    let main = (RAM.0, RAM.1);
    let at = place(&[main], &[high], FLOOR, WANT).unwrap();
    assert!(meets(at, WANT, IMAGE), "the defect this guards: pool at {at:#x}");
}

#[test]
fn the_physical_image_range_keeps_the_pool_clear_of_it() {
    let main = (IMAGE.1, RAM.1);
    let at = place(&[main], &[IMAGE], FLOOR, WANT).unwrap();
    assert!(!meets(at, WANT, IMAGE) && at >= IMAGE.1, "pool at {at:#x}");
}

#[test]
fn bounds_turns_both_symbols_physical() {
    let src = std::fs::read_to_string("../../src/arch/aarch64/boot/memory/image.rs").unwrap();
    let code: String = src.lines().filter(|l| !l.trim_start().starts_with("//")).collect();
    for sym in ["__kernel_image_start", "__kernel_image_end"] {
        let use_ = format!("pa((&raw const {sym}) as u64)");
        assert!(code.contains(&use_), "{sym} not turned physical");
    }
    assert!(code.contains("let pa = crate::arch::aarch64::mmu::image_pa;"));
}

#[test]
fn the_image_map_bounds_are_physical_too() {
    // `make_block_uncached` sets a physical block against them after the
    // move; given high-half bounds, its overlap guard could never refuse.
    let src = std::fs::read_to_string("../../src/arch/aarch64/mmu/image_bounds.rs").unwrap();
    let code: String = src.lines().filter(|l| !l.trim_start().starts_with("//")).collect();
    for sym in
        ["__kernel_image_start", "__kernel_rw_start", "__kernel_text_start", "__kernel_text_end"]
    {
        assert!(code.contains(&format!("pa((&raw const {sym}) as u64)")), "{sym} not physical");
    }
}
