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

//! The table-read rule: only firmware's memory, or the legacy BIOS window.

use super::machine::{pc, Machine};
use super::rule::{table_read, MemKind, Refusal, LEGACY_BIOS};

/// A UEFI laptop: the legacy window typed as RAM, the kernel image as
/// loader code (RAM to the rule), ACPI tables in reclaim and NVS, SMBIOS in
/// runtime services data, and nothing in the map past 0x8000_1000.
fn laptop() -> Machine {
    Machine {
        map: vec![
            (0, 0x10_0000, MemKind::Ram),
            (0x10_0000, 0x40_0000, MemKind::Ram),
            (0x40_0000, 0x7FD0_0000, MemKind::Ram),
            (0x7FD0_0000, 0x7FE0_0000, MemKind::EfiRuntime),
            (0x7FE0_0000, 0x7FF0_0000, MemKind::AcpiReclaim),
            (0x7FF0_0000, 0x8000_0000, MemKind::AcpiNvs),
            (0x8000_0000, 0x8000_1000, MemKind::Reserved),
        ],
        ..pc()
    }
}

#[test]
fn firmware_memory_is_read() {
    let m = laptop();
    assert_eq!(table_read(0x7FE0_0000, 0x1_0000, &m), Ok(()), "ACPI reclaim");
    assert_eq!(table_read(0x7FD0_1000, 0x4000, &m), Ok(()), "SMBIOS in runtime data");
    assert_eq!(table_read(0x7FDF_F000, 0x2_0000, &m), Ok(()), "runtime into reclaim");
    assert_eq!(table_read(0x7FFF_FF00, 0x1100, &m), Ok(()), "NVS into reserved");
    let (lo, hi) = LEGACY_BIOS;
    assert_eq!(table_read(lo, hi - lo, &m), Ok(()), "the legacy window, typed RAM");
    assert_eq!(table_read(0xF_5A00, 0x1F, &m), Ok(()), "an SMBIOS 2 entry point");
}

#[test]
fn ram_device_space_and_the_unmapped_are_refused() {
    let m = laptop();
    let ram = Err(Refusal::Memory(MemKind::Ram));
    assert_eq!(table_read(0x7FCF_F000, 0x2000, &m), ram, "straddles RAM");
    assert_eq!(table_read(0x20_0000, 0x1000, &m), ram, "inside the kernel image");
    assert_eq!(table_read(0xE_F000, 0x2000, &m), ram, "starts below the window");
    assert_eq!(table_read(0xF_F000, 0x2000, &m), ram, "runs out of the window");
    let gone = Err(Refusal::Memory(MemKind::Unmapped));
    assert_eq!(table_read(0x8000_0000, 0x2000, &m), gone, "past the map");
    assert_eq!(table_read(0xFEB0_0000, 0x1000, &m), gone, "device space");
    assert_eq!(table_read(0x7FE0_0000, 0, &m), Err(Refusal::Empty));
    assert_eq!(table_read(u64::MAX, 2, &m), Err(Refusal::Wraps));
}

#[test]
fn other_types_are_refused() {
    let m = Machine { map: vec![(0x9000_0000, 0x9000_1000, MemKind::Other)], ..pc() };
    assert_eq!(table_read(0x9000_0000, 0x100, &m), Err(Refusal::Memory(MemKind::Other)));
    let pc = pc();
    assert_eq!(table_read(0xF_0000, 0x1_0000, &pc), Ok(()), "reserved window too");
}
