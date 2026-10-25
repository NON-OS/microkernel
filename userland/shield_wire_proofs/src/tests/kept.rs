// NONOS Operating System (AGPL-3.0-or-later)
//! Spends kept before the last, each read by its number.

use shield_wire::Values;

use crate::service_reply::{done, follow, Followed};
use crate::wallet_reply::{kept_all, last};

use super::common::*;

#[test]
fn spends_kept_before_the_last_are_read_each_by_its_number() {
    let lands = Followed {
        state: "landed",
        minutes: 50,
        times: 3,
        self_settle: false,
        tx: Some("0xaa"),
        link: None,
        refusal: None,
    };
    let waits = Followed {
        state: "republished",
        minutes: 31,
        times: 2,
        self_settle: true,
        tx: None,
        link: None,
        refusal: None,
    };
    let latest = Followed {
        state: "waiting",
        minutes: 1,
        times: 1,
        self_settle: false,
        tx: None,
        link: None,
        refusal: None,
    };
    let body = follow_result(7, &latest, &[("0", lands), ("3", waits)]);
    let kept = kept_all(&body).expect("the kept spends were read");
    let read: Vec<(&str, &str, u32, bool, &str)> =
        kept.iter().map(|k| (k.id, k.state, k.minutes, k.self_settle, k.tx)).collect();
    assert_eq!(read, [("0", "landed", 50, false, "0xaa"), ("3", "republished", 31, true, "")]);
    assert_eq!(last(&body).map(|r| r.said), Some("waiting"), "the last spend is read beside them");
}

#[test]
fn a_failed_read_of_the_kept_spends_lets_none_of_them_go() {
    let mut job = Values::new();
    job.put("earlier_why", "the pool did not answer");
    follow(
        &mut job,
        &Followed {
            state: "waiting",
            minutes: 1,
            times: 1,
            self_settle: false,
            tx: None,
            link: None,
            refusal: None,
        },
    );
    let body = carried(1, &done("follow", job.text()));
    assert!(kept_all(&body).is_none());
}
