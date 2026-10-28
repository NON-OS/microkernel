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

//! The MAC made ready to move frames, receive still gated: ALDPS off,
//! wake-on-packet timers off, every frame refused, the MAC and the USB
//! buffer manager reset, the buffer link list rebuilt, VLAN tags left in
//! the frames, 1522-byte frames, and the FIFO thresholds Linux v6.12 sets.

use alloc::vec::Vec;

use super::bounds::{ALDPS_OFF, LINK_LIST};
use super::common::{no_u1u2, MAC_RESET_RUN, NO_U2P3};
use super::model::Model;
use super::op::{clear, put, set, until, Op, Target};
use super::reg::{pla16, pla32, pla8, usb8};
use super::regs::{phy, pla, usb};
use super::start_tail::{tail, THRESHOLDS};

/// The largest frame taken in: 1500 bytes, a VLAN header, the FCS.
pub const RX_MAX: u32 = 1522;
const LIST_READY: Op = until(pla8(pla::OOB), 0x02, &[0x02], LINK_LIST);

pub fn start(m: Model) -> Vec<Op> {
    let mut v = Vec::with_capacity(40);
    v.extend_from_slice(no_u1u2(m));
    v.extend_from_slice(&[
        NO_U2P3,
        Op::Edit(Target::Phy(phy::POWER_CFG), 0x0004, 0),
        until(pla16(pla::POWER_STATE), 0x0100, &[0x0100], ALDPS_OFF),
        set(pla16(pla::MISC1), 0x0008), // receive gated
        if m.is_b() { put(pla8(pla::TEREDO), 0xff) } else { clear(pla16(pla::TEREDO), 0x80ff) },
        put(pla16(pla::WATCHDOG6), 0x0010),
        put(pla16(pla::WAKE_TIMER), 0),
        put(pla32(pla::TEREDO_TIMER), 0),
        clear(pla32(pla::RX_CONFIG), 0x0f),
    ]);
    v.extend_from_slice(&MAC_RESET_RUN);
    v.extend_from_slice(&[
        clear(usb8(usb::BMU_RESET), 0x03),
        set(usb8(usb::BMU_RESET), 0x03),
        clear(pla8(pla::OOB), 0x80), // out of out-of-band mode
        clear(pla16(pla::SFF7), 0x4000),
        LIST_READY,
        set(pla16(pla::SFF7), 0x8000), // the link list rebuilt
        LIST_READY,
        clear(pla16(pla::CPLUS), 0x0040),
        put(pla16(pla::RX_MAX), RX_MAX),
        put(pla8(pla::TX_MAX), 12 * 1024 / 64),
        set(pla16(pla::TX_CONFIG), 0x0080),
    ]);
    v.extend_from_slice(&MAC_RESET_RUN);
    v.extend_from_slice(&THRESHOLDS);
    v.extend_from_slice(tail(m));
    v
}
