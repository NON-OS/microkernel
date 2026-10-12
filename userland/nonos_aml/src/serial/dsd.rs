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

//! Device properties from _DSD (ACPI 6.5, 6.2.5): pairs of a UUID and a
//! package, of which NONOS reads the device-properties set, each property
//! a name and a value. Other UUIDs (hierarchical data, vendors') are left.

use super::dsd_value::{property, Property};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec::Vec;

/// daffd814-6eba-4d8c-8a91-bc9bbf4aa301, in ToUUID's byte order.
pub const DEVICE_PROPERTIES_UUID: [u8; 16] = [
    0x14, 0xD8, 0xFF, 0xDA, 0xBA, 0x6E, 0x8C, 0x4D, 0x8A, 0x91, 0xBC, 0x9B, 0xBF, 0x4A, 0xA3, 0x01,
];
/// More than any device declares; the rest are dropped.
pub(super) const MAX: usize = 64;

/// The device's properties, in _DSD's order; none when it has no _DSD, it
/// fails, or it carries no device-properties set.
pub fn properties(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Vec<Property> {
    properties_under(aml, eval, dev, &DEVICE_PROPERTIES_UUID)
}

/// The name/value pairs in the set `uuid` names: the device-properties set,
/// or another vendor's set laid out the same way.
pub fn properties_under(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    uuid: &[u8; 16],
) -> Vec<Property> {
    let Some(Ok(Object::Package(dsd))) = child(aml, eval, dev, *b"_DSD", Vec::new()) else {
        return Vec::new();
    };
    let dsd = dsd.borrow();
    let ours = |u: &Object| matches!(u, Object::Buffer(b) if *b.borrow() == *uuid);
    let Some(i) = dsd.iter().step_by(2).position(ours) else { return Vec::new() };
    let Some(Object::Package(set)) = dsd.get(i * 2 + 1) else { return Vec::new() };
    let set = set.borrow();
    set.iter().take(MAX).filter_map(|o| property(aml, o)).collect()
}
