// NONOS Operating System (AGPL-3.0-or-later)
//! The frame allocator's span on riscv64: RAM aligned inward, and past the
//! kernel image wherever the two meet. QEMU virt's layout, its edges, and a
//! sweep of small layouts in which the span never holds a frame of the image.

use crate::memory_span::{inward, span_start};

const PAGE: u64 = 0x1000;

#[test]
fn virt_ram_starts_past_the_image() {
    let ram = inward(0x8000_0000, 0x2000_0000).unwrap();
    assert_eq!(ram, (0x8000_0000, 0xA000_0000));
    assert_eq!(span_start(ram, (0x8020_0000, 0x804A_3010)), 0x804A_4000, "page aligned up");
}

#[test]
fn an_image_elsewhere_leaves_the_region_whole() {
    assert_eq!(span_start((0x8000_0000, 0x9000_0000), (0x9000_0000, 0x9010_0000)), 0x8000_0000);
    assert_eq!(span_start((0x8000_0000, 0x9000_0000), (0x7000_0000, 0x8000_0000)), 0x8000_0000);
    // An image ending exactly at the region's start, or starting at its end,
    // does not meet it.
    assert_eq!(span_start((0x8000_0000, 0x9000_0000), (0x9000_0000, 0x9000_1000)), 0x8000_0000);
}

#[test]
fn inward_refuses_what_is_too_small_or_wraps() {
    assert_eq!(inward(0x1001, 0xFFF), None);
    assert_eq!(inward(0x1001, 0x2000), Some((0x2000, 0x3000)));
    assert_eq!(inward(0x1000, 0xFFF), None, "less than a page is nothing");
    assert_eq!(inward(u64::MAX - 0x10, 0x100), None);
}

#[test]
fn no_layout_puts_an_image_frame_in_the_span() {
    for rs in 0..12u64 {
        for rl in 1..12u64 {
            for is in 0..14u64 {
                for il in 1..6u64 {
                    let (start, end) = (rs * PAGE, (rs + rl) * PAGE);
                    let image = (is * PAGE + 0x10, (is + il) * PAGE + 0x10);
                    let from = span_start((start, end), image);
                    let clear = from >= end || image.1 <= from || image.0 >= end;
                    assert!(
                        clear && from.is_multiple_of(PAGE),
                        "{start:#x}..{end:#x} {image:x?} -> {from:#x}"
                    );
                }
            }
        }
    }
}
