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

//! Taking received packets in order, each whole one to the caller's handler.

use super::super::packet::{parse as parse_packet, Packet};
use super::super::region::Region;
use super::super::rxq::Taken;
use super::kinds::{Flow, PASS, TX_RESPONSE};
use super::state::Dev;
use crate::regs::Mmio;

impl<'a, M: Mmio, R: Region + ?Sized> Dev<'a, M, R> {
    /// Take up to a pass of received packets, giving each whole one to
    /// `handle`, then restock the device. `Done` if `handle` said so.
    pub fn drain(&mut self, handle: &mut dyn FnMut(&Packet<'_>) -> Flow) -> Flow {
        let mut flow = Flow::Continue;
        for _ in 0..PASS {
            match self.rxq.take(self.ctrl, self.rbs, &mut self.buf) {
                None => break,
                Some(Taken::BadId) => self.bad_ids = self.bad_ids.saturating_add(1),
                Some(Taken::Fragment) => {}
                Some(Taken::Buffer) => {
                    if let Some(p) = parse_packet(&self.buf) {
                        // A reply, however late, retires the command it answers.
                        if p.is_reply() && !(p.group == 0 && p.cmd == TX_RESPONSE) {
                            self.cmdq.retire(p.sequence);
                        }
                        if handle(&p) == Flow::Done {
                            flow = Flow::Done;
                            break;
                        }
                    }
                }
            }
        }
        self.rxq.kick(self.m);
        flow
    }
}
