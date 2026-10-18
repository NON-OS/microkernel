// NONOS Operating System (AGPL-3.0-or-later)
//! Two spends in flight, and the owner settles the earlier one.

use crate::pool::{Spend, SELF_SETTLE_AFTER};
use crate::wallet_reply::{kept_all, landed, last, said};

use super::common::*;

/// Two spends in flight: the earlier one kept by the service under a number,
/// the last one beside it. The lander lands the last; nothing lands the
/// earlier, so its owner is offered it at thirty minutes, settles it, and it
/// is followed to landed under its own number while the last stays landed.
#[test]
fn two_spends_in_flight_and_the_owner_settles_the_earlier_one() {
    let mut earlier_spend = spend();
    let mut last_spend =
        Spend { published_at: 5, last_published: 5, lands_at: Some(20), ..spend() };
    // The wallet proved both, in order, and knows neither number yet.
    let mut names: Vec<Option<String>> = vec![None];
    let mut settled_earlier = false;
    let mut earlier_landed_at = None;
    let mut last_landed_at = None;
    for now in 5..50u32 {
        if settled_earlier {
            earlier_spend.settling = true;
        }
        let kept = at(&mut earlier_spend, now);
        let latest = at(&mut last_spend, now);
        let (es, ls) = (kept.state, latest.state);
        let body = follow_result(now, &latest, &[("4", kept)]);
        // The wallet names its unnamed spend from the numbers the service reports.
        let reported = kept_all(&body).expect("the kept spends were read");
        let ids: Vec<&str> = reported.iter().map(|k| k.id).collect();
        assert!(name(&mut names, &ids), "minute {now}: no spend the wallet did not prove");
        assert_eq!(names, [Some(String::from("4"))], "the earlier spend is named by its number");
        let k = reported.iter().find(|k| k.id == "4").expect("the earlier spend");
        assert_eq!(k.state, es, "minute {now}: the earlier spend reads its own state");
        if k.self_settle && !settled_earlier {
            assert_eq!(now, SELF_SETTLE_AFTER, "offered at thirty minutes");
            // The owner settles it from their account; it lands three minutes on.
            settled_earlier = true;
            earlier_spend.owner_lands_at = Some(now + 3);
        }
        if landed(k.state) {
            earlier_landed_at.get_or_insert(now);
            assert_eq!(said(k.state, settled_earlier), "landed from your account");
        }
        let r = last(&body).expect("the follow says where the spend is");
        assert_eq!(r.said, ls, "minute {now}: the last spend reads its own state beside it");
        if landed(r.said) {
            last_landed_at.get_or_insert(now);
        }
    }
    assert_eq!(last_landed_at, Some(20), "the lander landed the last spend");
    assert_eq!(
        earlier_landed_at,
        Some(SELF_SETTLE_AFTER + 3),
        "the owner's settlement landed the earlier"
    );
}

/// Name the wallet's unnamed spends from the numbers the service reports,
/// as the wallet does; false if a number names a spend it did not prove.
fn name(names: &mut [Option<String>], ids: &[&str]) -> bool {
    let known: Vec<&str> = names.iter().filter_map(|n| n.as_deref()).collect();
    let unnamed = names.iter().filter(|n| n.is_none()).count();
    let (older, newer) = crate::kept_names::split(&known, unnamed, ids);
    let mut newer = newer.into_iter().map(String::from).collect::<Vec<_>>().into_iter();
    for n in names.iter_mut().filter(|n| n.is_none()) {
        *n = newer.next();
    }
    older.is_empty()
}
