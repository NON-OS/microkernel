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

//! How many connections the pool holds, how many it may, and making room.

use super::slots::Pool;
use crate::browser::fetch::wire::Wire;
use crate::browser::url::Url;

/* The usual courtesy to one host, and a share of the system's sockets. */
pub const PER_HOST: usize = 6;
pub const IN_ALL: usize = 8;

/* Through a network's proxy: streams the proxy gives one program, less the
 * navigation's, a kept image connection's and one let go and not yet reset
 * (net::mixnet::streams). Fewer side by side also leaves each a fair share
 * of a mixnet that carries little. */
pub const PROXIED_PER_HOST: usize = 4;
pub const PROXIED_IN_ALL: usize = 4;

impl Pool {
    /// Fetches running to `url`'s host and port.
    pub fn live_to(&self, url: &Url) -> usize {
        self.live.iter().filter(|f| f.url.host == url.host && f.url.port == url.port).count()
    }

    /// Connections open to `url`'s host and port, running or kept.
    pub fn open_to(&self, url: &Url) -> usize {
        let kept = self.idle.iter().filter(|i| i.host == url.host && i.port == url.port);
        self.live_to(url) + kept.count()
    }

    /// The limits in force for a connection that is `proxied`.
    pub fn limits(proxied: bool) -> (usize, usize) {
        if proxied {
            (PROXIED_PER_HOST, PROXIED_IN_ALL)
        } else {
            (PER_HOST, IN_ALL)
        }
    }

    /// Room for a new connection to `url`, closing kept connections to other
    /// hosts if that is what it takes. False while `url`'s host is at its
    /// limit or every connection is running.
    pub fn make_room<W: Wire>(&mut self, w: &mut W, url: &Url, proxied: bool) -> bool {
        let (per, all) = Pool::limits(proxied);
        if self.open_to(url) >= per {
            return false;
        }
        while self.live.len() + self.idle.len() >= all {
            if self.idle.is_empty() {
                return false;
            }
            let old = self.idle.remove(0);
            w.close(old.handle);
        }
        true
    }
}
