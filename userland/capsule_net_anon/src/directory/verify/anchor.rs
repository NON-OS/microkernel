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

//! Tying an authority certificate to a hardcoded v3 identity.

use crate::crypto::{equal, sha1_digest};

use super::cert::AuthorityCert;
use super::pool::{rsa_verify, HASH_SHA1};
use super::spki::wrap;

/// Why a certificate was not accepted for an authority.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnchorError {
    WrongAuthority,
    BadCertification,
    Expired,
}

/*
 * The identity key's SHA-1 is the v3 identity, and it is taken over the PKCS#1
 * RSAPublicKey DER the document carries, not over the SubjectPublicKeyInfo the
 * verifier wants. Hashing the wrapped form gives a fingerprint that matches no
 * authority, so both forms of the one key are used here.
 *
 * The certification is a self-signature by the identity key over the
 * certificate's own span, digested with SHA-1. That is the whole reason the pool
 * had to learn a twenty byte digest under the unprefixed scheme.
 */
pub fn check(
    cert: &AuthorityCert,
    body: &[u8],
    v3ident: &[u8; 20],
    now: u64,
) -> Result<(), AnchorError> {
    let fingerprint = sha1_digest(&cert.identity_pkcs1);
    if !equal(&fingerprint, v3ident) {
        return Err(AnchorError::WrongAuthority);
    }
    let (from, to) = cert.signed;
    let span = body.get(from..to).ok_or(AnchorError::BadCertification)?;
    let digest = sha1_digest(span);
    let spki = wrap(&cert.identity_pkcs1);
    if !rsa_verify(HASH_SHA1, &spki, &cert.certification, &digest) {
        return Err(AnchorError::BadCertification);
    }
    /*
     * Last, and only on a certificate that has already proved itself. A properly
     * signed certificate past its expiry is a rotated key, which is a different
     * thing from a forged one and is usually the clock; checked in this order the
     * two cannot be confused for each other.
     */
    if now >= cert.expires {
        return Err(AnchorError::Expired);
    }
    Ok(())
}
