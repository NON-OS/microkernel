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

//! A CertificateVerify signature checked with the leaf's own ECDSA key.

pub fn p256(point: &[u8], sig_der: &[u8], content: &[u8]) -> bool {
    let (Ok(pk), Some(digest)) =
        (<[u8; 65]>::try_from(point), super::hash_sha256::hash_sha256(content))
    else {
        return false;
    };
    let mut sig = [0u8; 64];
    super::ecdsa_sig_raw::ecdsa_sig_raw(sig_der, 32, &mut sig)
        && super::verify_p256::verify_p256(&pk, &sig, &digest)
}

pub fn p384(point: &[u8], sig_der: &[u8], content: &[u8]) -> bool {
    let (Ok(pk), Some(digest)) =
        (<[u8; 97]>::try_from(point), super::hash_sha384::hash_sha384(content))
    else {
        return false;
    };
    let mut sig = [0u8; 96];
    super::ecdsa_sig_raw::ecdsa_sig_raw(sig_der, 48, &mut sig)
        && super::verify_p384::verify_p384(&pk, &sig, &digest)
}
