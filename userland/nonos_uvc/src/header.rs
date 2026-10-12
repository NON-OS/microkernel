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
//! A payload's header: its length, then bmHeaderInfo, whose bit 0 (FID)
//! flips at each new frame, bit 1 (EOF) ends one, bit 6 (ERR) says the
//! payload is bad, and bit 7 (EOH) ends the header fields.

pub const FID: u8 = 1 << 0;
pub const EOF: u8 = 1 << 1;
pub const ERR: u8 = 1 << 6;

/// The header's length and bmHeaderInfo, or None when it cannot be one.
pub fn header(p: &[u8]) -> Option<(usize, u8)> {
    let hlen = *p.first()? as usize;
    (2..=p.len()).contains(&hlen).then(|| (hlen, p[1]))
}
