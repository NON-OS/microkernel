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

//! Each job result moves a purchase on: an offer to ORDER_NEW, the terms confirmed to Fund.

use nonos_payment_proto::{EAGAIN, EBUSY, EINVAL, ENOENT, ETIME};

use super::deposit_fixture::{made_at, ordered, salted};
use super::fixture::{buy, offer, NOW, REFUND_PAIR, REFUND_SALT, REFUND_TO};
use crate::order::buy::Buy;
use crate::order::purchase::{after_offer, After};

fn offered(b: &Buy, r: Result<&[u8], i32>) -> After {
    after_offer(b, r, (REFUND_TO, || Some(REFUND_SALT)), NOW)
}

#[test]
fn an_offer_goes_on_to_its_order_and_a_running_job_is_waited_on() {
    let b = buy();
    let want = b.order_new(&offer(), REFUND_PAIR, NOW);
    assert_eq!(offered(&b, Ok(&offer().encode())), After::Order(Box::new(want)));
    assert_eq!(offered(&b, Err(EAGAIN)), After::Wait);
    assert!(matches!(offered(&b, Ok(&[0; 95])), After::Failed(_)));
}

#[test]
fn an_order_is_funded_only_on_the_terms_confirmed() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let m = made_at(&n);
    assert_eq!(ordered(&n, Ok(&m.encode())), After::Fund(Box::new(m)));
    assert_eq!(ordered(&n, Err(EAGAIN)), After::Wait);
    for w in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
        let mut bad = m;
        bad.terms[w][31] ^= 1;
        let said = ordered(&n, Ok(&bad.encode()));
        assert!(matches!(said, After::Failed(s) if s.contains("do not fund")), "word {w}");
    }
    let other = salted(&n, 1, None);
    assert_ne!(other.terms[13], m.terms[13]);
    assert_eq!(ordered(&n, Ok(&other.encode())), After::Fund(Box::new(other)), "salt: the meter's");
    let mut far = n;
    far.chain_id = 1;
    assert!(matches!(ordered(&far, Ok(&m.encode())), After::Failed(_)), "no NOX on chain 1");
    assert!(matches!(ordered(&n, Ok(&m.encode()[1..])), After::Failed(_)));
}

#[test]
fn each_refusal_is_said_for_its_stage() {
    let b = buy();
    let n = b.order_new(&offer(), REFUND_PAIR, NOW);
    let said = |a: After| match a {
        After::Failed(s) => s,
        _ => panic!("not a refusal"),
    };
    let offer_time = said(offered(&b, Err(ETIME)));
    let order_time = said(ordered(&n, Err(ETIME)));
    assert_ne!(offer_time, order_time, "ETIME means another thing at each stage");
    assert!(order_time.contains("deadline"));
    assert!(said(ordered(&n, Err(ENOENT))).contains("offer"));
    assert!(said(ordered(&n, Err(EBUSY))).contains("paused"));
    assert!(said(offered(&b, Err(EINVAL))).contains("too large"));
}
