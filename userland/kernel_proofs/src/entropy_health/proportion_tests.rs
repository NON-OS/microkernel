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

//! The adaptive proportion test, window by window.

use super::bytes::{uniform, window_with};
use super::health::*;

const W: usize = APT_WINDOW as usize;

fn run(h: &mut Health, bytes: &[u8]) -> Result<(), Fault> {
    bytes.iter().try_for_each(|&x| h.sample(x))
}

#[test]
fn one_short_of_the_cutoff_in_a_window_passes() {
    let mut h = Health::new();
    assert_eq!(run(&mut h, &window_with(5, APT_CUTOFF as usize - 1, W)), Ok(()));
}

#[test]
fn the_cutoff_in_one_window_alarms() {
    let mut h = Health::new();
    assert_eq!(run(&mut h, &window_with(5, APT_CUTOFF as usize, W)), Err(Fault::Proportion));
}

#[test]
fn a_count_does_not_carry_into_the_next_window() {
    let mut h = Health::new();
    let short = window_with(5, APT_CUTOFF as usize - 1, W);
    for _ in 0..4 {
        assert_eq!(run(&mut h, &short), Ok(()), "each window counts afresh");
    }
}

#[test]
fn the_window_is_the_one_its_first_byte_opens() {
    // Windows open on whatever byte comes first, wherever the stream is.
    let mut h = Health::new();
    assert_eq!(run(&mut h, &uniform(3, 100)), Ok(()));
    assert_eq!(run(&mut h, &uniform(4, W - 100)), Ok(()));
    assert_eq!(run(&mut h, &window_with(9, APT_CUTOFF as usize, W)), Err(Fault::Proportion));
}
