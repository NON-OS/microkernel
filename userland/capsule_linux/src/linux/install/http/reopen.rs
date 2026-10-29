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

//! A connection opened, or opened once more once the network's services
//! have had time to come back. init starts a service that ended again after
//! SETTLE_MS (supervisor/watch_rule.rs); net.core ended once mid-install on
//! KVM, and the install's next GET, refused within that second, failed the
//! install. One more try past the restart, and no more: a network that is
//! gone still fails the install as it did.

use nonos_libc::mk_idle_ms;

use super::conn::Conn;

/// Past init's SETTLE_MS and the restarted service's own start.
pub const BACK_MS: u64 = 3_000;

impl Conn {
    pub fn open_or_again(at: &str, port: u16) -> Result<Conn, &'static str> {
        Conn::open(at, port).or_else(|_| {
            let _ = mk_idle_ms(BACK_MS);
            Conn::open(at, port)
        })
    }
}
