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
//! What OPEN and READ hand back.

use alloc::vec::Vec;

use crate::bind::Mic;
use crate::device::Dev;
use crate::uac::RATE_HZ;
use crate::wire::CHUNK;

/// `vendor u16, product u16, channels u8, 0, 0, 0, rate u32`.
pub fn opened(m: &Mic) -> [u8; 12] {
    let (v, p, r) = (m.ids.0.to_le_bytes(), m.ids.1.to_le_bytes(), RATE_HZ.to_le_bytes());
    [v[0], v[1], p[0], p[1], m.s.channels, 0, 0, 0, r[0], r[1], r[2], r[3]]
}

/// `overrun u32`, then up to `most` bytes of whole frames.
pub fn read(d: &mut Dev, most: u16) -> Vec<u8> {
    let (overrun, bytes) = d.ring.take((most as usize).min(CHUNK));
    let mut out = Vec::with_capacity(4 + bytes.len());
    out.extend(overrun.to_le_bytes());
    out.extend(bytes);
    out
}
