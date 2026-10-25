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

//! The domain words against the EIP-712 specification's own example, so the
//! encoding is checked by a separator someone else computed.

mod keccak;

use keccak::keccak;
use nonos_nox_config::{domain_words_at, hex20, hex32, Contract, Domain};

#[test]
fn the_ether_mail_example_separator() {
    let mail = Domain {
        name: "Ether Mail",
        name_hash: keccak(b"Ether Mail"),
        version: "1",
        contract: Contract::Settlement,
    };
    let at = hex20("cccccccccccccccccccccccccccccccccccccccc");
    let want = hex32("f2cee375fa42b42143804025fc449deafd50cc031ca257e0b194a650a912090f");
    assert_eq!(keccak(&domain_words_at(&mail, 1, at)), want);
}

#[test]
fn chain_and_contract_land_right_aligned() {
    let d = nonos_nox_config::SETTLEMENT;
    let w = domain_words_at(&d, 0x0102_0304_0506_0708, [0xee; 20]);
    assert_eq!(w[96..120], [0u8; 24]);
    assert_eq!(w[120..128], [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(w[128..140], [0u8; 12]);
    assert_eq!(w[140..160], [0xee; 20]);
    assert_eq!(w[32..64], d.name_hash);
}
