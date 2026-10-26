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

//! One PCCT subspace (ACPI 6.5, 14.1; offsets recalled, unverified). Types
//! 0 to 2 (62, 62 and 90 bytes): the shared memory's base u64 at 8 and
//! length u64 at 16, the doorbell GAS at 24 with its preserve and write
//! masks u64, the nominal latency and maximum periodic access rate u32 at
//! 52 and 56, the minimum turnaround u16 at 60; types 1 and 2 put their
//! interrupt u32 and flags u8 at 2, and type 2 an acknowledge GAS at 62
//! with its preserve and write masks. Types 3 and 4 (164) and 5 (96) are
//! `pcct_extended`.

use super::le::{u16_at, u32_at, u64_at, u8_at};
use super::pcct_extended::extended;
use super::pcct_types::{reg, Subspace};

/// None for a type this parser does not know or a structure too short.
pub(crate) fn subspace(s: &[u8]) -> Option<Subspace> {
    let kind = u8_at(s, 0)?;
    let need = match kind {
        0 | 1 => 62,
        2 => 90,
        3..=5 => return extended(s, kind),
        _ => return None,
    };
    let s = s.get(..need)?;
    Some(Subspace {
        kind,
        base: u64_at(s, 8)?,
        length: u64_at(s, 16)?,
        doorbell: reg(s, 24),
        interrupt: (kind > 0).then(|| Some((u32_at(s, 2)?, u8_at(s, 6)?))).flatten(),
        ack: if kind == 2 { reg(s, 62) } else { None },
        latency_us: u32_at(s, 52)?,
        max_rate: u32_at(s, 56)?,
        min_turnaround_us: u32::from(u16_at(s, 60)?),
        ..Subspace::default()
    })
}
