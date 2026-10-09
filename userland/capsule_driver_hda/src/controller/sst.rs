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
//! Intel's Smart Sound Technology engines, which are not HD Audio controllers.
//!
//! Before Skylake, Intel's audio DSP was a PCI function of its own, beside
//! the HD Audio controller or in place of it: the Haswell and Broadwell ULT
//! ADSP (pci.ids "Lynx Point-LP" and "Wildcat Point-LP Smart Sound
//! Technology Controller", 8086:9c36 and 8086:9cb6) and the Atom LPE engines
//! (Linux pci_ids.h PCI_DEVICE_ID_INTEL_SST_BYT 0x0f28, SST_BSW 0x22a8,
//! SST_TNG 0x119a). Their BAR0 is the DSP's shim, not the HD Audio register
//! set, and the speakers hang off the DSP over I2S. snd_hda_intel does not
//! list them; Linux runs them with catpt, the Atom SST driver or SOF. One
//! that reports class 0x0401 passed the class test here and was brought up
//! as an HD Audio controller, a reset that cannot work on a register set
//! that is not HD Audio's, so the driver would give up instead of saying the
//! machine needs the DSP.

use crate::constants::PCI_VENDOR_INTEL;

const SST: &[u16] = &[0x9c36, 0x9cb6, 0x0f28, 0x22a8, 0x119a];

pub fn intel_sst(vendor: u16, device: u16) -> bool {
    vendor == PCI_VENDOR_INTEL && SST.contains(&device)
}
