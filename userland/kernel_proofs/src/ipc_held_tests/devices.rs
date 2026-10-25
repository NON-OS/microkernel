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

//! What the kernel drives, which no capsule reaches.

use super::common::{owner, IPC};
use super::held::{callers_of, endpoint_admits, inbox_admits};

/// Keyboards, pointers, the random source, USB storage and the hub walker
/// answer the kernel alone.
#[test]
fn what_the_kernel_drives_no_capsule_reaches() {
    let everyone: &'static [&'static str] = &[
        "compositor",
        "input_router",
        "wm",
        "app.terminal",
        "app.browser",
        "audio.server",
        "net.core",
        "driver.xhci0",
        "driver.usb_hid0",
        "driver.i2c_hid0",
    ];
    for ep in [
        "driver.ps2_kbd0",
        "driver.usb_hid0",
        "driver.i2c_hid0",
        "driver.usb_msc0",
        "driver.usb_hub0",
        "driver.virtio_rng",
    ] {
        assert_eq!(callers_of(ep), Some(&[][..]), "{ep} names a caller");
        assert!(!endpoint_admits(ep, owner(everyone)), "{ep} took a capsule");
        assert!(
            !inbox_admits([(ep, IPC)], u64::MAX, owner(everyone)),
            "{ep} by pid, every bit held"
        );
    }
}
