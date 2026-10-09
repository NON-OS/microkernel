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

//! The weighted draw: range, and proportionality.

extern crate alloc;

use crate::draw::pick;

#[test]
fn an_empty_or_weightless_list_has_no_answer() {
    assert_eq!(pick(&[], 0), None);
    assert_eq!(pick(&[0, 0, 0], 12_345), None);
}

#[test]
fn every_index_is_reachable_and_none_is_exceeded() {
    let weights = [5u64, 1, 9, 3];
    let total: u64 = weights.iter().sum();
    let mut seen = [false; 4];
    for roll in 0..total {
        let index = pick(&weights, roll).expect("a weighted list answers");
        assert!(index < weights.len());
        seen[index] = true;
    }
    assert!(seen.iter().all(|s| *s), "some candidate can never be drawn");
}

#[test]
fn each_candidate_is_drawn_in_proportion_to_its_weight() {
    let weights = [5u64, 1, 9, 3];
    let total: u64 = weights.iter().sum();
    let mut counts = [0u64; 4];
    for roll in 0..total {
        counts[pick(&weights, roll).expect("answers")] += 1;
    }
    assert_eq!(counts.to_vec(), weights.to_vec());
}
