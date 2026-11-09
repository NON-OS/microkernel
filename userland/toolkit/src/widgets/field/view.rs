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

//! The run a field shows, and how its byte indices map to the text's.

use alloc::string::String;

/// The run a field shows, and where the text in it came from: `lead` bytes of
/// ellipsis, then the `keep` bytes of the text that start at `from`.
pub(super) struct View {
    pub(super) shown: String,
    pub(super) lead: usize,
    pub(super) from: usize,
    pub(super) keep: usize,
}

impl View {
    /// Where text index `c`, a char boundary, falls in the shown run. An
    /// index in the cut-off part falls just after the ellipsis.
    pub(super) fn shown_at(&self, c: usize) -> usize {
        self.lead.saturating_add(c.saturating_sub(self.from))
    }

    /// The text index shown index `i` stands for, in a text `len` long. The
    /// ellipsis before the kept part stands for its start, and the one after
    /// it for the end of the text.
    pub(super) fn text_at(&self, i: usize, len: usize) -> usize {
        match i.checked_sub(self.lead) {
            None => self.from,
            Some(j) if j <= self.keep => self.from.saturating_add(j),
            Some(_) => len,
        }
    }
}
