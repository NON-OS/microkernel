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
 * Opening a tunnel in slices: the same reset, greeting and CONNECT as
 * `Tunnel::open`, under the same OPEN_MS, but each step asks the proxy at
 * most once and for at most a slice, and the second between tries on a
 * network that has not connected yet is a time to come back at rather
 * than a sleep. The caller steps it until it says Done or Failed.
 */

use alloc::vec::Vec;

use crate::bounds::{NOT_YET_GAP_MS, OPEN_MS};
use crate::carrier::Carrier;
use crate::refusal::{Proxy, BAD_HOST, REP_NOT_YET};
use crate::slice::Slice;
use crate::socks::{connect_reply, connect_request, method_reply, Parsed, GREETING, REP_OK};
use crate::tunnel::Tunnel;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stage {
    Reset,
    Greet,
    Method,
    Connect,
    Reply,
    /* A network that has not connected yet is asked again from here on. */
    Again(i64),
}

pub struct Opening<C: Carrier> {
    tunnel: Tunnel<C>,
    ask: Vec<u8>,
    until: i64,
    stage: Stage,
}

impl<C: Carrier> Opening<C> {
    /*
     * Nothing is sent until the first step. The resets this stream sends,
     * on each try and as it is dropped, wait at most `slice_ms`: none of
     * them needs the proxy's answer, and the proxy takes what follows a
     * reset after it whether or not anyone waited.
     */
    pub fn begin(
        carrier: C,
        proxy: Proxy,
        host: &str,
        port: u16,
        slice_ms: u64,
    ) -> Result<Opening<C>, &'static str> {
        let ask = connect_request(host, port).ok_or(BAD_HOST)?;
        let mut tunnel = Tunnel::unopened(carrier, proxy);
        tunnel.reset_wait_ms = slice_ms;
        let until = tunnel.carrier.now_ms().saturating_add(OPEN_MS);
        Ok(Opening { tunnel, ask, until, stage: Stage::Reset })
    }

    /* One step, waiting at most `slice_ms` on the proxy. */
    pub fn step(&mut self, slice_ms: u64) -> Slice {
        let proxy = self.tunnel.proxy;
        let now = self.tunnel.carrier.now_ms();
        match self.stage {
            Stage::Again(at) if now < at => Slice::Waiting,
            Stage::Reset | Stage::Again(_) => {
                self.tunnel.restart();
                self.stage = Stage::Greet;
                Slice::Waiting
            }
            Stage::Greet => self.send(&GREETING, Stage::Method, slice_ms),
            Stage::Method => match self.reply(method_reply, slice_ms) {
                Ok(Some(true)) => {
                    self.stage = Stage::Connect;
                    Slice::Waiting
                }
                Ok(Some(false)) => Slice::Failed(proxy.full()),
                Ok(None) => Slice::Waiting,
                Err(why) => Slice::Failed(why),
            },
            Stage::Connect => {
                let ask = core::mem::take(&mut self.ask);
                let out = self.send(&ask, Stage::Reply, slice_ms);
                self.ask = ask;
                out
            }
            Stage::Reply => match self.reply(connect_reply, slice_ms) {
                Ok(Some(REP_OK)) => Slice::Done,
                Ok(Some(REP_NOT_YET)) if now < self.until => {
                    self.stage = Stage::Again(now.saturating_add(NOT_YET_GAP_MS as i64));
                    Slice::Waiting
                }
                Ok(Some(rep)) => Slice::Failed(proxy.refused(rep)),
                Ok(None) => Slice::Waiting,
                Err(why) => Slice::Failed(why),
            },
        }
    }

    /* The open tunnel, once a step said Done. */
    pub fn into_tunnel(self) -> Tunnel<C> {
        self.tunnel
    }

    fn send(&mut self, body: &[u8], next: Stage, slice_ms: u64) -> Slice {
        match self.tunnel.exchange_slice(body, slice_ms) {
            Slice::Done => {
                self.stage = next;
                Slice::Waiting
            }
            other => other,
        }
    }

    /*
     * The reply `parse` reads off what the proxy has answered, or None when
     * it is not whole yet, after asking the proxy once for more. A reply is
     * read before a close is believed, as `Tunnel::open` reads it.
     */
    fn reply<T>(
        &mut self,
        parse: fn(&[u8]) -> Parsed<T>,
        slice_ms: u64,
    ) -> Result<Option<T>, &'static str> {
        let t = &mut self.tunnel;
        if let Some(whole) = take_reply(t, parse)? {
            return Ok(Some(whole));
        }
        if t.closed {
            return Err(t.proxy.ended());
        }
        if t.carrier.now_ms() >= self.until {
            return Err(t.proxy.slow());
        }
        match t.exchange_slice(&[], slice_ms) {
            Slice::Failed(why) => Err(why),
            Slice::Done => take_reply(t, parse),
            Slice::Waiting => Ok(None),
        }
    }
}

/* The reply at the front of what is held, taken off it when whole. */
fn take_reply<C: Carrier, T>(
    t: &mut Tunnel<C>,
    parse: fn(&[u8]) -> Parsed<T>,
) -> Result<Option<T>, &'static str> {
    match parse(&t.pending) {
        Parsed::Done(reply, used) => {
            let used = used.min(t.pending.len());
            t.pending.drain(..used);
            Ok(Some(reply))
        }
        Parsed::Bad => Err(t.proxy.garbled()),
        Parsed::Need => Ok(None),
    }
}
