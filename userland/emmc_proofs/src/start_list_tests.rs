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

//! The kernel starts driver.sdhci0 for the Intel eMMC hosts and the other
//! SD hosts its inventory lists (src/hardware/inventory/emmc.rs and
//! sd_host.rs); the capsule serves the ones its own table takes. A host
//! the capsule takes and the kernel does not would never get its driver
//! started: 8086:9d2b was one.

use crate::emmc::pci::{classify, Kind, INTEL_EMMC, INTEL_SDIO};
use crate::kernel_inventory::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};
use crate::kernel_sd_host::{is_sd_host, INTEL_SDIO_DEVICE_IDS};

#[test]
fn the_kernel_starts_the_capsule_for_every_host_it_serves() {
    let mut kernel = INTEL_EMMC_DEVICE_IDS.to_vec();
    let mut capsule = INTEL_EMMC.to_vec();
    kernel.sort_unstable();
    capsule.sort_unstable();
    assert_eq!(kernel, capsule);
}

#[test]
fn the_kernel_starts_it_for_every_host_the_capsule_takes_and_no_other() {
    let mut kernel = INTEL_SDIO_DEVICE_IDS.to_vec();
    let mut capsule = INTEL_SDIO.to_vec();
    kernel.sort_unstable();
    capsule.sort_unstable();
    assert_eq!(kernel, capsule, "the SDIO hosts neither serves");
    let devices = (0..=0xffffu16)
        .step_by(7)
        .chain(INTEL_EMMC.iter().copied())
        .chain(INTEL_SDIO.iter().copied());
    for device in devices {
        for (vendor, progif) in [(0x8086, 0), (0x8086, 1), (0x8086, 3), (0x1b36, 1), (0x1217, 1)] {
            let capsule = classify(vendor, device, 0x08, 0x05, progif);
            // The inventory's order: an SD host at all, then a listed eMMC.
            let kernel = is_sd_host(vendor, device, 0x08, 0x05, progif);
            assert_eq!(capsule.is_some(), kernel, "{vendor:04x}:{device:04x} if {progif}");
            let emmc = kernel && is_intel_emmc(vendor, device, 0x08, 0x05);
            assert_eq!(capsule == Some(Kind::IntelEmmc), emmc);
        }
    }
    assert!(is_intel_emmc(0x8086, 0x9d2b, 0x08, 0x05), "Sunrise Point");
}
