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

//! What `nsh` tells the user, on standard error, as the session goes: each
//! step, and the record (`report_record`).

use super::session::Nsh;
use crate::{Event, Policy};
use alloc::format;
use alloc::string::String;
use std::io::Write;

pub struct Report<'a> {
    pub out: &'a mut dyn Write,
    pub policy: &'a Policy,
    pub records: Option<String>,
    escalated: usize,
}

pub(super) fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

impl<'a> Report<'a> {
    pub fn new(out: &'a mut dyn Write, policy: &'a Policy, records: Option<String>) -> Self {
        Report { out, policy, records, escalated: 0 }
    }

    /// How many escalations have been answered.
    pub fn escalations(&self) -> usize {
        self.escalated
    }

    pub fn event(&mut self, c: &Nsh, e: &Event) {
        match e {
            Event::HostAccepted => {
                say("host accepted: attestation, enrolment and firmware checked")
            }
            Event::LoggedIn(l) => {
                let caps: alloc::vec::Vec<String> =
                    l.caps.iter().map(|n| String::from_utf8_lossy(n).into()).collect();
                say(&format!("logged in: session {} with {}", hex(&l.session_id), caps.join(",")));
            }
            Event::Output(b) => {
                let _ = self.out.write_all(b).and_then(|_| self.out.flush());
            }
            Event::Escalated { reason, expires_ms } => {
                self.escalated += 1;
                say(&format!("escalation: {reason:?}, until {expires_ms} ms"));
            }
            Event::Exit(s) => say(&format!("the shell exited with {s}")),
            Event::Record(r) => self.record(c, r),
            Event::Closed => say("closed"),
        }
    }
}

/// One line on standard error, as `nsh: ...`. In raw mode a line needs its
/// carriage return.
pub fn say(s: &str) {
    let _ = write!(std::io::stderr(), "nsh: {s}\r\n");
}
