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

//! Finding the thermal zones (ACPI 6.5, 11.1.1): ThermalZone objects, under
//! \_TZ by convention but legal anywhere, in definition order.

use crate::load::Aml;
use crate::namespace::{Id, Kind, ROOT};
use crate::platform::listed;
use crate::platform::Eval;
use alloc::vec::Vec;

const MAX_VISITS: usize = 1 << 20;

pub fn zones(aml: &Aml) -> Vec<Id> {
    let (mut out, mut stack, mut visits) = (Vec::new(), alloc::vec![ROOT], 0);
    while let Some(id) = stack.pop() {
        visits += 1;
        if visits > MAX_VISITS {
            break;
        }
        let at = stack.len();
        for c in aml.ns.children(id) {
            match aml.ns.get(c).map(|e| e.kind) {
                Some(Kind::ThermalZone) => out.push(c),
                Some(k) if k.is_container() => stack.insert(at, c),
                _ => {}
            }
        }
    }
    out
}

/// _ALx: the devices that cool the zone at active level `x` (11.4.2).
pub fn active_devices(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, x: usize) -> Vec<Id> {
    listed(aml, eval, tz, [b'_', b'A', b'L', b'0' + x as u8])
}
