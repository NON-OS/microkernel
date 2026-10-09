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

use super::held::say;
use crate::admin::hmb::{ends_attempt, ONE_QUEUE_PAIR, QUEUES_TIMEOUT_MS};
use crate::admin::set_features::FID_NUMBER_OF_QUEUES;
use crate::admin::AdminQueue;
use crate::error::NvmeResult;
use crate::regs::Regs;

/// SET FEATURES Number of Queues, one pair, as Linux nvme_set_queue_count
/// asks before creating I/O queues. A controller that refuses it still
/// gives queue 1, so a refusal is carried on from; no answer at all means
/// the controller has stopped, and the attempt fails here rather than at
/// the next command.
pub fn number_of_queues(admin: &mut AdminQueue, regs: Regs, stride: u8) -> NvmeResult<()> {
    let dw = [ONE_QUEUE_PAIR, 0, 0, 0, 0];
    let what = "set number of queues";
    match admin.set_features(regs, stride, FID_NUMBER_OF_QUEUES, dw, what, QUEUES_TIMEOUT_MS) {
        Err(e) if ends_attempt(e) => Err(e),
        Err(_) => {
            say(b"queue count refused; carried on with queue 1", 0);
            Ok(())
        }
        Ok(()) => {
            say(b"queue count set: one I/O queue pair", 0);
            Ok(())
        }
    }
}
