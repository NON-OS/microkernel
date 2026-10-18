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

//! ORDER_NEW's reply with one byte of its deposit or its id wrong, or on a
//! chain the config does not pin: refused in words, the purchase never at
//! Fund, so no Shield withdraw is set up.

use super::deposit_fixture::{made_at, step_of, AT, K};
use super::fixture::{buy, offer, NOW, REFUND_PAIR};
use crate::order::deposit_check::{
    pinned, Pinned, DEPOSIT_WRONG, ID_WRONG, REFUND_WRONG, UNPINNED,
};
use crate::order::fees_kind::Fees;
use crate::order::fund_ask::fund_ask;
use crate::order::purchase::{after_order, After};
use crate::order::read::Named;

/// The withdraw set up after the reply `b`, if any.
fn withdraw(b: &[u8], at: Option<Pinned>) -> (After, bool) {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let a = after_order(&K, &n, at, Ok(b));
    let quoted = Fees::Quoted("0.02".into());
    let ask = fund_ask(&step_of(a.clone()), &Ok(Named::Escrow(buy())), &quoted);
    (a, ask.is_some())
}

#[test]
fn a_deposit_one_byte_off_is_refused_and_nothing_is_withdrawn() {
    let m = made_at(&buy().order_new(&offer(), REFUND_PAIR, NOW));
    let (a, funded) = withdraw(&m.encode(), Some(AT));
    assert_eq!((a, funded), (After::Fund(Box::new(m)), true), "the reply as derived funds");
    for i in 0..20 {
        let mut bad = m;
        bad.deposit[i] ^= 0x01;
        assert_eq!(withdraw(&bad.encode(), Some(AT)), (After::Failed(DEPOSIT_WRONG), false));
    }
}

#[test]
fn an_order_id_one_byte_off_is_refused_and_nothing_is_withdrawn() {
    let m = made_at(&buy().order_new(&offer(), REFUND_PAIR, NOW));
    for i in 0..32 {
        let mut bad = m;
        bad.order_id[i] ^= 0x80;
        assert_eq!(withdraw(&bad.encode(), Some(AT)), (After::Failed(ID_WRONG), false), "{i}");
    }
}

#[test]
fn no_pinned_forwarder_hash_or_escrow_means_no_fund() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let m = made_at(&n);
    assert_eq!(withdraw(&m.encode(), None), (After::Failed(UNPINNED), false));
    let real = pinned(n.chain_id);
    assert_eq!(withdraw(&m.encode(), real), (After::Failed(UNPINNED), false), "config: None");
    for s in [DEPOSIT_WRONG, ID_WRONG, REFUND_WRONG, UNPINNED] {
        assert!(s.ends_with("do not fund it.") && s.len() < 80, "{s}");
    }
}
