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

/* What a key does to a single-line text field. Shared by the address bar
 * and page form fields, so both read keys the same way. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditKey {
    Insert(char),
    Backspace,
    WordBackspace,
    Delete,
    WordDelete,
    Left { word: bool, extend: bool },
    Right { word: bool, extend: bool },
    Home { extend: bool },
    End { extend: bool },
    SelectAll,
    Copy,
    Cut,
    Paste,
    Commit,
    Cancel,
    /* Ctrl+L, Alt+D or F6: put the caret in the address bar from anywhere. */
    FocusOmnibox,
    Ignore,
}
