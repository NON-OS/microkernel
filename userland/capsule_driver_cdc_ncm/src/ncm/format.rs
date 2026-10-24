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

//! The two NTB formats a received block may take (USB NCM 1.0, 3.2 and
//! 3.3, as recalled, unverified). NTH16 and NTH32 differ only in the width
//! of their lengths and indices, and so do the NDPs they lead to, so one
//! description of each serves the whole walk.

use super::nth::{le16, le32};

pub struct Format {
    pub nth_sign: u32,
    pub nth_len: usize,
    /// "NCM0" or "ncm0": datagrams without a CRC. CRC mode is set off at
    /// bind, so an NDP with CRCs is not read.
    pub ndp_sign: u32,
    /// The NDP's fixed part before its entries, and where in it the next
    /// NDP's index sits.
    pub ndp_header: usize,
    pub ndp_next: usize,
    /// One entry: a datagram's index, then its length.
    pub entry: usize,
    /// Lengths and indices are 32 bits wide, else 16.
    pub wide: bool,
}

pub const NTB16: Format = Format {
    nth_sign: 0x484D_434E,
    nth_len: 12,
    ndp_sign: 0x304D_434E,
    ndp_header: 8,
    ndp_next: 6,
    entry: 4,
    wide: false,
};
pub const NTB32: Format = Format {
    nth_sign: 0x686D_636E,
    nth_len: 16,
    ndp_sign: 0x306D_636E,
    ndp_header: 16,
    ndp_next: 8,
    entry: 8,
    wide: true,
};

impl Format {
    /// A length or index of this format's width at `at`, when `b` holds it.
    pub fn field(&self, b: &[u8], at: usize) -> Option<usize> {
        if self.wide {
            le32(b, at)
        } else {
            le16(b, at)
        }
    }
}
