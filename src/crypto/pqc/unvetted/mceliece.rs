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

//! The Classic McEliece wrappers, as bytes in and bytes out.

use alloc::vec::Vec;

use super::super::mceliece;

pub fn mceliece348864_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let keypair = mceliece::mceliece_keygen()?;
    Ok((
        mceliece::mceliece_serialize_public_key(&keypair.public_key),
        mceliece::mceliece_serialize_secret_key(&keypair.secret_key),
    ))
}

pub fn mceliece348864_encapsulate(pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let public_key = mceliece::mceliece_deserialize_public_key(pk)?;
    let (ct, ss) = mceliece::mceliece_encaps(&public_key)?;
    Ok((mceliece::mceliece_serialize_ciphertext(&ct), ss.to_vec()))
}

pub fn mceliece348864_decapsulate(ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let ciphertext = mceliece::mceliece_deserialize_ciphertext(ct)?;
    let secret_key = mceliece::mceliece_deserialize_secret_key(sk)?;
    let ss = mceliece::mceliece_decaps(&ciphertext, &secret_key)?;
    Ok(ss.to_vec())
}
