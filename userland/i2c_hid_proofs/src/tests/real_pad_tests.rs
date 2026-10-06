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

//! What real pads do that the plain bench pad does not: a Synaptics-style
//! descriptor register the firmware did not name, a feature report id past
//! the command nibble, and a pad that only ever reports its mouse
//! collection.

use nonos_i2cmodel::touchpad::{report_descriptor, MODE_REPORT_ID, TOUCH_REPORT_LEN};
use nonos_i2cmodel::{Command, HidOverI2c};

use super::fixture::{pad, rig_with, HID_DESC_REG, PAD};
use crate::hid::{decode_mouse, parse_mouse_layout};
use crate::setup;

#[test]
fn a_pad_with_its_descriptor_at_0x20_binds_when_the_firmware_only_gave_the_default() {
    let _r = rig_with(pad().descriptor_at(0x0020), Some((PAD, HID_DESC_REG)));
    let state = setup::run().expect("setup");
    assert!(state.found(), "a Synaptics-style pad was left unbound");
    assert_eq!(state.addr, PAD);
}

#[test]
fn a_feature_report_id_of_15_or_more_is_sent_in_the_extended_command_form() {
    const EXT_ID: u8 = 0x14;
    let mut desc = report_descriptor();
    let at = desc.windows(2).position(|w| w == [0x85, MODE_REPORT_ID]).expect("mode report id");
    desc[at + 1] = EXT_ID;
    let device = HidOverI2c::new(PAD, desc, 2 + TOUCH_REPORT_LEN, 0x06CB, 0xCD5E);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    setup::run().expect("setup");
    let pad = r.pad.lock();
    assert!(
        pad.commands().iter().any(|c| matches!(c, Command::GetReport { ty: 3, id: EXT_ID })),
        "no GET_REPORT for id {EXT_ID:#x}: {:?}",
        pad.commands()
    );
    assert_eq!(pad.feature(EXT_ID).map(|f| f[1]), Some(3), "input mode not set to touchpad");
}

/// A boot-mouse-shaped collection with report id 2: three buttons, five bits
/// of padding, then 12-bit relative X and Y and an 8-bit wheel.
fn mouse_descriptor() -> Vec<u8> {
    [
        &[0x05, 0x01, 0x09, 0x02, 0xA1, 0x01, 0x85, 0x02][..],
        &[0x05, 0x09, 0x19, 0x01, 0x29, 0x03, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x03, 0x81, 0x02],
        &[0x75, 0x05, 0x95, 0x01, 0x81, 0x03],
        &[0x05, 0x01, 0x09, 0x30, 0x09, 0x31, 0x16, 0x01, 0xF8, 0x26, 0xFF, 0x07, 0x75, 0x0C, 0x95, 0x02, 0x81, 0x06],
        &[0x09, 0x38, 0x15, 0x81, 0x25, 0x7F, 0x75, 0x08, 0x95, 0x01, 0x81, 0x06],
        &[0xC0],
    ]
    .concat()
}

#[test]
fn the_mouse_collection_is_decoded_where_the_descriptor_puts_it() {
    let layout = parse_mouse_layout(&mouse_descriptor());
    assert!(layout.is_relative_mouse());
    assert_eq!(layout.report_id, 2);
    // Buttons 1 and 3, dx = -3 and dy = +260 in 12 bits, wheel -1.
    let dx = (-3i32 as u32) & 0xFFF;
    let dy = 260u32;
    let xy = dx | (dy << 12);
    let report = [2, 0b101, xy as u8, (xy >> 8) as u8, (xy >> 16) as u8, 0xFF];
    let s = decode_mouse(&report, &layout).expect("decoded");
    assert_eq!((s.buttons, s.dx, s.dy, s.wheel), (0b101, -3, 127, -1));
    assert!(decode_mouse(&[1, 0, 0, 0, 0, 0], &layout).is_none(), "another report id");
    assert!(decode_mouse(&[2, 0, 0], &layout).is_none(), "a short report");
}

#[test]
fn the_precision_touchpad_descriptor_has_no_relative_mouse_to_mistake_for_one() {
    assert!(!parse_mouse_layout(&report_descriptor()).is_relative_mouse());
}
