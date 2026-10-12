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

//! \_PIC (ACPI 6.5, 5.8.1): telling firmware which interrupt model the
//! operating system uses, before _PRT is read, since many _PRTs answer by
//! it.

use super::eval::{child, Eval};
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::ROOT;
use crate::object::Object;
use alloc::vec;

/// The argument's values (table 5.177).
pub const PIC_8259: u64 = 0;
pub const PIC_APIC: u64 = 1;

/// None when the firmware has no \_PIC.
pub fn set_pic(aml: &mut Aml, eval: &mut Eval<'_>, model: u64) -> Option<Result<(), Fault>> {
    let r = child(aml, eval, ROOT, *b"_PIC", vec![Object::Integer(model)])?;
    Some(r.map(|_| ()))
}
