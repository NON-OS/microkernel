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

//! SD memory card command indices and arguments (SD Physical Layer
//! Simplified Specification 9.10, as recalled, unverified), where they differ from eMMC's.

/// SEND_IF_COND, R7: the host's supply range and a check pattern.
pub const SEND_IF_COND: u8 = 8;
/// SEND_RELATIVE_ADDR, R6: the card publishes its own address.
pub const SEND_RELATIVE_ADDR: u8 = 3;
/// APP_CMD: the next command is an application command.
pub const APP_CMD: u8 = 55;
/// ACMD6 SET_BUS_WIDTH.
pub const SET_BUS_WIDTH: u8 = 6;
/// ACMD41 SD_SEND_OP_COND, R3.
pub const SD_SEND_OP_COND: u8 = 41;

/// CMD8 argument: VHS 0001b (2.7-3.6 V) in 11:8, check pattern 0xAA.
pub const IF_COND_ARG: u32 = 0x1aa;
/// ACMD41 HCS: the host takes a high or extended capacity card.
pub const HCS: u32 = 1 << 30;
/// ACMD6 argument for four data lines.
pub const BUS_WIDTH_4: u32 = 2;

/// Default Speed runs up to 25 MHz (SD Physical Layer, as recalled); after
/// CMD2 the bus may.
pub const DEFAULT_SPEED_HZ: u32 = 25_000_000;

/// ACMD41 finishes power up within a second (SD Physical Layer, as
/// recalled, unverified), asked every 10 ms.
pub const OP_COND_MS: u64 = 1_000;
pub const OP_COND_POLL_MS: u64 = 10;

/// The R7 a card that takes the host's supply echoes: VHS and pattern.
pub const fn if_cond_echoed(r7: u32) -> bool {
    r7 & 0xfff == IF_COND_ARG
}
