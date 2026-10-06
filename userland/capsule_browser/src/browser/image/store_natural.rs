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

use super::store::Store;

impl Store {
    /// Record the natural size read from the image, flagging the change.
    pub(crate) fn set_natural(&mut self, url: &str, size: (u32, u32)) {
        let e = self.entry(url);
        if e.natural != Some(size) {
            e.natural = Some(size);
            self.natural_dirty = true;
        }
    }

    /// The image's natural size once its header was read; it stays known
    /// after the raster is evicted.
    pub fn natural(&self, url: &str) -> Option<(u32, u32)> {
        self.entries.get(url)?.natural
    }

    /// Whether a natural size became known since the last call.
    pub fn take_natural_dirty(&mut self) -> bool {
        core::mem::take(&mut self.natural_dirty)
    }
}
