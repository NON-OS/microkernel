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

//! The fragments sent and not yet acknowledged.
//!
//! The mixnet loses packets. A message is split across many of them, and the
//! far end rebuilds it only when every one has arrived: a request to an exit
//! carries its reply blocks with it and runs to a dozen packets or more, so a
//! loss of a few percent at each hop leaves most requests short of one
//! fragment. The exit then holds a message it can never finish and answers
//! nothing, which from here reads exactly like an exit that is not there.
//!
//! The reference client keeps every fragment until its acknowledgement comes
//! home and sends it again, on a fresh route, when it does not. This is that
//! record. It holds the fragment as plaintext, never the packet: a packet sent
//! twice would be the same bytes on the wire twice, and a replay that every
//! mix refuses. A resend is sealed anew each time.

use alloc::vec::Vec;

/// Bytes naming a fragment: its set id and its position.
pub const LEDGER_FRAG_ID: usize = 5;

/// Bytes naming a recipient: identity, encryption key, gateway.
pub const LEDGER_RECIPIENT: usize = 96;

/// Sends a fragment gets in all, the first one included.
///
/// The reference client resends without a limit. Here a fragment that has not
/// arrived after this many tries, with the wait doubling between them, is past
/// the minute a reader waits for an answer, and holding it longer only keeps
/// memory for a stream that has already been given up on.
pub const MAX_SENDS: u8 = 6;

/// The most fragments held at once, and the most plaintext bytes.
///
/// A top up of a hundred reply blocks is about seventy fragments, so this is
/// a few such messages in flight. Past it the oldest is let go: it has had the
/// longest to arrive, and a ledger that grew without a bound would turn a
/// lossy network into an exhausted heap.
pub const MAX_HELD: usize = 256;
pub const MAX_HELD_BYTES: usize = 640 * 1024;

/// Longest wait between two sends of one fragment.
pub const MAX_WAIT_MS: i64 = 16_000;

/// One fragment waiting on its acknowledgement.
pub struct Outstanding {
    pub frag_id: [u8; LEDGER_FRAG_ID],
    pub recipient: [u8; LEDGER_RECIPIENT],
    pub fragment: Vec<u8>,
    /// When it is resent if no acknowledgement has come.
    pub due_ms: i64,
    /// How long the last send was given.
    pub wait_ms: i64,
    pub sends: u8,
}

/// A fragment to send again, taken out of the ledger until it is.
pub struct Resend {
    pub frag_id: [u8; LEDGER_FRAG_ID],
    pub fragment: Vec<u8>,
    pub sends: u8,
    pub wait_ms: i64,
}

/// What a sweep found.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Swept {
    /// Fragments for a recipient no longer in use, dropped.
    pub stale: usize,
    /// Fragments out of tries, dropped.
    pub given_up: usize,
}

pub struct Ledger {
    held: Vec<Outstanding>,
    bytes: usize,
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}

impl Ledger {
    pub const fn new() -> Self {
        Self { held: Vec::new(), bytes: 0 }
    }

    /// Fragments held.
    pub fn len(&self) -> usize {
        self.held.len()
    }

    pub fn is_empty(&self) -> bool {
        self.held.is_empty()
    }

    /// Plaintext bytes held.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// When the next fragment falls due, if any is held. Lets the idle loop
    /// pass over the ledger with one comparison when nothing is late.
    pub fn next_due(&self) -> Option<i64> {
        self.held.iter().map(|o| o.due_ms).min()
    }

    /// Record a fragment that has just gone out for the `sends`th time.
    ///
    /// Returns how many older fragments were let go to make room.
    pub fn sent(
        &mut self,
        frag_id: [u8; LEDGER_FRAG_ID],
        recipient: [u8; LEDGER_RECIPIENT],
        fragment: Vec<u8>,
        now_ms: i64,
        wait_ms: i64,
        sends: u8,
    ) -> usize {
        // The same fragment again replaces its record rather than doubling it.
        self.acked(&frag_id);
        let wait_ms = wait_ms.clamp(0, MAX_WAIT_MS);
        self.bytes += fragment.len();
        self.held.push(Outstanding {
            frag_id,
            recipient,
            fragment,
            due_ms: now_ms.saturating_add(wait_ms),
            wait_ms,
            sends,
        });
        let mut evicted = 0;
        while self.held.len() > MAX_HELD || (self.bytes > MAX_HELD_BYTES && self.held.len() > 1) {
            let oldest = self.held.remove(0);
            self.bytes -= oldest.fragment.len();
            evicted += 1;
        }
        evicted
    }

    /// The acknowledgement for `frag_id` came home. Returns whether it was
    /// one this ledger was waiting on: an acknowledgement for a fragment
    /// already resent and answered twice is normal and changes nothing.
    pub fn acked(&mut self, frag_id: &[u8; LEDGER_FRAG_ID]) -> bool {
        let Some(at) = self.held.iter().position(|o| &o.frag_id == frag_id) else {
            return false;
        };
        let done = self.held.remove(at);
        self.bytes -= done.fragment.len();
        true
    }

    /// Take every fragment whose wait has run out, for `current` recipient.
    ///
    /// A fragment for any other recipient is dropped: the session moved to
    /// another exit, and the old one is not going to be asked for anything
    /// again. A fragment that has had all its sends is dropped too. What is
    /// returned is out of the ledger until `sent` puts it back, so a resend
    /// that could not be built is simply not tried again.
    pub fn due(
        &mut self,
        now_ms: i64,
        current: Option<&[u8; LEDGER_RECIPIENT]>,
        out: &mut Vec<Resend>,
    ) -> Swept {
        let mut swept = Swept::default();
        let mut at = 0;
        while at < self.held.len() {
            let o = &self.held[at];
            let stale = current != Some(&o.recipient);
            let ready = now_ms >= o.due_ms;
            if !stale && !ready {
                at += 1;
                continue;
            }
            let o = self.held.remove(at);
            self.bytes -= o.fragment.len();
            if stale {
                swept.stale += 1;
            } else if o.sends >= MAX_SENDS {
                swept.given_up += 1;
            } else {
                out.push(Resend {
                    frag_id: o.frag_id,
                    fragment: o.fragment,
                    sends: o.sends,
                    wait_ms: o.wait_ms,
                });
            }
        }
        swept
    }
}

/// The wait the reference client gives a fragment before resending it.
///
/// The packet's own mixing delay out, the acknowledgement's delay home, half
/// as much again for the variance, and a fixed allowance for the gateways
/// either end. The floor is this capsule's own: it reads the link on an idle
/// tick, so an acknowledgement can sit unread for most of a second.
pub fn first_wait_ms(forward_delay_ms: u64, ack_delay_ms: u64) -> i64 {
    const ADDITION_MS: i64 = 1_500;
    const FLOOR_MS: i64 = 3_000;
    let delay = forward_delay_ms.saturating_add(ack_delay_ms).min(i64::MAX as u64 / 2) as i64;
    (delay + delay / 2 + ADDITION_MS).clamp(FLOOR_MS, MAX_WAIT_MS)
}

/// The wait after a resend: twice the last one, up to the cap.
///
/// A fragment lost once is likely to meet the same trouble again soon after,
/// a mix restarting or a gateway shedding load, and resending into it at the
/// same pace only adds to the load.
pub fn next_wait_ms(last_ms: i64) -> i64 {
    last_ms.saturating_mul(2).clamp(0, MAX_WAIT_MS)
}
