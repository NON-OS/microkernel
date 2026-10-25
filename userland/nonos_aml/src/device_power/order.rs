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

//! The order power resources are switched in (ACPI 6.5, 7.2.1 and
//! 19.6.104): each declares a ResourceOrder, and resources are turned on
//! lowest order first and off highest first.

use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin};
use alloc::vec::Vec;

/// DefPowerRes: its name, SystemLevel, then ResourceOrder.
pub fn resource_order(aml: &Aml, res: Id) -> u16 {
    match aml.ns.get(res).map(|e| (e.kind, e.origin)) {
        Some((Kind::PowerResource, Origin::Table { table, node })) => {
            aml.imm(table, aml.kid(table, node, 2)) as u16
        }
        _ => 0,
    }
}

/// Ascending ResourceOrder; a resource listed twice is taken once, and
/// anything that is no power resource is dropped.
pub fn in_order(aml: &Aml, mut res: Vec<Id>) -> Vec<Id> {
    res.retain(|r| aml.ns.get(*r).is_some_and(|e| e.kind == Kind::PowerResource));
    res.sort_by_key(|r| (resource_order(aml, *r), *r));
    res.dedup();
    res
}
