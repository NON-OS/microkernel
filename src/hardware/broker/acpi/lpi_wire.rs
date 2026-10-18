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

//! Decoding the idle levels at the end of a MkAcpiCpu record. Level 0 is
//! the processor's own and names no container; each one above names one.
//! A state's parent must be a state the next level up has.

use super::cpu_layout::{ABSENT, INTEGER, LPI_CONTAINER, LPI_OWN, MAX_LPI_STATES, REGISTER};
use super::cpu_record::Reg;
use super::cpu_take::Take;
use super::lpi_record::{LpiLevel, LpiState};
use alloc::vec::Vec;

fn kind_in(r: Reg, kinds: &[u8]) -> Option<Reg> {
    kinds.contains(&r.kind).then_some(r)
}

fn state(t: &mut Take<'_>) -> Option<LpiState> {
    Some(LpiState {
        min_residency_us: t.u32()?,
        wake_latency_us: t.u32()?,
        flags: t.u32()?,
        parent_state: t.u32()?,
        context_lost: t.u64()?,
        counter_hz: t.u64()?,
        entry: kind_in(t.reg()?, &[REGISTER, INTEGER])?,
        residency: kind_in(t.reg()?, &[ABSENT, REGISTER])?,
        usage: kind_in(t.reg()?, &[ABSENT, REGISTER])?,
    })
}

fn level(t: &mut Take<'_>, first: bool) -> Option<LpiLevel> {
    let (uid, level_id) = (t.u64()?, t.u64()?);
    let [kind, n] = t.bytes::<2>()?;
    t.zeros::<6>()?;
    let container = match (first, kind) {
        (true, LPI_OWN) if uid == 0 => None,
        (false, LPI_CONTAINER) => Some(uid),
        _ => return None,
    };
    if n as usize > MAX_LPI_STATES {
        return None;
    }
    let states = (0..n).map(|_| state(t)).collect::<Option<Vec<_>>>()?;
    Some(LpiLevel { container, level_id, states })
}

/// `n` levels, each state's parent within the level above; with no level
/// above, a parent named is one firmware's tables did not let the capsule
/// read, and the processor may still enter the state alone.
pub fn levels(t: &mut Take<'_>, n: u8) -> Option<Vec<LpiLevel>> {
    let out = (0..n).map(|i| level(t, i == 0)).collect::<Option<Vec<_>>>()?;
    let fits = out
        .windows(2)
        .all(|w| w[0].states.iter().all(|s| s.parent_state as usize <= w[1].states.len()));
    fits.then_some(out)
}
