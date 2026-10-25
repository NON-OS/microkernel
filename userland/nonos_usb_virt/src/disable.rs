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
//! What OP_DISABLE_SLOT means for a slot this table knows. Two holders
//! give a slot back: driver.usb_hub0, when the child leaves its hub port
//! (or its hub leaves), and a claimant, when it lets the child go.

use crate::table::Virt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disable {
    /// Not a slot of this table's: disabled as any slot is.
    Plain,
    /// The child left, unclaimed: its slot is disabled and its port gone.
    Detach(u8),
    /// The child left while claimed: nothing is disabled yet; the port
    /// reads disconnected until the claimant gives its slot back.
    Detached,
    /// A claimant let go of a child still there: only the claimant's own
    /// slot goes, and the child is parked unclaimed again.
    Release,
    /// A claimant let go of a child that left: its own slot and the
    /// child's slot both go, with the port.
    ReleaseGone(u8),
}

impl Virt {
    pub fn disable(&mut self, slot: u8) -> Disable {
        for e in self.list.iter_mut() {
            let Some(p) = e.as_mut() else { continue };
            if p.claimant == Some(slot) {
                p.claimant = None;
                if p.connected {
                    return Disable::Release;
                }
                let gone = p.slot;
                *e = None;
                return Disable::ReleaseGone(gone);
            }
            if p.slot == slot {
                if p.claimant.is_none() {
                    *e = None;
                    return Disable::Detach(slot);
                }
                (p.connected, p.changed) = (false, true);
                return Disable::Detached;
            }
        }
        Disable::Plain
    }
}
