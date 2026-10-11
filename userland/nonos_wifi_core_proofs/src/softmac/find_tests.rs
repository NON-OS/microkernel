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

//! Finding the network: one not on the air, or only on a channel the world
//! domain closes, is not found; a hidden one is probed for by name, but only
//! where the domain allows a probe request.

use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::rig;
use crate::soft::wire::{code, connect};

const FLAG_HIDDEN: u8 = 2;

fn probes(sent: &[Vec<u8>]) -> Vec<&Vec<u8>> {
    sent.iter().filter(|f| f[0] == 0x40).collect()
}

#[test]
fn an_unheard_network_is_not_found_and_says_what_was_heard() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    let r = connect(&mut st, b"elsewhere", PASS, 0);
    assert_eq!(code(&r), -2);
    assert!(u32::from_le_bytes(r[8..12].try_into().unwrap()) > 0, "other beacons were heard");
    assert!(st.radio().unwrap().sent.is_empty());
}

#[test]
fn a_network_only_on_a_closed_channel_is_not_found() {
    let mut st = rig(SimAp::new(SSID, PASS, 14, &RSNE_PSK));
    let r = connect(&mut st, SSID, PASS, 0);
    assert_eq!((code(&r), u32::from_le_bytes(r[8..12].try_into().unwrap())), (-2, 0));
}

#[test]
fn a_hidden_network_is_probed_for_by_name_where_probing_is_allowed() {
    let mut ap = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    ap.hidden = true;
    let mut st = rig(ap);
    assert_eq!(code(&connect(&mut st, SSID, PASS, FLAG_HIDDEN)), 0);
    let sent = &st.radio().unwrap().sent;
    assert!(probes(sent)
        .iter()
        .all(|p| p[24..26] == [0, SSID.len() as u8] && p[26..].starts_with(SSID)));
    assert!(!probes(sent).is_empty());
}

#[test]
fn a_hidden_network_on_a_passive_channel_is_never_probed_for() {
    let mut ap = SimAp::new(SSID, PASS, 13, &RSNE_PSK);
    ap.hidden = true;
    let mut st = rig(ap);
    let r = connect(&mut st, SSID, PASS, FLAG_HIDDEN);
    assert_eq!(code(&r), -2);
    let air = st.radio().unwrap();
    assert!(!probes(&air.sent).is_empty(), "probed where allowed");
    for (f, on) in air.sent.iter().zip(&air.sent_on) {
        let ch = on.unwrap().1;
        assert!(f[0] != 0x40 || (1..=11).contains(&ch), "a probe on channel {ch}");
    }
}
