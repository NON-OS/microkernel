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

//! Every bound the broker holds itself and its clients to.

/// One chunk of a file, and the most one object holds.
pub const CHUNK: usize = 1 << 20;
/// A name in a directory, as Linux's NAME_MAX.
pub const NAME_MAX: usize = 255;
/// A whole path on the wire, as Linux's PATH_MAX.
pub const PATH_MAX: usize = 4096;
/// Files and directories together, the root and its folders included.
pub const MAX_NODES: usize = 65_536;
/// The largest file: 65,536 chunks, so its chunk list is 512 KiB.
pub const MAX_FILE: u64 = 64 << 30;
/// Changed chunks held in RAM across every open file before the oldest is
/// written out: the broker's whole working set for file data.
pub const DIRTY_CHUNKS: usize = 16;
/// The most one read or write moves.
pub const IO_MAX: usize = 256 << 10;
/// Open files one client may hold, and all clients together.
pub const HANDLES_PER_CLIENT: usize = 256;
pub const HANDLES: usize = 4096;
/// A manifest body larger than this is refused at load: 65,536 nodes at
/// their largest names and chunk lists stay well under it.
pub const BODY_MAX: u64 = 64 << 20;
/// The bytes the person's files may hold together, unless the capsule sets
/// another quota. The volume's own ENOSPC still comes first when it is full.
pub const DEFAULT_QUOTA: u64 = 64 << 30;
/// The entries one directory listing reply carries at most.
pub const LIST_MAX: usize = 512;
