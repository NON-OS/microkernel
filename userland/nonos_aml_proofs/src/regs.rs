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

//! A model of the event registers: plain ports, write-one-to-clear status
//! ports, ports the kernel refuses, and firmware that sets SCI_EN some
//! sleeps after ACPI_ENABLE reaches SMI_CMD.

use std::collections::{BTreeMap, BTreeSet};

pub use crate::regs_shared::Shared;

#[derive(Default)]
pub struct Regs {
    pub ports: BTreeMap<u16, u16>,
    pub w1c: BTreeSet<u16>,
    pub refused: BTreeSet<u16>,
    pub writes: Vec<(u16, u16)>,
    pub slept_ms: u64,
    /// (SMI_CMD port, ACPI_ENABLE value, PM1 control port, sleeps to answer).
    pub smi: Option<(u16, u16, u16, u32)>,
    pub armed_smi: Option<u32>,
    /// The clock the storm bounds read, in milliseconds.
    pub now: u64,
}

impl Regs {
    pub fn get(&self, port: u16) -> u16 {
        self.ports.get(&port).copied().unwrap_or(0)
    }

    pub fn set(&mut self, port: u16, v: u16) {
        self.ports.insert(port, v);
    }
}
