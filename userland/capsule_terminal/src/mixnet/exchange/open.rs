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

//! Opening the connection on the network the person chose. A tunnel
//! through the Nym mixnet or the Anyone network is opened a slice at a
//! time; it is the slow one, many seconds of SOCKS round trips through
//! relays. A route that is down is refused with its reason, and nothing
//! tries another way.

use alloc::vec::Vec;

use nonos_route_link::{Route, RouteOpening, RouteStream, Slice};

use super::types::{Exchange, Phase};

/// The longest one step waits on a proxy.
pub(super) const SLICE_MS: u64 = 10;

impl Exchange {
    pub(super) fn start(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        if let Route::Down(why) = self.route {
            return Err(why);
        }
        /*
         * An .anyone service goes through net.anon whatever the default, so
         * the opening is asked first even on Direct; it declines a host that
         * is to be reached directly.
         */
        match RouteOpening::begin(self.route, &self.host, self.port, SLICE_MS) {
            Ok(opening) => {
                self.anonymous = true;
                self.phase = Phase::Opening(opening);
                Ok(None)
            }
            Err(why) if self.route != Route::Direct => Err(why),
            /*
             * Direct is opened in one call: net.sockets resolves and connects
             * together, bounded by its own nine seconds. It is the route a
             * person picks for a network they trust, where that is a round
             * trip or two.
             */
            Err(_) => {
                let stream = RouteStream::connect(self.route, &self.host, self.port)?;
                self.stream = Some(stream);
                self.connected()
            }
        }
    }

    pub(super) fn opening(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let Phase::Opening(opening) = &mut self.phase else {
            return Ok(None);
        };
        match opening.step(SLICE_MS) {
            Slice::Waiting => Ok(None),
            Slice::Failed(why) => Err(why),
            Slice::Done => {
                let Phase::Opening(opening) = core::mem::replace(&mut self.phase, Phase::Start)
                else {
                    return Ok(None);
                };
                self.stream = Some(opening.into_stream());
                self.connected()
            }
        }
    }
}
