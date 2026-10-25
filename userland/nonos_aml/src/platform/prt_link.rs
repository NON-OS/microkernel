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

//! A link device's interrupt (ACPI 6.5, 6.2.13): the one at SourceIndex
//! in its current resources.

use super::eval::{child, Eval};
use super::resource::interrupts;
use crate::exec::{Fault, FaultKind};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use alloc::vec::Vec;

/// The GSI a link device's _CRS names at `index`.
pub(super) fn link_gsi(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    link: Id,
    index: u64,
) -> Result<Option<u32>, Fault> {
    let crs = match child(aml, eval, link, *b"_CRS", Vec::new()) {
        None => return Ok(None),
        Some(r) => r?,
    };
    let Object::Buffer(b) = crs else { return Err(Fault::new(FaultKind::Type, 0, 0)) };
    let found = interrupts(&b.borrow());
    Ok(usize::try_from(index).ok().and_then(|i| found.get(i).copied()))
}
