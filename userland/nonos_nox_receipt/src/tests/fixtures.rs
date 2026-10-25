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

//! Payment verification against real-shaped receipt JSON.
//!
//! Topics and amounts are built from typed values so the fixtures cannot drift
//! from what they claim to encode. One good receipt is accepted, and every way
//! a receipt can fail to prove the payment is shown to be refused: reverted, no
//! status, wrong token, wrong recipient, too little, a non-transfer event, and
//! a decoy transfer beside the real one.

use crate::verify::TRANSFER_TOPIC;

// Whoever the listing names; nothing here is a house account.
pub const PAYEE: [u8; 20] = [0x5a; 20];
pub const PRICE: u128 = 250_000_000_000_000_000;
pub const BUYER: [u8; 20] = [
    0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x11, 0x22, 0x33,
];

/// The live NOX token, from the one place that names it.
pub fn token() -> [u8; 20] {
    nonos_nox_config::address(nonos_nox_config::MAINNET, nonos_nox_config::Contract::NoxToken)
        .expect("NOX is deployed on mainnet")
}

pub fn hex_of(bytes: &[u8]) -> String {
    let mut s = String::from("0x");
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xF) as u32, 16).unwrap());
    }
    s
}

/// A 20-byte address as a left-padded 32-byte indexed topic.
pub fn addr_topic(addr: &[u8; 20]) -> String {
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(addr);
    hex_of(&word)
}

/// A u128 amount as a 32-byte EVM word.
pub fn amount_word(v: u128) -> String {
    let mut word = [0u8; 32];
    word[16..].copy_from_slice(&v.to_be_bytes());
    hex_of(&word)
}

pub fn transfer_topic() -> String {
    hex_of(&TRANSFER_TOPIC)
}

pub fn paid_topics() -> [String; 3] {
    [transfer_topic(), addr_topic(&BUYER), addr_topic(&PAYEE)]
}

pub fn token_addr() -> String {
    hex_of(&token())
}
