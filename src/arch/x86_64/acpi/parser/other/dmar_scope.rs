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

/// DRHD flags bit 0: the unit covers every PCI device in its segment not named
/// by another unit's scope. The spec requires such a unit to be listed last.
pub const FLAG_INCLUDE_PCI_ALL: u8 = 1 << 0;
/// Device scope type 1: a PCI endpoint.
pub const SCOPE_ENDPOINT: u8 = 1;
/// Device scope type 2: a PCI bridge and every device below it.
pub const SCOPE_BRIDGE: u8 = 2;
/// Scopes kept per unit; a unit naming more is marked truncated.
pub const MAX_SCOPES: usize = 16;
/// Path hops kept per scope; a longer path is marked unresolvable.
pub const MAX_PATH: usize = 4;

const DRHD_HEADER_LEN: usize = 16;
const SCOPE_HEADER_LEN: usize = 6;

/// One PCI device scope: its type, the bus its path starts on, and the
/// (device, function) hops of the path.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Scope {
    pub kind: u8,
    pub start_bus: u8,
    pub path: [(u8, u8); MAX_PATH],
    pub path_len: u8,
}

impl Scope {
    const EMPTY: Scope = Scope { kind: 0, start_bus: 0, path: [(0, 0); MAX_PATH], path_len: 0 };
}

/// What a DRHD says it covers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitScope {
    pub include_all: bool,
    pub segment: u16,
    pub scopes: [Scope; MAX_SCOPES],
    pub count: u8,
    /// A PCI scope was dropped (too many, or a path too long to keep), so a
    /// device not matched may still be this unit's.
    pub truncated: bool,
}

impl UnitScope {
    pub const EMPTY: UnitScope = UnitScope {
        include_all: false,
        segment: 0,
        scopes: [Scope::EMPTY; MAX_SCOPES],
        count: 0,
        truncated: false,
    };
}

/// Parse one DRHD structure (`bytes` is exactly its `length` bytes, type 0).
/// `None` for a structure too short to be a DRHD. Scope types other than
/// endpoint and bridge (IOAPIC, HPET, ACPI namespace) name no PCI device and are
/// skipped; a malformed scope ends the walk and marks the unit truncated.
pub fn parse_drhd(bytes: &[u8]) -> Option<UnitScope> {
    if bytes.len() < DRHD_HEADER_LEN || u16::from_le_bytes([bytes[0], bytes[1]]) != 0 {
        return None;
    }
    let mut unit = UnitScope::EMPTY;
    unit.include_all = bytes[4] & FLAG_INCLUDE_PCI_ALL != 0;
    unit.segment = u16::from_le_bytes([bytes[6], bytes[7]]);
    let mut at = DRHD_HEADER_LEN;
    while at + 2 <= bytes.len() {
        let kind = bytes[at];
        let len = bytes[at + 1] as usize;
        if len < SCOPE_HEADER_LEN
            || at + len > bytes.len()
            || !(len - SCOPE_HEADER_LEN).is_multiple_of(2)
        {
            unit.truncated = true;
            break;
        }
        if kind == SCOPE_ENDPOINT || kind == SCOPE_BRIDGE {
            let hops = (len - SCOPE_HEADER_LEN) / 2;
            if hops == 0 || hops > MAX_PATH || unit.count as usize >= MAX_SCOPES {
                unit.truncated = true;
            } else {
                let mut s = Scope::EMPTY;
                s.kind = kind;
                s.start_bus = bytes[at + 5];
                for h in 0..hops {
                    let p = at + SCOPE_HEADER_LEN + h * 2;
                    s.path[h] = (bytes[p], bytes[p + 1]);
                }
                s.path_len = hops as u8;
                unit.scopes[unit.count as usize] = s;
                unit.count += 1;
            }
        }
        at += len;
    }
    Some(unit)
}

/// Whether a unit covers a device.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cover {
    Yes,
    No,
    /// A scope could not be resolved (a bridge did not answer, or the unit's
    /// scope list was truncated), so the answer is not known.
    Unknown,
}

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
