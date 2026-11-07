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

//! Making the host keys, once, on this machine. Every byte comes from the
//! kernel's generator: the Ed25519 seed directly, and the ML-DSA-65 key
//! pair from its key generation fed by the same source.

use super::layout::{join, PLAIN_LEN};
use super::seal::wipe;
use super::Fault;
use crate::rng;

/// New host keys, as their sealed plaintext.
pub fn fresh(out: &mut [u8; PLAIN_LEN]) -> Result<(), Fault> {
    let mut seed = [0u8; 32];
    if !rng::fill(&mut seed) {
        return Err(Fault::Entropy);
    }
    let made = nonos_mldsa65::keypair(&mut |b: &mut [u8]| rng::fill(b));
    let result = match made {
        Ok((public, secret)) => {
            join(&seed, secret.as_bytes(), &public, out);
            Ok(())
        }
        Err(_) => Err(Fault::Entropy),
    };
    wipe(&mut seed);
    result
}
