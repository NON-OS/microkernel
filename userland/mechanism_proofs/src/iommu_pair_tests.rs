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
//! Root, context and remapping entries carry their present bit in the low
//! quadword. A unit that reads the entry present must read the rest of it as
//! written, so the low quadword is the last store on a set and the first on a
//! clear. These pin the order the kernel's writer uses.

use crate::iommu::out_of_frame::OutOfFrame;
use crate::iommu::pair::{clear_pair, set_pair};
use crate::iommu::view::Frame;

#[test]
fn a_set_writes_the_high_quadword_before_the_present_one() {
    let frame = Frame::recording();
    set_pair(frame, 10, 0xA001, 0xB002).unwrap();
    assert_eq!(frame.stores(), [(11, 0xB002), (10, 0xA001)]);
}

#[test]
fn a_clear_drops_the_present_quadword_first() {
    let frame = Frame::recording();
    clear_pair(frame, 10).unwrap();
    assert_eq!(frame.stores(), [(10, 0), (11, 0)]);
}

#[test]
fn an_entry_past_the_frame_is_refused_before_it_goes_live() {
    let frame = Frame::recording();
    assert_eq!(set_pair(frame, 511, 1, 2), Err(OutOfFrame));
    assert_eq!(clear_pair(frame, 511), Err(OutOfFrame));
    // The set never wrote its present half; the clear did deny the entry.
    assert_eq!(frame.stores(), [(511, 0)]);
}

#[test]
fn the_last_entry_that_fits_is_written_whole() {
    let frame = Frame::recording();
    set_pair(frame, 510, 7, 8).unwrap();
    assert_eq!(frame.stores(), [(511, 8), (510, 7)]);
}
