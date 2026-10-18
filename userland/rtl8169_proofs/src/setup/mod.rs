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

//! The `setup` files, each the shipping one, and the discovery they take
//! from: the claim, the PCI Command word and its write, the BAR map, the DMA
//! grants, the driver record the sequence fills in, and the release, against
//! the shim's broker calls.

// Mounted from the driver's src directory, so discover.rs finds its own
// modules beside it, as in the capsule; the sequence takes what it found.
#[path = "../../../capsule_driver_rtl8169/src"]
mod driver_src {
    pub mod discover;
}
#[path = "../../../capsule_driver_rtl8169/src/setup/claim.rs"]
mod claim;
#[path = "../../../capsule_driver_rtl8169/src/setup/command_word.rs"]
mod command_word;
#[path = "../../../capsule_driver_rtl8169/src/setup/dma.rs"]
mod dma;
#[path = "../../../capsule_driver_rtl8169/src/setup/driver.rs"]
mod driver;
#[path = "../../../capsule_driver_rtl8169/src/setup/mmio.rs"]
mod mmio;
#[path = "../../../capsule_driver_rtl8169/src/setup/pci.rs"]
mod pci;
#[path = "../../../capsule_driver_rtl8169/src/setup/rollback.rs"]
pub mod rollback;
#[path = "../../../capsule_driver_rtl8169/src/setup/sequence.rs"]
mod sequence;

pub use command_word::command_word;
pub use driver::Driver;
pub use driver_src::discover;
pub use pci::command;
pub use sequence::run;
