// NONOS Operating System (AGPL-3.0-or-later)
//! A spend a lander lands, or refuses, as the wallet reads it.

use shield_wire::field;

use crate::pool::{look, Spend};
use crate::service_reply::Followed;
use crate::wallet_reply::{landed, landed_news, last, taken};

use super::common::*;

#[test]
fn a_spend_a_lander_lands_is_read_landed_by_the_wallet() {
    let mut s = Spend { lands_at: Some(42), ..spend() };
    let mut seen_landed = None;
    for now in 0..60u32 {
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
        assert_eq!(field(&body, "state"), Some("done"), "the envelope says the job ended");
        let r = last(&body).expect("the follow says where the spend is");
        assert_eq!(r.said, state, "minute {now}: the wallet reads the follow's own state");
        assert_eq!(r.minutes, Some(minutes));
        assert_eq!(r.self_settle, offered);
        if landed(r.said) {
            seen_landed.get_or_insert((now, String::from(r.tx)));
        }
    }
    assert_eq!(seen_landed, Some((42, String::from("0xlander"))));
}

#[test]
fn a_publication_a_lander_refused_is_not_handed_to_a_lander() {
    assert!(taken(&published("1", None)));
    assert!(!taken(&published("1", Some("the lander is full"))));
    assert!(!taken(&published("0", Some("no lander answered"))));
}

#[test]
fn a_landing_names_its_transaction_only_when_there_is_one() {
    assert_eq!(landed_news("The payment", "0xab"), "The payment landed (0xab).");
    assert_eq!(landed_news("The payment", ""), "The payment landed.");
}
