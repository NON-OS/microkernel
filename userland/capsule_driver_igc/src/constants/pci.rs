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

//! The parts this driver binds: every IGC_DEV_ID_* in Linux igc_hw.h, which
//! is also every entry of igc_pci_tbl in igc_main.c. The blank-NVM IDs are
//! bound as Linux binds them; the station address is drawn here, so an empty
//! NVM costs only the auto-read wait in the reset.

pub const INTEL_VENDOR_ID: u16 = 0x8086;

pub const IGC_DEVICE_IDS: &[u16] = &[
    0x15F2, // I225-LM
    0x15F3, // I225-V
    0x15F8, // I225-I
    0x15F7, // I220-V
    0x3100, // I225-K
    0x3101, // I225-K2
    0x3102, // I226-K
    0x5502, // I225-LMVP
    0x5503, // I226-LMVP
    0x0D9F, // I225-IT
    0x125B, // I226-LM
    0x125C, // I226-V
    0x125D, // I226-IT
    0x125E, // I221-V
    0x125F, // I226 blank NVM
    0x15FD, // I225 blank NVM
];

/// igc_probe maps pci_resource_start(pdev, 0): the register file is BAR0.
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
