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

use nonos_libc::{TAG_STDERR, TAG_STDOUT};

use crate::command::dispatch::{not_written, write_out, wrote_to, REDIRECT_MAX};
use crate::command::output::Output;

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

    /// One message of the program's output: the screen's when it is a tagged
    /// stderr, else this file's.
    pub fn route(&mut self, msg: &[u8], out: &mut Output<'_>) {
        match (self.split, msg.split_first()) {
            (true, Some((&TAG_STDERR, rest))) => out.feed_raw(rest),
            (true, Some((&TAG_STDOUT, rest))) => self.take(rest),
            _ => self.take(msg),
        }
    }

    /// Output the program wrote.
    pub fn take(&mut self, bytes: &[u8]) {
        if self.path.is_empty() {
            return;
        }
        let room = REDIRECT_MAX - self.held.len();
        let kept = bytes.len().min(room);
        self.held.extend_from_slice(&bytes[..kept]);
        self.over += bytes.len() - kept;
    }

    /// The program ended: write the file and say how that went. False when
    /// the file does not hold all the program wrote.
    pub fn finish(&mut self, out: &mut Output<'_>) -> bool {
        if self.path.is_empty() {
            return true;
        }
        let held = core::mem::take(&mut self.held);
        if let Err(e) = write_out(self.owner, &self.path, &held, self.append) {
            out.writeln_error(&not_written(&self.path, e));
            return false;
        }
        out.writeln(&wrote_to(&self.path));
        if self.over > 0 {
            out.writeln_error(&[&self.path[..], b": the output past 1 MiB was not kept"].concat());
            return false;
        }
        true
    }

    /// The program was stopped with Ctrl+C: what it wrote is dropped, with
    /// the file left as it stood when it started.
    pub fn interrupted(&self, out: &mut Output<'_>) {
        if !self.path.is_empty() {
            out.writeln_error(&not_written(&self.path, "the program was interrupted"));
        }
    }
}
