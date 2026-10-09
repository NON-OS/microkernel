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


//! A signed anyone_hosts list (the fork's anyone_hosts_parse.c), checked
//! line by line:
//!
//! ```text
//! anyone-hosts-version 1
//! anyone-hosts-status signed                 (optional)
//! published YYYY-MM-DD HH:MM:SS
//! valid-until YYYY-MM-DD HH:MM:SS
//! <name>.anyone <56 base32>.anyone           (one or more)
//! anyone-hosts-digest sha256 <hex>           (optional)
//! anyone-hosts-signature <signer>.anyone
//! -----BEGIN SIGNATURE-----
//! <base64 Ed25519 signature>
//! -----END SIGNATURE-----
//! ```
//!
//! The signature is Ed25519 under the signer's identity key over
//! "anyone-hosts-signature" followed by SHA-256 of everything from the
//! first byte through the newline ending the signature line. The fork
//! makes published and valid-until optional; this client requires both,
//! since without them neither expiry nor rollback can be checked. Any line
//! it does not know refuses the whole list.

extern crate alloc;

use alloc::vec::Vec;

use super::{normal, LIST_MAX, NAMES_MAX};
use crate::crypto::{ed25519_verify, sha256};
use crate::directory::base64;
use crate::directory::time;

const SIGN_PREFIX: &[u8] = b"anyone-hosts-signature";
const ADDRESS_LEN: usize = 56 + 7;

/// A verified list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameList {
    pub published: u64,
    pub valid_until: u64,
    /// Each name, lowercased, and the identity key it names.
    pub entries: Vec<(Vec<u8>, [u8; 32])>,
}

impl NameList {
    pub fn lookup(&self, name: &[u8]) -> Option<[u8; 32]> {
        self.entries.iter().find(|(n, _)| n.as_slice() == name).map(|(_, id)| *id)
    }
}

/// Why a list was refused, one reason each.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NameError {
    TooLarge,
    /// A line this client does not know, or one out of place.
    Malformed,
    /// Not version 1.
    Version,
    /// No signature, or no published or valid-until time.
    Unsigned,
    /// Signed by a key that is not one of the six services'.
    UntrustedSigner,
    BadSignature,
    Expired,
    /// Signed and current, but with no names in it.
    Empty,
    /// One name given two services.
    Ambiguous,
}

impl NameError {
    pub fn said(self) -> &'static [u8] {
        match self {
            Self::TooLarge => b"names list refused: too large",
            Self::Malformed => b"names list refused: malformed",
            Self::Version => b"names list refused: unknown version",
            Self::Unsigned => b"names list refused: unsigned or undated",
            Self::UntrustedSigner => b"names list refused: signer is not an Anyone DNS service",
            Self::BadSignature => b"names list refused: signature does not verify",
            Self::Expired => b"names list refused: expired",
            Self::Empty => b"names list refused: no names",
            Self::Ambiguous => b"names list refused: a name given two services",
        }
    }
}

