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

//! Starting a fetch in the pool.

use super::slots::Pool;
use crate::browser::fetch::open::open;
use crate::browser::fetch::reuse::reuse;
use crate::browser::fetch::types::Fetch;
use crate::browser::fetch::wire::Wire;
use crate::browser::net::mixnet::Way;
use crate::browser::url::Url;

/// The proxy a connection is made through, if one is set.
pub type Via<'a> = Option<(&'a str, u16)>;

/// A started fetch, or why none started.
pub type Started<'a> = Result<&'a mut Fetch, Refused>;

/// Why a fetch did not start.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /* No room now: it waits its turn. */
    Busy,
    Failed(&'static str),
}

impl Pool {
    /// Start a fetch of `url`: on a kept connection to its host when there
    /// is one, else on a new connection if the limits allow. The caller
    /// marks what the returned fetch is for.
    pub fn start<W: Wire>(&mut self, w: &mut W, url: Url, via: Via) -> Started<'_> {
        self.begin(w, url, via, true)
    }

    /// The same, never on a kept connection: for a request whose kept
    /// connection turned out to have been closed by the server.
    pub fn start_fresh<W: Wire>(&mut self, w: &mut W, url: Url, via: Via) -> Started<'_> {
        self.begin(w, url, via, false)
    }

    fn begin<W: Wire>(&mut self, w: &mut W, url: Url, via: Via, kept: bool) -> Started<'_> {
        let way = w.way(&url.host);
        if let Way::Refused(why) = way {
            return Err(Refused::Failed(why));
        }
        let (per, _) = Pool::limits(way.proxied());
        if self.live_to(&url) >= per {
            return Err(Refused::Busy);
        }
        /* A SOCKS proxy the reader set is never stacked on a network's. */
        let reusable = kept && (via.is_none() || way.proxied());
        let idle = if reusable { self.take_idle(w, &url) } else { None };
        let fetch = match idle.and_then(|idle| reuse(w, idle, url.clone())) {
            Some(f) => f,
            None if !self.make_room(w, &url, way.proxied()) => return Err(Refused::Busy),
            /* Every stream at the proxy is held: wait for one, as for a
             * place in the pool. It was refused as the proxy not running. */
            None if !w.room(way) => return Err(Refused::Busy),
            None => open(w, url, via).map_err(Refused::Failed)?,
        };
        self.live.push(fetch);
        self.live.last_mut().ok_or(Refused::Failed("socket failed"))
    }
}
