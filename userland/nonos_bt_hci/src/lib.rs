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

//! The host side of HCI for an LE-only host, from the Bluetooth Core
//! Specification 5.4 (cited as Vol, Part, section). The USB transport works
//! through `UsbPipes`, which a capsule backs with its USB controller
//! driver; the host core works through `Transport`, so it runs the same
//! over USB here and over a simulated controller in the proofs. Nothing
//! here sends a vendor command: a controller that needs vendor setup
//! before it answers the standard ones is refused, not patched.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod error;
pub mod host;
pub mod transport;
pub mod uart;
pub mod usb;

pub use error::HciError;
pub use host::{Controller, Host};
pub use transport::Transport;
