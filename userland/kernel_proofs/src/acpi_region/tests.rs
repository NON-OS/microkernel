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

//! The memory rule on a modelled PC: firmware memory and unclaimed device
//! space are granted; RAM, claimed windows and kernel hardware are not.

use super::machine::pc;
use super::rule::{memory, Hardware, Legacy, MemKind, Refusal};

#[test]
fn firmware_memory_and_free_device_space_are_granted() {
    let m = pc();
    assert_eq!(memory(0xF_0000, 0x1_0000, &m), Ok(()), "reserved");
    assert_eq!(memory(0x7FE0_0000, 0x20_0000, &m), Ok(()), "ACPI reclaimable then NVS");
    assert_eq!(memory(0xFED4_0000, 0x5000, &m), Ok(()), "unmapped device space (a TPM)");
    assert_eq!(memory(0xC_0000, 0x1000, &m), Ok(()), "a hole in the map below 1 MiB");
}

#[test]
fn ram_and_other_kinds_are_refused_even_in_part() {
    let m = pc();
    assert_eq!(memory(0x10_0000, 0x1000, &m), Err(Refusal::Memory(MemKind::Ram)));
    assert_eq!(
        memory(0x7FDF_F000, 0x2000, &m),
        Err(Refusal::Memory(MemKind::Ram)),
        "straddling RAM and ACPI"
    );
    assert_eq!(memory(0x8000_0000, 0x1000, &m), Err(Refusal::Memory(MemKind::Other)));
    assert_eq!(memory(0, 1, &m), Err(Refusal::Memory(MemKind::Ram)));
}

#[test]
fn claimed_windows_and_kernel_hardware_are_refused() {
    let m = pc();
    assert_eq!(memory(0xFEB0_2000, 0x10, &m), Err(Refusal::Claimed));
    assert_eq!(memory(0xFEE0_0300, 4, &m), Err(Refusal::Protected(Hardware::LocalApic)));
    assert_eq!(memory(0xFEB0_4000, 0x1000, &m), Ok(()), "beside the window, not in it");
    assert_eq!(memory(0xFEC0_0000, 0x20, &m), Err(Refusal::Protected(Hardware::IoApic)));
    assert_eq!(memory(0xFECF_F000, 0x8_0000, &m), Err(Refusal::Protected(Hardware::Hpet)));
    assert_eq!(
        memory(0xEFFF_F000, 0x1000, &m),
        Err(Refusal::Protected(Hardware::Ecam)),
        "a claimed function's config behind memory"
    );
}

#[test]
fn every_refusal_names_its_reason() {
    for r in [
        Refusal::Empty,
        Refusal::Wraps,
        Refusal::Claimed,
        Refusal::Memory(MemKind::Ram),
        Refusal::Protected(Hardware::Iommu),
        Refusal::Legacy(Legacy::PciConfig),
        Refusal::Legacy(Legacy::Rtc),
        Refusal::Legacy(Legacy::Serial),
        Refusal::Protected(Hardware::Ecam),
    ] {
        assert!(!r.reason().is_empty());
    }
}
