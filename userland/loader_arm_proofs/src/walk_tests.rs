// NONOS Operating System (AGPL-3.0-or-later)
//! The loader's own set/way walk, run on aarch64 against `walk_model`. Each
//! fixture is a cache hierarchy a CPU could show.

use crate::walk::run;
use crate::walk_model::{check, fixture};

#[test]
fn split_l1_and_unified_l2() {
    check(&[3, 4], 2, &[(6, 4, 256), (6, 8, 512)]);
}

#[test]
fn instruction_only_and_absent_levels_are_skipped() {
    check(&[1, 4, 0, 4], 4, &[(6, 2, 64), (6, 4, 128), (6, 1, 1), (7, 2, 32)]);
}

#[test]
fn levels_past_the_point_of_coherency_are_left() {
    check(&[3, 4, 4], 1, &[(6, 4, 64), (6, 8, 64), (6, 16, 64)]);
}

#[test]
fn no_level_of_coherency_issues_nothing() {
    assert!(run(&fixture(&[3, 4], 0, &[(6, 4, 64), (6, 8, 64)], false)).is_empty());
}

#[test]
fn one_way_puts_no_way_bits() {
    check(&[2], 1, &[(6, 1, 128)]);
}

#[test]
fn ways_that_are_no_power_of_two() {
    check(&[2, 4, 4], 3, &[(6, 3, 16), (6, 12, 32), (8, 20, 8)]);
}
