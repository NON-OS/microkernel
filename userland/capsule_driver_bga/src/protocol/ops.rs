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

//! The ops served, with the numbers and lengths driver.virtio_gpu0 gives
//! them (bga_proofs holds the two lists equal).

pub const OP_HEALTHCHECK: u16 = 0x0001;
pub const OP_TRANSFER_TO_HOST: u16 = 0x0008;
pub const OP_SET_SCANOUT: u16 = 0x0009;
pub const OP_FLUSH: u16 = 0x000A;
pub const OP_GET_PRIMARY_SURFACE: u16 = 0x000C;

pub const STATUS_LEN: usize = 4;
pub const TRANSFER_TO_HOST_REQ_LEN: usize = 32;
pub const SET_SCANOUT_REQ_LEN: usize = 24;
pub const FLUSH_REQ_LEN: usize = 20;
pub const GET_PRIMARY_SURFACE_RESP_LEN: usize = 32;
/// The largest request: a header and a transfer.
pub const REQUEST_MAX: usize = 20 + TRANSFER_TO_HOST_REQ_LEN;

pub const E_BUSY: i32 = -16;
pub const E_INVAL: i32 = -22;
pub const E_BAD_OP: i32 = -38;
