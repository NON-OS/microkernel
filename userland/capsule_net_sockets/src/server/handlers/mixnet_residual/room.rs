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

use super::types::{Slot, PER_PID_MAX, RESIDUAL};
use crate::server::handlers::mixnet_frame::MAX_BODY;
use crate::sockets::SocketKey;

/// The slot `store` keeps `key`'s remainder in, while its process holds
/// bytes in fewer than PER_PID_MAX: the one the socket already holds while
/// it is drained, else the first free one. None when the socket still has
/// bytes held, its process is at that bound, or every slot is held.
pub(super) fn slot_for(slots: &[Slot], key: SocketKey) -> Option<usize> {
    let owned = |s: &Slot| s.pid == key.pid && s.handle == key.handle;
    let at = slots.iter().position(owned);
    // Bytes still held come earlier in the stream and cannot be dropped to
    // make room for later ones.
    if at.is_some_and(|i| slots[i].off < slots[i].len) {
        return None;
    }
    // A drained slot the socket kept counts against its process as a free
    // one does, so drained slots refilled cannot carry it past its share.
    if held_by(slots, key.pid) >= PER_PID_MAX {
        return None;
    }
    at.or_else(|| slots.iter().position(|s| s.off >= s.len))
}

/// How many slots hold bytes for process `pid`.
fn held_by(slots: &[Slot], pid: u32) -> usize {
    slots.iter().filter(|s| s.pid == pid && s.off < s.len).count()
}

/// Whether a read of `out_len` bytes on `key` may pull a frame off the
/// mixnet queue. A buffer that takes any body whole leaves nothing to keep;
/// a smaller one needs the slot `store` would keep the rest in. Asked before
/// the frame is pulled, so when the table is full the read is refused with
/// the frame still queued, rather than the frame pulled and its rest lost.
pub fn room(key: SocketKey, out_len: usize) -> bool {
    out_len >= MAX_BODY || slot_for(&RESIDUAL.lock()[..], key).is_some()
}
