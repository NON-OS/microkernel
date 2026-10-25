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

pub fn e(kind: Kind, label: &str) -> Entry<'_> {
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

/// Each row keeps the tag of where it came from, and an index built from
/// labels alone runs its row; every action a row can carry is its own.
pub fn assert_rows_keep_their_source(items: &[Entry]) {
    let tags: Vec<&str> = items.iter().map(|x| x.kind.tag()).collect();
    assert_eq!(tags, ["cmd", "cmd", "run", "run", "tab", "dir", "act"]);
    assert!(items.iter().all(|x| x.hint.is_empty() && x.action == Action::Run));
    let all = [
        Action::Run,
        Action::Session(1),
        Action::Project(1),
        Action::NewSession,
        Action::ToggleMonitor,
        Action::ChangeTheme,
    ];
    for (i, a) in all.iter().enumerate() {
        for (j, b) in all.iter().enumerate() {
            assert_eq!(i == j, a == b, "{a:?} {b:?}");
        }
    }
}
