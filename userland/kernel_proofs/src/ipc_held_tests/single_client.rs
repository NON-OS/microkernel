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

//! The devices that take one client, or one short list.

use super::common::owner;
use super::held::endpoint_admits;

/// The class drivers above the controller, each finding its own devices.
const XHCI_CLIENTS: &[&str] = &[
    "driver.usb_hid0",
    "driver.usb_msc0",
    "driver.usb_audio0",
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.rndis0",
    "driver.ax88179_0",
    "driver.rtl8153_0",
];

/// The controllers, the GPU and the sound devices take their one client.
#[test]
fn each_device_takes_its_service_alone() {
    let cases: [(&str, &[&'static str]); 5] = [
        ("driver.xhci0", XHCI_CLIENTS),
        ("driver.i2c_pci0", &["driver.i2c_hid0"]),
        ("driver.virtio_gpu0", &["compositor"]),
        ("driver.hda0", &["audio.server"]),
        ("driver.usb_audio0", &["audio.server"]),
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
