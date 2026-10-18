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

//! On a link: verified beacons feed the signal and show the AP is there, so a
//! quiet link can still roam; refused ones are counted and do neither.

use crate::pmf_sim::AP_RSNE_MFP;
use crate::privacy::msg3_tests::beacons_refused;
use crate::roam::two_aps::{run_until, sent, two_aps_with};
use crate::soft::ap::SimAp;
use crate::soft::rig::Station;

/// Two protected APs, the first quiet: it sends no data, only beacons.
fn quiet(protect: bool, bare: bool, rssi: [i8; 2]) -> Station {
    let first = |a: &mut SimAp| {
        (a.rsne, a.chatter) = (AP_RSNE_MFP.to_vec(), false);
        (a.quirks.beacons.protect, a.quirks.beacons.bare) = (protect, bare);
    };
    let st = two_aps_with(first, |b| b.rsne = AP_RSNE_MFP.to_vec(), rssi);
    assert!(st.session().unwrap().pmf);
    st
}

#[test]
fn a_quiet_link_heard_weak_in_verified_beacons_looks() {
    let mut st = quiet(true, false, [-85, -40]);
    assert!(run_until(&mut st, |s| !sent(s, 0x48).is_empty()), "it looked");
    assert_eq!(beacons_refused(&mut st), 0);
}

#[test]
fn without_beacon_protection_a_quiet_link_never_looks() {
    let mut st = quiet(false, false, [-85, -40]);
    assert!(!run_until(&mut st, |s| !sent(s, 0x48).is_empty()), "never away");
    assert!(st.session().is_some(), "its bare beacons keep it");
}

#[test]
fn verified_beacons_keep_a_quiet_link() {
    let mut st = quiet(true, false, [-40, -85]);
    assert!(!run_until(&mut st, |s| s.session().is_none()), "never lost");
}

#[test]
fn beacons_that_fail_are_counted_and_keep_nothing() {
    let mut st = quiet(true, true, [-85, -40]);
    assert!(run_until(&mut st, |s| s.session().is_none()), "the link is lost");
    assert!(sent(&st, 0x48).is_empty(), "and the bare beacons never made it look");
    assert!(beacons_refused(&mut st) >= 50, "each one counted");
}
