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

//! The titlebar tab strip, drawn from the shared `Tabs` and icon buttons
//! and laid out in one `Controls`, at 1x and 2x: each tab, close mark and
//! the plus lights and acts only where it is drawn, choosing a tab marks
//! only it, and a close mark acts on its own tab.

#[path = "tab_strip_controls/kit.rs"]
mod kit;
#[path = "tab_strip_controls/look_tests.rs"]
mod look_tests;
#[path = "tab_strip_controls/press_tests.rs"]
mod press_tests;
