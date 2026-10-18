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
use crate::controller::{
    issue_noop_and_wait, power_all_ports, program_command_ring, program_dcbaa, program_event_ring,
    start, wait_hc_running,
};
use crate::error::XhciResult;
use crate::regs::runtime::imod_program;

/// Hand the halted controller its DCBAA, command ring and event ring, start
/// it, and see it answer a NOOP: a revival, whose bus mastering has stayed
/// on since bring-up. A failure leaves `driver` to be dropped, which stops
/// the controller before any of this is freed.
pub(super) fn bring_up(driver: &mut Driver) -> XhciResult<()> {
    program(driver);
    start_checked(driver)
}

/// Give the halted controller its DCBAA, whose entry 0 names the scratchpad
/// array, its command ring and its event ring, bus mastering already on:
/// the ERSTBA write starts the event ring, and the controller fetches its
/// segment table then (xHCI 1.2 section 5.5.2.3.2). Nothing here refuses.
pub(super) fn program(driver: &mut Driver) {
    let d = driver;
    let (op, intr) = (d.layout.op_base, d.layout.primary_intr_base);
    d.event_ring.set_op_base(op);
    program_dcbaa(op, &d.dcbaa, d.layout.max_slots);
    marker(b"[driver_xhci] dcbaa ok\n");
    program_command_ring(op, &mut d.command_ring);
    marker(b"[driver_xhci] cmd ring ok\n");
    imod_program(intr, 4000, 0);
    program_event_ring(intr, &d.event_ring);
    marker(b"[driver_xhci] evt ring ok\n");
}

/// Set RUN on the programmed controller, see it running and answer a NOOP.
pub(super) fn start_checked(driver: &mut Driver) -> XhciResult<()> {
    let d = driver;
    let (op, intr) = (d.layout.op_base, d.layout.primary_intr_base);
    start(op);
    wait_hc_running(op)?;
    marker(b"[driver_xhci] running\n");
    power_all_ports(op, d.layout.max_ports);
    issue_noop_and_wait(d.layout.doorbell_base, intr, &mut d.command_ring, &mut d.event_ring)?;
    marker(b"[driver_xhci] noop ok\n");
    marker(b"[driver_xhci] endpoint driver.xhci0 ready\n");
    Ok(())
}
