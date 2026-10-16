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

//! A purchase moves on each job result: a checked offer to ORDER_NEW, an
//! order whose words are the confirmed terms to funding, and nothing else.

use nonos_payment_proto::{EAGAIN, EBUSY, EINVAL, ENOENT, ETIME};

use super::deposit_fixture::{made_at, ordered, salted};
use super::fixture::{buy, offer, NOW, REFUND_TO};
use crate::order::purchase::{after_offer, After};

#[test]
fn an_offer_goes_on_to_its_order_and_a_running_job_is_waited_on() {
    let b = buy();
    let want = b.order_new(&offer(), REFUND_TO, NOW);
    assert_eq!(
        after_offer(&b, Ok(&offer().encode()), REFUND_TO, NOW),
        After::Order(Box::new(want))
    );
    assert_eq!(after_offer(&b, Err(EAGAIN), REFUND_TO, NOW), After::Wait);
    assert!(matches!(after_offer(&b, Ok(&[0; 95]), REFUND_TO, NOW), After::Failed(_)));
}

#[test]
fn an_order_is_funded_only_on_the_terms_confirmed() {
    let n = buy().order_new(&offer(), REFUND_TO, NOW);
    let m = made_at(&n);
    assert_eq!(ordered(&n, Ok(&m.encode())), After::Fund(Box::new(m)));
    assert_eq!(ordered(&n, Err(EAGAIN)), After::Wait);
    for w in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
        let mut bad = m;
        bad.terms[w][31] ^= 1;
        let said = ordered(&n, Ok(&bad.encode()));
        assert!(matches!(said, After::Failed(s) if s.contains("do not fund")), "word {w}");
    }
    let other = salted(&n, 1);
    assert_ne!((other.terms[12], other.terms[13]), (m.terms[12], m.terms[13]));
    let theirs = After::Fund(Box::new(other));
    assert_eq!(ordered(&n, Ok(&other.encode())), theirs, "refundCommit and salt: the meter's");
    let mut far = n;
    far.chain_id = 1;
    assert!(matches!(ordered(&far, Ok(&m.encode())), After::Failed(_)), "no NOX on chain 1");
    assert!(matches!(ordered(&n, Ok(&m.encode()[1..])), After::Failed(_)));
}

#[test]
fn each_refusal_is_said_for_its_stage() {
    let b = buy();
    let n = b.order_new(&offer(), REFUND_TO, NOW);
    let said = |a: After| match a {
        After::Failed(s) => s,
        _ => panic!("not a refusal"),
    };
    let offer_time = said(after_offer(&b, Err(ETIME), REFUND_TO, NOW));
    let order_time = said(ordered(&n, Err(ETIME)));
    assert_ne!(offer_time, order_time, "ETIME means another thing at each stage");
    assert!(order_time.contains("deadline"));
    assert!(said(ordered(&n, Err(ENOENT))).contains("offer"));
    assert!(said(ordered(&n, Err(EBUSY))).contains("paused"));
    assert!(said(after_offer(&b, Err(EINVAL), REFUND_TO, NOW)).contains("too large"));
}
