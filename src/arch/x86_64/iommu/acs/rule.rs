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

//! Devices below one PCIe switch that reach each other past the IOMMU (L72).
//! A switch routes a request from one of its downstream ports to another
//! itself, unless the port the request enters sends it upstream: the ACS
//! controls (PCIe base, the ACS extended capability; recalled, unverified).
//! With them off on any one port of a switch, a device below it may write a
//! peer's registers below the same switch past every remapping entry, so
//! everything below that switch is one group, held by one capsule at most
//! (the broker's `acs_span`). Pure, so the walk and the controls run on the host too.

/// The ACS extended capability's ID.
pub(super) const ACS_ID: u16 = 0x000D;
/// Source Validation, P2P Request Redirect, P2P Completion Redirect and
/// Upstream Forwarding, bits 0, 2, 3 and 4 of the capability and control.
pub(super) const NEEDED: u16 = 1 << 0 | 1 << 2 | 1 << 3 | 1 << 4;
/// Direct Translated P2P, bit 6: kept off, since no device here has ATS.
pub(super) const DIRECT: u16 = 1 << 6;

/// The control word that closes a port whose capability offers every needed
/// control, the rest as it was; `None` when one is missing.
pub(super) const fn closing(cap: u16, ctl: u16) -> Option<u16> {
    if cap & NEEDED != NEEDED {
        return None;
    }
    Some((ctl | NEEDED) & !DIRECT)
}

/// Whether a port's control word, as read back, has it closed.
pub(super) const fn closed(ctl: u16) -> bool {
    ctl & NEEDED == NEEDED && ctl & DIRECT == 0
}

/// Where extended capability `id` starts, walking from 100h. The list is not
/// ordered and cannot hold more entries than dwords above 100h, which bounds
/// a chain that loops; a header of all zeros or all ones ends it.
pub(super) fn find_ext(read: impl Fn(u16) -> u32, id: u16) -> Option<u16> {
    let mut at = 0x100u16;
    for _ in 0..(0x1000 - 0x100) / 4 {
        let header = read(at);
        if header == 0 || header == !0 {
            return None;
        }
        if header as u16 == id {
            return Some(at);
        }
        let next = (header >> 20) as u16 & !3;
        if next < 0x100 {
            return None;
        }
        at = next;
    }
    None
}
