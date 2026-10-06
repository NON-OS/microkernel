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

//! The host's state between commands, and what it may be asked to do.

use super::super::super::env::{pause, Clock, DmaBuf, Log, Mmio};
use super::super::super::text::Line;
use super::super::caps::Caps;
use super::super::power::Vdd;

pub struct Host<M: Mmio, C: Clock, L: Log> {
    pub io: M,
    pub clock: C,
    pub log: L,
    /// The page the ADMA2 descriptor table is written to.
    pub table: DmaBuf,
    /// A known Intel eMMC host (`pci::Kind::IntelEmmc`): its slot is wired
    /// eight bits wide and high speed whatever the capabilities say, as
    /// Linux's byt_emmc_probe_slot and glk_emmc_probe_slot set.
    pub intel_emmc: bool,
    pub caps: Caps,
    pub vdd: Option<Vdd>,
    pub sd_hz: u32,
    pub width: u8,
    pub hs: bool,
}

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    pub fn new(io: M, clock: C, log: L, table: DmaBuf, intel_emmc: bool) -> Self {
        Self {
            io,
            clock,
            log,
            table,
            intel_emmc,
            caps: Caps { caps: 0, caps1: 0, version: 0 },
            vdd: None,
            sd_hz: 0,
            width: 1,
            hs: false,
        }
    }

    pub fn say(&self, l: &Line) {
        self.log.line(l.as_bytes());
    }

    pub fn pause(&self, ms: u64) {
        pause(&self.clock, ms);
    }
}
