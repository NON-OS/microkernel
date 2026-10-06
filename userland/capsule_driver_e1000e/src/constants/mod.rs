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

//! Device IDs, register offsets and bits, ring sizes and every timeout.

pub mod ctrl;
mod family;
mod frame;
pub mod ids;
pub mod pch_bits;
pub mod pci;
pub mod phy;
pub mod queue;
pub mod regs;
pub mod regs_pch;
pub mod rxtx;
pub mod status;
pub mod timeouts;

pub use family::Family;
pub use frame::{MAC_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
