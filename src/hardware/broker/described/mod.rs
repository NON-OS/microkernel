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

//! Devices a firmware description names rather than a bus enumerates: the
//! device tree's eMMC, xHCI and AHCI controllers on an Arm board.
//!
//! A PCI function brings config space, and with it the class code a capsule
//! matches on and the Bus Master Enable bit the broker clears when a capsule
//! lets go. A described device has neither, so this module supplies both:
//! the class its interface would carry on PCI, and a stop written from that
//! interface's own specification.

mod interface;
mod lookup;
mod quiesce;
mod record;
mod register;
mod state;
mod stop_regs;
mod window;

pub use interface::Interface;
pub(in crate::hardware::broker) use lookup::dma_window;
pub(in crate::hardware::broker) use quiesce::stop;
pub use register::{register, Described};
pub use window::BusWindow;
