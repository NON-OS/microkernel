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

//! One line per attempt, and a verdict that is the exit status.

use std::process::ExitCode;

use crate::sys::out;

/// What a probe observed.
pub enum Seen {
    /// The personality said no; the value is the errno it gave.
    Refused(i64),
    /// The attempt reached something it should not have.
    Escaped(String),
}

pub struct Report {
    guest: &'static str,
    escaped: u32,
    tried: u32,
}

impl Report {
    pub fn new(guest: &'static str) -> Self {
        out(format!("[GUEST] {guest} start\n").as_bytes());
        Report { guest, escaped: 0, tried: 0 }
    }

    pub fn check(&mut self, what: &str, seen: Seen) {
        self.tried += 1;
        let line = match seen {
            Seen::Refused(e) => format!("[GUEST] {} refused {what} errno={}\n", self.guest, -e),
            Seen::Escaped(how) => {
                self.escaped += 1;
                format!("[GUEST] {} ESCAPED {what}: {how}\n", self.guest)
            }
        };
        out(line.as_bytes());
    }

    /// Zero only when every attempt was refused.
    pub fn finish(self) -> ExitCode {
        let verdict = if self.escaped == 0 { "held" } else { "BROKEN" };
        out(format!(
            "[GUEST] {} {verdict}: {} tried, {} escaped\n",
            self.guest, self.tried, self.escaped
        )
        .as_bytes());
        ExitCode::from(u8::from(self.escaped != 0))
    }
}
