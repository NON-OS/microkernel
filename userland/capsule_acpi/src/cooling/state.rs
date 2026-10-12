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
use nonos_aml::thermal::{Trips, Verdict};

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
    pub verdict: Verdict,
    /// m°C; None when _TMP failed or was not believed.
    pub temp: Option<i32>,
    pub next_ms: u64,
    /// When _CRT was first reached, waiting on a second reading.
    pub critical_at: Option<u64>,
    /// _CRT reached on two readings a second apart.
    pub critical: bool,
}

#[derive(Default)]
pub struct Cooling {
    pub zones: Vec<Zone>,
    pub fans: Vec<Fan>,
    pub pool: PowerPool,
    /// The thermal word last reported, and when.
    pub reported: Option<u64>,
    pub reported_ms: u64,
}
