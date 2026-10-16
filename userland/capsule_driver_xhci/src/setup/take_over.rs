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
use super::marker::marker;
use crate::controller::{halt, legacy_handoff, reset, wait_cnr_clear, ControllerLayout};
use crate::error::XhciResult;

/// Take the controller from firmware and leave it halted and reset, before
/// it is given anything of ours.
pub(super) fn take_over(layout: &ControllerLayout, mmio_va: u64, mmio_len: u64) -> XhciResult<()> {
    // Take the controller from BIOS/SMM before halting or resetting it. On real
    // firmware xHCI is still owned by SMM for boot-keyboard emulation; driving
    // it without this handshake races SMM and can wedge the reset. No-op on a
    // controller that advertises no legacy capability (QEMU).
    legacy_handoff(mmio_va, mmio_len);
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
    Ok(())
}
