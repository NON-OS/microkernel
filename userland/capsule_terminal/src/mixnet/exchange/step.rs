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

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::types::{After, Exchange, Phase, Poll};
use super::wait::Wait;

#[path = "step_new.rs"]
mod step_new;
#[path = "step_read.rs"]
mod step_read;

impl Exchange {
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
}
