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

//! The ops served and the statuses answered, numbered as virtio-gpu numbers
//! them (capsule_driver_virtio_gpu/src/protocol), so the compositor needs no
//! path of its own for this adapter beyond finding it.

pub const OP_TRANSFER_TO_HOST: u16 = 0x0008;
pub const OP_SET_SCANOUT: u16 = 0x0009;
pub const OP_FLUSH: u16 = 0x000A;
pub const OP_GET_PRIMARY_SURFACE: u16 = 0x000C;

pub const TRANSFER_LEN: usize = 32;
pub const SET_SCANOUT_LEN: usize = 24;
pub const FLUSH_LEN: usize = 20;
pub const PRIMARY_LEN: usize = 32;

/// A frame that does not parse, a body of the wrong length, a resource
/// other than the primary, or a rectangle outside the mode.
pub const E_INVAL: i32 = -22;
/// The primary is another process's, or nobody's yet.
pub const E_BUSY: i32 = -16;
/// An op this driver does not serve, asked with no body.
pub const E_BAD_OP: i32 = -38;
/// The adapter no longer reads back the mode it is driven in.
pub const E_DEVICE: i32 = -110;
