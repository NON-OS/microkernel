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

//! The watchdog's lines, written into a buffer on its stack.

use core::fmt::Write;

use super::stall_rule::{Say, Step};

/// `say` as one log line; what does not fit in the buffer is cut.
pub fn line(say: Say) -> ([u8; 160], usize) {
    let mut w = Line { buf: [0; 160], len: 0 };
    let _ = write_say(&mut w, say);
    (w.buf, w.len)
}

fn write_say(w: &mut Line, say: Say) -> core::fmt::Result {
    match say {
        Say::Stuck(s, ms) => {
            write!(w, "[DISPLAY] turn {} stuck {} ms in {}", s.turn, ms, name(s.step))?;
            if s.step == Step::Request {
                write!(w, " op {:#06x} from pid {}", s.op, s.from)?;
            }
            w.write_str("\n")
        }
        Say::WentOn(turn, ms) => {
            writeln!(w, "[DISPLAY] turn {} went on after {} ms more", turn, ms)
        }
    }
}

fn name(s: Step) -> &'static str {
    match s {
        Step::Wait => "wait",
        Step::Request => "request",
        Step::Upkeep => "upkeep",
        Step::Compose => "compose",
        Step::Flush => "flush",
    }
}

/// A line on the stack; what does not fit is cut.
struct Line {
    buf: [u8; 160],
    len: usize,
}

impl Write for Line {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let n = s.len().min(self.buf.len() - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&s.as_bytes()[..n]);
        self.len += n;
        Ok(())
    }
}
