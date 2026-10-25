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

//! _CST (ACPI 6.5, 8.4.1.1): the idle states, a count and then one package
//! per state: the register that enters it, its type (C1 to C3), its exit
//! latency and its power.

use super::register::{register, Register};
use crate::object::Object;
use alloc::vec::Vec;

/// More than any processor lists; _CST's count could claim anything.
pub const MAX_CSTATES: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CState {
    pub register: Register,
    /// 1, 2 or 3: what the state costs the caches and bus masters.
    pub kind: u8,
    pub latency_us: u32,
    pub milliwatts: u32,
}

fn cstate(o: &Object) -> Option<CState> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    let int = |i: usize| match e.get(i) {
        Some(Object::Integer(v)) => u32::try_from(*v).ok(),
        _ => None,
    };
    let kind = u8::try_from(int(1)?).ok().filter(|k| (1..=3).contains(k))?;
    Some(CState { register: register(e.first()?)?, kind, latency_us: int(2)?, milliwatts: int(3)? })
}

/// The states, as many as the count says and the package holds. One that
/// is malformed or of no known type is left out: OSPM picks idle states by
/// their latency, not their position.
pub fn parse_cst(o: &Object) -> Vec<CState> {
    let Object::Package(p) = o else { return Vec::new() };
    let e = p.borrow();
    let count = match e.first() {
        Some(Object::Integer(n)) => usize::try_from(*n).unwrap_or(usize::MAX),
        _ => return Vec::new(),
    };
    e.iter().skip(1).take(count.min(MAX_CSTATES)).filter_map(cstate).collect()
}
