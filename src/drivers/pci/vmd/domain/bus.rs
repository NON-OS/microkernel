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

//! Where a VMD's child buses start and how many CFGBAR decodes, and where a
//! child function's register sits in CFGBAR.

use super::ids::BUS_RESTRICTED;
use super::window::MIB;

/// VMD capability register; bit 0 says the bus range is restricted.
pub const VMCAP: u16 = 0x40;
/// VMD configuration register; bits 9:8 select where child buses start.
pub const VMCONFIG: u16 = 0x44;

/// The bus number the child domain's root bus has. `None` for a reserved
/// encoding, which leaves the domain unusable rather than guessed at.
pub fn bus_start(device: u16, vmcap: u16, vmconfig: u16) -> Option<u8> {
    if !BUS_RESTRICTED.contains(&device) || vmcap & 1 == 0 {
        return Some(0);
    }
    match (vmconfig >> 8) & 0x3 {
        0 => Some(0),
        1 => Some(128),
        2 => Some(224),
        _ => None,
    }
}

/// How many child buses CFGBAR decodes: one MiB of config space each, and
/// never past bus 255.
pub fn bus_count(bus_start: u8, cfgbar_size: u64) -> u16 {
    let decoded = (cfgbar_size / MIB).min(256) as u16;
    decoded.min(256 - bus_start as u16)
}

/// Offset of a child function's register inside CFGBAR, ECAM-shaped but
/// relative to the domain's first bus. `None` outside the decoded buses or
/// past the 4 KiB of one function.
pub fn cfg_offset(
    bus_start: u8,
    bus_count: u16,
    bus: u8,
    device: u8,
    function: u8,
    offset: u16,
) -> Option<u64> {
    if bus < bus_start || (bus - bus_start) as u16 >= bus_count {
        return None;
    }
    if device > 31 || function > 7 || offset > 0xFFC {
        return None;
    }
    let rel = (bus - bus_start) as u64;
    Some((rel << 20) | ((device as u64) << 15) | ((function as u64) << 12) | offset as u64)
}
