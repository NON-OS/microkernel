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

//! The link's keys, wiped when the association ends and when the station
//! is dropped: the pairwise key, the group keys, the integrity group key.
//! The supplicant wipes itself as it is dropped.

use super::{Ccmp, LinkStation};
use crate::vault::wipe;

impl LinkStation {
    pub(super) fn wipe_keys(&mut self) {
        match &mut self.ccmp {
            Ccmp::Software { tk } | Ccmp::SoftwareTxHwRx { tk } => wipe(tk),
            Ccmp::Gcmp256 { tk } => wipe(tk),
            Ccmp::Hardware => {}
        }
        self.ccmp = Ccmp::Hardware;
        // Each group key wipes itself as it is dropped.
        self.group_keys = [const { None }; 4];
        if let Some((key, _)) = self.pmf.igtk.as_mut() {
            wipe(key);
        }
        if let Some((key, _)) = self.pmf.bigtk.as_mut() {
            wipe(key);
        }
        self.supplicant = None;
    }
}

impl LinkStation {
    /// Whether any key of the link is still held here: pairwise, group,
    /// integrity group, beacon integrity, or the supplicant that has them.
    pub fn holds_keys(&self) -> bool {
        !matches!(self.ccmp, Ccmp::Hardware)
            || self.group_keys.iter().any(Option::is_some)
            || self.pmf.igtk.is_some()
            || self.pmf.bigtk.is_some()
            || self.supplicant.is_some()
    }
}

impl Drop for LinkStation {
    fn drop(&mut self) {
        self.wipe_keys();
    }
}
