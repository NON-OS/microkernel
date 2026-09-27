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

use super::super::keys::{rsa_public_operation, RsaPublicKey};
use super::digest::{
    pkcs1_digest_info_sha256, pkcs1_digest_info_sha256_no_null, pkcs1_digest_info_sha384,
    pkcs1_digest_info_sha384_no_null, pkcs1_digest_info_sha512, pkcs1_digest_info_sha512_no_null,
};
use super::padding::pkcs1_pad_type1;
use crate::crypto::constant_time::ct_eq;
use crate::crypto::hash::sha256;
use crate::crypto::hash::sha384::sha384;
use crate::crypto::hash::sha512::sha512;
use crate::crypto::util::bigint::BigUint;
use alloc::vec::Vec;

pub fn verify_pkcs1v15(public_key: &RsaPublicKey, message: &[u8], signature: &[u8]) -> bool {
    let hash = sha256(message);
    verify_pkcs1v15_with_digest(
        public_key,
        signature,
        &pkcs1_digest_info_sha256(&hash),
        &pkcs1_digest_info_sha256_no_null(&hash),
    )
}

pub fn verify_pkcs1v15_sha384(public_key: &RsaPublicKey, message: &[u8], signature: &[u8]) -> bool {
    let hash = sha384(message);
    verify_pkcs1v15_with_digest(
        public_key,
        signature,
        &pkcs1_digest_info_sha384(&hash),
        &pkcs1_digest_info_sha384_no_null(&hash),
    )
}

pub fn verify_pkcs1v15_sha512(public_key: &RsaPublicKey, message: &[u8], signature: &[u8]) -> bool {
    let hash = sha512(message);
    verify_pkcs1v15_with_digest(
        public_key,
        signature,
        &pkcs1_digest_info_sha512(&hash),
        &pkcs1_digest_info_sha512_no_null(&hash),
    )
}

fn verify_pkcs1v15_with_digest(
    public_key: &RsaPublicKey,
    signature: &[u8],
    expected_with_null: &[u8],
    expected_no_null: &[u8],
) -> bool {
    let em_len = public_key.bits / 8;
    let Some(em) = recover_encoded_message(public_key, signature, em_len) else {
        return false;
    };
    matches_encoding(&em, expected_with_null) | matches_encoding(&em, expected_no_null)
}

fn recover_encoded_message(
    public_key: &RsaPublicKey,
    signature: &[u8],
    em_len: usize,
) -> Option<Vec<u8>> {
    if signature.len() != em_len {
        return None;
    }
    let s = BigUint::from_bytes_be(signature);
    if s >= public_key.n {
        return None;
    }
    let raw = rsa_public_operation(&s, public_key).ok()?.to_bytes_be();
    if raw.len() > em_len {
        return None;
    }
    let mut em = alloc::vec![0u8; em_len];
    em[em_len - raw.len()..].copy_from_slice(&raw);
    Some(em)
}

fn matches_encoding(em: &[u8], digest_info: &[u8]) -> bool {
    if em.len() < digest_info.len() + 11 {
        return false;
    }
    match pkcs1_pad_type1(digest_info, em.len()) {
        Ok(expected) => ct_eq(em, &expected),
        Err(_) => false,
    }
}

pub fn verify_signature(msg: &[u8], sig: &[u8], key: &RsaPublicKey) -> bool {
    verify_pkcs1v15(key, msg, sig)
}
