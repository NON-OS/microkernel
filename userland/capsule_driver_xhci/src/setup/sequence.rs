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
use nonos_libc::mk_device_release;

use super::marker::marker;
use super::{
    assemble::assemble, claim::claim, driver::Driver, irq_bind::irq_bind, layout::read_layout,
    mmio_map::mmio_map, pci::enable_bus_master,
};
use crate::controller::{
    halt, issue_noop_and_wait, legacy_handoff, power_all_ports, program_command_ring,
    program_dcbaa, program_event_ring, reset, set_irq_grant, start, wait_cnr_clear,
    wait_hc_running, Scratchpads,
};
use crate::discover::Found;
use crate::dma::DmaPool;
use crate::error::{XhciError, XhciResult};
use crate::handles::BrokerHandles;
use crate::regs::cap::ac64;
use crate::regs::op::{page_bytes, pagesize_read};
use crate::regs::runtime::imod_program;
use crate::rings::{command::CommandRing, event::EventRing};

/// One bring-up attempt on the controller discovery found. Before
/// `BrokerHandles` exists a failure releases the claim itself; after it, the
/// handles' Drop unbinds, unmaps and releases, and the rings and pool drop
/// their DMA first, so the next attempt can claim the controller again.
pub fn run(dev: Found) -> XhciResult<Driver> {
    let claim_epoch = claim(dev.device_id)?;
    if let Err(e) = enable_bus_master(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio_map(dev.device_id, claim_epoch, dev.bar0_size)?;
    let irq = irq_bind(dev, claim_epoch);
    set_irq_grant(irq.grant_id);
    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let layout = read_layout(&handles, mmio.length)?;
    // Take the controller from BIOS/SMM before halting or resetting it. On real
    // firmware xHCI is still owned by SMM for boot-keyboard emulation; driving
    // it without this handshake races SMM and can wedge the reset. No-op on a
    // controller that advertises no legacy capability (QEMU).
    legacy_handoff(mmio.user_va, mmio.length);
    marker(b"[driver_xhci] legacy handoff ok\n");
    // No operational register may be written while CNR is set (xHCI 1.2
    // section 5.4.2); straight after the handoff firmware may still have the
    // controller coming up. Linux waits here too before its halt.
    wait_cnr_clear(layout.op_base)?;
    halt(layout.op_base)?;
    reset(layout.op_base)?;
    marker(b"[driver_xhci] reset ok\n");
    wait_cnr_clear(layout.op_base)?;
    marker(b"[driver_xhci] cnr cleared\n");
    let dma_pool = DmaPool::new(dev.device_id, claim_epoch).with_ac64(ac64(mmio.user_va));
    let page = page_bytes(pagesize_read(layout.op_base)).ok_or(XhciError::ControllerUnsupported)?;
    let scratchpads = Scratchpads::allocate(&dma_pool, layout.max_scratchpad, page)?;
    marker(b"[driver_xhci] scratchpads ok\n");
    let dcbaa =
        program_dcbaa(&dma_pool, layout.op_base, layout.max_slots, scratchpads.array_phys())?;
    marker(b"[driver_xhci] dcbaa ok\n");
    let mut command_ring = CommandRing::new(&dma_pool)?;
    program_command_ring(layout.op_base, &mut command_ring);
    marker(b"[driver_xhci] cmd ring ok\n");
    let mut event_ring = EventRing::new(&dma_pool)?;
    imod_program(layout.primary_intr_base, 4000, 0);
    program_event_ring(layout.primary_intr_base, &event_ring);
    marker(b"[driver_xhci] evt ring ok\n");
    start(layout.op_base);
    wait_hc_running(layout.op_base)?;
    marker(b"[driver_xhci] running\n");
    power_all_ports(layout.op_base, layout.max_ports);
    issue_noop_and_wait(
        layout.doorbell_base,
        layout.primary_intr_base,
        &mut command_ring,
        &mut event_ring,
    )?;
    marker(b"[driver_xhci] noop ok\n");
    marker(b"[driver_xhci] endpoint driver.xhci0 ready\n");
    Ok(assemble(handles, dcbaa, scratchpads, dma_pool, command_ring, event_ring, layout))
}
