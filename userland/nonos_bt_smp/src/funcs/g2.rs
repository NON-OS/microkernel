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

//! g2, the numeric comparison value (Vol 3, Part H, 2.2.9): the low 32
//! bits of AES-CMAC under X of U || V || Y; the user compares it modulo
//! 10^6 as six decimal digits (Vol 3, Part H, 2.3.5.6.2).

use crate::cmac::aes_cmac;

pub fn g2(u: &[u8; 32], v: &[u8; 32], x: &[u8; 16], y: &[u8; 16]) -> u32 {
    let mac = aes_cmac(x, &[u, v, y]);
    u32::from_be_bytes([mac[12], mac[13], mac[14], mac[15]])
}

/// The six digits shown for a g2 value.
pub fn passkey_display(g2: u32) -> u32 {
    g2 % 1_000_000
}
