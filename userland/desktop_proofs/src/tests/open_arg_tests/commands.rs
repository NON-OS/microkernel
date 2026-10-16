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

//! The command each tool tile hands the terminal.

use crate::open_arg::command::{runs_bare, RUN, TYPE};
use crate::open_arg::tool_command;
use crate::tool_apps::TOOL_APPS;

#[test]
fn pastel_runs_bare_because_bare_it_prints_its_help() {
    assert_eq!(tool_command(b"pastel").as_deref(), Some("run:pastel"));
}

#[test]
fn a_tool_that_needs_input_is_typed_with_the_cursor_after_a_space() {
    for tool in ["grex", "jsonxf", "huniq", "csview", "tokei", "dotenv-linter"] {
        let want = format!("type:{tool} ");
        assert_eq!(tool_command(tool.as_bytes()).as_deref(), Some(want.as_str()), "{tool}");
    }
}

#[test]
fn every_tile_hands_a_command_and_only_pastel_runs() {
    for tool in TOOL_APPS.iter() {
        let line = tool_command(tool.label).expect("every generated name is a plain word");
        let name = core::str::from_utf8(tool.label).unwrap();
        if tool.label == b"pastel" {
            assert_eq!(line, format!("{RUN}{name}"));
        } else {
            assert_eq!(line, format!("{TYPE}{name} "));
        }
        assert_eq!(runs_bare(tool.label), tool.label == b"pastel");
    }
}
