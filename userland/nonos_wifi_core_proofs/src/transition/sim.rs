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

//! The Transition Disable KDE and the frames that carry it.

use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::Supplicant;

/// A Transition Disable KDE carrying `bits`.
pub fn td_kde(bits: u8) -> Vec<u8> {
    vec![0xDD, 5, 0x50, 0x6F, 0x9A, 0x20, bits]
}

/// Message 3's key data: the AP's RSNE, the GTK KDE, then `extra`.
pub fn kd3(extra: &[u8]) -> Vec<u8> {
    [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK), extra].concat()
}

/// A PSK supplicant past message 1, and the PTK that handshake runs under.
pub fn after_message1(m1_kd: &[u8]) -> (Supplicant, [u8; 48]) {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    let info = nonos_wifi_core::eapol::parse::KEY_INFO_PAIRWISE
        | nonos_wifi_core::eapol::parse::KEY_INFO_ACK;
    sup.step(&frame(Akm::Psk, info, 1, &ANONCE, m1_kd, &[0u8; 16]));
    (sup, ptk_for(Akm::Psk, &psk(), &ANONCE))
}

/// A supplicant connected by a message 3 whose key data is `kd`.
pub fn connected(kd: &[u8]) -> (Supplicant, [u8; 48]) {
    let (mut sup, ptk) = after_message1(&[]);
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, kd));
    (sup, ptk)
}
