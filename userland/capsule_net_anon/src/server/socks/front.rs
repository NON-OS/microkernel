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

//! SOCKS callers, each with its conversation and its last answer.

extern crate alloc;

use alloc::vec::Vec;

use super::conv::Conv;
use super::frame::{ask, Ask};
use super::kept::Kept;
use super::reply::encode;
use super::tunnel::Tunnel;

/// Callers served at once. One page load is one conversation per capsule,
/// so this is the number of capsules using the network together.
pub const CALLERS_MAX: usize = 32;

#[derive(Default)]
pub struct Front {
    pub(super) held: Vec<(u32, Conv)>,
    kept: Kept,
}

impl Front {
    /// Answer one frame from `pid`. Every frame is answered, with at least
    /// the marker, because the caller is blocked on the reply.
    pub fn serve(&mut self, tunnel: &mut impl Tunnel, pid: u32, frame: &[u8]) -> Vec<u8> {
        match ask(frame) {
            Some(Ask::Stream(data)) => self.step(tunnel, pid, data),
            Some(Ask::Numbered(seq, data)) => self.kept.again(pid, seq).unwrap_or_else(|| {
                let reply = self.step(tunnel, pid, data);
                self.kept.keep(pid, seq, &reply);
                reply
            }),
            /* An unknown frame ends the conversation at once, so the caller
             * fails now rather than at its timeout. */
            other => {
                self.forget(tunnel, pid);
                self.kept.forget(pid);
                encode(other.is_none(), &[])
            }
        }
    }
}
