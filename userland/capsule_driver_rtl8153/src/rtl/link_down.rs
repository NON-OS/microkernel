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

//! The link went down: every frame refused, receive gated, the FIFOs left
//! to drain, then the MAC and the buffer manager reset, as Linux v6.12
//! does. Each wait is short and goes on when it ends: this runs on the
//! thread that answers net.core.

use super::bounds::DRAIN;
use super::op::{clear, put, set, until, Op};
use super::reg::{pla16, pla32, pla8, usb8};
use super::regs::{pla, usb};

pub const LINK_DOWN: [Op; 8] = [
    clear(pla32(pla::RX_CONFIG), 0x0f),
    set(pla16(pla::MISC1), 0x0008),
    until(pla8(pla::OOB), 0x30, &[0x30], DRAIN),
    until(pla16(pla::TX_CONFIG), 0x0800, &[0x0800], DRAIN),
    put(pla8(pla::COMMAND), 0x10),
    until(pla8(pla::COMMAND), 0x10, &[0], DRAIN),
    clear(usb8(usb::BMU_RESET), 0x03),
    set(usb8(usb::BMU_RESET), 0x03),
];
