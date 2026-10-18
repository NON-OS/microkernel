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

//! A resolvable private address: prand in the upper 24 bits, its hash under
//! the IRK in the lower 24 (Vol 6, Part B, 1.3.2.2). `BdAddr` holds the
//! address least significant octet first, so the hash is octets 0 to 2 and
//! prand octets 3 to 5, each turned around from the order ah takes.

use super::ah::ah;
use nonos_bt_wire::addr::{BdAddr, RandomKind};

/// prand from 24 random bits: the top two set to 0b01, which marks the
/// address resolvable. None when the other 22 are all 0 or all 1, which
/// 1.3.2.2 forbids; the caller draws again.
pub fn prand(draw: [u8; 3]) -> Option<[u8; 3]> {
    let p = [(draw[0] & 0x3f) | 0x40, draw[1], draw[2]];
    let random = u32::from_be_bytes([0, p[0] & 0x3f, p[1], p[2]]);
    (random != 0 && random != 0x3f_ffff).then_some(p)
}

/// The address for `prand` under `irk`.
pub fn rpa(irk: &[u8; 16], prand: &[u8; 3]) -> BdAddr {
    let hash = ah(irk, prand);
    BdAddr([hash[2], hash[1], hash[0], prand[2], prand[1], prand[0]])
}

/// Whether `addr` is a resolvable private address made under `irk`
/// (Vol 6, Part B, 1.3.2.3). Any other kind of address resolves under
/// no key.
pub fn resolves(irk: &[u8; 16], addr: &BdAddr) -> bool {
    if addr.random_kind() != Ok(RandomKind::Resolvable) {
        return false;
    }
    let a = addr.0;
    ah(irk, &[a[5], a[4], a[3]]) == [a[2], a[1], a[0]]
}
