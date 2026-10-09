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

//! Building a 128-bit register field by field for the CSD proofs.

/// Place `value` at bits start..start+size of a 128-bit register held as
/// Linux holds a response, word 0 the highest.
pub fn put(resp: &mut [u32; 4], start: u32, size: u32, value: u64) {
    for i in 0..size {
        if value >> i & 1 == 1 {
            let bit = start + i;
            resp[3 - (bit / 32) as usize] |= 1 << (bit % 32);
        }
    }
}
