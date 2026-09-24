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

//! A block device behind whichever driver capsule owns it.
//!
//! The three drivers share a header and differ in magic, opcode numbers and
//! service name. [`Driver`] carries those three facts per driver, the wire
//! module encodes and decodes the shared header, and [`BlockDevice`] is what
//! a caller holds: capacity, read, write, flush, in whole sectors, sixty-four
//! per request because that is the smallest ceiling any of the three has.

#![no_std]

extern crate alloc;

mod device;
mod disks;
mod driver;
mod error;
mod sink;
mod wire;

pub use device::{discover, BlockDevice, Found, Identity};
pub use disks::{scan, Contents, Disk};
pub use driver::Driver;
pub use error::BlkError;
pub use sink::DeviceSink;
