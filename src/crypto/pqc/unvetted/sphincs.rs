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

//! The SPHINCS+ wrappers, as bytes in and bytes out.

use alloc::vec::Vec;

use super::super::sphincs;

pub fn sphincs128s_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let keypair = sphincs::sphincs_keygen()?;
    Ok((
        sphincs::sphincs_serialize_public_key(&keypair.public_key),
        sphincs::sphincs_serialize_secret_key(&keypair.secret_key),
    ))
}

pub fn sphincs128s_sign(message: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let secret_key = sphincs::sphincs_deserialize_secret_key(sk)?;
    let sig = sphincs::sphincs_sign(&secret_key, message)?;
    Ok(sphincs::sphincs_serialize_signature(&sig))
}

pub fn sphincs128s_verify(message: &[u8], sig: &[u8], pk: &[u8]) -> bool {
    let public_key = match sphincs::sphincs_deserialize_public_key(pk) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let signature = match sphincs::sphincs_deserialize_signature(sig) {
        Ok(s) => s,
        Err(_) => return false,
    };
    sphincs::sphincs_verify(&public_key, message, &signature)
}
