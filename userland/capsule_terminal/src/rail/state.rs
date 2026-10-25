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

use super::metrics::Sample;
use super::ring::SparkRing;

#[path = "state_tick.rs"]
mod state_tick;

/// Where the store-usage query starts its cycle, phased against the lease
/// query so the two never share a tick.
const DISK_PHASE: u32 = 4;

#[derive(Clone, Copy)]
pub struct Rail {
    pub sample: Sample,
    pub spark: SparkRing,
    ticks: u32,
    net_polls: u32,
    disk_polls: u32,
    warm: bool,
}

impl Rail {
    pub const fn new() -> Self {
        Rail {
            sample: Sample::EMPTY,
            spark: SparkRing::new(),
            ticks: 0,
            net_polls: 0,
            disk_polls: DISK_PHASE,
            warm: false,
        }
    }
}
