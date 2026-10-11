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

//! A model of the vmxnet3 device at revision 1, enough of it to hold the
//! driver to vmxnet3_defs.h: the revision registers, the commands with their
//! answers in CMD, ACTIVATE_DEV reading and checking the shared area and the
//! queue table, and both queues with their generation bits, every DMA
//! confined to the regions the test granted.

mod activate;
mod command;
mod device;
mod events;
mod memory;
mod ring;
mod rx;
mod thread;
mod tx;
mod window;

pub use device::Device;
pub use events::{Event, Faults};
pub use memory::{Memory, Region};
pub use ring::Ring;
pub use rx::Delivered;
pub use thread::{install, take, with_device};
pub use window::Bar;
