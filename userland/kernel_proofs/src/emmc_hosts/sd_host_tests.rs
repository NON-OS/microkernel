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

use super::emmc::INTEL_EMMC_DEVICE_IDS;
use super::sd_host::{is_sd_host, INTEL_SDIO_DEVICE_IDS};

#[test]
fn card_readers_and_other_vendors_hosts_start_the_capsule() {
    // GLK, BYT, BSW, APL, BXT, SPT, JSL and EHL SD readers; QEMU, O2 Micro.
    for id in [0x31ca, 0x0f16, 0x2296, 0x5aca, 0x0aca, 0x9d2d, 0x4df8, 0x4b48] {
        assert!(is_sd_host(0x8086, id, 0x08, 0x05, 0x01), "{id:04x}");
    }
    assert!(is_sd_host(0x1b36, 0x0007, 0x08, 0x05, 0x01));
    assert!(is_sd_host(0x1217, 0x8620, 0x08, 0x05, 0x00));
    for id in INTEL_EMMC_DEVICE_IDS {
        assert!(is_sd_host(0x8086, id, 0x08, 0x05, 0x01), "eMMC {id:04x}");
    }
}

#[test]
fn sdio_hosts_and_other_functions_do_not() {
    for id in INTEL_SDIO_DEVICE_IDS {
        assert!(!is_sd_host(0x8086, id, 0x08, 0x05, 0x01), "{id:04x}");
        assert!(is_sd_host(0x1217, id, 0x08, 0x05, 0x01), "only Intel's");
    }
    assert!(!is_sd_host(0x1b36, 0x0007, 0x08, 0x05, 0x03), "interface 03h");
    assert!(!is_sd_host(0x1b36, 0x0007, 0x08, 0x80, 0x01), "another peripheral");
    assert!(!is_sd_host(0x1b36, 0x0007, 0x01, 0x05, 0x01), "storage class");
}
