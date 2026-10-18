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
use super::next_completion::CommandCompletion;
use super::settle_abort::settle_abort;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::{Abort, CommandRing};
use crate::rings::event::EventRing;

/// Abort the command at `issued` and consume what that leaves. Unless its
/// own completion came, the controller may never have taken the TRB up: a
/// ring that was not running, or one stopped before it fetched it, whose
/// Command Ring Stopped names the TRB itself (xHCI 1.2 section 4.6.1.2).
/// The next doorbell would run it, so it becomes a No Op (Linux
/// `trb_to_noop`), and stays queued until a later command completes.
pub(super) fn give_up(
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    issued: u64,
) -> XhciResult<CommandCompletion> {
    let aborted = cmd_ring.abort();
    // CRR never dropped, or the controller failed: it is treated as dead,
    // and a revival's reset makes it forget the ring.
    if aborted == Abort::Stuck {
        return Err(XhciError::ControllerDead);
    }
    let got = settle_abort(intr_base, evt_ring, issued, aborted == Abort::Stopped);
    match got {
        Err(XhciError::CommandCompletionTimeout) => cmd_ring.cancel(issued),
        _ => cmd_ring.retire(),
    }
    got
}
