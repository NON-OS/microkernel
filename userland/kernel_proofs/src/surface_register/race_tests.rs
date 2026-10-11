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

//! The register's second walk, and the busy ranges it is refused over.

use super::register_race::{ranges_meet, still_mapped};

#[test]
fn a_range_an_mmap_or_munmap_is_changing_is_met_by_any_overlap() {
    assert!(ranges_meet(0x1000, 0x3000, 0x2000, 0x4000));
    assert!(ranges_meet(0x2000, 0x3000, 0x1000, 0x9000));
    assert!(!ranges_meet(0x1000, 0x2000, 0x2000, 0x3000), "touching is not meeting");
    assert!(!ranges_meet(0x5000, 0x6000, 0x1000, 0x2000));
}

#[test]
fn pages_still_mapped_to_the_frames_the_walk_read_pass() {
    assert!(still_mapped(&[0x7000, 0x3000], [Some(0x7000), Some(0x3000)]));
}

#[test]
fn a_page_unmapped_since_the_walk_fails() {
    assert!(!still_mapped(&[0x7000, 0x3000], [Some(0x7000), None]));
}

#[test]
fn a_page_mapped_to_another_frame_since_the_walk_fails() {
    // A munmap and a fresh mmap at the same address between the two walks.
    assert!(!still_mapped(&[0x7000, 0x3000], [Some(0x7000), Some(0x8000)]));
}

#[test]
fn a_second_walk_shorter_or_longer_than_the_first_fails() {
    assert!(!still_mapped(&[0x7000, 0x3000], [Some(0x7000)]));
    assert!(!still_mapped(&[0x7000], [Some(0x7000), Some(0x3000)]));
}
