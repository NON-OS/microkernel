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

//! The slot table. Slot `s` is device address `s`, so nothing maps between
//! the wire's slot ids and USB addresses; held.rs says who holds a port.

use crate::sched::layout::INTR_MAX;
use crate::xfer::bulk::BulkEp;
use crate::xfer::interrupt::IntrEp;

/// Slots, so addresses, 1 to DEVS.
pub const DEVS: usize = 16;

#[derive(Clone, Copy, Default)]
pub struct Dev {
    pub port: u8, // 0 until addressed
    pub low_speed: bool,
    pub mps0: u16,
    /// A class driver has configured an endpoint on it.
    pub claimed: bool,
    pub bulk: Option<(BulkEp, BulkEp)>,
    /// The polled bulk IN is armed: its chain's TDs.
    pub poll: Option<usize>,
    /// Its route below the root port (0: on it), and whether it is a hub.
    pub route: u32,
    pub hub: bool,
}

#[derive(Default)]
pub struct Table {
    pub devs: [Option<Dev>; DEVS],
    /// The interrupt endpoints armed, by queue: the slot and endpoint.
    pub intr: [Option<(u8, IntrEp)>; INTR_MAX],
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
