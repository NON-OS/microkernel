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
//! the two rings.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_mmio_unmap};

use crate::constants::csr::{CSR0, CSR0_SWR, CSR6};
use crate::constants::MAC_LEN;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

pub struct Driver {
    pub device_id: u64,
    /// The PCI device id: 0x0009 (21140) or 0x0019 (21143).
    pub device: u16,
    pub mmio_grant: u64,
    pub control_grant: u64,
    pub rx_pool_grant: u64,
    pub tx_pool_grant: u64,
    pub regs: Regs,
    pub mac: [u8; MAC_LEN],
    pub rx: RxRing,
    pub tx: TxRing,
}

impl Driver {
    /// Stop the part, then give every grant back in the reverse order setup
    /// took them. The stop comes first: a part left running would go on
    /// writing frames into pages the broker is about to hand to someone else.
    /// A release refused because the grant is already gone is harmless.
    pub fn release(&self) {
        self.regs.set_csr(CSR6, 0);
        self.regs.set_csr(CSR0, CSR0_SWR);
        let _ = mk_dma_unmap(self.tx_pool_grant);
        let _ = mk_dma_unmap(self.rx_pool_grant);
        let _ = mk_dma_unmap(self.control_grant);
        let _ = mk_mmio_unmap(self.mmio_grant);
        let _ = mk_device_release(self.device_id);
    }
}
