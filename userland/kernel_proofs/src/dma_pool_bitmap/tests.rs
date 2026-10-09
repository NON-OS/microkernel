// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::bitmap::{clear_run, count_used, run_offset, take_run};

const PAGE: u64 = 4096;
const BASE: u64 = 0x0100_0000;

#[test]
fn runs_are_taken_first_fit_and_never_overlap() {
    let mut used = [0u64; 2];
    assert_eq!(take_run(&mut used, 128, 3), Some(0));
    assert_eq!(take_run(&mut used, 128, 2), Some(3));
    assert_eq!(count_used(&used), 5);
    clear_run(&mut used, 0, 3);
    assert_eq!(take_run(&mut used, 128, 4), Some(5));
    assert_eq!(take_run(&mut used, 128, 3), Some(0));
}

#[test]
fn a_run_crosses_a_word_boundary() {
    let mut used = [0u64; 2];
    assert_eq!(take_run(&mut used, 128, 60), Some(0));
    assert_eq!(take_run(&mut used, 128, 10), Some(60));
    assert_eq!(used[0], u64::MAX);
    assert_eq!(used[1], 0x3F);
}

#[test]
fn a_full_pool_or_a_run_past_the_limit_is_refused() {
    let mut used = [0u64; 2];
    assert_eq!(take_run(&mut used, 100, 100), Some(0));
    assert_eq!(take_run(&mut used, 100, 1), None);
    let mut fresh = [0u64; 2];
    assert_eq!(take_run(&mut fresh, 100, 101), None);
    assert_eq!(count_used(&fresh), 0);
}

#[test]
fn a_returned_run_must_lie_inside_the_pool_on_a_page() {
    assert_eq!(run_offset(BASE, 2048, BASE, 1), Some(0));
    assert_eq!(run_offset(BASE, 2048, BASE + 2047 * PAGE, 1), Some(2047));
    assert_eq!(run_offset(BASE, 2048, BASE + 2047 * PAGE, 2), None);
    assert_eq!(run_offset(BASE, 2048, BASE - PAGE, 1), None);
    assert_eq!(run_offset(BASE, 2048, BASE + 1, 1), None);
    assert_eq!(run_offset(BASE, 2048, BASE, 0), None);
    assert_eq!(run_offset(BASE, 0, BASE, 1), None);
    assert_eq!(run_offset(BASE, 2048, BASE, usize::MAX), None);
}
