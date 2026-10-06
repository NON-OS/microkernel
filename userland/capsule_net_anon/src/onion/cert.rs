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


//! Checking the Ed25519 certificates an onion descriptor carries.

use crate::link::constants::SECONDS_PER_HOUR;
use crate::link::ed_cert::parse;

/// cert-spec section A.1: the descriptor signing key, certified by the
/// blinded key; an introduction point's auth key and its encryption key's
/// cross certificate, both certified by the descriptor signing key.
pub const TYPE_DESC_SIGNING: u8 = 0x08;
pub const TYPE_INTRO_AUTH: u8 = 0x09;
pub const TYPE_INTRO_ENC: u8 = 0x0B;

/// The key `cert` certifies, when it is of type `kind`, names its signing
/// key in the signed-with extension (cert_is_valid requires it), that key is
/// `signer` when one is given, the signature verifies under it and it has
/// not expired at `now`. Returns the certified key and the signing key.
pub fn check(cert: &[u8], kind: u8, signer: Option<&[u8; 32]>, now: u64) -> Option<([u8; 32], [u8; 32])> {
    let parsed = parse(cert)?;
    if parsed.cert_type != kind {
        return None;
    }
    let signing = parsed.signed_with?;
    if signer.is_some_and(|want| *want != signing) {
        return None;
    }
    if now >= (parsed.expiry_hours as u64).saturating_mul(SECONDS_PER_HOUR) {
        return None;
    }
    let signature = nonos_ed25519::Signature::from_bytes(&parsed.signature);
    if !nonos_ed25519::verify(&signing, parsed.signed, &signature) {
        return None;
    }
    Some((parsed.certified_key, signing))
}
