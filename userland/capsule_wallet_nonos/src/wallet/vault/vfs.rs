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

//! The three calls this needs from vfs_pool, and nothing else.

#[path = "vfs_call.rs"]
mod vfs_call;
pub(super) use vfs_call::call;

const VFS_PORT: u32 = 4104;
const MAGIC: u32 = 0x4E4F_5646;
const VERSION: u16 = 1;
pub(super) const HDR_LEN: usize = 20;

/// The store's "no such file".
pub(super) const ENOENT: i32 = -2;

pub(super) const OP_OPEN: u16 = 1;
pub(super) const OP_CLOSE: u16 = 2;
pub(super) const OP_READ: u16 = 3;
pub(super) const OP_WRITE: u16 = 4;
pub(super) const OP_MKDIR: u16 = 8;
pub(super) const OP_STORE_PERSIST: u16 = 17;

pub(super) const O_CREATE: u32 = 1 << 0;
/// A second save must not leave the first blob's tail behind it. Both are the
/// same length today, so the bytes would be overwritten either way; the flag
/// is here because a format that grows would otherwise read back as a valid
/// blob with a corrupt tail.
pub(super) const O_TRUNC: u32 = 1 << 1;
