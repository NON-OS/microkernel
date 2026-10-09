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

//! SHA-384, computed in the caller.

use sha2::{Digest, Sha384};

/*
 * This was an IPC to the crypto pool through a fixed 1536 byte request, so a
 * certificate body longer than 1516 bytes could not be hashed at all and its
 * chain was refused as though its signature were bad: a leaf naming many hosts
 * under a P-384 authority failed for its size alone. Hashed here, any length
 * is hashed, and the signature check itself still goes to the pool.
 */
pub fn hash_sha384(data: &[u8]) -> Option<[u8; 48]> {
    let mut out = [0u8; 48];
    out.copy_from_slice(&Sha384::digest(data));
    Some(out)
}
