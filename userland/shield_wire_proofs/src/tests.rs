// NONOS Operating System (AGPL-3.0-or-later)
//! The service builds each reply with its own builders, wraps a job's in
//! the RESULT envelope, frames it; the wallet unframes it and reads it with
//! its own readers. A mocked pool and lander drive the follow minute by
//! minute, and every minute the wallet must read what the service said.

use shield_wire::{decode_reply, encode_reply, field, Values, STATUS_OK};

use crate::pool::{look, Spend, SELF_SETTLE_AFTER};
use crate::service_reply::{done, earlier, entry, follow, Followed};
use crate::wallet::state::shield_log::{Kind, Stage};
use crate::wallet_reply::{entries, kept_all, landed, landed_news, last, said, taken, SETTLING};

/// A reply as the wallet receives it, framed and unframed.
fn carried(seq: u32, v: &Values) -> String {
    let bytes = encode_reply(seq, STATUS_OK, v.text());
    let r = decode_reply(&bytes).expect("the frame reads back");
    assert_eq!(r.seq, seq);
    String::from(r.body)
}

/// The RESULT a finished follow job answers, as the wallet reads it.
fn follow_result(seq: u32, f: &Followed<'_>, kept: &[(&str, Followed<'_>)]) -> String {
    let mut job = Values::new();
    for (id, k) in kept {
        earlier(&mut job, id, k);
    }
    follow(&mut job, f);
    carried(seq, &done("follow", job.text()))
}

fn spend() -> Spend {
    Spend {
        published_at: 0,
        last_published: 0,
        times: 1,
        lands_at: None,
        owner_lands_at: None,
        settling: false,
    }
}

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

#[test]
fn every_value_a_follow_writes_reaches_the_wallet_under_its_name() {
    let f = Followed {
        state: "landed",
        minutes: 12,
        times: 2,
        self_settle: false,
        tx: Some("0xbb"),
        link: Some("https://sepolia.etherscan.io/tx/0xbb"),
        refusal: Some("busy"),
    };
    let mut job = Values::new();
    follow(&mut job, &f);
    let body = carried(2, &done("follow", job.text()));
    for line in job.text().lines() {
        let (k, v) = line.split_once('=').expect("name=value");
        let read_as = match k {
            "state" | "job" => format!("job_{k}"),
            _ => String::from(k),
        };
        assert_eq!(field(&body, &read_as), Some(v), "{k} is lost under the envelope");
    }
}

#[test]
fn the_history_a_state_reply_carries_is_the_wallet_s_history() {
    let mut v = Values::new();
    v.put("unlocked", "1").put("history", "1");
    for e in [
        entry("deposit", "ETH", "0.1", "done", "0x01", 10),
        entry("sent", "NOX", "5", "on its way", "", 20),
        entry("withdrawn", "ETH", "0.02", "settling", "0x02", 30),
        entry("received", "NOX", "1.5", "done", "", 40),
        entry("sent", "ETH", "0.3", "taken back", "", 50),
        entry("mystery", "ETH", "1", "done", "", 60),
    ] {
        v.put("entry", &e);
    }
    let read = entries(&carried(3, &v)).expect("a history");
    let got: Vec<(Kind, u8, &str, Stage, Option<&str>)> = read
        .iter()
        .map(|e| (e.kind, e.asset, e.amount.as_str(), e.stage, e.tx.as_deref()))
        .collect();
    assert_eq!(
        got,
        [
            (Kind::Deposit, 0, "0.1", Stage::Settled, Some("0x01")),
            (Kind::Send, 1, "5", Stage::Sent, None),
            (Kind::Withdraw, 0, "0.02", Stage::Settling, Some("0x02")),
            (Kind::Received, 1, "1.5", Stage::Settled, None),
            (Kind::Send, 0, "0.3", Stage::Failed, None),
        ],
        "a kind this build does not know is left out, never misread"
    );
    let mut older = Values::new();
    older.put("unlocked", "1");
    assert!(
        entries(&carried(4, &older)).is_none(),
        "a service without a history keeps the shown one"
    );
}

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
        let (es, em, eo, etx) = look(&mut earlier_spend, now);
        let (ls, lm, lo, ltx) = look(&mut last_spend, now);
        let kept = Followed {
            state: es,
            minutes: em,
            times: earlier_spend.times,
            self_settle: eo,
            tx: etx,
            link: None,
            refusal: None,
        };
        let latest = Followed {
            state: ls,
            minutes: lm,
            times: last_spend.times,
            self_settle: lo,
            tx: ltx,
            link: None,
            refusal: None,
        };
        let body = follow_result(now, &latest, &[("4", kept)]);
        // The wallet names its unnamed spend from the numbers the service reports.
        let reported = kept_all(&body).expect("the kept spends were read");
        let ids: Vec<&str> = reported.iter().map(|k| k.id).collect();
        let known: Vec<&str> = names.iter().filter_map(|n| n.as_deref()).collect();
        let unnamed = names.iter().filter(|n| n.is_none()).count();
        let (older, newer) = crate::kept_names::split(&known, unnamed, &ids);
        assert!(older.is_empty(), "minute {now}: no spend the wallet did not prove");
        let mut newer = newer.into_iter();
        for n in names.iter_mut().filter(|n| n.is_none()) {
            *n = newer.next().map(String::from);
        }
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

/// A proved spend's publication, as the send job answers it.
fn published(record: &str, refusal: Option<&str>) -> String {
    let mut job = Values::new();
    job.put("published", record);
    if let Some(why) = refusal {
        job.put("lander_refusal", why);
    }
    carried(9, &done("send", job.text()))
}

#[test]
fn a_publication_a_lander_refused_is_not_handed_to_a_lander() {
    assert!(taken(&published("1", None)));
    assert!(!taken(&published("1", Some("the lander is full"))));
    assert!(!taken(&published("0", Some("no lander answered"))));
}

#[test]
fn a_follow_that_names_no_state_is_not_read_as_waiting() {
    let body = carried(10, &done("follow", Values::new().text()));
    assert!(last(&body).is_none());
}

#[test]
fn a_landing_names_its_transaction_only_when_there_is_one() {
    assert_eq!(landed_news("The payment", "0xab"), "The payment landed (0xab).");
    assert_eq!(landed_news("The payment", ""), "The payment landed.");
}

#[test]
fn an_entry_in_a_coin_the_wallet_does_not_know_is_not_listed_as_eth() {
    let mut v = Values::new();
    v.put("history", "1");
    v.put("entry", &entry("sent", "NOX", "5", "done", "0x1", 1));
    v.put("entry", &entry("sent", "DAI", "5", "done", "0x2", 2));
    v.put("entry", &entry("deposit", "ETH", "0.1", "on its way", "", 3));
    let body = carried(11, &v);
    let read = entries(&body).expect("the reply carries a history");
    let assets: Vec<u8> = read.iter().map(|e| e.asset).collect();
    assert_eq!(assets, [1, 0], "the DAI entry is skipped, never shown as ETH");
}

#[test]
fn a_history_the_store_would_not_give_leaves_the_list_shown() {
    let mut v = Values::new();
    v.put("history_why", "the store is busy");
    assert!(entries(&carried(12, &v)).is_none());
}
