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

//! When the chip hands received data up: after a time, or once it holds
//! more than the early size. Receive aggregation is off (wake), so a
//! transfer is one frame; the early size still keeps any transfer the chip
//! gathers inside this driver's buffer, a second bound beside the first.

use alloc::vec::Vec;

use super::model::Model;
use super::op::{put, Op};
use super::reg::usb16;
use super::regs::usb;
use super::rx::{DESC, RX_BUFFER};
use super::start::RX_MAX;
use super::usb_speed::{hold_ns, UsbSpeed};

/// The buffer less one whole frame, its descriptor and its alignment, in
/// the units each chip counts (4 bytes on the first, 8 on the B).
pub fn early_size(m: Model) -> u32 {
    let room = (RX_BUFFER - (RX_MAX as usize + DESC + 8)) as u32;
    if m.is_b() {
        room / 8
    } else {
        room / 4
    }
}

/// The B times out on its extra-aggregation timer (15 us), its early
/// timeout fixed at 128 ns; the first chip on its early timeout, by bus
/// speed. Both count in units of 8 ns.
pub fn hand_off(m: Model, bus: UsbSpeed) -> Vec<Op> {
    let mut v = Vec::with_capacity(3);
    if m.is_b() {
        v.push(put(usb16(usb::RX_EARLY_TIMEOUT), 128 / 8));
        v.push(put(usb16(usb::RX_EXTRA_AGG), 15_000 / 8));
    } else {
        v.push(put(usb16(usb::RX_EARLY_TIMEOUT), hold_ns(bus) / 8));
    }
    v.push(put(usb16(usb::RX_EARLY_SIZE), early_size(m)));
    v
}
