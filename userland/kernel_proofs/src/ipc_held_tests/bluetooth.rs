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

//! driver.bt_usb0 answers Settings, in any window, and the shell; no other
//! capsule may reach it, not even one of the Bluetooth host's own peers on
//! the USB side. The capsule then checks each op again (letter E-25).

use super::common::owner;
use super::held::endpoint_admits;

const BT: &str = "driver.bt_usb0";

#[test]
fn the_bluetooth_host_takes_settings_and_the_shell() {
    for caller in ["app.settings", "app.settings.1", "app.settings.2", "desktop_shell"] {
        let one: &'static [&'static str] = Box::leak(Box::new([caller]));
        assert!(endpoint_admits(BT, owner(one)), "refused {caller}");
    }
}

#[test]
fn the_bluetooth_host_refuses_everyone_else() {
    for caller in ["app.browser", "app.settings.3", "net.core", "driver.xhci0", "driver.usb_hid0"] {
        let one: &'static [&'static str] = Box::leak(Box::new([caller]));
        assert!(!endpoint_admits(BT, owner(one)), "took {caller}");
    }
    assert!(!endpoint_admits(BT, owner(&[])), "took a caller owning nothing");
}
