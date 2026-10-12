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
//! The controllers served, and where each host slot is.

use super::hc::Hc;
use crate::devices::table::DEVS;
use nonos_usb_virt::Virt;

/// Controllers served at most: an ICH's six UHCI, with room.
pub const MAX_HC: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bind {
    Free,
    /// Enabled, not yet addressed on any controller.
    Enabled,
    /// Addressed on controller `k`.
    On(usize),
}

pub struct Front<H: Hc> {
    pub hcs: [Option<H>; MAX_HC],
    pub bound: [Bind; DEVS],
    /// The children below hubs, parked or claimed (nonos_usb_virt).
    pub virt: Virt,
}

impl<H: Hc> Front<H> {
    pub fn new() -> Self {
        Front {
            hcs: core::array::from_fn(|_| None),
            bound: [Bind::Free; DEVS],
            virt: Virt::default(),
        }
    }

    /// The first free place for a controller, taken by `hc`.
    pub fn add(&mut self, hc: H) -> bool {
        let Some(place) = self.hcs.iter_mut().find(|h| h.is_none()) else { return false };
        *place = Some(hc);
        true
    }
}

impl<H: Hc> Default for Front<H> {
    fn default() -> Self {
        Self::new()
    }
}
