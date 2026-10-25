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

//! A message's bytes taken as they come, and one let go as broken.

use super::assemble::Assembler;
use super::heard::Heard;

impl Assembler {
    pub(super) fn take(&mut self, data: &[u8]) -> Heard {
        let n = (self.need - self.got.len()).min(data.len());
        self.got.extend_from_slice(&data[..n]);
        if self.got.len() < self.need {
            return Heard::Nothing;
        }
        self.seq = None;
        Heard::Message(self.cmd, core::mem::take(&mut self.got))
    }

    pub(super) fn broken(&mut self) -> Heard {
        (self.seq, self.need) = (None, 0);
        self.got.clear();
        Heard::Broken
    }
}
