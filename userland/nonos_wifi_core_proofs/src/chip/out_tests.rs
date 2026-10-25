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

//! Every key of a link leaves the station on every orderly end: the
//! pairwise key, the group key, the integrity group key and the beacon
//! integrity key, with the supplicant that holds them. A soft-MAC chip is
//! given none of them (`Radio` has no way to take a key), so there is
//! nothing in it to take back; what the station held it wipes.

use crate::ap_sim::{AA, SPA};
use crate::beacons::sim::joined_bp;
use crate::pmf_frames::{mgmt, sealed};
use nonos_wifi_core::station::{Ccmp, MgmtRx};

#[test]
fn every_key_goes_when_the_station_leaves() {
    let (mut s, _) = joined_bp();
    assert!(s.holds_keys(), "a link with a PTK, GTK, IGTK and BIGTK");
    s.deassociate();
    assert!(!s.holds_keys());
}

#[test]
fn every_key_goes_when_the_access_point_ends_the_link() {
    let (mut s, ptk) = joined_bp();
    let deauth = sealed(&mgmt(12, SPA, &[3, 0]), 9, &ptk, false);
    assert_eq!(s.receive_mgmt(&deauth, false), MgmtRx::Left(3));
    assert!(!s.holds_keys());
}

#[test]
fn a_new_association_keeps_nothing_of_the_last() {
    let (mut s, _) = joined_bp();
    s.associate(AA, Ccmp::Hardware);
    assert!(!s.holds_keys(), "none until this association's handshake");
}
