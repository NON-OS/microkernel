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

//! Finding, ageing and clearing places in the cache.

use super::cache::{PmksaCache, PMKSA_MAX, PMK_LIFETIME_MS};
use super::entry::wipe;

impl PmksaCache {
    pub(super) fn index(&self, bssid: &[u8; 6], ssid: &[u8]) -> Option<usize> {
        self.kept.iter().position(|k| {
            k.is_some_and(|k| k.sa.bssid == *bssid && k.ssid.get(..k.ssid_len) == Some(ssid))
        })
    }

    pub(super) fn oldest(&self) -> usize {
        let age = |i: usize| self.kept[i].map_or(0, |k| k.since_ms);
        (0..PMKSA_MAX).min_by_key(|&i| age(i)).unwrap_or(0)
    }

    pub(super) fn remove_at(&mut self, i: usize) {
        if let Some(mut k) = self.kept[i].take() {
            wipe(&mut k.sa, &mut k.tag);
        }
    }

    pub(super) fn expire(&mut self, now_ms: u64) {
        for i in 0..PMKSA_MAX {
            let old =
                self.kept[i].is_some_and(|k| now_ms.saturating_sub(k.since_ms) >= PMK_LIFETIME_MS);
            if old {
                self.remove_at(i);
            }
        }
    }
}

impl Drop for PmksaCache {
    fn drop(&mut self) {
        self.wipe_all();
    }
}
