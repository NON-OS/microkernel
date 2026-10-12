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

//! The tables MkAcpiCpu carries, decoded; `cpu_layout` says how.

use super::cpu_layout::CPC_FIELDS;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reg {
    pub kind: u8,
    pub space: u8,
    pub width: u8,
    pub offset: u8,
    pub access: u8,
    /// The register's address, or the entry's integer.
    pub value: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PState {
    pub mhz: u32,
    pub milliwatts: u32,
    pub latency_us: u32,
    pub bus_master_latency_us: u32,
    pub control: u32,
    pub status: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CState {
    pub reg: Reg,
    pub kind: u8,
    pub latency_us: u32,
    pub milliwatts: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CpuTables {
    pub uid: u64,
    pub ppc: Option<u64>,
    pub pct: Option<[Reg; 2]>,
    pub pstates: Vec<PState>,
    pub cstates: Vec<CState>,
    pub cpc: Option<(u8, [Reg; CPC_FIELDS])>,
}
