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

use nonos_libc::{mk_irq_bind, IrqBindOut, MK_IRQ_BIND_MSIX};

use crate::constants::REG_INTMS;
use crate::discover::Found;
use crate::regs::Regs;

pub fn bind(dev: Found, claim_epoch: u64) -> IrqBindOut {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    let r = mk_irq_bind(dev.device_id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out);
    if r < 0 {
        // MSI-X binding is best effort. The driver polls every admin and I/O
        // completion (admin/queue/wait.rs, nvm/wait.rs) and never waits on the
        // interrupt, so a failed bind is not fatal. Continue in polling mode
        // with a zero grant, which BrokerHandles::drop unbinds harmlessly.
        return IrqBindOut { grant_id: 0, vector: 0 };
    }
    out
}

/// We poll every completion. Without MSI-X the controller would signal on
/// its legacy pin or MSI, which nothing services: mask them all, so a level
/// interrupt cannot stay asserted. INTMS is off limits under MSI-X (NVMe
/// 3.1.4), so it is written only when the bind failed (`grant_id` zero).
pub fn mask_unbound(regs: Regs, grant_id: u64) {
    if grant_id == 0 {
        // SAFETY: REG_INTMS lies in the controller's register block, which
        // bring-up checked fits the mapped BAR before any register access.
        unsafe { regs.w32(REG_INTMS, u32::MAX) };
    }
}
