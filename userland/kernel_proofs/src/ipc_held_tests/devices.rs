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

//! What the kernel drives, and the devices with one client.

use super::common::{owner, IPC};
use super::held::{callers_of, endpoint_admits, inbox_admits};

/// A keyboard's or pointer's driver, the random source and USB storage
/// answer the kernel alone: no capsule reaches them, whatever it owns.
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

/// The controllers, the GPU and the sound card take their one client.
#[test]
fn each_device_takes_its_service_alone() {
    let cases: [(&str, &[&'static str]); 4] = [
        ("driver.xhci0", &["driver.usb_hid0", "driver.usb_msc0"]),
        ("driver.i2c_pci0", &["driver.i2c_hid0"]),
        ("driver.virtio_gpu0", &["compositor"]),
        ("driver.hda0", &["audio.server"]),
    ];
    for (ep, clients) in cases {
        for c in clients {
            let one: &'static [&'static str] = Box::leak(Box::new([*c]));
            assert!(endpoint_admits(ep, owner(one)), "{ep} refused {c}");
        }
        for other in
            ["app.browser", "app.terminal", "wm", "input_router", "app.audio_player", "net.core"]
        {
            let one: &'static [&'static str] = Box::leak(Box::new([other]));
            assert!(!endpoint_admits(ep, owner(one)), "{ep} took {other}");
        }
    }
}
