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

//! The link came up: the receive hand-off sized to this driver's buffer,
//! the inter-frame gap set for the speed, the filter's microcontroller
//! restarted, receive and transmit on, the gate lifted, and the filter set
//! to unicast to the drawn address, broadcast and every multicast group
//! (the stack keeps no group list to hash). Linux v6.12's order.

use alloc::vec::Vec;

use super::bounds::FW_TASK_MS;
use super::hand_off::hand_off;
use super::link_word::{half_slow, TEN};
use super::model::Model;
use super::op::{clear, put, set, Op, Target};
use super::reg::{pla16, pla32, pla8, usb16, usb8};
use super::regs::{pla, usb};
use super::usb_speed::UsbSpeed;

pub fn link_up(m: Model, bus: UsbSpeed, status: u32) -> Vec<Op> {
    let mut v = Vec::with_capacity(24);
    // Energy-efficient Ethernet's MAC side, at 10 Mb/s only.
    let eee = Target::Mac(pla16(pla::EEE_PLUS));
    v.push(if status & TEN != 0 { Op::Edit(eee, 0, 0x0002) } else { Op::Edit(eee, 0x0002, 0) });
    v.extend(hand_off(m, bus));
    // The inter-frame gap, and the 10 Mb/s idle mode beside it.
    let (gap, off, on) = if half_slow(status) { (0x0200, 0x0100, 0) } else { (0x0300, 0, 0x0100) };
    v.push(Op::Edit(Target::Mac(pla16(pla::TX_CONFIG1)), 0x0308, gap));
    v.push(Op::Edit(Target::Mac(pla16(pla::MAC_POWER4)), off, on));
    if m.code() == 0x6010 {
        let task = usb16(usb::FW_TASK);
        v.extend_from_slice(&[clear(task, 0x0002), Op::Pause(FW_TASK_MS), set(task, 0x0002)]);
    }
    v.extend_from_slice(&[
        clear(pla16(pla::FILTER_MCU), 0x0001),
        set(pla16(pla::FILTER_MCU), 0x0001),
        set(pla8(pla::COMMAND), 0x08 | 0x04),
    ]);
    if m.is_b() {
        v.push(put(usb8(usb::RX_DMA_OWN), 0x01 | 0x02));
    }
    v.extend_from_slice(&[
        clear(pla16(pla::MISC1), 0x0008), // receive no longer gated
        put(pla32(pla::MULTICAST), u32::MAX),
        put(pla32(pla::MULTICAST + 4), u32::MAX),
        // Broadcast 08h, multicast 04h, unicast to the station 02h; never
        // every frame (01h).
        Op::Edit(Target::Mac(pla32(pla::RX_CONFIG)), 0x0f, 0x08 | 0x04 | 0x02),
    ]);
    v
}
