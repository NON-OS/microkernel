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

//! `Driver` is the live state the server loop holds: one grant per broker
//! primitive, what the part is, the words tg3 keeps of its mode registers,
//! the rings, the drawn station address and the link last logged.
//! `release` gives the grants back in the reverse of the order setup took
//! them; the broker scrubs each DMA grant as it goes.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_mmio_unmap};

use crate::chip::{Chip, Modes};
use crate::constants::MAC_LEN;
use crate::link::LinkState;
use crate::queue::{RxQueue, Status, TxRing};
use crate::regs::Regs;

use super::config::Config;

pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub fast_ethernet_only: bool,
    pub mmio_grant: u64,
    pub rings_grant: u64,
    pub rx_buffer_grant: u64,
    pub tx_buffer_grant: u64,
    /// Device address of the rings grant; each ring's is an offset into it.
    pub rings_device_addr: u64,
    pub cfg: Config,
    pub pcie_cap: u32,
    pub regs: Regs,
    pub chip: Chip,
    pub modes: Modes,
    pub mac: [u8; MAC_LEN],
    pub rx: RxQueue,
    pub tx: TxRing,
    pub status: Status,
    /// What the last "tg3: link" line said.
    pub link: Option<LinkState>,
}

impl Driver {
    /// Each broker call is best-effort: an `EINVAL` from a grant already
    /// dropped is harmless, the broker having revoked it.
    pub fn release(&self) {
        let _ = mk_dma_unmap(self.tx_buffer_grant);
        let _ = mk_dma_unmap(self.rx_buffer_grant);
        let _ = mk_dma_unmap(self.rings_grant);
        let _ = mk_mmio_unmap(self.mmio_grant);
        let _ = mk_device_release(self.device_id);
    }
}
