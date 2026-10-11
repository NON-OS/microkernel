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

//! A driver over host memory and a window, as the setup sequence would
//! leave it before bring-up.

use nonos_devmodel::FakeBar;

use crate::chip::hw_cfg::cfg::NO_SIGNATURE;
use crate::chip::hw_cfg::HwCfg;
use crate::chip::{Chip, Modes};
use crate::constants::MAC_LEN;
use crate::regs::Regs;
use crate::setup::config::Config;
use crate::setup::Driver;

use super::memory::{Memory, RINGS_PHYS};

pub const PCI_DEVICE: u16 = 0x16B4; // TG3PCI_DEVICE_TIGON3_57765
pub const PCIE_CAP: u32 = 0xAC;
pub const REV_57765_A0: u32 = crate::constants::chip::CHIPREV_ID_57765_A0;
/// A 57766 past A0: ASIC_REV_57766 in the top bits.
pub const REV_57766: u32 = 0x5776_6001;
/// A board with no bootcode signature, as tg3 assumes it: a LOM.
pub const LOM: HwCfg = NO_SIGNATURE;

impl Memory {
    pub fn driver(&mut self, bar: &FakeBar, rev: u32, hw: HwCfg) -> Driver {
        let chip = Chip { rev, hw };
        Driver {
            device_id: 9,
            pci_device: PCI_DEVICE,
            fast_ethernet_only: false,
            mmio_grant: 20,
            rings_grant: 21,
            rx_buffer_grant: 22,
            tx_buffer_grant: 23,
            rings_device_addr: RINGS_PHYS,
            cfg: Config { device_id: 9, epoch: 1 },
            pcie_cap: PCIE_CAP,
            regs: Regs::new(bar.base()),
            modes: Modes::new(&chip),
            chip,
            mac: [0u8; MAC_LEN],
            rx: self.rx_queue(),
            tx: self.tx_ring(),
            status: self.status(),
            link: None,
        }
    }
}
