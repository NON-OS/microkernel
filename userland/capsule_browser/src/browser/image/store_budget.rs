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

use super::store::{Decoded, Store};
use super::store_entry::Status;

/* Ceiling on resident decoded pixels, kept well under the process heap so
 * a page's DOM, layout and transient fetch buffers always have room. */
pub(crate) const BYTE_BUDGET: usize = 16 * 1024 * 1024;

impl Store {
    /// Make room for `need` bytes of raster for `url` before it is
    /// decoded: evict other rasters, least recently painted first, until it
    /// fits or none is left. Returns the bytes then free.
    pub(crate) fn room(&mut self, url: &str, need: usize) -> usize {
        while self.bytes.saturating_add(need) > BYTE_BUDGET {
            let victim = self
                .entries
                .iter()
                .filter(|(k, e)| k.as_str() != url && matches!(e.status, Status::Ready(_)))
                .min_by_key(|(_, e)| e.used.get())
                .map(|(k, _)| k.clone());
            let Some(v) = victim else { break };
            self.drop_raster(&v, Status::Evicted);
            #[cfg(not(feature = "harness"))]
            self.note_evicted(&v);
        }
        BYTE_BUDGET.saturating_sub(self.bytes)
    }

    /// Keep a decoded raster for `url`, replacing any earlier one; a raster
    /// that cannot fit even after eviction is recorded as failed.
    pub(crate) fn set_ready(&mut self, url: &str, d: Decoded) {
        let cost = d.px.len().saturating_mul(4);
        self.drop_raster(url, Status::Pending);
        if self.room(url, cost) < cost {
            self.set_failed(url);
            return;
        }
        self.bytes += cost;
        self.clock.set(self.clock.get() + 1);
        let now = self.clock.get();
        let e = self.entry(url);
        e.status = Status::Ready(d);
        e.used.set(now);
    }

    /// Put `url` in state `to`, returning its raster's bytes to the budget.
    fn drop_raster(&mut self, url: &str, to: Status) {
        let Some(e) = self.entries.get_mut(url) else { return };
        if let Status::Ready(d) = &e.status {
            self.bytes = self.bytes.saturating_sub(d.px.len() * 4);
            e.status = to;
        }
    }
}
