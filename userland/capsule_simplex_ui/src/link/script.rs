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

//! The recorded session's text: one line per entry.
//!
//! `> <command>` opens a block; each `< <resp>` after it is an answer to that
//! command; `! <resp>` is an event the core pushes later, in order. `#` lines
//! and blank lines are notes. A `<resp>` is the `resp` object alone; the
//! transcript puts the frame and `corrId` around it.

use alloc::vec::Vec;

pub struct Block {
    pub cmd: &'static str,
    pub answers: Vec<&'static str>,
}

pub struct Script {
    pub blocks: Vec<Block>,
    pub events: Vec<&'static str>,
}

pub fn script(text: &'static str) -> Script {
    let mut s = Script { blocks: Vec::new(), events: Vec::new() };
    for line in text.lines() {
        if let Some(cmd) = line.strip_prefix("> ") {
            s.blocks.push(Block { cmd: cmd.trim_end(), answers: Vec::new() });
        } else if let Some(resp) = line.strip_prefix("< ") {
            if let Some(block) = s.blocks.last_mut() {
                block.answers.push(resp);
            }
        } else if let Some(resp) = line.strip_prefix("! ") {
            s.events.push(resp);
        }
    }
    s
}

impl Script {
    pub fn answers(&self, cmd: &str) -> Option<&[&'static str]> {
        self.blocks.iter().find(|b| b.cmd == cmd).map(|b| b.answers.as_slice())
    }
}
