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

//! A battery's static information: _BIX (ACPI 6.5, 10.2.2.2) when firmware
//! has it, else _BIF (10.2.2.1). Both are read into one shape.

use super::fields::{elements, known, text};
use crate::object::Object;
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BatteryInfo {
    /// Capacities in mAh (and rates in mA) rather than mWh (and mW).
    pub milliamp: bool,
    pub design_capacity: Option<u32>,
    pub last_full: Option<u32>,
    pub rechargeable: bool,
    pub design_voltage: Option<u32>,
    pub warning: Option<u32>,
    pub low: Option<u32>,
    /// _BIX only.
    pub cycle_count: Option<u32>,
    pub model: Vec<u8>,
    pub serial: Vec<u8>,
    pub kind: Vec<u8>,
    pub oem: Vec<u8>,
}

/// _BIF: unit, design, last full, technology, voltage, warning, low, two
/// granularities, then model, serial, type and OEM strings.
pub fn parse_bif(o: &Object) -> Option<BatteryInfo> {
    let e = elements(o)?;
    (e.len() >= 13).then(|| BatteryInfo {
        milliamp: known(&e, 0) == Some(1),
        design_capacity: known(&e, 1),
        last_full: known(&e, 2),
        rechargeable: known(&e, 3) == Some(1),
        design_voltage: known(&e, 4),
        warning: known(&e, 5),
        low: known(&e, 6),
        cycle_count: None,
        model: text(&e, 9),
        serial: text(&e, 10),
        kind: text(&e, 11),
        oem: text(&e, 12),
    })
}
