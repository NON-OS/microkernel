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

//! Bluetooth wire formats for an LE-only host, from the Bluetooth Core
//! Specification 5.4 (cited as Vol, Part, section). Every parser returns a
//! `WireError` on malformed input instead of panicking, so a bad packet from
//! a controller or a peer ends that packet and nothing else.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod addr;
pub mod att;
pub mod cursor;
pub mod error;
pub mod hci;
pub mod l2cap;

pub use error::WireError;
