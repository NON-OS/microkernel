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

//! The key a device's pin is bound to: its bus controller, whose holder
//! alone hears the pin. There is one form, `DEVICE | broker device id`, for
//! a controller that is a PCI function and for one that is an ACPI
//! platform device alike; capsule_acpi works it out from the tables and
//! hands it over in SERIAL. A capsule never sees bus, device or function
//! numbers, so no other form reaches this crate. capsule_acpi's HOLDER
//! takes the same key and runs the kernel's claim test for it.

pub const DEVICE: u64 = 1 << 63;

/// The broker device id a key names; None for anything but this form.
pub fn device_of(key: u64) -> Option<u64> {
    (key & DEVICE != 0).then_some(key & !DEVICE)
}
