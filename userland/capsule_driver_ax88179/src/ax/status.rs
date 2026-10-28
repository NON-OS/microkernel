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

//! The chip's report on its interrupt IN endpoint (as Linux v6.12 reads it,
//! read for behaviour): 8 bytes, the first 32-bit word little-endian, its
//! bit 16 the link. It only says when to read the PHY (watch.rs): the
//! PHY's own status, not this, decides the link and the medium.

/// The report's length; what one look arms at most.
pub const STATUS_LEN: usize = 8;
const LINK: u32 = 1 << 16;

/// The link the report `n` gives; `None` when it is shorter than a report.
pub fn link_hint(n: &[u8]) -> Option<bool> {
    let w = n.get(..4).filter(|_| n.len() >= STATUS_LEN)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]) & LINK != 0)
}
