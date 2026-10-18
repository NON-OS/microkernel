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

//! _BCL (ACPI 6.5, B.6.2): the brightness levels an output supports, in
//! percent. The first element is the level firmware suggests on AC power,
//! the second on battery; the rest are the levels themselves, in any
//! order. A level above 100 is not believed.

use crate::object::Object;
use crate::power::fields::elements;
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Levels {
    pub on_ac: Option<u8>,
    pub on_battery: Option<u8>,
    /// Ascending, each once; never empty.
    pub steps: Vec<u8>,
}

fn percent(o: &Object) -> Option<u8> {
    match o {
        Object::Integer(v) if *v <= 100 => Some(*v as u8),
        _ => None,
    }
}

pub fn parse_bcl(o: &Object) -> Option<Levels> {
    let e = elements(o)?;
    let mut steps: Vec<u8> = e.iter().skip(2).filter_map(percent).collect();
    steps.sort_unstable();
    steps.dedup();
    (!steps.is_empty()).then(|| Levels {
        on_ac: e.first().and_then(percent),
        on_battery: e.get(1).and_then(percent),
        steps,
    })
}
