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

//! No refusal turns Bus Master on, and a refused Bus Master write ends the
//! attempt. The controller is one a predecessor left running, its queues
//! and host memory buffer armed (fake::left_running).

use nonos_libc::{calls, refuse, refuse_only};

use super::bring_up::stopped;
use super::bus_master_tests::{bus_master, first, FOUND};
use crate::fake::{left_running, registers};

#[test]
fn no_refusal_turns_bus_master_on() {
    // A refused claim leaves nothing to let go; after a refused Command write
    // or BAR0 map the claim is released; then a blank block; then a wedge.
    for (call, last) in
        [("claim", "claim"), ("pci_command", "device_release"), ("mmio_map", "device_release")]
    {
        let _controller = left_running();
        refuse(call);
        assert!(stopped(FOUND).is_err(), "{call} refused");
        assert_eq!(calls().last().map(|c| c.0), Some(last), "{call} refused");
    }
    refuse("");
    let _blank = registers();
    assert!(stopped(FOUND).is_err(), "not an NVMe register block");
    let (_, fake) = left_running();
    fake.borrow_mut().wedged = true;
    assert!(stopped(FOUND).is_err(), "RDY never falls");
    assert_eq!(first(0, bus_master), None, "{:?}", calls());
    // The Bus Master write itself refused, once the controller was seen
    // stopped: the attempt ends there and the claim is released.
    let (_controller, from) = (left_running(), calls().len());
    refuse_only("pci_command", 0x406);
    assert!(stopped(FOUND).is_err(), "Bus Master refused");
    let seen: Vec<_> = calls()[from..].iter().map(|c| c.0).collect();
    assert!(seen.contains(&"stopped") && seen.last() == Some(&"device_release"), "{seen:?}");
}
