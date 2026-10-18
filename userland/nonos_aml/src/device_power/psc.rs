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

//! Where a device is now, by its own word (ACPI 6.5, 7.3.6, _PSC). Without
//! _PSC the state is unknown to the device; a device with no power
//! methods or resources at all is always in D0.

use super::state::DState;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::{child, integer, Eval};
use alloc::vec::Vec;

pub fn psc(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<DState> {
    let r = child(aml, eval, dev, *b"_PSC", Vec::new())?.ok()?;
    integer(&r).and_then(DState::from_psc)
}

/// Whether the device has any power control: a _PSx or _PRx.
pub fn power_managed(aml: &Aml, dev: Id) -> bool {
    let segs = [*b"_PS0", *b"_PS3", *b"_PR0", *b"_PR3"];
    segs.iter().any(|s| aml.ns.child(dev, *s).is_some())
}
