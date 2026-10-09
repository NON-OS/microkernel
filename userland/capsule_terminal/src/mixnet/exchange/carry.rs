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

//! Moving bytes, bounded a tick: writes and reads go on until a few
//! milliseconds have passed or the far end has nothing more to send, so a
//! large pack arrives at the speed of the network and a frame still comes
//! round every tick.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::open::SLICE_MS;
use super::types::{After, Exchange, Phase};
use super::wait::{Heard, TOTAL_ANON, TOTAL_DIRECT};

/// Time one step may spend carrying or reading.
const STEP_MS: i64 = 12;
/// Read size; a tunnel hands back what it holds, a socket what it has.
const CHUNK: usize = 16 * 1024;

const BROKE: &str = "the connection failed";
const TOO_LARGE: &str = "the response is larger than allowed";
/// What a server's handshake flight may make this allocate before anything
/// in it has been verified.
const MAX_FLIGHT: usize = 128 * 1024;

impl Exchange {
    pub(super) fn send(&mut self, after: After) -> Result<Option<Vec<u8>>, &'static str> {
        let Some(stream) = self.stream.as_mut() else {
            return Err(BROKE);
        };
        let until = mk_time_millis().saturating_add(STEP_MS);
        while self.sent < self.out.len() {
            let n = stream.write_slice(&self.out[self.sent..], SLICE_MS)?;
            self.sent += n;
            if n == 0 || mk_time_millis() >= until {
                break;
            }
        }
        if self.sent < self.out.len() {
            return match self.wait.check(mk_time_millis()) {
                Heard::Total if self.anonymous => Err(TOTAL_ANON),
                Heard::Total => Err(TOTAL_DIRECT),
                Heard::Waiting | Heard::Quiet => Ok(None),
            };
        }
        self.out = Vec::new();
        self.sent = 0;
        self.wait.heard(mk_time_millis());
        self.phase = match after {
            After::Flight => Phase::Flight,
            After::Response => Phase::Response,
        };
        Ok(None)
    }

    /// Read what has arrived onto `raw`; how many bytes came this step.
    pub(super) fn take(&mut self) -> Result<usize, &'static str> {
        let Some(stream) = self.stream.as_mut() else {
            return Err(BROKE);
        };
        // The handshake flight has its own bound, as the blocking session
        // had: a certificate chain is not held to a small response's limit.
        let limit = match self.phase {
            Phase::Flight => self.limit.max(MAX_FLIGHT),
            _ => self.limit,
        };
        let until = mk_time_millis().saturating_add(STEP_MS);
        let mut chunk = vec![0u8; CHUNK];
        let mut got = 0;
        loop {
            let n = stream.read_slice(&mut chunk, SLICE_MS)?;
            if n == 0 {
                break;
            }
            if self.raw.len().saturating_add(n) > limit {
                return Err(TOO_LARGE);
            }
            self.raw.extend_from_slice(&chunk[..n]);
            got += n;
            if mk_time_millis() >= until {
                break;
            }
        }
        Ok(got)
    }
}
