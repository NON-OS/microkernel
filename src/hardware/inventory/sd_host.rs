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

//! Every other SD Host Controller, class 08h subclass 05h: a card reader,
//! or an embedded slot the eMMC list does not name. It starts the SD host
//! controller capsule too, which serves a fixed slot's eMMC or, on a
//! machine with none, a reader's card. Intel's SDIO hosts carry the board's
//! Wi-Fi or Bluetooth, never a disk (recalled), and are left out by id; the
//! ids are unverified until re-sourced from Intel's datasheets (L56).
//!
//! Pure, with no imports, so host proofs can include it by path.

const INTEL: u16 = 0x8086;

/// Bay Trail, Braswell, Broxton, Apollo Lake, Gemini Lake, Sunrise Point.
pub const INTEL_SDIO_DEVICE_IDS: [u16; 6] = [0x0f15, 0x2295, 0x0ad0, 0x5ad0, 0x31d0, 0x9d2c];

const PCI_CLASS_SYSTEM: u8 = 0x08;
const PCI_SUBCLASS_SDHCI: u8 = 0x05;
/// Interface 00h (no DMA), 01h (DMA) and 02h (vendor specific): the
/// capsule decides on ADMA2 from the host's own capabilities.
const PROGIF_MAX: u8 = 0x02;

/// Whether a function is an SD host the capsule may serve. Intel eMMC
/// hosts are SD hosts too; the inventory names them first.
pub const fn is_sd_host(vendor: u16, device: u16, class: u8, subclass: u8, progif: u8) -> bool {
    if class != PCI_CLASS_SYSTEM || subclass != PCI_SUBCLASS_SDHCI || progif > PROGIF_MAX {
        return false;
    }
    if vendor != INTEL {
        return true;
    }
    let mut i = 0;
    while i < INTEL_SDIO_DEVICE_IDS.len() {
        if INTEL_SDIO_DEVICE_IDS[i] == device {
            return false;
        }
        i += 1;
    }
    true
}
