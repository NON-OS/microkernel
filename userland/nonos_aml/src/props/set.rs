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

//! The sets a _DSD package holds: pairs of a UUID buffer and a package.
//! The device-properties set and any vendor's set laid out the same way
//! are read as name/value pairs; a set NONOS does not ask for is left.

use super::uuid::DEVICE_PROPERTIES_UUID;
use super::value::{property, Property, MAX};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec::Vec;

/// The package paired with `uuid` in a _DSD-shaped package's elements.
pub(super) fn set_in(dsd: &[Object], uuid: &[u8; 16]) -> Option<Object> {
    let ours = |u: &Object| matches!(u, Object::Buffer(b) if *b.borrow() == *uuid);
    let i = dsd.iter().step_by(2).position(ours)?;
    match dsd.get(i * 2 + 1) {
        Some(o @ Object::Package(_)) => Some(o.clone()),
        _ => None,
    }
}

pub(super) fn pairs(aml: &Aml, set: &Object) -> Vec<Property> {
    let Object::Package(set) = set else { return Vec::new() };
    let set = set.borrow();
    set.iter().take(MAX).filter_map(|o| property(aml, o)).collect()
}

/// The device's properties, in _DSD's order; none when it has no _DSD, it
/// fails, or it carries no device-properties set.
pub fn properties(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Vec<Property> {
    properties_under(aml, eval, dev, &DEVICE_PROPERTIES_UUID)
}

/// The name/value pairs in the set `uuid` names.
pub fn properties_under(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    uuid: &[u8; 16],
) -> Vec<Property> {
    let Some(Ok(Object::Package(dsd))) = child(aml, eval, dev, *b"_DSD", Vec::new()) else {
        return Vec::new();
    };
    let set = set_in(&dsd.borrow(), uuid);
    set.map(|s| pairs(aml, &s)).unwrap_or_default()
}
