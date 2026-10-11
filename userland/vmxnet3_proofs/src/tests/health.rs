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

//! What the health check reads: the queue status the device writes on
//! GET_QUEUE_STATUS, and the queue error bits in the event cause word.

use super::util::up;
use crate::bench::{Bench, CONTROL_DEV};
use crate::constants::cmd::GET_QUEUE_STATUS;
use crate::model::with_device;
use crate::shared::queues_failed;

#[test]
fn running_queues_are_healthy() {
    let mut bench = Bench::new();
    let d = up(&mut bench);
    d.regs.command(GET_QUEUE_STATUS);
    assert!(!queues_failed(d.control_va));
}

#[test]
fn a_stopped_queue_or_a_queue_error_is_a_failure() {
    let mut bench = Bench::new();
    let d = up(&mut bench);
    with_device(|dev| dev.faults.stop_queues = true);
    d.regs.command(GET_QUEUE_STATUS);
    assert!(queues_failed(d.control_va));
    with_device(|dev| dev.faults.stop_queues = false);
    d.regs.command(GET_QUEUE_STATUS);
    assert!(!queues_failed(d.control_va));
    // VMXNET3_ECR_TQERR, written into the shared area's ecr word.
    with_device(|dev| dev.mem.set_u32(CONTROL_DEV + 696, 0x2));
    assert!(queues_failed(d.control_va));
}
