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

//! L55's flag: set once and never again, read back as set, and asked for
//! only by the `acpi` owner's thread group. And F-76's three SD/eMMC
//! record shapes classify as the ACPI-published SD host, while PCI eMMC
//! hosts and other platform records do not.

use super::acpi::first_pass::{done, mark};
use super::acpi::first_pass_rule::{admits, wait_needed};
use super::acpi::platform_record::record;
use super::acpi::platform_wire::{GPIO, I2C, SD};
use super::device::BusKind;
use super::round_trip_tests::platform;
use crate::hardware::platform_sd::is_platform_sd_host;

#[test]
fn the_flag_goes_up_once_and_stays_up() {
    assert!(mark(), "the first time sets it");
    (0..3).for_each(|_| assert!(!mark(), "again changes nothing"));
    assert!(done());
}

#[test]
fn only_the_owners_thread_group_may_set_it() {
    // Pids 10 and 11 are one group (10); 20 is its own.
    let group = |p: u32| match p {
        10 | 11 => Some(10),
        20 => Some(20),
        _ => None,
    };
    assert!(admits(Some(10), 10, group) && admits(Some(10), 11, group), "owner and its thread");
    assert!(!admits(Some(10), 20, group), "another process");
    assert!(!admits(None, 10, group), "no one runs acpi");
    assert!(!admits(Some(0), 0, group), "pid 0 is no owner");
    assert!(!admits(Some(10), 0, group), "pid 0 is no caller");
}

#[test]
fn the_three_sd_shapes_are_sd_hosts_and_nothing_else_is() {
    for hid in [&b"80860F14"[..], b"80860F16", b"AMDI0040"] {
        let r = record(&platform(SD, hid, &[(0xFED0_0000, 0x1000)], b"\\_SB.SDHC"), 0);
        assert!(is_platform_sd_host(r.bus_kind, r.pci_class, r.pci_subclass), "{hid:?}");
    }
    for class in [I2C, GPIO] {
        let r = record(&platform(class, b"INT33C2", &[(0xFED1_0000, 0x1000)], b"\\_SB.I2C0"), 0);
        assert!(!is_platform_sd_host(r.bus_kind, r.pci_class, r.pci_subclass));
    }
    assert!(!is_platform_sd_host(BusKind::Pci as u8, 0x08, 0x05), "a PCI eMMC host stays PCI");
}

#[test]
fn no_boot_without_a_spawned_acpi_owner_ever_waits() {
    for bits in 0..8u8 {
        let (acpi, spawned, done) = (bits & 1 != 0, bits & 2 != 0, bits & 4 != 0);
        let waits = wait_needed(acpi, spawned, done);
        assert_eq!(waits, acpi && spawned && !done);
        assert!(spawned || !waits, "never without a spawned owner");
        assert!(acpi || !waits, "never on a boot with no ACPI");
    }
}
