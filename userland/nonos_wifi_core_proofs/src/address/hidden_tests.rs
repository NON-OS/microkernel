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

//! A hidden network is probed from a scan's own address, fresh each scan,
//! and each address starts its sequence numbers at 0.

use super::join_tests::from_auth;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{sta_of, PASS, SSID};
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect};
use nonos_wifi_core::softmac::join::FLAG_HIDDEN;

pub fn hidden() -> Station {
    let mut ap = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    ap.hidden = true;
    rig(ap)
}

fn probes_from(st: &Station) -> Vec<[u8; 6]> {
    let sent = &st.radio().unwrap().sent;
    sent.iter().filter(|f| f[0] == 0x40).map(|f| f[10..16].try_into().unwrap()).collect()
}

fn seq(f: &[u8]) -> u16 {
    u16::from_le_bytes([f[22], f[23]]) >> 4
}

#[test]
fn a_hidden_network_is_probed_from_a_scans_address_then_joined_from_its_own() {
    let mut st = hidden();
    assert_eq!(code(&connect(&mut st, SSID, PASS, FLAG_HIDDEN)), 0);
    let (scan, net) = (probes_from(&st), sta_of(&st));
    assert!(!scan.is_empty() && scan.iter().all(|a| *a == scan[0]), "one scan, one address");
    assert!(scan[0] != net && scan[0][0] & 0x03 == 0x02);
    assert!(from_auth(&st).iter().all(|a| *a == net));
    let probe = st.radio().unwrap().sent.iter().find(|f| f[0] == 0x40).unwrap().clone();
    assert_eq!(&probe[24..26 + SSID.len()], &[&[0, SSID.len() as u8][..], SSID].concat()[..]);
    st.radio_mut().unwrap().sent.clear();
    assert_eq!(code(&connect(&mut st, SSID, PASS, FLAG_HIDDEN)), 0);
    let again = probes_from(&st);
    assert!(again[0] != scan[0] && again[0] != net, "the next scan, a fresh address");
    assert_eq!(sta_of(&st), net, "and the same network's address");
}

#[test]
fn each_address_starts_its_sequence_numbers_at_zero() {
    let mut st = hidden();
    assert_eq!(code(&connect(&mut st, SSID, PASS, FLAG_HIDDEN)), 0);
    let sent = &st.radio().unwrap().sent;
    assert_eq!(seq(sent.iter().find(|f| f[0] == 0x40).unwrap()), 0, "the scan's first");
    assert_eq!(seq(sent.iter().find(|f| f[0] == 0xb0).unwrap()), 0, "the network's first");
}
