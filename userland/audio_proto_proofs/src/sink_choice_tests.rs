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

//! Which driver the server plays through: a ready USB device over the
//! fixed sink, HD Audio or AC'97 (sink_machine_tests.rs), and the USB driver
//! alone kept so a device can still come.

use crate::sink_choose::with_usb;

const HDA: u32 = 4218;
const USB: u32 = 4246;

#[test]
fn a_ready_usb_device_plays_over_hd_audio() {
    assert_eq!(with_usb(true, Some(HDA), Some(USB)), Some(USB));
    assert_eq!(with_usb(true, None, Some(USB)), Some(USB));
}

#[test]
fn hd_audio_plays_while_the_usb_driver_has_no_device() {
    assert_eq!(with_usb(false, Some(HDA), Some(USB)), Some(HDA));
    assert_eq!(with_usb(false, Some(HDA), None), Some(HDA));
    // Readiness from a driver that is not there changes nothing.
    assert_eq!(with_usb(true, Some(HDA), None), Some(HDA));
}

#[test]
fn the_usb_driver_alone_is_kept_so_a_device_can_still_come() {
    assert_eq!(with_usb(false, None, Some(USB)), Some(USB));
    assert_eq!(with_usb(false, None, None), None);
    assert_eq!(with_usb(true, None, None), None);
}
