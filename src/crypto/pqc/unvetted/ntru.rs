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

//! The NTRU wrappers, as bytes in and bytes out.

use alloc::vec::Vec;

use super::super::ntru;

pub fn ntruhps4096821_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let keypair = ntru::ntru_keygen()?;
    Ok((
        ntru::ntru_serialize_public_key(&keypair.public_key),
        ntru::ntru_serialize_secret_key(&keypair.secret_key),
    ))
}

pub fn ntruhps4096821_encapsulate(pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let public_key = ntru::ntru_deserialize_public_key(pk)?;
    let (ct, ss) = ntru::ntru_encaps(&public_key)?;
    Ok((ntru::ntru_serialize_ciphertext(&ct), ss.to_vec()))
}

pub fn ntruhps4096821_decapsulate(ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let ciphertext = ntru::ntru_deserialize_ciphertext(ct)?;
    let secret_key = ntru::ntru_deserialize_secret_key(sk)?;
    let ss = ntru::ntru_decaps(&ciphertext, &secret_key)?;
    Ok(ss.to_vec())
}
