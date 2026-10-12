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
//! A class driver's claim on a virtual port: ADDRESS_DEVICE naming it,
//! with a slot the claimant enabled. First claim wins.

use crate::table::{Parked, Virt, VPORT_BASE};

impl Virt {
    /// A claim on `vport` by `alias`, a slot the claimant enabled: the
    /// entry, or None when it is gone, claimed already, or `alias` is in
    /// use here.
    pub fn claim(&mut self, vport: u8, alias: u8) -> Option<Parked> {
        if self.knows(alias) {
            return None;
        }
        let i = vport.checked_sub(VPORT_BASE)? as usize;
        let p = self.list.get_mut(i)?.as_mut()?;
        if !p.connected || p.claimant.is_some() {
            return None;
        }
        p.claimant = Some(alias);
        Some(*p)
    }
}
