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

//! The driver's live state, the BAR map, the PCI Command word, and the
//! release, without the broker sequence that builds the state. The tests
//! build one over host memory and a window instead.

#[path = "../../../capsule_driver_e1000/src/setup/driver.rs"]
mod driver;
#[path = "../../../capsule_driver_e1000/src/setup/mmio.rs"]
pub mod mmio;
#[path = "../../../capsule_driver_e1000/src/setup/pci.rs"]
mod pci;
#[path = "../../../capsule_driver_e1000/src/setup/rollback.rs"]
pub mod rollback;

pub use driver::Driver;
pub use pci::command;
