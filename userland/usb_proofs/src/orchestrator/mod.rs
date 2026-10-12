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

//! The driver's binding step, which state a binding is driven with after
//! its report descriptor is read, beside the endpoint type and poll loop it
//! names by their places under `orchestrator`.

#[path = "../../../capsule_driver_usb_hid/src/orchestrator/device_for.rs"]
pub mod device_for;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/report_descriptor.rs"]
pub mod report_descriptor;

pub use crate::{enumerate, poll};
