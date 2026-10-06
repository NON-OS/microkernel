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

//! The firmware as the join and the link use it: host commands run to their
//! reply, frames sent on the access point's queues, and bounded waits for
//! received frames and session events. Everything the firmware posts
//! meanwhile goes through the inbox, so a frame that arrives while a command
//! waits is read afterwards rather than lost.

use alloc::vec::Vec;

use super::super::dev::{Dev, Flow, WaitError};
use super::super::region::{Clock, Region};
use super::super::station::session::Session;
use super::inbox::{Inbox, Queues};
use crate::regs::Mmio;

/// Which of the access point's queues a frame goes on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Queue {
    Mgmt,
    Data,
}

/// A command the firmware did not complete: its group and opcode, and why.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CommandFailed {
    pub group: u8,
    pub cmd: u8,
    pub why: WaitError,
}

pub struct Fw<'a, 'd, M: Mmio, R: Region + ?Sized, C: Clock> {
    pub dev: &'a mut Dev<'d, M, R>,
    /// The transmit region both queues live in.
    pub tx: &'a R,
    pub clock: &'a mut C,
    pub q: &'a mut Queues,
    pub inbox: &'a mut Inbox,
}

impl<M: Mmio, R: Region + ?Sized, C: Clock> Fw<'_, '_, M, R, C> {
    /// Run a command to its reply and return the reply's payload.
    pub fn command(&mut self, group: u8, cmd: u8, payload: &[u8]) -> Result<Vec<u8>, CommandFailed> {
        let mut out = Vec::new();
        let (q, inbox) = (&mut *self.q, &mut *self.inbox);
        self.dev
            .command(self.clock, (group, cmd), payload, &mut |r| out = r.to_vec(), &mut |p| {
                inbox.route(p, q);
            })
            .map_err(|why| CommandFailed { group, cmd, why })?;
        Ok(out)
    }

    /// Queue a frame at `rate_n_flags`; `false` when the queue would not
    /// take it.
    pub fn send(&mut self, which: Queue, frame: &[u8], rate_n_flags: u32) -> bool {
        let q = match which {
            Queue::Mgmt => &mut self.q.mgmt,
            Queue::Data => &mut self.q.data,
        };
        q.send(self.dev.m, self.tx, frame, rate_n_flags)
    }

    /// Take what the firmware has posted, without waiting.
    pub fn pump(&mut self) {
        let (q, inbox) = (&mut *self.q, &mut *self.inbox);
        self.dev.drain(&mut |p| {
            inbox.route(p, q);
            Flow::Continue
        });
    }

    /// Wait up to `ms` for a received frame or a session event. `Ok(false)`
    /// when none came; an error when the firmware raised its error cause.
    pub fn wait(&mut self, ms: u32) -> Result<bool, WaitError> {
        if !self.inbox.frames.is_empty() || self.inbox.session.is_some() {
            return Ok(true);
        }
        let (q, inbox) = (&mut *self.q, &mut *self.inbox);
        match self.dev.wait(self.clock, ms, &mut |p| if inbox.route(p, q) { Flow::Done } else { Flow::Continue }) {
            Ok(()) => Ok(true),
            Err(WaitError::TimedOut) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Wait up to `ms` for the firmware to answer every queued management
    /// frame (a leaving station's deauthentication before its queues go).
    pub fn settle(&mut self, ms: u32) {
        if self.q.mgmt.in_flight() == 0 {
            return;
        }
        let (q, inbox) = (&mut *self.q, &mut *self.inbox);
        let _ = self.dev.wait(self.clock, ms, &mut |p| {
            inbox.route(p, q);
            if q.mgmt.in_flight() == 0 {
                Flow::Done
            } else {
                Flow::Continue
            }
        });
    }

    /// Wait up to `ms` for a session event and take it.
    pub fn wait_session(&mut self, ms: u32) -> Result<Option<Session>, WaitError> {
        if self.inbox.session.is_none() {
            let (q, inbox) = (&mut *self.q, &mut *self.inbox);
            let got = self.dev.wait(self.clock, ms, &mut |p| {
                inbox.route(p, q);
                if inbox.session.is_some() {
                    Flow::Done
                } else {
                    Flow::Continue
                }
            });
            match got {
                Ok(()) | Err(WaitError::TimedOut) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(self.inbox.session.take())
    }
}
