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

//! The driver's descriptor walks, loaded from its own files: the
//! configuration descriptor and the report descriptor.

#[path = "../../../capsule_driver_usb_hid/src/descriptors/binding.rs"]
pub mod binding;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/config.rs"]
pub mod config;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/packet_size.rs"]
pub mod packet_size;
// The parser returns Result<_, ()>, its own choice.
#[allow(clippy::result_unit_err)]
#[path = "../../../capsule_driver_usb_hid/src/descriptors/parse.rs"]
pub mod parse;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/records.rs"]
pub mod records;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/report/mod.rs"]
pub mod report;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/types.rs"]
pub mod types;

pub use binding::HidBinding;
pub use parse::hid_bindings;
pub use types::HidKind;
