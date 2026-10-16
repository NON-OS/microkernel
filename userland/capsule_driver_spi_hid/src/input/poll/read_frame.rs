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

//! The shared poll's frame read, over SPI: one input report read whole and
//! laid out as a HID-over-I2C frame. A reset the device announces sends it
//! back through bring-up, as its descriptors may have changed.

use nonos_spi_hid::{IN_DATA, IN_RESET};

use super::signal_raw_report::signal_raw_report;
use crate::shape::shape;
use crate::state::{State, FRAME_MAX};

pub(super) fn read_frame(state: &mut State, buf: &mut [u8; FRAME_MAX]) -> Option<usize> {
    let link = crate::link::get()?;
    state.input_polls = state.input_polls.wrapping_add(1);
    let r = crate::report::read(&link)?;
    if r.kind == IN_RESET {
        state.descriptor_len = 0;
        return None;
    }
    let n = shape(r.id, &r.content, buf).filter(|_| r.kind == IN_DATA)?;
    state.input_reports = state.input_reports.wrapping_add(1);
    signal_raw_report();
    Some(n)
}
