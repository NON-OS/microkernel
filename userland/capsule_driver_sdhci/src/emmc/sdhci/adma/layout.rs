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

//! Descriptor sizes and attribute bits, the chunk and table limits, and
//! the addressing width.

/// 32-bit addressing: attribute u16, length u16, address u32.
pub const DESC32_LEN: usize = 8;
/// 64-bit addressing in version 3 mode: attribute u16, length u16,
/// address u64 (Linux's SDHCI_ADMA2_64_DESC_SZ without v4 mode).
pub const DESC64_LEN: usize = 12;

pub const ATTR_VALID: u16 = 1 << 0;
pub const ATTR_END: u16 = 1 << 1;
pub const ACT_TRAN: u16 = 2 << 4;

/// The most bytes one descriptor moves here. The 16-bit length field reaches
/// 65535, and 0 means 65536 only on hosts that allow it; 32 KiB stays clear
/// of both edges.
pub const MAX_CHUNK: usize = 32 * 1024;

/// Room for a table: sixteen descriptors of either width, 512 KiB of data.
pub const TABLE_BYTES: usize = 16 * DESC64_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    A32,
    A64,
}

impl Width {
    pub const fn desc_len(self) -> usize {
        match self {
            Width::A32 => DESC32_LEN,
            Width::A64 => DESC64_LEN,
        }
    }

    /// Alignment the host requires of a data address and of the table.
    pub const fn align(self) -> u64 {
        match self {
            Width::A32 => 4,
            Width::A64 => 8,
        }
    }
}
