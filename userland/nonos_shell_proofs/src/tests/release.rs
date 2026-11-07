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

//! `version` and `commands` in an nsh session.

use crate::kit::{Lines, Script};
use nonos_shell::session::{release_line, Session};

fn run(s: &mut Session, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    s.run_line(line.as_bytes(), &mut out, &mut Script::default());
    out.text()
}

#[test]
fn version_prints_the_trees_release() {
    let release = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../VERSION"))
        .expect("the tree's VERSION file");
    let want = format!("NONOS {}", release.trim_end());
    assert_eq!(String::from_utf8_lossy(&release_line()), want);
    let mut s = Session::default();
    assert_eq!(run(&mut s, "version; echo $?"), [want.as_str(), "0"]);
}

#[test]
fn version_takes_no_argument() {
    let mut s = Session::default();
    assert_eq!(run(&mut s, "version x; echo $?"), ["usage: version", "1"]);
}

#[test]
fn commands_is_help() {
    let mut s = Session::default();
    assert_eq!(run(&mut s, "commands"), run(&mut s, "help"));
    assert_eq!(
        run(&mut s, "type commands version"),
        ["commands is built into the shell", "version is built into the shell"]
    );
}
