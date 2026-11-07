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

//! The newest role table and revocation list this boot has served under.
//! The records are read again for each connection, so a table or list put
//! back over a newer one is refused, as the role store refuses one older
//! than it holds: an older table may grant a role since taken away, and an
//! older list may leave out a grant since revoked. The marks last as long
//! as nsh.front runs; across a reboot, the list's freshness bound (the
//! role table's revocation_max_age_ms) is what keeps an old list out.
//! Pure, so nsh_front_proofs holds it.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Marks {
    serial: u64,
    issued_ms: u64,
}

impl Marks {
    pub const fn new() -> Marks {
        Marks { serial: 0, issued_ms: 0 }
    }

    /// The lowest table serial a connection may be served under.
    pub fn serial(&self) -> u64 {
        self.serial
    }

    /// Whether a table of `serial` with a list issued at `issued_ms` may be
    /// served under; when it may, it becomes the newest.
    pub fn admit(&mut self, serial: u64, issued_ms: u64) -> bool {
        if serial < self.serial || issued_ms < self.issued_ms {
            return false;
        }
        self.serial = serial;
        self.issued_ms = issued_ms;
        true
    }
}
