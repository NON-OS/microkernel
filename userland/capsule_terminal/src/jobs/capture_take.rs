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

//! Holding a program's output as it arrives, split by stream when tagged.

use nonos_libc::{TAG_STDERR, TAG_STDOUT};

use crate::command::dispatch::REDIRECT_MAX;
use crate::command::output::Output;

use super::Capture;

impl Capture {
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
}
