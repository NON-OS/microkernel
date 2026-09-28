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

//! OSC requests that are not titles or colours: the working directory,
//! the clipboard and shell prompt marks.

use super::dispatch::clean_text;
use crate::limits::MAX_PROMPT_MARKS;
use crate::term::state::Term;
use crate::term::types::ClipboardRequest;
use alloc::string::String;
use alloc::vec::Vec;

impl Term {
    /// `file://host/path`: the host part is dropped, the path kept.
    pub(in crate::term) fn set_cwd(&mut self, rest: &[u8]) {
        let s = clean_text(rest, 1024);
        let path = match s.strip_prefix("file://") {
            Some(r) => r.find('/').map(|i| String::from(&r[i..])).unwrap_or_default(),
            None => s,
        };
        self.cwd = (!path.is_empty()).then_some(path);
    }

    /// `52;targets;base64`. A `?` asks to read the clipboard; it is refused
    /// and counted, because a program that can read the clipboard can read
    /// whatever the user copied from anywhere else.
    pub(in crate::term) fn osc_clipboard(&mut self, rest: &[u8]) {
        let Some(i) = rest.iter().position(|&b| b == b';') else { return };
        let (targets, data) = (&rest[..i], &rest[i + 1..]);
        if data == b"?" {
            self.clipboard_reads_refused = self.clipboard_reads_refused.saturating_add(1);
            return;
        }
        let ok = data.iter().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'/' | b'='));
        if ok {
            let targets = if targets.is_empty() { Vec::from(&b"s0"[..]) } else { targets.to_vec() };
            self.clipboard = Some(ClipboardRequest { targets, base64: data.to_vec() });
        }
    }

    /// `133;A` marks where a prompt starts, so the host can jump between
    /// commands.
    pub(in crate::term) fn osc_prompt(&mut self, rest: &[u8]) {
        if rest.first() == Some(&b'A') && !self.alt_active {
            let line = self.scrolled + self.primary.cur.y as u64;
            if self.prompts.back() != Some(&line) {
                if self.prompts.len() >= MAX_PROMPT_MARKS {
                    self.prompts.pop_front();
                }
                self.prompts.push_back(line);
            }
        }
    }
}
