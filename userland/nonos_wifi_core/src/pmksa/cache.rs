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

//! The station's PMKSA cache. It is held in memory for this boot only and
//! never written anywhere, as Bluetooth bonds are, and holds at most
//! `PMKSA_MAX` entries, each for one AP of one network and the passphrase
//! it was made with: a changed or wrong passphrase runs SAE again, and
//! fails there, rather than joining on the old PMK. An entry older than
//! the PMK lifetime is not offered. Every entry removed, replaced or
//! outlived is wiped, and so is the whole cache when the person
//! disconnects or the cache is dropped.

use super::entry::{Kept, Pmksa};
use crate::wpa::sha256::hmac_sha256;

pub const PMKSA_MAX: usize = 4;
/// dot11RSNAConfigPMKLifetime's default, 43200 s (802.11-2020, Annex C).
pub const PMK_LIFETIME_MS: u64 = 43_200_000;

#[derive(Default)]
pub struct PmksaCache {
    pub(super) kept: [Option<Kept>; PMKSA_MAX],
}

impl PmksaCache {
    /// The live entry for `bssid` in network `ssid` under passphrase `pass`.
    pub fn find(
        &mut self,
        bssid: &[u8; 6],
        ssid: &[u8],
        pass: &[u8],
        now_ms: u64,
    ) -> Option<Pmksa> {
        self.expire(now_ms);
        let k = self.kept[self.index(bssid, ssid)?]?;
        (k.tag == hmac_sha256(ssid, pass)).then_some(k.sa)
    }

    /// Keep `sa` for network `ssid`, in place of any entry for the same AP,
    /// else in a free place, else in place of the oldest.
    pub fn keep(&mut self, ssid: &[u8], pass: &[u8], sa: Pmksa, now_ms: u64) {
        if ssid.len() > 32 {
            return;
        }
        let slot =
            self.index(&sa.bssid, ssid).or_else(|| self.kept.iter().position(Option::is_none));
        let slot = slot.unwrap_or_else(|| self.oldest());
        self.remove_at(slot);
        let tag = hmac_sha256(ssid, pass);
        let mut k = Kept { ssid: [0; 32], ssid_len: ssid.len(), since_ms: now_ms, tag, sa };
        k.ssid[..ssid.len()].copy_from_slice(ssid);
        self.kept[slot] = Some(k);
    }
}
