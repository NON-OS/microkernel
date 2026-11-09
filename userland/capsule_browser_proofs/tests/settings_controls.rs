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

//! The settings page's buttons, drawn as shared `Button`s and laid out in
//! one `Controls`, at 1x and 2x: each lights and acts only where it is
//! drawn, one with nothing to act on never lights or acts, the choice in
//! force moves only between its row's buttons, and Remove all bookmarks
//! shows its second click before it removes anything.

#[path = "settings_controls/kit.rs"]
mod kit;
#[path = "settings_controls/look_tests.rs"]
mod look_tests;
#[path = "settings_controls/press_tests.rs"]
mod press_tests;
