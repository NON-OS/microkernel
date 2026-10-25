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

//! A 0x address as text: what a paste must be to be taken, and whether the
//! capital letters it carries are its EIP-55 checksum. A paste is taken
//! whole or not at all, so a hash, a sentence or a cut address never
//! becomes a different address by losing what is not hex. The keccak is
//! passed in, so wallet_proofs checks the EIP's own examples on the host.

use nonos_marketplace_abi::{address_text, Keccak};

use crate::wallet::event::hex_digit::hex_digit;

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

/// Whether the case of `hex` says anything: one written all in lower or
/// all in upper case carries no checksum, any other must be the checksum.
pub fn mixed_case(hex: &[u8; 40]) -> bool {
    let upper = hex.iter().any(u8::is_ascii_uppercase);
    let lower = hex.iter().any(u8::is_ascii_lowercase);
    upper && lower
}

/// Whether the case of `hex` is right: none to check, or the EIP-55
/// spelling of the address it reads as, marketplace_abi's address_text.
/// Only a paste already read as 40 hex digits is asked about.
pub fn case_ok(k: &impl Keccak, hex: &[u8; 40]) -> bool {
    let digit = |c: u8| hex_digit(u32::from(c)).unwrap_or(0);
    let a: [u8; 20] = core::array::from_fn(|i| (digit(hex[2 * i]) << 4) | digit(hex[2 * i + 1]));
    !mixed_case(hex) || address_text(k, &a)[2..] == *hex
}
