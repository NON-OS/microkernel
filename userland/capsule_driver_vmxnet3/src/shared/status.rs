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

//! What the device writes back into the shared area: each queue's status
//! after GET_QUEUE_STATUS, and the event cause word.

use core::ptr::read_volatile;

use crate::constants::cmd::ECR_QUEUE_ERRORS;
use crate::constants::layout::{RXQ_DESC_OFF, TXQ_DESC_OFF};

/// Vmxnet3_QueueStatus follows ctrl (16) and conf (64) in a queue descriptor.
const STATUS: usize = 80;
const ECR: usize = 696;

/// Whether the device has stopped a queue or flagged an error on one.
pub fn queues_failed(control_va: u64) -> bool {
    let at = |off: usize| (control_va as usize + off) as *const u8;
    // SAFETY: the control grant is CONTROL_BYTES long and every offset here
    // is inside it; the device writes these fields, so each read is volatile.
    unsafe {
        let stopped = read_volatile(at(TXQ_DESC_OFF + STATUS)) != 0
            || read_volatile(at(RXQ_DESC_OFF + STATUS)) != 0;
        let ecr = read_volatile(at(ECR) as *const u32);
        stopped || ecr & ECR_QUEUE_ERRORS != 0
    }
}
