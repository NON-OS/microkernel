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

//! Feeding the client's bytes in. Frames are cut out one at a time, each
//! judged against the stage the server is in when its length arrives. The
//! first refusal ends the connection; once keys exist, the client is told
//! the one reason code that refusal allows, and otherwise nothing.

use super::Server;
use crate::{Attestation, HostSigner, Rand, Roles, Why};
use nsh_wire::{split, Expect};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// Take bytes from the client at `now_ms`. Events go to `events`,
    /// bytes to send to `take_output`. Bytes after the connection is over
    /// are dropped: the client may have sent them before it read the end.
    pub fn feed(&mut self, mut input: &[u8], now_ms: u64, rand: Rand<'_>) {
        while !input.is_empty() {
            let Some(want) = self.stage.receives() else { return };
            let step = match self.framer.fill(&mut input, want) {
                Ok(None) => return,
                Ok(Some((len, bytes))) => self.frame(want, len, &bytes, now_ms, rand),
                Err(e) => Err(Why::Wire(e)),
            };
            if let Err(why) = step {
                self.refuse(why, now_ms, rand);
            }
        }
    }

    fn frame(
        &mut self,
        want: Expect,
        len: [u8; 4],
        bytes: &[u8],
        now_ms: u64,
        rand: Rand<'_>,
    ) -> Result<(), Why> {
        if !want.sealed() {
            return self.client_hello(bytes, rand);
        }
        let mut inner = self.open(len, bytes)?;
        let done = match split(&inner, want) {
            Ok((kind, body)) => self.sealed(kind, body, now_ms, rand),
            Err(e) => Err(Why::Wire(e)),
        };
        nonos_hash::wipe(&mut inner);
        done
    }
}
