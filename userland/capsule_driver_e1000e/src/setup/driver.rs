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

//! `Driver` is the live state the server loop holds: the claim and its epoch
//! (config reads and writes need it), the family the bring-up branched on,
//! the BAR0 window and the ring states over the four DMA grants. `release`
//! gives the device back, and the broker frees every grant tied to it.

use nonos_libc::Deadline;

use crate::constants::{Family, MAC_LEN};
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

pub struct Driver {
    pub device_id: u64,
    pub claim_epoch: u64,
    pub pci_device: u16,
    pub family: Family,
    pub rx_ring_device_addr: u64,
    pub tx_ring_device_addr: u64,
    pub regs: Regs,
    pub mac: [u8; MAC_LEN],
    pub rx: RxRing,
    pub tx: TxRing,
    /// The link state last written to the log, so a change is logged once.
    pub link_logged: Option<bool>,
    /// Falls due once frames have sat in flight with the link up and no
    /// completion for TX_HANG_MS; renewed on every sign of life.
    pub tx_watch: Deadline,
    /// Set once `start` has turned Bus Master on over this driver's rings,
    /// after a reset that completed: every descriptor the part has fetched
    /// since is theirs, so the I219 flush may run on them.
    pub on_own_rings: bool,
}

impl Driver {
    /// Quiesce the part as before a reset (the I219 descriptor flush runs on
    /// this driver's own rings, Bus Master still on, once the part has run
    /// on them), turn Bus Master off, reset it, then give the device back
    /// (see `rollback`).
    pub fn release(&self) {
        crate::init::quiesce(self);
        super::rollback::release(self.device_id, self.claim_epoch, &self.regs, Some(self.family));
    }
}
