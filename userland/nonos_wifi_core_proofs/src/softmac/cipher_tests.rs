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

//! Joins pick the cipher radio and AP share: a radio that says nothing joins
//! a GCMP-256 network with CCMP-128, a GCMP-256 radio joins with it and
//! carries frames both ways, and a GCMP-only network refuses a CCMP radio.

use crate::ap_sim::AA;
use crate::cipher_sim::{GCMP_ONLY, MIXED};
use crate::soft::join::{ticks, PASS, SSID};
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect, control, netif};
use crate::soft::{air::STA, ap::SimAp, frames::from_ds};
use nonos_wifi_core::dot11::gcmp::encrypt;
use nonos_wifi_core::rsn::{Cipher, TemporalKey};

fn joined(rsne: &[u8], gcmp: bool) -> (Station, i32) {
    let mut st = rig(SimAp::new(SSID, PASS, 6, rsne));
    st.radio_mut().unwrap().gcmp = gcmp;
    let r = connect(&mut st, SSID, PASS, 0);
    (st, code(&r))
}

fn cipher_told(st: &Station) -> Option<Cipher> {
    st.radio().unwrap().assoc.iter().flatten().last().map(|a| a.cipher)
}

#[test]
fn a_radio_that_says_nothing_joins_with_ccmp_128() {
    let (st, code) = joined(&MIXED, false);
    assert_eq!(code, 0);
    assert!(matches!(st.radio().unwrap().ap.tk, Some(TemporalKey::Ccmp128(_))));
    assert_eq!(cipher_told(&st), Some(Cipher::Ccmp128));
}

#[test]
fn a_gcmp_radio_joins_with_gcmp_256_and_carries_frames_both_ways() {
    let (mut st, code) = joined(&MIXED, true);
    assert_eq!(code, 0);
    assert_eq!(cipher_told(&st), Some(Cipher::Gcmp256), "the radio is told the cipher");
    let Some(TemporalKey::Gcmp256(tk)) = st.radio().unwrap().ap.tk else { panic!("GCMP-256") };
    let eth = [&AA[..], &STA, &[0x08, 0x00, 0x45, 0x00]].concat();
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    let got = st.radio().unwrap().ap.got.last().cloned();
    assert_eq!(got.unwrap(), [0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0x00]);
    let down = encrypt(&from_ds(STA, &[0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x99]), 1, &tk);
    st.radio_mut().unwrap().inbox.push_back(down.unwrap());
    ticks(&mut st, 1);
    let (status, rx) = netif(&mut st, 5, &[]);
    assert_eq!((status, &rx[4..]), (0, &[&STA[..], &AA, &[0x08, 0x00, 0x99]].concat()[..]));
}

#[test]
fn a_gcmp_only_network_needs_a_gcmp_radio() {
    let (mut st, code) = joined(&GCMP_ONLY, false);
    assert_ne!(code, 0, "refused");
    assert_eq!(control(&mut st, 5, &[])[0], 0, "no link");
    let (mut st, code) = joined(&GCMP_ONLY, true);
    assert_eq!(code, 0, "joined, the 32-octet GTK taken");
    assert_eq!(control(&mut st, 5, &[])[0], 1);
}
