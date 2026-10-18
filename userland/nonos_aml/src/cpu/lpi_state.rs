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

//! One _LPI state package: minimum residency, worst-case wake latency,
//! flags, architectural context lost, residency counter frequency, enabled
//! parent state, entry method, residency counter, usage counter, name
//! (ACPI 6.5, 8.4.4.3; order recalled, unverified).

use super::lpi_types::{LpiEntry, LpiState};
use super::register::{register, Register};
use crate::object::Object;
use alloc::vec::Vec;

pub const MAX_NAME: usize = 32;
const FIELDS: usize = 10;

/// A counter firmware does not implement is a null register; an integer
/// 0 is read the same way.
fn counter(o: &Object) -> Option<Option<Register>> {
    match o {
        Object::Integer(0) => Some(None),
        _ => register(o).map(|r| (!r.is_null()).then_some(r)),
    }
}

fn entry(o: &Object) -> Option<LpiEntry> {
    match o {
        Object::Integer(v) => Some(LpiEntry::Integer(*v)),
        _ => register(o).filter(|r| !r.is_null()).map(LpiEntry::Register),
    }
}

pub fn state(o: &Object) -> Option<LpiState> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    if e.len() < FIELDS {
        return None;
    }
    let int = |i: usize| match &e[i] {
        Object::Integer(v) => Some(*v),
        _ => None,
    };
    let small = |i: usize| int(i).and_then(|v| u32::try_from(v).ok());
    let name: Vec<u8> = match &e[9] {
        Object::String(s) => s.borrow().iter().copied().take(MAX_NAME).collect(),
        _ => Vec::new(),
    };
    Some(LpiState {
        min_residency_us: small(0)?,
        wake_latency_us: small(1)?,
        flags: small(2)?,
        context_lost: int(3)?,
        counter_hz: int(4)?,
        parent_state: small(5)?,
        entry: entry(&e[6])?,
        residency: counter(&e[7])?,
        usage: counter(&e[8])?,
        name,
    })
}
