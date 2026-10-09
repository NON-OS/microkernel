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
//! Which position the refill trusts.

use crate::controller::position::{Position, Source};

const RING: u32 = 0x8000;

#[test]
fn a_position_buffer_that_moves_is_trusted() {
    let mut p = Position::new(true);
    assert_eq!(p.pick(0x2000, 0x1f80, RING), 0x2000);
    assert_eq!(p.pick(0, 0x10, RING), 0, "a buffer once seen moving is kept");
    assert_eq!(p.source, Source::Buffer);
}

#[test]
fn a_position_buffer_that_never_moves_is_dropped_for_lpib() {
    let mut p = Position::new(true);
    assert_eq!(p.pick(0, 0x2040, RING), 0x2040);
    assert_eq!(p.source, Source::Lpib);
    assert_eq!(p.pick(0x4000, 0x2100, RING), 0x2100);
}

#[test]
fn lpib_controllers_never_read_the_buffer() {
    let mut p = Position::new(false);
    assert_eq!(p.pick(0x6000, 0x1000, RING), 0x1000);
}

#[test]
fn a_reading_past_the_ring_is_folded_back_into_it() {
    let mut p = Position::new(false);
    assert_eq!(p.pick(0, RING + 0x100, RING), 0x100);
}
