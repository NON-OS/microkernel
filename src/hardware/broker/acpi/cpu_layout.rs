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

//! One processor's performance and idle tables as the ACPI capsule hands
//! them over (ACPI 6.5, 8.4): what MkAcpiCpu carries, and what the kernel's
//! frequency and idle control may read. Little-endian throughout:
//!
//! - header, 24 bytes: version, P-state count, C-state count, flags
//!   (`HAS_PCT`, `HAS_PPC`, `HAS_CPC`), _LPI level count, three zero
//!   bytes, the processor's UID (the MADT's ACPI processor id), _PPC;
//! - _PCT's control and status registers, when flagged;
//! - each _PSS state, six u32: MHz, mW, latency, bus-master latency,
//!   control, status;
//! - each _CST state: its register, then type, latency and mW as u32;
//! - _CPC, when flagged: its revision, seven zero bytes, 21 entries;
//! - each _LPI level, the processor's own first: container _UID (0 for
//!   the processor's own), level id, kind (`LPI_OWN`, `LPI_CONTAINER`),
//!   state count, six zero bytes; then each state: minimum residency,
//!   wake latency, flags and parent state as u32, context lost and
//!   counter frequency as u64, and the entry, residency and usage
//!   registers.
//!
//! A register or _CPC entry is 16 bytes: kind, address space, bit width,
//! bit offset, access size, three zero bytes, then the address or integer.

pub const VERSION: u8 = 2;
pub const HEADER: usize = 24;
pub const REG: usize = 16;
pub const PSTATE: usize = 24;
pub const CSTATE: usize = REG + 12;
pub const CPC_FIELDS: usize = 21;
pub const CPC: usize = 8 + CPC_FIELDS * REG;
pub const MAX_PSTATES: usize = 32;
pub const MAX_CSTATES: usize = 8;
pub const LEVEL: usize = 24;
pub const LPI_STATE: usize = 32 + 3 * REG;
pub const MAX_LEVELS: usize = 4;
pub const MAX_LPI_STATES: usize = 8;
pub const LPI: usize = MAX_LEVELS * (LEVEL + MAX_LPI_STATES * LPI_STATE);
pub const MAX_RECORD: usize =
    HEADER + 2 * REG + MAX_PSTATES * PSTATE + MAX_CSTATES * CSTATE + CPC + LPI;

pub const HAS_PCT: u8 = 1;
pub const HAS_PPC: u8 = 2;
pub const HAS_CPC: u8 = 4;

pub const ABSENT: u8 = 0;
pub const REGISTER: u8 = 1;
pub const INTEGER: u8 = 2;

pub const LPI_OWN: u8 = 0;
pub const LPI_CONTAINER: u8 = 1;
