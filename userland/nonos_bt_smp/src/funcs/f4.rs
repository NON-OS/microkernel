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

//! f4, the confirm value (Vol 3, Part H, 2.2.6): AES-CMAC under X of
//! U || V || Z. U and V are public key X coordinates, X a nonce, Z one
//! octet (0 for Just Works, 0x80 or 0x81 per passkey bit).

use crate::cmac::aes_cmac;

pub fn f4(u: &[u8; 32], v: &[u8; 32], x: &[u8; 16], z: u8) -> [u8; 16] {
    aes_cmac(x, &[u, v, &[z]])
}
