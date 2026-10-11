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

//! The live state the server loop holds: one grant per broker primitive and
//! both queues.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_mmio_unmap};

use crate::constants::cmd::{QUIESCE_DEV, RESET_DEV};
use crate::constants::MAC_LEN;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

pub struct Driver {
    pub device_id: u64,
    pub pt_grant: u64,
    pub vd_grant: u64,
    pub control_grant: u64,
    pub rx_pool_grant: u64,
    pub tx_pool_grant: u64,
    /// The control grant, as the device addresses it and as the driver does.
    pub control_dev: u64,
    pub control_va: u64,
    pub regs: Regs,
    pub mac: [u8; MAC_LEN],
    pub rx: RxRing,
    pub tx: TxRing,
}

impl Driver {
    /// Quiesce and reset the device and clear its shared address, then give
    /// every grant back in the reverse order setup took them. The device
    /// goes first: left active it would go on writing completions and frames
    /// into pages the broker is about to hand to someone else.
    pub fn release(&self) {
        self.regs.command(QUIESCE_DEV);
        self.regs.command(RESET_DEV);
        self.regs.set_shared(0);
        let _ = mk_dma_unmap(self.tx_pool_grant);
        let _ = mk_dma_unmap(self.rx_pool_grant);
        let _ = mk_dma_unmap(self.control_grant);
        let _ = mk_mmio_unmap(self.vd_grant);
        let _ = mk_mmio_unmap(self.pt_grant);
        let _ = mk_device_release(self.device_id);
    }
}
