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
//! Frames put together from reports. In hybrid mode a device sends a
//! frame's fingers over several reports: the first carries Contact Count,
//! the rest a count of 0 (Windows precision touchpad and touchscreen
//! protocol, as recalled, unverified). In parallel mode one report is the
//! whole frame. A report without Contact Count is a frame of its own.

use super::frame::Frame;
use super::slots::MAX_SLOTS;

#[derive(Clone, Copy, Default)]
pub struct Assembler {
    pending: Frame,
    /// Fingers the frame being put together still waits for.
    expect: usize,
}

impl Assembler {
    /// The frame `report` completes, if it completes one.
    pub fn push(&mut self, report: Frame) -> Option<Frame> {
        match report.count {
            None => return Some(report),
            Some(0) if self.expect == 0 => return None,
            Some(0) => {}
            Some(c) => {
                self.pending = Frame { n: 0, ..report };
                self.expect = (c as usize).min(MAX_SLOTS);
            }
        }
        for t in &report.touches[..report.n] {
            if self.pending.n == self.expect {
                break;
            }
            self.pending.touches[self.pending.n] = *t;
            self.pending.n += 1;
        }
        if self.pending.n < self.expect {
            return None;
        }
        self.expect = 0;
        Some(self.pending)
    }
}
