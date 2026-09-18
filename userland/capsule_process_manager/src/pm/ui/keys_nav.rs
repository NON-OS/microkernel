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

//! Getting around, and ordering the table.

use super::keys::{Act, Binding};
use super::keys_group::Group;

// Both cases of every letter, because a reader with caps lock on is asking for
// the same thing. The digits are the nav row's own positions.
pub const NAV: &[Binding] = &[
    Binding {
        key: b"1-6",
        codes: &[],
        label: b"jump to a screen by its number",
        group: Group::Move,
        act: Act::Screen(0),
    },
    Binding {
        key: b"Tab",
        codes: &[0x1005],
        label: b"next screen",
        group: Group::Move,
        act: Act::NextScreen,
    },
    Binding {
        key: b"s",
        codes: &[0x53, 0x73],
        label: b"show the security panel",
        group: Group::Move,
        act: Act::Security,
    },
    Binding {
        key: b"/",
        codes: &[0x2F],
        label: b"search by name",
        group: Group::Move,
        act: Act::Search,
    },
    Binding { key: b"?", codes: &[0x3F], label: b"this list", group: Group::Move, act: Act::Help },
    Binding {
        key: b"Esc",
        codes: &[],
        label: b"close the window",
        group: Group::Move,
        act: Act::Close,
    },
];
