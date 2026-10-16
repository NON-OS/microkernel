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

//! The driver's register access, assembled from its files with the one
//! that touches the BAR replaced: the shipping HAIMR and PHY paths run
//! unchanged over `mmio.rs`, which hands every access to the modelled
//! reader (`crate::model`) instead of a mapped window.

mod mmio;
#[path = "../../../capsule_driver_rtsx/src/hw/phy.rs"]
mod phy;
#[path = "../../../capsule_driver_rtsx/src/hw/reg.rs"]
mod reg;

pub use mmio::Regs;
pub use phy::{write_phy, PHY_MS};
pub use reg::{read_register, write_register, ACCESS_MS};
