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

//! The serve loop's own lines on the kernel's log (with Debug), each once,
//! so a boot shows where the compositor stands: serving, its first request,
//! its first frame let through, and any turn that held the loop past
//! SLOW_MS (at most SLOW_LINES of them), split into the drain, the upkeep
//! and the frame. Callers wait SLOW_MS * 5 for an answer, so a turn this
//! slow is one that leaves them unanswered. Pure: the loop says the lines.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub const SLOW_MS: i64 = 1000;
pub const SLOW_LINES: u8 = 4;

#[derive(Default)]
pub struct TurnWatch {
    served: bool,
    first_request: bool,
    first_frame: bool,
    slow_said: u8,
}

/// One turn: when it began, after the drain, after the upkeep, and after
/// the frame (uptime ms); the first request it served (op, sender); and
/// whether the first frame's hold has let go.
pub struct Turn {
    pub at: [i64; 4],
    pub request: Option<(u16, u32)>,
    pub released: bool,
}

impl TurnWatch {
    /// The lines this turn adds.
    pub fn observe(&mut self, t: &Turn) -> Vec<String> {
        let [t0, t1, t2, t3] = t.at;
        let mut out = Vec::new();
        if !core::mem::replace(&mut self.served, true) {
            out.push(format!("serving: first turn at {} ms took {} ms", t0, t3 - t0));
        }
        if let Some((op, pid)) = t.request.filter(|_| !self.first_request) {
            self.first_request = true;
            out.push(format!("first request op {:#06x} from pid {} at {} ms", op, pid, t1));
        }
        if t.released && !core::mem::replace(&mut self.first_frame, true) {
            out.push(format!("first frame let through at {} ms", t3));
        }
        if t3 - t0 >= SLOW_MS && self.slow_said < SLOW_LINES {
            self.slow_said += 1;
            let (d, u, f) = (t1 - t0, t2 - t1, t3 - t2);
            out.push(format!("slow turn {} ms (drain {}, upkeep {}, frame {})", t3 - t0, d, u, f));
        }
        out
    }
}
