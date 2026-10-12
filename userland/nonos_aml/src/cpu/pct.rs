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

//! _PCT (ACPI 6.5, 8.4.5.1): the registers a performance state is set
//! through and read back from. Fixed hardware (0x7F) means the processor's
//! own interface, model-specific registers on x86.

use super::register::{register, Register};
use crate::object::Object;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pct {
    pub control: Register,
    pub status: Register,
}

pub fn parse_pct(o: &Object) -> Option<Pct> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    Some(Pct { control: register(e.first()?)?, status: register(e.get(1)?)? })
}
