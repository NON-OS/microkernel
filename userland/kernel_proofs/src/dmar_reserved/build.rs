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

//! DMAR structures as firmware lays them out, for the RMRR proofs.

use std::vec::Vec;

/// An RMRR: type 1, length, reserved, segment, base, limit, then scopes.
pub(super) fn rmrr(base: u64, limit: u64, scopes: &[Vec<u8>]) -> Vec<u8> {
    let mut b = vec![1u8, 0, 0, 0, 0, 0, 0, 0];
    b.extend_from_slice(&base.to_le_bytes());
    b.extend_from_slice(&limit.to_le_bytes());
    scopes.iter().for_each(|s| b.extend_from_slice(s));
    let len = b.len() as u16;
    b[2..4].copy_from_slice(&len.to_le_bytes());
    b
}

pub(super) fn scope(kind: u8, start_bus: u8, path: &[(u8, u8)]) -> Vec<u8> {
    let mut s = vec![kind, (6 + 2 * path.len()) as u8, 0, 0, 0, start_bus];
    path.iter().for_each(|&(d, f)| s.extend_from_slice(&[d, f]));
    s
}

/// One bridge at 0:1c.0 with buses 2..=3 behind it.
pub(super) fn bridges(bus: u8, dev: u8, func: u8) -> Option<(u8, u8)> {
    ((bus, dev, func) == (0, 0x1c, 0)).then_some((2, 3))
}

pub(super) const GFX: (u8, u8, u8) = (0, 2, 0);
