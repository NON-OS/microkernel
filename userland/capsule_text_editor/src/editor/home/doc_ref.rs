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

//! A document as the Home list shows it, and the words around the list.

use alloc::string::String;

pub(super) struct DocRef {
    pub path: String,
    pub name: String,
}

pub(super) fn section_title(nav: usize) -> &'static str {
    match nav {
        1 => "Recent Documents",
        _ => "All Documents",
    }
}

pub(super) fn empty_line(nav: usize) -> &'static str {
    match nav {
        1 => "No documents opened yet this session.",
        _ => "No documents in the store.",
    }
}
