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

//! Descriptors and the store, as the server reaches them. A pool's
//! descriptor is mapped here as the client mapped it, read-only and shared,
//! so the server reads the pixels the client draws.

use super::guest::{note_mapped, Fd, Guest, Held, MemfdSpan};

pub mod flags {
    pub const O_CLOEXEC: u64 = 0o2000000;
}

/// A memfd's identity: the host file's, so the same descriptor or a dup of
/// it is the same memfd. Asked for a pool's descriptor, as the capsule asks
/// it, it is also where the host does what the client's mmap does there: the
/// file mapped whole, read-only and shared, and recorded as the memfd's
/// latest span. A file of no size yet is a memfd not mapped.
pub fn memfd_id(guest: &mut Guest, fd: u64) -> Option<u32> {
    let Held::Host(host) = guest.fds.get(fd as usize)?.held else { return None };
    // SAFETY: fstat and mmap of a descriptor this process holds; the
    // mapping is kept for the life of the process, as reads may reach it.
    unsafe {
        let mut st: libc::stat = core::mem::zeroed();
        if libc::fstat(host, &mut st) != 0 {
            return None;
        }
        // The inode alone: every memfd lives on the one internal mount, and
        // an inode that does not fit, or is zero, is no memfd named here.
        let id = u32::try_from(st.st_ino).ok().filter(|&i| i != 0)?;
        if st.st_size <= 0 {
            return Some(id);
        }
        let len = st.st_size as usize;
        let at = libc::mmap(core::ptr::null_mut(), len, libc::PROT_READ, libc::MAP_SHARED, host, 0);
        if at == libc::MAP_FAILED {
            return Some(id);
        }
        note_mapped(at as u64, len as u64);
        let span = MemfdSpan { id, at: at as u64, len: len as u64, off: 0 };
        let _ = super::guest::memfd_spans::record(&mut guest.memfd_spans, span);
        Some(id)
    }
}

/// Put `fd` in the first free slot.
pub fn install(guest: &mut Guest, fd: Fd) -> Option<u64> {
    match guest.fds.iter().position(|f| !f.is_open()) {
        Some(i) => {
            guest.fds[i] = fd;
            Some(i as u64)
        }
        None => {
            guest.fds.push(fd);
            Some(guest.fds.len() as u64 - 1)
        }
    }
}

/// A path in the store.
pub fn key(visible: &[u8]) -> Vec<u8> {
    visible.to_vec()
}

/// The store holds the keymap the image packs, from the tree.
pub fn store_read(key: &[u8], most: u32) -> Result<Vec<u8>, i64> {
    const KEYMAP: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../linux_userland/keymap-us.xkb");
    if key != b"/usr/share/nonos/keymap.xkb" {
        return Err(-2);
    }
    let mut bytes = std::fs::read(KEYMAP).map_err(|_| -2i64)?;
    bytes.truncate(most as usize);
    Ok(bytes)
}
