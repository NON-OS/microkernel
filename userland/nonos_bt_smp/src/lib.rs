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

//! The Security Manager Protocol for an LE-only host, from the Bluetooth
//! Core Specification 5.4, Vol 3, Part H (cited by section). Only LE Secure
//! Connections is offered: legacy pairing's key exchange can be broken by a
//! passive listener (Vol 3, Part H, 2.3.5.6), so a peer that offers only
//! legacy pairing is refused, not downgraded to.
//!
//! The crypto functions take and return values most significant octet
//! first, as the specification writes them; SMP PDUs carry them least
//! significant octet first (Vol 3, Part H, 3.1), and `pdu` converts.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod cmac;
pub mod ecdh;
pub mod error;
pub mod funcs;
pub mod method;
pub mod pairing;
pub mod pdu;
pub mod privacy;

pub use error::SmpError;
