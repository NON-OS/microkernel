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

//! Waiting for the device to use the command in flight. The wait ends when
//! `expired` says so, which the caller makes a deadline by the clock
//!; it used to be 8192 yields of 1024 reads each, whose length
//! was the processor's and the scheduler's.

use super::{layout::QueueLayout, used};

/// Reads between the clock's questions: a command is often answered within
/// them, which spares a yield.
const READS_PER_CHECK: u32 = 1024;

pub fn used_moved(
    layout: QueueLayout,
    from: u16,
    mut expired: impl FnMut() -> bool,
) -> Result<(), &'static str> {
    loop {
        for _ in 0..READS_PER_CHECK {
            if used::read_idx(layout) != from {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        if expired() {
            return Err("virtio-gpu: device timeout");
        }
    }
}
