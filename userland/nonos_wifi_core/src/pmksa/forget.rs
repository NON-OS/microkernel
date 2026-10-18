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

//! Forgetting: one AP's entry when it refused it, or every entry when the
//! person disconnects.

use super::cache::{PmksaCache, PMKSA_MAX};

impl PmksaCache {
    /// Forget the entry for `bssid` in network `ssid`.
    pub fn remove(&mut self, bssid: &[u8; 6], ssid: &[u8]) {
        if let Some(i) = self.index(bssid, ssid) {
            self.remove_at(i);
        }
    }

    /// Forget every entry.
    pub fn wipe_all(&mut self) {
        (0..PMKSA_MAX).for_each(|i| self.remove_at(i));
    }

    pub fn len(&self) -> usize {
        self.kept.iter().flatten().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
