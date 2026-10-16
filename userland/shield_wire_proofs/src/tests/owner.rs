// NONOS Operating System (AGPL-3.0-or-later)
//! The owner offered a stuck spend at thirty minutes, and followed to landed.

use crate::pool::{look, SELF_SETTLE_AFTER};
use crate::service_reply::Followed;
use crate::wallet_reply::{landed, last, said, SETTLING};

use super::common::*;

#[test]
fn the_owner_is_offered_after_thirty_minutes_and_their_settlement_is_followed_to_landed() {
    let mut s = spend();
    let mut offered_at = None;
    for now in 0..SELF_SETTLE_AFTER + 1 {
        let (state, minutes, offered, tx) = look(&mut s, now);
        let f = Followed {
            state,
            minutes,
            times: s.times,
            self_settle: offered,
            tx,
            link: None,
            refusal: None,
        };
        let body = follow_result(now, &f, &[]);
        let r = last(&body).expect("the follow says where the spend is");
        if r.self_settle {
            offered_at.get_or_insert(now);
        }
    }
    assert_eq!(offered_at, Some(SELF_SETTLE_AFTER), "offered at thirty minutes, not before");
    s.settling = true;
    s.owner_lands_at = Some(SELF_SETTLE_AFTER + 3);
    for now in SELF_SETTLE_AFTER + 1..SELF_SETTLE_AFTER + 6 {
        let times = s.times;
        let (state, minutes, offered, tx) = look(&mut s, now);
        assert_eq!(s.times, times, "a spend its owner settles is never published again");
        let f =
            Followed { state, minutes, times, self_settle: offered, tx, link: None, refusal: None };
        let body = follow_result(now, &f, &[]);
        let r = last(&body).expect("the follow says where the spend is");
        assert!(!r.self_settle, "never offered twice");
        let text = said(r.said, true);
        if now < SELF_SETTLE_AFTER + 3 {
            assert_eq!(text, SETTLING);
        } else {
            assert_eq!(text, "landed from your account");
            assert!(landed(r.said));
        }
    }
}
