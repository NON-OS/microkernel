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

//! Processor performance and idle control, the AML half (ACPI 6.5, 8.4):
//! _PSS, _PCT, _PPC, _CPC, _CST and _LPI with the processor containers
//! above it, read for the kernel, which decides what to do with them.

mod cpc;
mod cpc_index;
mod container;
mod container_uid;
mod cst;
mod find;
mod lpi;
mod lpi_state;
mod lpi_types;
mod osc;
mod pct;
mod pss;
mod register;
mod tables;
mod tsd;
mod tss;

pub use cpc::{parse_cpc, Cpc, CpcField};
pub use cpc_index::*;
pub use container::{idle_levels, LpiLevel, CONTAINER_HID, MAX_CONTAINERS};
pub use cst::{parse_cst, CState, MAX_CSTATES};
pub use find::{processors, Processor, PROCESSOR_HID};
pub use lpi::{parse_lpi, LPI_ENABLED, MAX_LPI_STATES};
pub use lpi_state::MAX_NAME;
pub use lpi_types::{Lpi, LpiEntry, LpiState};
pub use osc::{processor_osc, ProcessorOsc, PROCESSOR_UUID};
pub use pct::{parse_pct, Pct};
pub use pss::{parse_pss, ppc_index, PState, MAX_PSTATES};
pub use register::{
    register, register_in, Register, FIXED_HARDWARE, PCC, SYSTEM_IO, SYSTEM_MEMORY,
};
pub use tables::{cpu_tables, ppc, CpuTables};
pub use tsd::{parse_tsd, Domain};
pub use tss::{parse_tss, TState, MAX_TSTATES};
