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

//! Wire constants for the vfs_pool protocol.

pub const VFS_PORT: u32 = 4104;
pub const MAGIC: u32 = 0x4E4F_5646;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_OPEN: u16 = 1;
pub const OP_CLOSE: u16 = 2;
pub const OP_LIST: u16 = 6;
pub const OP_MKDIR: u16 = 8;
pub const OP_UNLINK: u16 = 9;
pub const OP_RENAME: u16 = 10;
pub const OP_RMDIR: u16 = 11;
pub const OP_STORE_STATUS: u16 = 19;
pub const OP_GENERATION: u16 = 26;

pub const O_CREATE: u32 = 1;

/// What the desktop asks on its own, every second or on a change: a
/// listing, the store's generation and status. The shell serves its frame
/// loop on this thread, so a busy file service costs it this much and no
/// more; the next look asks again.
pub const TIMEOUT_MS: u64 = 300;

/// What a person asked for: create, delete, rename or move. On a slow
/// laptop's eMMC, a write to the installed store can outlast 300 ms, and the
/// desktop said "the file service did not answer" for a change that was then
/// made. The kernel's own reply default: the person waits on their click,
/// and a service that has truly stopped is still said.
pub const WRITE_TIMEOUT_MS: u64 = 5_000;

/// The reply budget for `op`.
pub fn budget(op: u16) -> u64 {
    match op {
        OP_OPEN | OP_CLOSE | OP_MKDIR | OP_UNLINK | OP_RENAME | OP_RMDIR => WRITE_TIMEOUT_MS,
        _ => TIMEOUT_MS,
    }
}

/// Longest path or name the wire header can carry (one length byte).
pub const MAX_NAME: usize = 255;
