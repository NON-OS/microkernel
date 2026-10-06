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

//! The boot keyboard decode at `crate::hid::keyboard`, the path the
//! driver's `pub(in crate::hid::keyboard)` items are scoped to.

#[path = "../../../../capsule_driver_usb_hid/src/hid/keyboard/boot_report.rs"]
pub mod boot_report;
#[path = "../../../../capsule_driver_usb_hid/src/hid/keyboard/is_error_code.rs"]
pub mod is_error_code;
#[path = "../../../../capsule_driver_usb_hid/src/hid/keyboard/is_real_key.rs"]
pub mod is_real_key;
#[path = "../../../../capsule_driver_usb_hid/src/hid/keyboard/key_changes.rs"]
pub mod key_changes;
