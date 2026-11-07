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

//! The shell behind the sealed session: the shared NONOS shell
//! (`nonos_shell::session`), the same command language the terminal reads,
//! with its builtins, pipe filters and control flow, run as the admitted
//! login, which `whoami` prints. Each admitted line is recorded with
//! `command` before its output, as nsh/1 requires; the record goes to the
//! client at the session's end, and nsh.front keeps no copy, since it holds
//! no filesystem capability. An escalation is refused, since no second
//! session launcher exists in the guest yet.

use crate::line::{Line, Lines};
use crate::session::Host;
use crate::shell_host::FrontHost;
use crate::{clock, refused_said, rng};
use nonos_libc::mk_debug;
use nonos_shell::out::Crlf;
use nonos_shell::session::{Flow, Session, Who};
use nsh_server::Event as S;

const BANNER: &[u8] = b"nsh: NONOS attested shell (development); help lists the commands\r\n";
const TOO_LONG: &[u8] = b"nsh: line too long, dropped\r\n";

/// Line-buffers terminal input and runs whole lines in one shell session.
pub struct Shell {
    lines: Lines,
    session: Session,
}

impl Shell {
    pub fn new() -> Self {
        Shell { lines: Lines::default(), session: Session::default() }
    }

    /// Act on one server event.
    pub fn act(&mut self, s: &mut Host, e: &S) {
        let now = clock::now();
        let mut rand = rng::fill;
        match e {
            S::Admit(a) => {
                self.session.who = Some(Who {
                    user: a.user.clone(),
                    role: a.role.clone(),
                    subject: a.subject,
                    caps: a.caps.names(),
                });
                let _ = s.begin(a.caps, now, &mut rand);
                let _ = s.output(BANNER, now, &mut rand);
            }
            S::Input(bytes) => self.feed(s, bytes),
            S::Escalate { .. } => {
                let _ = s.escalation_failed(now, &mut rand);
            }
            S::Stop(_) => {
                let _ = s.exit(0, now, &mut rand);
            }
            S::Refused(why) => {
                let text = refused_said::line(why);
                let _ = mk_debug(text.as_ptr(), text.len());
            }
            _ => {}
        }
    }

    fn feed(&mut self, s: &mut Host, bytes: &[u8]) {
        for line in self.lines.feed(bytes) {
            match line {
                Line::Command(cmd) => self.run(s, &cmd),
                Line::TooLong => {
                    let _ = s.output(TOO_LONG, clock::now(), &mut rng::fill);
                }
            }
        }
    }

    fn run(&mut self, s: &mut Host, raw: &[u8]) {
        let mut rand = rng::fill;
        let _ = s.command(raw, clock::now(), &mut rand);
        let mut out = Crlf::default();
        let flow = self.session.run_line(trim_cr(raw), &mut out, &mut FrontHost);
        if !out.0.is_empty() {
            let _ = s.output(&out.0, clock::now(), &mut rand);
        }
        if let Flow::Exit(code) = flow {
            let _ = s.exit(code, clock::now(), &mut rand);
        }
    }
}

/// A line typed at a terminal may end in a carriage return.
fn trim_cr(b: &[u8]) -> &[u8] {
    b.strip_suffix(b"\r").unwrap_or(b)
}
