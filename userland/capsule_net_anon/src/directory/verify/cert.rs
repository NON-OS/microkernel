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

//! An authority certificate: its keys and the span its signature covers.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::consensus::object_after;
use crate::directory::lines::lines;
use crate::directory::time::parse as time_parse;

/// The two keys and the signature an authority certificate carries.
pub struct AuthorityCert {
    /// PKCS#1 RSAPublicKey DER. Its SHA-1 is the v3 identity.
    pub identity_pkcs1: Vec<u8>,
    /// PKCS#1 RSAPublicKey DER of the key that signs consensuses.
    pub signing_pkcs1: Vec<u8>,
    /// The certification signature, made by the identity key.
    pub certification: Vec<u8>,
    /// The byte range the certification covers.
    pub signed: (usize, usize),
    /*
     * dir-spec 3.1 puts `dir-key-expires` on every authority certificate, and an
     * authority rotates its signing key before that date. A client ignoring it
     * keeps accepting consensuses signed by a retired key, and a key is often
     * retired precisely because it should no longer be trusted.
     */
    /// When the signing key stops being usable, in seconds since the epoch.
    pub expires: u64,
}

/// Parse one certificate out of `body`.
///
pub fn parse(body: &[u8]) -> Option<AuthorityCert> {
    let mut identity = None;
    let mut signing = None;
    let mut certification = None;
    let mut expires = None;
    for line in lines(body) {
        match line.keyword {
            b"dir-identity-key" => identity = object_after(body, line.at),
            b"dir-signing-key" => signing = object_after(body, line.at),
            b"dir-key-certification" => certification = object_after(body, line.at),
            b"dir-key-expires" => expires = time_parse(line.rest),
            _ => {}
        }
    }
    Some(AuthorityCert {
        identity_pkcs1: identity?,
        signing_pkcs1: signing?,
        certification: certification?,
        signed: super::span::cert_range(body)?,
        // A certificate with no readable expiry is refused rather than treated as
        // never expiring: the field is required, and the permissive reading of a
        // missing one is the reading an attacker would want.
        expires: expires?,
    })
}