/// Check `body`, signed by one of `trusted`, at `now`.
pub fn verify(body: &[u8], trusted: &[[u8; 32]], now: u64) -> Result<NameList, NameError> {
    if body.len() > LIST_MAX {
        return Err(NameError::TooLarge);
    }
    if !body.iter().all(|b| *b == b'\n' || (0x20..0x7f).contains(b) || *b == b'\t') {
        return Err(NameError::Malformed);
    }
    let mut lines = Lines { body, at: 0 };
    if lines.next() != Some(b"anyone-hosts-version 1".as_slice()) {
        return Err(if body.starts_with(b"anyone-hosts-version") { NameError::Version } else { NameError::Unsigned });
    }

    let (mut status, mut published, mut valid_until, mut digest) = (false, None, None, false);
    let mut mappings: Vec<(&[u8], &[u8])> = Vec::new();
    let signer = loop {
        let Some(line) = lines.next() else { return Err(NameError::Unsigned) };
        if line.is_empty() {
            continue;
        }
        let (keyword, rest) = split(line);
        match keyword {
            b"anyone-hosts-status" if !status && !rest.is_empty() && !rest.contains(&b' ') => status = true,
            b"published" if published.is_none() => published = Some(date(rest)?),
            b"valid-until" if valid_until.is_none() => valid_until = Some(date(rest)?),
            b"anyone-hosts-digest" if !digest => digest = true,
            b"anyone-hosts-signature" if !rest.is_empty() && !rest.contains(&b' ') => break rest,
            b"anyone-hosts-version" | b"anyone-hosts-status" | b"published" | b"valid-until" | b"anyone-hosts-digest"
            | b"anyone-hosts-signature" => return Err(NameError::Malformed),
            _ => {
                if mappings.len() >= NAMES_MAX {
                    return Err(NameError::TooLarge);
                }
                if keyword.is_empty() || rest.is_empty() || rest.iter().any(|b| *b == b' ' || *b == b'\t') {
                    return Err(NameError::Malformed);
                }
                mappings.push((keyword, rest));
            }
        }
    };
    let signed_end = lines.at;

    if lines.next() != Some(b"-----BEGIN SIGNATURE-----".as_slice()) {
        return Err(NameError::Malformed);
    }
    let mut encoded = Vec::new();
    loop {
        match lines.next() {
            Some(b"-----END SIGNATURE-----") => break,
            Some(line) if line.len() <= 76 => encoded.extend_from_slice(line),
            _ => return Err(NameError::Malformed),
        }
    }
    for line in lines {
        if !line.is_empty() {
            return Err(NameError::Malformed);
        }
    }
    /* Only the one encoding of 64 bytes: 86 symbols and "==". The directory
     * decoder stops at the first "=", so without this anything after the
     * padding would be read past unseen. */
    let canonical = encoded.len() == 88
        && encoded.ends_with(b"==")
        && encoded[..86].iter().all(|b| b.is_ascii_alphanumeric() || *b == b'+' || *b == b'/');
    if !canonical {
        return Err(NameError::Malformed);
    }
    let signature: [u8; 64] = base64::decode(&encoded).ok_or(NameError::Malformed)?.try_into().map_err(|_| NameError::Malformed)?;
    let (Some(published), Some(valid_until)) = (published, valid_until) else {
        return Err(NameError::Unsigned);
    };

    if signer.len() != ADDRESS_LEN {
        return Err(NameError::UntrustedSigner);
    }
    let key = crate::onion::address::parse(signer).ok_or(NameError::UntrustedSigner)?;
    if !trusted.contains(&key) {
        return Err(NameError::UntrustedSigner);
    }
    let digest = sha256(&body[..signed_end]).map_err(|_| NameError::BadSignature)?;
    let mut message = Vec::with_capacity(SIGN_PREFIX.len() + 32);
    message.extend_from_slice(SIGN_PREFIX);
    message.extend_from_slice(&digest);
    if !ed25519_verify(&key, &message, &signature) {
        return Err(NameError::BadSignature);
    }

    if valid_until < now || published > valid_until {
        return Err(NameError::Expired);
    }
    let mut entries: Vec<(Vec<u8>, [u8; 32])> = Vec::new();
    for (name, target) in mappings {
        let name = normal(name);
        if !is_short_name(&name) || target.len() != ADDRESS_LEN {
            return Err(NameError::Malformed);
        }
        let id = crate::onion::address::parse(target).ok_or(NameError::Malformed)?;
        match entries.iter().find(|(n, _)| *n == name) {
            Some((_, other)) if *other != id => return Err(NameError::Ambiguous),
            Some(_) => {}
            None => entries.push((name, id)),
        }
    }
    if entries.is_empty() {
        return Err(NameError::Empty);
    }
    Ok(NameList { published, valid_until, entries })
}

/// A short name: lowercase letters, digits and hyphens in non-empty labels,
/// ending in `.anyone`, no longer than a host name, and not itself an
/// address (whose last label before the suffix is 56 characters).
pub fn is_short_name(name: &[u8]) -> bool {
    let Some(stem) = name.strip_suffix(b".anyone") else { return false };
    if stem.is_empty() || name.len() > 255 {
        return false;
    }
    let labels_ok = stem.split(|b| *b == b'.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && label.iter().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
    });
    let last = stem.rsplit(|b| *b == b'.').next().unwrap_or(stem);
    labels_ok && last.len() != 56
}

fn split(line: &[u8]) -> (&[u8], &[u8]) {
    match line.iter().position(|b| *b == b' ' || *b == b'\t') {
        Some(at) => {
            let rest = &line[at + 1..];
            let skip = rest.iter().take_while(|b| **b == b' ' || **b == b'\t').count();
            (&line[..at], &rest[skip..])
        }
        None => (line, &[]),
    }
}

fn date(text: &[u8]) -> Result<u64, NameError> {
    if text.len() != 19 {
        return Err(NameError::Malformed);
    }
    time::parse(text).ok_or(NameError::Malformed)
}

/// Lines split on newline, with `at` just past the last one taken.
struct Lines<'a> {
    body: &'a [u8],
    at: usize,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.at >= self.body.len() {
            return None;
        }
        let rest = &self.body[self.at..];
        match rest.iter().position(|b| *b == b'\n') {
            Some(n) => {
                self.at += n + 1;
                Some(&rest[..n])
            }
            None => {
                self.at = self.body.len();
                Some(rest)
            }
        }
    }
}
