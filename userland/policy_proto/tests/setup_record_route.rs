/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 5 keeps the default network beside everything version 4 kept; a
 * version 4 record an earlier build kept still reads, as the Nym mixnet, and
 * a network this build does not know is refused by name.
 */

use nonos_policy_proto::route::{known, ANYONE, DIRECT, NYM, ROUTE_LABELS};
use nonos_policy_proto::setup_record::{check_record, Answers, Host, Name, Record, Refused, Tier};
use nonos_policy_proto::setup_record::{ANSWERS_LEN, ANSWERS_V4_LEN, ANSWERS_V5_LEN};
use nonos_policy_proto::{enum_table, Field};
use nonos_policy_proto::wallpapers_kept::ALL;

fn record(route: u8) -> Record {
    let answers = Answers {
        keyboard_layout: 1,
        timezone: -3,
        wallpaper: 2,
        username: Name::new(b"ada").unwrap(),
        qwen_tier: Tier::new(b"qwen3-0.6b").unwrap(),
    };
    Record { answers, apps_off: 2, hostname: Host::new(b"ada-laptop").unwrap(), route, wallpapers_kept: ALL }
}

#[test]
fn every_network_round_trips_in_the_current_version() {
    for route in [NYM, ANYONE, DIRECT] {
        let r = record(route);
        let raw = r.encode();
        assert_eq!((raw.len(), &raw[..4]), (ANSWERS_LEN, &b"NSA6"[..]));
        assert_eq!(Record::decode(&raw), Some(r));
        assert_eq!(check_record(&raw).unwrap().route, route);
    }
}

#[test]
fn a_version_4_record_reads_as_the_mixnet_with_all_it_held() {
    let raw = record(DIRECT).encode_v4();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_V4_LEN, &b"NSA4"[..]));
    let back = check_record(&raw).unwrap();
    assert_eq!(back.route, NYM, "no route kept means the private default");
    assert_eq!(back.hostname.as_bytes(), b"ada-laptop");
    assert_eq!((back.apps_off, back.answers.qwen_tier.as_bytes()), (2, &b"qwen3-0.6b"[..]));
}

#[test]
fn a_network_this_build_does_not_know_is_refused_by_name() {
    let mut raw = record(NYM).encode();
    raw[ANSWERS_V5_LEN - 1] = ROUTE_LABELS.len() as u8;
    assert_eq!(check_record(&raw), Err(Refused::Route));
    assert_eq!(Refused::Route.name(), "network route");
}

#[test]
fn the_setting_names_each_network_in_route_order() {
    assert!(known(NYM) && known(ANYONE) && known(DIRECT) && !known(3));
    assert_eq!(enum_table(Field::NetworkRoute), Some(ROUTE_LABELS));
    assert_eq!(ROUTE_LABELS[NYM as usize], b"Nym mixnet");
    assert_eq!(ROUTE_LABELS[DIRECT as usize], b"Direct");
}
