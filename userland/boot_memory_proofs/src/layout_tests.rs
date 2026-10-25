// NONOS Operating System (AGPL-3.0-or-later)
//! Real board layouts: where the pool lands, and that it is never the image.

use crate::dma_place::place;

const MIB: u64 = 1 << 20;
const FLOOR: u64 = 16 * MIB;
const WANT: u64 = 8 * MIB;

fn meets(at: u64, len: u64, (a, b): (u64, u64)) -> bool {
    a < at + len && at < b
}

#[test]
fn virt_carves_the_pool_past_the_image_never_over_it() {
    let ram = (0x4000_0000, 0x4000_0000 + 2048 * MIB);
    let image = (0x4008_0000, 0x4541_0000);
    let tree = (0x4000_0000, 0x4010_0000);
    let main = (image.1, ram.1);
    // Outside the allocator's span: only the space below the image, which
    // the tree fills and which is too small anyway.
    assert_eq!(place(&[ram], &[main, image, tree], FLOOR, WANT), None);
    let carved = place(&[main], &[image, tree], FLOOR, WANT).unwrap();
    assert_eq!(carved, image.1);
    assert!(!meets(carved, WANT, image) && !meets(carved, WANT, tree));
    // What the pool used to be given: the bottom of RAM, over the image.
    assert!(meets(0x4000_0000, WANT, image), "the old placement");
}

#[test]
fn ram_at_zero_keeps_the_pool_off_the_first_sixteen_mib() {
    let ram = (0, 2048 * MIB);
    let image = (0x8_0000, 0x50_0000);
    let main = (image.1, ram.1);
    assert_eq!(place(&[ram], &[main, image], FLOOR, WANT), None);
    assert_eq!(place(&[main], &[image], FLOOR, WANT), Some(FLOOR));
}

#[test]
fn a_low_region_the_allocator_does_not_take_is_used_first() {
    let low = (0, 768 * MIB);
    let high = (0x4000_0000, 0x1_4000_0000);
    let image = (0x4008_0000, 0x4541_0000);
    let main = (image.1, high.1);
    assert_eq!(place(&[low, high], &[main, image], FLOOR, WANT), Some(FLOOR));
    let reserved = (FLOOR, FLOOR + MIB + 1);
    let past = place(&[low, high], &[main, image, reserved], FLOOR, WANT);
    assert_eq!(past, Some(FLOOR + MIB + 0x1000), "past a reservation, page aligned");
}

#[test]
fn nothing_at_or_past_four_gib_and_nothing_that_wraps() {
    let high = (0xffc0_0000, 0x1_8000_0000);
    assert_eq!(place(&[high], &[], 0, WANT), None);
    assert_eq!(place(&[(0xff80_0000, 0x1_0000_0000)], &[], 0, WANT), Some(0xff80_0000));
    assert_eq!(place(&[(u64::MAX - 0xfff, u64::MAX)], &[], 0, 0x1000), None);
    assert_eq!(place(&[(0, 1 << 30)], &[(0, u64::MAX)], 0, 0x1000), None);
    assert_eq!(place(&[(0, 1 << 30)], &[], 0, 0), None);
}
