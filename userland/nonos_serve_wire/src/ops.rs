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

//! The numbers both sides use.

pub const MAGIC: [u8; 4] = *b"NSO1";
pub const HEADER: usize = 12;
/// The most bytes a frame carries after its header.
pub const MAX_BODY: usize = 64 * 1024;
/// The longest a READ waits for the guest to write.
pub const MAX_WAIT_MS: u32 = 5_000;
/// The endpoint a served box is reached at, followed by the capsule's name.
pub const ENDPOINT_PREFIX: &str = "linux.serve.";

pub const OPEN: u8 = 1;
pub const WRITE: u8 = 2;
pub const READ: u8 = 3;
pub const CLOSE: u8 = 4;

pub const OK: u8 = 0;
pub const CLOSED: u8 = 1;
pub const EBADF: u8 = 9;
pub const EAGAIN: u8 = 11;
pub const EBUSY: u8 = 16;
pub const EINVAL: u8 = 22;
pub const ECONNREFUSED: u8 = 111;
