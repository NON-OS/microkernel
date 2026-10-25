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

//! When find runs, read as text (L79). The broker is seeded in one fixed
//! order: the PCI table first, the devices the firmware's description names
//! (C's, on aarch64), its port windows and the legacy records next, then the
//! framebuffer's registration, which finds who covers it; so every record
//! and its BARs exist when find reads the table. The
//! firmware's I2C and GPIO controllers, the SCI, the NUMA facts and the
//! SDEV posture come after, and none of them is asked.

use super::glue_text::{body, code};

const SEED: &str = "kernel_core/init/platform/hardware_broker.rs";

const ORDER: &str = concat!(
    "crate::hardware::broker::init_from_pci(&devices);",
    "let_=crate::arch::publish_described_devices();",
    "#[cfg(target_arch=\"x86_64\")]{",
    "crate::bus::pci::report_aer(&devices);",
    "crate::arch::iommu::acs::close_switches(&devices);",
    "}",
    "let_=crate::hardware::broker::register_legacy_platform_devices();",
    "crate::hardware::broker::register_boot_framebuffer();",
    "#[cfg(target_arch=\"x86_64\")]{",
    "crate::hardware::broker::register_acpi_i2c();",
    "crate::hardware::broker::register_acpi_gpio();",
    "let_=crate::hardware::broker::acpi::register_sci();",
    "crate::hardware::broker::acpi::numa_refresh::refresh();",
    "crate::hardware::broker::acpi::sdev_posture::sdev_posture();",
    "}",
);

#[test]
fn find_runs_once_the_pci_table_and_the_legacy_records_exist() {
    let src = code(SEED);
    let seed = body(&src, "pub(super)fnseed_hardware_broker(){");
    let scan = "letdevices=matchcrate::drivers::pci::manager::scan_and_collect_safe(){";
    assert!(seed.starts_with(scan), "a step before the scan");
    let at = seed.find(ORDER).expect("the seeding order changed");
    assert!(seed[..at].ends_with("return;}};"), "a step between the scan and the table");
    assert_eq!(seed.matches("register_boot_framebuffer(").count(), 1, "registered twice");
    assert_eq!(seed.matches("init_from_pci(").count(), 1, "the table seeded twice");
}
