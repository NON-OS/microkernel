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

//! Platform controllers the namespace declares by `_HID` or `_CID` with a
//! memory `_CRS` and no PCI function standing for them (ACPI 6.5, 6.1.5,
//! 6.2.2), matched against a list of ids the caller supplies as text.

mod device;
mod find;
mod known;
mod windows;

pub use device::{Found, PlatformDevice, Skip};
pub use find::platform_devices;
pub use known::{known, Class, Known};
pub use windows::windows;
