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

//! The device made ready for input: its device descriptor asked for and
//! read, then its report descriptor, and the HID driver's state filled from
//! them as a touchpad's on I2C is. The input mode is not switched: a pad
//! stays in its mouse collection until the feature report is written here.

use alloc::vec::Vec;

use nonos_libc::mk_idle_ms;
use nonos_spi_hid::{descriptor, DEVICE_DESCRIPTOR_LEN, IN_DEVICE_DESCRIPTOR};
use nonos_spi_hid::{IN_REPORT_DESCRIPTOR, OUT_DEVICE_DESCRIPTOR, OUT_REPORT_DESCRIPTOR};

use crate::hid::{parse_mouse_layout, parse_report_descriptor, Multitouch};
use crate::link::Link;
use crate::report::{ask, read};
use crate::state::State;

/// The shared poll reads only a device with an input register. An SPI
/// device's reports come at the firmware's address instead, so any value
/// but zero lets the poll read.
const INPUT_REGISTER: u16 = 1;
/// Reads after a request before its answer is given up on.
const ANSWER_READS: u32 = 50;
const ANSWER_MS: u64 = 10;

pub fn bring_up(l: &Link, state: &mut State) -> Result<(), &'static str> {
    let d = answer(l, OUT_DEVICE_DESCRIPTOR, IN_DEVICE_DESCRIPTOR)
        .ok_or("spi_hid: the device gave no device descriptor")?;
    let desc = descriptor(&d).ok_or("spi_hid: a device descriptor of another length")?;
    let rd = answer(l, OUT_REPORT_DESCRIPTOR, IN_REPORT_DESCRIPTOR)
        .filter(|rd| rd.len() == usize::from(desc.report_desc_len))
        .ok_or("spi_hid: the device gave no whole report descriptor")?;
    state.descriptor[..DEVICE_DESCRIPTOR_LEN].copy_from_slice(&d[..DEVICE_DESCRIPTOR_LEN]);
    state.descriptor_len = DEVICE_DESCRIPTOR_LEN;
    state.input_register = INPUT_REGISTER;
    state.input_len = usize::from(desc.max_input);
    state.touch_layout = parse_report_descriptor(&rd);
    state.mt = Multitouch::new(&rd, &state.touch_layout);
    state.mouse_layout = parse_mouse_layout(&rd);
    Ok(())
}

/// The request written, then reports read until one of `kind` answers it;
/// any other met meanwhile (a reset announced, stale input) is let go.
fn answer(l: &Link, request: u8, kind: u8) -> Option<Vec<u8>> {
    if !ask(l, request) {
        return None;
    }
    for _ in 0..ANSWER_READS {
        match read(l) {
            Some(r) if r.kind == kind => return Some(r.content),
            Some(_) => {}
            None => {
                let _ = mk_idle_ms(ANSWER_MS);
            }
        }
    }
    None
}
