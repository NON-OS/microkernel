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


//! SHA3-256 and SHAKE-256, for onion services.
//!
//! Every hash an onion service uses is one of these two: the address
//! checksum, key blinding, the HSDir ring, the descriptor's key derivation and
//! MAC, hs-ntor and the virtual hop's running digest (rend-spec-v3). Here
//! rather than in the pool for the reason SHA-1 is: the running digest is
//! updated once per cell and has to be cloned and peeked, which a one shot
//! syscall cannot do.

mod permute;
mod sponge;

pub use sponge::{Sha3_256, Shake256};

/// SHA3-256 of the parts in order, as if they were one input.
pub fn sha3_256_parts(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = Sha3_256::new();
    for part in parts {
        h.update(part);
    }
    h.finish()
}

/// The MAC onion services use everywhere: SHA3-256 of the key's length as
/// eight big endian bytes, the key, then the message (crypto_mac_sha3_256).
pub fn mac_sha3_256(key: &[u8], message: &[u8]) -> [u8; 32] {
    sha3_256_parts(&[&(key.len() as u64).to_be_bytes(), key, message])
}
