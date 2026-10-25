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

//! A queue head the ring does not have is a failed unit, never an index
//!.

use super::queue::{queue_offset, queue_slot, QUEUE_ENTRIES};

#[test]
fn every_head_on_the_ring_is_its_slot() {
    for i in 0..QUEUE_ENTRIES {
        assert_eq!(queue_slot(queue_offset(i)), Some(i));
    }
}

#[test]
fn a_head_past_the_ring_is_no_slot() {
    assert_eq!(queue_slot(queue_offset(QUEUE_ENTRIES)), None, "256");
    assert_eq!(queue_slot(queue_offset(0x7FFF)), None, "the widest the field holds");
    assert_eq!(queue_slot(u64::MAX), None, "a unit reading all ones");
}
