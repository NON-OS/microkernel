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

//! A POSIX shared memory object mapped MAP_SHARED: shm_open makes a file
//! under /dev/shm, the family's own, and a Wayland client (swayimg) maps it
//! to draw into and hands its descriptor to the display. Shared file
//! mappings are otherwise refused, since nothing gives two address spaces
//! one frame (call/mem/map_kind.rs). Such a mapping is what a memfd's is:
//! pages in this process, found by the descriptor (memfd_map.rs). So the
//! descriptor becomes a memfd holding the object's bytes, and its file is
//! written out and let go, as on close.
//!
//! What it does not do: a second descriptor on the same name, or another
//! process, does not see what is drawn. Each mapping is its own pages.

use alloc::vec::Vec;

use crate::linux::guest::{Guest, Kind};

use super::held::rw::{read_at, MAX_IO};

const SHM: &[u8] = b"/dev/shm/";

/// Whether `fd` holds a file under /dev/shm, mapped from its start.
pub fn is_shm_object(guest: &Guest, fd: u64, off: u64) -> bool {
    let file = |e: &crate::linux::guest::Fd| e.kind == Kind::File && e.path.starts_with(SHM);
    off == 0 && guest.fds.get(fd as usize).is_some_and(file)
}

/// The descriptor made a memfd of the object's size, its bytes staged for
/// the mapping, or the errno that stopped it.
pub fn into_memfd(guest: &mut Guest, fd: u64) -> Result<(), i64> {
    let size = guest.fds.get(fd as usize).map_or(0, |e| e.size);
    let mut bytes = Vec::new();
    while (bytes.len() as u64) < size {
        let chunk = read_at(guest, fd, bytes.len() as u64, MAX_IO)?;
        if chunk.is_empty() {
            break;
        }
        bytes.extend_from_slice(&chunk);
    }
    super::close::flush(guest, fd)?;
    let Some(entry) = guest.fds.get_mut(fd as usize) else { return Ok(()) };
    entry.kind = Kind::Memfd;
    entry.offset = 0;
    entry.stream = None;
    // Fresh pages are zeros already; only bytes that say something are copied.
    entry.pending = if bytes.iter().any(|b| *b != 0) { bytes } else { Vec::new() };
    Ok(())
}
