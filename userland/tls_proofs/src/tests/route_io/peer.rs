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

//! The fake server behind a route: silent for a while, as a far end over
//! the Nym mixnet can be, then one byte per 10 ms on the shim's clock.

use nonos_libc::skip_ms;
use nonos_route_link::{Peer, Proxy, RouteStream, Tunnel};

use super::super::super::fake_server::Wire;
use crate::session::Io;

/// The first byte this far off, in ms: 45 s, inside Nym's 60 s patience.
pub(super) const FIRST_BYTE_MS: u64 = 45_000;

struct Played {
    wire: Wire,
    silent_ms: u64,
}

impl Peer for Played {
    fn write_all(&mut self, data: &[u8]) -> Result<(), &'static str> {
        self.wire.write_all(data).map_err(|_| "write")
    }

    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        if self.silent_ms > 0 {
            let waited = self.silent_ms.min(wait_ms);
            self.silent_ms -= waited;
            skip_ms(waited as i64);
            return Ok(0);
        }
        let n = self.wire.read(&mut into[..1]).map_err(|_| "read")?;
        skip_ms(10 * n as i64);
        Ok(n)
    }
}

fn played() -> Box<dyn Peer> {
    Box::new(Played { wire: Wire::new(), silent_ms: FIRST_BYTE_MS })
}

/// Through the Nym mixnet, whose patience is its Proxy's.
pub(super) fn over_nym() -> RouteStream {
    RouteStream::Proxied(Tunnel::new(Proxy::Nym, played()))
}

/// A direct socket: patience 0, so HANDSHAKE_MS and the quiet rule hold.
pub(super) fn direct() -> RouteStream {
    RouteStream::Direct(played())
}
