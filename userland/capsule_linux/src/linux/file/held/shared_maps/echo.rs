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

//! A file's new bytes shown in every shared map of it in the family, as the
//! page cache shows them on Linux: a peer write into each process, which
//! the kernel makes whatever the page's protection.

use nonos_libc::peer::mk_peer_write;

use super::spans::{echo, zeroed};
use super::table::picked;
use crate::linux::guest::MAX_SPAN;

/// Zeroes written a piece at a time.
const CHUNK: u64 = 64 << 10;

/// `bytes`, just written at `at` of `path`, into every map that shows them.
pub fn wrote(path: &[u8], at: u64, bytes: &[u8]) {
    for e in picked(|e| e.shows(path)) {
        if let Some((addr, from, to)) = echo(&e, at, bytes.len() as u64) {
            /* A process that is ending refuses it, and reads nothing more. */
            put(e.pid, addr, &bytes[from..to]);
        }
    }
}

/// `path` cut or grown from `was` to `len` bytes: its maps read zero
/// between the two ends.
pub fn truncated(path: &[u8], was: u64, len: u64) {
    let maps = picked(|e| e.shows(path));
    let zeros = alloc::vec![0u8; if maps.is_empty() { 0 } else { CHUNK as usize }];
    for e in &maps {
        let Some((addr, n)) = zeroed(e, was, len) else {
            continue;
        };
        let mut done = 0;
        while done < n {
            let take = (n - done).min(CHUNK);
            put(e.pid, addr + done, &zeros[..take as usize]);
            done += take;
        }
    }
}

/// `bytes` into process `pid` at `addr`, a peer call's most at a time.
fn put(pid: u32, addr: u64, bytes: &[u8]) {
    for (i, piece) in bytes.chunks(MAX_SPAN as usize).enumerate() {
        if mk_peer_write(pid, addr + (i as u64) * MAX_SPAN, piece) < 0 {
            return;
        }
    }
}
