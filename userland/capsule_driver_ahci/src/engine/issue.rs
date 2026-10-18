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

use super::cause::Cause;
use super::completion::wait_done;
use super::port::Port;
use super::verdict::SLOT0;
use crate::clock::Deadline;
use crate::constants::regs::{PORT_CI, PORT_IS, PORT_SACT, PORT_TFD};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Issue the command built in slot 0 and wait for it, all inside the port's
/// command budget, so the reply reaches the kernel before it stops waiting
/// (`budget::REQUEST_MS`). A port not ready for it is given no command
/// (`prepare`). A command that fails leaves PxTFD in `last_tfd`, and the
/// port is recovered by how it failed (`Cause::of`): an error the drive
/// ended it with, or a drive that may still be running it, whose failure
/// is the HBA's (`HbaFailed`). A port the recovery finds wedged is marked
/// so.
pub(super) fn run(port: &mut Port, regs: Regs) -> AhciResult<()> {
    let base = port.base;
    // SAFETY: the port's register block lies inside the mapped ABAR.
    let read = |off: u32| unsafe { regs.r32(base + off) };
    let deadline = Deadline::after_ms(port.command_ms);
    if let Err(e) = super::prepare::prepare(port, regs) {
        port.last_tfd = read(PORT_TFD);
        return failed(port, e, Ok(()));
    }
    // The HBA DMA-reads the command list and table only after it sees the
    // command-issue bit set. Order those stores ahead of the CI write so it
    // never fetches a stale command header, FIS, or PRDT.
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    // SAFETY: as above.
    unsafe { regs.w32(base + PORT_CI, SLOT0) };
    let Err(e) = wait_done(read, || deadline.expired()) else {
        return Ok(());
    };
    // The disk's own status and error, and how the command left the port,
    // read before recovery clears them.
    port.last_tfd = read(PORT_TFD);
    let cause = Cause::of(e, read(PORT_IS), read(PORT_CI), read(PORT_SACT));
    let recovered = super::recover::recover(port, regs, cause);
    // Only a command the drive failed is the disk's error: one the HBA
    // failed, or the link changed under, is not, whatever PxTFD holds.
    let abandoned = e == AhciError::CommandFailed && cause == Cause::Abandoned;
    failed(port, if abandoned { AhciError::HbaFailed } else { e }, recovered)
}

/// End a request that failed with `e`, after recovery ended in `recovered`.
fn failed(port: &mut Port, e: AhciError, recovered: AhciResult<()>) -> AhciResult<()> {
    if e == AhciError::Wedged || recovered == Err(AhciError::Wedged) {
        port.wedged = true;
    }
    Err(e)
}
