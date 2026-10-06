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

//! Entropy hedged against a weak source: the stream the witness draws from
//! is keyed by the caller's bytes, the seed and the request together, so a
//! repeated or poor draw under another spend still gives an unrelated stream.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use stark_proofs::crypto::stark::hash::keccak256;

use crate::api::ENTROPY_BYTES;

pub const DOMAIN: &[u8] = b"NOX-HEDGED-ENTROPY-1";

pub fn hedge(entropy: &[u8], seed: &str, request: &str) -> Result<Vec<u8>, String> {
    if entropy.len() < ENTROPY_BYTES {
        return Err(format!(
            "not enough entropy: {} bytes, pass at least {ENTROPY_BYTES} random bytes",
            entropy.len()
        ));
    }
    let len = u32::try_from(entropy.len()).map_err(|_| String::from("entropy longer than 4 GiB"))?;
    let mut k = Vec::with_capacity(DOMAIN.len() + 4 + entropy.len() + 64);
    k.extend_from_slice(DOMAIN);
    k.extend_from_slice(&len.to_le_bytes());
    k.extend_from_slice(entropy);
    k.extend_from_slice(&keccak256(seed.as_bytes()));
    k.extend_from_slice(&keccak256(request.as_bytes()));
    let key = keccak256(&k);
    let mut out = Vec::with_capacity(ENTROPY_BYTES + 32);
    let mut i = 0u32;
    while out.len() < ENTROPY_BYTES {
        let mut b = [0u8; 36];
        b[..32].copy_from_slice(&key);
        b[32..].copy_from_slice(&i.to_le_bytes());
        out.extend_from_slice(&keccak256(&b));
        i += 1;
    }
    out.truncate(ENTROPY_BYTES);
    Ok(out)
}
