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

//! The station's own address, changed only between associations.

use super::LinkStation;
use crate::dot11::header::MacAddr;

impl LinkStation {
    /// Take `mac` as the address the next association runs under. Refused
    /// (false, nothing changed) while associated: a session keeps the
    /// address it began with.
    pub fn readdress(&mut self, mac: MacAddr) -> bool {
        if self.associated {
            return false;
        }
        self.our_mac = mac;
        true
    }
}
