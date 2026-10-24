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

//! The X-Wing exchange of the hellos. The client makes a key pair from a
//! fresh 32-byte seed and sends its encapsulation key; the server
//! encapsulates to it with 64 fresh bytes and sends the ciphertext; both
//! hold the same 32-byte secret. A client key is used once: decapsulating
//! consumes it.

use crate::{CryptoError, Secret};
use x_wing::{Decapsulate, DecapsulationKey, Decapsulator, EncapsulationKey, KeyExport};

pub const EK_LEN: usize = x_wing::ENCAPSULATION_KEY_SIZE;
pub const CT_LEN: usize = x_wing::CIPHERTEXT_SIZE;

pub struct ClientKem {
    dk: DecapsulationKey,
}

impl ClientKem {
    /// A key pair from `seed`, which must be 32 fresh random bytes made for
    /// this one connection. The encapsulation key goes in `CLIENT_HELLO`.
    pub fn new(seed: Secret<32>) -> (Self, [u8; EK_LEN]) {
        // The seed is handed over by value, as the KEM takes it; the copy it
        // keeps is wiped with the key, which is consumed by `decapsulate`.
        let dk = DecapsulationKey::from(*seed.expose());
        let mut ek = [0u8; EK_LEN];
        ek.copy_from_slice(&dk.encapsulation_key().to_bytes());
        (ClientKem { dk }, ek)
    }

    /// The shared secret from the server's ciphertext.
    pub fn decapsulate(self, ct: &[u8; CT_LEN]) -> Secret<32> {
        let mut ss: [u8; 32] = self.dk.decapsulate(&(*ct).into()).into();
        Secret::take(&mut ss)
    }
}

/// The server's half: a ciphertext for the client's key, and the shared
/// secret. `rand` must be 64 fresh random bytes, used for nothing else.
pub fn encapsulate(
    ek: &[u8; EK_LEN],
    rand: Secret<64>,
) -> Result<([u8; CT_LEN], Secret<32>), CryptoError> {
    let ek = EncapsulationKey::try_from(&ek[..]).map_err(|_| CryptoError::BadKey)?;
    let mut r = (*rand.expose()).into();
    let (ct, ss) = ek.encapsulate_deterministic(&r);
    nonos_hash::wipe(r.as_mut_slice());
    let mut ss: [u8; 32] = ss.into();
    Ok((ct.into(), Secret::take(&mut ss)))
}
