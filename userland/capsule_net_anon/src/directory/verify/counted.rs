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

//! Whether one signature line counts toward the quorum.

use crate::crypto::{equal, sha1_digest};
use crate::directory::authority::AUTHORITIES;
use crate::directory::consensus::Signature;

use super::cert::AuthorityCert;
use super::pool::{rsa_verify, HASH_SHA256};
use super::spki::wrap;

/// The authority index this signature counts for, or `None`.
pub(super) fn accepts(
    signature: &Signature,
    digest: &[u8; 32],
    certs: &[(usize, AuthorityCert)],
) -> Option<usize> {
    if !signature.sha256 {
        return None;
    }
    let index = AUTHORITIES.iter().position(|a| equal(&a.v3ident, &signature.identity))?;
    let (_, cert) = certs.iter().find(|(held, _)| *held == index)?;
    if !equal(&sha1_digest(&cert.signing_pkcs1), &signature.signing_key) {
        return None;
    }
    let spki = wrap(&cert.signing_pkcs1);
    if !rsa_verify(HASH_SHA256, &spki, &signature.bytes, digest) {
        return None;
    }
    Some(index)
}
