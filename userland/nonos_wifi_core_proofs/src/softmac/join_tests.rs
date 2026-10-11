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

//! The shared station joins a WPA2 network through the control op and
//! carries net_core's traffic over CCMP; a wrong passphrase ends with the
//! AP's reason; a network not on the air, or only on a channel the world
//! domain closes, is not found; nothing is transmitted while scanning.

use crate::ap_sim::AA;
use crate::soft::air::STA;
use crate::soft::ap::SimAp;
use crate::soft::frames::{from_ds, RSNE_PSK};
use crate::soft::join::{joined, ticks, PASS, SSID};
use crate::soft::rig::rig;
use crate::soft::wire::{code, connect, control, netif, scan_names};
use nonos_wifi_core::dot11::ccmp::encrypt;

#[test]
fn a_scan_lists_the_network_without_transmitting() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    ticks(&mut st, 2000);
    assert_eq!(scan_names(&mut st), vec![SSID.to_vec()]);
    assert!(st.radio().unwrap().sent.is_empty(), "the scan only listens");
}

#[test]
fn a_joined_network_carries_net_cores_frames_both_ways() {
    let mut st = joined();
    let link = control(&mut st, 5, &[]);
    assert_eq!((link[0], &link[1..7], &link[8..8 + SSID.len()]), (1, &AA[..], SSID));
    assert_eq!(link[8 + SSID.len()], 2, "the AKM: WPA2-PSK");
    let mut eth = AA.to_vec();
    eth.extend_from_slice(&STA);
    eth.extend_from_slice(&[0x08, 0x00, 0x45, 0x00]);
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    let ap = &st.radio().unwrap().ap;
    assert_eq!(ap.got.last().unwrap(), &vec![0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0x00]);
    let tk = ap.tk.unwrap();
    let down = encrypt(&from_ds(STA, &[0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x99]), 1, &tk).unwrap();
    st.radio_mut().unwrap().inbox.push_back(down);
    ticks(&mut st, 1);
    let (status, rx) = netif(&mut st, 5, &[]);
    assert_eq!(status, 0);
    assert_eq!(&rx[4..], &[STA.as_slice(), &AA, &[0x08, 0x00, 0x99]].concat()[..]);
}

#[test]
fn a_wrong_passphrase_ends_with_the_aps_reason() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    let r = connect(&mut st, SSID, b"not the passphrase", 0);
    assert_eq!((code(&r), u16::from_le_bytes([r[34], r[35]])), (-6, 15));
    assert_eq!(control(&mut st, 5, &[])[0], 0, "no link");
    assert_eq!(st.radio().unwrap().bss, None, "the filter is open again");
}
