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


//! An Anyone onion address: `<56 base32 characters>.anyone`.
//!
//! The fork keeps Tor's v3 layout (rend-spec-v3 section 6) and changes the
//! suffix and the checksum prefix: `.anyone` and ".anyone checksum"
//! (HS_SERVICE_ADDR_SUFFIX and HS_SERVICE_ADDR_CHECKSUM_PREFIX in its
//! src/feature/hs/hs_common.h). An address is the service's Ed25519 identity
//! key, two checksum bytes and the version, base32 encoded.

use crate::crypto::keccak::sha3_256_parts;

/// The suffix a host name must end in to be looked up as an onion service.
pub const SUFFIX: &[u8] = b".anyone";
const CHECKSUM_PREFIX: &[u8] = b".anyone checksum";
const VERSION: u8 = 3;
const ENCODED_BYTES: usize = 56;

/// Whether `host` names an onion service at all. A name that ends in the
/// suffix but is not a valid address is still an onion name: it is refused,
/// never sent to an exit as a host name. The root's trailing dot names the
/// same thing (`x.anyone.`), and nonos_route_link passes it through as
/// written, so it is an onion name too; read without it, it went to an exit
/// in a BEGIN and named the service there.
pub fn is_onion(host: &[u8]) -> bool {
    /* Every trailing dot is stripped here, so a name malformed that way is
     * still caught as an onion name and refused below, not sent out. */
    let mut host = host;
    while let Some(shorter) = host.strip_suffix(b".") {
        host = shorter;
    }
    host.len() >= SUFFIX.len() && host[host.len() - SUFFIX.len()..].eq_ignore_ascii_case(SUFFIX)
}

/// `host` without one trailing root dot.
fn rooted(host: &[u8]) -> &[u8] {
    host.strip_suffix(b".").unwrap_or(host)
}

/// The identity key `host` names. Any labels before the address are
/// ignored, as Tor ignores them (`www.<address>.anyone` names the same
/// service). `None` for a wrong length, a bad character, a version other
/// than 3 or a checksum that does not match.
pub fn parse(host: &[u8]) -> Option<[u8; 32]> {
    if !is_onion(host) {
        return None;
    }
    let host = rooted(host);
    if !host[host.len() - SUFFIX.len()..].eq_ignore_ascii_case(SUFFIX) {
        return None;
    }
    let name = &host[..host.len() - SUFFIX.len()];
    let label = match name.iter().rposition(|b| *b == b'.') {
        Some(dot) => &name[dot + 1..],
        None => name,
    };
    if label.len() != ENCODED_BYTES {
        return None;
    }
    let raw = base32(label)?;
    if raw[34] != VERSION {
        return None;
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&raw[..32]);
    let sum = checksum(&key);
    if raw[32..34] != sum {
        return None;
    }
    Some(key)
}

/// The address of the service whose identity key is `key`, lowercase, the
/// suffix included: what a resolved short name is shown as.
pub fn encode(key: &[u8; 32]) -> [u8; ENCODED_BYTES + 7] {
    let mut raw = [0u8; 35];
    raw[..32].copy_from_slice(key);
    raw[32..34].copy_from_slice(&checksum(key));
    raw[34] = VERSION;
    let alphabet = b"abcdefghijklmnopqrstuvwxyz234567";
    let mut out = [0u8; ENCODED_BYTES + 7];
    let (mut acc, mut bits, mut at) = (0u64, 0u32, 0usize);
    for byte in raw {
        acc = (acc << 8) | u64::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out[at] = alphabet[((acc >> bits) & 31) as usize];
            at += 1;
        }
    }
    out[ENCODED_BYTES..].copy_from_slice(SUFFIX);
    out
}

fn checksum(key: &[u8; 32]) -> [u8; 2] {
    let digest = sha3_256_parts(&[CHECKSUM_PREFIX, key, &[VERSION]]);
    [digest[0], digest[1]]
}

/// RFC 4648 base32 without padding, either case. 56 characters carry 280
/// bits, exactly 35 bytes, so there are no leftover bits to check.
fn base32(text: &[u8]) -> Option<[u8; 35]> {
    let mut out = [0u8; 35];
    let mut acc = 0u64;
    let mut bits = 0u32;
    let mut at = 0usize;
    for c in text {
        let value = match c {
            b'a'..=b'z' => c - b'a',
            b'A'..=b'Z' => c - b'A',
            b'2'..=b'7' => c - b'2' + 26,
            _ => return None,
        };
        acc = (acc << 5) | value as u64;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            *out.get_mut(at)? = (acc >> bits) as u8;
            at += 1;
        }
    }
    (at == 35).then_some(out)
}
