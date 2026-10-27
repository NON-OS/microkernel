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

//! The controller discovery found, as `setup` takes it, and the search
//! itself over the shim's device list (find_tests.rs).

#[cfg(test)]
#[path = "../../capsule_driver_xhci/src/discover/find.rs"]
mod find;
#[path = "../../capsule_driver_xhci/src/discover/found.rs"]
mod found;
#[cfg(test)]
#[path = "../../capsule_driver_xhci/src/discover/thunderbolt.rs"]
mod thunderbolt;

#[cfg(test)]
#[path = "find_tests.rs"]
mod find_tests;

pub use found::Found;
