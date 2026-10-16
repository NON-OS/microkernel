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

//! A battery's present state, _BST (ACPI 6.5, 10.2.2.11).

use super::fields::{elements, known};
use crate::object::Object;

/// The state bits _BST defines: discharging, charging, critical and charge
/// limiting (bit 3 recalled, unverified). Bits past them are ignored, not
/// refused: a later revision may define them, and a bit we do not know
/// must not cost the battery its status or its critical bit.
const STATE_BITS: u32 = 0xF;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BatteryStatus {
    pub discharging: bool,
    pub charging: bool,
    /// Firmware said charging and discharging at once, which exclude each
    /// other: a fault, and the direction is unknown, so neither is set.
    pub contradicts: bool,
    pub critical: bool,
    /// The state bits past those defined, ignored; kept to be said once.
    pub reserved: u32,
    /// mA or mW, as the battery's information says.
    pub rate: Option<u32>,
    pub remaining: Option<u32>,
    pub voltage: Option<u32>,
}

impl BatteryStatus {
    /// Discharging, or a direction firmware made unknown: the low and
    /// critical checks act on this, since a wrong "not discharging" would
    /// keep a critical battery from its power-off.
    pub fn may_discharge(&self) -> bool {
        self.discharging || self.contradicts
    }
}

/// None when the package is short or the state word is not a known
/// integer; otherwise every field firmware gave, its lies set aside.
pub fn parse_bst(o: &Object) -> Option<BatteryStatus> {
    let e = elements(o)?;
    let s = known(&e, 0)?;
    let both = s & 3 == 3;
    (e.len() >= 4).then(|| BatteryStatus {
        discharging: s & 1 != 0 && !both,
        charging: s & 2 != 0 && !both,
        contradicts: both,
        critical: s & 4 != 0,
        reserved: s & !STATE_BITS,
        rate: known(&e, 1),
        remaining: known(&e, 2),
        voltage: known(&e, 3),
    })
}
