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
//! One axis of a PS/2 movement packet: the delta byte, its sign bit from
//! byte 0, and the overflow bit that says the delta did not fit.

/// Movement on one axis. When the controller reports an overflow the delta byte
/// is the low bits of a value that did not fit, so applying it verbatim makes
/// the cursor leap to a garbage position. Cap it to a bounded step in the
/// reported direction instead, which keeps fast motion smooth and monotonic.
pub fn axis(v: u8, sign_bit: u8, overflow: bool) -> i16 {
    if overflow {
        if sign_bit != 0 {
            -255
        } else {
            255
        }
    } else {
        sign(v, sign_bit)
    }
}

fn sign(v: u8, sign_bit: u8) -> i16 {
    if sign_bit != 0 {
        v as i16 | -256i16
    } else {
        v as i16
    }
}
