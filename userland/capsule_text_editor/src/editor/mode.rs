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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Code,
    Document,
}

/// The view a file opens in.
///
/// Code, always. This used to open `.txt`, `.md` and anything without an
/// extension as a document, which put a word-processor ribbon, a page count
/// and a word count in front of a reader who had opened a config file, and
/// took away the line numbers and syntax colouring they were looking for. On a
/// system whose files are mostly source and configuration, the document view
/// is the special case, not the default.
///
/// Ctrl+M switches, so nothing is lost: a reader who wants pages and a word
/// count is one keystroke away, and a reader who wants a gutter no longer has
/// to rename the file to get one.
pub(super) fn mode_for_path(_path: &str) -> Mode {
    Mode::Code
}
