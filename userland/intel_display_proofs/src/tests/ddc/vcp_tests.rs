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

//! The allowlist, over every code there is: four may be read, and only
//! when the display's capabilities name them; brightness and contrast set
//! within the monitor's maximum; the input source set only to an input the
//! display lists; the power mode set to on or off alone, as listed.

use super::caps_rig::CAPS;
use crate::ddc::caps::Caps;
use crate::ddc::error::DdcError;
use crate::ddc::vcp::{may_read, may_set};

#[test]
fn only_four_controls_are_spoken_of_and_only_those_the_display_names() {
    let all = Caps::parse(CAPS).unwrap();
    let read: Vec<u8> = (0..=255u8).filter(|c| may_read(*c, &all).is_ok()).collect();
    assert_eq!(read, [0x10, 0x12, 0x60, 0xD6]);
    for c in (0..=255u8).filter(|c| !read.contains(c)) {
        assert_eq!(may_read(c, &all), Err(DdcError::NotAllowed));
        assert_eq!(may_set(c, 1, 100, &all), Err(DdcError::NotAllowed));
    }
    let some = Caps::parse(b"(vcp(10))").unwrap();
    assert_eq!(may_read(0x10, &some), Ok(()));
    assert_eq!(may_read(0x12, &some), Err(DdcError::NotOffered), "not named");
}

#[test]
fn each_allowed_control_takes_only_its_values() {
    let caps = Caps::parse(CAPS).unwrap();
    for c in [0x10, 0x12] {
        assert_eq!(may_set(c, 100, 100, &caps), Ok(()));
        assert_eq!(may_set(c, 0, 100, &caps), Ok(()));
        assert_eq!(may_set(c, 101, 100, &caps), Err(DdcError::Value), "past the maximum");
    }
    let inputs: Vec<u16> = (0..=0x1FFu16).filter(|v| may_set(0x60, *v, 0, &caps).is_ok()).collect();
    assert_eq!(inputs, [0x0F, 0x11], "only the inputs it lists");
    let power: Vec<u16> = (0..=0x1FFu16).filter(|v| may_set(0xD6, *v, 0, &caps).is_ok()).collect();
    assert_eq!(power, [1, 4], "on and off");
    let bare = Caps::parse(b"(vcp(60 D6))").unwrap();
    assert_eq!(may_set(0x60, 0x0F, 0, &bare), Err(DdcError::Value), "no inputs listed");
    assert_eq!(may_set(0xD6, 1, 0, &bare), Err(DdcError::Value), "no modes listed");
}
