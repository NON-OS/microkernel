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

//! Reading the system policy store.
//!
//! A capsule that owns a setting needs three things: the store's port, one
//! request and reply, and the confidence that the reply is an answer to the
//! question it asked. This carries all three so that each owner is a field name
//! and a line to apply it, rather than another hand-written round trip.

#![no_std]

mod call;
mod get;
mod lookup;
mod status;

pub use call::{call, Reply};
pub use get::{get_bool, get_i8, get_str, get_u8};
pub use lookup::lookup;
pub use status::status;
