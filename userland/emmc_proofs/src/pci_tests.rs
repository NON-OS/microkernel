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

//! Which PCI functions are served as the internal disk.

use crate::emmc::pci::*;

#[test]
fn intel_emmc_hosts_are_served_first() {
    for &d in INTEL_EMMC {
        for progif in [0, 1, 2] {
            assert_eq!(classify(0x8086, d, 0x08, 0x05, progif), Some(Kind::IntelEmmc), "{d:#x}");
        }
    }
    assert!(Kind::IntelEmmc < Kind::Generic);
    assert_eq!(classify(0x8086, 0x31cc, 0x08, 0x05, 0x01), Some(Kind::IntelEmmc));
}

#[test]
fn intel_sd_readers_and_sdio_hosts_are_never_served() {
    for &d in INTEL_NOT_EMMC {
        assert_eq!(classify(0x8086, d, 0x08, 0x05, 0x01), None, "{d:#x}");
    }
    for d in [
        0x31ca, 0x31d0, 0x0f16, 0x0f15, 0x2296, 0x2295, 0x5aca, 0x5ad0, 0x0aca, 0x0ad0, 0x4df8,
        0x4b48,
    ] {
        assert_eq!(classify(0x8086, d, 0x08, 0x05, 0x01), None, "{d:#x}");
    }
}

#[test]
fn the_lists_do_not_overlap() {
    for d in INTEL_EMMC {
        assert!(!INTEL_NOT_EMMC.contains(d), "{d:#x}");
    }
}

#[test]
fn the_coordinated_id_list_is_covered() {
    for d in [
        0x0f14, 0x0f50, 0x2294, 0x0acc, 0x1aa8, 0x5acc, 0x31cc, 0x9dc4, 0x34c4, 0x18db, 0x4b47,
        0x4dc4,
    ] {
        assert!(INTEL_EMMC.contains(&d), "{d:#x}");
    }
}

#[test]
fn other_sdhci_hosts_are_generic_and_the_rest_are_not_hosts() {
    assert_eq!(classify(0x1217, 0x8620, 0x08, 0x05, 0x01), Some(Kind::Generic));
    assert_eq!(classify(0x8086, 0x1234, 0x08, 0x05, 0x01), Some(Kind::Generic));
    assert_eq!(classify(0x8086, 0x31cc, 0x08, 0x05, 0x03), None);
    assert_eq!(classify(0x8086, 0x31cc, 0x08, 0x06, 0x01), None);
    assert_eq!(classify(0x8086, 0x31cc, 0x01, 0x05, 0x01), None);
    assert_eq!(classify(0x8086, 0x31cc, 0x01, 0x06, 0x01), None);
}

#[test]
fn slot_information_names_the_bar() {
    assert_eq!(first_bar(0x00), 0);
    assert_eq!(first_bar(0x10), 0);
    assert_eq!(first_bar(0x02), 2);
    assert_eq!(first_bar(0x05), 5);
    assert_eq!(first_bar(0x06), 0);
    assert_eq!(first_bar(0x07), 0);
}
