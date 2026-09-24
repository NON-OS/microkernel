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

//! The shared header and the request/reply round trip.

mod call;
mod decode;
mod encode;

pub use call::call;
pub use decode::decode_reply;
pub use encode::{encode_request, rw_header};

pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
pub const SECTOR_SIZE: usize = 512;
/// The smallest per-request ceiling among the three drivers.
pub const MAX_SECTORS: usize = 64;
pub const MAX_BYTES: usize = MAX_SECTORS * SECTOR_SIZE;
