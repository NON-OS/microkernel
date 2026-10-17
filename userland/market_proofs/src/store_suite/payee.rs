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
//! The payee a paid listing's detail says (B9): the commerce block's own,
//! decoded as the store and the wallet both decode it, in full and in the
//! wallet's EIP-55 spelling; none where nothing is sold.

use nonos_marketplace_abi::{commerce_bytes, commerce_from_bytes, Commerce, Keccak, SettleMode};
use nonos_nox_config::UNIT_INSTALL;

use crate::chain_hosts::Host;
use crate::store::payee::{payee_line, CHECKED};
use crate::store::price::Price;
use crate::wallet_address_text::checksummed;

/* From EIP-55: every capital is the checksum's. */
pub const EXAMPLES: [&str; 4] = [
    "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
    "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
    "0x52908400098527886E0F7030069857D2E4169EE7",
    "0x8617E340B3D01FA5F11F306F4090FD50E238070D",
];

pub fn bytes(text: &str) -> [u8; 20] {
    core::array::from_fn(|i| u8::from_str_radix(&text[2 + 2 * i..4 + 2 * i], 16).unwrap())
}

/// The Price the store makes of a served commerce block (market/price.rs).
pub fn served(payee: [u8; 20], settle_mode: SettleMode) -> Price {
    let c = Commerce {
        payee,
        bond_id: [0xb0; 32],
        unit: UNIT_INSTALL,
        unit_price: 4_000_000_000_000_000_000,
        min_charge: 1,
        settle_mode,
        proof_reqs: alloc::vec![],
        binding_sig: [0x51; 65],
    };
    let c = commerce_from_bytes(&commerce_bytes(&c)).expect("the block reads");
    let (mode, unit_price, payee) = (c.settle_mode as u8, c.unit_price, c.payee);
    Price { mode, unit: b"install", unit_price, payee, trusted: true }
}

#[test]
fn the_payee_line_is_the_commerce_blocks_payee_in_full_checksummed() {
    for text in EXAMPLES {
        let line = payee_line(&Host, &served(bytes(text), SettleMode::Escrow)).expect("paid");
        assert_eq!(line, [b"pays: ", text.as_bytes(), CHECKED].concat(), "{text}");
        let hex: [u8; 40] = text.as_bytes()[2..].try_into().unwrap();
        let wallet = checksummed(&hex, &Host.keccak256(&hex.to_ascii_lowercase()));
        assert_eq!(&line[8..48], &wallet[..], "the wallet's spelling of {text}");
    }
    assert!(payee_line(&Host, &served(bytes(EXAMPLES[0]), SettleMode::Voucher)).is_some());
}
