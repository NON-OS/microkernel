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

//! The radio hears of the association once the AP accepts it, before the
//! handshake, with what the response and the beacon settled; and hears it
//! end when the join fails after it or the session ends.

use crate::ap_sim::AA;
use crate::soft::air::STA;
use crate::soft::ap::SimAp;
use crate::soft::frames::{mgmt, RSNE_PSK};
use crate::soft::join::{joined, ticks, PASS, SSID};
use crate::soft::rig::rig;
use crate::soft::wire::connect;
use nonos_wifi_core::softmac::Association;

/// SimAp's response: capability 0x0411, AID 1 (0xc001 on the air); its
/// beacons advertise 100 TU.
fn sim_assoc() -> Association {
    Association {
        bssid: AA,
        aid: 1,
        capability: 0x0411,
        beacon_int: 100,
        rates: [0; 16],
        n_rates: 0,
    }
}

#[test]
fn a_join_reports_the_association_before_the_handshake_ends() {
    let st = joined();
    assert_eq!(st.radio().unwrap().assoc, vec![Some(sim_assoc())]);
}

#[test]
fn a_join_failing_after_association_reports_its_end() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    connect(&mut st, SSID, b"not the passphrase", 0);
    assert_eq!(st.radio().unwrap().assoc.last(), Some(&None));
}

#[test]
fn a_deauthentication_reports_the_end() {
    let mut st = joined();
    st.radio_mut().unwrap().inbox.push_back(mgmt(12, STA, &[3, 0]));
    ticks(&mut st, 1);
    assert_eq!(st.radio().unwrap().assoc.last(), Some(&None));
}

#[test]
fn the_rates_come_from_the_response_with_their_basic_bits() {
    let body = [0x11, 4, 0, 0, 2, 0xc0, 1, 2, 0x82, 0x84, 50, 1, 0x6c];
    let mut beacon = mgmt(8, [0xff; 6], &[0u8; 12]);
    beacon[32..34].copy_from_slice(&200u16.to_le_bytes());
    let a = Association::parse(&mgmt(1, STA, &body), &beacon).unwrap();
    assert_eq!((a.aid, a.beacon_int, a.n_rates), (2, 200, 3));
    assert_eq!(&a.rates[..3], &[0x82, 0x84, 0x6c]);
}
