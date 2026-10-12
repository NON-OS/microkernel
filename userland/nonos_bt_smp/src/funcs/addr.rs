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

//! A1 and A2 as f5 and f6 take them (Vol 3, Part H, 2.2.7): the address
//! type in the top octet (0 public, 1 random) over the 48-bit address,
//! most significant octet first. `BdAddr` holds the address in its wire
//! order, least significant first, so it is turned around here.

use nonos_bt_wire::addr::{AddrType, BdAddr};

pub fn addr56(kind: AddrType, addr: &BdAddr) -> [u8; 7] {
    let mut out = [0u8; 7];
    out[0] = kind.wire() & 1;
    for (o, b) in out[1..].iter_mut().zip(addr.0.iter().rev()) {
        *o = *b;
    }
    out
}
