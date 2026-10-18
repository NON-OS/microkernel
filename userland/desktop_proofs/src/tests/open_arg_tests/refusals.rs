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

//! What is never handed over, and how long a command waits.

use crate::open_arg::command::{RUN, TYPE};
use crate::open_arg::pending::COMMAND_TTL_MS;
use crate::open_arg::{is_path, still_wanted, tool_command};
use crate::tool_apps::TOOL_APPS;

#[test]
fn a_command_is_never_a_path_and_a_path_never_a_command() {
    for tool in TOOL_APPS.iter() {
        assert!(!is_path(&tool_command(tool.label).unwrap()));
    }
    assert!(!RUN.starts_with('/') && !TYPE.starts_with('/'));
    assert!(is_path("/home/nonos/notes.txt"));
    for refused in ["run:grex", "type:rm -r /", "notes.txt", "", " /x"] {
        assert!(!is_path(refused), "{refused:?} would be left as a path");
    }
}

#[test]
fn a_name_that_is_not_a_plain_word_hands_nothing() {
    for odd in [&b""[..], b"grex; rm -r /", b"a b", b"x\n", b"\xff", b"../grex"] {
        assert_eq!(tool_command(odd), None, "{odd:?}");
    }
}

#[test]
fn a_command_waits_for_the_terminal_only_so_long() {
    assert!(still_wanted(1_000, 1_000));
    assert!(still_wanted(1_000, 1_000 + COMMAND_TTL_MS - 1));
    assert!(!still_wanted(1_000, 1_000 + COMMAND_TTL_MS));
    // A clock that went back is not a reason to run an old command.
    assert!(!still_wanted(1_000, 999));
}
