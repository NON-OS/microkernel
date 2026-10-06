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

/*
 * An address pasted into Send is taken whole or not at all, and the
 * capitals of a mixed-case one must be its EIP-55 checksum: the EIP's own
 * examples, keccak computed here as the crypto service computes it.
 */

use crate::address_text::{case_ok, checksummed, lower, mixed_case, pasted, NOT_AN_ADDRESS};
use sha3::{Digest, Keccak256};

fn hash(hex: &[u8; 40]) -> [u8; 32] {
    Keccak256::digest(lower(hex)).into()
}

fn digits(text: &str) -> [u8; 40] {
    pasted(text).expect("an address")
}

/* From EIP-55. */
const EXAMPLES: [&str; 6] = [
    "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
    "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
    "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
    "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
    "0x52908400098527886E0F7030069857D2E4169EE7",
    "0x8617E340B3D01FA5F11F306F4090FD50E238070D",
];

#[test]
fn the_eips_examples_are_their_own_checksum() {
    for text in EXAMPLES {
        let hex = digits(text);
        let lowered = lower(&hex);
        let spelled = checksummed(&lowered, &hash(&hex));
        assert_eq!(&spelled[..], &text.as_bytes()[2..], "{text}");
        assert!(case_ok(&hex, &hash(&hex)));
    }
}

#[test]
fn one_wrong_capital_is_caught() {
    let mut hex = digits(EXAMPLES[0]);
    /* 'a' at index 1 is a capital A in the checksum: lower it back. */
    let at = hex.iter().position(u8::is_ascii_uppercase).expect("a capital");
    hex[at] = hex[at].to_ascii_lowercase();
    assert!(mixed_case(&hex));
    assert!(!case_ok(&hex, &hash(&hex)));
}

#[test]
fn all_lower_or_all_upper_carries_no_checksum() {
    let low = lower(&digits(EXAMPLES[0]));
    assert!(!mixed_case(&low) && case_ok(&low, &hash(&low)));
    let mut up = low;
    up.iter_mut().for_each(|c| *c = c.to_ascii_uppercase());
    assert!(!mixed_case(&up) && case_ok(&up, &hash(&up)));
}

#[test]
fn a_paste_is_one_whole_address_or_nothing() {
    assert!(pasted("  0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed\n").is_ok());
    assert!(pasted("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed").is_ok());
    let hash64 = "0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b";
    for bad in [
        hash64,
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAe",
        "Send 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeg",
        "",
    ] {
        assert_eq!(pasted(bad), Err(NOT_AN_ADDRESS), "{bad:?} was taken");
    }
}
