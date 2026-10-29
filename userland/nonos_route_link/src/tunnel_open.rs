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

/*
 * Opening a tunnel in one call: a reset that reached the proxy, then the
 * greeting and the CONNECT, asked again while a network that runs has not
 * connected yet, all under OPEN_MS. An install's download opens the same
 * way with the install mark in its greeting (install.rs).
 */

use crate::bounds::{ASK_GAP_MS, NOT_YET_GAP_MS, OPEN_MS};
use crate::carrier::Carrier;
use crate::refusal::{Proxy, BAD_HOST, REP_NOT_YET};
use crate::socks::connect_request;
use crate::tunnel::{Attempt, Tunnel};

impl<C: Carrier> Tunnel<C> {
    /*
     * A stream to `host` on `port` through `proxy`, or the sentence that
     * says why there is none. Bounded by OPEN_MS whatever the proxy does.
     */
    pub fn open(
        carrier: C,
        proxy: Proxy,
        host: &str,
        port: u16,
    ) -> Result<Tunnel<C>, &'static str> {
        Tunnel::open_as(carrier, proxy, host, port, false)
    }

    /* The same, marked as an install's download when `install` is set. */
    pub fn open_as(
        carrier: C,
        proxy: Proxy,
        host: &str,
        port: u16,
        install: bool,
    ) -> Result<Tunnel<C>, &'static str> {
        let ask = connect_request(host, port).ok_or(BAD_HOST)?;
        let mut t = Tunnel::unopened(carrier, proxy);
        t.install = install;
        let until = t.carrier.now_ms().saturating_add(OPEN_MS);
        loop {
            if !t.restart() {
                if t.carrier.now_ms() >= until {
                    return Err(proxy.silent());
                }
                t.carrier.pause(ASK_GAP_MS);
                continue;
            }
            match t.handshake(&ask, until) {
                Ok(()) => return Ok(t),
                Err(Attempt::NotYet) if t.carrier.now_ms() < until => {
                    t.carrier.pause(NOT_YET_GAP_MS)
                }
                Err(Attempt::NotYet) => return Err(proxy.refused(REP_NOT_YET)),
                Err(Attempt::Refused(why)) => return Err(why),
            }
        }
    }
}
