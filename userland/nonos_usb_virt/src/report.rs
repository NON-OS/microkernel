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
//! Virtual ports in PORT_STATUS. The reply's head is `ports u8, virtual
//! u8, 0, 0`; `ports` 8-byte entries follow, root ports first, then the
//! virtual ones, each `port u8, owner u8, kind u8 (0 root, 1 virtual), 0,
//! PORTSC u32 le` in xHCI's form (connected bit 0, enabled bit 1, speed
//! bits 13:10, connect change bit 17); then `virtual` 8-byte extensions,
//! `port u8, parent slot u8, TT hub slot u8, TT port u8, route u32 le`.
//! A parser that knows only root ports reads the virtual ones as ports.

use crate::table::{Parked, VPORT_BASE};

const CONNECTED: u32 = 1 << 0;
const ENABLED: u32 = 1 << 1;
const SPEED_SHIFT: u32 = 10;
const CONNECT_CHANGE: u32 = 1 << 17;
const VIRTUAL: u8 = 1;

/// `vport`'s entry: free (owner 0) until claimed, then the parked slot's
/// owner byte as the host keeps it (`held`).
pub fn port_entry(vport: u8, p: &Parked, held: u8) -> [u8; 8] {
    let mut sc = (p.speed as u32 & 0xF) << SPEED_SHIFT;
    if p.connected {
        sc |= CONNECTED | ENABLED;
    }
    if p.changed {
        sc |= CONNECT_CHANGE;
    }
    let owner = if p.claimant.is_some() { held.max(1) } else { 0 };
    let [a, b, c, d] = sc.to_le_bytes();
    [vport, owner, VIRTUAL, 0, a, b, c, d]
}

/// `vport`'s extension: where the child sits.
pub fn ext(vport: u8, p: &Parked) -> [u8; 8] {
    let [r0, r1, r2, r3] = p.route.to_le_bytes();
    [vport, p.parent, p.tt.0, p.tt.1, r0, r1, r2, r3]
}

impl crate::table::Virt {
    /// Every entry, with its virtual port.
    pub fn entries(&self) -> impl Iterator<Item = (u8, &Parked)> {
        let all = self.list.iter().enumerate();
        all.filter_map(|(i, p)| p.as_ref().map(|p| (VPORT_BASE + i as u8, p)))
    }

    /// Every connection change reported once: cleared after a listing.
    pub fn reported(&mut self) {
        self.list.iter_mut().flatten().for_each(|p| p.changed = false);
    }
}
