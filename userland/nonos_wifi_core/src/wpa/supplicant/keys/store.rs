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

//! Keep the group keys a message delivered in the supplicant.

use super::*;

impl Supplicant {
    /// Store the group keys a message delivered. Returns whether the GTK is a
    /// different key from the one already held (a repeat is not reinstalled).
    pub(in crate::wpa::supplicant) fn store_group_keys(&mut self, k: &GroupKeys) -> bool {
        let len = self.group.tk_len();
        let changed =
            self.gtk_len != len || self.gtk[..len] != k.gtk[..len] || self.gtk_id != k.gtk_id;
        self.gtk[..len].copy_from_slice(&k.gtk[..len]);
        self.gtk_len = len;
        self.gtk_id = k.gtk_id;
        self.gtk_rsc = k.rsc;
        if let Some((key, id, ipn)) = k.igtk {
            self.igtk = key;
            self.igtk_id = id;
            self.igtk_ipn = ipn;
            self.igtk_set = true;
        }
        if k.bigtk.is_some() {
            self.bigtk = k.bigtk;
        }
        changed
    }
}
