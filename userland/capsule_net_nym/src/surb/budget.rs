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

//! How many reply blocks the far end holds for us, and when to send more.
//!
//! Every packet an exit sends back spends one block. It keeps a reserve it
//! will not spend, and once down to it, it stops and asks for more, which
//! costs a full round trip before another byte comes back. A 100 KB answer
//! is well over a hundred packets, far past what a few requests carry, so
//! without more blocks arriving ahead of need the answer stalls at every
//! reserve. This keeps an estimate of what the far end holds, counting what
//! went out and what came back, and sends more while it is still answering.
//! The estimate only errs high (a reply lost on the way spent a block we
//! never see), and the far end asking is the correction for that.

/// Blocks the far end holds back and will not spend.
pub const RESERVE: u32 = 10;

/// Blocks a request carries while the far end holds fewer than
/// `HIGH_WATER`.
pub const PER_REQUEST: u32 = 24;

/// Past this many held, requests carry none. Each block is a kilobyte on the
/// wire, and blocks the far end never spends only pile up there.
pub const HIGH_WATER: u32 = 300;

/// While answers are arriving and the far end is believed to hold fewer
/// than this, more are sent unasked.
///
/// The estimate counts blocks the far end has already spent on packets
/// still crossing the mixnet, and a top up takes a crossing to arrive, so
/// low has to mean low enough to last two crossings: about fifty packets a
/// second for a second each way, on top of the reserve.
pub const LOW_WATER: u32 = RESERVE + 100;

/// How many are sent unasked: more than the far end spends in the spacing
/// below, so a steady answer is never left waiting on its reserve.
pub const TOP_UP: u32 = 100;

/// The most sent in answer to one request for more. A requester asks for
/// what its queue needs, up to its own maximum of a hundred.
pub const ANSWER_MAX: u32 = 100;

/// The fewest sent in answer to one, so a request for very few still leaves
/// the far end room past its reserve.
pub const ANSWER_MIN: u32 = 20;

/// The least time between two unasked top ups. One is already on its way
/// for this long, and the count it added is what the next decision reads.
pub const SPACING_MS: i64 = 1_000;

#[derive(Clone, Copy)]
pub struct Budget {
    held: u32,
    last_top_up_ms: Option<i64>,
}

impl Default for Budget {
    fn default() -> Self {
        Self::new()
    }
}

impl Budget {
    pub const fn new() -> Self {
        Self { held: 0, last_top_up_ms: None }
    }

    /// Blocks the far end is believed to hold.
    pub fn held(&self) -> u32 {
        self.held
    }

    /// How many blocks the next request should carry.
    pub fn for_request(&self) -> u32 {
        HIGH_WATER.saturating_sub(self.held).min(PER_REQUEST)
    }

    /// `count` blocks went out, with a request or as a top up.
    pub fn sent(&mut self, count: u32) {
        self.held = self.held.saturating_add(count);
    }

    /// A packet came back on one of our blocks.
    pub fn spent(&mut self) {
        self.held = self.held.saturating_sub(1);
    }

    /// The far end asked for `amount` more. It only asks once down to its
    /// reserve, so that is what it holds now, whatever the estimate said.
    /// Returns how many to send.
    pub fn asked(&mut self, amount: u32) -> u32 {
        self.held = self.held.min(RESERVE);
        amount.clamp(ANSWER_MIN, ANSWER_MAX)
    }

    /// How many to send unasked now, if any: only while answers are coming
    /// back, which is when this is consulted, and only once the far end is
    /// believed to be running low.
    pub fn top_up_due(&self, now_ms: i64) -> Option<u32> {
        if self.held >= LOW_WATER {
            return None;
        }
        match self.last_top_up_ms {
            Some(last) if now_ms.saturating_sub(last) < SPACING_MS => None,
            _ => Some(TOP_UP),
        }
    }

    /// An unasked top up of `count` went out at `now_ms`.
    pub fn topped_up(&mut self, count: u32, now_ms: i64) {
        self.sent(count);
        self.last_top_up_ms = Some(now_ms);
    }
}
