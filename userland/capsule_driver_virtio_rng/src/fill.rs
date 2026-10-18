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

use super::queue::{Fault, Queue};
use super::transport::Transport;

/// One request, polled to completion, its bytes copied into `out`. No
/// interrupt is involved: the device has Interrupt Disable set (see
/// `setup::pci`). The used ring is the only completion signal.
///
/// The wait ends when `expired` says so, which the caller makes a deadline
/// by the clock, never a count of polls. A request that times
/// out stays the device's: the queue stalls, its buffer is never read, and
/// no request is posted over it until the device is reset.
/// Posting over it used to let its late completion pass for the next
/// request's, so the queue ran one completion behind and two fills could
/// be handed the same bytes.
pub fn fill(
    transport: Transport,
    queue: &mut Queue,
    out: &mut [u8],
    mut expired: impl FnMut() -> bool,
) -> Result<usize, Fault> {
    queue.post()?;
    transport.notify(0);
    loop {
        if let Some(done) = queue.completion() {
            return done.map(|len| queue.take(len, out));
        }
        if expired() {
            queue.stall();
            return Err(Fault::TimedOut);
        }
    }
}
