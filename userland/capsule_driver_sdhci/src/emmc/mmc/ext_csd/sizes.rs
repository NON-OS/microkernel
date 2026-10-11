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

//! The boot and RPMB partition sizes, and the volatile cache.

use super::offsets::{BOOT_SIZE_MULT, CACHE_CTRL, CACHE_SIZE, RPMB_SIZE_MULT};
use super::register::ExtCsd;

impl ExtCsd {
    /// Each boot partition, in 512-byte sectors (BOOT_SIZE_MULT x 128 KiB).
    pub const fn boot_sectors(&self) -> u64 {
        self.raw[BOOT_SIZE_MULT] as u64 * 256
    }

    /// The RPMB partition, in 512-byte sectors (RPMB_SIZE_MULT x 128 KiB).
    pub const fn rpmb_sectors(&self) -> u64 {
        self.raw[RPMB_SIZE_MULT] as u64 * 256
    }

    /// The volatile cache size in KiB (CACHE_SIZE, bytes 252:249).
    pub const fn cache_kib(&self) -> u32 {
        u32::from_le_bytes([
            self.raw[CACHE_SIZE],
            self.raw[CACHE_SIZE + 1],
            self.raw[CACHE_SIZE + 2],
            self.raw[CACHE_SIZE + 3],
        ])
    }

    /// The card has a cache and it is on: writes may sit in it until a
    /// FLUSH_CACHE.
    pub const fn cache_on(&self) -> bool {
        self.cache_kib() != 0 && self.raw[CACHE_CTRL as usize] & 1 != 0
    }
}
