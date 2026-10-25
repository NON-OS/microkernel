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

//! Two APs of one network for the roaming proofs: the station joined to
//! the first, on channel 1, and a second on channel 6, switched on after
//! the join so the join cannot pick it. Both send protected data once
//! keyed, which is what the station judges an AP's signal by.

use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect};
use nonos_wifi_core::softmac::roam::COOLDOWN_MS;

pub const AA: [u8; 6] = crate::ap_sim::AA;
pub const BB: [u8; 6] = [0x02, 0, 0, 0, 0, 0x0b];

/// Joined to AA; BB, as `second` sets it up, heard at `rssi` dBm.
pub fn two_aps(second: impl FnOnce(&mut SimAp), rssi: [i8; 2]) -> Station {
    two_aps_with(|_| {}, second, rssi)
}

/// The same, with AA set up by `first` before the join.
pub fn two_aps_with(
    first: impl FnOnce(&mut SimAp),
    second: impl FnOnce(&mut SimAp),
    rssi: [i8; 2],
) -> Station {
    let mut a = SimAp::new(SSID, PASS, 1, &RSNE_PSK);
    a.chatter = true;
    first(&mut a);
    let mut st = rig(a);
    let mut b = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    (b.bssid, b.on, b.chatter) = (BB, false, true);
    second(&mut b);
    let air = st.radio_mut().unwrap();
    air.ap2 = Some(b);
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    assert_eq!(st.session().unwrap().bssid, AA);
    let air = st.radio_mut().unwrap();
    air.ap2.as_mut().unwrap().on = true;
    air.rssi = rssi;
    st
}

/// Serve passes until `done` or one cooldown of the simulated clock.
pub fn run_until(st: &mut Station, done: impl Fn(&Station) -> bool) -> bool {
    let start = st.radio().unwrap().clock.get();
    while st.radio().unwrap().clock.get() < start + COOLDOWN_MS {
        st.tick();
        if done(st) {
            return true;
        }
    }
    false
}

/// The frames the station sent with this first octet.
pub fn sent(st: &Station, fc0: u8) -> Vec<Vec<u8>> {
    st.radio().unwrap().sent.iter().filter(|f| f[0] == fc0).cloned().collect()
}
