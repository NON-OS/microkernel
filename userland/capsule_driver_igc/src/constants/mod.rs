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

//! Register offsets, bit fields, sizes and wait bounds. Each file names the
//! Linux igc header or function its values come from.

pub mod ctrl;
mod frame;
pub mod pci;
pub mod phy;
pub mod queue;
pub mod regs;
pub mod rx_bits;
pub mod timeouts;
pub mod tx_bits;

pub use frame::{MAC_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
