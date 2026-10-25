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

//! A processor's low-power idle levels as MkAcpiCpu carries them (ACPI 6.5,
//! 8.4.4): its own _LPI first, then each processor container's above it,
//! nearest first. `cpu_layout` says the bytes.

use super::cpu_record::Reg;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LpiState {
    pub min_residency_us: u32,
    pub wake_latency_us: u32,
    /// Bit 0: firmware has enabled the state. A disabled one keeps its
    /// place, since the level below names states by position.
    pub flags: u32,
    /// The state of the next level up this one allows, counted from 1; 0
    /// for none. Never past that level's states.
    pub parent_state: u32,
    pub context_lost: u64,
    pub counter_hz: u64,
    /// A register to write, or an integer combined with the levels below.
    pub entry: Reg,
    /// ABSENT when firmware has no such counter.
    pub residency: Reg,
    pub usage: Reg,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LpiLevel {
    /// The container's _UID, which every processor in it shares; None for
    /// the processor's own level.
    pub container: Option<u64>,
    pub level_id: u64,
    pub states: Vec<LpiState>,
}
