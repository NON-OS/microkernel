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

//! Which RSA scheme a request selects, and the digest width that goes with it.

use rsa::pkcs8::DecodePublicKey;
use rsa::pss::Pss;
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use sha2::{Sha256, Sha384, Sha512};

/*
 * hashid 3 is a twenty byte digest and is reachable only under scheme 2. A
 * directory authority certificate is signed over a SHA-1 digest with no
 * DigestInfo, so that length has to be accepted for the chain to be checkable
 * at all. It is deliberately not paired with a prefixed scheme: there is no
 * reason to sign a new SHA-1 DigestInfo and every reason not to offer one.
 */
/// The digest length `hashid` names, or `None` if the pair is not offered.
pub fn digest_len(scheme: u8, hashid: u8) -> Option<usize> {
    match hashid {
        0 => Some(32),
        1 => Some(48),
        2 => Some(64),
        3 if scheme == 2 => Some(20),
        _ => None,
    }
}

/*
 * Scheme 2 is PKCS#1 v1.5 with no DigestInfo prefix: the padded block holds the
 * bare digest and nothing saying which hash produced it. TLS never does that and
 * the Anyone and Tor directory protocols always do, so a verifier built for TLS
 * rejects every directory signature.
 *
 * `new_unprefixed` reconstructs the whole padded block and compares it, so this
 * is a constructing verifier, not the parsing shape Bleichenbacher exploits.
 */
/// Verify `sig` over `digest` under the named scheme.
///
pub fn verify(scheme: u8, hashid: u8, spki: &[u8], sig: &[u8], digest: &[u8]) -> Option<bool> {
    if digest.len() != digest_len(scheme, hashid)? {
        return None;
    }
    let key = RsaPublicKey::from_public_key_der(spki).ok()?;
    Some(match (scheme, hashid) {
        (0, 0) => key.verify(Pkcs1v15Sign::new::<Sha256>(), digest, sig).is_ok(),
        (0, 1) => key.verify(Pkcs1v15Sign::new::<Sha384>(), digest, sig).is_ok(),
        (0, 2) => key.verify(Pkcs1v15Sign::new::<Sha512>(), digest, sig).is_ok(),
        (1, 0) => key.verify(Pss::new::<Sha256>(), digest, sig).is_ok(),
        (1, 1) => key.verify(Pss::new::<Sha384>(), digest, sig).is_ok(),
        (1, 2) => key.verify(Pss::new::<Sha512>(), digest, sig).is_ok(),
        (2, _) => key.verify(Pkcs1v15Sign::new_unprefixed(), digest, sig).is_ok(),
        _ => return None,
    })
}
