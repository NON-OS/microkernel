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

//! Between the air's order (least significant octet first) and the
//! crypto functions' (most significant first).

/// A value from the air in function order.
pub fn be<const N: usize>(le: &[u8; N]) -> [u8; N] {
    let mut out = *le;
    out.reverse();
    out
}

/// A function value in air order.
pub fn le<const N: usize>(be: &[u8; N]) -> [u8; N] {
    let mut out = *be;
    out.reverse();
    out
}
