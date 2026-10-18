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

//! Row 1.2: every handshake draws fresh randomness. Two roams, there and
//! back, send three different SNonces with the join's; two SAE joins send
//! two different commit scalars.

use crate::roam::beacons::RSNE_SAE;
use crate::roam::two_aps::{run_until, two_aps, AA, BB};
use crate::soft::ap::SimAp;
use crate::soft::frames::eapol_of;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::{rig, Station};
use crate::soft::wire::connect;

/// The nonces of the station's EAPOL-Key frames that carry one (message 2).
fn snonces(st: &Station) -> Vec<Vec<u8>> {
    let sent = &st.radio().unwrap().sent;
    let nonce = |e: &[u8]| e.get(17..49).map(<[u8]>::to_vec);
    let mut n: Vec<Vec<u8>> = sent.iter().filter_map(|f| eapol_of(f).and_then(nonce)).collect();
    n.retain(|x| x.iter().any(|&b| b != 0));
    n
}

fn on(st: &mut Station, ap: [u8; 6]) -> bool {
    (0..3).any(|_| run_until(st, |s| s.session().is_some_and(|s| s.bssid == ap)))
}

#[test]
fn every_roam_runs_on_a_fresh_snonce() {
    let mut st = two_aps(|_| {}, [-85, -50]);
    assert!(on(&mut st, BB), "moved");
    st.radio_mut().unwrap().rssi = [-50, -85];
    assert!(on(&mut st, AA), "and back");
    let n = snonces(&st);
    assert!(n.len() >= 3, "the join's and two roams'");
    for (i, a) in n.iter().enumerate() {
        assert!(n[i + 1..].iter().all(|b| b != a), "SNonce {i} repeated");
    }
}

#[test]
fn every_sae_exchange_runs_on_a_fresh_scalar() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_SAE));
    connect(&mut st, SSID, PASS, 1);
    connect(&mut st, SSID, PASS, 1);
    let sent = &st.radio().unwrap().sent;
    // Authentication, SAE, a commit: the scalar after the group number.
    let commit = |f: &&Vec<u8>| f[0] == 0xb0 && f.get(24..28) == Some(&[3, 0, 1, 0]);
    let scalars: Vec<&[u8]> = sent.iter().filter(commit).map(|f| &f[32..64]).collect();
    assert!(scalars.len() >= 2, "two commits");
    assert_ne!(scalars[0], scalars[scalars.len() - 1]);
}
