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

//! What the access point answers to each frame it hears from the station.

use crate::ap_sim::{message1, ANONCE};
use crate::soft::ap::SimAp;
use crate::soft::frames::{beacon_from, eapol_from, eapol_of, mgmt_from};
use nonos_wifi_core::dot11::{ccmp, gcmp};
use nonos_wifi_core::rsn::TemporalKey;
use nonos_wifi_core::wpa::akm::Akm;

impl SimAp {
    /// What the AP sends back for one frame it heard.
    pub fn on_frame(&mut self, f: &[u8]) -> Vec<Vec<u8>> {
        let (mut sta, me) = ([0u8; 6], self.bssid);
        sta.copy_from_slice(&f[10..16]);
        match (f[0] >> 2 & 3, f[0] >> 4) {
            (0, 11) => vec![mgmt_from(me, 11, sta, &[0, 0, 2, 0, 0, 0])],
            (0, sub @ (0 | 2)) if self.refuse => {
                vec![mgmt_from(me, sub + 1, sta, &[0x11, 4, 17, 0, 0, 0])]
            }
            (0, sub @ (0 | 2)) if self.quirks.mute_eapol => {
                vec![mgmt_from(me, sub + 1, sta, &[0x11, 4, 0, 0, 1, 0xc0])]
            }
            // An association, or a reassociation answered in kind; the keys
            // of any session before are gone, and the handshake starts.
            (0, sub @ (0 | 2)) => {
                self.tk = None;
                vec![
                    mgmt_from(me, sub + 1, sta, &[0x11, 4, 0, 0, 1, 0xc0]),
                    eapol_from(me, sta, &message1(Akm::Psk, 1, &ANONCE)),
                ]
            }
            (0, 4)
                if f.get(24..26) == Some(&[0, self.ssid.len() as u8])
                    && f[26..].starts_with(&self.ssid) =>
            {
                vec![beacon_from(me, 5, sta, (&self.ssid, self.channel, &self.rsne))]
            }
            (2, _) if f[1] & 0x40 != 0 => {
                let body = match self.tk {
                    Some(TemporalKey::Ccmp128(tk)) => ccmp::decrypt(f, &tk),
                    Some(TemporalKey::Gcmp256(tk)) => gcmp::decrypt(f, &tk),
                    None => None,
                };
                if let Some(body) = body {
                    self.got.push(body);
                }
                vec![]
            }
            (2, _) => eapol_of(f).map_or(vec![], |e| self.on_eapol(sta, e.to_vec())),
            _ => vec![],
        }
    }
}
