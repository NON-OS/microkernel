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

//! A hostile twin of the access point on the simulated air. It sends what
//! the AP sent, again, late, out of order and changed; to build those
//! frames this harness borrows the AP's keys, as a recording of the air
//! and the AP's own retransmissions would hold them.

use crate::ap_sim::{group1, gtk_kde, message1, message3, AA, ANONCE, GTK};
use crate::soft::data_frames::{eapol_from, from_ds_of};
use crate::soft::join::sta_of;
use crate::soft::rig::Station;
use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::wpa::akm::Akm;

/// LLC/SNAP for IPv4, then a few octets.
pub const IP: [u8; 12] = [0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0, 0, 20];

fn ptk(st: &Station) -> Vec<u8> {
    st.radio().unwrap().ap.ptk.as_ref().expect("the AP is keyed").1.clone()
}

/// Message 1 or 3 of the 4-way handshake, at replay counter `rc`.
pub fn four_way(st: &Station, third: bool, rc: u8) -> Vec<u8> {
    let kd = [&crate::soft::frames::RSNE_PSK[..], &gtk_kde(1, &GTK)].concat();
    let m = if third {
        message3(Akm::Psk, rc, &ANONCE, &ptk(st), &kd)
    } else {
        message1(Akm::Psk, rc, &ANONCE)
    };
    eapol_from(AA, sta_of(st), &m)
}

/// Group message 1 delivering `gtk` at index `id`, at replay counter `rc`.
pub fn group(st: &Station, rc: u8, gtk: &[u8; 16], id: u8) -> Vec<u8> {
    eapol_from(AA, sta_of(st), &group1(Akm::Psk, rc, &ptk(st), &gtk_kde(id, gtk)))
}

/// A data frame from the AP at `pn`: to the station under the pairwise key,
/// or to every station under `gtk` at index `id`.
pub fn data(st: &Station, pn: u64, gtk: Option<(&[u8; 16], u8)>) -> Vec<u8> {
    let tk: [u8; 16] = ptk(st)[32..48].try_into().unwrap();
    let dst = if gtk.is_some() { [0xff; 6] } else { sta_of(st) };
    let mut f = encrypt(&from_ds_of(AA, dst, &IP), pn, gtk.map_or(&tk, |g| g.0)).unwrap();
    f[27] = 0x20 | (gtk.map_or(0, |g| g.1) << 6);
    f
}

/// `plain`, an AP frame to the station, sealed under the pairwise key at `pn`.
pub fn seal(st: &Station, plain: &[u8], pn: u64) -> Vec<u8> {
    encrypt(plain, pn, &ptk(st)[32..48].try_into().unwrap()).unwrap()
}
