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

//! E-68 helpers: a supplicant brought to Connected, and one hostile frame
//! per round — a message 3 retransmit, a message 1 or group replay, a
//! rekey, the key in use resent, or a forged MIC — with its replay counter,
//! whether its MIC is forged, and whether the replay-counter rule governs
//! it (message 1 restarts the exchange, so it stands apart).

use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for};
use crate::sweep::rng::Rng;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::Supplicant;

pub const KEYS: [[u8; 16]; 3] = [GTK, [0x7e; 16], [0x3c; 16]];

/// A supplicant joined under WPA2-PSK, with the PTK the AP derived.
pub fn connected() -> (Supplicant, [u8; 48]) {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    (sup, ptk)
}

/// One hostile frame: `(frame, replay counter, MIC forged, counter-governed)`.
pub fn act(rng: &mut Rng, sup: &Supplicant, ptk: &[u8; 48], max: u8) -> (Vec<u8>, u8, bool, bool) {
    let fresh = if max < 250 { max + 1 } else { 1 };
    let old = 1 + rng.below(max as usize) as u8;
    let (mut cur, id) = ([0u8; 16], sup.gtk_id());
    cur.copy_from_slice(sup.gtk());
    let m3 = |c, k: &[u8; 16]| {
        message3(Akm::Psk, c, &ANONCE, ptk, &[&AP_RSNE_MIXED[..], &gtk_kde(1, k)].concat())
    };
    let g = |c, i, k: &[u8; 16]| group1(Akm::Psk, c, ptk, &gtk_kde(i, k));
    match rng.below(7) {
        0 => (m3(fresh, &GTK), fresh, false, true),
        1 => (m3(old, &GTK), old, false, true),
        2 => (g(fresh, 1 + rng.below(2) as u8, &KEYS[rng.below(3)]), fresh, false, true),
        3 => (g(old, id, &cur), old, false, true),
        4 => (g(fresh, id, &cur), fresh, false, true),
        5 => {
            let mut f = if rng.below(2) == 0 { m3(fresh, &GTK) } else { g(fresh, id, &cur) };
            f[90] ^= 1u8 << (rng.below(7) as u8);
            (f, fresh, true, true)
        }
        _ => (message1(Akm::Psk, old, &ANONCE), old, false, false),
    }
}
