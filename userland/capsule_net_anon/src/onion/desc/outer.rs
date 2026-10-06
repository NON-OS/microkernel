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


//! The outer, plaintext layer of a descriptor and its signature
//! (desc_decode_plaintext_v3 and desc_sig_is_valid in the fork).

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::base64::decode;
use crate::directory::consensus::object_after;
use crate::directory::lines::{arg, lines};
use crate::directory::number::decimal;

use super::super::cert::{check, TYPE_DESC_SIGNING};
use super::error::DescError;

const SIG_PREFIX: &[u8] = b"Tor onion service descriptor sig v3";
/// HS_DESC_MAX_LIFETIME, twelve hours, in minutes.
const LIFETIME_MAX: u64 = 12 * 60;

/// What the outer layer gives, once its signature has been checked.
pub struct Outer {
    pub revision: u64,
    /// How long the descriptor may be used, in seconds.
    pub lifetime: u64,
    /// The descriptor signing key, which certifies the introduction points.
    pub signing_key: [u8; 32],
    pub superencrypted: Vec<u8>,
}

/// Parse and check the outer layer of `body` for a service whose blinded
/// key for this period is `blinded`.
pub fn outer(body: &[u8], blinded: &[u8; 32], now: u64) -> Result<Outer, DescError> {
    if !body.starts_with(b"hs-descriptor 3\n") {
        return Err(DescError::Malformed);
    }
    let mut lifetime = None;
    let mut cert = None;
    let mut revision = None;
    let mut superencrypted = None;
    let mut signature = None;
    for line in lines(body) {
        let slot_taken = match line.keyword {
            b"descriptor-lifetime" => lifetime.replace(arg(line.rest, 0).and_then(decimal)).is_some(),
            b"descriptor-signing-key-cert" => cert.replace(object_after(body, line.at)).is_some(),
            b"revision-counter" => revision.replace(arg(line.rest, 0).and_then(decimal)).is_some(),
            b"superencrypted" => superencrypted.replace(object_after(body, line.at)).is_some(),
            b"signature" => signature.replace((line.at, arg(line.rest, 0))).is_some(),
            _ => false,
        };
        /* Each of these is exactly once in the fork's token table. A second
         * copy is a document two parsers could read two ways. */
        if slot_taken {
            return Err(DescError::Malformed);
        }
    }
    let lifetime = lifetime.flatten();
    let cert = cert.flatten();
    let revision = revision.flatten();
    let superencrypted = superencrypted.flatten();
    let lifetime = lifetime.ok_or(DescError::Malformed)?;
    if lifetime == 0 || lifetime > LIFETIME_MAX {
        return Err(DescError::Malformed);
    }
    let cert = cert.ok_or(DescError::Malformed)?;
    let revision = revision.ok_or(DescError::Malformed)?;
    let superencrypted = superencrypted.ok_or(DescError::Malformed)?;
    let (sig_at, sig_text) = signature.ok_or(DescError::Malformed)?;

    let (signing_key, certifier) =
        check(&cert, TYPE_DESC_SIGNING, None, now).ok_or(DescError::Certificate)?;
    /* The blinded key is what this client derived for the address; a
     * descriptor certified by any other key is for some other service. */
    if certifier != *blinded {
        return Err(DescError::WrongService);
    }

    /* The signature covers the document up to the start of its own line,
     * prefixed, and that line must be the last. */
    let sig_bytes = decode(sig_text.ok_or(DescError::Malformed)?).ok_or(DescError::Malformed)?;
    let sig: [u8; 64] = sig_bytes.try_into().map_err(|_| DescError::Malformed)?;
    let tail = &body[sig_at..];
    let line_end = tail.iter().position(|b| *b == b'\n').unwrap_or(tail.len());
    if tail[line_end..].iter().any(|b| !b.is_ascii_whitespace()) {
        return Err(DescError::Malformed);
    }
    let mut message = Vec::with_capacity(SIG_PREFIX.len() + sig_at);
    message.extend_from_slice(SIG_PREFIX);
    message.extend_from_slice(&body[..sig_at]);
    if !nonos_ed25519::verify(&signing_key, &message, &nonos_ed25519::Signature::from_bytes(&sig)) {
        return Err(DescError::Signature);
    }
    Ok(Outer { revision, lifetime: lifetime * 60, signing_key, superencrypted })
}
