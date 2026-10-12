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
//! What the hub class needs of a USB host controller driver and of the
//! The parked children and their claims.

/// Virtual ports are numbered from here, clear of any root port's number.
pub const VPORT_BASE: u8 = 0x80;
/// Children listed at most, across every hub of a host.
pub const VPORTS: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parked {
    /// The slot driver.usb_hub0 addressed the child in.
    pub slot: u8,
    /// The hub's slot and the port the child is on.
    pub parent: u8,
    pub parent_port: u8,
    /// The host wire's speed id, route string, TT (0, 0 for none) and
    /// EP0's packet size, as the routed address gave them.
    pub speed: u8,
    pub route: u32,
    pub tt: (u8, u8),
    pub mps0: u16,
    pub connected: bool,
    /// A connection change not yet reported.
    pub changed: bool,
    /// The claimant's slot, standing for `slot`.
    pub claimant: Option<u8>,
}

pub fn is_virtual(port: u8) -> bool {
    port >= VPORT_BASE
}

#[derive(Default)]
pub struct Virt {
    pub(crate) list: [Option<Parked>; VPORTS],
}

impl Virt {
    /// The entry at virtual port `vport`.
    pub fn at(&self, vport: u8) -> Option<&Parked> {
        let i = vport.checked_sub(VPORT_BASE)? as usize;
        self.list.get(i)?.as_ref()
    }

    /// The slot a request for `slot` goes to: the parked one for a
    /// claimant's alias, else `slot` itself.
    pub fn real(&self, slot: u8) -> u8 {
        let mut all = self.list.iter().flatten();
        all.find(|p| p.claimant == Some(slot)).map_or(slot, |p| p.slot)
    }

    /// Whether `slot` is parked or stands for a parked one.
    pub fn knows(&self, slot: u8) -> bool {
        self.list.iter().flatten().any(|p| p.slot == slot || p.claimant == Some(slot))
    }
}
