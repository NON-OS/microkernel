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

//! The account's address as the 0x text a person reads, copies and scans.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::state::State;

/// The address in lower case, as the services it is handed to compare it.
pub fn address_hex(state: &State) -> String {
    let mut hex = String::from("0x");
    for b in state.address {
        hex.push_str(&alloc::format!("{b:02x}"));
    }
    hex
}

/// The address as a person reads and copies it: 0x and its EIP-55
/// spelling, whose capitals let another wallet catch a mistyped digit.
pub fn address_text(state: &State) -> String {
    checksummed_text(&state.address)
}

/// Any 20-byte address in its EIP-55 spelling. Lower case, which is the
/// same address with no checksum, only if keccak cannot be had.
pub fn checksummed_text(address: &[u8; 20]) -> String {
    let mut hex = [0u8; 40];
    for (i, b) in address.iter().enumerate() {
        hex[i * 2] = HEX[(b >> 4) as usize];
        hex[i * 2 + 1] = HEX[(b & 15) as usize];
    }
    let shown =
        keccak(&hex).map_or(hex, |h| crate::wallet::event::address_text::checksummed(&hex, &h));
    let mut out = String::with_capacity(42);
    out.push_str("0x");
    out.extend(shown.iter().map(|c| char::from(*c)));
    out
}

/// keccak-256 of an address's lower-case digits, from the crypto service.
pub fn keccak(hex: &[u8; 40]) -> Option<[u8; 32]> {
    let lower = crate::wallet::event::address_text::lower(hex);
    let mut out = [0u8; 32];
    crate::wallet::tx_hash::tx_hash(&lower, &mut out).then_some(out)
}

const HEX: &[u8; 16] = b"0123456789abcdef";

/// What is known about the address, as facts; returns their height.
pub fn facts(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let key = if state.vault_saved { "sealed to this machine" } else { "RAM only, gone at reboot" };
    let h = fact(fb, c.x, y, c.w, "network", crate::wallet::chain::current().name);
    h + fact(fb, c.x, y + h, c.w, "key", key)
}
