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

extern crate alloc;
use alloc::vec::Vec;

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
use super::kyber;

#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
use super::ml_dsa_65;

pub use super::unvetted::*;

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub fn kyber1024_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    match kyber::kyber_keygen() {
        Ok(keypair) => Ok((keypair.public_key.bytes.to_vec(), keypair.secret_key.bytes.to_vec())),
        Err(_) => Err("Kyber keygen failed"),
    }
}

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub fn kyber768_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    match kyber::kyber_keygen() {
        Ok(keypair) => Ok((keypair.public_key.bytes.to_vec(), keypair.secret_key.bytes.to_vec())),
        Err(_) => Err("Kyber keygen failed"),
    }
}

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub fn kyber1024_encapsulate(pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let kyber_pk = kyber::kyber_deserialize_public_key(pk).map_err(|_| "Invalid public key")?;
    match kyber::kyber_encaps(&kyber_pk) {
        Ok((ct, ss)) => Ok((ct.bytes.to_vec(), ss.to_vec())),
        Err(_) => Err("Encapsulation failed"),
    }
}

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub fn kyber768_encapsulate(pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let kyber_pk = kyber::kyber_deserialize_public_key(pk).map_err(|_| "Invalid public key")?;
    match kyber::kyber_encaps(&kyber_pk) {
        Ok((ct, ss)) => Ok((ct.bytes.to_vec(), ss.to_vec())),
        Err(_) => Err("Encapsulation failed"),
    }
}

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub fn kyber1024_decapsulate(ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let kyber_ct = kyber::kyber_deserialize_ciphertext(ct).map_err(|_| "Invalid ciphertext")?;
    let kyber_sk = kyber::kyber_deserialize_secret_key(sk).map_err(|_| "Invalid secret key")?;
    match kyber::kyber_decaps(&kyber_ct, &kyber_sk) {
        Ok(shared_secret) => Ok(shared_secret.to_vec()),
        Err(_) => Err("Kyber decapsulation failed"),
    }
}

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub fn kyber768_decapsulate(ct: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let kyber_ct = kyber::kyber_deserialize_ciphertext(ct).map_err(|_| "Invalid ciphertext")?;
    let kyber_sk = kyber::kyber_deserialize_secret_key(sk).map_err(|_| "Invalid secret key")?;
    match kyber::kyber_decaps(&kyber_ct, &kyber_sk) {
        Ok(shared_secret) => Ok(shared_secret.to_vec()),
        Err(_) => Err("Kyber decapsulation failed"),
    }
}

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
pub fn kyber1024_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Err("Kyber not enabled (enable mlkem feature)")
}

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
pub fn kyber768_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Err("Kyber not enabled (enable mlkem feature)")
}

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
pub fn kyber1024_encapsulate(_pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Err("Kyber not enabled")
}

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
pub fn kyber768_encapsulate(_pk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Err("Kyber not enabled")
}

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
pub fn kyber1024_decapsulate(_ct: &[u8], _sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    Err("Kyber not enabled")
}

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
pub fn kyber768_decapsulate(_ct: &[u8], _sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    Err("Kyber not enabled")
}

#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
pub fn ml_dsa_653_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    match ml_dsa_65::ml_dsa_65_keypair() {
        Ok(keypair) => Ok((
            ml_dsa_65::ml_dsa_65_serialize_public_key(&keypair.public_key),
            ml_dsa_65::ml_dsa_65_serialize_secret_key(&keypair.secret_key),
        )),
        Err(_) => Err("MlDsa65 keygen failed"),
    }
}

#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
pub fn ml_dsa_653_sign(message: &[u8], sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    let ml_dsa_65_sk =
        ml_dsa_65::ml_dsa_65_deserialize_secret_key(sk).map_err(|_| "Invalid secret key")?;
    match ml_dsa_65::ml_dsa_65_sign(&ml_dsa_65_sk, message) {
        Ok(sig) => Ok(ml_dsa_65::ml_dsa_65_serialize_signature(&sig)),
        Err(_) => Err("Signing failed"),
    }
}

#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
pub fn ml_dsa_653_verify(message: &[u8], sig: &[u8], pk: &[u8]) -> bool {
    let ml_dsa_65_pk = match ml_dsa_65::ml_dsa_65_deserialize_public_key(pk) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let ml_dsa_65_sig = match ml_dsa_65::ml_dsa_65_deserialize_signature(sig) {
        Ok(s) => s,
        Err(_) => return false,
    };
    ml_dsa_65::ml_dsa_65_verify(&ml_dsa_65_pk, message, &ml_dsa_65_sig)
}

#[cfg(not(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5")))]
pub fn ml_dsa_653_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Err("MlDsa65 not enabled (enable mldsa feature)")
}

#[cfg(not(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5")))]
pub fn ml_dsa_653_sign(_message: &[u8], _sk: &[u8]) -> Result<Vec<u8>, &'static str> {
    Err("MlDsa65 not enabled")
}

#[cfg(not(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5")))]
pub fn ml_dsa_653_verify(_message: &[u8], _sig: &[u8], _pk: &[u8]) -> bool {
    false
}

pub fn lattice_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    kyber1024_keypair()
}
