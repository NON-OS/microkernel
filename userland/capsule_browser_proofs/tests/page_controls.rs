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

//! The list pages and the Tabs page, drawn from shared widgets and laid
//! out in one `Controls`, at 1x and 2x: each control acts across one box
//! and lights only inside it, a row's remove button sits at its right end
//! and covers the row, the filter or tab in force is the only one marked,
//! New group is dead once the window is full, nothing under the header
//! band takes a click, and a press on an entry or tab that moved or went
//! does not act on what took its place.

#[path = "page_controls/band_tests.rs"]
mod band_tests;
#[path = "page_controls/fixtures.rs"]
mod fixtures;
#[path = "page_controls/found.rs"]
mod found;
#[path = "page_controls/group_tests.rs"]
mod group_tests;
#[path = "page_controls/kit.rs"]
mod kit;
#[path = "page_controls/look_tests.rs"]
mod look_tests;
#[path = "page_controls/mark_tests.rs"]
mod mark_tests;
#[path = "page_controls/press_tests.rs"]
mod press_tests;
#[path = "page_controls/stale_tests.rs"]
mod stale_tests;
#[path = "page_controls/tab_tests.rs"]
mod tab_tests;
