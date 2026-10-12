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

//! The statuses a reply carries, negative errno values as driver.xhci0
//! answers them, and the largest request and reply.

pub const E_IO: i32 = -5;
pub const E_AGAIN: i32 = -11;
pub const E_BUSY: i32 = -16;
pub const E_INVAL: i32 = -22;
pub const E_PIPE: i32 = -32;
/// The largest request: a bulk OUT's four-byte head and 4 KiB.
pub const REQUEST_MAX: usize = 4 + 4096;
/// The largest reply after the header and status: a bulk IN's prefix and
/// 4 KiB, which covers the two-port status list.
pub const REPLY_MAX: usize = 4 + 4096;
