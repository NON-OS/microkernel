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

//! _BIX (ACPI 6.5, 10.2.2.2), into the same shape as _BIF.

use super::fields::{elements, known, text};
use super::info::BatteryInfo;
use crate::object::Object;

/// _BIX: a revision first, then the _BIF integers with cycle count and the
/// sampling fields between them and the strings.
pub fn parse_bix(o: &Object) -> Option<BatteryInfo> {
    let e = elements(o)?;
    (e.len() >= 20).then(|| BatteryInfo {
        milliamp: known(&e, 1) == Some(1),
        design_capacity: known(&e, 2),
        last_full: known(&e, 3),
        rechargeable: known(&e, 4) == Some(1),
        design_voltage: known(&e, 5),
        warning: known(&e, 6),
        low: known(&e, 7),
        cycle_count: known(&e, 8),
        model: text(&e, 16),
        serial: text(&e, 17),
        kind: text(&e, 18),
        oem: text(&e, 19),
    })
}
