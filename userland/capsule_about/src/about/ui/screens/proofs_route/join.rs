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

//! Byte strings run together into a fixed buffer, cut where it fills.

pub fn join<const N: usize>(out: &mut [u8; N], a: &[u8], b: &[u8], c: &[u8]) -> usize {
    join3(out, a, b, c)
}

pub fn join3<const N: usize>(out: &mut [u8; N], a: &[u8], b: &[u8], c: &[u8]) -> usize {
    let mut n = 0;
    for part in [a, b, c] {
        for &x in part {
            if n == N {
                return n;
            }
            out[n] = x;
            n += 1;
        }
    }
    n
}
