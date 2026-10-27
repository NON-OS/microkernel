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

//! A controller brought up on the fake the way bring-up does it: admin
//! queue programmed, enabled, and queue 1 created by the driver's own
//! nvm::bring_up.

use std::cell::RefCell;
use std::rc::Rc;

use super::{info, install, register, registers, Fake};
use crate::admin::AdminQueue;
use crate::constants::REG_INTMS;
use crate::handles::BrokerHandles;
use crate::nvm::{bring_up, IoQueue, NamespaceGeometry};
use crate::regs::Regs;
use crate::setup::Device;

pub(crate) struct Up {
    pub fake: Rc<RefCell<Fake>>,
    pub regs: Regs,
    pub device: Device,
    pub admin: AdminQueue,
    pub io: IoQueue,
}

/// MSI-X unbound (irq grant 0); MMIO grant 0x10 of device 7, epoch 1.
pub(crate) fn up() -> Up {
    up_on(0)
}

/// `up` with MSI-X grant `irq` bound (0 for none). Without one, the enable
/// must leave INTMS masking every vector, as nothing services them; under
/// MSI-X it must leave INTMS alone (NVMe 1.4, 3.1.4).
pub(crate) fn up_on(irq: u64) -> Up {
    let regs = registers();
    let fake = install(regs);
    let mut device = Device::new(BrokerHandles::new(7, 1, 0x10, 0, irq), regs, info());
    let mut admin = AdminQueue::allocate(7, 1).expect("host memory");
    admin.program_registers(regs);
    device.enable().expect("the fake comes ready");
    let masked = if irq == 0 { u32::MAX } else { 0 };
    assert_eq!(register(regs, REG_INTMS), masked, "INTMS after the enable, grant {irq}");
    let g =
        NamespaceGeometry { nsid: 1, capacity_sectors: 1 << 20, lba_size: 512, max_sectors: 64 };
    let io = bring_up(7, 1, regs, info(), &mut admin, g).expect("queue 1 created");
    Up { fake, regs, device, admin, io }
}
