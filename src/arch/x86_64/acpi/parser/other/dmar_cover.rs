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

//! Whether a unit's or a region's scope list names a device, resolved
//! through the bridges between.

use super::dmar_scope::{Scope, UnitScope, SCOPE_BRIDGE};
use super::dmar_translated::Cover;

/// Does `unit`'s explicit scope list name the device at `segment`/`bdf`?
/// `bridge_buses(bus, dev, func)` returns a bridge's (secondary, subordinate)
/// bus numbers, or `None` if that function is not an answering bridge.
/// INCLUDE_PCI_ALL is not consulted here; see `translated_by_first`.
pub fn scope_covers<F>(unit: &UnitScope, segment: u16, bdf: (u8, u8, u8), bridge_buses: &F) -> Cover
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    if segment != unit.segment {
        return Cover::No;
    }
    let mut unknown = unit.truncated;
    for s in &unit.scopes[..unit.count as usize] {
        let Some(target) = resolve(s, bridge_buses) else {
            unknown = true;
            continue;
        };
        if target == bdf {
            return Cover::Yes;
        }
        if s.kind == SCOPE_BRIDGE {
            match bridge_buses(target.0, target.1, target.2) {
                Some((sec, sub)) if sec <= bdf.0 && bdf.0 <= sub => return Cover::Yes,
                Some(_) => {}
                None => unknown = true,
            }
        }
    }
    if unknown {
        Cover::Unknown
    } else {
        Cover::No
    }
}

// Walk a scope's path: every hop but the last is a bridge whose secondary bus
// the next hop sits on (VT-d 8.3.1).
fn resolve<F>(s: &Scope, bridge_buses: &F) -> Option<(u8, u8, u8)>
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    let hops = &s.path[..s.path_len as usize];
    let (last, through) = hops.split_last()?;
    let mut bus = s.start_bus;
    for &(dev, func) in through {
        bus = bridge_buses(bus, dev, func)?.0;
    }
    Some((bus, last.0, last.1))
}
