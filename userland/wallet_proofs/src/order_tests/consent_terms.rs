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

//! Every line of the escrow consent traced to the market's record (the
//! the maintainers' consent ruling): what is bought by the market's title and
//! release, the payee in full and checksummed, the price and the Shield's
//! fee in NOX, the deadline the order carries, bond and package on the
//! detail line.

use super::deposit_fixture::K;
use super::fixture::{offer, NOW, REFUND_PAIR};
use super::gate_fixture::names;
use super::ids_fixture::version;
use super::record_fixture::{read, recorded, BOND, LEN, PAYEE, PAYEE_TEXT, PRICE};
use crate::order::fees_kind::Fees;
use crate::order::terms::lines::lines;
use crate::order::terms::words::DELIVERY_DAYS;

pub const WALLET: &str = "NONOS Wallet";

#[test]
fn each_consent_line_is_the_markets_record() {
    let n = names();
    let (b, r) = read(&n, &recorded(&n, b"Quiet Notes"));
    assert_eq!((b.payee, b.amount, b.bond_id, b.content_len), (PAYEE, PRICE, BOND, LEN));
    let l = lines(&K, WALLET, &n, &r, &b, &Fees::Quoted("0.0125".into()));
    let release = String::from_utf8(version(&n)).unwrap();
    let want = [
        ("buying", format!("Quiet Notes, release {release}")),
        ("pay to", String::from(PAYEE_TEXT)),
        ("price", String::from("7.250000000000000003 NOX, once")),
        ("fee", String::from("0.0125 NOX, the Shield's, to pay it privately")),
    ];
    assert_eq!(l.rows, want);
    assert_eq!(l.action, Some("Buy"));
    let bond = "b0".repeat(32);
    let detail = format!(
        "Detail: bond 0x{bond}, checked by the market; package 123457 bytes, checked chunk by \
         chunk; the store called it {}, {release}.",
        String::from_utf8(n.listing.clone()).unwrap()
    );
    assert_eq!(l.detail, detail, "bond and size moved off the main rows");
    assert!(l.rows.iter().all(|(k, _)| !["bond", "package"].contains(k)));
}

#[test]
fn the_consent_says_the_deadline_the_order_will_carry() {
    let n = names();
    let (b, r) = read(&n, &recorded(&n, b"Quiet Notes"));
    let order = b.order_new(&offer(), REFUND_PAIR, NOW);
    assert_eq!(order.deadline - NOW, DELIVERY_DAYS * 86_400, "the order's own deadline");
    let l = lines(&K, WALLET, &n, &r, &b, &Fees::Quoted("0".into()));
    let said = format!(
        "Held in escrow until delivery; if it is not delivered within {} days it comes back \
         to a fresh address of this wallet. A later pause of the escrow can delay the \
         refund's gas, never the refund.",
        (order.deadline - NOW) / 86_400
    );
    assert_eq!((l.escrow, DELIVERY_DAYS), (said, 3));
}
