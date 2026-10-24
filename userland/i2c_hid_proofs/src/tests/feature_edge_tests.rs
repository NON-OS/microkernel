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

//! Feature fields at the edges: one declared outside every collection is no
//! part of the pad's configuration, and one whose end does not fit 32 bits
//! is never built into a report and written.

use nonos_i2cmodel::touchpad::report_descriptor;
use nonos_i2cmodel::Command;

use super::fixture::rig;
use crate::hid::{configure_reporting, parse_report_descriptor, Field, I2cFeatures, TouchLayout};
use crate::setup;

#[test]
fn an_input_mode_outside_every_collection_is_not_taken() {
    // Report 9, Input Mode, a byte, as a Feature at the top level.
    let top: &[u8] = &[0x05, 0x0D, 0x85, 0x09, 0x09, 0x52, 0x75, 0x08, 0x95, 0x01, 0xB1, 0x02];
    let l = parse_report_descriptor(&[top, &report_descriptor()].concat());
    assert!(l.is_absolute_touch());
    assert_ne!(l.input_mode_report_id, 9, "a field outside every collection was taken");
}

#[test]
fn a_feature_field_ending_past_32_bits_is_never_written() {
    let r = rig();
    let state = setup::run().expect("setup");
    r.pad.lock().take_commands();
    // Feature report 9 is one the pad does not answer GET_REPORT for.
    let huge = Field { bit_offset: u32::MAX - 3, bit_size: 8, logical_min: 0, logical_max: 3 };
    let l = TouchLayout { input_mode: huge, input_mode_report_id: 9, ..TouchLayout::default() };
    let port = I2cFeatures { port: state.i2c_port, addr: state.addr, desc: &state.descriptor };
    let c = configure_reporting(&port, &l);
    assert!(c.mode_unwritable, "a field that cannot be placed was taken as writable");
    let cmds = r.pad.lock().take_commands();
    assert!(!cmds.iter().any(|c| matches!(c, Command::SetReport { .. })), "{cmds:?}");
}
