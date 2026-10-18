// NONOS Operating System (AGPL-3.0-or-later)
//! Every value a follow writes, under its name.

use shield_wire::{field, Values};

use crate::service_reply::{done, follow, Followed};
use crate::wallet_reply::last;

use super::common::*;

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
fn a_follow_that_names_no_state_is_not_read_as_waiting() {
    let body = carried(10, &done("follow", Values::new().text()));
    assert!(last(&body).is_none());
}
