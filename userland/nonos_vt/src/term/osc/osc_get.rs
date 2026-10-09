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

//! What the host reads back of what programs set through OSC.

use crate::term::state::Term;
use crate::term::types::ClipboardRequest;

impl Term {
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Changes each time the title does, so the host redraws it only then.
    pub fn title_generation(&self) -> u32 {
        self.title_gen
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    pub fn take_clipboard_request(&mut self) -> Option<ClipboardRequest> {
        self.clipboard.take()
    }

    pub fn clipboard_reads_refused(&self) -> u32 {
        self.clipboard_reads_refused
    }

    /// Bells since the last call.
    pub fn take_bell(&mut self) -> u32 {
        core::mem::take(&mut self.bell)
    }

    pub fn prompt_lines(&self) -> impl Iterator<Item = u64> + '_ {
        self.prompts.iter().copied()
    }
}
