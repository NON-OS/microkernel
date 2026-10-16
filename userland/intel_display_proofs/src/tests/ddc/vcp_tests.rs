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

//! The allowlist, over every code there is: four may be read; brightness
//! and contrast set within the monitor's maximum; the input source never
//! set; the power mode set to on or off alone.

use crate::ddc::error::DdcError;
use crate::ddc::vcp::{may_read, may_set};

#[test]
fn only_four_controls_are_spoken_of() {
    let read: Vec<u8> = (0..=255u8).filter(|c| may_read(*c).is_ok()).collect();
    assert_eq!(read, [0x10, 0x12, 0x60, 0xD6]);
    for c in (0..=255u8).filter(|c| !read.contains(c)) {
        assert_eq!(may_read(c), Err(DdcError::NotAllowed));
        assert_eq!(may_set(c, 1, 100), Err(DdcError::NotAllowed));
    }
}

#[test]
fn each_allowed_control_takes_only_its_values() {
    for c in [0x10, 0x12] {
        assert_eq!(may_set(c, 100, 100), Ok(()));
        assert_eq!(may_set(c, 0, 100), Ok(()));
        assert_eq!(may_set(c, 101, 100), Err(DdcError::Value), "past the monitor's maximum");
    }
    for v in [0u16, 1, 0x0F, 0xFFFF] {
        assert_eq!(may_set(0x60, v, 0xFFFF), Err(DdcError::ReadOnly));
    }
    let power: Vec<u16> = (0..=0x1FFu16).filter(|v| may_set(0xD6, *v, 0).is_ok()).collect();
    assert_eq!(power, [1, 4], "on and off");
}
