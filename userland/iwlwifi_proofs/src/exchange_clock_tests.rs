// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The join exchange on a channel that is never quiet: a neighbour's beacon
//! arrives at every poll. The resend and the exchange's end are on the
//! clock, so the beacons neither hold back a resend nor use up the join.

use nonos_wifi_core::rsn::JoinPolicy;

use crate::gen3::join::exchange::limits::{EXCHANGE_MS, IDLE_TRIES};
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::run::{join, JoinEnd};
use crate::join_rig::*;

// A beacon from another BSS on the same channel, at every restock.
fn flood(r: &Rig) {
    let mut other = r.ap.borrow().beacon();
    other[10] ^= 1;
    other[16] ^= 1;
    r.model.s.borrow_mut().flood = Some(rx(&other, 0));
}

fn run(r: &Rig) -> (Result<(), JoinEnd>, u64) {
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    let mut out = (Ok(()), 0);
    with_fw(r, &mut st, |fw| {
        let start = fw.clock.delays_us / 1000;
        let mut p = Progress::default();
        let end = join(fw, &request(JoinPolicy::ANY), &beacon, 6, 3, 3, &mut p).map(|_| ());
        out = (end, fw.clock.delays_us / 1000 - start);
    });
    out
}

#[test]
fn a_lost_authentication_is_resent_on_a_busy_channel_and_the_join_completes() {
    let r = rig(Security::Wpa2, Switches { drop_first_auth: true, ..Switches::default() });
    flood(&r);
    let (end, _) = run(&r);
    assert_eq!(end, Ok(()), "the resent authentication was answered");
    let auths = r.sent().iter().filter(|s| s.frame[0] == 0xB0).count();
    assert_eq!(auths, 2, "sent, lost, resent once on the clock");
}

#[test]
fn a_silent_access_point_on_a_busy_channel_is_resent_to_then_given_up_on_the_clock() {
    let r = rig(Security::Wpa2, Switches { silent: true, ..Switches::default() });
    flood(&r);
    let (end, ms) = run(&r);
    assert_eq!(end, Err(JoinEnd::TimedOut));
    let auths = r.sent().iter().filter(|s| s.frame[0] == 0xB0).count() as u32;
    assert_eq!(auths, 1 + IDLE_TRIES, "every resend went out despite the beacons");
    assert!(ms < EXCHANGE_MS + 1000, "ended on the clock, not after it: {ms} ms");
}
