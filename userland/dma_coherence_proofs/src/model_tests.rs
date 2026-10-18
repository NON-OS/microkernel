// NONOS Operating System (AGPL-3.0-or-later)
//! What goes wrong without the maintenance, and why the kernel keeps no
//! plain invalidate and no cacheable alias of an uncached grant.

use crate::cache_span::lines;
use crate::model::{System, LINE};

fn each_line(s: &mut System, start: u64, len: u64, op: fn(&mut System, u64)) {
    if let Some((first, n)) = lines(start, len, LINE) {
        (0..n).for_each(|i| op(s, first / LINE + i));
    }
}

#[test]
fn a_device_reads_stale_data_until_the_lines_are_cleaned() {
    let (mut s, buf) = (System::default(), 0x1000);
    (0..100).for_each(|i| s.cpu_write(buf + i, 7));
    assert_eq!(s.device_read(buf + 99), 0, "the write is still in the cache");
    each_line(&mut s, buf, 100, System::clean);
    assert!((0..100).all(|i| s.device_read(buf + i) == 7));
}

#[test]
fn the_cpu_reads_stale_data_until_the_lines_are_dropped() {
    let (mut s, buf) = (System::default(), 0x2000);
    assert_eq!(s.cpu_read(buf), 0);
    s.device_write(buf, 9);
    assert_eq!(s.cpu_read(buf), 0, "the line from before the device wrote");
    each_line(&mut s, buf, 1, System::clean_invalidate);
    assert_eq!(s.cpu_read(buf), 9);
}

#[test]
fn a_line_shared_with_a_neighbour_loses_one_side_either_way() {
    // The buffer starts mid-line; the neighbour owns the line's first byte.
    let (neighbour, buf) = (0x3000, 0x3010);
    let start = || {
        let mut s = System::default();
        s.cpu_write(neighbour, 5);
        s.device_write(buf, 9);
        s
    };
    let (mut s, mut dropped) = (start(), start());
    each_line(&mut dropped, buf, 16, System::invalidate);
    assert_eq!(dropped.device_read(neighbour), 0, "a plain invalidate lost the neighbour");
    each_line(&mut s, buf, 16, System::clean_invalidate);
    assert_eq!(s.device_read(buf), 0, "a clean wrote the stale line over the device's data");
}

#[test]
fn uncached_on_both_sides_needs_nothing_and_one_cacheable_alias_breaks_it() {
    let (mut s, ring) = (System::default(), 0x4000);
    s.device_write(ring, 3);
    assert_eq!(s.cpu_read_uncached(ring), 3);
    let mut aliased = System::default();
    aliased.cpu_read(ring); // the kernel's cacheable alias, fetched ahead
    aliased.device_write(ring, 3);
    assert_eq!(aliased.cpu_read_uncached(ring), 0, "the alias's line answers the read");
}
