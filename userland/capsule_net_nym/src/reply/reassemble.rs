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

use alloc::vec::Vec;

use super::assembly::Assembly;
use crate::message::parse;

/// How many part-built messages may wait at once.
///
/// A browser fetches concurrently and an exit answers a TLS flight as dozens
/// of messages sent back to back, each split over two or more packets whose
/// mix delays are drawn separately, so many sets are half done at any moment.
/// Eight was not enough: a ninth set evicted the oldest, its missing pieces
/// arrived to find nothing, the message was gone for good, and the stream
/// behind it waited on a number nobody would send again.
pub const MAX_PENDING: usize = 64;

/// The largest message one set may rebuild to. A proxied read is a few
/// kilobytes; a set claiming far more is refused before anything is held for
/// it rather than discovered once it has filled the pool.
pub const MESSAGE_MAX: usize = 512 * 1024;

/// Payload bytes all part-built messages may hold together. Past it the set
/// heard from least recently is abandoned to make room.
pub const PENDING_BYTES_MAX: usize = 2 * 1024 * 1024;

/// How long a set may go without a fragment before it is abandoned. The far
/// end resends a lost fragment once its acknowledgement fails to return, so a
/// set that has heard nothing for this long has been given up on there too.
pub const STALE_MS: i64 = 90_000;

/// Sets recently completed, remembered so a late copy of one of their
/// fragments is not taken for the start of a new message. The far end
/// resends a fragment whose acknowledgement was slow, and the copy can land
/// after the message it belongs to has already been delivered.
pub const RECENT_DONE: usize = 256;

/// What one fragment amounted to.
pub enum Collected {
    /// It completed a message, which is handed back whole.
    Complete(Vec<u8>),
    /// It was placed; the message is still missing pieces.
    Held,
    /// A copy of a fragment already held or already delivered.
    Duplicate,
    /// Not a fragment, or one that claims more than this will hold.
    Refused,
}

/// Messages being put back together from the fragments they arrived as.
pub struct Reassembly {
    pending: Vec<Assembly>,
    bytes: usize,
    done: [i32; RECENT_DONE],
    done_len: usize,
    done_next: usize,
}

impl Default for Reassembly {
    fn default() -> Self {
        Self::new()
    }
}

impl Reassembly {
    pub const fn new() -> Self {
        Self { pending: Vec::new(), bytes: 0, done: [0; RECENT_DONE], done_len: 0, done_next: 0 }
    }

    /// Part-built messages held now.
    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    /// Payload bytes held for them.
    pub fn held_bytes(&self) -> usize {
        self.bytes
    }

    /// Take one fragment, and hand back the message once it completes one.
    ///
    /// The mixnet does not preserve order, so a fragment is placed by the
    /// position in its own header rather than by when it turned up. A repeat
    /// is dropped rather than counted twice, which would otherwise let a
    /// replayed fragment complete a message that is still missing a piece.
    pub fn collect(&mut self, fragment: &[u8], now_ms: i64) -> Collected {
        let Some((header, payload)) = parse(fragment) else {
            return Collected::Refused;
        };
        self.sweep(now_ms);
        if self.recently_done(header.set_id) {
            return Collected::Duplicate;
        }
        let at = header.current as usize - 1;

        match self.pending.iter().position(|a| a.holds(header.set_id)) {
            // One set has one size. A fragment naming a different one for a
            // set already under way is not part of it.
            Some(i) if self.pending[i].total != header.total => return Collected::Refused,
            Some(i) if self.pending[i].pieces.get(at).is_some_and(|p| p.is_some()) => {
                return Collected::Duplicate;
            }
            Some(_) => {}
            None => {
                let Some(projected) = (header.total as usize).checked_mul(payload.len()) else {
                    return Collected::Refused;
                };
                if projected > MESSAGE_MAX {
                    return Collected::Refused;
                }
                self.make_room(None, projected);
                self.pending.push(Assembly::new(header.set_id, header.total, now_ms));
            }
        }

        // The pool is bounded by what it holds, checked as each piece lands:
        // a set's size is only known for certain once all of it is in.
        if self.bytes.saturating_add(payload.len()) > PENDING_BYTES_MAX {
            self.make_room(Some(header.set_id), payload.len());
        }
        if self.bytes.saturating_add(payload.len()) > PENDING_BYTES_MAX {
            return Collected::Refused;
        }
        let Some(idx) = self.pending.iter().position(|a| a.holds(header.set_id)) else {
            return Collected::Refused;
        };
        let assembly = &mut self.pending[idx];
        let Some(grown) = assembly.bytes.checked_add(payload.len()) else {
            return Collected::Refused;
        };
        if grown > MESSAGE_MAX {
            return Collected::Refused;
        }
        let Some(place) = assembly.pieces.get_mut(at) else {
            return Collected::Refused;
        };
        *place = Some(payload.to_vec());
        assembly.held += 1;
        assembly.bytes = grown;
        assembly.touched_ms = now_ms;
        self.bytes = self.bytes.saturating_add(payload.len());

        if assembly.held < assembly.total {
            return Collected::Held;
        }
        let done = self.pending.remove(idx);
        self.bytes = self.bytes.saturating_sub(done.bytes);
        self.remember_done(done.set_id);
        let mut out = Vec::with_capacity(done.bytes);
        for piece in done.pieces.iter() {
            match piece {
                Some(piece) => out.extend_from_slice(piece),
                None => return Collected::Refused,
            }
        }
        Collected::Complete(out)
    }

    /// Abandon sets that have heard nothing for too long.
    fn sweep(&mut self, now_ms: i64) {
        let mut freed = 0usize;
        self.pending.retain(|a| {
            let keep = now_ms.saturating_sub(a.touched_ms) <= STALE_MS;
            if !keep {
                freed = freed.saturating_add(a.bytes);
            }
            keep
        });
        self.bytes = self.bytes.saturating_sub(freed);
    }

    /// Abandon the sets heard from least recently, other than `keep`, until
    /// `need` more bytes fit and, for a new set, there is a slot to put it in.
    fn make_room(&mut self, keep: Option<i32>, need: usize) {
        loop {
            let crowded = keep.is_none() && self.pending.len() >= MAX_PENDING;
            let heavy = self.bytes.saturating_add(need) > PENDING_BYTES_MAX;
            if !crowded && !heavy {
                return;
            }
            let mut oldest: Option<usize> = None;
            for (i, a) in self.pending.iter().enumerate() {
                if Some(a.set_id) == keep {
                    continue;
                }
                if oldest.is_none_or(|o| a.touched_ms < self.pending[o].touched_ms) {
                    oldest = Some(i);
                }
            }
            let Some(oldest) = oldest else { return };
            let gone = self.pending.remove(oldest);
            self.bytes = self.bytes.saturating_sub(gone.bytes);
        }
    }

    fn recently_done(&self, set_id: i32) -> bool {
        self.done[..self.done_len].contains(&set_id)
    }

    fn remember_done(&mut self, set_id: i32) {
        self.done[self.done_next] = set_id;
        self.done_next = (self.done_next + 1) % RECENT_DONE;
        if self.done_len < RECENT_DONE {
            self.done_len += 1;
        }
    }
}
