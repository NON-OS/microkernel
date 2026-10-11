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

//! The data-path counters the status op reports, kept across sessions.

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkStats {
    pub tx_ok: u32,
    pub tx_drop: u32,
    /// Frames the radio delivered while associated.
    pub rx_ring: u32,
    /// Frames that reached the stack as Ethernet.
    pub rx_eth: u32,
    pub netif_reqs: u32,
    /// Frames the receive queue had no room for.
    pub rx_err: u32,
    /// Frames the station's checks refused.
    pub rx_refused: u32,
    pub rekeys: u32,
}

impl LinkStats {
    pub(crate) fn bump(c: &mut u32) {
        *c = c.wrapping_add(1);
    }
}
