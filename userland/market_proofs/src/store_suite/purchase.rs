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

//! The store hands a sold listing to the wallet by name (B8, the plan): an
//! escrow listing whose price is trusted goes to the wallet; the meter's
//! answer is said plainly, EBUSY as a purchase already waiting.

use nonos_payment_proto::{EAGAIN, EBUSY, EPERM};

use super::fixture::listing;
use crate::store::asked::Asked;
use crate::store::order_line::order_line;
use crate::store::price::{Price, ESCROW, FREE, VOUCHER};
use crate::store::purchase_said::{by_wallet, handed, ordered};

fn priced(mode: u8, trusted: bool) -> crate::store::listing::Listing {
    let mut l = listing(b"linux.qwen-small", b"Qwen");
    l.known.price = Some(Price { mode, unit: b"install", unit_price: 1, payee: [0; 20], trusted });
    l
}

#[test]
fn only_a_trusted_escrow_listing_is_bought_in_the_wallet() {
    assert!(by_wallet(&priced(ESCROW, true)));
    assert!(!by_wallet(&priced(ESCROW, false)), "a price its payee has not confirmed");
    assert!(by_wallet(&priced(VOUCHER, true)), "a voucher listing not installed opens its channel");
    let mut installed = priced(VOUCHER, true);
    installed.progress = crate::store::progress::Progress::Installed;
    assert!(!by_wallet(&installed), "an installed voucher listing pays as it runs");
    assert!(!by_wallet(&priced(VOUCHER, false)), "a voucher price its payee has not confirmed");
    assert!(
        ordered(&priced(ESCROW, true)) && !ordered(&priced(VOUCHER, true)),
        "only escrow is an order"
    );
    assert!(!by_wallet(&priced(FREE, true)));
    assert!(!by_wallet(&listing(b"linux.x", b"X")), "no price read yet");
}

#[test]
fn the_meters_answer_is_said_plainly() {
    assert!(handed(Ok(())) == Asked::InWallet);
    assert!(handed(Err(EBUSY)) == Asked::WalletWaiting);
    assert!(handed(Err(EAGAIN)) == Asked::NoMeter && handed(Err(EPERM)) == Asked::NoMeter);
    assert_eq!(Asked::InWallet.label(), b"confirm the purchase in your wallet");
    assert!(Asked::WalletWaiting.label().starts_with(b"another purchase waits in your wallet"));
}

#[test]
fn the_order_reads_by_the_escrows_state_and_plain() {
    assert_eq!(Asked::Order(1, false).label(), b"bought: waiting for the seller to deliver");
    assert_eq!(order_line(2, false), b"delivered: your wallet checks it");
    assert_eq!(order_line(2, true), b"delivered and checked: ready to install");
    let lines: Vec<&[u8]> = (0..6).map(|s| order_line(s, false)).collect();
    for (i, a) in lines.iter().enumerate() {
        assert!(lines[i + 1..].iter().all(|b| a != b), "state {i} reads as its own");
    }
    assert_eq!(order_line(9, true), b"an order state this store does not know");
}
