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

//! The slot table. Slot `s` is device address `s`; its bulk EDs are
//! `2(s-1)` (IN) and the one after (OUT). held.rs says who holds a port.

use crate::sched::layout::INTR_MAX;
use crate::xfer::bulk::BulkEp;
use crate::xfer::interrupt::IntrEp;

/// Slots, so addresses, 1 to DEVS (bulk EDs: two each).
pub const DEVS: usize = 16;

#[derive(Clone, Copy, Default)]
pub struct Dev {
    pub port: u8, // 0 until addressed
    pub low_speed: bool,
    pub mps0: u16,
    /// A class driver has configured an endpoint on it.
    pub claimed: bool,
    pub bulk: Option<(BulkEp, BulkEp)>,
    /// The polled bulk IN is armed: the place it holds (place.rs) and the
    /// bytes it asked for.
    pub poll: Option<(usize, usize)>,
    /// Its route below the root port (0: on it), and whether it is a hub.
    pub route: u32,
    pub hub: bool,
}

#[derive(Default)]
pub struct Table {
    pub devs: [Option<Dev>; DEVS],
    /// The interrupt endpoints armed, by ED, with their slots.
    pub intr: [Option<(u8, IntrEp)>; INTR_MAX],
    /// The root hub's port count, and the control ED's resting place.
    pub ports: u8,
    pub ctrl_at: u8,
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
