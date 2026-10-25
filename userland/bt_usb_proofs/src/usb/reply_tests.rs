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

//! An interrupt reply from driver.xhci0 is taken only as long as the read
//! armed and the octets the reply holds, whatever count it claims.

use super::limit::{reply_len, INTERRUPT_MAX};

#[test]
fn a_reply_within_its_read_and_its_octets_is_taken_whole() {
    assert_eq!(reply_len(16, 16, 18), Some(16));
    assert_eq!(reply_len(16, 3, 5), Some(3));
    assert_eq!(reply_len(64, 64, 66), Some(64));
    // A zero-length packet ends a transfer; it carries nothing to join.
    assert_eq!(reply_len(16, 0, 2), Some(0));
}

#[test]
fn a_count_over_the_read_is_refused_even_when_the_octets_are_there() {
    assert_eq!(reply_len(16, 17, 66), None);
    assert_eq!(reply_len(0, 1, 66), None);
    assert_eq!(reply_len(INTERRUPT_MAX, INTERRUPT_MAX + 1, 1000), None);
}

#[test]
fn a_count_over_the_octets_held_is_refused() {
    assert_eq!(reply_len(64, 16, 17), None, "one short");
    assert_eq!(reply_len(64, 0, 1), None, "no room for the count itself");
    assert_eq!(reply_len(64, 0, 0), None);
    assert_eq!(reply_len(usize::MAX, usize::MAX, usize::MAX), None, "no wrap");
}

#[test]
fn every_small_case_matches_the_rule() {
    for armed in 0..=INTERRUPT_MAX + 2 {
        for claimed in 0..=INTERRUPT_MAX + 4 {
            for held in 0..=INTERRUPT_MAX + 4 {
                let ok = claimed <= armed && claimed + 2 <= held;
                assert_eq!(reply_len(armed, claimed, held), ok.then_some(claimed));
            }
        }
    }
}
