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

//! What a configuration says of the device as a whole: the value that
//! selects it, and whether it is a hub.

use crate::config_blob::{config, HID, INTERRUPT};
use crate::descriptors::config::{configuration_value, is_hub, CLASS_HUB};

#[test]
fn the_configuration_value_is_the_descriptors_own() {
    let mut raw = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 8, 10).build();
    assert_eq!(configuration_value(&raw), Some(1));
    raw[5] = 2;
    assert_eq!(configuration_value(&raw), Some(2));
    assert_eq!(configuration_value(&raw[..8]), None, "no header, no value");
}

#[test]
fn a_hub_is_recognised_by_its_interface_class() {
    let hub = config().iface(0, CLASS_HUB, 0, 0).ep(0x81, INTERRUPT, 1, 12).build();
    assert!(is_hub(&hub));
    let kbd = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 8, 10).build();
    assert!(!is_hub(&kbd));
    // A record that runs past wTotalLength ends the walk: nothing is read
    // beyond the bytes there.
    let mut cut = hub.clone();
    cut[9] = 200;
    assert!(!is_hub(&cut));
    assert!(!is_hub(&hub[..5]));
}
