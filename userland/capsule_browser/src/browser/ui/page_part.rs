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

/// What a click on a pill does: picks History's or Downloads' filter
/// `i`, filters the Tabs page to a group (every tab for None), or adds one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pick {
    Filter(u8),
    Group(Option<u8>),
    NewGroup,
}

/// A control on a list page or the Tabs page: a header pill, the filtered
/// group's name and Delete group, a row that opens entry `i` of its list
/// and the button that removes it, and a tab's card, close button and
/// group chip. An entry or tab is named by its index in the shell and the
/// key of what stood there when it was drawn (`page_key`), so a press acts
/// only while that is still what the index holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PagePart {
    Pill(Pick),
    Name,
    Delete,
    Open(usize, u64),
    Remove(usize, u64),
    Tab(usize, u64),
    Close(usize, u64),
    Chip(usize, u64),
}
