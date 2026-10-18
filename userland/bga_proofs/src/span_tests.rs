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

//! A mode against the framebuffer mapping it has to fit, at and past every
//! bound: the byte count is a product of device-sized numbers, so it is
//! checked, and a mapping one byte short of it is refused.

use crate::constants::{MODE_HEIGHT, MODE_WIDTH};
use crate::dispi::span;
use crate::error::BgaError;

const MODE: u64 = MODE_WIDTH as u64 * MODE_HEIGHT as u64 * 4;

#[test]
fn a_mapping_that_holds_the_mode_exactly_gives_a_span() {
    assert!(span(0x1000, MODE, MODE_WIDTH, MODE_HEIGHT).is_ok());
    assert!(span(0x1000, u64::MAX, MODE_WIDTH, MODE_HEIGHT).is_ok());
}

#[test]
fn a_mapping_any_shorter_than_the_mode_is_refused() {
    for len in [0, 1, 1 << 20, 2 << 20, MODE - 4, MODE - 1] {
        let r = span(0x1000, len, MODE_WIDTH, MODE_HEIGHT).err();
        assert_eq!(r, Some(BgaError::ModeTooLarge), "a {len}-byte mapping took the mode");
    }
}

#[test]
fn the_largest_mode_the_registers_can_name_is_counted_without_overflow() {
    let bytes = u16::MAX as u64 * u16::MAX as u64 * 4;
    assert!(span(0, bytes - 1, u16::MAX, u16::MAX).is_err());
    assert!(span(0, bytes, u16::MAX, u16::MAX).is_ok());
    assert!(span(0, 0, 0, 0).is_ok(), "an empty mode fits an empty mapping");
}
