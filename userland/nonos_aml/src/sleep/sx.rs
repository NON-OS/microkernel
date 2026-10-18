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

//! \_Sx (ACPI 6.5, 7.4.2): the SLP_TYP values PM1a and PM1b control take
//! to enter sleeping state x, each three bits. The package holds the PM1a
//! value, the PM1b value, then reserved integers. Firmware written to
//! ACPI 1.0 sometimes gives one integer holding both, PM1b's in its second
//! byte; that form is read too.

use crate::object::Object;
use crate::power::fields::elements;

const SLP_TYP: u64 = 0x7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SleepTypes {
    pub a: u8,
    pub b: u8,
}

fn int(o: Option<&Object>) -> Option<u64> {
    match o? {
        Object::Integer(v) => Some(*v),
        _ => None,
    }
}

pub fn parse_sx(o: &Object) -> Option<SleepTypes> {
    let e = elements(o)?;
    let first = int(e.first())?;
    let second = match e.len() {
        1 => first >> 8,
        _ => int(e.get(1))?,
    };
    Some(SleepTypes { a: (first & SLP_TYP) as u8, b: (second & SLP_TYP) as u8 })
}

/// `\_S0` to `\_S5`'s name.
pub fn sx_seg(state: u8) -> [u8; 4] {
    [b'_', b'S', b'0' + state.min(5), b'_']
}
