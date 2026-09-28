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

//! socketpair: two sockets connected from the start.

use super::table::Socks;
use super::types::{Domain, Proto};

impl Socks {
    /// Two connected sockets, both held by `pid`.
    pub fn pair(&mut self, domain: Domain, proto: Proto, pid: u32) -> (u32, u32) {
        let a = self.open(domain, proto, Some(pid));
        let b = self.open(domain, proto, Some(pid));
        for (me, other) in [(a, b), (b, a)] {
            if let Some(s) = self.get_mut(me) {
                s.peer = Some(other);
                s.connected = true;
            }
        }
        (a, b)
    }
}
