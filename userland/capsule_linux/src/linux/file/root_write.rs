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

//! Whether a guest may write where a key points, and its path as the guest
//! names it, for a box's report.

use super::root::Key;

impl Key {
    /// The path as the guest names it.
    pub fn visible(&self) -> &[u8] {
        match &self.0[self.2..] {
            [] => b"/",
            v => v,
        }
    }

    /// Whether a write here is allowed, and a refused one counted against a
    /// box: what a guest does, not what it asks about (`may_write`).
    pub fn writable(&self) -> Result<(), &'static str> {
        let allowed = self.may_write();
        if allowed.is_err() {
            crate::linux::boxed::note(crate::linux::boxed::Kind::Write, self.visible());
        }
        allowed
    }

    /// The shared tree is written by installs alone, and a boxed run writes
    /// only the folders its box names; asked, as access(W_OK) asks, unrecorded.
    pub fn may_write(&self) -> Result<(), &'static str> {
        if self.1 && !super::private::shared_writes_allowed() {
            return Err("read-only file system");
        }
        match crate::linux::boxed::may_write(self.visible()) {
            true => Ok(()),
            false => Err("read-only file system: outside the box's folders"),
        }
    }
}
