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

//! Connections that arrive while a session runs. nsh.front serves one
//! session at a time, so each newcomer is told a plaintext `ABORT(BUSY)`,
//! nsh/1's answer from a host at its connection limit, instead of waiting
//! unanswered in the backlog until it gives up. Each one told is held, and
//! what it sends is read and dropped, until its peer has gone or `HOLD_MS`
//! has passed, so the close after the answer does not reset the connection
//! before the client reads it. At most `HELD_MAX` are held; one more is
//! closed as soon as it is told. Times are the monotonic uptime, which never
//! steps back and is there before the wall clock is set. Pure, so
//! nsh_front_proofs holds it.

use crate::read::Read;

/// How often a running session asks the listener for a newcomer and reads
/// the ones it holds, so a stream of connections costs the session at most
/// one round of calls each period.
pub const ASK_EVERY_MS: u64 = 250;
/// The longest a newcomer told BUSY is held for its hello.
pub const HOLD_MS: u64 = 2_000;
/// How many newcomers told BUSY are held at once.
pub const HELD_MAX: usize = 4;
/// How long the wait after a session parks between reads of the newcomers it
/// still holds.
pub const LET_GO_IDLE_MS: u64 = 10;
/// The most reads that wait makes of each one before it closes whatever is
/// left: HOLD_MS of parks, so it ends even on a clock that does not move.
pub const LET_GO_PASSES: u64 = HOLD_MS / LET_GO_IDLE_MS;

/// The newcomers told BUSY and still held, when the listener was last
/// asked, and the number of the round, which numbers each read of a held
/// newcomer so net.sockets hands out fresh bytes rather than the last ones.
pub struct Away {
    held: [Option<(u32, u64)>; HELD_MAX],
    asked_ms: Option<u64>,
    round: u32,
    lost: bool,
}

impl Default for Away {
    fn default() -> Away {
        Away::new()
    }
}

impl Away {
    pub const fn new() -> Away {
        Away { held: [None; HELD_MAX], asked_ms: None, round: 0, lost: false }
    }

    /// Whether a round is due at `now`; a due round is counted from `now`.
    /// None is due once the listener is gone.
    pub fn due(&mut self, now: u64) -> bool {
        if self.lost {
            return false;
        }
        let due = match self.asked_ms {
            None => true,
            Some(at) => now.saturating_sub(at) >= ASK_EVERY_MS,
        };
        if due {
            self.asked_ms = Some(now);
            self.next_round();
        }
        due
    }

    /// Start a round of reads that asks the listener nothing: waiting out
    /// the ones held once the session is over.
    pub fn next_round(&mut self) {
        self.round = self.round.wrapping_add(1).max(1);
    }

    /// The number this round's reads carry, from 1 as the session's own
    /// reads are numbered; two rounds in a row never share one.
    pub fn seq(&self) -> u32 {
        self.round.max(1)
    }

    /// net.sockets said the listener is gone: no newcomer is asked for again
    /// in this session, and the accept loop opens a new listener after it.
    pub fn lose_listener(&mut self) {
        self.lost = true;
    }

    /// Whether the listener is still there, as far as this session knows.
    pub fn listening(&self) -> bool {
        !self.lost
    }

    /// Hold `handle`, told BUSY at `now`. It comes back when every slot is
    /// taken, for the caller to close at once.
    pub fn hold(&mut self, handle: u32, now: u64) -> Option<u32> {
        match self.held.iter_mut().find(|slot| slot.is_none()) {
            Some(slot) => {
                *slot = Some((handle, now));
                None
            }
            None => Some(handle),
        }
    }

    /// The handles held, for the caller to read once each.
    pub fn held(&self) -> [Option<u32>; HELD_MAX] {
        self.held.map(|slot| slot.map(|(handle, _)| handle))
    }

    /// Whether `handle` is done, given what one read of it found at `now`:
    /// its peer went, or it was held `HOLD_MS`. Bytes it sent keep it held:
    /// the client closes once it has read the answer. A done handle leaves
    /// its slot, for the caller to close.
    pub fn settle(&mut self, handle: u32, read: Read, now: u64) -> bool {
        let Some(slot) = self.held.iter_mut().find(|s| matches!(s, Some((h, _)) if *h == handle))
        else {
            return false;
        };
        let since = slot.map_or(now, |(_, at)| at);
        let done = read == Read::Gone || now.saturating_sub(since) >= HOLD_MS;
        if done {
            *slot = None;
        }
        done
    }

    /// Whether any newcomer is still held.
    pub fn holding(&self) -> bool {
        self.held.iter().any(Option::is_some)
    }

    /// Every newcomer still held, each leaving its slot, for the caller to
    /// close now.
    pub fn release_all(&mut self) -> [Option<u32>; HELD_MAX] {
        let left = self.held();
        self.held = [None; HELD_MAX];
        left
    }
}
