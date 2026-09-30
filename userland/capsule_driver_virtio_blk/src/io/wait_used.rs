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
use super::error::BlkError;
use super::wait_slice::{wait_slice, Slice};
use crate::queue::Queue;
use crate::regs::Regs;

/// The whole-request budget. It is counted in slices actually spent, wait or
/// not: a shared interrupt line that keeps advancing the sequence must consume
/// budget on every pass, or a request the device never completes pins the
/// server in this loop forever.
const MAX_SLICES: u32 = 50;
const MAX_PASSES: u32 = 5000;

/// Block on the interrupt instead of yield-polling. The old loop spun up to
/// 200k yields per request; every disk read then cycled the whole run queue
/// for the request's full latency, and on one CPU the rest of the system paid
/// for each sector. Sliced waits keep the same total budget and the used-ring
/// check on every wake covers a completion whose interrupt was suppressed or
/// already consumed.
///
/// Two bounds, each safe alone. Waits that sleep out their whole slice count
/// toward the time budget, so an idle device gets the full five seconds and no
/// more. Every pass counts toward the iteration guard, so a shared line whose
/// sequence keeps advancing cannot hold the loop open forever, and a healthy
/// request finishes thousands of iterations under it.
pub(super) fn wait_used(
    regs: Regs,
    queue: &mut Queue,
    irq_grant: u64,
    mut seq: u64,
) -> Result<(), BlkError> {
    let mut timed_out_slices = 0u32;
    let mut passes = 0u32;
    loop {
        let observed = queue.used_idx();
        if observed.wrapping_sub(queue.last_used) != 0 {
            queue.last_used = observed;
            return Ok(());
        }
        passes = passes.wrapping_add(1);
        if passes > MAX_PASSES {
            queue.last_used = queue.used_idx();
            return Err(BlkError::Timeout);
        }
        match wait_slice(regs, irq_grant, seq)? {
            Slice::Woke(next) => seq = next,
            Slice::TimedOut => {
                timed_out_slices = timed_out_slices.wrapping_add(1);
                if timed_out_slices > MAX_SLICES {
                    queue.last_used = queue.used_idx();
                    return Err(BlkError::Timeout);
                }
            }
            Slice::Failed => {
                queue.last_used = queue.used_idx();
                return Err(BlkError::Io);
            }
        }
    }
}
