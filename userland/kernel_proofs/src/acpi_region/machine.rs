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

//! A modelled machine for the region rule: a memory map, the windows and
//! ports other capsules hold, and where the kernel's own hardware sits.

use super::rule::{Hardware, MemKind};

pub struct Machine {
    /// [base, end) and its kind; address space no entry covers is Unmapped.
    pub map: Vec<(u64, u64, MemKind)>,
    pub windows: Vec<(u64, u64)>,
    pub ports: Vec<(u16, u16)>,
    pub hardware: Vec<(u64, u64, Hardware)>,
    pub pci: Vec<(u16, u8, u8, u8)>,
}

/// A small PC: RAM below 640 KiB and from 1 MiB, firmware's reserved and
/// ACPI ranges, an NVMe window another capsule holds, the keyboard ports a
/// driver holds, and the local APIC, I/O APIC, HPET and ECAM where PCs put
/// them.
pub fn pc() -> Machine {
    Machine {
        map: vec![
            (0, 0xA_0000, MemKind::Ram),
            (0xF_0000, 0x10_0000, MemKind::Reserved),
            (0x10_0000, 0x7FE0_0000, MemKind::Ram),
            (0x7FE0_0000, 0x7FF0_0000, MemKind::AcpiReclaim),
            (0x7FF0_0000, 0x8000_0000, MemKind::AcpiNvs),
            (0x8000_0000, 0x8000_1000, MemKind::Other),
        ],
        windows: vec![(0xFEB0_0000, 0xFEB0_4000)],
        ports: vec![(0x60, 0x61), (0x64, 0x65)],
        hardware: vec![
            (0xFEE0_0000, 0xFEE0_1000, Hardware::LocalApic),
            (0xFEC0_0000, 0xFEC0_1000, Hardware::IoApic),
            (0xFED0_0000, 0xFED0_0400, Hardware::Hpet),
            (0xE000_0000, 0xF000_0000, Hardware::Ecam),
        ],
        pci: vec![(0, 0, 3, 0)],
    }
}
