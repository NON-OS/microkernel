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

//! The slot table. Slot `s` is device address `s`, with its own control
//! QH and bulk QHs in the ring (sched/places.rs); held.rs says who holds
//! a root port.

use crate::sched::layout::{INTR_MAX, SLOTS};
use crate::sched::link::Link;
use crate::xfer::bulk::BulkEp;
use crate::xfer::interrupt::IntrEp;

#[derive(Clone, Copy, Default)]
pub struct Dev {
    pub port: u8, // 0 until addressed
    /// A class driver has configured an endpoint on it.
    pub claimed: bool,
    pub bulk: Option<(BulkEp, BulkEp)>,
    /// The polled bulk IN is armed: the place it holds (place.rs) and the
    /// bytes it asked for.
    pub poll: Option<(usize, usize)>,
    /// Its speed and TT, and its route below the root port (0: on it).
    pub link: Link,
    pub route: u32,
    /// Marked a hub by OP_CONFIGURE_HUB.
    pub hub: bool,
}

#[derive(Default)]
pub struct Table {
    pub devs: [Option<Dev>; SLOTS],
    /// The interrupt endpoints armed, by QH, with their slots.
    pub intr: [Option<(u8, IntrEp)>; INTR_MAX],
    /// The operational base, the port count, and which ports hold a
    /// high-speed device this host keeps (index: port less one).
    pub op: u32,
    pub ports: u8,
    pub high: [bool; 15],
}

impl Table {
    /// The lowest free slot, taken.
    pub fn enable(&mut self) -> Option<u8> {
        let i = self.devs.iter().position(|d| d.is_none())?;
        self.devs[i] = Some(Dev::default());
        Some(i as u8 + 1)
    }

    pub fn dev(&mut self, slot: u8) -> Option<&mut Dev> {
        self.devs.get_mut((slot as usize).checked_sub(1)?)?.as_mut()
    }
}
