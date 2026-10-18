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

//! Which AP a look's findings move the station to: the strongest of the
//! same network and security, heard at least `HYSTERESIS_DB` stronger.

use super::beacons::{ap, beacon_of, heard, RSNE_PSK, RSNE_PSK_MFPR, RSNE_SAE};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::rsn::Selection;
use nonos_wifi_core::softmac::roam::{offered, pick, Heard, Want, HYSTERESIS_DB};

pub const HOME: &[u8] = b"home";

pub fn runs() -> Selection {
    offered(&beacon_of(ap(1), HOME, 1, &RSNE_PSK), JoinPolicy::PSK_ONLY).unwrap()
}

/// The pick for a PSK session on `HOME` running `runs`, nothing barred.
pub fn picks(current: ([u8; 6], i16), runs: &Selection, heard: &[Heard]) -> Option<usize> {
    let policy = nonos_wifi_core::softmac::roam::roam_policy(
        runs,
        nonos_wifi_core::rsn::Ciphers::CCMP_128_ONLY,
    );
    let want = Want { ssid: HOME, runs, group_mgmt: None, policy, barred: &[] };
    pick(current, &want, heard)
}

#[test]
fn the_strongest_ap_of_the_network_past_the_margin_is_chosen() {
    let found = [
        heard(2, HOME, -70, &RSNE_PSK),
        heard(3, HOME, -60, &RSNE_PSK),
        heard(4, HOME, -65, &RSNE_PSK),
    ];
    assert_eq!(picks((ap(1), -80), &runs(), &found), Some(1));
}

#[test]
fn the_margin_is_exactly_hysteresis_db() {
    let edge = -80 + HYSTERESIS_DB as i8;
    let at = [heard(2, HOME, edge, &RSNE_PSK)];
    assert_eq!(picks((ap(1), -80), &runs(), &at), Some(0));
    let short = [heard(2, HOME, edge - 1, &RSNE_PSK)];
    assert_eq!(picks((ap(1), -80), &runs(), &short), None);
}

#[test]
fn the_current_ap_another_network_or_other_security_is_never_chosen() {
    let cur = (ap(1), -90);
    assert_eq!(picks(cur, &runs(), &[heard(1, HOME, -40, &RSNE_PSK)]), None, "itself");
    assert_eq!(picks(cur, &runs(), &[heard(2, b"homer", -40, &RSNE_PSK)]), None, "a name");
    assert_eq!(picks(cur, &runs(), &[heard(2, HOME, -40, &RSNE_SAE)]), None, "SAE only");
    assert_eq!(picks(cur, &runs(), &[heard(2, HOME, -40, &RSNE_PSK_MFPR)]), None, "PMF");
    assert_eq!(picks(cur, &runs(), &[heard(2, HOME, -40, &[])]), None, "open");
}

#[test]
fn a_beacon_that_names_another_bssid_than_it_was_heard_from_is_not_chosen() {
    let mut h = heard(2, HOME, -40, &RSNE_PSK);
    h.beacon = beacon_of(ap(7), HOME, 3, &RSNE_PSK);
    assert_eq!(picks((ap(1), -90), &runs(), &[h]), None);
}
