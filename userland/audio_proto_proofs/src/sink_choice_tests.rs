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

//! The server's choice of sink: one fixed sink registered is taken without a
//! question, two are asked in order, and a machine where neither plays keeps
//! the first, whose status says why; a ready USB device is played through
//! over it, and the USB driver alone is kept so a device can still come.

use crate::sink_choose::{choose, with_usb, SINKS};

const HDA: u32 = 4218;
const USB: u32 = 4246;

#[test]
fn the_sinks_are_hda_then_ac97() {
    assert_eq!(SINKS, ["driver.hda0", "driver.ac97_0"]);
}

#[test]
fn a_lone_sink_is_taken_without_asking() {
    let never = |_: usize| -> bool { panic!("a lone sink is not asked") };
    assert_eq!(choose([true, false], never), Some(0));
    assert_eq!(choose([false, true], never), Some(1));
}

#[test]
fn no_sink_is_none() {
    assert_eq!(choose([false, false], |_| true), None);
}

// An HD Audio controller that cannot play (HDMI only, a laptop for SOF)
// must not hide a working AC'97 beside it.
#[test]
fn with_both_the_first_that_plays_is_taken() {
    assert_eq!(choose([true, true], |i| i == 1), Some(1));
    let mut asked = Vec::new();
    assert_eq!(
        choose([true, true], |i| {
            asked.push(i);
            true
        }),
        Some(0)
    );
    assert_eq!(asked, vec![0], "the second is not asked once the first plays");
}

#[test]
fn with_both_and_neither_playing_the_first_keeps_its_say() {
    assert_eq!(choose([true, true], |_| false), Some(0));
}

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
