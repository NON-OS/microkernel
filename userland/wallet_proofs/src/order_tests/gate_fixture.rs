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

//! Every step a purchase can stand at, every way its terms can read, every
//! state of its quote, and every channel kept: what the gates are tried on.

use nonos_payment_proto::{Names, OpenConfirm};

use super::fixture::{buy, made, offer, NOW, REFUND_PAIR, REFUND_TO};
use super::held::row;
use super::read_fixture::CHAIN;
use super::voucher::voucher;
use crate::order::fees_kind::Fees;
use crate::order::held::Held;
use crate::order::pending_step::Step;
use crate::order::read::Named;
use crate::order::reading_step::UNREAD;

pub const FRESH: [u8; 20] = [0x3C; 20];

/// Names the store might send, their bytes shaped like addresses.
pub fn names() -> Names {
    let listing = b"0x5555555555555555555555555555555555555555".to_vec();
    Names { chain_id: CHAIN, listing, release: [0x44; 20].to_vec() }
}

/// Every Step, Consent first and Fund fourth.
pub fn steps() -> Vec<Step> {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let s = voucher().open(voucher().unit_price, NOW + 1, REFUND_TO, NOW).unwrap();
    let confirm = OpenConfirm { start: s, open_tx: [9; 32] };
    vec![
        Step::Consent,
        Step::Offering(1),
        Step::Ordering(2, Box::new(n)),
        Step::Fund(Box::new(made(&n))),
        Step::Opening(3, Box::new(s)),
        Step::Confirming(Box::new(confirm), NOW),
        Step::Opened([1; 32]),
        Step::Kept([2; 32]),
        Step::Failed("refused"),
    ]
}

/// Terms not yet read, unreadable, escrow, voucher.
pub fn reads() -> Vec<Result<Named, &'static str>> {
    let reading = Err("Reading the listing's terms from the market.");
    vec![reading, Err(UNREAD), Ok(Named::Escrow(buy())), Ok(Named::Voucher(voucher()))]
}

/// Unasked, asking, quoted, refused.
pub fn fees() -> Vec<Fees> {
    let (quoted, refused) = (Fees::Quoted("0.01".into()), Fees::Refused("no".into()));
    vec![Fees::Unasked, Fees::Asking, quoted, refused]
}

pub fn helds() -> Vec<Held> {
    let open = row(voucher().listing_id, CHAIN, NOW + 1, false);
    vec![Held::Asking(0), Held::Asking(3), Held::Free, Held::Open(open), Held::Another]
}
