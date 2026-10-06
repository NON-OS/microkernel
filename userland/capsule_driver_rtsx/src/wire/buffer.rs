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

//! A command buffer being built: up to 256 entries, as the 1 KiB host
//! command area holds (HOST_CMDS_BUF_LEN / 4). An entry past the end is not
//! silently dropped as in Linux; the buffer remembers it and is refused.

use super::encode::cmd_entry;
use super::kind::CmdKind;

pub const MAX_CMDS: usize = 256;

pub struct CmdBuf {
    words: [u32; MAX_CMDS],
    count: usize,
    overflow: bool,
}

impl CmdBuf {
    pub const fn new() -> Self {
        Self { words: [0; MAX_CMDS], count: 0, overflow: false }
    }

    pub fn add(&mut self, kind: CmdKind, reg: u16, mask: u8, data: u8) -> &mut Self {
        if self.count == MAX_CMDS {
            self.overflow = true;
            return self;
        }
        self.words[self.count] = cmd_entry(kind, reg, mask, data);
        self.count += 1;
        self
    }

    pub fn write(&mut self, reg: u16, mask: u8, data: u8) -> &mut Self {
        self.add(CmdKind::Write, reg, mask, data)
    }

    pub fn words(&self) -> &[u32] {
        &self.words[..self.count]
    }

    pub const fn overflowed(&self) -> bool {
        self.overflow
    }
}

impl Default for CmdBuf {
    fn default() -> Self {
        Self::new()
    }
}
