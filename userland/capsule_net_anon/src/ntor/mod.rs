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

//! The ntor handshake, client side.
//!
//! One of these per hop. It is the only handshake this capsule speaks: the
//! fork deleted TAP outright, and ntor v3 is only chosen for an exit that
//! advertises congestion control, which this capsule does not negotiate.

mod constants;
mod create;
mod finish;
mod inputs;

pub use constants::KEY_MATERIAL_BYTES;
pub use create::Handshake;
