// NONOS Operating System (AGPL-3.0-or-later)
//! Free RAM runs: sorted, disjoint, merged where they meet or touch, and
//! when storage runs out only the smallest are left out.

use crate::usable::{Usable, MAX_RUNS};

fn of(ranges: &[(u64, u64)]) -> Usable {
    let mut u = Usable::EMPTY;
    for &(a, b) in ranges {
        u.add(a, b);
    }
    u
}

#[test]
fn runs_come_out_sorted_and_merged() {
    let u = of(&[(0x5000, 0x6000), (0x1000, 0x2000), (0x2000, 0x3000), (0x5800, 0x9000)]);
    assert_eq!(u.runs(), [(0x1000, 0x3000), (0x5000, 0x9000)]);
}

#[test]
fn one_range_can_join_several_runs() {
    let u = of(&[(0x1000, 0x2000), (0x3000, 0x4000), (0x5000, 0x6000), (0x1800, 0x5000)]);
    assert_eq!(u.runs(), [(0x1000, 0x6000)]);
}

#[test]
fn empty_and_inverted_ranges_add_nothing() {
    assert!(of(&[(0x1000, 0x1000), (0x2000, 0x1000)]).runs().is_empty());
}

#[test]
fn the_largest_is_the_widest() {
    let u = of(&[(0x1000, 0x3000), (0x10_0000, 0x10_1000), (0x8000, 0xF000)]);
    assert_eq!(u.largest(), Some((0x8000, 0xF000)));
    assert_eq!(Usable::EMPTY.largest(), None);
}

#[test]
fn when_full_the_smallest_run_is_left_out() {
    // MAX_RUNS runs of growing size, a page apart so none touch.
    let at = |i: u64| (i * 0x10_0000, i * 0x10_0000 + (i + 1) * 0x1000);
    let mut u = of(&(0..MAX_RUNS as u64).map(at).collect::<Vec<_>>());
    assert_eq!(u.runs().len(), MAX_RUNS);
    u.add(0x7000_0000, 0x7010_0000);
    assert_eq!(u.runs().len(), MAX_RUNS);
    assert!(!u.runs().contains(&at(0)), "the smallest went");
    assert!(u.runs().contains(&(0x7000_0000, 0x7010_0000)));
    u.add(0x7800_0000, 0x7800_0800);
    assert!(!u.runs().contains(&(0x7800_0000, 0x7800_0800)), "smaller than every run");
}
