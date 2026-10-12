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

//! A battery's present state, _BST (ACPI 6.5, 10.2.2.11), and the charge it
//! amounts to; and an AC adapter's _PSR (10.3.1).

use super::fields::{elements, known};
use super::info::BatteryInfo;
use crate::object::Object;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BatteryStatus {
    pub discharging: bool,
    pub charging: bool,
    pub critical: bool,
    /// mA or mW, as the battery's information says.
    pub rate: Option<u32>,
    pub remaining: Option<u32>,
    pub voltage: Option<u32>,
}

pub fn parse_bst(o: &Object) -> Option<BatteryStatus> {
    let e = elements(o)?;
    let state = known(&e, 0)?;
    (e.len() >= 4).then(|| BatteryStatus {
        discharging: state & 1 != 0,
        charging: state & 2 != 0,
        critical: state & 4 != 0,
        rate: known(&e, 1),
        remaining: known(&e, 2),
        voltage: known(&e, 3),
    })
}

/// The charge as a percentage of the last full charge (else of the design
/// capacity), 0 to 100; None when either number is unknown or zero.
pub fn percent(info: &BatteryInfo, status: &BatteryStatus) -> Option<u8> {
    let full = info.last_full.filter(|f| *f > 0).or(info.design_capacity.filter(|d| *d > 0))?;
    let left = status.remaining?;
    Some((u64::from(left.min(full)) * 100 / u64::from(full)) as u8)
}

/// _PSR: 1 when the adapter supplies power.
pub fn online(o: &Object) -> Option<bool> {
    match o {
        Object::Integer(v) => Some(*v != 0),
        _ => None,
    }
}
