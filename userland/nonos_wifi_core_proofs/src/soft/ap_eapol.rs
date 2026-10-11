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

//! The access point's half of the four-way handshake.

use crate::ap_sim::{gtk_kde, message3, AA, ANONCE, GTK};
use crate::soft::ap::SimAp;
use crate::soft::frames::{eapol_to, mgmt};
use nonos_wifi_core::eapol::mic::compute_mic;
use nonos_wifi_core::eapol::parse::{parse, KEY_INFO_SECURE, MIC_OFFSET};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;

impl SimAp {
    pub(crate) fn on_eapol(&mut self, sta: [u8; 6], e: Vec<u8>) -> Vec<Vec<u8>> {
        let Some(k) = parse(&e) else { return vec![] };
        if k.key_info & KEY_INFO_SECURE != 0 {
            self.tk = self.ptk.map(|p| p[32..48].try_into().unwrap());
            return vec![];
        }
        let ptk = Akm::Psk.derive_ptk(&pmk(&self.pass, &self.ssid), &AA, &sta, &ANONCE, &k.nonce);
        if compute_mic(&ptk[..16], &e) != e[MIC_OFFSET..MIC_OFFSET + 16] {
            return vec![mgmt(12, sta, &15u16.to_le_bytes())];
        }
        self.ptk = Some(ptk);
        let mut kd = self.rsne.clone();
        kd.extend_from_slice(&gtk_kde(1, &GTK));
        vec![eapol_to(sta, &message3(Akm::Psk, 2, &ANONCE, &ptk, &kd))]
    }
}
