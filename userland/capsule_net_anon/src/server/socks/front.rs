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

//! SOCKS callers, each with its conversations and their last answers.

extern crate alloc;

use alloc::vec::Vec;

use super::conv::Conv;
use super::frame::{ask, Ask, FIRST_SEQ};
use super::kept::{Again, Kept};
use super::reply::{encode, progress, REPLY_CLOSED, REPLY_LOST, REPLY_OPEN};
use super::stage::OUT_MAX;
use super::tunnel::Tunnel;
use super::who::{pid_of, who, Who, UNNAMED};

/// Conversations served at once, of every caller together. One page load
/// is a few conversations of the browser's (`who::STREAMS_PER_CALLER` at
/// most), so this is shared among the capsules using the network.
pub const CALLERS_MAX: usize = 32;

#[derive(Default)]
pub struct Front {
    pub(super) held: Vec<(Who, Conv)>,
    kept: Kept,
}

impl Front {
    /// Answer one frame from `pid`. Every frame is answered, with at least
    /// the marker, because the caller is blocked on the reply. A frame that
    /// names a stream is that stream's conversation; one that names none is
    /// stream 0's, as every frame was before streams were named.
    pub fn serve(&mut self, tunnel: &mut impl Tunnel, pid: u32, frame: &[u8]) -> Vec<u8> {
        let unnamed = who(pid, UNNAMED);
        match ask(frame) {
            Some(Ask::Stream(data)) => self.step(tunnel, unnamed, data, OUT_MAX),
            Some(Ask::Numbered(seq, data)) => self.numbered(tunnel, unnamed, seq, data),
            Some(Ask::NumberedOn(stream, seq, data)) => {
                self.numbered(tunnel, who(pid, stream), seq, data)
            }
            Some(Ask::ResetOn(stream)) => self.end(tunnel, who(pid, stream), false),
            Some(Ask::Reset) => self.end(tunnel, unnamed, false),
            Some(Ask::Status) => progress(tunnel.progress()),
            /* An unknown frame ends the conversation at once, so the caller
             * fails now rather than at its timeout. */
            None => self.end(tunnel, unnamed, true),
        }
    }

    /// End `w`'s conversation and forget its kept answer.
    fn end(&mut self, tunnel: &mut impl Tunnel, w: Who, closed: bool) -> Vec<u8> {
        self.forget(tunnel, w);
        self.kept.forget(w);
        encode(closed, &[])
    }

    /// Let go of every conversation of a caller `alive` says has ended. Only
    /// a caller resets its own conversations, so one that crashed kept its
    /// places and its streams open for good, and after CALLERS_MAX of them
    /// the Anyone front refused every program.
    pub fn reap(&mut self, tunnel: &mut impl Tunnel, alive: impl Fn(u32) -> bool) {
        let ended: Vec<Who> =
            self.held.iter().map(|h| h.0).filter(|&w| !alive(pid_of(w))).collect();
        for w in ended {
            self.forget(tunnel, w);
        }
        self.kept.forget_ended(alive);
    }

    /// A numbered exchange, by the rule in `kept`: the kept answer for a
    /// repeat, new bytes carried even under a number whose answer was lost
    /// with that answer in front of theirs, and nothing new carried once the
    /// stream has ended.
    ///
    /// A later exchange of a conversation not held, with no answer kept for
    /// it, was lost (this capsule restarted under it, or ended and forgot
    /// it), and is said to be. Begun afresh, its stream bytes would be read
    /// as a greeting and closed, which the caller could only take for the
    /// exit hanging up.
    fn numbered(&mut self, tunnel: &mut impl Tunnel, w: Who, seq: u32, data: &[u8]) -> Vec<u8> {
        let held = self.held.iter().any(|h| h.0 == w);
        let reply = match self.kept.again(w, seq, data) {
            Again::New if seq != FIRST_SEQ && !held => return Vec::from([REPLY_LOST]),
            Again::Same(reply) => return reply,
            /* A stream that ended has nowhere to take new bytes; the close
             * the caller missed is its answer. */
            Again::Missed(reply) if reply.first() == Some(&REPLY_CLOSED) => return reply,
            Again::Missed(reply) => {
                let earlier = reply.get(1..).unwrap_or_default();
                let now = self.step(tunnel, w, data, OUT_MAX.saturating_sub(earlier.len()));
                let mut out = Vec::with_capacity(earlier.len() + now.len());
                out.push(now.first().copied().unwrap_or(REPLY_OPEN));
                out.extend_from_slice(earlier);
                out.extend_from_slice(now.get(1..).unwrap_or_default());
                out
            }
            Again::New => self.step(tunnel, w, data, OUT_MAX),
        };
        self.kept.keep(w, seq, data, &reply);
        reply
    }
}
