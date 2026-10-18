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

//! The access point's half of the four-way handshake, keyed for the pairwise
//! cipher the station's RSNE in message 2 names.

use crate::ap_sim::{bigtk_kde, gtk_kde, igtk_kde, message3, ANONCE, GTK};
use crate::cipher_sim::GTK256;
use crate::pmf_sim::IGTK;
use crate::soft::ap::SimAp;
use crate::soft::frames::{eapol_from, mgmt_from};
use crate::soft::protect::BIGTK;
use nonos_wifi_core::eapol::mic::compute_mic;
use nonos_wifi_core::eapol::parse::{parse, KEY_INFO_SECURE, MIC_OFFSET};
use nonos_wifi_core::rsn::{parse_rsne, Cipher, Rsne, TemporalKey};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use Cipher::{Ccmp128, Gcmp256};

fn rsne_of(element: &[u8]) -> Option<Rsne> {
    parse_rsne(element.get(2..)?)
}

impl SimAp {
    pub(crate) fn on_eapol(&mut self, sta: [u8; 6], e: Vec<u8>) -> Vec<Vec<u8>> {
        let Some(k) = parse(&e).filter(|_| !self.quirks.mute_eapol) else { return vec![] };
        if k.key_info & KEY_INFO_SECURE != 0 {
            self.tk = self.ptk.as_ref().and_then(|(c, p)| TemporalKey::new(*c, &p[32..]));
            return vec![];
        }
        // Keyed, this AP starts no rekey: no message 2 now answers its own.
        if self.tk.is_some() {
            return vec![];
        }
        let cipher =
            rsne_of(k.key_data).filter(|r| r.pairwise_gcmp256).map_or(Ccmp128, |_| Gcmp256);
        let mut ptk = vec![0u8; 32 + cipher.tk_len()];
        let pmk = pmk(&self.pass, &self.ssid);
        Akm::Psk.derive_ptk_into(&pmk, (&self.bssid, &sta), (&ANONCE, &k.nonce), &mut ptk);
        if compute_mic(&ptk[..16], &e) != e[MIC_OFFSET..MIC_OFFSET + 16] {
            return vec![mgmt_from(self.bssid, 12, sta, &15u16.to_le_bytes())];
        }
        // The group cipher is the AP's own; GCMP-256 takes a 32-octet GTK.
        let group = rsne_of(&self.rsne).and_then(|r| Cipher::from_selector(r.group));
        let gtk: &[u8] = if group == Some(Gcmp256) { &GTK256 } else { &GTK };
        let mut kd = self.quirks.msg3_rsne.clone().unwrap_or_else(|| self.rsne.clone());
        kd.extend_from_slice(&gtk_kde(1, gtk));
        // With management frame protection, the integrity group key too.
        if rsne_of(&self.rsne).is_some_and(|r| r.mfpc()) {
            kd.extend_from_slice(&igtk_kde(&IGTK));
        }
        if self.quirks.beacons.protect {
            kd.extend_from_slice(&bigtk_kde(6, 0, &BIGTK));
        }
        if let Some(bits) = self.quirks.transition_disable {
            kd.extend_from_slice(&[0xDD, 5, 0x50, 0x6F, 0x9A, 0x20, bits]);
        }
        let m3 = message3(Akm::Psk, 2, &ANONCE, &ptk, &kd);
        self.ptk = Some((cipher, ptk));
        vec![eapol_from(self.bssid, sta, &m3)]
    }
}
