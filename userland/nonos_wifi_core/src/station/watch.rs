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

//! What the station keeps to tell a live link from a dead one: the time the
//! driver last gave it, the access point's liveness, a running SA Query, and
//! the replay floors of the management frames the access point protects.

use super::alive::Alive;
use super::query::Query;

/// The longest SSID an element holds.
pub const SSID_MAX: usize = 32;

pub(super) struct Watch {
    /// The driver's milliseconds at its last tick, once one came.
    pub(super) clock: Option<u64>,
    pub(super) alive: Alive,
    pub(super) query: Query,
    /// The IGTK index and the last IPN a group-addressed frame was taken with.
    pub(super) ipn: Option<(u16, u64)>,
    /// The last packet number of a protected individually addressed
    /// management frame: a counter of its own, apart from data's.
    pub(super) mgmt_pn: u64,
    /// The network's SSID, for the probes that ask after the access point.
    pub(super) ssid: [u8; SSID_MAX],
    pub(super) ssid_len: usize,
}

impl Watch {
    pub(super) const fn new() -> Self {
        Self {
            clock: None,
            alive: Alive::new(),
            query: Query::new(),
            ipn: None,
            mgmt_pn: 0,
            ssid: [0; SSID_MAX],
            ssid_len: 0,
        }
    }

    /// The access point was heard now, if the clock has started.
    pub(super) fn heard(&mut self) {
        if let Some(now) = self.clock {
            self.alive.heard(now);
        }
    }
}
