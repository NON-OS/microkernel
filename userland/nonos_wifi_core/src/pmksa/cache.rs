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
//! `PMKSA_MAX` entries, each for one AP of one network, made from one
//! station address with one passphrase: a changed or wrong passphrase runs
//! SAE again, and fails there, rather than joining on the old PMK. The
//! passphrase is named by its `Tag`; a cache with no tag key keeps
//! nothing. An entry older than the PMK lifetime is not offered. Every
//! entry removed, replaced or outlived is wiped, and so is the whole cache
//! when the person disconnects or the cache is dropped.

use super::entry::{Kept, Pmksa};
use super::tag::Tag;
use crate::vault::Vault;

pub const PMKSA_MAX: usize = 4;
/// dot11RSNAConfigPMKLifetime's default, 43200 s (802.11-2020, Annex C).
pub const PMK_LIFETIME_MS: u64 = 43_200_000;

#[derive(Default)]
pub struct PmksaCache {
    pub(super) kept: [Option<Kept>; PMKSA_MAX],
    pub(super) key: Option<Vault<32>>,
}

impl PmksaCache {
    /// The live entry for `bssid` in network `ssid`, made from station
    /// `sta` under the passphrase `tag` names.
    pub fn find(
        &mut self,
        at: ([u8; 6], [u8; 6]),
        ssid: &[u8],
        tag: &Tag,
        now: u64,
    ) -> Option<Pmksa> {
        let (bssid, sta) = at;
        self.expire(now);
        let k = self.kept[self.index(&bssid, ssid)?].as_ref()?;
        (k.sta == sta && tag.same(&k.tag)).then(|| k.sa.again())
    }

    /// Keep `sa` for network `ssid`, in place of any entry for the same AP,
    /// else in a free place, else in place of the oldest.
    pub fn keep(&mut self, sta: [u8; 6], ssid: &[u8], tag: &Tag, sa: Pmksa, now_ms: u64) {
        if ssid.len() > 32 || self.key.is_none() {
            return;
        }
        let slot =
            self.index(&sa.bssid, ssid).or_else(|| self.kept.iter().position(Option::is_none));
        let slot = slot.unwrap_or_else(|| self.oldest());
        self.remove_at(slot);
        let mut t = [0u8; 32];
        t.copy_from_slice(tag.0.get());
        let mut k = Kept { sta, ssid: [0; 32], ssid_len: ssid.len(), since_ms: now_ms, tag: t, sa };
        k.ssid[..ssid.len()].copy_from_slice(ssid);
        self.kept[slot] = Some(k);
    }
}
