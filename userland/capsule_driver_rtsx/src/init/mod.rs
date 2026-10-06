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

//! Bringing the reader chip up, as rtsx_pci_init_chip and rtsx_pci_init_hw
//! do with the RTS5227 family's ops (drivers/misc/cardreader/rts5227.c).

mod aspm;
mod bier;
pub mod driving;
mod extra;
mod extra_522a;
mod hw;
mod ic;
mod ocp;
mod phy;
mod sequence;

pub use hw::init_hw;
pub use ocp::enable_ocp;
