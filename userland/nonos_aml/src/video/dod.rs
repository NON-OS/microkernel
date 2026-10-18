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

//! _DOD (ACPI 6.5, B.4.2): the graphics adapter's outputs, as 32-bit
//! device IDs. Bit 31 says the ID follows the scheme of B.4.2's table;
//! the low bits hold the output's type and index, and match the _ADR of
//! the output device beneath the adapter.

use crate::object::Object;
use crate::power::fields::elements;
use alloc::vec::Vec;

pub fn parse_dod(o: &Object) -> Vec<u32> {
    let id = |e: &Object| match e {
        Object::Integer(v) => u32::try_from(*v).ok(),
        _ => None,
    };
    elements(o).map_or_else(Vec::new, |e| e.iter().filter_map(id).collect())
}
