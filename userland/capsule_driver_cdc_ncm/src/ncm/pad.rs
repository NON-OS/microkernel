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

//! How long a sent NTB is made. The device learns where a block ends from
//! a short packet, or from its length reaching dwNtbOutMaxSize (NCM 1.0,
//! 3.8, as recalled, unverified). The controller cannot send a zero-length
//! packet here, so a block that ends on a packet boundary below the
//! device's size gets one zero byte more, counted in its wBlockLength.

use super::limits::ends_on_packet;
use super::shape::TxShape;

pub fn padded(len: usize, s: &TxShape) -> usize {
    if len < s.out_max && ends_on_packet(len, s.mps) {
        len + 1
    } else {
        len
    }
}
