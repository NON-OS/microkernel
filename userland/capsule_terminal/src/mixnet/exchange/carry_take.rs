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

//! Reading what the far end has sent, bounded by a tick and a size.

use alloc::vec;

use nonos_libc::mk_time_millis;

use super::super::open::SLICE_MS;
use super::super::types::{Exchange, Phase};
use super::{BROKE, CHUNK, MAX_FLIGHT, STEP_MS, TOO_LARGE};

impl Exchange {
    /// Read what has arrived onto `raw`; how many bytes came this step.
    pub(in crate::mixnet::exchange) fn take(&mut self) -> Result<usize, &'static str> {
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
