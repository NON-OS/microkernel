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

//! _STA (ACPI 6.5, 6.3.7): whether a device is present, enabled, shown and
//! functioning. A device with no _STA is all four.

use super::eval::{child, integer, Eval};
use crate::exec::{Fault, FaultKind};
use crate::load::Aml;
use crate::namespace::Id;
use alloc::vec::Vec;

pub const PRESENT: u64 = 1 << 0;
pub const ENABLED: u64 = 1 << 1;
pub const SHOWN: u64 = 1 << 2;
pub const FUNCTIONING: u64 = 1 << 3;
pub const BATTERY: u64 = 1 << 4;
pub const DEFAULT: u64 = PRESENT | ENABLED | SHOWN | FUNCTIONING;

pub fn sta(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Result<u64, Fault> {
    match child(aml, eval, dev, *b"_STA", Vec::new()) {
        None => Ok(DEFAULT),
        Some(r) => integer(&r?).ok_or(Fault::new(FaultKind::Type, 0, 0)),
    }
}
