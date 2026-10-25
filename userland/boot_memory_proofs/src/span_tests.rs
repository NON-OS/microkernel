// NONOS Operating System (AGPL-3.0-or-later)
//! The frame allocator's span never holds a frame of the kernel image: the
//! virt layouts this tree boots, the edges, and a sweep of small layouts.

use crate::span::span_start;

const RAM: (u64, u64) = (0x4000_0000, 0x6000_0000);
const IMAGE: (u64, u64) = (0x4008_0000, 0x4581_0000);

fn clear((start, end): (u64, u64), image: (u64, u64)) -> bool {
    let from = span_start((start, end), image);
    from >= end || image.1 <= from || image.0 >= end
}

#[test]
fn on_virt_the_span_starts_past_the_image() {
    assert_eq!(span_start(RAM, IMAGE), IMAGE.1);
}

#[test]
fn an_image_outside_the_region_leaves_it_whole() {
    assert_eq!(span_start(RAM, (0x8000_0000, 0x8100_0000)), RAM.0);
    assert_eq!(span_start(RAM, (0x1000_0000, RAM.0)), RAM.0);
    assert_eq!(span_start(RAM, (RAM.1, RAM.1 + 0x1000)), RAM.0);
}

#[test]
fn an_image_over_either_edge_is_kept_out() {
    assert_eq!(span_start(RAM, (RAM.0 - 0x1000, RAM.0 + 0x1000)), RAM.0 + 0x1000);
    // Reaching the end: nothing is left, rather than the image's frames.
    assert!(span_start(RAM, (RAM.1 - 0x1000, RAM.1 + 0x1000)) >= RAM.1);
}

#[test]
fn no_small_layout_puts_a_frame_of_the_image_in_the_span() {
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..200_000 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let (a, b, c, d) = (seed % 64, (seed >> 8) % 64, (seed >> 16) % 64, (seed >> 24) % 64);
        let region = (a.min(b) * 0x1000, a.max(b) * 0x1000 + 0x1000);
        let image = (c.min(d) * 0x1000, c.max(d) * 0x1000 + 0x1000);
        assert!(clear(region, image), "region {region:x?} image {image:x?}");
    }
}
