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

//! One request, from post to status.

use super::error::{ended, BlkError};
use super::read_seq::read_seq;
use super::rearm::rearm;
use super::slice::slice;
use super::wait::wait;
use crate::clock;
use crate::queue::{Direction, Queue};
use crate::transport::Transport;

/// Post the request, tell the device, wait for its answer by the clock,
/// and read its status. A request still the device's when the wait ends
/// (it timed out, or the wait itself failed) stalls the queue: nothing is
/// posted over it, and the server resets the device before the next
/// request, which so never waits it out (plan/50 C1).
pub fn submit(
    transport: Transport,
    queue: &mut Queue,
    irq_grant: u64,
    dir: Direction,
    lba: u64,
    nsectors: u32,
) -> Result<(), BlkError> {
    /*
     * The sequence is read, and the line lowered and unmasked (see
     * `rearm`), before the device is told. A device that completes at once
     * raises its interrupt between a snapshot taken after the notify and
     * the wait, which then waits for a second one that never comes; and
     * an unmask after it leaves a shared line masked with the completion
     * still to come. An interrupt the unmask lets through ends the first
     * sleep, and the loop rearms.
     */
    let mut seq = read_seq(irq_grant)?;
    rearm(transport, irq_grant)?;
    queue.post(dir, lba, nsectors)?;
    transport.notify(0);
    let budget = clock::budget(dir.budget_ms());
    let answered = wait(
        || queue.completion(),
        |ms| slice(transport, irq_grant, &mut seq, ms),
        || budget.left(clock::uptime_ms()),
    );
    if let Err(fault) = answered {
        if queue.outstanding() {
            queue.stall(fault);
        }
        return Err(fault.into());
    }
    let status = queue.status();
    rearm(transport, irq_grant)?;
    let end = ended(status);
    if end.is_ok() {
        queue.note_served();
    }
    end
}
