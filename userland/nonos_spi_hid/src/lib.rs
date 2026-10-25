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
//! HID over SPI, the protocol alone: input report headers and bodies, the
//! device descriptor, and the frames that carry them. Pure, for the proofs
//! and for driver.spi_hid0.

#![no_std]

extern crate alloc;

mod descriptor;
mod facts;
mod frame;
mod input;

pub use descriptor::{descriptor, Descriptor};
pub use facts::*;
pub use frame::{read_frame, write_frame, Params};
pub use input::{body, header, Body, Header};
