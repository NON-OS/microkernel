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

//! The certificate chain check that binds a TLS session to a relay.

use crate::crypto::{ed25519_verify, equal, sha256};

use super::super::constants::SECONDS_PER_HOUR;
use super::super::ed_cert::EdCert;
use super::error::BindError;
use super::find::find;

/*
 * The rule is or_handshake_certs_ed25519_ok in torcert.c: cert 4 carries the
 * signing key and must include the identity key that signed it, so it verifies
 * against its own extension; cert 5 carries SHA-256 of the peer's DER
 * certificate as its certified key and verifies against cert 4's signing key;
 * neither may have expired.
 *
 * The identity that falls out is then compared with the one the consensus
 * published. Without that last step the chain proves some relay signed it, not
 * that this is the relay we meant to reach.
 */
pub fn bind(certs: &[u8], leaf: &[u8], identity: &[u8; 32], now: u64) -> Result<(), BindError> {
    let (signing, link) = find(certs)?;
    let named = signing.signed_with.ok_or(BindError::MissingSigningCert)?;
    if expired(&signing, now) || expired(&link, now) {
        return Err(BindError::Expired);
    }
    if !equal(&named, identity) {
        return Err(BindError::WrongIdentity);
    }
    if !ed25519_verify(&named, signing.signed, &signing.signature) {
        return Err(BindError::BadSignature);
    }
    if !ed25519_verify(&signing.certified_key, link.signed, &link.signature) {
        return Err(BindError::BadSignature);
    }
    let digest = sha256(leaf).map_err(|_| BindError::Malformed)?;
    if !equal(&link.certified_key, &digest) {
        return Err(BindError::WrongSession);
    }
    Ok(())
}

fn expired(cert: &EdCert<'_>, now: u64) -> bool {
    (cert.expiry_hours as u64).saturating_mul(SECONDS_PER_HOUR) < now
}
