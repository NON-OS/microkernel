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
 * A TCP stream carried by net.socks5 to an exit on the Nym mixnet, or by
 * net.anon to an exit on the Anyone network. The host name goes to the exit
 * unresolved, so this machine never looks it up and never reaches it.
 *
 * Both proxies key a conversation on the caller's pid, so a capsule holds
 * one stream through each at a time: opening one ends the last. Opening
 * starts with a reset, so nothing a crashed or abandoned stream left behind
 * is read as this one's, and dropping a stream resets it again, which ends
 * its tunnel at the proxy.
 */

use alloc::vec;
use alloc::vec::Vec;

use crate::answer::ANSWER_BUF;
use crate::bounds::{NOT_YET_GAP_MS, OPEN_MS, POLL_GAP_MS, RESET_WAIT_MS};
use crate::carrier::Carrier;
use crate::frame::{reset, FIRST_SEQ};
use crate::refusal::{Proxy, BAD_HOST, REP_NOT_YET};
use crate::socks::{connect_reply, connect_request, method_reply, Parsed, GREETING, REP_OK};

pub struct Tunnel<C: Carrier> {
    pub(crate) carrier: C,
    pub(crate) proxy: Proxy,
    /* The number of the next exchange. */
    pub(crate) seq: u32,
    /* What the proxy has answered that the reader has not taken. */
    pub(crate) pending: Vec<u8>,
    /* The proxy said the far end finished: nothing more will come. */
    pub(crate) closed: bool,
    /* Where an answer is read, sized once. */
    pub(crate) rx: Vec<u8>,
    /* The exchange a sliced caller asked and has no answer to yet. */
    pub(crate) unanswered: Option<crate::slice::Unanswered>,
    /* How long the reset that ends this stream waits for its answer. */
    pub(crate) reset_wait_ms: u64,
}

/* How one attempt at opening ended, when it did not open. */
enum Attempt {
    /* The network runs but has not connected; worth asking again. */
    NotYet,
    Refused(&'static str),
}

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
        let ask = connect_request(host, port).ok_or(BAD_HOST)?;
        let mut t = Tunnel::unopened(carrier, proxy);
        let until = t.carrier.now_ms().saturating_add(OPEN_MS);
        loop {
            t.restart();
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

    /* A conversation not begun yet: nothing has been sent to the proxy. */
    pub(crate) fn unopened(carrier: C, proxy: Proxy) -> Tunnel<C> {
        Tunnel {
            carrier,
            proxy,
            seq: FIRST_SEQ,
            pending: Vec::new(),
            closed: false,
            rx: vec![0; ANSWER_BUF],
            unanswered: None,
            reset_wait_ms: RESET_WAIT_MS,
        }
    }

    pub fn proxy(&self) -> Proxy {
        self.proxy
    }

    /* A conversation from the start: the proxy forgets the last one. */
    pub(crate) fn restart(&mut self) {
        self.forget();
        self.seq = FIRST_SEQ;
        self.pending.clear();
        self.closed = false;
        self.unanswered = None;
    }

    /*
     * Ask the proxy to forget this caller's conversation, ending its tunnel.
     * No answer is needed: a proxy with nothing to forget is the normal case,
     * and one that is not there fails the greeting that follows.
     */
    fn forget(&mut self) {
        let _ = self.carrier.call(&reset(), self.reset_wait_ms, &mut self.rx);
    }

    fn handshake(&mut self, ask: &[u8], until: i64) -> Result<(), Attempt> {
        self.exchange(&GREETING).map_err(Attempt::Refused)?;
        if !self.await_reply(until, method_reply)? {
            return Err(Attempt::Refused(self.proxy.full()));
        }
        self.exchange(ask).map_err(Attempt::Refused)?;
        match self.await_reply(until, connect_reply)? {
            REP_OK => Ok(()),
            REP_NOT_YET => Err(Attempt::NotYet),
            rep => Err(Attempt::Refused(self.proxy.refused(rep))),
        }
    }

    /*
     * The next reply `parse` reads off what the proxy has answered, asking
     * for more until it is whole. net.anon answers a CONNECT only once its
     * exit has connected, so this is where that wait is spent. A reply is
     * read before a close is believed: a refusal arrives as the reply and
     * the close together.
     */
    fn await_reply<T>(&mut self, until: i64, parse: fn(&[u8]) -> Parsed<T>) -> Result<T, Attempt> {
        loop {
            match parse(&self.pending) {
                Parsed::Done(reply, used) => {
                    let used = used.min(self.pending.len());
                    self.pending.drain(..used);
                    return Ok(reply);
                }
                Parsed::Bad => return Err(Attempt::Refused(self.proxy.garbled())),
                Parsed::Need => {}
            }
            if self.closed {
                return Err(Attempt::Refused(self.proxy.ended()));
            }
            if self.carrier.now_ms() >= until {
                return Err(Attempt::Refused(self.proxy.slow()));
            }
            let had = self.pending.len();
            self.exchange(&[]).map_err(Attempt::Refused)?;
            if self.pending.len() == had {
                self.carrier.pause(POLL_GAP_MS);
            }
        }
    }
}

impl<C: Carrier> Drop for Tunnel<C> {
    fn drop(&mut self) {
        self.forget();
    }
}
