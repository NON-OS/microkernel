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

//! The server serves one request at a time, so a device that never
//! finishes SET_ADDRESS held every keyboard and disk on the machine for the
//! 5 s every command gets. Address Device is given up after the 500 ms
//! Linux allows a slow device; USB 2.0 gives it 50 ms.

use std::time::{Duration, Instant};

use super::bench::Bench;
use super::hc_device::lock;
use crate::controller::command::issue_address_device::ADDRESS_TIMEOUT_MS;
use crate::protocol::{E_IO, OP_ADDRESS_DEVICE};

const HIGH_SPEED: u32 = 3;

#[test]
fn an_address_device_that_never_completes_is_given_up_within_a_second() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.enable_slot();
    lock(&b.hc).stall_address = Some(slot);
    let started = Instant::now();
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[slot, 1]).status, E_IO);
    let took = started.elapsed();
    assert!(took >= Duration::from_millis(ADDRESS_TIMEOUT_MS), "{took:?}");
    assert!(took < Duration::from_secs(2), "the port, the command and its abort took {took:?}");
}
