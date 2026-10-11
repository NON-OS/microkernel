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

//! The input devices a hypervisor emulates answer the kernel alone, as the
//! physical keyboards and pointers do: their drivers post onto the input
//! ring and serve nothing.

use super::common::{owner, IPC};
use super::held::{callers_of, endpoint_admits, inbox_admits};

/// Every virtual input device's driver endpoint.
const VIRTUAL_INPUT: &[&str] = &["driver.virtio_input0"];

#[test]
fn a_virtual_pointer_or_keyboard_is_reached_by_no_capsule() {
    let everyone: &'static [&'static str] =
        &["compositor", "input_router", "wm", "app.terminal", "net.core", "driver.usb_hid0"];
    for &ep in VIRTUAL_INPUT {
        assert_eq!(callers_of(ep), Some(&[][..]), "{ep} names a caller");
        assert!(!endpoint_admits(ep, owner(everyone)), "{ep} took a capsule");
        assert!(!inbox_admits([(ep, IPC)], u64::MAX, owner(everyone)), "{ep} by pid");
    }
}
