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
//! Where the store says no payee, and that the one it says is never cut.

use nonos_marketplace_abi::SettleMode;

use super::payee::{bytes, served, EXAMPLES};
use crate::chain_hosts::Host;
use crate::store::payee::{payee_line, CHECKED};
use crate::store::price::{Price, FREE};

#[test]
fn a_listing_with_no_commerce_block_shows_no_payee_line() {
    // What market/price.rs makes of an empty block: free, nothing to pay.
    let free = Price { mode: FREE, unit: b"unit", unit_price: 0, payee: [0; 20], trusted: true };
    assert_eq!(payee_line(&Host, &free), None);
    assert_eq!(payee_line(&Host, &served(bytes(EXAMPLES[0]), SettleMode::Free)), None);
    let mut unconfirmed = served(bytes(EXAMPLES[0]), SettleMode::Escrow);
    unconfirmed.trusted = false;
    assert_eq!(payee_line(&Host, &unconfirmed), None, "no price is shown, so no payee");
}

#[test]
fn the_payee_line_never_shortens() {
    let payees = [[0u8; 20], [0xff; 20], core::array::from_fn(|i| i as u8 * 13)];
    for payee in payees.into_iter().chain(EXAMPLES.map(bytes)) {
        let line = payee_line(&Host, &served(payee, SettleMode::Voucher)).expect("paid");
        assert_eq!(line.len(), b"pays: 0x".len() + 40 + CHECKED.len());
        let address = &line[6..48];
        assert!(address.starts_with(b"0x") && address[2..].iter().all(u8::is_ascii_hexdigit));
        let digits: alloc::string::String =
            payee.iter().map(|b| alloc::format!("{b:02x}")).collect();
        assert_eq!(address[2..].to_ascii_lowercase(), digits.as_bytes(), "every digit, in order");
        assert!(!line.windows(3).any(|w| w == "…".as_bytes() || w == b"..."));
    }
}
