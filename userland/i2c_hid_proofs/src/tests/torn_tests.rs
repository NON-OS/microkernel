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

//! A frame whose length prefix says more than was read is torn: it is
//! skipped whole, never decoded from what did arrive, but heard as a report.

use nonos_i2cmodel::HidOverI2c;
use nonos_libc::take_events;

use super::descs::MOUSE;
use super::fixture::{rig_with, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;

#[test]
fn a_frame_whose_prefix_is_longer_than_the_read_is_skipped() {
    // wMaxInputLength 6: the read stops after the length and four bytes.
    let device = HidOverI2c::new(PAD, MOUSE.to_vec(), 2 + 4, 0x04F3, 0x0002);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let mut s = setup::run().expect("setup");
    take_events();
    // An eight-byte report: its prefix says ten, six bytes are read.
    r.pad.lock().push_input(&[3, 0b01, 5, 0, 9, 9, 9, 9]);
    // Still a report the pad sent: it keeps the reads at the active pace.
    assert!(poll(&mut s), "a torn report was not heard");
    let events = take_events();
    assert!(events.is_empty(), "a torn frame was decoded: {events:?}");
    r.pad.lock().push_input(&[3, 0b01, 5, 0]);
    poll(&mut s);
    assert!(!take_events().is_empty(), "a whole frame after it was not");
}
