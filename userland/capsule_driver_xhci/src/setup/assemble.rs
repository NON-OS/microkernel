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

use super::driver::Driver;
use super::marker::marker;
use crate::controller::park::IrqWait;
use crate::controller::{alloc_dcbaa, ControllerLayout, Scratchpads};
use crate::dma::DmaPool;
use crate::error::{XhciError, XhciResult};
use crate::handles::BrokerHandles;
use crate::regs::cap::ac64;
use crate::regs::op::{page_bytes, pagesize_read};
use crate::rings::{command::CommandRing, event::EventRing};
use crate::slots::SlotTable;

/// Allocate everything the halted controller will be given, and nothing
/// more: no register names any of it yet, so a failure here frees memory
/// the controller never heard of.
pub(super) fn assemble(
    handles: BrokerHandles,
    layout: ControllerLayout,
    claim_epoch: u64,
) -> XhciResult<Driver> {
    let ac64 = ac64(handles.mmio_user_va());
    let dma_pool = DmaPool::new(handles.device_id(), claim_epoch).with_ac64(ac64);
    let page = page_bytes(pagesize_read(layout.op_base)).ok_or(XhciError::ControllerUnsupported)?;
    let scratchpads = Scratchpads::allocate(&dma_pool, layout.max_scratchpad, page)?;
    marker(b"[driver_xhci] scratchpads ok\n");
    let dcbaa = alloc_dcbaa(&dma_pool, layout.max_slots, scratchpads.array_phys())?;
    let command_ring = CommandRing::new(&dma_pool)?;
    let mut event_ring = EventRing::new(&dma_pool)?;
    event_ring.irq = IrqWait::new(handles.irq_grant_id());
    Ok(Driver {
        handles,
        dcbaa,
        scratchpads,
        dma_pool,
        command_ring,
        event_ring,
        layout,
        slots: SlotTable::new(),
        dead: false,
        generation: 0,
        revivals: 0,
    })
}
