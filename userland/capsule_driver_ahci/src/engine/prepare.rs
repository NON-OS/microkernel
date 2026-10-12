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

//! Making sure a port can take the command built in slot 0 before it is
//! issued. Each wait has its own short bound, so however the port answers,
//! the command keeps most of its budget (`budget::PRE_ISSUE_MS`).

use super::cause::Cause;
use super::holdup::{holdup, in_window};
use super::link::established::{link_state, LinkState};
use super::port::Port;
use super::ready::wait_ready;
use crate::clock::Deadline;
use crate::constants::regs::PORT_SSTS;
use crate::constants::timing::READY_MS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Get the port ready to take a command, by its link (PxSSTS.DET).
///
/// With no device (DET 0), the PHY offline (4), or any value but 1 and 3,
/// the request is refused at once: the disk is gone, and a COMRESET would
/// find nothing. With a device but no link (1), the link is reset, though
/// no more than once a reset window (`Port::reset_window`): inside one the
/// request is refused at once, and the COMRESET sent is left to work. With
/// the link up (3), what holds the port (`holdup`) decides. A port that
/// does not take a command is recovered once and given READY_MS more; one
/// that still does not is given no command.
pub(super) fn prepare(port: &mut Port, regs: Regs) -> AhciResult<()> {
    let base = port.base;
    // SAFETY: the port's register block lies inside the mapped ABAR.
    let read = |off: u32| unsafe { regs.r32(base + off) };
    let cause = match link_state(read(PORT_SSTS)) {
        LinkState::Up => match holdup(port, regs)? {
            Some(cause) => cause,
            None => return Ok(()),
        },
        LinkState::Negotiating if !in_window(port) => Cause::Unlinked,
        _ => return Err(AhciError::NoDisk),
    };
    super::recover::recover(port, regs, cause)?;
    let second = Deadline::after_ms(READY_MS);
    wait_ready(read, || second.expired())
}
