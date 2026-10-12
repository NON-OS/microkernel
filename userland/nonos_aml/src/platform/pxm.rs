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

//! A device's proximity domain (ACPI 6.5, 6.2.14): its _PXM, else the
//! nearest ancestor's, a domain covering everything below the device that
//! declares it. That an ancestor's _PXM applies is recalled from the
//! specification, not read here: unverified. None when no device on the
//! way to the root declares one, or a _PXM fails or is no integer.

use super::eval::{child, integer, Eval};
use crate::load::Aml;
use crate::namespace::{Id, ROOT};
use alloc::vec::Vec;

/// Levels walked up; deeper than any namespace firmware writes.
const MAX_LEVELS: usize = 64;

pub fn pxm(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<u32> {
    let mut at = dev;
    for _ in 0..MAX_LEVELS {
        if let Some(r) = child(aml, eval, at, *b"_PXM", Vec::new()) {
            let v = r.ok().as_ref().and_then(integer)?;
            return u32::try_from(v).ok();
        }
        if at == ROOT {
            return None;
        }
        at = aml.ns.get(at)?.parent;
    }
    None
}
