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

//! The station's check of one beacon under the BIGTK.

use super::sim::{bare, joined_bp, joined_with, signed};
use crate::pmf_sim::joined_pmf;
use crate::soft::protect::BIGTK;
use nonos_wifi_core::station::{BeaconRx, MgmtDrop};

#[test]
fn without_a_bigtk_the_link_runs_no_beacon_protection() {
    let (mut s, _) = joined_pmf();
    assert_eq!(s.receive_beacon(&bare()), BeaconRx::Unprotected);
    let (mut s, _) = joined_with(&[]);
    assert_eq!(s.receive_beacon(&signed(6, 4, &BIGTK)), BeaconRx::Unprotected);
}

#[test]
fn a_bare_forged_or_misindexed_beacon_is_refused() {
    let (mut s, _) = joined_bp();
    let refused = BeaconRx::Refused;
    assert_eq!(s.receive_beacon(&bare()), refused(MgmtDrop::Unprotected));
    assert_eq!(s.receive_beacon(&signed(6, 4, &[0x67; 16])), refused(MgmtDrop::Undecryptable));
    assert_eq!(s.receive_beacon(&signed(7, 4, &BIGTK)), refused(MgmtDrop::Undecryptable));
    let mut other = signed(6, 4, &BIGTK);
    other[16..22].copy_from_slice(&[2, 0, 0, 0, 0, 0x77]);
    assert_eq!(s.receive_beacon(&other), refused(MgmtDrop::NotOurs));
    assert_eq!(
        s.receive_beacon(&signed(6, 4, &BIGTK)),
        BeaconRx::Verified,
        "all that left no mark"
    );
}

#[test]
fn the_aps_beacon_verifies_while_its_bipn_climbs() {
    let (mut s, _) = joined_bp();
    assert_eq!(s.receive_beacon(&signed(6, 4, &BIGTK)), BeaconRx::Verified);
    let replay = BeaconRx::Refused(MgmtDrop::Replay);
    assert_eq!(s.receive_beacon(&signed(6, 3, &BIGTK)), replay, "the BIPN the KDE gave");
    assert_eq!(s.receive_beacon(&signed(6, 9, &BIGTK)), BeaconRx::Verified);
    assert_eq!(s.receive_beacon(&signed(6, 9, &BIGTK)), replay, "the same beacon again");
    assert_eq!(s.receive_beacon(&signed(6, 5, &BIGTK)), replay, "an older one");
    assert_eq!(s.receive_beacon(&signed(6, 10, &BIGTK)), BeaconRx::Verified);
}

#[test]
fn the_timestamp_is_outside_the_mic_and_the_rest_is_not() {
    let (mut s, _) = joined_bp();
    let mut late = signed(6, 4, &BIGTK);
    late[24..32].copy_from_slice(&7_000_000u64.to_le_bytes());
    assert_eq!(s.receive_beacon(&late), BeaconRx::Verified, "the Timestamp is masked");
    let mut changed = signed(6, 5, &BIGTK);
    changed[32] ^= 1;
    let r = s.receive_beacon(&changed);
    assert_eq!(r, BeaconRx::Refused(MgmtDrop::Undecryptable), "the Beacon Interval is not");
}
