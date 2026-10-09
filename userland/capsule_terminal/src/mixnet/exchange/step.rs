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

//! Making and stepping an exchange. Each step does a bounded amount: one
//! slice of a tunnel's opening, a few milliseconds of carrying or reading,
//! one look at the handshake. A job calls it once a tick, so the window
//! paints and takes Ctrl+C between steps, and dropping the exchange closes
//! whatever it had open (the socket's and the tunnel's own drops).

use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::mk_time_millis;
use nonos_route_link::Route;

use super::types::{After, Exchange, Phase, Poll, Stage};
use super::wait::{settle_handshake, settle_response, Settle, Wait};

impl Exchange {
    /// `request` is the whole HTTP request; `limit` bounds what the far end
    /// can make this allocate.
    pub fn new(
        route: Route,
        host: &str,
        port: u16,
        secure: bool,
        rtc: u64,
        request: Vec<u8>,
        limit: usize,
    ) -> Self {
        Exchange {
            host: String::from(host),
            port,
            route,
            anonymous: route.is_anonymous(),
            secure,
            rtc,
            request,
            limit,
            phase: Phase::Start,
            stream: None,
            out: Vec::new(),
            sent: 0,
            raw: Vec::new(),
            tls: None,
            wait: Wait::new(mk_time_millis(), route.is_anonymous()),
        }
    }

    pub fn stage(&self) -> Stage {
        match self.phase {
            Phase::Start | Phase::Opening(_) => Stage::Connecting,
            Phase::Send(After::Flight) | Phase::Flight => Stage::Handshake,
            Phase::Send(After::Response) => Stage::Sending,
            Phase::Response | Phase::Done => Stage::Receiving,
        }
    }

    /// Bytes of the flight or response that have arrived so far.
    pub fn received(&self) -> usize {
        self.raw.len()
    }

    pub fn step(&mut self) -> Poll {
        let result = match self.phase {
            Phase::Start => self.start(),
            Phase::Opening(_) => self.opening(),
            Phase::Send(after) => self.send(after),
            Phase::Flight => self.flight(),
            Phase::Response => self.response(),
            Phase::Done => Err("the exchange is already over"),
        };
        match result {
            Ok(Some(body)) => self.end(Ok(body)),
            Ok(None) => Poll::Pending,
            Err(why) => self.end(Err(why)),
        }
    }

    fn end(&mut self, result: Result<Vec<u8>, &'static str>) -> Poll {
        self.phase = Phase::Done;
        self.stream = None;
        self.tls = None;
        self.raw = Vec::new();
        Poll::Ready(result)
    }

    /// The connection is up: TLS says hello, plain HTTP sends its request.
    pub(super) fn connected(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        self.wait = Wait::new(mk_time_millis(), self.anonymous);
        if self.secure {
            return self.hello();
        }
        self.out = core::mem::take(&mut self.request);
        self.sent = 0;
        self.phase = Phase::Send(After::Response);
        Ok(None)
    }

    fn flight(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let got = self.take()?;
        let now = mk_time_millis();
        if got > 0 {
            self.wait.heard(now);
            return self.advance();
        }
        let ended = self.stream.as_ref().is_some_and(|s| s.ended());
        match settle_handshake(self.wait.check(now), ended, self.anonymous) {
            Settle::More | Settle::Finish => Ok(None),
            Settle::Fail(why) => Err(why),
        }
    }

    fn response(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let got = self.take()?;
        let now = mk_time_millis();
        if got > 0 {
            self.wait.heard(now);
            self.open_records();
        }
        let ended = self.stream.as_ref().is_some_and(|s| s.ended());
        let heard = if got > 0 { super::wait::Heard::Waiting } else { self.wait.check(now) };
        match settle_response(heard, ended, self.anonymous) {
            Settle::More => Ok(None),
            Settle::Finish => Ok(Some(self.body())),
            Settle::Fail(why) => Err(why),
        }
    }

    /// The response as the caller reads it: decrypted for TLS, as it came
    /// for plain HTTP.
    fn body(&mut self) -> Vec<u8> {
        match self.tls.take() {
            Some(tls) => tls.reader.into_plaintext(),
            None => core::mem::take(&mut self.raw),
        }
    }
}
