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

//! A small mixed index of palette entries.

use crate::palette::entry::{Action, Entry, Kind};

pub fn e(kind: Kind, label: &str) -> Entry {
    Entry { kind, label, hint: "", action: Action::Run }
}

pub fn index() -> Vec<Entry<'static>> {
    vec![
        e(Kind::Command, "clear"),
        e(Kind::Command, "history"),
        e(Kind::History, "git status"),
        e(Kind::History, "cargo check"),
        e(Kind::Session, "src"),
        e(Kind::Project, "/home/user/src"),
        e(Kind::Action, "Change Theme"),
    ]
}
