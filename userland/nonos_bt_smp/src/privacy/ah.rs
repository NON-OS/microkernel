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

//! ah, the random address hash (Vol 3, Part H, 2.2.2): AES-128 under the
//! IRK of prand padded with 104 zero bits on the left, and the low 24 bits
//! of the result. Values most significant octet first, as 2.2.2 writes
//! them.

use nonos_aes::Aes128;

pub fn ah(irk: &[u8; 16], prand: &[u8; 3]) -> [u8; 3] {
    let mut block = [0u8; 16];
    block[13..].copy_from_slice(prand);
    Aes128::new(irk).encrypt_block(&mut block);
    [block[13], block[14], block[15]]
}
