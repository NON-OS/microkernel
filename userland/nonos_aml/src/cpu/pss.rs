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

//! _PSS (ACPI 6.5, 8.4.5.2): the performance states, fastest first, and
//! _PPC (8.4.5.3), how many of the fastest the platform forbids just now.

use crate::object::Object;
use alloc::vec::Vec;

/// More than any processor lists; a longer package stops here.
pub const MAX_PSTATES: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PState {
    pub mhz: u32,
    pub milliwatts: u32,
    /// Microseconds the processor is unavailable while it changes state.
    pub latency_us: u32,
    pub bus_master_latency_us: u32,
    /// What to write to _PCT's control register, and what its status
    /// register reads once the state is reached.
    pub control: u32,
    pub status: u32,
}

fn word(e: &[Object], i: usize) -> Option<u32> {
    match e.get(i)? {
        Object::Integer(v) => u32::try_from(*v).ok(),
        _ => None,
    }
}

fn pstate(o: &Object) -> Option<PState> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    Some(PState {
        mhz: word(&e, 0)?,
        milliwatts: word(&e, 1)?,
        latency_us: word(&e, 2)?,
        bus_master_latency_us: word(&e, 3)?,
        control: word(&e, 4)?,
        status: word(&e, 5)?,
    })
}

/// Every state up to the first that is not six integers: _PPC counts by
/// position, so a state cannot be dropped from the middle.
pub fn parse_pss(o: &Object) -> Vec<PState> {
    let Object::Package(p) = o else { return Vec::new() };
    let e = p.borrow();
    e.iter().take(MAX_PSTATES).map_while(pstate).collect()
}

/// _PPC's value as an index the states can take: one past the slowest
/// would leave none, so it is held to the slowest.
pub fn ppc_index(ppc: u64, states: usize) -> usize {
    let last = states.saturating_sub(1);
    usize::try_from(ppc).map_or(last, |i| i.min(last))
}
