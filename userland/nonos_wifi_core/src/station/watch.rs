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
//! driver last gave it, the access point's liveness, the SA Query the station
//! itself asked from `receive` (the reason of the deauthentication that
//! started it), and the network's SSID for the probes. The query and the
//! replay floors of protected management frames are management frame
//! protection's (`pmf_state`), one of each for both ways in.

use super::alive::Alive;

/// The longest SSID an element holds.
pub const SSID_MAX: usize = 32;

pub(super) struct Watch {
    /// The driver's milliseconds at its last tick, once one came.
    pub(super) clock: Option<u64>,
    pub(super) alive: Alive,
    /// The reason of the deauthentication the running query asks about.
    pub(super) asked: Option<u16>,
    /// A query begun before the clock ran, by its first identifier: it
    /// starts at the next tick.
    pub(super) unstamped: Option<[u8; 2]>,
    /// The next transaction identifier of the station's own queries.
    pub(super) next_txid: u16,
    /// The network's SSID, for the probes that ask after the access point.
    pub(super) ssid: [u8; SSID_MAX],
    pub(super) ssid_len: usize,
}

impl Watch {
    pub(super) const fn new() -> Self {
        Self {
            clock: None,
            alive: Alive::new(),
            asked: None,
            unstamped: None,
            next_txid: 1,
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

    /// The identifier the next request of the station's own takes.
    pub(super) fn txid(&self) -> [u8; 2] {
        self.next_txid.to_le_bytes()
    }

    /// That identifier was sent: the next is another.
    pub(super) fn took_txid(&mut self) {
        self.next_txid = self.next_txid.wrapping_add(1);
    }
}
