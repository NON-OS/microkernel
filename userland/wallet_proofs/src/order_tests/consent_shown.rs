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

//! Nothing of a purchase shows until every market answer is in and the
//! terms read for the names asked (B8): no half screen. Focus lost over
//! the consent puts it back there, shown nothing until read again.

use nonos_market_proto::Commerce;
use nonos_marketplace_abi::SettleMode;

use super::deposit_fixture::K;
use super::gate_fixture::names;
use super::ids_fixture::commerce_for;
use super::market_answers::{app_body, commerce_body};
use super::market_drive::{escrow_market, Market};
use crate::order::buy_gate::buyable;
use crate::order::fees_kind::Fees;
use crate::order::pending_step::Step;
use crate::order::reading_step::{Read, Reading};
use crate::order::terms::answer::answered;
use crate::order::terms::arming::Guard;
use crate::order::terms::shown::{opens, unread};

/// Each answer in turn; whether the view opened after each.
fn opened(m: &Market) -> Vec<bool> {
    let u = unread();
    let (mut at, mut read, mut record) = (u.reading, u.read, u.record);
    let mut out = Vec::new();
    for answer in m {
        assert!(!opens(at.as_ref(), &read, record.as_ref()), "shown before the answer");
        let Some(now) = at.take() else { break };
        match answered(&K, &names(), now, answer).unwrap() {
            Read::More(next) => at = Some(next),
            Read::Done(Ok(t)) => (read, record) = (Ok(t.named), Some(t.record)),
            Read::Done(Err(why)) => read = Err(why),
        }
        out.push(opens(at.as_ref(), &read, record.as_ref()));
    }
    out
}

#[test]
fn the_view_opens_only_once_every_read_is_done_and_ok() {
    assert_eq!(opened(&escrow_market(1)), [false, false, false, true]);
    let untrusted = Commerce { commerce_verdict: 1, ..commerce_for(&names(), SettleMode::Escrow) };
    let mut m = escrow_market(1);
    m[0] = commerce_body(&untrusted);
    assert_eq!(opened(&m), [false, false, false], "refused at INSTALL_READY: never shown");
    let mut m = escrow_market(1);
    m[3] = app_body(b"linux.other", b"Quiet Notes", &[0x2B; 32]);
    assert_eq!(opened(&m), [false, false, false, false], "another listing: never shown");
}

#[test]
fn a_focus_loss_puts_the_purchase_back_to_reading_shown_nothing() {
    let u = unread();
    assert!(!opens(u.reading.as_ref(), &u.read, u.record.as_ref()), "nothing shown");
    assert!(matches!(u.reading, Some(Reading::Commerce)), "the first call due again");
    assert!(matches!(u.fees, Fees::Unasked), "the Shield asked again");
    assert_eq!(u.guard, Guard::default(), "Buy unarmed, its second not begun");
    assert!(buyable(&Step::Consent, &u.read, &Fees::Quoted("0.01".into())).is_none());
}
