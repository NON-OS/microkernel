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

//! How much the store holds, in plaintext bytes and files.
//!
//! The capsule runs in the libc default heap, 16 MiB, and every write opens
//! the whole file and seals it again: a file of N bytes costs its sealed
//! copy, the opened one and the new sealed one, about three times N, while
//! every other file stays where it is. The offset of a write and the length
//! of a truncate come from the caller, and the kernel forwards a /ram file's
//! offset, which lseek sets, and the length ftruncate names, as they are. A
//! buffer resized to one of them was how any process could make ramfs ask
//! for more memory than exists and stop, taking every other process's /ram
//! files with it.

/// The largest file.
pub const MAX_FILE_BYTES: usize = 2 * 1024 * 1024;

/// Every file together: with all of it held and the largest file being
/// rewritten (two more copies of it), 12 MiB, with room left in the heap for
/// the request buffers and the tables.
pub const MAX_STORE_BYTES: usize = 8 * 1024 * 1024;

/// Files, each a key, a nonce, its path and a map node.
pub const MAX_FILES: usize = 1024;
