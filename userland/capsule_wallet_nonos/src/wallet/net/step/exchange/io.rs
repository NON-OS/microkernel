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

//! The exchange's reads a slice at a time, their bounds, and when the
//! server's flight can be judged.

use nonos_tls::flight::ClientFlight;
use nonos_tls::{HandshakeState, Progress, Start};

use super::super::super::bounds::Bounds;
use super::super::super::read_tls_flight::routed;
use super::super::gather::Gathered;
use super::state::Exchange;
use super::types::UNLINKED;

impl Exchange {
    /// One slice read into the gather under way.
    pub(super) fn gather_slice(&mut self, done: impl FnMut(&[u8]) -> bool) -> Gathered {
        let (Some(link), Some(gather)) = (self.link.as_mut(), self.gather.as_mut()) else {
            return Gathered::Nothing;
        };
        let mut chunk = [0u8; 4096];
        match link.read_slice(&mut chunk) {
            Ok(read) => gather.take(&chunk[..read.n], read.ended, nonos_libc::mk_uptime_ms(), done),
            Err(()) => gather.broke(),
        }
    }

    pub(super) fn bounds(&self) -> Bounds {
        match &self.link {
            Some(link) => routed(link.patience_ms),
            None => UNLINKED,
        }
    }
}

/* Whether the flight so far can be judged: a whole server Finished came,
 * or the server alerted, or a record failed. Keyed from the ServerHello on
 * the first look that has one whole, and kept. */
pub(super) fn judged(
    client: &ClientFlight,
    handshake: &mut Option<HandshakeState>,
    bytes: &[u8],
) -> bool {
    if handshake.is_none() {
        match HandshakeState::begin(client, bytes) {
            Start::Waiting => return false,
            Start::Ready(state) => *handshake = Some(*state),
            Start::Alert(_) | Start::Retry | Start::Unusable => return true,
        }
    }
    handshake.as_mut().is_none_or(|state| state.advance(bytes) != Progress::Incomplete)
}
