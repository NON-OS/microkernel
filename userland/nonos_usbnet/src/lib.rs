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

//! The core every USB network class driver shares: the client side of
//! driver.xhci0 (with a polled bulk IN, so an idle link never holds the
//! controller), the descriptor walk, the NNET frame service net.core and
//! net.l2 speak, and the search for the device. A driver capsule brings
//! its binding and its transfer framing and calls `run`.

#![no_std]

extern crate alloc;

pub mod bind;
pub mod bus;
pub mod desc;
pub mod found;
pub mod halt;
pub mod nic;
pub mod nnet;
pub mod run;
pub mod say;
pub mod scan;
pub mod setup;
pub mod xhci;

pub use bind::Bind;
pub use bus::{Bus, Pipes};
pub use found::Found;
pub use nic::Nic;
pub use run::run;
pub use setup::Setup;
