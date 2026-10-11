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

//! A named purchase is read from the market alone: trusted commerce on the
//! asked chain, every gate but the package, a bond that passes, and for
//! escrow an escrowed package; anything less is refused in words.

use nonos_market_proto::{BondCode, Commerce, ReleaseEscrow};
use nonos_marketplace_abi::SettleMode;

use super::fixture::buy;
use super::read_fixture::{commerce, escrow, ready, CHAIN};
use crate::order::read::{read_named, Named};

#[test]
fn the_market_alone_makes_the_purchase() {
    let c = commerce(SettleMode::Escrow);
    let got = read_named(CHAIN, &c, &escrow(), &ready(&[1], BondCode::Active));
    assert_eq!(got, Ok(Named::Escrow(buy())), "the package gate alone may fail");
}

#[test]
fn anything_less_is_refused() {
    let (c, e, ok) = (commerce(SettleMode::Escrow), escrow(), ready(&[], BondCode::Active));
    let untrusted = Commerce { binding_verdict: 1, ..commerce(SettleMode::Escrow) };
    assert!(read_named(CHAIN, &untrusted, &e, &ok).is_err());
    assert!(read_named(1, &c, &e, &ok).is_err(), "another chain");
    for gate in [0, 2, 6, 10] {
        assert!(
            read_named(CHAIN, &c, &e, &ready(&[gate], BondCode::Active)).is_err(),
            "gate {gate}"
        );
    }
    assert!(read_named(CHAIN, &c, &e, &ready(&[], BondCode::Unread)).is_err(), "bond unread");
    let free = commerce(SettleMode::Free);
    assert_eq!(
        read_named(CHAIN, &free, &e, &ok),
        Err("This release is free: there is nothing to buy.")
    );
    let bare = ReleaseEscrow { content_len: 0, ..e.clone() };
    assert!(read_named(CHAIN, &c, &bare, &ok).is_err(), "no escrowed package");
    let mut per_call = commerce(SettleMode::Escrow);
    let mut b = nonos_marketplace_abi::commerce_from_bytes(&per_call.block).unwrap();
    b.unit = nonos_nox_config::UNIT_CALL;
    per_call.block = nonos_marketplace_abi::commerce_bytes(&b);
    let said = read_named(CHAIN, &per_call, &e, &ok);
    assert!(matches!(said, Err(s) if s.contains("by the install")), "escrow is per install");
    let garbled = Commerce { block: vec![1, 2, 3], ..commerce(SettleMode::Escrow) };
    assert!(read_named(CHAIN, &garbled, &e, &ok).is_err());
}
