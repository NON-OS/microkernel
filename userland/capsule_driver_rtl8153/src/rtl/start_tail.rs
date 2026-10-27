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

//! The end of the start phase: the FIFO thresholds Linux v6.12 sets, then
//! what each chip takes last.

use super::model::Model;
use super::op::{clear, put, set, Op};
use super::reg::{pla16, pla32, pla8, usb16, usb32};
use super::regs::{pla, usb};

/// The receive FIFO's three credit thresholds and the transmit FIFO's.
pub const THRESHOLDS: [Op; 4] = [
    put(pla32(pla::RX_FIFO_A), 0x0008_0002),
    put(pla16(pla::RX_FIFO_B), 0x00a0),
    put(pla16(pla::RX_FIFO_C), 0x0110),
    put(pla32(pla::TX_FIFO), 0x0100_0008),
];

/// The B's receive threshold and its MCU clock not slowed; the first
/// chip's LAN-wake clear on, its wake pin off and its SuperSpeed PHY's
/// power-change delay off.
const B: [Op; 2] =
    [put(usb32(usb::RX_THRESHOLD), 0x0001_0001), clear(pla16(pla::MAC_POWER3), 0x4000)];
const FIRST: [Op; 3] = [
    set(pla8(pla::CONFIG6), 0x01),
    clear(pla8(pla::LAN_WAKE), 0x80),
    clear(usb16(usb::SS_LINK1), 0x0002),
];

pub fn tail(m: Model) -> &'static [Op] {
    if m.is_b() {
        &B
    } else {
        &FIRST
    }
}
