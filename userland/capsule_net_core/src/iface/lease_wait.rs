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

//! When to say that DHCP has not given the bound interface a lease.
//!
//! A Wi-Fi join that succeeds and a DHCP exchange that never ends left the
//! log with the join line and nothing after it: no sign that the stack was
//! still waiting for an address, which is the step the owner needs named.

/// How long the stack may be bound without a lease before it says so.
pub const NO_LEASE_SAY_MS: i64 = 15_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LeaseWait {
    port: u32,
    since_ms: i64,
    said: bool,
}

impl LeaseWait {
    pub const fn new() -> Self {
        Self { port: 0, since_ms: 0, said: false }
    }

    /// Whether to say now that no lease came: once, NO_LEASE_SAY_MS after the
    /// stack was bound to `port` or last held a lease. A lease dropped and not
    /// taken again is said again the same way.
    pub fn observe(&mut self, now_ms: i64, port: u32, leased: bool) -> bool {
        if port != self.port || leased {
            *self = Self { port, since_ms: now_ms, said: false };
            return false;
        }
        if port == 0 || self.said || now_ms - self.since_ms < NO_LEASE_SAY_MS {
            return false;
        }
        self.said = true;
        true
    }
}
