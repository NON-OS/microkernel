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

//! SRAT read from its bytes: every kind with its domain, a disabled entry
//! and an unknown kind skipped, a short entry or one that runs past the
//! table ending the walk.

use super::srat_build::*;
use crate::arm_acpi::numa::srat_entry::{entries, Handle, SratEntry};

#[test]
fn every_kind_with_its_domain() {
    let mut pci = [0u8; 16];
    pci[..4].copy_from_slice(&[1, 0, 0x3A, 0x10]);
    let mut acpi = [0u8; 16];
    acpi[..12].copy_from_slice(b"ACPI0007\x05\0\0\0");
    let t = srat(&[
        apic(4, 0x0102_0301, true),
        apic(5, 1, false),
        memory(1, 0x1_0000_0000, 0x4000_0000, 0b111),
        memory(2, 0, 0x1000, 0b110),
        x2apic(0x200, 3, true),
        gicc(7, 2, true),
        initiator(1, 4, pci),
        initiator(0, 5, acpi),
        initiator(2, 6, [0; 16]),
        super::build::structure(9, &[0; 6]),
    ]);
    let e = entries(&t);
    let mem = SratEntry::Memory {
        domain: 1,
        base: 0x1_0000_0000,
        len: 0x4000_0000,
        hot_pluggable: true,
        non_volatile: true,
    };
    let want = [
        SratEntry::Apic { apic_id: 4, domain: 0x0102_0301 },
        mem,
        SratEntry::Apic { apic_id: 0x200, domain: 3 },
        SratEntry::Gicc { uid: 7, domain: 2 },
        SratEntry::Initiator {
            domain: 4,
            handle: Handle::Pci { segment: 1, bus: 0x3A, devfn: 0x10 },
        },
        SratEntry::Initiator { domain: 5, handle: Handle::Acpi { hid: *b"ACPI0007", uid: 5 } },
    ];
    assert_eq!(e, want, "disabled, unknown handle and unknown kind skipped");
}

#[test]
fn a_bad_entry_ends_the_walk() {
    let good = gicc(1, 1, true);
    let mut short = srat(&[good.clone(), vec![3, 1], good.clone()]);
    assert_eq!(entries(&short).len(), 1, "a length under 2");
    let len = short.len();
    short.truncate(len - 4);
    assert_eq!(entries(&short).len(), 1);
    let mut cut = gicc(1, 1, true);
    cut[1] = 12;
    assert_eq!(entries(&srat(&[cut, good])), [], "a GICC entry shorter than its kind");
    assert_eq!(entries(&[]), []);
}
