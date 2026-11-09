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

//! The browser's pop-ups as shared widgets, at 1x and 2x: a select's list
//! is a Menu whose options light and are chosen only where they are drawn,
//! the current option ticked, a heading never lit or chosen, opening by its
//! select inside the page; the link bubble is a Tooltip and the short-name
//! notice a Toast, each placed and drawn inside the page, the notice whole.

#[path = "popup_controls/held_tests.rs"]
mod held_tests;
#[path = "popup_controls/kit.rs"]
mod kit;
#[path = "popup_controls/menu_tests.rs"]
mod menu_tests;
#[path = "popup_controls/notice_tests.rs"]
mod notice_tests;
#[path = "popup_controls/place_tests.rs"]
mod place_tests;
#[path = "popup_controls/spots.rs"]
mod spots;
