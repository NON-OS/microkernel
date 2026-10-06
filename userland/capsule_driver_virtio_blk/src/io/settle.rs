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

//! Waiting out a request given up on, before the slot is used again.

use super::error::BlkError;
use super::read_seq::read_seq;
use super::rearm::rearm;
use super::wait_used::wait_used;
use crate::queue::Queue;
use crate::transport::Transport;

/*
 * A request whose wait ran out is still the device's: it may yet read the
 * data buffer or write into it. Nothing touches the buffer or posts again
 * until the device has answered it; if it does not answer within another
 * request's budget, the new request is refused and the buffer left alone.
 */
pub fn settle(transport: Transport, queue: &mut Queue, irq_grant: u64) -> Result<(), BlkError> {
    if queue.idle() {
        return Ok(());
    }
    let seq = read_seq(irq_grant)?;
    rearm(transport, irq_grant)?;
    wait_used(transport, queue, irq_grant, seq)
}
