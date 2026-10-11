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

//! Which PCI devices a DMAR remapping unit (DRHD) covers. A device may be
//! confined to a capsule's domain only when a unit in service translates it; a
//! device no unit covers would take the IOVA it is handed for a physical address
//! and DMA the wrong memory. Bring-up programs every segment 0 unit, so this is
//! the check that the broker's IOVA and the hardware agree, not a second way of
//! choosing units. This parses a
//! DRHD's flags and device scopes (VT-d 3.x, 8.3 and 8.3.1) and answers the
//! coverage question with no hardware, given a way to read a bridge's bus range.
//! It holds no allocation so the host proofs can include it unchanged.

use super::dmar_cover::scope_covers;
use super::dmar_scope::UnitScope;

/// Whether a unit covers a device.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cover {
    Yes,
    No,
    /// A scope could not be resolved (a bridge did not answer, or the unit's
    /// scope list was truncated), so the answer is not known.
    Unknown,
}

/// Whether some unit in `units` (every one of them programmed) translates the
/// device: a unit that covers its whole segment (INCLUDE_PCI_ALL), or a unit
/// whose scope names the device. A device no unit claims, or whose claim cannot
/// be resolved, is not translated: it keeps its physical address, which it can
/// always use, instead of an IOVA nothing would translate.
pub fn translated_by_some<F>(
    units: &[UnitScope],
    segment: u16,
    bdf: (u8, u8, u8),
    bridge_buses: &F,
) -> bool
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    units.iter().any(|u| {
        (u.include_all && u.segment == segment)
            || scope_covers(u, segment, bdf, bridge_buses) == Cover::Yes
    })
}
