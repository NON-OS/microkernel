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

//! Who holds a root port: the device on it, not one below a hub there.

use super::table::{Dev, Table};

impl Table {
    /// The slot on root port `port` itself (a hub's, when one is there).
    pub fn holder(&self, port: u8) -> Option<u8> {
        let held = |d: &Option<Dev>| d.is_some_and(|d| d.port == port && d.route == 0);
        self.devs.iter().position(held).map(|i| i as u8 + 1)
    }

    /// The wire's owner byte for `port`: 0 free, 1 addressed, 2 claimed.
    pub fn owner(&self, port: u8) -> u8 {
        match self.holder(port).and_then(|s| self.devs[s as usize - 1]) {
            None => 0,
            Some(d) if d.claimed => 2,
            Some(_) => 1,
        }
    }

    /// Whether a hub is addressed on root port `port`.
    pub fn hub_on(&self, port: u8) -> bool {
        self.devs.iter().flatten().any(|d| d.port == port && d.hub)
    }
}
