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

const DRIVER: &str = include_str!("../../../../capsule_driver_usb_msc/src/protocol/errno.rs");
const STATUS: &str = include_str!("../../../../../src/hardware/usb_msc_capsule/status.rs");

#[test]
fn both_ends_number_the_empty_slot_alike() {
    for (name, value) in [("E_NOMEDIUM", -123), ("E_NODEV", -19)] {
        assert!(DRIVER.contains(&format!("pub const {name}: i32 = {value};")), "{name}");
        assert!(STATUS.contains(&format!("const {name}: i32 = {value};")), "{name}");
    }
}

#[test]
fn an_empty_slot_is_no_disk_not_a_failing_one() {
    assert!(STATUS.contains("E_NODEV | E_NOMEDIUM => E::NoDevice,"));
}
