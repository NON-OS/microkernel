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

//! The store hands a sold listing to the wallet by name (B8, plan/27): an
//! escrow listing whose price is trusted goes to the wallet; the meter's
//! answer is said plainly, EBUSY as a purchase already waiting.

use nonos_payment_proto::{EAGAIN, EBUSY, EPERM};

use super::fixture::listing;
use crate::store::asked::Asked;
use crate::store::price::{Price, ESCROW, FREE, VOUCHER};
use crate::store::purchase_said::{by_wallet, handed};

fn priced(mode: u8, trusted: bool) -> crate::store::listing::Listing {
    let mut l = listing(b"linux.qwen-small", b"Qwen");
    l.known.price = Some(Price { mode, unit: b"install", unit_price: 1, trusted });
    l
}

#[test]
fn only_a_trusted_escrow_listing_is_bought_in_the_wallet() {
    assert!(by_wallet(&priced(ESCROW, true)));
    assert!(!by_wallet(&priced(ESCROW, false)), "a price its payee has not confirmed");
    assert!(!by_wallet(&priced(VOUCHER, true)), "vouchers pay as they install");
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
