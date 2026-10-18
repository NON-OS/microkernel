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

//! The configuration descriptor walk over lengths the device chose: every
//! record's bLength, the header's wTotalLength, and records cut short. The
//! walk must end, read nothing past wTotalLength or the buffer, and bind
//! only a HID interface's interrupt IN endpoint.

mod binding_tests;
mod header_tests;
mod hid_descriptor_tests;
mod kbd;
mod length_tests;
mod packet_tests;
mod record_tests;

pub use kbd::KBD;
