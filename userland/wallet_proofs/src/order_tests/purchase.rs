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

use super::fixture::{buy, made, offer, NOW, REFUND_TO};
use crate::order::purchase::{after_offer, after_order, After};

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
    let m = made(&n);
    assert_eq!(after_order(&n, Ok(&m.encode())), After::Fund(Box::new(m)));
    assert_eq!(after_order(&n, Err(EAGAIN)), After::Wait);
    for w in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
        let mut bad = m;
        bad.terms[w][31] ^= 1;
        let said = after_order(&n, Ok(&bad.encode()));
        assert!(matches!(said, After::Failed(s) if s.contains("do not fund")), "word {w}");
    }
    let mut other = m;
    (other.terms[12][0], other.terms[13][0]) = (1, 1);
    let theirs = After::Fund(Box::new(other));
    assert_eq!(after_order(&n, Ok(&other.encode())), theirs, "refundCommit and salt: the meter's");
    let mut far = n;
    far.chain_id = 1;
    assert!(matches!(after_order(&far, Ok(&m.encode())), After::Failed(_)), "no NOX on chain 1");
    assert!(matches!(after_order(&n, Ok(&m.encode()[1..])), After::Failed(_)));
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
    let order_time = said(after_order(&n, Err(ETIME)));
    assert_ne!(offer_time, order_time, "ETIME means another thing at each stage");
    assert!(order_time.contains("deadline"));
    assert!(said(after_order(&n, Err(ENOENT))).contains("offer"));
    assert!(said(after_order(&n, Err(EBUSY))).contains("paused"));
    assert!(said(after_offer(&b, Err(EINVAL), REFUND_TO, NOW)).contains("too large"));
}
