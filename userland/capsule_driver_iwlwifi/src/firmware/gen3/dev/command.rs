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

//! A bounded wait for a packet, and a host command run to its reply.

use super::super::packet::Packet;
use super::super::region::{Clock, Region};
use super::kinds::{Flow, WaitError, COMMAND_MS, TX_RESPONSE};
use super::state::Dev;
use crate::regs::Mmio;

impl<'a, M: Mmio, R: Region + ?Sized> Dev<'a, M, R> {
    /// Wait up to `ms` for `handle` to say `Done`.
    pub fn wait<C: Clock>(
        &mut self,
        c: &mut C,
        ms: u32,
        handle: &mut dyn FnMut(&Packet<'_>) -> Flow,
    ) -> Result<(), WaitError> {
        let mut outcome = None;
        c.poll_for(ms, &mut || {
            if self.drain(handle) == Flow::Done {
                outcome = Some(Ok(()));
            } else if self.error_cause() {
                outcome = Some(Err(WaitError::FirmwareError));
            }
            outcome.is_some()
        });
        outcome.unwrap_or(Err(WaitError::TimedOut))
    }

    /// Send a command and wait for its reply; `reply` gets the reply payload.
    /// Notifications arriving meanwhile go to `other`. A command left
    /// unanswered stays in flight, and the next is refused (`Busy`) until
    /// its reply drains.
    pub fn command<C: Clock>(
        &mut self,
        c: &mut C,
        (group, cmd): (u8, u8),
        payload: &[u8],
        reply: &mut dyn FnMut(&[u8]),
        other: &mut dyn FnMut(&Packet<'_>),
    ) -> Result<(), WaitError> {
        if self.cmdq.busy() {
            return Err(WaitError::Busy);
        }
        let seq =
            self.cmdq.send(self.m, self.ctrl, group, cmd, 0, payload).ok_or(WaitError::TooLarge)?;
        self.cmdq.mark(seq);
        self.wait(c, COMMAND_MS, &mut |p| {
            if p.is_reply() && p.sequence == seq && !(p.group == 0 && p.cmd == TX_RESPONSE) {
                reply(p.payload);
                Flow::Done
            } else {
                other(p);
                Flow::Continue
            }
        })
    }
}
