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

use super::room::slot_for;
use super::types::RESIDUAL;
use crate::server::handlers::mixnet_frame::MAX_BODY;
use crate::sockets::SocketKey;

/// Hold what a read could not take.
///
/// A frame is gone from the mixnet queue once it has been read, so a caller
/// with a smaller buffer than the frame would lose the difference outright.
/// Keeping it means the next read continues the stream rather than skipping
/// part of it. Returns false when the remainder could not be held, which is
/// the one case where bytes are still dropped and so is worth telling apart
/// from a read that simply came up short.
pub fn store(key: SocketKey, rest: &[u8]) -> bool {
    if rest.is_empty() {
        return true;
    }
    // A remainder is part of one frame's body, so it is never longer than the
    // slot that holds it (`[u8; MAX_BODY]`). `decode` already bounds a frame's
    // body to MAX_BODY, so every remainder today is within it; refuse anything
    // longer here rather than let the `copy_from_slice` below run past the
    // slot, keeping the bound local to the copy it protects should the reader
    // buffer or `decode` ever change.
    if rest.len() > MAX_BODY {
        return false;
    }
    let mut slots = RESIDUAL.lock();
    let Some(at) = slot_for(&slots[..], key) else {
        return false;
    };
    let slot = &mut slots[at];
    slot.pid = key.pid;
    slot.handle = key.handle;
    slot.buf[..rest.len()].copy_from_slice(rest);
    slot.len = rest.len();
    slot.off = 0;
    true
}
