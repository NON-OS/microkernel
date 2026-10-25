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

//! What the session table needs of net.nym's crypto: the session key type,
//! as `crypto/types.rs` defines it, and a random source. The table never
//! reads the bytes it draws, so a fixed fill stands in for the kernel's.

/// A session key, `crypto/types.rs`'s `Key`.
pub type Key = [u8; 32];

pub mod random {
    /// The source failed; never, here.
    pub struct Unavailable;

    /// Fill `out`; the kernel's source on the device, a fixed byte here.
    pub fn fill_random(out: &mut [u8]) -> Result<(), Unavailable> {
        out.fill(0x5A);
        Ok(())
    }
}

/// net.nym's own volatile wipe, the one its secrets are cleared with.
#[path = "../../capsule_net_nym/src/crypto/wipe.rs"]
pub mod wipe;

/// net.nym's `CryptoError` and key types, as the capsule defines them.
#[path = "../../capsule_net_nym/src/crypto/types.rs"]
pub mod types;

/// BLAKE3, which the capsule asks the kernel for, from the reference crate.
pub mod hash {
    use super::types::CryptoError;

    pub fn blake3(input: &[u8], out: &mut [u8; 32]) -> Result<(), CryptoError> {
        out.copy_from_slice(blake3::hash(input).as_bytes());
        Ok(())
    }
}

/// HKDF over BLAKE3, the capsule's own, which the gateway key is derived with.
#[path = "../../capsule_net_nym/src/crypto/hkdf_blake3/mod.rs"]
pub mod hkdf_blake3;

pub mod ed25519 {
    /// The identity's public half. The wipe proofs never read it, so it is
    /// the seed's first bytes rather than a curve point.
    pub fn pubkey(seed: &[u8; 32], out: &mut [u8; 32]) -> usize {
        out.copy_from_slice(seed);
        32
    }
}
