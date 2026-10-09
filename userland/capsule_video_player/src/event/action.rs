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

use crate::ui::screen::Route;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    None,
    TogglePlay,
    SeekBy(i32),
    SeekToPermille(u32),
    Restart,
    OpenIndex(usize),
    OpenSelected,
    MoveSel(i32),
    /// A wheel step over a list page: the list moves, the selection stays.
    Scroll(i32),
    ShowLibrary,
    Goto(Route),
    Back,
    ToggleGrid,
    /// A printable key typed into the search field.
    Type(u8),
    /// Backspace in the search field.
    Erase,
    /// A folder in the Folders rail: `None` is every folder.
    Folder(Option<usize>),
    Close,
}
