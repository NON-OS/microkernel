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

//! A platform device as the ACPI capsule hands it over: a controller with a
//! `_HID` and a memory `_CRS` that no PCI function stands for (LPSS and AMD
//! DesignWare I2C, SD/eMMC hosts, GPIO controllers), with what its
//! evaluated `_CRS` gave. Pure: `kernel_proofs` mounts this file as it is.
//!
//! Little-endian: version u8 (1), class u8 (`I2C`, `SD`, `GPIO`), window
//! count u8 (1 to 5), interrupt flags u8 (`HAS_IRQ`, `EDGE`,
//! `ACTIVE_LOW`, `SHARED`, `WAKE`; none without `HAS_IRQ`), record flags
//! u8 (`DEPS_UNMET`), three zero bytes; `_HID` as eight bytes (printable,
//! NUL-padded); `_UID` u64; the GSI u32; four zero bytes; each window's
//! base and length u64; the namespace path's length u8 (1 to 127) and its
//! bytes, starting with '\'.

use alloc::vec::Vec;

pub const I2C: u8 = 1;
pub const SD: u8 = 2;
pub const GPIO: u8 = 3;
pub const HAS_IRQ: u8 = 1;
pub const EDGE: u8 = 2;
pub const ACTIVE_LOW: u8 = 4;
pub const SHARED: u8 = 8;
pub const WAKE: u8 = 16;
pub const IRQ_KNOWN: u8 = HAS_IRQ | EDGE | ACTIVE_LOW | SHARED | WAKE;
/// Record flag: a _DEP entry is missing, dangling or in a cycle; the device
/// is published anyway, once, and flagged.
pub const DEPS_UNMET: u8 = 1;
pub const MAX_WINDOWS: usize = 5;
pub const MAX_RECORD: usize = 32 + MAX_WINDOWS * 16 + 128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Platform {
    pub class: u8,
    pub irq_flags: u8,
    pub flags: u8,
    pub hid: [u8; 8],
    pub uid: u64,
    pub gsiv: u32,
    pub windows: Vec<(u64, u64)>,
    pub path: Vec<u8>,
}
