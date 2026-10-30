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

//! Letting the reader go (C2), as a failed bring-up does and as the end of
//! the driver would. A powered card has its clock, outputs and power taken
//! away while the command engine still runs; both engines are stopped and
//! the DMA engine reset; the reader raises no more events; and the PCI
//! command is written with Bus Master clear. The claim's release follows,
//! as Driver's last field drops, and it is the real guard: the kernel
//! clears Bus Master with a read-back before it drains the DMA buffers.
//! The driver's own clear is defence in depth, taking effect once the
//! kernel stores exactly the bits written.

use nonos_libc::{mk_pci_config_write, MK_PCI_CFG_COMMAND};

use super::driver::Driver;
use super::master::QUIET;
use crate::card::power_off;
use crate::engine::stop;
use crate::regs::host::BIER;

impl Drop for Driver {
    fn drop(&mut self) {
        if self.card_on {
            let _ = power_off(self);
        }
        stop(self);
        self.regs.w32(BIER, 0);
        let (id, epoch) = (self.claim.device_id, self.claim.epoch);
        let _ = mk_pci_config_write(id, epoch, MK_PCI_CFG_COMMAND, QUIET);
    }
}
