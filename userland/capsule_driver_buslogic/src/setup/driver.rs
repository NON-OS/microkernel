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

//! What the capsule holds for its life: the units it serves over the
//! adapter, and what it gives back if it leaves.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_pio_release};

use crate::constants::regs::{CTL_HARD_RESET, REG_CONTROL};
use crate::hba::BtHba;
use crate::port::Port;
use crate::Units;

pub struct Driver {
    pub units: Units<BtHba>,
    pub device_id: u64,
    pub pio_grant: u64,
    pub dma_grant: u64,
}

impl Driver {
    /// A hard reset, so the adapter stops touching the mailboxes and the
    /// buffers, then the DMA grant, the port grant and the claim given back.
    pub fn release(&self) {
        abandon(&self.units.hba.port, self.device_id, self.pio_grant, Some(self.dma_grant));
    }
}

/// The same, for a bring-up that fails after the ports are granted.
pub fn abandon(port: &Port, device_id: u64, pio_grant: u64, dma_grant: Option<u64>) {
    port.write8(REG_CONTROL, CTL_HARD_RESET);
    if let Some(g) = dma_grant {
        let _ = mk_dma_unmap(g);
    }
    let _ = mk_pio_release(pio_grant);
    let _ = mk_device_release(device_id);
}
