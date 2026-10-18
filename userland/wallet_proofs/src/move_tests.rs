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
//! Moving NOX at a fresh address into the Shield (v1.4.56): the review read
//! from the Shield's answer, and the one button each state of it shows.

use crate::shield::fresh_move::{reviewed, sent, Moving, Reviewed};
use crate::shield::fresh_move_act::{act, Act};

const BODY: &str = "id=7\napproval=1\npieces=3\nshielded=36\nmax_network_fee=0.002\n\
     remainder=500\nshort_eth=0";

fn moving(review: Option<Reviewed>, gas_paid_s: Option<u64>) -> Moving {
    Moving { address: [9; 20], amount: 37, review, gas_paid_s, ..Moving::default() }
}

#[test]
fn a_review_reads_whole_or_not_at_all() {
    let r = reviewed(BODY).unwrap();
    assert_eq!((r.id, r.approval, r.pieces, r.remainder, r.short_eth), (7, true, 3, 500, 0));
    assert_eq!(r.refusal, None);
    let refused = reviewed(&format!("{BODY}\nrefusal=no")).unwrap();
    assert_eq!(refused.refusal.as_deref(), Some("no"));
    assert_eq!(reviewed(&BODY.replace("id=7", "id=-1")), None, "a negative id");
    assert_eq!(reviewed(&BODY.replace("remainder=500", "remainder=1e3")), None);
    let past = format!("{}0", u128::MAX);
    assert_eq!(reviewed(&BODY.replace("short_eth=0", &format!("short_eth={past}"))), None);
    assert_eq!(reviewed("tx=0xab\ntx=0xcd"), None, "a confirm's answer is not a review");
    assert_eq!(reviewed(""), None);
}

#[test]
fn a_confirm_reads_each_tx_up_to_the_most() {
    let (txs, stopped) = sent("tx=0x1\ntx=0x2\nstopped=the node refused", 32);
    assert_eq!((txs.len(), stopped.as_deref()), (2, Some("the node refused")));
    let many = "tx=0x1\n".repeat(100);
    assert_eq!(sent(&many, 32).0.len(), 32, "bounded");
}

#[test]
fn each_state_shows_its_one_button_and_the_gas_is_held() {
    let r = reviewed(BODY).unwrap();
    assert_eq!(act(None, 0), Act::Move);
    assert_eq!(act(Some(&moving(None, None)), 0), Act::Move, "nothing reviewed yet");
    assert_eq!(act(Some(&moving(Some(r.clone()), None)), 0), Act::Confirm { approval: true });
    let deposits = Reviewed { approval: false, ..r.clone() };
    assert_eq!(act(Some(&moving(Some(deposits), None)), 0), Act::Confirm { approval: false });
    let refused = Reviewed { refusal: Some("no".into()), ..r.clone() };
    assert_eq!(act(Some(&moving(Some(refused), None)), 0), Act::Move, "review again");
    let short = Reviewed { short_eth: 5, ..r };
    assert_eq!(act(Some(&moving(Some(short.clone()), None)), 100), Act::PayGas);
    let paid = |at| moving(Some(short.clone()), Some(at));
    assert_eq!(act(Some(&paid(100)), 100 + 599), Act::GasWaiting, "a second press pays nothing");
    assert_eq!(act(Some(&paid(100)), 100 + 600), Act::PayGas, "ten minutes on");
    assert_eq!(act(Some(&paid(u64::MAX)), 0), Act::GasWaiting, "no wrap");
    assert_eq!(Act::Confirm { approval: true }.label(), "Approve exactly their sum");
}
