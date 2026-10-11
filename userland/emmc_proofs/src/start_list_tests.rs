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

//! The kernel starts driver.sdhci0 for the Intel hosts its inventory lists
//! (src/hardware/inventory/emmc.rs); the capsule serves the ones its own
//! table lists. A host on the second and not the first would never get its
//! driver started: 8086:9d2b was one.

use crate::emmc::pci::{INTEL_EMMC, INTEL_NOT_EMMC};
use crate::kernel_inventory::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};

#[test]
fn the_kernel_starts_the_capsule_for_every_host_it_serves() {
    let mut kernel = INTEL_EMMC_DEVICE_IDS.to_vec();
    let mut capsule = INTEL_EMMC.to_vec();
    kernel.sort_unstable();
    capsule.sort_unstable();
    assert_eq!(kernel, capsule);
}

#[test]
fn no_card_reader_or_sdio_host_starts_it() {
    for &id in INTEL_NOT_EMMC {
        assert!(!is_intel_emmc(0x8086, id, 0x08, 0x05), "{id:04x}");
    }
    assert!(is_intel_emmc(0x8086, 0x9d2b, 0x08, 0x05), "Sunrise Point");
}
