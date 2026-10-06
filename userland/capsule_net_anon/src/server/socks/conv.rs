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

//! One caller's SOCKS conversation, from greeting to relay.

extern crate alloc;

use alloc::vec::Vec;

use super::stage::{Stage, Step, UNSENT_MAX};
use super::tunnel::Tunnel;

#[derive(Default)]
pub struct Conv {
    pub(super) stage: Stage,
    /// Bytes from the caller not yet read or not yet sent.
    pub(super) unsent: Vec<u8>,
}

impl Conv {
    /// The stream this conversation holds, if it opened one.
    pub fn stream(&self) -> Option<u16> {
        match self.stage {
            Stage::Connecting(id) | Stage::Relay(id) => Some(id),
            _ => None,
        }
    }

    /// Take `data` from the caller and move the conversation as far as it
    /// will go. Returns whether it is over, and what to answer. An empty
    /// `data` asks whether anything has arrived, and is also when bytes held
    /// back for want of a window are retried. The answer brings back at most
    /// `room` stream bytes, the rest waiting for the next one.
    pub fn feed(&mut self, tunnel: &mut impl Tunnel, data: &[u8], room: usize) -> (bool, Vec<u8>) {
        let mut out = Vec::new();
        if self.unsent.len() + data.len() > UNSENT_MAX {
            return (true, out);
        }
        self.unsent.extend_from_slice(data);
        loop {
            let step = match self.stage {
                Stage::Greeting => self.greet(&mut out),
                Stage::Request => self.request(tunnel, &mut out),
                Stage::Connecting(id) => self.connecting(tunnel, id, &mut out),
                Stage::Relay(id) => self.relay(tunnel, id, &mut out, room),
            };
            match step {
                Step::Next => {}
                Step::Wait => return (false, out),
                Step::Over => return (true, out),
            }
        }
    }
}
