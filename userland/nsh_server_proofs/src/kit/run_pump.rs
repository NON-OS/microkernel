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

//! The run's exchange: whatever one side hands back fed to the other until
//! neither has anything left to say, with the host's answers to an admitted
//! login or escalation.

use super::run::Run;
use nsh_server::Event;

impl Run {
    /// Exchange bytes until neither side has anything to send.
    pub fn pump(&mut self) {
        loop {
            let c_out = self.client.take_output();
            self.c2s.extend_from_slice(&c_out);
            let s_rng = &mut self.s_rng;
            self.server.feed(&c_out, self.now, &mut |b| s_rng.fill(b));
            let moved = self.server_events() | !c_out.is_empty();
            let s_out = self.server.take_output();
            self.s2c.extend_from_slice(&s_out);
            if self.c_err.is_none() && !s_out.is_empty() {
                let c_rng = &mut self.c_rng;
                if let Err(e) = self.client.feed(&s_out, &mut |b| c_rng.fill(b)) {
                    self.c_err = Some(e);
                }
            }
            self.c_events.extend(self.client.events());
            if !moved && s_out.is_empty() {
                return;
            }
        }
    }

    /// Keep the server's events, answering an admitted login or
    /// escalation when `answer` is set. Whether there were any.
    pub(super) fn server_events(&mut self) -> bool {
        let events = self.server.events();
        let any = !events.is_empty();
        for e in events {
            let s_rng = &mut self.s_rng;
            let rand = &mut |b: &mut [u8]| s_rng.fill(b);
            match &e {
                Event::Admit(a) if self.answer => {
                    self.server.begin(a.caps, self.now, rand).expect("begin");
                }
                Event::Escalate { .. } if self.answer => {
                    self.server.escalation_begun(self.now, rand).expect("escalation begun");
                }
                _ => {}
            }
            self.s_events.push(e);
        }
        any
    }
}
