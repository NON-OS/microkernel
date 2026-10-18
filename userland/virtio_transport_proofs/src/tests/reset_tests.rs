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

//! A reset is waited for by elapsed time, not by how fast the processor
//! reads: a device that takes many reads to settle (vDPA, a SmartNIC) is
//! still waited for, and one that never settles is given up on when the
//! caller's time runs out.

use super::slow_reset::SlowReset;
use crate::common::{reset, reset_until, start_until, RESET_POLLS};
use crate::error::VirtioError;

/// A clock that moves one millisecond per question, as the drivers' sleep
/// between reads does; expired once `budget_ms` have passed.
fn clock(budget_ms: u32) -> impl FnMut() -> bool {
    let mut now = 0u32;
    move || {
        now += 1;
        now >= budget_ms
    }
}

#[test]
fn a_device_that_settles_after_many_reads_is_still_waited_for() {
    let dev = SlowReset::new(1500);
    assert_eq!(reset_until(&dev, clock(2000)), Ok(()), "gave up after {} reads", dev.reads.get());
    assert_eq!(dev.reads.get(), 1501);
}

#[test]
fn a_device_that_never_settles_is_given_up_on_when_the_time_is_spent() {
    let dev = SlowReset::new(u32::MAX);
    assert_eq!(reset_until(&dev, clock(1000)), Err(VirtioError::ResetTimeout));
    assert_eq!(dev.reads.get(), 1000, "one read per millisecond");
    assert_eq!(start_until(&dev, clock(5)), Err(VirtioError::ResetTimeout));
    assert_eq!(dev.status.get(), 0, "nothing written past the reset");
}

#[test]
fn without_a_clock_the_wait_is_still_bounded_by_reads() {
    let dev = SlowReset::new(u32::MAX);
    assert_eq!(reset(&dev), Err(VirtioError::ResetTimeout));
    assert_eq!(dev.reads.get(), RESET_POLLS);
}
