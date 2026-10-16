// NONOS Operating System (AGPL-3.0-or-later)
//! The history a state reply carries, as the wallet lists it.

use shield_wire::Values;

use crate::service_reply::entry;
use crate::wallet::state::shield_log::{Kind, Stage};
use crate::wallet_reply::entries;

use super::common::*;

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
