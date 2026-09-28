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

//! The last chunk each stream reader was handed, kept until it asks for the
//! next one.
//!
//! A read takes bytes out of the socket before the reply carrying them is
//! delivered, and the kernel drops a reply whose caller has already timed
//! out. A reader that gave up on one reply therefore lost those bytes for
//! good: the browser, which waits 200 ms per read, lost the first 23,854
//! bytes of an 89,386 byte page this way and was left with 65,532 bytes and
//! no headers. A reader that numbers its reads can ask again for the one it
//! never saw, and gets the same bytes; asking for the next number releases
//! them. A reader that sends no number reads as it always did.

use alloc::vec::Vec;

use spin::Mutex;

use crate::sockets::SocketKey;

struct Kept {
    key: SocketKey,
    seq: u32,
    data: Vec<u8>,
}

static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

fn same(a: SocketKey, b: SocketKey) -> bool {
    a.pid == b.pid && a.handle == b.handle
}

/// The bytes already handed out for read `seq`, copied into `out`, when that
/// is the read being asked for again. A different number means the reader
/// has what it was sent, so the kept copy is released.
pub fn again(key: SocketKey, seq: u32, out: &mut [u8]) -> Option<usize> {
    let mut kept = KEPT.lock();
    let at = kept.iter().position(|k| same(k.key, key))?;
    if kept[at].seq != seq {
        kept.swap_remove(at);
        return None;
    }
    let n = kept[at].data.len().min(out.len());
    out[..n].copy_from_slice(&kept[at].data[..n]);
    Some(n)
}

/// Keep what read `seq` handed out until the reader moves past it.
pub fn keep(key: SocketKey, seq: u32, data: &[u8]) {
    let mut kept = KEPT.lock();
    kept.retain(|k| !same(k.key, key));
    kept.push(Kept { key, seq, data: data.to_vec() });
}

/// Forget a socket's kept chunk; it is closing, and its handle may be reused.
pub fn release(key: SocketKey) {
    KEPT.lock().retain(|k| !same(k.key, key));
}
