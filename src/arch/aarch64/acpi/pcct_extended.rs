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

//! PCCT's extended subspaces (offsets recalled from ACPI 6.5, 14.1.6 and
//! 14.1.7, unverified). Types 3 and 4, the master and slave channels (164
//! bytes): interrupt u32 and flags u8 at 2, base u64 at 8, length u32 at
//! 16, the doorbell at 20, latency, rate and turnaround u32 at 48, 52 and
//! 56, the acknowledge register at 60, the command-complete check GAS at
//! 96 with its mask, the command-complete update at 116, the error status
//! GAS at 144 with its mask. Type 5, register based (96 bytes): version
//! u16 at 2, base u64 at 4, length u64 at 12, the doorbell at 20, the
//! command-complete check at 48, the error status at 68 (each GAS with
//! one mask), latency u32 at 88 and turnaround u32 at 92.

use super::le::{u32_at, u64_at, u8_at};
use super::pcct_types::{reg, Reg, Subspace};
use super::spcr_tail::gas;

/// A GAS and the one mask that goes with it.
fn masked(s: &[u8], at: usize) -> Option<Reg> {
    Some(Reg { gas: gas(s, at)?, preserve: u64_at(s, at + 12)?, write: 0 })
}

pub(super) fn extended(s: &[u8], kind: u8) -> Option<Subspace> {
    if kind == 5 {
        let s = s.get(..96)?;
        return Some(Subspace {
            kind,
            base: u64_at(s, 4)?,
            length: u64_at(s, 12)?,
            doorbell: reg(s, 20),
            complete: masked(s, 48),
            error: masked(s, 68),
            latency_us: u32_at(s, 88)?,
            min_turnaround_us: u32_at(s, 92)?,
            ..Subspace::default()
        });
    }
    let s = s.get(..164)?;
    Some(Subspace {
        kind,
        base: u64_at(s, 8)?,
        length: u64::from(u32_at(s, 16)?),
        doorbell: reg(s, 20),
        interrupt: Some((u32_at(s, 2)?, u8_at(s, 6)?)),
        ack: reg(s, 60),
        complete: masked(s, 96),
        error: masked(s, 144),
        latency_us: u32_at(s, 48)?,
        max_rate: u32_at(s, 52)?,
        min_turnaround_us: u32_at(s, 56)?,
    })
}
