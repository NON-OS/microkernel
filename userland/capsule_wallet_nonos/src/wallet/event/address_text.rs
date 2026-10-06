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

//! A 0x address as text: what a paste must be to be taken, and the EIP-55
//! checksum its capital letters carry. A paste is taken whole or not at
//! all, so a hash, a sentence or a cut address never becomes a different
//! address by losing what is not hex. Pure: the keccak of the address is
//! passed in, so wallet_proofs checks the EIP's own examples on the host.

/// Said when a paste is not one address.
pub const NOT_AN_ADDRESS: &str = "That is not an address. An address is 0x and 40 hex digits; \
     the field is unchanged.";

/// The 40 hex digits a pasted text holds, as written, when the text is
/// exactly one address: optional spaces, an optional 0x, 40 hex digits.
pub fn pasted(text: &str) -> Result<[u8; 40], &'static str> {
    let t = text.trim();
    let t = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    let b = t.as_bytes();
    if b.len() != 40 || !b.iter().all(u8::is_ascii_hexdigit) {
        return Err(NOT_AN_ADDRESS);
    }
    let mut out = [0u8; 40];
    out.copy_from_slice(b);
    Ok(out)
}

/// The digits in lower case: what the checksum is computed over.
pub fn lower(hex: &[u8; 40]) -> [u8; 40] {
    let mut out = *hex;
    out.iter_mut().for_each(|c| *c = c.to_ascii_lowercase());
    out
}

/// The EIP-55 spelling of `hex`, given `hash`, the keccak-256 of its
/// lower-case digits: a letter is a capital where its nibble of the hash
/// is 8 or more.
pub fn checksummed(hex: &[u8; 40], hash: &[u8; 32]) -> [u8; 40] {
    let mut out = lower(hex);
    for (i, c) in out.iter_mut().enumerate() {
        let byte = hash[i / 2];
        let nibble = if i % 2 == 0 { byte >> 4 } else { byte & 0x0f };
        if c.is_ascii_alphabetic() && nibble >= 8 {
            *c = c.to_ascii_uppercase();
        }
    }
    out
}

/// Whether the case of `hex` says anything: one written all in lower or
/// all in upper case carries no checksum, any other must be the checksum.
pub fn mixed_case(hex: &[u8; 40]) -> bool {
    let upper = hex.iter().any(u8::is_ascii_uppercase);
    let lower = hex.iter().any(u8::is_ascii_lowercase);
    upper && lower
}

/// Whether the case of `hex` is right, given the keccak of its lower case.
pub fn case_ok(hex: &[u8; 40], hash: &[u8; 32]) -> bool {
    !mixed_case(hex) || checksummed(hex, hash) == *hex
}
