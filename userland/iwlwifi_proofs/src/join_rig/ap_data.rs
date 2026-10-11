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

//! The four-way handshake and CCMP data at the access point.

use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::eapol::parse::{parse as parse_key, KEY_INFO_PAIRWISE, KEY_INFO_SECURE};

use super::*;
use crate::ap_sim::{gtk_kde, message3, GTK};
use crate::gen3_model::Out;

impl Ap {
    pub(super) fn on_data(&mut self, f: &[u8]) -> Vec<Out> {
        let body = if f[1] & 0x40 != 0 {
            let Some(plain) = decrypt(f, &self.tk()) else { return vec![] };
            plain
        } else {
            f[MAC_HEADER_LEN..].to_vec()
        };
        if body.len() < 8 || body[6..8] != [0x88, 0x8E] {
            if f[1] & 0x40 != 0 {
                self.got.push(body);
            }
            return vec![];
        }
        let eapol = &body[8..];
        self.eapol_replies.push(eapol.to_vec());
        let Some(k) = parse_key(eapol) else { return vec![] };
        if k.key_info & KEY_INFO_PAIRWISE == 0 {
            return vec![]; // a group message 2
        }
        if k.key_info & KEY_INFO_SECURE != 0 {
            self.keyed = true; // message 4
            return vec![];
        }
        if self.sw.deauth_instead_of_m3 {
            return vec![rx(&mgmt(12, &[15, 0]), 0)];
        }
        self.ptk = self.akm().derive_ptk(&self.pmk, &AP, &STA, &ANONCE, &k.nonce);
        let mut kd = self.rsne.clone();
        if let Some(x) = &self.rsnxe {
            kd.extend_from_slice(x);
        }
        kd.extend_from_slice(&gtk_kde(1, &GTK));
        let s = self.next_seq();
        vec![rx(&eapol_from_ap(&message3(self.akm(), 2, &ANONCE, &self.ptk, &kd), s), 0)]
    }
}
