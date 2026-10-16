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

//! The x86 GPIO controller's engine, register models and pin router. The
//! AMD bank and Intel communities each implement `Controller` over a
//! window the caller maps; `Router` arms firmware-named pins, takes the
//! shared interrupt apart into deliveries for each pin's owner, masks a
//! storming pin alone, and drives an output only for its device's holder.

#![no_std]

extern crate alloc;

mod access;
pub mod amd;
pub mod amd_bits;
mod amd_io;
pub mod controller;
mod demux;
pub mod device_key;
pub mod intel;
pub mod intel_bits;
mod intel_io;
mod intel_layout;
pub mod router;
pub mod serial_key;
pub mod storm;
pub mod trigger;
pub mod window;

pub use access::OutputError;
pub use controller::{Controller, GpioError};
pub use demux::Delivery;
pub use device_key::{device_of, DEVICE};
pub use serial_key::bus_key;
pub use router::{BindError, Owner, Router};
pub use trigger::{Polarity, Trigger};
