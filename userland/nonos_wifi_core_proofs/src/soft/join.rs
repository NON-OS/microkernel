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

//! A station joined to the simulated WPA2 network, for the proofs that start
//! from a live link.

use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect};

pub const SSID: &[u8] = b"ThisIsASSID";
pub const PASS: &[u8] = b"ThisIsAPassword";

pub fn joined() -> Station {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    let r = connect(&mut st, SSID, PASS, 0);
    assert_eq!((code(&r), r[33]), (0, 2), "joined with WPA2-PSK");
    st
}

/// The address the station sends from now: its network's, once joined.
pub fn sta_of(st: &Station) -> [u8; 6] {
    st.radio().unwrap().addr.0
}

/// The AP's deauthentication (reason 3), addressed to the station as it is now.
pub fn deauth_from_ap(st: &mut Station) {
    let sta = sta_of(st);
    let f = crate::soft::frames::mgmt(12, sta, &[3, 0]);
    st.radio_mut().unwrap().inbox.push_back(f);
}

pub fn ticks(st: &mut Station, n: usize) {
    for _ in 0..n {
        st.tick();
    }
}
