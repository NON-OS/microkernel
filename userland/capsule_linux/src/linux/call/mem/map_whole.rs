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

//! The whole of a file a guest is mapping executable, read through the
//! descriptor it named. The loader has just mapped the same file without
//! exec through that descriptor (map_fill.rs), and a packed file's stream
//! keeps every page it read, each already checked against its digest
//! (file/store/stream.rs): so only the pages that mapping did not cover cross
//! to the store again, and a library is read once instead of twice. Whatever
//! comes back is only bytes in hand: map_exec.rs proves exactly these bytes
//! and maps exactly these, so where they came from decides nothing.

use alloc::vec::Vec;

use crate::linux::guest::Fd;

/// The descriptor's file whole, up to `max` bytes, from its own stream; None
/// when it has none to read from, was opened to write, has writes not yet on
/// the store, is larger than `max`, or reads back another length than it had
/// when opened. The caller then reads the store as before.
pub fn whole(entry: &mut Fd, max: u32) -> Option<Vec<u8>> {
    if entry.writable || !entry.pending.is_empty() || entry.size > u64::from(max) {
        return None;
    }
    let size = entry.size;
    let bytes = entry.stream.as_mut()?.read_window(0, size as u32).ok()?;
    (bytes.len() as u64 == size).then_some(bytes)
}
