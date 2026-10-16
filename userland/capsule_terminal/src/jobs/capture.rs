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

//! A running program's output sent to a file (`> f`, `>> f`) or dropped
//! (`> /dev/null`) instead of the screen. It is held as it arrives and
//! written when the program ends, so the file holds what the program wrote,
//! byte for byte, and a write that fails is said.

use alloc::vec::Vec;

#[path = "capture_end.rs"]
mod capture_end;
#[path = "capture_take.rs"]
mod capture_take;

pub struct Capture {
    /// The terminal's pid, which the vfs knows its files by.
    owner: u32,
    /// The file, resolved against the directory it was typed in; empty
    /// for `/dev/null`.
    path: Vec<u8>,
    append: bool,
    held: Vec<u8>,
    /// Bytes past `REDIRECT_MAX`, which no file is given.
    over: usize,
    /// The program tags each message with its stream (`TAG_STDOUT`,
    /// `TAG_STDERR`): the Linux personality, whose stderr is on the screen.
    split: bool,
}

impl Capture {
    pub fn to_file(owner: u32, path: Vec<u8>, append: bool) -> Self {
        Self { owner, path, append, held: Vec::new(), over: 0, split: false }
    }

    pub fn discard() -> Self {
        Self { owner: 0, path: Vec::new(), append: false, held: Vec::new(), over: 0, split: false }
    }

    /// Output from `tool`, which tags its streams when it is the Linux
    /// personality: stderr goes on to the screen, stdout to the file.
    pub fn split_by(mut self, tool: &[u8]) -> Self {
        self.split = tool == b"linux";
        self
    }
}
