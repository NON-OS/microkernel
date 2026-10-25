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

//! A join goes out from its network's address, from the first
//! authentication frame on, and keeps it; another network gets another.

use crate::soft::air::STA;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{sta_of, PASS, SSID};
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect};

/// The addresses the station's frames went out from, from the first
/// authentication on.
pub fn from_auth(st: &Station) -> Vec<[u8; 6]> {
    let sent = &st.radio().unwrap().sent;
    let at = sent.iter().position(|f| f[0] == 0xb0).expect("an authentication");
    sent[at..].iter().map(|f| f[10..16].try_into().unwrap()).collect()
}

#[test]
fn a_join_goes_out_from_its_networks_address_and_a_reconnect_keeps_it() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    let net = sta_of(&st);
    assert!(net != STA && net[0] & 0x03 == 0x02, "not the radio's own, and local");
    assert!(from_auth(&st).iter().all(|a| *a == net), "every frame from the first on");
    st.radio_mut().unwrap().sent.clear();
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    assert_eq!(sta_of(&st), net, "the same network, the same address");
    assert!(from_auth(&st).iter().all(|a| *a == net));
}

#[test]
fn another_network_sees_another_address() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    let first = sta_of(&st);
    st.radio_mut().unwrap().ap.ssid = b"Elsewhere".to_vec();
    assert_eq!(code(&connect(&mut st, b"Elsewhere", PASS, 0)), 0);
    let second = sta_of(&st);
    assert_ne!(first, second);
    assert!(second[0] & 0x03 == 0x02);
}

#[test]
fn a_visible_network_is_found_without_a_probe() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    assert!(st.radio().unwrap().sent.iter().all(|f| f[0] != 0x40), "nothing sent before");
}
