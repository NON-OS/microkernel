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

//! Signing a body with the authority's key, and checking the result as
//! the client will: a certificate that does not parse, names another
//! authority or fails either half of its signature is never written.

use nonos_ed25519::Signature;
use nsh_crypto::{labelled, Label};
use nsh_wire::keys::HYBRID_SIG_LEN;
use nsh_wire::Enrol;

use crate::keys::{AuthorityKey, PUBLIC_FILE_LEN};
use crate::Error;

pub fn sign(body: &[u8], key: &AuthorityKey, now_ms: u64) -> Result<Vec<u8>, Error> {
    let shaped = [body, &[0u8; HYBRID_SIG_LEN][..]].concat();
    let e = Enrol::parse(&shaped).map_err(|_| Error::Value("certificate body"))?;
    if *e.authority != key.id() {
        return Err(Error::Value("authority: the body names another key"));
    }
    if e.not_after_ms <= now_ms {
        return Err(Error::Value("validity: the body has run out"));
    }
    let digest = labelled(Label::Enrol, body);
    let ed = nonos_ed25519::sign(&key.ed, &digest).to_bytes();
    let ml = nonos_mldsa65::sign(&key.ml_secret, &digest, &mut crate::rand::source())
        .map_err(|_| Error::Randomness)?;
    let cert = [body, &ed[..], &ml[..]].concat();
    let public: &[u8; PUBLIC_FILE_LEN] = &key.public().try_into().map_err(|_| Error::KeyFile)?;
    verify(&cert, public)?;
    Ok(cert)
}

/// The certificate, read and checked against an authority's public key.
pub fn verify<'a>(cert: &'a [u8], authority: &[u8; PUBLIC_FILE_LEN]) -> Result<Enrol<'a>, Error> {
    let e = Enrol::parse(cert).map_err(|_| Error::SelfCheck)?;
    if *e.authority != nonos_hash::sha256(authority) {
        return Err(Error::SelfCheck);
    }
    let digest = labelled(Label::Enrol, e.signed);
    let ed_pub: &[u8; 32] = authority[..32].try_into().map_err(|_| Error::SelfCheck)?;
    let ml_pub: &[u8; 1952] = authority[32..].try_into().map_err(|_| Error::SelfCheck)?;
    let ed = nonos_ed25519::verify(ed_pub, &digest, &Signature::from_bytes(e.sig.ed));
    match ed && nonos_mldsa65::verify(ml_pub, &digest, e.sig.ml) {
        true => Ok(e),
        false => Err(Error::SelfCheck),
    }
}
