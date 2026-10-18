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

//! The machine the F-105b sweep reads on, where its tables are named, and
//! which bytes of it the model counts as firmware's.

use super::super::machine::{pc, Machine};
use super::super::rule::MemKind;
use super::gen::Rng;
use crate::smbios::smbios_entry::TABLE_MAX;

/// A UEFI laptop: SMBIOS in runtime data, ACPI in reclaim and NVS.
pub fn laptop() -> Machine {
    let map = vec![
        (0, 0x10_0000, MemKind::Ram),
        (0x10_0000, 0x7FD0_0000, MemKind::Ram),
        (0x7FD0_0000, 0x7FE0_0000, MemKind::EfiRuntime),
        (0x7FE0_0000, 0x7FF0_0000, MemKind::AcpiReclaim),
        (0x7FF0_0000, 0x8000_0000, MemKind::AcpiNvs),
        (0x8000_0000, 0x8000_1000, MemKind::Reserved),
    ];
    Machine { map, ..pc() }
}

/// Every byte of [start, end) in the legacy window or in a map entry of a
/// kind firmware keeps its tables in (ACPI reclaim and NVS, reserved, UEFI
/// runtime data); a byte no entry covers is device space.
pub fn firmware_only(m: &Machine, start: u64, end: u64) -> bool {
    // The legacy BIOS area, 0xF0000 to 0xFFFFF (DSP0134 5.2.1; recalled).
    let (lo, hi) = (0xF_0000, 0x10_0000);
    let ours = [MemKind::Reserved, MemKind::AcpiNvs, MemKind::AcpiReclaim, MemKind::EfiRuntime];
    let mut at = start;
    while at < end {
        if (lo..hi).contains(&at) {
            at = hi;
            continue;
        }
        let Some(&(_, top, _)) =
            m.map.iter().find(|(b, t, k)| (*b..*t).contains(&at) && ours.contains(k))
        else {
            return false;
        };
        at = top;
    }
    true
}

/// Where a table might be named, against `laptop()`'s map.
pub fn place(r: &mut Rng) -> u64 {
    let near = [0x7FD0_0000, 0x7FE0_0000, 0x7FF0_0000, 0x7FCF_F000, 0xF_0000, 0xF_F000, 0xE_F000];
    match r.below(5) {
        0 => u64::MAX - r.below(u64::from(TABLE_MAX)),
        1 => r.below(u64::MAX),
        2 => 0x8000_1000 + r.below(0x10_0000),
        _ => near[r.below(7) as usize] + r.below(0x1_0000),
    }
}
