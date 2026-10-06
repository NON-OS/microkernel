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

//! The answers queued for the host to write back to the program.

use crate::limits::MAX_REPLY;
use crate::out::Out;
use crate::term::state::Term;
use alloc::vec::Vec;
use core::fmt::Write;

impl Term {
    /// Queue an answer. A program that never reads cannot grow the queue
    /// past its ceiling; the answer that would is dropped whole.
    pub(in crate::term) fn reply(&mut self, bytes: &[u8]) {
        if self.replies.len() + bytes.len() <= MAX_REPLY {
            self.replies.extend_from_slice(bytes);
        }
    }

    pub(in crate::term) fn reply_fmt(&mut self, args: core::fmt::Arguments) {
        let mut buf = Vec::new();
        let _ = Out(&mut buf).write_fmt(args);
        self.reply(&buf);
    }

    /// Bytes the host must write back to the program, oldest first.
    pub fn take_replies(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.replies)
    }
}
