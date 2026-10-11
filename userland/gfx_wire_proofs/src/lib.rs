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

//! The display wire, proved without a driver or a compositor.
//!
//! The `#[path]` includes pull in the shipping crate at the module names
//! its `crate::` paths use, so these tests run the bytes both ends speak.

#[path = "../../nonos_gfx_wire/src/body/mod.rs"]
pub mod body;
#[path = "../../nonos_gfx_wire/src/bytes.rs"]
pub mod bytes;
#[path = "../../nonos_gfx_wire/src/errno.rs"]
pub mod errno;
#[path = "../../nonos_gfx_wire/src/fit.rs"]
pub mod fit;
#[path = "../../nonos_gfx_wire/src/header.rs"]
pub mod header;
#[path = "../../nonos_gfx_wire/src/kind.rs"]
pub mod kind;
#[path = "../../nonos_gfx_wire/src/ops.rs"]
pub mod ops;
#[path = "../../nonos_gfx_wire/src/reply.rs"]
pub mod reply;
#[path = "../../nonos_gfx_wire/src/request.rs"]
pub mod request;
#[path = "../../nonos_gfx_wire/src/serial.rs"]
pub mod serial;

#[cfg(test)]
mod tests;
