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

//! A writer swept asleep at any of its requests leaves a volume the next
//! writer and a reboot both go on from; each knob that undoes the design
//! is caught.

use super::run::{ABOVE, LOST, REUSE, UNWRITTEN};
use super::search::search;
use super::kinds::Knob;

#[test]
fn a_swept_writer_leaves_nothing_the_next_writer_or_a_reboot_trips_on() {
    let (bad, n) = search(Knob::Fixed);
    assert_eq!(bad, 0, "marks {bad:#x} over {n} scenarios");
    assert_eq!(n, 252, "every cut of every pair in every directory shape");
}

#[test]
fn a_mount_copied_into_the_writer_hands_its_blocks_out_again() {
    let (bad, _) = search(Knob::LocalCopy);
    assert_eq!(bad & REUSE, REUSE, "{bad:#x}");
}

#[test]
fn a_request_in_flight_landing_late_loses_the_next_writers_work() {
    let (bad, _) = search(Knob::NoFifo);
    assert_eq!(bad & LOST, LOST, "{bad:#x}");
    assert_eq!(bad & (ABOVE | UNWRITTEN), 0, "late is lost, not torn: {bad:#x}");
}
