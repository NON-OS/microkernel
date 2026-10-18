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

//! A GICv2m frame: message signalled interrupts as SPIs, raised by a device
//! writing the SPI's INTID to the frame. The frame does not know who wrote,
//! so unlike an ITS it cannot keep one device from raising another's SPI;
//! that is the hardware's limit, stated where the controller is announced.

mod frame;
mod state;
mod vectors;

pub use frame::bring_up;
pub use vectors::{bind, enable, unbind};
