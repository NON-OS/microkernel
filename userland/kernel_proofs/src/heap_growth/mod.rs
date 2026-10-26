// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel heap's second region and the floor a process's mapping leaves:
//! the sizing and the refusal by their pure rules here, the kernel's use of
//! them by its source (source.rs).

#[path = "../../../../src/memory/heap/manager/sizing.rs"]
mod sizing;
mod source;

use sizing::{second_region_size, user_page_refused};

const MIB: usize = 1024 * 1024;
const MIN: usize = 32 * MIB;
const MAX: usize = 256 * MIB;
const PAGE: usize = 4096;

#[test]
fn the_region_is_an_eighth_of_ram_within_its_bounds_in_whole_pages() {
    assert_eq!(second_region_size(1 << 30, MIN, MAX, PAGE), 128 * MIB, "1 GiB gives an eighth");
    assert_eq!(second_region_size(8 << 30, MIN, MAX, PAGE), MAX, "8 GiB is held to the maximum");
    assert_eq!(second_region_size(64 << 30, MIN, MAX, PAGE), MAX);
    assert_eq!(second_region_size(128 * MIB as u64, MIN, MAX, PAGE), MIN, "a small machine");
    assert_eq!(second_region_size(0, MIN, MAX, PAGE), MIN);
    for ram in [(1u64 << 30) + 12_345, 3 << 30, 5_000_000_007] {
        assert_eq!(second_region_size(ram, MIN, MAX, PAGE) % PAGE, 0, "{ram}");
    }
}

#[test]
fn only_a_process_page_is_refused_and_only_under_the_floor() {
    let floor = 16 * MIB;
    assert!(user_page_refused(true, floor - 1, floor));
    assert!(user_page_refused(true, 0, floor));
    assert!(!user_page_refused(true, floor, floor), "at the floor a page still maps");
    assert!(!user_page_refused(true, 200 * MIB, floor));
    assert!(!user_page_refused(false, 0, floor), "the kernel's own mapping is never refused");
}
