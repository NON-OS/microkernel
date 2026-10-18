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

//! Processor throttling (ACPI 6.5, 8.4.6; recalled, unverified): _TSS,
//! the throttling states, fastest first, each a percentage of full speed
//! with its power, transition latency, and the control and status values
//! for _PTC's registers; _PTC, those registers, shaped as _PCT is; _TPC,
//! how many of the fastest states the platform forbids just now.

use crate::object::Object;
use alloc::vec::Vec;

/// More than any processor lists; a longer package stops here.
pub const MAX_TSTATES: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TState {
    /// Of full speed, 1 to 100.
    pub percent: u32,
    pub milliwatts: u32,
    pub latency_us: u32,
    pub control: u32,
    pub status: u32,
}

pub(super) fn words<const N: usize>(o: &Object) -> Option<[u32; N]> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    let mut out = [0u32; N];
    for (i, w) in out.iter_mut().enumerate() {
        let Some(Object::Integer(v)) = e.get(i) else { return None };
        *w = u32::try_from(*v).ok()?;
    }
    Some(out)
}

/// Every state up to the first that is not five integers, a percentage
/// from 1 to 100, and no faster than the one before it: _TPC counts by
/// position, so a state cannot be dropped from the middle.
pub fn parse_tss(o: &Object) -> Vec<TState> {
    let Object::Package(p) = o else { return Vec::new() };
    let mut last = 100;
    let state = |o: &Object| {
        let [percent, milliwatts, latency_us, control, status] = words::<5>(o)?;
        let ok = (1..=last).contains(&percent);
        last = percent;
        ok.then_some(TState { percent, milliwatts, latency_us, control, status })
    };
    p.borrow().iter().take(MAX_TSTATES).map_while(state).collect()
}
