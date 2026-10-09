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

use super::emmc::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};

#[test]
fn every_listed_intel_emmc_host_is_taken() {
    for id in INTEL_EMMC_DEVICE_IDS {
        assert!(is_intel_emmc(0x8086, id, 0x08, 0x05), "{id:04x}");
    }
    assert!(is_intel_emmc(0x8086, 0x31cc, 0x08, 0x05), "Gemini Lake, the HP 15s-fq0");
}

#[test]
fn sd_card_and_sdio_hosts_are_not_a_disk() {
    for id in [0x31ca, 0x31d0, 0x0f16, 0x0f15, 0x2296, 0x2295, 0x5aca, 0x5ad0, 0x0aca, 0x0ad0] {
        assert!(!is_intel_emmc(0x8086, id, 0x08, 0x05), "{id:04x}");
    }
}

#[test]
fn the_class_and_vendor_must_match_too() {
    assert!(!is_intel_emmc(0x8086, 0x31cc, 0x01, 0x05));
    assert!(!is_intel_emmc(0x8086, 0x31cc, 0x08, 0x80));
    assert!(!is_intel_emmc(0x1022, 0x31cc, 0x08, 0x05));
}
