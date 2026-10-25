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

//! Reading an output's capabilities string, one message a step and never
//! waiting: a step sends the request for the next fragment when the bus is
//! quiet, and a later one reads the reply once the display has had time to
//! make it. The serve loop steps it on every hot-plug poll, so the frames
//! it serves are never held up by the read. Until the string has come back
//! whole and names a vcp list, no control is asked of the display at all;
//! one that does not answer, answers wrong three times running, or runs
//! past CAPS_MAX is refused until it is plugged again.

use super::caps::Caps;
use super::error::DdcError;

/// The longest string read.
pub const CAPS_MAX: usize = 1024;
/// How long the display has to make a reply before it is read.
pub const CAPS_REPLY_US: u64 = 50_000;
pub(super) const TRIES: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Fetch {
    Ready,
    Asked(u64),
    Got(Caps),
    Refused,
}

#[derive(Clone, Copy)]
pub struct CapsFetch {
    pub(super) state: Fetch,
    pub(super) text: [u8; CAPS_MAX],
    pub(super) len: usize,
    pub(super) misses: u8,
}

impl CapsFetch {
    pub const fn new() -> CapsFetch {
        CapsFetch { state: Fetch::Ready, text: [0; CAPS_MAX], len: 0, misses: 0 }
    }

    /// A read already over: `caps` taken some other way (the proofs hold a
    /// transaction to a display whose string they give).
    pub const fn known(caps: Caps) -> CapsFetch {
        CapsFetch { state: Fetch::Got(caps), text: [0; CAPS_MAX], len: 0, misses: 0 }
    }

    /// What the display said it has, once it has said it.
    pub fn caps(&self) -> Result<&Caps, DdcError> {
        match &self.state {
            Fetch::Got(c) => Ok(c),
            _ => Err(DdcError::NoCapabilities),
        }
    }

    /// Whether the read is over, either way.
    pub fn settled(&self) -> bool {
        matches!(self.state, Fetch::Got(_) | Fetch::Refused)
    }
}
