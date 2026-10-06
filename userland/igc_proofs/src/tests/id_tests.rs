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

//! The device table against Linux igc_hw.h, entry for entry.

use crate::constants::pci::{IGC_DEVICE_IDS, INTEL_VENDOR_ID};

/// IGC_DEV_ID_* in igc_hw.h, in the order the header lists them.
const IGC_HW_H: [(u16, &str); 16] = [
    (0x15F2, "I225_LM"),
    (0x15F3, "I225_V"),
    (0x15F8, "I225_I"),
    (0x15F7, "I220_V"),
    (0x3100, "I225_K"),
    (0x3101, "I225_K2"),
    (0x3102, "I226_K"),
    (0x5502, "I225_LMVP"),
    (0x5503, "I226_LMVP"),
    (0x0D9F, "I225_IT"),
    (0x125B, "I226_LM"),
    (0x125C, "I226_V"),
    (0x125D, "I226_IT"),
    (0x125E, "I221_V"),
    (0x125F, "I226_BLANK_NVM"),
    (0x15FD, "I225_BLANK_NVM"),
];

#[test]
fn every_igc_hw_h_id_is_bound_and_nothing_else() {
    let header: Vec<u16> = IGC_HW_H.iter().map(|(id, _)| *id).collect();
    assert_eq!(IGC_DEVICE_IDS, header.as_slice());
    assert_eq!(INTEL_VENDOR_ID, 0x8086);
}

#[test]
fn no_id_is_listed_twice() {
    let mut ids = IGC_DEVICE_IDS.to_vec();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), IGC_DEVICE_IDS.len());
}

/// A different reset and PHY: QEMU's igb (82576), the I210/I211 that igb
/// drives, and the e1000 parts. Binding any of them would run this
/// sequence on the wrong silicon.
#[test]
fn igb_and_e1000_parts_are_not_taken() {
    for id in [0x10C9u16, 0x10E6, 0x10E7, 0x10E8, 0x1533, 0x1539, 0x157B, 0x100E, 0x10D3] {
        assert!(!IGC_DEVICE_IDS.contains(&id), "{id:#06x} is not an igc part");
    }
}
