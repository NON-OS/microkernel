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

//! address_text against EIP-55's own examples, letter by letter: every
//! capital the EIP writes, and every lower-case letter, in the order it
//! writes them. The keccak is a host crate's, as the ids' tests pass one.

use nonos_marketplace_abi::{address_text, Keccak};
use sha3::{Digest, Keccak256};

struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

/* From EIP-55: all caps, all lower, then the normal ones. */
const EXAMPLES: [&str; 8] = [
    "0x52908400098527886E0F7030069857D2E4169EE7",
    "0x8617E340B3D01FA5F11F306F4090FD50E238070D",
    "0xde709f2102306220921060314715629080e2fb77",
    "0x27b1fdb04752bbc536007a920d24acb045561c26",
    "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
    "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
    "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
    "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
];

fn bytes(text: &str) -> [u8; 20] {
    core::array::from_fn(|i| u8::from_str_radix(&text[2 + 2 * i..4 + 2 * i], 16).unwrap())
}

#[test]
fn each_eip55_example_is_spelt_letter_by_letter_as_the_eip_writes_it() {
    for text in EXAMPLES {
        let spelt = address_text(&K, &bytes(text));
        assert_eq!(spelt.len(), text.len(), "{text}");
        for (at, (got, want)) in spelt.iter().zip(text.as_bytes()).enumerate() {
            assert_eq!(char::from(*got), char::from(*want), "letter {at} of {text}");
        }
    }
}
