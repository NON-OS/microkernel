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
//! Where SLP_EN lands, and whether a port write sets it.
//!
//! PM1a and PM1b control are the ACPI capsule's: it reads SCI_EN there and
//! writes GBL_RLS. Only SLP_EN is the kernel's, since setting it enters the
//! sleep state SLP_TYP names, soft-off among them, with RAM unwiped. A write
//! of any width, at any port, that sets the bit is refused; one that clears
//! it, or leaves it out of its bytes, goes ahead.

use super::power::SPACE_IO;

/// SLP_EN in PM1x_CNT (ACPI 6.5, 4.8.3.2.1).
pub const PM1_SLP_EN: u8 = 13;
/// SLP_EN in the hardware-reduced SLEEP_CONTROL register (4.8.3.7).
pub const SLEEP_CONTROL_SLP_EN: u8 = 5;

/// The port byte that holds one register's SLP_EN, and its bit in that byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SleepEnable {
    pub port: u64,
    pub bit: u8,
}

/// Where SLP_EN, bit `slp` of a register at `bit_offset`, lands; `None` for an
/// absent register or one in memory, which the memory rule refuses whole.
pub fn sleep_enable(space: u8, address: u64, bit_offset: u8, slp: u8) -> Option<SleepEnable> {
    if address == 0 || space != SPACE_IO {
        return None;
    }
    let bit = u64::from(bit_offset) + u64::from(slp);
    Some(SleepEnable { port: address.checked_add(bit / 8)?, bit: (bit % 8) as u8 })
}

/// Whether `value`, written `width` bytes wide at `at`, sets one of `enables`.
/// A width other than 1 or 2 is written as 4 bytes (`port_access`).
pub fn sets_sleep_enable<'a>(
    enables: impl IntoIterator<Item = &'a SleepEnable>,
    at: u64,
    width: u8,
    value: u32,
) -> bool {
    let bytes = match width {
        1 => 1,
        2 => 2,
        _ => 4,
    };
    enables.into_iter().any(|e| {
        let within = e.port >= at && e.port - at < bytes;
        within && (value >> ((e.port - at) * 8 + u64::from(e.bit))) & 1 == 1
    })
}
