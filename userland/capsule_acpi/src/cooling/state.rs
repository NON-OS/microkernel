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

use alloc::vec::Vec;
use nonos_aml::device_power::PowerPool;
use nonos_aml::namespace::Id;
use nonos_aml::thermal::{Trips, Watch};

pub use super::passive::Passive;

/// How a fan is run: an ACPI 4.0 fan through _FSL, with the levels for on
/// and off; an older one by its device power state.
#[derive(Clone, Copy)]
pub enum Drive {
    Level { on: u64, off: u64 },
    Power,
}

pub struct Fan {
    pub id: Id,
    pub drive: Drive,
    /// None until the capsule has set it: firmware may have left it either way.
    pub on: Option<bool>,
}

pub struct Zone {
    pub id: Id,
    pub trips: Trips,
    /// For _AL0 onward, the fans (indexes into `Cooling::fans`) it runs.
    pub levels: Vec<Vec<usize>>,
    /// Its readings: the last temperature believed, the verdict, _CRT's watch.
    pub w: Watch,
    pub next_ms: u64,
    pub passive: Option<Passive>,
    /// The trip faults, and an unread _TMP, already said (`trip_faults`).
    pub told: u16,
}

#[derive(Default)]
pub struct Cooling {
    pub zones: Vec<Zone>,
    pub fans: Vec<Fan>,
    /// Every power resource's count, the fans' and the claimed devices'.
    pub pool: PowerPool,
    /// The thermal word last reported, and when.
    pub reported: Option<u64>,
    pub reported_ms: u64,
    /// The ceilings the kernel holds, by processor UID.
    pub sent: Vec<(u64, u8)>,
}
