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

//! The integrity group keys the handshakes delivered.

use super::*;

impl Supplicant {
    /// The integrity group key, its index and the AP's IGTK packet number,
    /// when management frame protection delivered one.
    pub fn igtk(&self) -> Option<(&[u8; 16], u16, [u8; 6])> {
        self.igtk_set.then_some((&self.igtk, self.igtk_id, self.igtk_ipn))
    }

    /// The BIGTK, its index and the BIPN it came with, once delivered.
    pub fn bigtk(&self) -> Option<(&[u8; 16], u16, [u8; 6])> {
        self.bigtk.as_ref().map(|(k, id, ipn)| (k, *id, *ipn))
    }
}
