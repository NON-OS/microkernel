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
 * examples, keccak computed here as the crypto service computes it. The
 * spelling itself is marketplace_abi's, tested letter by letter there.
 */

use crate::address_text::{case_ok, mixed_case, pasted};
use nonos_marketplace_abi::Keccak;
use sha3::{Digest, Keccak256};

struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
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
        assert!(case_ok(&K, &digits(text)), "{text}");
    }
}

#[test]
fn one_wrong_capital_is_caught() {
    for text in &EXAMPLES[..4] {
        let hex = digits(text);
        for at in (0..40).filter(|&i| hex[i].is_ascii_alphabetic()) {
            let mut one = hex;
            one[at] ^= 0x20;
            assert!(mixed_case(&one) && !case_ok(&K, &one), "{text} at {at}");
        }
    }
}

#[test]
fn all_lower_or_all_upper_carries_no_checksum() {
    let low = digits(EXAMPLES[0]).map(|c| c.to_ascii_lowercase());
    assert!(!mixed_case(&low) && case_ok(&K, &low));
    let up = low.map(|c| c.to_ascii_uppercase());
    assert!(!mixed_case(&up) && case_ok(&K, &up));
}
