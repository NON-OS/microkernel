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

//! One trap answered for whichever of the family's guests made it.

use core::mem;

use nonos_libc::{mk_foreign_reply, ForeignFrame, FOREIGN_NR_DIED, FOREIGN_NR_INTERRUPTED};

use super::answer::Answer;
use super::family::Family;
use super::pid_map::frame_in;
use super::pid_out::value_out;
use super::route_life::answer;

impl Family {
    pub fn answer(&mut self, frame: &ForeignFrame) {
        if frame.nr == FOREIGN_NR_DIED {
            self.thread_died(frame.pid, frame.arg0 as i32);
            return;
        }
        if frame.nr == FOREIGN_NR_INTERRUPTED {
            self.interrupted(frame.pid);
            return;
        }
        let Some(i) = self.guests.iter().position(|g| g.owns(frame.pid)) else {
            return;
        };
        /*
         * A call outside the declared set skips its pids' translation, which
         * would answer a pid outside the family ESRCH or ECHILD: the kernel
         * counts that as a success, in a bucket the set leaves clear
         *. Untranslated, it reaches route_life and is EPERM.
         */
        let frame = match crate::linux::app::undeclared(&self.guests[i], frame.nr) {
            true => *frame,
            false => match frame_in(&self.ns, frame) {
                Some(frame) => frame,
                None => return,
            },
        };
        crate::linux::boxed::call_begins();
        self.lend(i);
        self.lend_view(i, &frame);
        let got = answer(&mut self.guests[i], &frame);
        self.take_view();
        self.take_back(i);
        let g = &mut self.guests[i];
        let born = mem::take(&mut g.forked);
        if let Answer::Reply(value) = got {
            // A caught signal for this thread is delivered in place of the reply.
            super::deliver_pipe::broken_pipe(g, frame.pid, frame.nr, frame.args(), value);
            let out = value_out(&mut self.ns, frame.nr, value);
            super::clone_tid::write(g, &frame, out);
            if !super::deliver::maybe_deliver(g, frame.pid, out) {
                let _ = mk_foreign_reply(frame.pid, out);
            }
        }
        self.guests.extend(born);
    }
}
