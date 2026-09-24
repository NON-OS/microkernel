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

//! The settings that are wired to behaviour, by name.
//!
//! The section tables store their switches as bit positions, which is the
//! right shape for a table-driven panel and the wrong shape for the code that
//! has to obey them: a painter reading `sect_on(1, 4)` says nothing about what
//! it is asking. Every switch that actually changes what the editor does gets
//! a named reader here, and the paint and edit paths call these.
//!
//! This file is also the honest list. A switch with no function here is a
//! switch the editor draws and does not obey, and the aim is for that list to
//! be empty. Anything still on it is named in the module below so it can be
//! found rather than discovered. As of now that list is empty: every switch
//! the panel draws is read here.

use super::sect_state::sect_on;

/// Nav index of the Editing section, matching `sect::section_for`.
const EDITING: usize = 1;

/// Bit positions inside the Editing section, in the order its table lists them.
const BIT_INVISIBLES: u32 = 1;
const BIT_CURRENT_LINE: u32 = 4;

/// Draw a band behind the line the caret is on.
pub(crate) fn highlight_current_line() -> bool {
    sect_on(EDITING, BIT_CURRENT_LINE)
}

/// Mark spaces and tabs in the body text. Line ends are not marked; the
/// painter says why.
pub(crate) fn show_invisibles() -> bool {
    sect_on(EDITING, BIT_INVISIBLES)
}
