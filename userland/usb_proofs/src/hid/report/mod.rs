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

//! The driver's report-descriptor decode at `crate::hid::report`, its
//! parts public so the proofs can read a report through a layout directly.

#[path = "../../../../capsule_driver_usb_hid/src/hid/report/device.rs"]
pub mod device;
#[path = "../../../../capsule_driver_usb_hid/src/hid/report/keys.rs"]
pub mod keys;
#[path = "../../../../capsule_driver_usb_hid/src/hid/report/pointer.rs"]
pub mod pointer;
#[path = "../../../../capsule_driver_usb_hid/src/hid/report/read.rs"]
pub mod read;

pub use device::ReportDevice;
