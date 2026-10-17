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

//! A refund salt lost before Fund (a restart between ORDER_NEW and Fund:
//! the salt lives only in Step::Ordering's order, in memory) means a new
//! order. The wallet keeps no purchase across a restart; TAKE starts one
//! with nothing read, at consent, and only a new Buy, a new offer and a new
//! ORDER_NEW with a fresh salt lead to Fund. The old order's reply never
//! funds the new one.

use super::deposit_fixture::{made_at, ordered, step_of};
use super::fixture::{buy, offer, NOW, REFUND_PAIR};
use crate::order::buy_gate::buyable;
use crate::order::fees_kind::Fees;
use crate::order::fund_ask::fund_ask;
use crate::order::pending_step::Step;
use crate::order::purchase::After;
use crate::order::read::Named;
use crate::order::terms::shown::unread;

#[test]
fn a_purchase_taken_again_starts_at_consent_and_cannot_fund() {
    let u = unread();
    let quoted = Fees::Quoted("0.01".into());
    assert!(fund_ask(&Step::Consent, &u.read, &quoted).is_none());
    assert!(buyable(&Step::Consent, &u.read, &quoted).is_none(), "nothing read: no Buy yet");
    let read = Ok(Named::Escrow(buy()));
    assert!(fund_ask(&Step::Consent, &read, &quoted).is_none(), "Fund only from an order made");
}

#[test]
fn the_old_orders_reply_never_funds_an_order_with_a_new_salt() {
    let old = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let fresh = ([0x45; 20], [0x77; 32]);
    let new = buy().order_new(&offer(), fresh, NOW + 60);
    assert!(matches!(ordered(&old, Ok(&made_at(&old).encode())), After::Fund(_)));
    let a = ordered(&new, Ok(&made_at(&old).encode()));
    assert!(matches!(a, After::Failed(_)), "the old order is not the new one");
    let quoted = Fees::Quoted("0.01".into());
    assert!(fund_ask(&step_of(a), &Ok(Named::Escrow(buy())), &quoted).is_none());
    let same_terms = buy().order_new(&offer(), (REFUND_PAIR.0, fresh.1), NOW);
    let b = ordered(&same_terms, Ok(&made_at(&old).encode()));
    assert!(matches!(b, After::Failed(_)), "a salt drawn again is not the salt committed");
}
